//! GGML rotary graph transport. Prism supplies addresses and Verdant computes coefficients and values.

use super::{graph_tensor_data, scalar_word, workspace, BackendError, Tensor, TensorDescriptor};
use verdant_tensor::linear::{
    convert_float_storage, rotate_pair_f32, signed_word_f32, RotaryParameters, RotarySpectrum,
};
use vtensor_rotary::{
    Adt0 as Request, Adt1 as Layout, Adt2 as Descriptor, Adt3 as Plan, Adt4 as Response,
};

fn request(query: Request) -> Result<Response, BackendError> {
    vtensor_rotary::run(query).map_err(|_| BackendError::Overflow)
}

fn descriptor(
    TensorDescriptor::C0(a, b, c, d, e, f, g, h, i, j, k): TensorDescriptor,
) -> Descriptor {
    Descriptor::C0(a, b, c, d, e, f, g, h, i, j, k)
}

pub(super) fn plan(
    tensor: &Tensor,
    source: TensorDescriptor,
    positions: TensorDescriptor,
    factors: Option<TensorDescriptor>,
    output: TensorDescriptor,
) -> Result<Option<Plan>, BackendError> {
    let param = |i: usize| u64::from(tensor.op_params[i] as u32);
    let layout = Layout::C0(
        param(1),
        param(2),
        param(15),
        param(11),
        param(12),
        param(13),
        param(14),
    );
    match request(Request::C0(
        tensor.op as u64,
        tensor.src[3..].iter().any(|v| !v.is_null()),
        layout,
        descriptor(source),
        descriptor(positions),
        factors.map(descriptor),
        descriptor(output),
    ))? {
        Response::C2(plan) => Ok(Some(plan)),
        Response::C0 => Ok(None),
        _ => Err(BackendError::InvalidTensor),
    }
}

fn convert(from: u64, to: u64, word: u64) -> Result<u64, BackendError> {
    convert_float_storage(from, to, word)
        .map_err(|_| BackendError::Overflow)?
        .ok_or(BackendError::InvalidTensor)
}

fn spectrum(tensor: &Tensor) -> Result<RotarySpectrum, BackendError> {
    let p = &tensor.op_params;
    let value = |i: usize| f32::from_bits(p[i] as u32);
    RotarySpectrum::new(RotaryParameters {
        dimensions: p[1] as u32,
        context: p[4],
        base: value(5),
        frequency_scale: value(6),
        extension: value(7),
        magnitude: value(8),
        rotations: [value(9), value(10)],
        truncate: p[3] == 0,
    })
    .map_err(|_| BackendError::InvalidTensor)
}

pub(super) fn execute(tensor: &Tensor, plan: Plan) -> Result<(), BackendError> {
    let Plan::C0(width, pairs, channels, heads, tokens, batches, layout, _, _) = &plan;
    // SAFETY: graph admission requires data and positions and validates the optional factor descriptor.
    let (source, positions, factors) =
        unsafe { (&*tensor.src[0], &*tensor.src[1], tensor.src[2].as_ref()) };
    let (input, output) = (graph_tensor_data(source)?, graph_tensor_data(tensor)?);
    let axis_count = (positions.ne[0] as u64)
        .checked_div(*tokens)
        .ok_or(BackendError::InvalidTensor)? as usize;
    let positions = graph_tensor_data(positions)?;
    let factors = factors.map(graph_tensor_data).transpose()?;
    let spectrum = spectrum(tensor)?;
    let format = source.ty as u64;
    let mut coefficients = workspace(*pairs, (0_f32, 0_f32))?;
    let mut schedule = workspace(*pairs, (0_usize, None, 1_f32))?;
    for (lane, slot) in schedule.iter_mut().enumerate() {
        let Response::C6(axis, reset) = request(Request::C3(layout.clone(), lane as u64))? else {
            return Err(BackendError::InvalidTensor);
        };
        let factor = match factors {
            // SAFETY: Prism admitted this contiguous F32 factor word.
            Some(pointer) => {
                f32::from_bits(unsafe { scalar_word(pointer.add(lane * 4), 4, None)? } as u32)
            }
            None => 1.0,
        };
        *slot = (
            axis as usize,
            reset.checked_sub(1).map(|axis| axis as usize),
            factor,
        );
    }
    let mut lanes = workspace(channels / 2, (None, 0, 0))?;
    for (index, item) in lanes.iter_mut().enumerate() {
        *item = match request(Request::C2(plan.clone(), index as u64))? {
            Response::C4(slot, a, b) => (Some(slot as usize), a as usize, b as usize),
            Response::C5(a, b) => (None, a as usize, b as usize),
            _ => return Err(BackendError::InvalidTensor),
        };
    }
    for token in 0..*tokens {
        let mut base = [0.0; 4];
        for (axis, value) in base.iter_mut().enumerate().take(axis_count) {
            // SAFETY: Prism bounded the contiguous position vector for every admitted axis.
            let word = unsafe {
                scalar_word(
                    positions.add((axis * *tokens as usize + token as usize) * 4),
                    4,
                    None,
                )?
            };
            *value = signed_word_f32(word as i32).map_err(|_| BackendError::InvalidTensor)?;
        }
        let mut phase = base;
        for (lane, (weight, &(axis, reset, factor))) in
            coefficients.iter_mut().zip(&schedule).enumerate()
        {
            if let Some(reset) = reset {
                phase[reset] = base[reset];
            }
            *weight = spectrum
                .weights(phase[axis], factor, lane as u32, tensor.op == 46)
                .map_err(|_| BackendError::InvalidTensor)?;
            for value in phase.iter_mut().take(axis_count) {
                *value = spectrum
                    .advance(*value)
                    .map_err(|_| BackendError::InvalidTensor)?;
            }
        }
        for batch in 0..*batches {
            for head in 0..*heads {
                let Response::C3(src, dst) =
                    request(Request::C1(plan.clone(), head, token, batch))?
                else {
                    return Err(BackendError::InvalidTensor);
                };
                for &(slot, a, b) in &lanes {
                    // SAFETY: the certified plan bounds both lane addresses in these admitted rows.
                    // Both loads precede either store so an identical source/output view is supported.
                    unsafe {
                        let read =
                            |i: usize| scalar_word(input.add(src as usize + i), *width, None);
                        let (x, y) = (read(a)?, read(b)?);
                        let (x, y) = match slot {
                            Some(slot) => {
                                let f = |word| {
                                    convert(format, 0, word).map(|bits| f32::from_bits(bits as u32))
                                };
                                let (x, y) = rotate_pair_f32((f(x)?, f(y)?), coefficients[slot])
                                    .map_err(|_| BackendError::InvalidTensor)?;
                                (
                                    convert(0, format, u64::from(x.to_bits()))?,
                                    convert(0, format, u64::from(y.to_bits()))?,
                                )
                            }
                            None => (x, y),
                        };
                        scalar_word(output.add(dst as usize + a), *width, Some(x))?;
                        scalar_word(output.add(dst as usize + b), *width, Some(y))?;
                    }
                }
            }
        }
    }
    Ok(())
}
