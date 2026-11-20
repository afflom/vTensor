#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]
extern crate alloc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComputeError {
    AddOverflow,
    MulOverflow,
    ShiftExponentTooLarge,
    ShiftOverflow,
    PowExponentTooLarge,
    PowOverflow,
    OutputTooSmall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pin {
    pub expected: crate::Digest,
    pub observed: crate::Digest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Digest {
    pub w0: u64,
    pub w1: u64,
    pub w2: u64,
    pub w3: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Observation {
    pub devcontainer: crate::Pin,
    pub bootstrap: crate::Pin,
    pub agents: crate::Pin,
    pub verification: crate::Pin,
    pub prismpmLock: crate::Pin,
    pub templateContract: crate::Pin,
}

pub fn supportsF64Matmul4(typeIsF64: bool, lhsShapeIs4x4: bool, rhsShapeIs4x4: bool) -> bool {
    match typeIsF64 {
        false => typeIsF64,
        true => match lhsShapeIs4x4 {
            false => lhsShapeIs4x4,
            true => rhsShapeIs4x4,
        },
    }
}

pub fn conformant(observation: crate::Observation) -> bool {
    {
        let _x_48 = (observation).devcontainer;
        {
            let _x_49 = pinMatches(_x_48);
            match _x_49 {
                false => _x_49,
                true => {
                    let _x_92 = (observation).bootstrap;
                    {
                        let _x_93 = pinMatches(_x_92);
                        match _x_93 {
                            false => _x_93,
                            true => {
                                let _x_112 = (observation).agents;
                                {
                                    let _x_113 = pinMatches(_x_112);
                                    match _x_113 {
                                        false => _x_113,
                                        true => {
                                            let _x_126 = (observation).verification;
                                            {
                                                let _x_127 = pinMatches(_x_126);
                                                match _x_127 {
                                                    false => _x_127,
                                                    true => {
                                                        let _x_134 = (observation).prismpmLock;
                                                        {
                                                            let _x_135 = pinMatches(_x_134);
                                                            match _x_135 {
                                                                false => _x_135,
                                                                true => {
                                                                    let _x_139 = (observation)
                                                                        .templateContract;
                                                                    {
                                                                        let _x_140 =
                                                                            pinMatches(_x_139);
                                                                        _x_140
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn digestAt(
    value: alloc::vec::Vec<u8>,
    offset: u64,
) -> Result<crate::Digest, crate::ComputeError> {
    __prod_borrowed_digestAt(value.as_ref(), offset)
}

fn __prod_borrowed_digestAt(
    value: &[u8],
    offset: u64,
) -> Result<crate::Digest, crate::ComputeError> {
    Ok({
        let _x_1 = 8;
        {
            let _x_4 = 0;
            {
                let _x_7 = __prod_borrowed_digestFrom((value).as_ref(), offset, _x_1, _x_4)?;
                {
                    let _x_32 = core::convert::identity::<u64>(offset)
                        .checked_add(_x_1)
                        .ok_or(crate::ComputeError::AddOverflow)?;
                    {
                        let _x_12 =
                            __prod_borrowed_digestFrom((value).as_ref(), _x_32, _x_1, _x_4)?;
                        {
                            let _x_13 = 16;
                            {
                                let _x_37 = core::convert::identity::<u64>(offset)
                                    .checked_add(_x_13)
                                    .ok_or(crate::ComputeError::AddOverflow)?;
                                {
                                    let _x_17 = __prod_borrowed_digestFrom(
                                        (value).as_ref(),
                                        _x_37,
                                        _x_1,
                                        _x_4,
                                    )?;
                                    {
                                        let _x_18 = 24;
                                        {
                                            let _x_42 = core::convert::identity::<u64>(offset)
                                                .checked_add(_x_18)
                                                .ok_or(crate::ComputeError::AddOverflow)?;
                                            {
                                                let _x_22 = __prod_borrowed_digestFrom(
                                                    (value).as_ref(),
                                                    _x_42,
                                                    _x_1,
                                                    _x_4,
                                                )?;
                                                {
                                                    let _x_23 = crate::Digest {
                                                        w0: _x_7,
                                                        w1: _x_12,
                                                        w2: _x_17,
                                                        w3: _x_22,
                                                    };
                                                    _x_23
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}

pub fn digestFrom(
    x_1: alloc::vec::Vec<u8>,
    x_2: u64,
    x_3: u64,
    x_4: u64,
) -> Result<u64, crate::ComputeError> {
    __prod_borrowed_digestFrom(x_1.as_ref(), x_2, x_3, x_4)
}

fn __prod_borrowed_digestFrom(
    x_1: &[u8],
    mut x_2: u64,
    mut x_3: u64,
    mut x_4: u64,
) -> Result<u64, crate::ComputeError> {
    loop {
        return Ok(match x_3 {
            0 => x_4,
            _ => {
                let n_162 = (x_3).saturating_sub(1);
                {
                    let _x_410 = usize::try_from(x_2)
                        .ok()
                        .and_then(|__index| (x_1).get(__index).cloned());
                    match _x_410 {
                        None => x_4,
                        Some(val_412) => {
                            let _x_414 = 1;
                            {
                                let _x_415 = core::convert::identity::<u64>(x_2)
                                    .checked_add(_x_414)
                                    .ok_or(crate::ComputeError::AddOverflow)?;
                                {
                                    let _x_417 = 256;
                                    {
                                        let _x_418 = core::convert::identity::<u64>(x_4)
                                            .checked_mul(_x_417)
                                            .ok_or(crate::ComputeError::MulOverflow)?;
                                        {
                                            let _x_422 = 128;
                                            {
                                                let _x_423 = 128;
                                                {
                                                    let _x_424 = (val_412 & _x_423);
                                                    {
                                                        let _x_425 = (_x_424 == _x_423);
                                                        {
                                                            let _y_427 = match _x_425 {
                                                                false => {
                                                                    let _x_532 = false;
                                                                    _x_532
                                                                }
                                                                true => {
                                                                    let _x_533 = true;
                                                                    _x_533
                                                                }
                                                            };
                                                            {
                                                                let _y_429 = match _y_427 {
                                                                    false => {
                                                                        let _x_530 = 0;
                                                                        _x_530
                                                                    }
                                                                    true => _x_422,
                                                                };
                                                                {
                                                                    let _x_430 = 64;
                                                                    {
                                                                        let _x_431 = 64;
                                                                        {
                                                                            let _x_432 =
                                                                                (val_412 & _x_431);
                                                                            {
                                                                                let _x_433 = (_x_432
                                                                                    == _x_431);
                                                                                {
                                                                                    let _y_435 = match _x_433 {
        false => { let _x_527 = false; _x_527 },
        true => { let _x_528 = true; _x_528 },
    };
                                                                                    {
                                                                                        let _y_437 = match _y_435 {
        false => { let _x_525 = 0; _x_525 },
        true => _x_430,
    };
                                                                                        {
                                                                                            let _x_438 = core::convert::identity::<u64>(_y_429).checked_add(_y_437).ok_or(crate::ComputeError::AddOverflow)?;
                                                                                            {
                                                                                                let _x_439 = 32;
                                                                                                {
                                                                                                    let _x_440 = 32;
                                                                                                    {
                                                                                                        let _x_441 = (val_412 & _x_440);
                                                                                                        {
                                                                                                            let _x_442 = (_x_441 == _x_440);
                                                                                                            {
                                                                                                                let _y_444 = match _x_442 {
        false => { let _x_522 = false; _x_522 },
        true => { let _x_523 = true; _x_523 },
    };
                                                                                                                {
                                                                                                                    let _y_446 = match _y_444 {
        false => { let _x_520 = 0; _x_520 },
        true => _x_439,
    };
                                                                                                                    {
                                                                                                                        let _x_447 = 16;
                                                                                                                        {
                                                                                                                            let _x_448 = 16;
                                                                                                                            {
                                                                                                                                let _x_449 = (val_412 & _x_448);
                                                                                                                                {
                                                                                                                                    let _x_450 = (_x_449 == _x_448);
                                                                                                                                    {
                                                                                                                                        let _y_452 = match _x_450 {
        false => { let _x_517 = false; _x_517 },
        true => { let _x_518 = true; _x_518 },
    };
                                                                                                                                        {
                                                                                                                                            let _y_454 = match _y_452 {
        false => { let _x_515 = 0; _x_515 },
        true => _x_447,
    };
                                                                                                                                            {
                                                                                                                                                let _x_455 = core::convert::identity::<u64>(_y_446).checked_add(_y_454).ok_or(crate::ComputeError::AddOverflow)?;
                                                                                                                                                {
                                                                                                                                                    let _x_456 = core::convert::identity::<u64>(_x_438).checked_add(_x_455).ok_or(crate::ComputeError::AddOverflow)?;
                                                                                                                                                    {
                                                                                                                                                        let _x_457 = 8;
                                                                                                                                                        {
                                                                                                                                                            let _x_458 = 8;
                                                                                                                                                            {
                                                                                                                                                                let _x_459 = (val_412 & _x_458);
                                                                                                                                                                {
                                                                                                                                                                    let _x_460 = (_x_459 == _x_458);
                                                                                                                                                                    {
                                                                                                                                                                        let _y_462 = match _x_460 {
        false => { let _x_512 = false; _x_512 },
        true => { let _x_513 = true; _x_513 },
    };
                                                                                                                                                                        {
                                                                                                                                                                            let _y_464 = match _y_462 {
        false => { let _x_510 = 0; _x_510 },
        true => _x_457,
    };
                                                                                                                                                                            {
                                                                                                                                                                                let _x_465 = 4;
                                                                                                                                                                                {
                                                                                                                                                                                    let _x_466 = 4;
                                                                                                                                                                                    {
                                                                                                                                                                                        let _x_467 = (val_412 & _x_466);
                                                                                                                                                                                        {
                                                                                                                                                                                            let _x_468 = (_x_467 == _x_466);
                                                                                                                                                                                            {
                                                                                                                                                                                                let _y_470 = match _x_468 {
        false => { let _x_507 = false; _x_507 },
        true => { let _x_508 = true; _x_508 },
    };
                                                                                                                                                                                                {
                                                                                                                                                                                                    let _y_472 = match _y_470 {
        false => { let _x_505 = 0; _x_505 },
        true => _x_465,
    };
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let _x_473 = core::convert::identity::<u64>(_y_464).checked_add(_y_472).ok_or(crate::ComputeError::AddOverflow)?;
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let _x_474 = 2;
                                                                                                                                                                                                            {
                                                                                                                                                                                                                let _x_475 = 2;
                                                                                                                                                                                                                {
                                                                                                                                                                                                                    let _x_476 = (val_412 & _x_475);
                                                                                                                                                                                                                    {
                                                                                                                                                                                                                        let _x_477 = (_x_476 == _x_475);
                                                                                                                                                                                                                        {
                                                                                                                                                                                                                            let _y_479 = match _x_477 {
        false => { let _x_502 = false; _x_502 },
        true => { let _x_503 = true; _x_503 },
    };
                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                let _y_481 = match _y_479 {
        false => { let _x_500 = 0; _x_500 },
        true => _x_474,
    };
                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                    let _x_482 = 1;
                                                                                                                                                                                                                                    {
                                                                                                                                                                                                                                        let _x_483 = (val_412 & _x_482);
                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                            let _x_484 = (_x_483 == _x_482);
                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                let _y_486 = match _x_484 {
        false => { let _x_497 = false; _x_497 },
        true => { let _x_498 = true; _x_498 },
    };
                                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                                    let _y_488 = match _y_486 {
        false => { let _x_495 = 0; _x_495 },
        true => _x_414,
    };
                                                                                                                                                                                                                                                    {
                                                                                                                                                                                                                                                        let _x_489 = core::convert::identity::<u64>(_y_481).checked_add(_y_488).ok_or(crate::ComputeError::AddOverflow)?;
                                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                                            let _x_490 = core::convert::identity::<u64>(_x_473).checked_add(_x_489).ok_or(crate::ComputeError::AddOverflow)?;
                                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                                let _x_491 = core::convert::identity::<u64>(_x_456).checked_add(_x_490).ok_or(crate::ComputeError::AddOverflow)?;
                                                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                                                    let _x_492 = core::convert::identity::<u64>(_x_418).checked_add(_x_491).ok_or(crate::ComputeError::AddOverflow)?;
                                                                                                                                                                                                                                                                    {
                                                                                                                                                                                                                                                                        (x_2, x_3, x_4,) = (_x_415, n_162, _x_492,);
                                                                                                                                                                                                                                                                        continue;
                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                }
                                                                                                                                                                                                                            }
                                                                                                                                                                                                                        }
                                                                                                                                                                                                                    }
                                                                                                                                                                                                                }
                                                                                                                                                                                                            }
                                                                                                                                                                                                        }
                                                                                                                                                                                                    }
                                                                                                                                                                                                }
                                                                                                                                                                                            }
                                                                                                                                                                                        }
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });
    }
}

pub fn dispatch(value: alloc::vec::Vec<u8>) -> Result<alloc::vec::Vec<u8>, crate::ComputeError> {
    Ok({
        let _x_47 = (value).len() as u64;
        {
            let _x_2 = 384;
            {
                let _x_5 = (_x_47 == _x_2);
                {
                    let _x_6 = true;
                    match _x_5 {
                        false => {
                            let _x_114 =
                                (alloc::string::String::from("invalid request")).into_bytes();
                            _x_114
                        }
                        true => {
                            let _x_115 = supportsF64Matmul4(_x_6, _x_6, _x_6);
                            match _x_115 {
                                false => {
                                    let _y_117 = _x_115;
                                    match _y_117 {
                                        false => {
                                            let _x_132 = (alloc::string::String::from("drift:"))
                                                .into_bytes();
                                            {
                                                let _x_133 =
                                                    __prod_borrowed_observe((value).as_ref())?;
                                                {
                                                    let _x_134 = drifted(_x_133);
                                                    {
                                                        let _x_135 = {
                                                            let mut __append = (_x_132, _x_134);
                                                            __append.0.extend(__append.1);
                                                            __append.0
                                                        };
                                                        _x_135
                                                    }
                                                }
                                            }
                                        }
                                        true => {
                                            let _x_137 =
                                                (alloc::string::String::from("conformant"))
                                                    .into_bytes();
                                            _x_137
                                        }
                                    }
                                }
                                true => {
                                    let _x_128 = __prod_borrowed_observe((value).as_ref())?;
                                    {
                                        let _x_129 = conformant(_x_128);
                                        {
                                            let prod_local_0 = _x_129;
                                            match prod_local_0 {
                                                false => {
                                                    let prod_local_3 =
                                                        (alloc::string::String::from("drift:"))
                                                            .into_bytes();
                                                    {
                                                        let prod_local_4 = __prod_borrowed_observe(
                                                            (value).as_ref(),
                                                        )?;
                                                        {
                                                            let prod_local_5 =
                                                                drifted(prod_local_4);
                                                            {
                                                                let prod_local_6 = {
                                                                    let mut __append = (
                                                                        prod_local_3,
                                                                        prod_local_5,
                                                                    );
                                                                    __append.0.extend(__append.1);
                                                                    __append.0
                                                                };
                                                                prod_local_6
                                                            }
                                                        }
                                                    }
                                                }
                                                true => {
                                                    let prod_local_8 =
                                                        (alloc::string::String::from("conformant"))
                                                            .into_bytes();
                                                    prod_local_8
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}

pub fn driftEntry(pin: crate::Pin, path: alloc::vec::Vec<u8>) -> alloc::vec::Vec<u8> {
    {
        let _x_1 = pinMatches(pin);
        match _x_1 {
            false => path,
            true => alloc::vec::Vec::<u8>::new(),
        }
    }
}

pub fn drifted(observation: crate::Observation) -> alloc::vec::Vec<u8> {
    {
        let _x_2 = (observation).devcontainer;
        {
            let _x_4 =
                (alloc::string::String::from(" .devcontainer/devcontainer.json")).into_bytes();
            {
                let _x_5 = driftEntry(_x_2, _x_4);
                {
                    let _x_6 = (observation).bootstrap;
                    {
                        let _x_8 =
                            (alloc::string::String::from(" .github/workflows/bootstrap.yml"))
                                .into_bytes();
                        {
                            let _x_9 = driftEntry(_x_6, _x_8);
                            {
                                let _x_10 = (observation).agents;
                                {
                                    let _x_12 =
                                        (alloc::string::String::from(" AGENTS.md")).into_bytes();
                                    {
                                        let _x_13 = driftEntry(_x_10, _x_12);
                                        {
                                            let _x_14 = (observation).verification;
                                            {
                                                let _x_16 = (alloc::string::String::from(
                                                    " VERIFICATION.md",
                                                ))
                                                .into_bytes();
                                                {
                                                    let _x_17 = driftEntry(_x_14, _x_16);
                                                    {
                                                        let _x_18 = (observation).prismpmLock;
                                                        {
                                                            let _x_20 =
                                                                (alloc::string::String::from(
                                                                    " prismpm.lock",
                                                                ))
                                                                .into_bytes();
                                                            {
                                                                let _x_21 =
                                                                    driftEntry(_x_18, _x_20);
                                                                {
                                                                    let _x_22 = (observation)
                                                                        .templateContract;
                                                                    {
                                                                        let _x_24 = (alloc::string::String::from(" template-contract.json")).into_bytes();
                                                                        {
                                                                            let _x_25 = driftEntry(
                                                                                _x_22, _x_24,
                                                                            );
                                                                            {
                                                                                let _x_26 = {
                                                                                    let mut
                                                                                    __append = (
                                                                                        _x_21,
                                                                                        _x_25,
                                                                                    );
                                                                                    __append
                                                                                        .0
                                                                                        .extend(
                                                                                        __append.1,
                                                                                    );
                                                                                    __append.0
                                                                                };
                                                                                {
                                                                                    let _x_27 = {
                                                                                        let mut
                                                                                        __append = (
                                                                                            _x_17,
                                                                                            _x_26,
                                                                                        );
                                                                                        __append.0.extend(__append.1);
                                                                                        __append.0
                                                                                    };
                                                                                    {
                                                                                        let _x_28 = {
                                                                                            let mut __append = (_x_13, _x_27);
                                                                                            __append.0.extend(__append.1);
                                                                                            __append
                                                                                                .0
                                                                                        };
                                                                                        {
                                                                                            let _x_29 = {
                                                                                                let mut __append = (_x_9, _x_28);
                                                                                                __append.0.extend(__append.1);
                                                                                                __append.0
                                                                                            };
                                                                                            {
                                                                                                let _x_30 = { let mut __append = (_x_5, _x_29); __append.0.extend(__append.1); __append.0 };
                                                                                                _x_30
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn observe(value: alloc::vec::Vec<u8>) -> Result<crate::Observation, crate::ComputeError> {
    __prod_borrowed_observe(value.as_ref())
}

fn __prod_borrowed_observe(value: &[u8]) -> Result<crate::Observation, crate::ComputeError> {
    Ok({
        let _x_1 = 0;
        {
            let _x_4 = __prod_borrowed_pinAt((value).as_ref(), _x_1)?;
            {
                let _x_5 = 64;
                {
                    let _x_8 = __prod_borrowed_pinAt((value).as_ref(), _x_5)?;
                    {
                        let _x_9 = 128;
                        {
                            let _x_12 = __prod_borrowed_pinAt((value).as_ref(), _x_9)?;
                            {
                                let _x_13 = 192;
                                {
                                    let _x_16 = __prod_borrowed_pinAt((value).as_ref(), _x_13)?;
                                    {
                                        let _x_17 = 256;
                                        {
                                            let _x_20 =
                                                __prod_borrowed_pinAt((value).as_ref(), _x_17)?;
                                            {
                                                let _x_21 = 320;
                                                {
                                                    let _x_24 = __prod_borrowed_pinAt(
                                                        (value).as_ref(),
                                                        _x_21,
                                                    )?;
                                                    {
                                                        let _x_25 = crate::Observation {
                                                            devcontainer: _x_4,
                                                            bootstrap: _x_8,
                                                            agents: _x_12,
                                                            verification: _x_16,
                                                            prismpmLock: _x_20,
                                                            templateContract: _x_24,
                                                        };
                                                        _x_25
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}

pub fn pinAt(value: alloc::vec::Vec<u8>, offset: u64) -> Result<crate::Pin, crate::ComputeError> {
    __prod_borrowed_pinAt(value.as_ref(), offset)
}

fn __prod_borrowed_pinAt(value: &[u8], offset: u64) -> Result<crate::Pin, crate::ComputeError> {
    Ok({
        let _x_1 = __prod_borrowed_digestAt((value).as_ref(), offset)?;
        {
            let _x_5 = 32;
            {
                let _x_18 = core::convert::identity::<u64>(offset)
                    .checked_add(_x_5)
                    .ok_or(crate::ComputeError::AddOverflow)?;
                {
                    let _x_9 = __prod_borrowed_digestAt((value).as_ref(), _x_18)?;
                    {
                        let _x_10 = crate::Pin {
                            expected: _x_1,
                            observed: _x_9,
                        };
                        _x_10
                    }
                }
            }
        }
    })
}

pub fn pinMatches(pin: crate::Pin) -> bool {
    {
        let _x_1 = (pin).expected;
        {
            let _x_2 = (pin).observed;
            {
                let _x_3 = sameDigest(_x_1, _x_2);
                _x_3
            }
        }
    }
}

pub fn sameDigest(left: crate::Digest, right: crate::Digest) -> bool {
    {
        let _x_31 = (left).w0;
        {
            let _x_32 = (right).w0;
            {
                let _x_33 = (_x_31 == _x_32);
                match _x_33 {
                    false => _x_33,
                    true => {
                        let _x_60 = (left).w1;
                        {
                            let _x_61 = (right).w1;
                            {
                                let _x_62 = (_x_60 == _x_61);
                                match _x_62 {
                                    false => _x_62,
                                    true => {
                                        let _x_70 = (left).w2;
                                        {
                                            let _x_71 = (right).w2;
                                            {
                                                let _x_72 = (_x_70 == _x_71);
                                                match _x_72 {
                                                    false => _x_72,
                                                    true => {
                                                        let _x_76 = (left).w3;
                                                        {
                                                            let _x_77 = (right).w3;
                                                            {
                                                                let _x_78 = (_x_76 == _x_77);
                                                                _x_78
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn byteAt(value: alloc::vec::Vec<u8>, offset: u64) -> Option<u8> {
    __prod_borrowed_byteAt(value.as_ref(), offset)
}

fn __prod_borrowed_byteAt(value: &[u8], offset: u64) -> Option<u8> {
    {
        let _x_2 = usize::try_from(offset)
            .ok()
            .and_then(|__index| (value).get(__index).cloned());
        _x_2
    }
}

pub fn byteLength(value: alloc::vec::Vec<u8>) -> u64 {
    __prod_borrowed_byteLength(value.as_ref())
}

fn __prod_borrowed_byteLength(value: &[u8]) -> u64 {
    {
        let _x_2 = (value).len() as u64;
        _x_2
    }
}
