#![no_std]
#![forbid(unsafe_code)]
use core::cmp::Ordering;

use core::sync::atomic::{AtomicU64, Ordering as Memory};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Overflow;

pub type R<T> = Result<T, Overflow>;

static WORK: AtomicU64 = AtomicU64::new(0);

pub fn tick(units: u64) { WORK.fetch_add(units, Memory::Relaxed); }

pub fn work() -> u64 { WORK.load(Memory::Relaxed) }

pub trait Same {
    fn same(&self, other: &Self) -> bool;
}

pub trait Key {
    fn key(&self, other: &Self) -> Ordering;
}

impl Same for bool {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for bool {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for u8 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for u8 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for u16 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for u16 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for u32 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for u32 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for u64 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for u64 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for i8 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for i8 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for i16 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for i16 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for i32 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for i32 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for i64 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for i64 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for Ordering {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl<A: Key, B: Key> Key for (A, B) {
    fn key(&self, other: &Self) -> Ordering { match self.0.key(&other.0) { Ordering::Equal => self.1.key(&other.1), decided => decided, } }
}

pub fn equal<T: Same>(a: T, b: T) -> bool { a.same(&b) }

pub fn compare<T: Key>(a: T, b: T) -> Ordering { a.key(&b) }

pub fn nat_add(p0: u64, p1: u64) -> R<u64> {
    match u64::checked_add(p0, p1) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<u64, Overflow>(Overflow)
        }
    }
}

pub fn nat_sub(p0: u64, p1: u64) -> u64 {
    u64::saturating_sub(p0, p1)
}

pub fn nat_mul(p0: u64, p1: u64) -> R<u64> {
    match u64::checked_mul(p0, p1) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<u64, Overflow>(Overflow)
        }
    }
}

pub fn nat_quot(p0: u64, p1: u64, p2: u64) -> u64 {
    u64::checked_div(p0, p1).unwrap_or(p2)
}

pub fn nat_rem(p0: u64, p1: u64, p2: u64) -> u64 {
    u64::checked_rem(p0, p1).unwrap_or(p2)
}

pub fn nat_eq(p0: u64, p1: u64) -> bool {
    core::cmp::PartialEq::eq(&p0, &p1)
}

pub fn nat_le(p0: u64, p1: u64) -> bool {
    core::cmp::PartialOrd::le(&p0, &p1)
}

pub fn nat_lt(p0: u64, p1: u64) -> bool {
    core::cmp::PartialOrd::lt(&p0, &p1)
}

pub fn nat_succ(p0: u64) -> R<u64> {
    match u64::checked_add(p0, 1u64) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<u64, Overflow>(Overflow)
        }
    }
}

pub fn int_add(p0: i64, p1: i64) -> R<i64> {
    match i64::checked_add(p0, p1) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<i64, Overflow>(Overflow)
        }
    }
}

pub fn int_sub(p0: i64, p1: i64) -> R<i64> {
    match i64::checked_sub(p0, p1) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<i64, Overflow>(Overflow)
        }
    }
}

pub fn int_mul(p0: i64, p1: i64) -> R<i64> {
    match i64::checked_mul(p0, p1) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<i64, Overflow>(Overflow)
        }
    }
}

pub fn int_neg(p0: i64) -> R<i64> {
    match i64::checked_neg(p0) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<i64, Overflow>(Overflow)
        }
    }
}

pub fn int_quot(p0: i64, p1: i64, p2: i64) -> R<i64> {
    { let value = { let (a, b, z): (i64, i64, i64) = (p0, p1, p2); if b == 0 { Some(z) } else { a.checked_div(b) } }; match value {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<i64, Overflow>(Overflow)
        }
    } }
}

pub fn int_rem(p0: i64, p1: i64, p2: i64) -> i64 {
    { let (a, b, z): (i64, i64, i64) = (p0, p1, p2); if b == 0 { z } else { a.wrapping_rem(b) } }
}

pub fn bool_not(p0: bool) -> bool {
    !p0
}

pub fn bool_and(p0: bool, p1: bool) -> bool {
    if p0 {
        p1
    } else {
        false
    }
}

pub fn bool_or(p0: bool, p1: bool) -> bool {
    if p0 {
        true
    } else {
        p1
    }
}

pub mod fixed_u8 {
    pub fn checked_add(p0: u8, p1: u8) -> Option<u8> {
        u8::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: u8, p1: u8) -> Option<u8> {
        u8::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: u8, p1: u8) -> Option<u8> {
        u8::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: u8, p1: u8) -> Option<u8> {
        u8::checked_div(p0, p1)
    }

    pub fn bit_and(p0: u8, p1: u8) -> u8 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: u8, p1: u8) -> u8 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: u8, p1: u8) -> u8 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: u8) -> u8 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: u8, p1: u32) -> Option<u8> {
        u8::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: u8, p1: u32) -> Option<u8> {
        u8::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<u8> {
        u8::try_from(p0).ok()
    }
}

pub mod fixed_u16 {
    pub fn checked_add(p0: u16, p1: u16) -> Option<u16> {
        u16::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: u16, p1: u16) -> Option<u16> {
        u16::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: u16, p1: u16) -> Option<u16> {
        u16::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: u16, p1: u16) -> Option<u16> {
        u16::checked_div(p0, p1)
    }

    pub fn bit_and(p0: u16, p1: u16) -> u16 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: u16, p1: u16) -> u16 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: u16, p1: u16) -> u16 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: u16) -> u16 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: u16, p1: u32) -> Option<u16> {
        u16::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: u16, p1: u32) -> Option<u16> {
        u16::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<u16> {
        u16::try_from(p0).ok()
    }
}

pub mod fixed_u32 {
    pub fn checked_add(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_div(p0, p1)
    }

    pub fn bit_and(p0: u32, p1: u32) -> u32 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: u32, p1: u32) -> u32 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: u32, p1: u32) -> u32 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: u32) -> u32 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<u32> {
        u32::try_from(p0).ok()
    }
}

pub mod fixed_u64 {
    pub fn checked_add(p0: u64, p1: u64) -> Option<u64> {
        u64::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: u64, p1: u64) -> Option<u64> {
        u64::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: u64, p1: u64) -> Option<u64> {
        u64::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: u64, p1: u64) -> Option<u64> {
        u64::checked_div(p0, p1)
    }

    pub fn bit_and(p0: u64, p1: u64) -> u64 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: u64, p1: u64) -> u64 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: u64, p1: u64) -> u64 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: u64) -> u64 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: u64, p1: u32) -> Option<u64> {
        u64::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: u64, p1: u32) -> Option<u64> {
        u64::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<u64> {
        u64::try_from(p0).ok()
    }
}

pub mod fixed_i8 {
    pub fn checked_add(p0: i8, p1: i8) -> Option<i8> {
        i8::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: i8, p1: i8) -> Option<i8> {
        i8::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: i8, p1: i8) -> Option<i8> {
        i8::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: i8, p1: i8) -> Option<i8> {
        i8::checked_div(p0, p1)
    }

    pub fn bit_and(p0: i8, p1: i8) -> i8 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: i8, p1: i8) -> i8 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: i8, p1: i8) -> i8 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: i8) -> i8 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: i8, p1: u32) -> Option<i8> {
        i8::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: i8, p1: u32) -> Option<i8> {
        i8::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<i8> {
        i8::try_from(p0).ok()
    }
}

pub fn checked_neg_i8(p0: i8) -> Option<i8> {
    i8::checked_neg(p0)
}

pub mod fixed_i16 {
    pub fn checked_add(p0: i16, p1: i16) -> Option<i16> {
        i16::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: i16, p1: i16) -> Option<i16> {
        i16::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: i16, p1: i16) -> Option<i16> {
        i16::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: i16, p1: i16) -> Option<i16> {
        i16::checked_div(p0, p1)
    }

    pub fn bit_and(p0: i16, p1: i16) -> i16 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: i16, p1: i16) -> i16 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: i16, p1: i16) -> i16 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: i16) -> i16 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: i16, p1: u32) -> Option<i16> {
        i16::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: i16, p1: u32) -> Option<i16> {
        i16::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<i16> {
        i16::try_from(p0).ok()
    }
}

pub fn checked_neg_i16(p0: i16) -> Option<i16> {
    i16::checked_neg(p0)
}

pub mod fixed_i32 {
    pub fn checked_add(p0: i32, p1: i32) -> Option<i32> {
        i32::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: i32, p1: i32) -> Option<i32> {
        i32::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: i32, p1: i32) -> Option<i32> {
        i32::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: i32, p1: i32) -> Option<i32> {
        i32::checked_div(p0, p1)
    }

    pub fn bit_and(p0: i32, p1: i32) -> i32 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: i32, p1: i32) -> i32 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: i32, p1: i32) -> i32 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: i32) -> i32 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: i32, p1: u32) -> Option<i32> {
        i32::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: i32, p1: u32) -> Option<i32> {
        i32::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<i32> {
        i32::try_from(p0).ok()
    }
}

pub fn checked_neg_i32(p0: i32) -> Option<i32> {
    i32::checked_neg(p0)
}

pub mod fixed_i64 {
    pub fn checked_add(p0: i64, p1: i64) -> Option<i64> {
        i64::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: i64, p1: i64) -> Option<i64> {
        i64::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: i64, p1: i64) -> Option<i64> {
        i64::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: i64, p1: i64) -> Option<i64> {
        i64::checked_div(p0, p1)
    }

    pub fn bit_and(p0: i64, p1: i64) -> i64 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: i64, p1: i64) -> i64 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: i64, p1: i64) -> i64 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: i64) -> i64 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: i64, p1: u32) -> Option<i64> {
        i64::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: i64, p1: u32) -> Option<i64> {
        i64::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<i64> {
        i64::try_from(p0).ok()
    }
}

pub fn checked_neg_i64(p0: i64) -> Option<i64> {
    i64::checked_neg(p0)
}

#[derive(Clone)]
pub enum Adt0 {
    C0,
    C1(u64),
    C2(u64, u64),
    C3(u64, u64, u64),
    C4(u64, u64, u64, u64, u64),
    C5(u64, u64, u64, u64, u64),
    C6(u64, u64, u64, u64, u64, u64),
    C7(u64, u64, u64, u64, u64, u64, u64),
    C8,
}

#[derive(Clone)]
pub enum Adt1 {
    C0,
    C1,
    C2(u64, Adt0),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64),
    C1(u64, u64),
    C2(u64, u64, u64),
    C3(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt4 {
    C0,
    C1(u64),
}

#[derive(Clone)]
pub enum Adt5 {
    C0,
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(u64, u64, u64),
}

pub fn f0(v0: u64, v1: Adt0) -> R<Adt1> {
    let m1 = v1;
    match m1 {
        Adt0::C0 => {
            let m2 = f1(v0, Adt2::C0(0u64))?;
            match m2 {
                None => {
                    Ok(Adt1::C1)
                }
                Some(v2) => {
                    Ok(Adt1::C2(v2, Adt0::C1(v2)))
                }
            }
        }
        Adt0::C1(v3) => {
            let m3 = f1(v0, Adt2::C0(1u64))?;
            match m3 {
                None => {
                    Ok(Adt1::C1)
                }
                Some(v4) => {
                    Ok(Adt1::C2(v4, Adt0::C2(v3, v4)))
                }
            }
        }
        Adt0::C2(v5, v6) => {
            let m4 = f1(v0, Adt2::C0(2u64))?;
            match m4 {
                None => {
                    Ok(Adt1::C1)
                }
                Some(v7) => {
                    Ok(Adt1::C2(v7, Adt0::C3(v5, v6, v7)))
                }
            }
        }
        Adt0::C3(v8, v9, v10) => {
            let m5 = f1(v0, Adt2::C0(3u64))?;
            match m5 {
                None => {
                    Ok(Adt1::C1)
                }
                Some(v11) => {
                    Ok(Adt1::C2(v11, Adt0::C4(v8, v9, v10, 0u64, 0u64)))
                }
            }
        }
        Adt0::C4(v12, v13, v14, v15, v16) => {
            let m6 = f1(v0, Adt2::C1(v15, v16))?;
            match m6 {
                None => {
                    Ok(Adt1::C1)
                }
                Some(v17) => {
                    Ok(Adt1::C2(v17, {
                        let a9 = {
                            let a7 = v16;
                            let a8 = 1u64;
                            nat_add(a7, a8)?
                        };
                        let a10 = v13;
                        if nat_lt(a9, a10) {
                            Adt0::C4(v12, v13, v14, v15, {
                                let a11 = v16;
                                let a12 = 1u64;
                                nat_add(a11, a12)?
                            })
                        } else {
                            Adt0::C5(v12, v13, v14, v15, 0u64)
                        }
                    }))
                }
            }
        }
        Adt0::C5(v18, v19, v20, v21, v22) => {
            let m13 = f1(v0, Adt2::C2(v21, v22, 0u64))?;
            match m13 {
                None => {
                    Ok(Adt1::C1)
                }
                Some(v23) => {
                    Ok(Adt1::C2(v23, Adt0::C6(v18, v19, v20, v21, v22, v23)))
                }
            }
        }
        Adt0::C6(v24, v25, v26, v27, v28, v29) => {
            let m14 = f1(v0, Adt2::C2(v27, v28, 1u64))?;
            match m14 {
                None => {
                    Ok(Adt1::C1)
                }
                Some(v30) => {
                    Ok(Adt1::C2(v30, {
                        let a15 = v29;
                        let a16 = 0u64;
                        if nat_eq(a15, a16) {
                            f2(v24, v25, v26, v27, v28)?
                        } else {
                            Adt0::C7(v24, v25, v26, v27, v28, v29, 0u64)
                        }
                    }))
                }
            }
        }
        Adt0::C7(v31, v32, v33, v34, v35, v36, v37) => {
            let m17 = f1(v0, Adt2::C3(v34, v35, v37))?;
            match m17 {
                None => {
                    Ok(Adt1::C1)
                }
                Some(v38) => {
                    Ok(Adt1::C2(v38, {
                        let a20 = {
                            let a18 = v37;
                            let a19 = 1u64;
                            nat_add(a18, a19)?
                        };
                        let a21 = v36;
                        if nat_lt(a20, a21) {
                            Adt0::C7(v31, v32, v33, v34, v35, v36, {
                                let a22 = v37;
                                let a23 = 1u64;
                                nat_add(a22, a23)?
                            })
                        } else {
                            f2(v31, v32, v33, v34, v35)?
                        }
                    }))
                }
            }
        }
        Adt0::C8 => {
            Ok(Adt1::C0)
        }
    }
}

pub fn f1(v0: u64, v1: Adt2) -> R<Option<u64>> {
    let a24 = v0;
    let a25 = 0u64;
    if nat_eq(a24, a25) {
        f3(0u64, v1)
    } else {
        let a26 = v0;
        let a27 = 1u64;
        if nat_eq(a26, a27) {
            let m28 = v1;
            match m28 {
                Adt2::C0(v2) => {
                    f4(0u64, 0u64, 0u64, v2)
                }
                Adt2::C1(v3, v4) => {
                    f4(1u64, v3, 0u64, v4)
                }
                Adt2::C2(v5, v6, v7) => {
                    f4(2u64, v5, v6, v7)
                }
                Adt2::C3(v8, v9, v10) => {
                    f4(3u64, v8, v9, v10)
                }
            }
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f2(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt0> {
    let a31 = {
        let a29 = v4;
        let a30 = 1u64;
        nat_add(a29, a30)?
    };
    let a32 = v2;
    if nat_lt(a31, a32) {
        Ok(Adt0::C5(v0, v1, v2, v3, {
            let a33 = v4;
            let a34 = 1u64;
            nat_add(a33, a34)?
        }))
    } else {
        let a37 = {
            let a35 = v3;
            let a36 = 1u64;
            nat_add(a35, a36)?
        };
        let a38 = v0;
        if nat_lt(a37, a38) {
            Ok(Adt0::C4(v0, v1, v2, {
                let a39 = v3;
                let a40 = 1u64;
                nat_add(a39, a40)?
            }, 0u64))
        } else {
            Ok(Adt0::C8)
        }
    }
}

pub fn f3(v0: u64, v1: Adt2) -> R<Option<u64>> {
    let a41 = v0;
    let a42 = 0u64;
    if nat_eq(a41, a42) {
        let m43 = v1;
        match m43 {
            Adt2::C0(v2) => {
                let a44 = v2;
                let a45 = 4u64;
                if nat_lt(a44, a45) {
                    Ok(Some({
                        let a46 = v2;
                        let a47 = 0u64;
                        if nat_eq(a46, a47) {
                            1280u64
                        } else {
                            let a48 = v2;
                            let a49 = 1u64;
                            if nat_eq(a48, a49) {
                                24u64
                            } else {
                                let a50 = v2;
                                let a51 = 2u64;
                                if nat_eq(a50, a51) {
                                    2u64
                                } else {
                                    165u64
                                }
                            }
                        }
                    }))
                } else {
                    Ok(None::<u64>)
                }
            }
            Adt2::C1(v3, v4) => {
                let a52 = v3;
                let a53 = 1280u64;
                if if nat_lt(a52, a53) {
                    let a54 = v4;
                    let a55 = 24u64;
                    nat_lt(a54, a55)
                } else {
                    false
                } {
                    Ok(Some(f5(v3, v4)?))
                } else {
                    Ok(None::<u64>)
                }
            }
            Adt2::C2(v5, v6, v7) => {
                let a56 = v5;
                let a57 = 1280u64;
                if if if nat_lt(a56, a57) {
                    let a58 = v6;
                    let a59 = 2u64;
                    nat_lt(a58, a59)
                } else {
                    false
                } {
                    let a60 = v7;
                    let a61 = 2u64;
                    nat_lt(a60, a61)
                } else {
                    false
                } {
                    Ok(Some({
                        let a62 = v7;
                        let a63 = 0u64;
                        if nat_eq(a62, a63) {
                            f6(v5, v6)?
                        } else {
                            4u64
                        }
                    }))
                } else {
                    Ok(None::<u64>)
                }
            }
            Adt2::C3(v8, v9, v10) => {
                let a64 = v8;
                let a65 = 1280u64;
                if if if nat_lt(a64, a65) {
                    let a66 = v9;
                    let a67 = 2u64;
                    nat_lt(a66, a67)
                } else {
                    false
                } {
                    let a68 = v10;
                    let a69 = f6(v8, v9)?;
                    nat_lt(a68, a69)
                } else {
                    false
                } {
                    Ok(Some(f7(v8, v9, v10)?))
                } else {
                    Ok(None::<u64>)
                }
            }
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f4(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a70 = 38u64;
    let a71 = v0;
    if nat_le(a70, a71) {
        f8({
            let a72 = v0;
            let a73 = 38u64;
            nat_sub(a72, a73)
        }, v1, v2, v3)
    } else {
        let a74 = 31u64;
        let a75 = v0;
        if nat_le(a74, a75) {
            f9(1u64, {
                let a76 = v0;
                let a77 = 31u64;
                nat_sub(a76, a77)
            }, v1, v2, v3)
        } else {
            let a78 = 24u64;
            let a79 = v0;
            if nat_le(a78, a79) {
                f9(0u64, {
                    let a80 = v0;
                    let a81 = 24u64;
                    nat_sub(a80, a81)
                }, v1, v2, v3)
            } else {
                let a82 = 17u64;
                let a83 = v0;
                if nat_le(a82, a83) {
                    f10({
                        let a84 = v0;
                        let a85 = 17u64;
                        nat_sub(a84, a85)
                    }, v1, v2, v3)
                } else {
                    let a86 = 11u64;
                    let a87 = v0;
                    if nat_le(a86, a87) {
                        f11({
                            let a88 = v0;
                            let a89 = 11u64;
                            nat_sub(a88, a89)
                        }, v1, v2, v3)
                    } else {
                        let a90 = 5u64;
                        let a91 = v0;
                        if nat_le(a90, a91) {
                            f12({
                                let a92 = v0;
                                let a93 = 5u64;
                                nat_sub(a92, a93)
                            }, v1, v2, v3)
                        } else {
                            let a94 = v0;
                            let a95 = 2u64;
                            if nat_lt(a94, a95) {
                                let a96 = v0;
                                let a97 = 0u64;
                                if nat_eq(a96, a97) {
                                    let a98 = v3;
                                    let a99 = 4u64;
                                    if nat_lt(a98, a99) {
                                        Ok(Some({
                                            let a100 = v3;
                                            let a101 = 2u64;
                                            if nat_lt(a100, a101) {
                                                let a102 = v3;
                                                let a103 = 1u64;
                                                if nat_lt(a102, a103) {
                                                    120u64
                                                } else {
                                                    24u64
                                                }
                                            } else {
                                                let a104 = v3;
                                                let a105 = 3u64;
                                                if nat_lt(a104, a105) {
                                                    2u64
                                                } else {
                                                    165u64
                                                }
                                            }
                                        }))
                                    } else {
                                        Ok(None::<u64>)
                                    }
                                } else {
                                    let a106 = v1;
                                    let a107 = 120u64;
                                    if if nat_lt(a106, a107) {
                                        let a108 = v3;
                                        let a109 = 24u64;
                                        nat_lt(a108, a109)
                                    } else {
                                        false
                                    } {
                                        Ok(Some(f13(v1, v3)?))
                                    } else {
                                        Ok(None::<u64>)
                                    }
                                }
                            } else {
                                let a110 = v0;
                                let a111 = 4u64;
                                if if nat_lt(a110, a111) {
                                    let a112 = v1;
                                    let a113 = 120u64;
                                    if nat_lt(a112, a113) {
                                        let a114 = v2;
                                        let a115 = 2u64;
                                        nat_lt(a114, a115)
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                } {
                                    let a116 = v0;
                                    let a117 = 2u64;
                                    if nat_eq(a116, a117) {
                                        let a118 = v3;
                                        let a119 = 2u64;
                                        if nat_lt(a118, a119) {
                                            Ok(Some({
                                                let a120 = v3;
                                                let a121 = 0u64;
                                                if nat_eq(a120, a121) {
                                                    f14(v1, v2)?
                                                } else {
                                                    4u64
                                                }
                                            }))
                                        } else {
                                            Ok(None::<u64>)
                                        }
                                    } else {
                                        let a122 = v3;
                                        let a123 = f14(v1, v2)?;
                                        if nat_lt(a122, a123) {
                                            Ok(Some(f15(v2, v3)?))
                                        } else {
                                            Ok(None::<u64>)
                                        }
                                    }
                                } else {
                                    Ok(None::<u64>)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f5(v0: u64, v1: u64) -> R<u64> {
    let a124 = v1;
    let a125 = 0u64;
    if nat_eq(a124, a125) {
        Ok(f16(v0))
    } else {
        let a126 = v1;
        let a127 = 1u64;
        if nat_eq(a126, a127) {
            Ok(f17(v0))
        } else {
            let a128 = v1;
            let a129 = 2u64;
            if nat_eq(a128, a129) {
                Ok(f18(v0))
            } else {
                let a130 = v1;
                let a131 = 3u64;
                if nat_eq(a130, a131) {
                    Ok(f19(v0))
                } else {
                    let a132 = v1;
                    let a133 = 4u64;
                    if nat_eq(a132, a133) {
                        Ok(f20(v0, 0u64))
                    } else {
                        let a134 = v1;
                        let a135 = 5u64;
                        if nat_eq(a134, a135) {
                            Ok(f21(v0, 0u64))
                        } else {
                            let a136 = v1;
                            let a137 = 6u64;
                            if nat_eq(a136, a137) {
                                Ok(f20(v0, 1u64))
                            } else {
                                let a138 = v1;
                                let a139 = 7u64;
                                if nat_eq(a138, a139) {
                                    Ok(f21(v0, 1u64))
                                } else {
                                    let a140 = v1;
                                    let a141 = 16u64;
                                    if nat_lt(a140, a141) {
                                        f22(v0, {
                                            let a144 = {
                                                let a142 = v1;
                                                let a143 = 8u64;
                                                nat_sub(a142, a143)
                                            };
                                            let a145 = 4u64;
                                            let a146 = 0u64;
                                            nat_quot(a144, a145, a146)
                                        }, {
                                            let a149 = {
                                                let a147 = v1;
                                                let a148 = 8u64;
                                                nat_sub(a147, a148)
                                            };
                                            let a150 = 4u64;
                                            let a151 = 0u64;
                                            nat_rem(a149, a150, a151)
                                        })
                                    } else {
                                        let a152 = v1;
                                        let a153 = 16u64;
                                        if nat_eq(a152, a153) {
                                            Ok(f23(v0))
                                        } else {
                                            let a154 = v1;
                                            let a155 = 17u64;
                                            if nat_eq(a154, a155) {
                                                f24(v0)
                                            } else {
                                                let a156 = v1;
                                                let a157 = 18u64;
                                                if nat_eq(a156, a157) {
                                                    f25(v0)
                                                } else {
                                                    let a158 = v1;
                                                    let a159 = 19u64;
                                                    if nat_eq(a158, a159) {
                                                        f26(v0)
                                                    } else {
                                                        let a160 = v1;
                                                        let a161 = 20u64;
                                                        if nat_eq(a160, a161) {
                                                            f6(v0, 0u64)
                                                        } else {
                                                            let a162 = v1;
                                                            let a163 = 21u64;
                                                            if nat_eq(a162, a163) {
                                                                f6(v0, 1u64)
                                                            } else {
                                                                let a164 = v1;
                                                                let a165 = 22u64;
                                                                if nat_eq(a164, a165) {
                                                                    let a166 = v0;
                                                                    let a167 = 416u64;
                                                                    if nat_eq(a166, a167) {
                                                                        Ok(1u64)
                                                                    } else {
                                                                        Ok(0u64)
                                                                    }
                                                                } else {
                                                                    Ok(v0)
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

pub fn f6(v0: u64, v1: u64) -> R<u64> {
    let a172 = {
        let a170 = {
            let a168 = f18(v0);
            let a169 = f27(v0, v1);
            nat_mul(a168, a169)?
        };
        let a171 = f20(v0, v1);
        nat_mul(a170, a171)?
    };
    let a173 = f21(v0, v1);
    nat_mul(a172, a173)
}

pub fn f7(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a174 = v2;
        let a175 = f18(v0);
        let a176 = 0u64;
        nat_rem(a174, a175, a176)
    };
    let v4: u64 = {
        let a177 = v2;
        let a178 = f18(v0);
        let a179 = 0u64;
        nat_quot(a177, a178, a179)
    };
    let v5: u64 = {
        let a180 = v4;
        let a181 = f27(v0, v1);
        let a182 = 0u64;
        nat_rem(a180, a181, a182)
    };
    let v6: u64 = {
        let a183 = v4;
        let a184 = f27(v0, v1);
        let a185 = 0u64;
        nat_quot(a183, a184, a185)
    };
    let v7: u64 = {
        let a186 = v6;
        let a187 = f20(v0, v1);
        let a188 = 0u64;
        nat_rem(a186, a187, a188)
    };
    let v8: u64 = {
        let a189 = v6;
        let a190 = f20(v0, v1);
        let a191 = 0u64;
        nat_quot(a189, a190, a191)
    };
    let a192 = f16(v0);
    let a193 = 1u64;
    if if if nat_eq(a192, a193) {
        let a194 = f17(v0);
        let a195 = 1u64;
        nat_eq(a194, a195)
    } else {
        false
    } {
        let a196 = f18(v0);
        let a197 = 1u64;
        nat_eq(a196, a197)
    } else {
        false
    } {
        f28(v0, v1)
    } else {
        Ok(f29({
            let a216 = {
                let a214 = {
                    let a210 = {
                        let a206 = {
                            let a202 = {
                                let a198 = v3;
                                let a199 = 7u64;
                                nat_mul(a198, a199)?
                            };
                            let a203 = {
                                let a200 = v5;
                                let a201 = 11u64;
                                nat_mul(a200, a201)?
                            };
                            nat_add(a202, a203)?
                        };
                        let a207 = {
                            let a204 = v7;
                            let a205 = 3u64;
                            nat_mul(a204, a205)?
                        };
                        nat_add(a206, a207)?
                    };
                    let a211 = {
                        let a208 = v8;
                        let a209 = 5u64;
                        nat_mul(a208, a209)?
                    };
                    nat_add(a210, a211)?
                };
                let a215 = {
                    let a212 = v1;
                    let a213 = 0u64;
                    if nat_eq(a212, a213) {
                        1u64
                    } else {
                        2u64
                    }
                };
                nat_add(a214, a215)?
            };
            let a217 = 19u64;
            let a218 = 0u64;
            nat_rem(a216, a217, a218)
        }))
    }
}

pub fn f8(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a219 = v0;
    let a220 = 4u64;
    if if nat_eq(a219, a220) {
        let a221 = v1;
        let a222 = 256u64;
        nat_lt(a221, a222)
    } else {
        false
    } {
        f30(v1, v2, v3)
    } else {
        let a223 = v0;
        let a224 = 0u64;
        if nat_eq(a223, a224) {
            let a225 = v1;
            let a226 = 0u64;
            if if if nat_eq(a225, a226) {
                let a227 = v2;
                let a228 = 0u64;
                nat_eq(a227, a228)
            } else {
                false
            } {
                let a229 = v3;
                let a230 = 6u64;
                nat_lt(a229, a230)
            } else {
                false
            } {
                Ok(Some({
                    let a231 = v3;
                    let a232 = 0u64;
                    if nat_eq(a231, a232) {
                        256u64
                    } else {
                        let a233 = v3;
                        let a234 = 1u64;
                        if nat_eq(a233, a234) {
                            10u64
                        } else {
                            let a235 = v3;
                            let a236 = 2u64;
                            if nat_eq(a235, a236) {
                                256u64
                            } else {
                                let a237 = v3;
                                let a238 = 3u64;
                                if nat_eq(a237, a238) {
                                    512u64
                                } else {
                                    let a239 = v3;
                                    let a240 = 4u64;
                                    if nat_eq(a239, a240) {
                                        165u64
                                    } else {
                                        168u64
                                    }
                                }
                            }
                        }
                    }
                }))
            } else {
                Ok(None::<u64>)
            }
        } else {
            let a241 = v1;
            let a242 = 256u64;
            if nat_lt(a241, a242) {
                let a243 = v0;
                let a244 = 1u64;
                if nat_eq(a243, a244) {
                    let a245 = v2;
                    let a246 = 0u64;
                    if if nat_eq(a245, a246) {
                        let a247 = v3;
                        let a248 = 10u64;
                        nat_lt(a247, a248)
                    } else {
                        false
                    } {
                        Ok(Some(f31(v1, v3)))
                    } else {
                        Ok(None::<u64>)
                    }
                } else {
                    let a249 = v0;
                    let a250 = 2u64;
                    if nat_eq(a249, a250) {
                        let a251 = v2;
                        let a252 = 0u64;
                        if if if nat_eq(a251, a252) {
                            true
                        } else {
                            let a253 = v2;
                            let a254 = 1u64;
                            if nat_eq(a253, a254) {
                                true
                            } else {
                                let a255 = v2;
                                let a256 = 3u64;
                                nat_eq(a255, a256)
                            }
                        } {
                            let a257 = v3;
                            let a258 = 10u64;
                            nat_lt(a257, a258)
                        } else {
                            false
                        } {
                            Ok(Some({
                                let a259 = v3;
                                let a260 = 4u64;
                                if nat_lt(a259, a260) {
                                    f32(v1, v3)
                                } else {
                                    let a261 = v3;
                                    let a262 = 8u64;
                                    if nat_lt(a261, a262) {
                                        f33(v1, v2, {
                                            let a263 = v3;
                                            let a264 = 4u64;
                                            nat_sub(a263, a264)
                                        })?
                                    } else {
                                        let a265 = v3;
                                        let a266 = 8u64;
                                        if nat_eq(a265, a266) {
                                            f34(v1, v2)?
                                        } else {
                                            f35(v1)?
                                        }
                                    }
                                }
                            }))
                        } else {
                            Ok(None::<u64>)
                        }
                    } else {
                        let a267 = v0;
                        let a268 = 3u64;
                        if nat_eq(a267, a268) {
                            let a269 = v2;
                            let a270 = 0u64;
                            if if nat_eq(a269, a270) {
                                let a271 = v3;
                                let a272 = 512u64;
                                nat_lt(a271, a272)
                            } else {
                                false
                            } {
                                Ok(Some(f36(v1, v3)?))
                            } else {
                                Ok(None::<u64>)
                            }
                        } else {
                            Ok(None::<u64>)
                        }
                    }
                }
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f9(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Option<u64>> {
    let a273 = v1;
    let a274 = 0u64;
    if nat_eq(a273, a274) {
        let a275 = v2;
        let a276 = 0u64;
        if if if nat_eq(a275, a276) {
            let a277 = v3;
            let a278 = 0u64;
            nat_eq(a277, a278)
        } else {
            false
        } {
            let a279 = v4;
            let a280 = 5u64;
            nat_lt(a279, a280)
        } else {
            false
        } {
            Ok(Some({
                let a281 = v4;
                let a282 = 2u64;
                if nat_lt(a281, a282) {
                    let a283 = v4;
                    let a284 = 1u64;
                    if nat_lt(a283, a284) {
                        f37(v0)
                    } else {
                        10u64
                    }
                } else {
                    let a285 = v4;
                    let a286 = 3u64;
                    if nat_lt(a285, a286) {
                        let a287 = v0;
                        let a288 = 0u64;
                        if nat_eq(a287, a288) {
                            8u64
                        } else {
                            3u64
                        }
                    } else {
                        let a289 = v4;
                        let a290 = 4u64;
                        if nat_lt(a289, a290) {
                            f38(v0)
                        } else {
                            165u64
                        }
                    }
                }
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        let a291 = v2;
        let a292 = f37(v0);
        if nat_lt(a291, a292) {
            let a293 = v1;
            let a294 = 1u64;
            if nat_eq(a293, a294) {
                let a295 = v3;
                let a296 = 0u64;
                if if nat_eq(a295, a296) {
                    let a297 = v4;
                    let a298 = 10u64;
                    nat_lt(a297, a298)
                } else {
                    false
                } {
                    Ok(Some(f39(v0, v2, v4)?))
                } else {
                    Ok(None::<u64>)
                }
            } else {
                let a299 = v1;
                let a300 = 2u64;
                if nat_eq(a299, a300) {
                    let a301 = v3;
                    let a302 = 2u64;
                    if if if nat_lt(a301, a302) {
                        true
                    } else {
                        let a303 = v3;
                        let a304 = 3u64;
                        nat_eq(a303, a304)
                    } {
                        let a305 = v4;
                        let a306 = 10u64;
                        nat_lt(a305, a306)
                    } else {
                        false
                    } {
                        Ok(Some({
                            let a307 = v4;
                            let a308 = 4u64;
                            if nat_lt(a307, a308) {
                                f40(v0, v2, v3, v4)?
                            } else {
                                let a309 = v4;
                                let a310 = 8u64;
                                if nat_lt(a309, a310) {
                                    f41(v0, v2, v3, {
                                        let a311 = v4;
                                        let a312 = 4u64;
                                        nat_sub(a311, a312)
                                    })?
                                } else {
                                    let a313 = v4;
                                    let a314 = 8u64;
                                    if nat_eq(a313, a314) {
                                        f42(v0, v2, v3)?
                                    } else {
                                        f43(v0, v2, v3)?
                                    }
                                }
                            }
                        }))
                    } else {
                        Ok(None::<u64>)
                    }
                } else {
                    let a315 = v1;
                    let a316 = 3u64;
                    if nat_eq(a315, a316) {
                        let a317 = v3;
                        let a318 = 0u64;
                        if if nat_eq(a317, a318) {
                            let a319 = v4;
                            let a320 = f38(v0);
                            nat_lt(a319, a320)
                        } else {
                            false
                        } {
                            Ok(Some(f44(v0, v2, v4)?))
                        } else {
                            Ok(None::<u64>)
                        }
                    } else {
                        let a321 = v1;
                        let a322 = 4u64;
                        if nat_eq(a321, a322) {
                            let a323 = v4;
                            let a324 = 2u64;
                            if nat_lt(a323, a324) {
                                let a325 = v3;
                                let a326 = 0u64;
                                if nat_eq(a325, a326) {
                                    let a327 = v4;
                                    let a328 = 0u64;
                                    if nat_eq(a327, a328) {
                                        Ok(Some(f45(v0, v2)?))
                                    } else {
                                        Ok(None::<u64>)
                                    }
                                } else {
                                    let a329 = v3;
                                    let a330 = f45(v0, v2)?;
                                    if nat_le(a329, a330) {
                                        Ok(Some({
                                            let v5: u64 = f38(v0);
                                            let v6: u64 = {
                                                let a333 = {
                                                    let a331 = v3;
                                                    let a332 = 1u64;
                                                    nat_sub(a331, a332)
                                                };
                                                let a334 = v5;
                                                nat_mul(a333, a334)?
                                            };
                                            let v7: u64 = {
                                                let a335 = f42(v0, v2, 0u64)?;
                                                let a336 = v6;
                                                nat_sub(a335, a336)
                                            };
                                            let a337 = v4;
                                            let a338 = 0u64;
                                            if nat_eq(a337, a338) {
                                                v6
                                            } else {
                                                let a339 = v7;
                                                let a340 = v5;
                                                if nat_lt(a339, a340) {
                                                    v7
                                                } else {
                                                    v5
                                                }
                                            }
                                        }))
                                    } else {
                                        Ok(None::<u64>)
                                    }
                                }
                            } else {
                                Ok(None::<u64>)
                            }
                        } else {
                            Ok(None::<u64>)
                        }
                    }
                }
            }
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f10(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a341 = v0;
    let a342 = 0u64;
    if nat_eq(a341, a342) {
        let a343 = v1;
        let a344 = 0u64;
        if if if nat_eq(a343, a344) {
            let a345 = v2;
            let a346 = 0u64;
            nat_eq(a345, a346)
        } else {
            false
        } {
            let a347 = v3;
            let a348 = 5u64;
            nat_lt(a347, a348)
        } else {
            false
        } {
            Ok(Some({
                let a349 = v3;
                let a350 = 2u64;
                if nat_lt(a349, a350) {
                    let a351 = v3;
                    let a352 = 0u64;
                    if nat_eq(a351, a352) {
                        10368u64
                    } else {
                        9u64
                    }
                } else {
                    let a353 = v3;
                    let a354 = 4u64;
                    if nat_lt(a353, a354) {
                        let a355 = v3;
                        let a356 = 2u64;
                        if nat_eq(a355, a356) {
                            288u64
                        } else {
                            256u64
                        }
                    } else {
                        165u64
                    }
                }
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        let a357 = v1;
        let a358 = 10368u64;
        if nat_lt(a357, a358) {
            let a359 = v0;
            let a360 = 1u64;
            if nat_eq(a359, a360) {
                let a361 = v2;
                let a362 = 0u64;
                if if nat_eq(a361, a362) {
                    let a363 = v3;
                    let a364 = 9u64;
                    nat_lt(a363, a364)
                } else {
                    false
                } {
                    Ok(Some(f46(v1, v3)?))
                } else {
                    Ok(None::<u64>)
                }
            } else {
                let a365 = v0;
                let a366 = 2u64;
                if nat_eq(a365, a366) {
                    let a367 = v2;
                    let a368 = 4u64;
                    if if nat_lt(a367, a368) {
                        let a369 = v3;
                        let a370 = 10u64;
                        nat_lt(a369, a370)
                    } else {
                        false
                    } {
                        Ok(Some({
                            let a371 = v3;
                            let a372 = 4u64;
                            if nat_lt(a371, a372) {
                                f47(v1, v2, v3)?
                            } else {
                                let a373 = v3;
                                let a374 = 8u64;
                                if nat_lt(a373, a374) {
                                    f48(v1, v2, {
                                        let a375 = v3;
                                        let a376 = 4u64;
                                        nat_sub(a375, a376)
                                    })?
                                } else {
                                    let a377 = v3;
                                    let a378 = 8u64;
                                    if nat_eq(a377, a378) {
                                        f49(v1, v2)?
                                    } else {
                                        f50(v1, v2)?
                                    }
                                }
                            }
                        }))
                    } else {
                        Ok(None::<u64>)
                    }
                } else {
                    let a379 = v0;
                    let a380 = 3u64;
                    if nat_eq(a379, a380) {
                        let a381 = v2;
                        let a382 = 2u64;
                        if if nat_lt(a381, a382) {
                            let a383 = v3;
                            let a384 = 256u64;
                            nat_lt(a383, a384)
                        } else {
                            false
                        } {
                            Ok(Some(f51(v1, v2, v3)?))
                        } else {
                            Ok(None::<u64>)
                        }
                    } else {
                        let a385 = v0;
                        let a386 = 4u64;
                        if if nat_eq(a385, a386) {
                            true
                        } else {
                            let a387 = v0;
                            let a388 = 5u64;
                            nat_eq(a387, a388)
                        } {
                            let a389 = v3;
                            let a390 = 2u64;
                            if nat_lt(a389, a390) {
                                let a391 = v2;
                                let a392 = 0u64;
                                if nat_eq(a391, a392) {
                                    let a393 = v3;
                                    let a394 = 0u64;
                                    if nat_eq(a393, a394) {
                                        Ok(Some(f52(v1, {
                                            let a395 = v0;
                                            let a396 = 4u64;
                                            if nat_eq(a395, a396) {
                                                0u64
                                            } else {
                                                1u64
                                            }
                                        })?))
                                    } else {
                                        Ok(None::<u64>)
                                    }
                                } else {
                                    let a399 = v2;
                                    let a400 = f52(v1, {
                                        let a397 = v0;
                                        let a398 = 4u64;
                                        if nat_eq(a397, a398) {
                                            0u64
                                        } else {
                                            1u64
                                        }
                                    })?;
                                    if nat_le(a399, a400) {
                                        Ok(Some({
                                            let a401 = v3;
                                            let a402 = 0u64;
                                            if nat_eq(a401, a402) {
                                                let a405 = {
                                                    let a403 = v2;
                                                    let a404 = 1u64;
                                                    nat_sub(a403, a404)
                                                };
                                                let a406 = 256u64;
                                                nat_mul(a405, a406)?
                                            } else {
                                                let v4: u64 = {
                                                    let a413 = f49(v1, {
                                                        let a407 = v0;
                                                        let a408 = 4u64;
                                                        if nat_eq(a407, a408) {
                                                            0u64
                                                        } else {
                                                            2u64
                                                        }
                                                    })?;
                                                    let a414 = {
                                                        let a411 = {
                                                            let a409 = v2;
                                                            let a410 = 1u64;
                                                            nat_sub(a409, a410)
                                                        };
                                                        let a412 = 256u64;
                                                        nat_mul(a411, a412)?
                                                    };
                                                    nat_sub(a413, a414)
                                                };
                                                let a415 = v4;
                                                let a416 = 256u64;
                                                if nat_lt(a415, a416) {
                                                    v4
                                                } else {
                                                    256u64
                                                }
                                            }
                                        }))
                                    } else {
                                        Ok(None::<u64>)
                                    }
                                }
                            } else {
                                Ok(None::<u64>)
                            }
                        } else {
                            Ok(None::<u64>)
                        }
                    }
                }
            }
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f11(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a417 = v0;
    let a418 = 0u64;
    if nat_eq(a417, a418) {
        let a419 = v1;
        let a420 = 0u64;
        if if if nat_eq(a419, a420) {
            let a421 = v2;
            let a422 = 0u64;
            nat_eq(a421, a422)
        } else {
            false
        } {
            let a423 = v3;
            let a424 = 5u64;
            nat_lt(a423, a424)
        } else {
            false
        } {
            let a425 = v3;
            let a426 = 0u64;
            if nat_eq(a425, a426) {
                Ok(Some(160u64))
            } else {
                f12(0u64, 0u64, 0u64, v3)
            }
        } else {
            Ok(None::<u64>)
        }
    } else {
        let a427 = v1;
        let a428 = 160u64;
        if nat_lt(a427, a428) {
            let a429 = v0;
            let a430 = 2u64;
            if nat_eq(a429, a430) {
                let a431 = v2;
                let a432 = 4u64;
                if if nat_lt(a431, a432) {
                    let a433 = v3;
                    let a434 = 10u64;
                    nat_lt(a433, a434)
                } else {
                    false
                } {
                    Ok(Some({
                        let a435 = v3;
                        let a436 = 4u64;
                        if nat_lt(a435, a436) {
                            f53(v1, v2, v3)?
                        } else {
                            let a437 = v3;
                            let a438 = 8u64;
                            if nat_lt(a437, a438) {
                                f54(v1, v2, {
                                    let a439 = v3;
                                    let a440 = 4u64;
                                    nat_sub(a439, a440)
                                })?
                            } else {
                                let a441 = v3;
                                let a442 = 8u64;
                                if nat_eq(a441, a442) {
                                    f55(v1, v2)?
                                } else {
                                    f56(v1, v2)?
                                }
                            }
                        }
                    }))
                } else {
                    Ok(None::<u64>)
                }
            } else {
                f12(v0, f57(v1)?, v2, v3)
            }
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f12(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a443 = v0;
    let a444 = 0u64;
    if nat_eq(a443, a444) {
        let a445 = v1;
        let a446 = 0u64;
        if if if nat_eq(a445, a446) {
            let a447 = v2;
            let a448 = 0u64;
            nat_eq(a447, a448)
        } else {
            false
        } {
            let a449 = v3;
            let a450 = 5u64;
            nat_lt(a449, a450)
        } else {
            false
        } {
            Ok(Some({
                let a451 = v3;
                let a452 = 2u64;
                if nat_lt(a451, a452) {
                    let a453 = v3;
                    let a454 = 0u64;
                    if nat_eq(a453, a454) {
                        3456u64
                    } else {
                        9u64
                    }
                } else {
                    let a455 = v3;
                    let a456 = 4u64;
                    if nat_lt(a455, a456) {
                        let a457 = v3;
                        let a458 = 2u64;
                        if nat_eq(a457, a458) {
                            144u64
                        } else {
                            256u64
                        }
                    } else {
                        165u64
                    }
                }
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        let a459 = v1;
        let a460 = 3456u64;
        if if nat_lt(a459, a460) {
            f58(v1)?
        } else {
            false
        } {
            let a461 = v0;
            let a462 = 2u64;
            if nat_lt(a461, a462) {
                let a463 = v0;
                let a464 = 1u64;
                if nat_eq(a463, a464) {
                    let a465 = v2;
                    let a466 = 0u64;
                    if if nat_eq(a465, a466) {
                        let a467 = v3;
                        let a468 = 9u64;
                        nat_lt(a467, a468)
                    } else {
                        false
                    } {
                        Ok(Some(f59(v1, v3)?))
                    } else {
                        Ok(None::<u64>)
                    }
                } else {
                    Ok(None::<u64>)
                }
            } else {
                let a469 = v0;
                let a470 = 4u64;
                if nat_lt(a469, a470) {
                    let a471 = v0;
                    let a472 = 2u64;
                    if nat_eq(a471, a472) {
                        let a473 = v2;
                        let a474 = 2u64;
                        if if nat_lt(a473, a474) {
                            let a475 = v3;
                            let a476 = 10u64;
                            nat_lt(a475, a476)
                        } else {
                            false
                        } {
                            Ok(Some({
                                let a477 = v3;
                                let a478 = 4u64;
                                if nat_lt(a477, a478) {
                                    f60(v1, v2, v3)?
                                } else {
                                    let a479 = v3;
                                    let a480 = 8u64;
                                    if nat_lt(a479, a480) {
                                        f61(v1, v2, {
                                            let a481 = v3;
                                            let a482 = 4u64;
                                            nat_sub(a481, a482)
                                        })?
                                    } else {
                                        let a483 = v3;
                                        let a484 = 8u64;
                                        if nat_eq(a483, a484) {
                                            f62(v1, v2)?
                                        } else {
                                            f63(v1, v2)?
                                        }
                                    }
                                }
                            }))
                        } else {
                            Ok(None::<u64>)
                        }
                    } else {
                        let a485 = v2;
                        let a486 = 0u64;
                        if if nat_eq(a485, a486) {
                            let a487 = v3;
                            let a488 = 256u64;
                            nat_lt(a487, a488)
                        } else {
                            false
                        } {
                            Ok(Some(f64(v1, v3)?))
                        } else {
                            Ok(None::<u64>)
                        }
                    }
                } else {
                    let a489 = v0;
                    let a490 = 4u64;
                    if nat_eq(a489, a490) {
                        let a491 = v3;
                        let a492 = 2u64;
                        if nat_lt(a491, a492) {
                            let a493 = v2;
                            let a494 = 0u64;
                            if nat_eq(a493, a494) {
                                let a495 = v3;
                                let a496 = 0u64;
                                if nat_eq(a495, a496) {
                                    Ok(Some(f65(v1)?))
                                } else {
                                    Ok(None::<u64>)
                                }
                            } else {
                                let a497 = v2;
                                let a498 = f65(v1)?;
                                if nat_le(a497, a498) {
                                    Ok(Some({
                                        let a499 = v3;
                                        let a500 = 0u64;
                                        if nat_eq(a499, a500) {
                                            let a503 = {
                                                let a501 = v2;
                                                let a502 = 1u64;
                                                nat_sub(a501, a502)
                                            };
                                            let a504 = 256u64;
                                            nat_mul(a503, a504)?
                                        } else {
                                            let v4: u64 = {
                                                let a509 = f62(v1, 0u64)?;
                                                let a510 = {
                                                    let a507 = {
                                                        let a505 = v2;
                                                        let a506 = 1u64;
                                                        nat_sub(a505, a506)
                                                    };
                                                    let a508 = 256u64;
                                                    nat_mul(a507, a508)?
                                                };
                                                nat_sub(a509, a510)
                                            };
                                            let a511 = v4;
                                            let a512 = 256u64;
                                            if nat_lt(a511, a512) {
                                                v4
                                            } else {
                                                256u64
                                            }
                                        }
                                    }))
                                } else {
                                    Ok(None::<u64>)
                                }
                            }
                        } else {
                            Ok(None::<u64>)
                        }
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f13(v0: u64, v1: u64) -> R<u64> {
    let a513 = v1;
    let a514 = 12u64;
    if nat_lt(a513, a514) {
        let a515 = v1;
        let a516 = 4u64;
        if nat_lt(a515, a516) {
            let a517 = v1;
            let a518 = 2u64;
            if nat_lt(a517, a518) {
                let a519 = v1;
                let a520 = 1u64;
                if nat_lt(a519, a520) {
                    f66(v0)
                } else {
                    let a521 = f66(v0)?;
                    let a522 = 4u64;
                    if nat_lt(a521, a522) {
                        let a523 = f66(v0)?;
                        let a524 = 2u64;
                        if nat_lt(a523, a524) {
                            let a525 = f66(v0)?;
                            let a526 = 1u64;
                            if nat_lt(a525, a526) {
                                Ok(2u64)
                            } else {
                                Ok(5u64)
                            }
                        } else {
                            let a527 = f66(v0)?;
                            let a528 = 3u64;
                            if nat_lt(a527, a528) {
                                Ok(6u64)
                            } else {
                                Ok(7u64)
                            }
                        }
                    } else {
                        let a529 = f66(v0)?;
                        let a530 = 6u64;
                        if nat_lt(a529, a530) {
                            let a531 = f66(v0)?;
                            let a532 = 5u64;
                            if nat_lt(a531, a532) {
                                Ok(9u64)
                            } else {
                                Ok(8u64)
                            }
                        } else {
                            Ok(89u64)
                        }
                    }
                }
            } else {
                let a533 = v1;
                let a534 = 3u64;
                if nat_lt(a533, a534) {
                    let a535 = f66(v0)?;
                    let a536 = 6u64;
                    if nat_eq(a535, a536) {
                        Ok(2u64)
                    } else {
                        Ok(0u64)
                    }
                } else {
                    Ok(f67(v0))
                }
            }
        } else {
            Ok(f68(v0, {
                let a539 = {
                    let a537 = v1;
                    let a538 = 4u64;
                    nat_sub(a537, a538)
                };
                let a540 = 4u64;
                let a541 = 0u64;
                nat_quot(a539, a540, a541)
            }, {
                let a544 = {
                    let a542 = v1;
                    let a543 = 4u64;
                    nat_sub(a542, a543)
                };
                let a545 = 4u64;
                let a546 = 0u64;
                nat_rem(a544, a545, a546)
            }))
        }
    } else {
        let a547 = v1;
        let a548 = 20u64;
        if nat_lt(a547, a548) {
            f69(v0, {
                let a551 = {
                    let a549 = v1;
                    let a550 = 12u64;
                    nat_sub(a549, a550)
                };
                let a552 = 4u64;
                let a553 = 0u64;
                nat_quot(a551, a552, a553)
            }, {
                let a556 = {
                    let a554 = v1;
                    let a555 = 12u64;
                    nat_sub(a554, a555)
                };
                let a557 = 4u64;
                let a558 = 0u64;
                nat_rem(a556, a557, a558)
            })
        } else {
            let a559 = v1;
            let a560 = 22u64;
            if nat_lt(a559, a560) {
                f14(v0, {
                    let a561 = v1;
                    let a562 = 20u64;
                    nat_sub(a561, a562)
                })
            } else {
                let a563 = v1;
                let a564 = 22u64;
                if nat_eq(a563, a564) {
                    let a565 = v0;
                    let a566 = 0u64;
                    if nat_eq(a565, a566) {
                        Ok(1u64)
                    } else {
                        let a567 = v0;
                        let a568 = 108u64;
                        if nat_eq(a567, a568) {
                            Ok(2u64)
                        } else {
                            Ok(0u64)
                        }
                    }
                } else {
                    Ok(v0)
                }
            }
        }
    }
}

pub fn f14(v0: u64, v1: u64) -> R<u64> {
    let a569 = v1;
    let a570 = 0u64;
    if nat_eq(a569, a570) {
        let a571 = f70(v0);
        let a572 = 12u64;
        nat_mul(a571, a572)
    } else {
        let a573 = 96u64;
        let a574 = v0;
        if nat_le(a573, a574) {
            Ok(0u64)
        } else {
            let a575 = f71(v0);
            let a576 = 2u64;
            if nat_lt(a575, a576) {
                let a577 = f71(v0);
                let a578 = 1u64;
                if nat_lt(a577, a578) {
                    let a579 = f70(v0);
                    let a580 = 12u64;
                    nat_mul(a579, a580)
                } else {
                    Ok(1u64)
                }
            } else {
                let a581 = f71(v0);
                let a582 = 3u64;
                if nat_lt(a581, a582) {
                    Ok(f70(v0))
                } else {
                    Ok(6u64)
                }
            }
        }
    }
}

pub fn f15(v0: u64, v1: u64) -> R<u64> {
    let a583 = v1;
    let a584 = 16u64;
    if nat_lt(a583, a584) {
        Ok(f72(f73(v0, v1)?))
    } else {
        Ok(f74({
            let a585 = f75(v0);
            let a586 = f76(v1)?;
            nat_add(a585, a586)?
        }))
    }
}

pub fn f16(v0: u64) -> u64 {
    let a593 = {
        let a590 = {
            let a587 = v0;
            let a588 = 320u64;
            let a589 = 0u64;
            nat_quot(a587, a588, a589)
        };
        let a591 = 4u64;
        let a592 = 0u64;
        nat_rem(a590, a591, a592)
    };
    let a594 = 0u64;
    if nat_eq(a593, a594) {
        0u64
    } else {
        let a601 = {
            let a598 = {
                let a595 = v0;
                let a596 = 320u64;
                let a597 = 0u64;
                nat_quot(a595, a596, a597)
            };
            let a599 = 4u64;
            let a600 = 0u64;
            nat_rem(a598, a599, a600)
        };
        let a602 = 1u64;
        if nat_eq(a601, a602) {
            1u64
        } else {
            let a609 = {
                let a606 = {
                    let a603 = v0;
                    let a604 = 320u64;
                    let a605 = 0u64;
                    nat_quot(a603, a604, a605)
                };
                let a607 = 4u64;
                let a608 = 0u64;
                nat_rem(a606, a607, a608)
            };
            let a610 = 2u64;
            if nat_eq(a609, a610) {
                2u64
            } else {
                5u64
            }
        }
    }
}

pub fn f17(v0: u64) -> u64 {
    let a617 = {
        let a614 = {
            let a611 = v0;
            let a612 = 80u64;
            let a613 = 0u64;
            nat_quot(a611, a612, a613)
        };
        let a615 = 4u64;
        let a616 = 0u64;
        nat_rem(a614, a615, a616)
    };
    let a618 = 0u64;
    if nat_eq(a617, a618) {
        0u64
    } else {
        let a625 = {
            let a622 = {
                let a619 = v0;
                let a620 = 80u64;
                let a621 = 0u64;
                nat_quot(a619, a620, a621)
            };
            let a623 = 4u64;
            let a624 = 0u64;
            nat_rem(a622, a623, a624)
        };
        let a626 = 1u64;
        if nat_eq(a625, a626) {
            1u64
        } else {
            let a633 = {
                let a630 = {
                    let a627 = v0;
                    let a628 = 80u64;
                    let a629 = 0u64;
                    nat_quot(a627, a628, a629)
                };
                let a631 = 4u64;
                let a632 = 0u64;
                nat_rem(a630, a631, a632)
            };
            let a634 = 2u64;
            if nat_eq(a633, a634) {
                3u64
            } else {
                7u64
            }
        }
    }
}

pub fn f18(v0: u64) -> u64 {
    let a641 = {
        let a638 = {
            let a635 = v0;
            let a636 = 16u64;
            let a637 = 0u64;
            nat_quot(a635, a636, a637)
        };
        let a639 = 5u64;
        let a640 = 0u64;
        nat_rem(a638, a639, a640)
    };
    let a642 = 0u64;
    if nat_eq(a641, a642) {
        0u64
    } else {
        let a649 = {
            let a646 = {
                let a643 = v0;
                let a644 = 16u64;
                let a645 = 0u64;
                nat_quot(a643, a644, a645)
            };
            let a647 = 5u64;
            let a648 = 0u64;
            nat_rem(a646, a647, a648)
        };
        let a650 = 1u64;
        if nat_eq(a649, a650) {
            1u64
        } else {
            let a657 = {
                let a654 = {
                    let a651 = v0;
                    let a652 = 16u64;
                    let a653 = 0u64;
                    nat_quot(a651, a652, a653)
                };
                let a655 = 5u64;
                let a656 = 0u64;
                nat_rem(a654, a655, a656)
            };
            let a658 = 2u64;
            if nat_eq(a657, a658) {
                2u64
            } else {
                let a665 = {
                    let a662 = {
                        let a659 = v0;
                        let a660 = 16u64;
                        let a661 = 0u64;
                        nat_quot(a659, a660, a661)
                    };
                    let a663 = 5u64;
                    let a664 = 0u64;
                    nat_rem(a662, a663, a664)
                };
                let a666 = 3u64;
                if nat_eq(a665, a666) {
                    7u64
                } else {
                    33u64
                }
            }
        }
    }
}

pub fn f19(v0: u64) -> u64 {
    let a670 = {
        let a667 = v0;
        let a668 = 4u64;
        let a669 = 0u64;
        nat_quot(a667, a668, a669)
    };
    let a671 = 4u64;
    let a672 = 0u64;
    nat_rem(a670, a671, a672)
}

pub fn f20(v0: u64, v1: u64) -> u64 {
    let a673 = v1;
    let a674 = 0u64;
    if nat_eq(a673, a674) {
        let a675 = 2u64;
        let a676 = f77(v0);
        if nat_le(a675, a676) {
            2u64
        } else {
            1u64
        }
    } else {
        let a677 = f77(v0);
        let a678 = 0u64;
        if nat_eq(a677, a678) {
            1u64
        } else {
            let a679 = f77(v0);
            let a680 = 1u64;
            if nat_eq(a679, a680) {
                3u64
            } else {
                4u64
            }
        }
    }
}

pub fn f21(v0: u64, v1: u64) -> u64 {
    let a681 = v1;
    let a682 = 0u64;
    if nat_eq(a681, a682) {
        let a683 = f77(v0);
        let a684 = 3u64;
        if nat_eq(a683, a684) {
            2u64
        } else {
            1u64
        }
    } else {
        let a685 = f77(v0);
        let a686 = 0u64;
        if nat_eq(a685, a686) {
            1u64
        } else {
            let a687 = f77(v0);
            let a688 = 3u64;
            if nat_eq(a687, a688) {
                4u64
            } else {
                2u64
            }
        }
    }
}

pub fn f22(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a689 = v2;
    let a690 = 0u64;
    if nat_eq(a689, a690) {
        Ok(4u64)
    } else {
        let a691 = v2;
        let a692 = 1u64;
        if nat_eq(a691, a692) {
            f78(v0)
        } else {
            let a693 = v2;
            let a694 = 2u64;
            if nat_eq(a693, a694) {
                f79(v0, v1)
            } else {
                f80(v0, v1)
            }
        }
    }
}

pub fn f23(v0: u64) -> u64 {
    let a695 = 2u64;
    let a696 = f19(v0);
    if nat_le(a695, a696) {
        8u64
    } else {
        4u64
    }
}

pub fn f24(v0: u64) -> R<u64> {
    let a699 = {
        let a697 = f23(v0);
        let a698 = f16(v0);
        nat_mul(a697, a698)?
    };
    let a700 = f81(v0);
    nat_add(a699, a700)
}

pub fn f25(v0: u64) -> R<u64> {
    let a703 = {
        let a701 = f24(v0)?;
        let a702 = f17(v0);
        nat_mul(a701, a702)?
    };
    let a704 = f81(v0);
    nat_add(a703, a704)
}

pub fn f26(v0: u64) -> R<u64> {
    let a707 = {
        let a705 = f25(v0)?;
        let a706 = f20(v0, 1u64);
        nat_mul(a705, a706)?
    };
    let a708 = f81(v0);
    nat_add(a707, a708)
}

pub fn f27(v0: u64, v1: u64) -> u64 {
    let a709 = v1;
    let a710 = 0u64;
    if nat_eq(a709, a710) {
        f16(v0)
    } else {
        f17(v0)
    }
}

pub fn f28(v0: u64, v1: u64) -> R<u64> {
    let a718 = {
        let a716 = {
            let a714 = {
                let a711 = f19(v0);
                let a712 = 2u64;
                let a713 = 0u64;
                nat_rem(a711, a712, a713)
            };
            let a715 = 4u64;
            nat_mul(a714, a715)?
        };
        let a717 = f77(v0);
        nat_add(a716, a717)?
    };
    let a719 = 0u64;
    if nat_eq(a718, a719) {
        let a720 = v1;
        let a721 = 0u64;
        if nat_eq(a720, a721) {
            Ok(2139095040u64)
        } else {
            Ok(0u64)
        }
    } else {
        let a729 = {
            let a727 = {
                let a725 = {
                    let a722 = f19(v0);
                    let a723 = 2u64;
                    let a724 = 0u64;
                    nat_rem(a722, a723, a724)
                };
                let a726 = 4u64;
                nat_mul(a725, a726)?
            };
            let a728 = f77(v0);
            nat_add(a727, a728)?
        };
        let a730 = 1u64;
        if nat_eq(a729, a730) {
            let a731 = v1;
            let a732 = 0u64;
            if nat_eq(a731, a732) {
                Ok(2139095040u64)
            } else {
                Ok(3212836864u64)
            }
        } else {
            let a740 = {
                let a738 = {
                    let a736 = {
                        let a733 = f19(v0);
                        let a734 = 2u64;
                        let a735 = 0u64;
                        nat_rem(a733, a734, a735)
                    };
                    let a737 = 4u64;
                    nat_mul(a736, a737)?
                };
                let a739 = f77(v0);
                nat_add(a738, a739)?
            };
            let a741 = 2u64;
            if nat_eq(a740, a741) {
                let a742 = v1;
                let a743 = 0u64;
                if nat_eq(a742, a743) {
                    Ok(2143289344u64)
                } else {
                    Ok(1065353216u64)
                }
            } else {
                let a751 = {
                    let a749 = {
                        let a747 = {
                            let a744 = f19(v0);
                            let a745 = 2u64;
                            let a746 = 0u64;
                            nat_rem(a744, a745, a746)
                        };
                        let a748 = 4u64;
                        nat_mul(a747, a748)?
                    };
                    let a750 = f77(v0);
                    nat_add(a749, a750)?
                };
                let a752 = 3u64;
                if nat_eq(a751, a752) {
                    let a753 = v1;
                    let a754 = 0u64;
                    if nat_eq(a753, a754) {
                        Ok(2147483648u64)
                    } else {
                        Ok(1065353216u64)
                    }
                } else {
                    let a762 = {
                        let a760 = {
                            let a758 = {
                                let a755 = f19(v0);
                                let a756 = 2u64;
                                let a757 = 0u64;
                                nat_rem(a755, a756, a757)
                            };
                            let a759 = 4u64;
                            nat_mul(a758, a759)?
                        };
                        let a761 = f77(v0);
                        nat_add(a760, a761)?
                    };
                    let a763 = 4u64;
                    if nat_eq(a762, a763) {
                        let a764 = v1;
                        let a765 = 0u64;
                        if nat_eq(a764, a765) {
                            Ok(1u64)
                        } else {
                            Ok(1065353216u64)
                        }
                    } else {
                        let a773 = {
                            let a771 = {
                                let a769 = {
                                    let a766 = f19(v0);
                                    let a767 = 2u64;
                                    let a768 = 0u64;
                                    nat_rem(a766, a767, a768)
                                };
                                let a770 = 4u64;
                                nat_mul(a769, a770)?
                            };
                            let a772 = f77(v0);
                            nat_add(a771, a772)?
                        };
                        let a774 = 5u64;
                        if nat_eq(a773, a774) {
                            let a775 = v1;
                            let a776 = 0u64;
                            if nat_eq(a775, a776) {
                                Ok(8388608u64)
                            } else {
                                Ok(1056964608u64)
                            }
                        } else {
                            let a784 = {
                                let a782 = {
                                    let a780 = {
                                        let a777 = f19(v0);
                                        let a778 = 2u64;
                                        let a779 = 0u64;
                                        nat_rem(a777, a778, a779)
                                    };
                                    let a781 = 4u64;
                                    nat_mul(a780, a781)?
                                };
                                let a783 = f77(v0);
                                nat_add(a782, a783)?
                            };
                            let a785 = 6u64;
                            if nat_eq(a784, a785) {
                                let a786 = v1;
                                let a787 = 0u64;
                                if nat_eq(a786, a787) {
                                    Ok(2139095039u64)
                                } else {
                                    Ok(1073741824u64)
                                }
                            } else {
                                let a788 = v1;
                                let a789 = 0u64;
                                let _ = nat_eq(a788, a789);
                                Ok(4286578688u64)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f29(v0: u64) -> u64 {
    let a790 = v0;
    let a791 = 0u64;
    if nat_eq(a790, a791) {
        3222274048u64
    } else {
        let a792 = v0;
        let a793 = 1u64;
        if nat_eq(a792, a793) {
            3221225472u64
        } else {
            let a794 = v0;
            let a795 = 2u64;
            if nat_eq(a794, a795) {
                3219128320u64
            } else {
                let a796 = v0;
                let a797 = 3u64;
                if nat_eq(a796, a797) {
                    3217031168u64
                } else {
                    let a798 = v0;
                    let a799 = 4u64;
                    if nat_eq(a798, a799) {
                        3214934016u64
                    } else {
                        let a800 = v0;
                        let a801 = 5u64;
                        if nat_eq(a800, a801) {
                            3212836864u64
                        } else {
                            let a802 = v0;
                            let a803 = 6u64;
                            if nat_eq(a802, a803) {
                                3208642560u64
                            } else {
                                let a804 = v0;
                                let a805 = 7u64;
                                if nat_eq(a804, a805) {
                                    3204448256u64
                                } else {
                                    let a806 = v0;
                                    let a807 = 8u64;
                                    if nat_eq(a806, a807) {
                                        3196059648u64
                                    } else {
                                        let a808 = v0;
                                        let a809 = 9u64;
                                        if nat_eq(a808, a809) {
                                            0u64
                                        } else {
                                            let a810 = v0;
                                            let a811 = 10u64;
                                            if nat_eq(a810, a811) {
                                                1048576000u64
                                            } else {
                                                let a812 = v0;
                                                let a813 = 11u64;
                                                if nat_eq(a812, a813) {
                                                    1056964608u64
                                                } else {
                                                    let a814 = v0;
                                                    let a815 = 12u64;
                                                    if nat_eq(a814, a815) {
                                                        1061158912u64
                                                    } else {
                                                        let a816 = v0;
                                                        let a817 = 13u64;
                                                        if nat_eq(a816, a817) {
                                                            1065353216u64
                                                        } else {
                                                            let a818 = v0;
                                                            let a819 = 14u64;
                                                            if nat_eq(a818, a819) {
                                                                1067450368u64
                                                            } else {
                                                                let a820 = v0;
                                                                let a821 = 15u64;
                                                                if nat_eq(a820, a821) {
                                                                    1069547520u64
                                                                } else {
                                                                    let a822 = v0;
                                                                    let a823 = 16u64;
                                                                    if nat_eq(a822, a823) {
                                                                        1071644672u64
                                                                    } else {
                                                                        let a824 = v0;
                                                                        let a825 = 17u64;
                                                                        if nat_eq(a824, a825) {
                                                                            1073741824u64
                                                                        } else {
                                                                            1074790400u64
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

pub fn f30(v0: u64, v1: u64, v2: u64) -> R<Option<u64>> {
    let v3: u64 = f34(v0, 0u64)?;
    let v4: u64 = {
        let a828 = {
            let a826 = v3;
            let a827 = 511u64;
            nat_add(a826, a827)?
        };
        let a829 = 512u64;
        let a830 = 0u64;
        nat_quot(a828, a829, a830)
    };
    let a831 = v1;
    let a832 = 0u64;
    if nat_eq(a831, a832) {
        let a833 = v2;
        let a834 = 0u64;
        if nat_eq(a833, a834) {
            Ok(Some(v4))
        } else {
            Ok(None::<u64>)
        }
    } else {
        let a835 = v1;
        let a836 = v4;
        if if nat_le(a835, a836) {
            let a837 = v2;
            let a838 = 2u64;
            nat_lt(a837, a838)
        } else {
            false
        } {
            let v5: u64 = {
                let a841 = {
                    let a839 = v1;
                    let a840 = 1u64;
                    nat_sub(a839, a840)
                };
                let a842 = 512u64;
                nat_mul(a841, a842)?
            };
            Ok(Some({
                let a843 = v2;
                let a844 = 0u64;
                if nat_eq(a843, a844) {
                    v5
                } else {
                    let v6: u64 = {
                        let a845 = v3;
                        let a846 = v5;
                        nat_sub(a845, a846)
                    };
                    let a847 = v6;
                    let a848 = 512u64;
                    if nat_lt(a847, a848) {
                        v6
                    } else {
                        512u64
                    }
                }
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f31(v0: u64, v1: u64) -> u64 {
    let a849 = v1;
    let a850 = 0u64;
    if nat_eq(a849, a850) {
        f82(v0)
    } else {
        let a851 = v1;
        let a852 = 1u64;
        if nat_eq(a851, a852) {
            f83(v0)
        } else {
            let a853 = v1;
            let a854 = 2u64;
            if nat_eq(a853, a854) {
                f84(v0)
            } else {
                let a855 = v1;
                let a856 = 3u64;
                if nat_eq(a855, a856) {
                    f85(v0)
                } else {
                    let a857 = v1;
                    let a858 = 4u64;
                    if nat_eq(a857, a858) {
                        f32(v0, 0u64)
                    } else {
                        let a859 = v1;
                        let a860 = 5u64;
                        if nat_eq(a859, a860) {
                            f32(v0, 1u64)
                        } else {
                            let a861 = v1;
                            let a862 = 6u64;
                            if nat_eq(a861, a862) {
                                f32(v0, 3u64)
                            } else {
                                let a863 = v1;
                                let a864 = 7u64;
                                if nat_eq(a863, a864) {
                                    v0
                                } else {
                                    let a865 = v1;
                                    let a866 = 8u64;
                                    if nat_eq(a865, a866) {
                                        f86(v0)
                                    } else {
                                        f87(v0)
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

pub fn f32(v0: u64, v1: u64) -> u64 {
    let a867 = 240u64;
    let a868 = v0;
    if if nat_le(a867, a868) {
        let a874 = v1;
        let a875 = {
            let a871 = {
                let a869 = v0;
                let a870 = 240u64;
                nat_sub(a869, a870)
            };
            let a872 = 4u64;
            let a873 = 0u64;
            nat_rem(a871, a872, a873)
        };
        nat_eq(a874, a875)
    } else {
        false
    } {
        0u64
    } else {
        let a876 = v1;
        let a877 = 0u64;
        if nat_eq(a876, a877) {
            5u64
        } else {
            let a878 = v1;
            let a879 = 1u64;
            if nat_eq(a878, a879) {
                let a880 = f85(v0);
                let a881 = 3u64;
                if nat_eq(a880, a881) {
                    3u64
                } else {
                    2u64
                }
            } else {
                let a882 = v1;
                let a883 = 2u64;
                if nat_eq(a882, a883) {
                    2u64
                } else {
                    1u64
                }
            }
        }
    }
}

pub fn f33(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = f88(v0);
    let v4: u64 = {
        let a884 = f84(v0);
        let a885 = 8u64;
        if if nat_eq(a884, a885) {
            let a886 = v1;
            let a887 = 0u64;
            if nat_eq(a886, a887) {
                true
            } else {
                let a888 = v1;
                let a889 = 3u64;
                if nat_eq(a888, a889) {
                    let a890 = f83(v0);
                    let a891 = 1u64;
                    nat_eq(a890, a891)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let a892 = 2u64;
            let a893 = v3;
            nat_mul(a892, a893)?
        } else {
            v3
        }
    };
    let v5: u64 = {
        let a894 = f32(v0, 0u64);
        let a895 = v4;
        nat_mul(a894, a895)?
    };
    let v6: u64 = f89(v0, v1);
    let v7: u64 = {
        let a896 = f84(v0);
        let a897 = 9u64;
        if if if nat_eq(a896, a897) {
            let a898 = v1;
            let a899 = 3u64;
            if nat_eq(a898, a899) {
                true
            } else {
                let a900 = f83(v0);
                let a901 = 1u64;
                nat_eq(a900, a901)
            }
        } else {
            false
        } {
            true
        } else {
            let a902 = f89(v0, v1);
            let a903 = 4u64;
            nat_eq(a902, a903)
        } {
            0u64
        } else {
            let a904 = v6;
            let a905 = 2u64;
            if nat_eq(a904, a905) {
                let a910 = {
                    let a908 = {
                        let a906 = v5;
                        let a907 = v3;
                        nat_add(a906, a907)?
                    };
                    let a909 = f32(v0, 2u64);
                    nat_mul(a908, a909)?
                };
                let a911 = v3;
                nat_add(a910, a911)?
            } else {
                let a912 = v6;
                let a913 = 3u64;
                if nat_eq(a912, a913) {
                    let a914 = 5u64;
                    let a915 = v5;
                    nat_mul(a914, a915)?
                } else {
                    let a916 = v6;
                    let a917 = 1u64;
                    if nat_eq(a916, a917) {
                        let a918 = v5;
                        let a919 = v3;
                        nat_add(a918, a919)?
                    } else {
                        v5
                    }
                }
            }
        }
    };
    let v8: u64 = {
        let a920 = v6;
        let a921 = 2u64;
        if nat_eq(a920, a921) {
            let a922 = v5;
            let a923 = v3;
            nat_add(a922, a923)?
        } else {
            let a924 = v6;
            let a925 = 3u64;
            if nat_eq(a924, a925) {
                let a926 = 7u64;
                let a927 = v5;
                nat_mul(a926, a927)?
            } else {
                let a928 = v6;
                let a929 = 4u64;
                if nat_eq(a928, a929) {
                    let a930 = v5;
                    let a931 = v3;
                    nat_add(a930, a931)?
                } else {
                    let a936 = {
                        let a932 = v7;
                        let a933 = f32(v0, 1u64);
                        nat_mul(a932, a933)?
                    };
                    let a937 = {
                        let a934 = v6;
                        let a935 = 1u64;
                        if nat_eq(a934, a935) {
                            v3
                        } else {
                            0u64
                        }
                    };
                    nat_add(a936, a937)?
                }
            }
        }
    };
    let a938 = v2;
    let a939 = 0u64;
    if nat_eq(a938, a939) {
        Ok(v4)
    } else {
        let a940 = v2;
        let a941 = 1u64;
        if nat_eq(a940, a941) {
            Ok(v7)
        } else {
            let a942 = v2;
            let a943 = 2u64;
            if nat_eq(a942, a943) {
                Ok(v8)
            } else {
                let a948 = {
                    let a944 = v7;
                    let a945 = f32(v0, 1u64);
                    nat_mul(a944, a945)?
                };
                let a949 = {
                    let a946 = v8;
                    let a947 = f32(v0, 2u64);
                    nat_mul(a946, a947)?
                };
                nat_add(a948, a949)
            }
        }
    }
}

pub fn f34(v0: u64, v1: u64) -> R<u64> {
    let a950 = f35(v0)?;
    let a951 = 0u64;
    if nat_eq(a950, a951) {
        Ok(0u64)
    } else {
        let a974 = {
            let a968 = {
                let a962 = {
                    let a956 = f88(v0);
                    let a957 = {
                        let a954 = {
                            let a952 = f32(v0, 0u64);
                            let a953 = 1u64;
                            nat_sub(a952, a953)
                        };
                        let a955 = f33(v0, v1, 0u64)?;
                        nat_mul(a954, a955)?
                    };
                    nat_add(a956, a957)?
                };
                let a963 = {
                    let a960 = {
                        let a958 = f32(v0, 1u64);
                        let a959 = 1u64;
                        nat_sub(a958, a959)
                    };
                    let a961 = f33(v0, v1, 1u64)?;
                    nat_mul(a960, a961)?
                };
                nat_add(a962, a963)?
            };
            let a969 = {
                let a966 = {
                    let a964 = f32(v0, 2u64);
                    let a965 = 1u64;
                    nat_sub(a964, a965)
                };
                let a967 = f33(v0, v1, 2u64)?;
                nat_mul(a966, a967)?
            };
            nat_add(a968, a969)?
        };
        let a975 = {
            let a972 = {
                let a970 = f32(v0, 3u64);
                let a971 = 1u64;
                nat_sub(a970, a971)
            };
            let a973 = f33(v0, v1, 3u64)?;
            nat_mul(a972, a973)?
        };
        nat_add(a974, a975)
    }
}

pub fn f35(v0: u64) -> R<u64> {
    let a980 = {
        let a976 = f32(v0, 0u64);
        let a977 = f32(v0, 1u64);
        nat_mul(a976, a977)?
    };
    let a981 = {
        let a978 = f32(v0, 2u64);
        let a979 = f32(v0, 3u64);
        nat_mul(a978, a979)?
    };
    nat_mul(a980, a981)
}

pub fn f36(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f88(v0);
    let v3: u64 = {
        let a982 = v1;
        let a983 = v2;
        let a984 = 0u64;
        nat_rem(a982, a983, a984)
    };
    let a997 = {
        let a994 = f90(v0, {
            let a985 = v1;
            let a986 = v2;
            let a987 = 0u64;
            nat_quot(a985, a986, a987)
        })?;
        let a995 = {
            let a988 = v3;
            let a989 = 0u64;
            if nat_eq(a988, a989) {
                1u64
            } else {
                let a990 = v3;
                let a991 = 1u64;
                if nat_eq(a990, a991) {
                    256u64
                } else {
                    let a992 = v3;
                    let a993 = 2u64;
                    if nat_eq(a992, a993) {
                        65536u64
                    } else {
                        16777216u64
                    }
                }
            }
        };
        let a996 = 0u64;
        nat_quot(a994, a995, a996)
    };
    let a998 = 256u64;
    let a999 = 0u64;
    Ok(nat_rem(a997, a998, a999))
}

pub fn f37(v0: u64) -> u64 {
    let a1000 = v0;
    let a1001 = 0u64;
    if nat_eq(a1000, a1001) {
        15360u64
    } else {
        1920u64
    }
}

pub fn f38(v0: u64) -> u64 {
    let a1002 = v0;
    let a1003 = 0u64;
    if nat_eq(a1002, a1003) {
        256u64
    } else {
        192u64
    }
}

pub fn f39(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1004 = v2;
    let a1005 = 5u64;
    if nat_lt(a1004, a1005) {
        let a1006 = v2;
        let a1007 = 2u64;
        if nat_lt(a1006, a1007) {
            let a1008 = v2;
            let a1009 = 1u64;
            if nat_lt(a1008, a1009) {
                Ok(f91(v0, v1))
            } else {
                f92(v0, v1)
            }
        } else {
            let a1010 = v2;
            let a1011 = 3u64;
            if nat_lt(a1010, a1011) {
                Ok(f93(v0, v1))
            } else {
                let a1012 = v2;
                let a1013 = 4u64;
                if nat_lt(a1012, a1013) {
                    Ok(f94(v1))
                } else {
                    f95(v1)
                }
            }
        }
    } else {
        let a1014 = v2;
        let a1015 = 7u64;
        if nat_lt(a1014, a1015) {
            let a1016 = v2;
            let a1017 = 6u64;
            if nat_lt(a1016, a1017) {
                f96(v1)
            } else {
                f97(v1)
            }
        } else {
            let a1018 = v2;
            let a1019 = 8u64;
            if nat_lt(a1018, a1019) {
                f98(v0, v1)
            } else {
                let a1020 = v2;
                let a1021 = 9u64;
                if nat_lt(a1020, a1021) {
                    Ok(0u64)
                } else {
                    let a1022 = v0;
                    let a1023 = 0u64;
                    if nat_eq(a1022, a1023) {
                        Ok(f99({
                            let a1024 = f93(v0, v1);
                            let a1025 = 5u64;
                            let a1026 = 0u64;
                            nat_quot(a1024, a1025, a1026)
                        }))
                    } else {
                        f100(v0, v1)
                    }
                }
            }
        }
    }
}

pub fn f40(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    if if f101(v0, v1, v2)? {
        let a1027 = f94(v1);
        let a1028 = 3u64;
        nat_eq(a1027, a1028)
    } else {
        false
    } {
        let a1029 = v3;
        let a1030 = 2u64;
        if nat_lt(a1029, a1030) {
            let a1031 = v3;
            let a1032 = 0u64;
            if nat_eq(a1031, a1032) {
                Ok(1u64)
            } else {
                f102(v0, v1)
            }
        } else {
            Ok(1u64)
        }
    } else {
        f103(v0, v1, v3)
    }
}

pub fn f41(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1033 = v3;
    let a1034 = 2u64;
    if nat_lt(a1033, a1034) {
        let a1035 = v3;
        let a1036 = 0u64;
        if nat_eq(a1035, a1036) {
            f104(v0, v1, v2)
        } else {
            f105(v0, v1, v2)
        }
    } else {
        let a1037 = v3;
        let a1038 = 2u64;
        if nat_eq(a1037, a1038) {
            f106(v0, v1, v2)
        } else {
            f107(v0, v1, v2)
        }
    }
}

pub fn f42(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let m1039 = f108(Adt3::C0(f40(v0, v1, v2, 0u64)?, f40(v0, v1, v2, 1u64)?, f40(v0, v1, v2, 2u64)?, f40(v0, v1, v2, 3u64)?, f41(v0, v1, v2, 0u64)?, f41(v0, v1, v2, 1u64)?, f41(v0, v1, v2, 2u64)?, f41(v0, v1, v2, 3u64)?, 1u64, f109(v0, v1, v2)?))?;
    match m1039 {
        Adt4::C0 => {
            Ok(0u64)
        }
        Adt4::C1(v3) => {
            Ok(v3)
        }
    }
}

pub fn f43(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1044 = {
        let a1040 = f40(v0, v1, v2, 0u64)?;
        let a1041 = f40(v0, v1, v2, 1u64)?;
        nat_mul(a1040, a1041)?
    };
    let a1045 = {
        let a1042 = f40(v0, v1, v2, 2u64)?;
        let a1043 = f40(v0, v1, v2, 3u64)?;
        nat_mul(a1042, a1043)?
    };
    nat_mul(a1044, a1045)
}

pub fn f44(v0: u64, v1: u64, v2: u64) -> R<u64> {
    f110(v0, f98(v0, v1)?, v2)
}

pub fn f45(v0: u64, v1: u64) -> R<u64> {
    let a1050 = {
        let a1048 = f42(v0, v1, 0u64)?;
        let a1049 = {
            let a1046 = f38(v0);
            let a1047 = 1u64;
            nat_sub(a1046, a1047)
        };
        nat_add(a1048, a1049)?
    };
    let a1051 = f38(v0);
    let a1052 = 0u64;
    Ok(nat_quot(a1050, a1051, a1052))
}

pub fn f46(v0: u64, v1: u64) -> R<u64> {
    let a1053 = v1;
    let a1054 = 0u64;
    if nat_eq(a1053, a1054) {
        f111(v0)
    } else {
        let a1055 = v1;
        let a1056 = 1u64;
        if nat_eq(a1055, a1056) {
            Ok(f112(v0))
        } else {
            let a1057 = v1;
            let a1058 = 2u64;
            if nat_eq(a1057, a1058) {
                Ok(0u64)
            } else {
                let a1059 = v1;
                let a1060 = 3u64;
                if nat_eq(a1059, a1060) {
                    Ok(f113(v0))
                } else {
                    let a1061 = v1;
                    let a1062 = 4u64;
                    if nat_eq(a1061, a1062) {
                        Ok(f114(v0))
                    } else {
                        let a1063 = v1;
                        let a1064 = 5u64;
                        if nat_eq(a1063, a1064) {
                            Ok(f115(v0))
                        } else {
                            let a1065 = v1;
                            let a1066 = 6u64;
                            if nat_eq(a1065, a1066) {
                                f116(v0)
                            } else {
                                let a1067 = v1;
                                let a1068 = 7u64;
                                if nat_eq(a1067, a1068) {
                                    f117(v0, 0u64)
                                } else {
                                    f117(v0, 1u64)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f47(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1069 = v1;
    let a1070 = 2u64;
    if nat_lt(a1069, a1070) {
        let a1071 = v1;
        let a1072 = 0u64;
        if nat_eq(a1071, a1072) {
            f118(v0, v2)
        } else {
            let a1073 = f113(v0);
            let a1074 = 3u64;
            if nat_eq(a1073, a1074) {
                let a1075 = v2;
                let a1076 = 2u64;
                if nat_lt(a1075, a1076) {
                    let a1077 = v2;
                    let a1078 = 0u64;
                    if nat_eq(a1077, a1078) {
                        f119(v0)
                    } else {
                        let a1079 = f120(v0)?;
                        let a1080 = f119(v0)?;
                        let a1081 = 0u64;
                        Ok(nat_quot(a1079, a1080, a1081))
                    }
                } else {
                    Ok(1u64)
                }
            } else {
                f118(v0, v2)
            }
        }
    } else {
        let a1082 = v1;
        let a1083 = 2u64;
        if nat_eq(a1082, a1083) {
            f121(v0, v2)
        } else {
            let a1084 = v2;
            let a1085 = f112(v0);
            if nat_eq(a1084, a1085) {
                let a1086 = f118(v0, v2)?;
                let a1087 = f121(v0, v2)?;
                nat_add(a1086, a1087)
            } else {
                f118(v0, v2)
            }
        }
    }
}

pub fn f48(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1088 = v2;
    let a1089 = 2u64;
    if nat_lt(a1088, a1089) {
        let a1090 = v2;
        let a1091 = 0u64;
        if nat_eq(a1090, a1091) {
            f122(v0, v1)
        } else {
            f123(v0, v1)
        }
    } else {
        let a1092 = v2;
        let a1093 = 2u64;
        if nat_eq(a1092, a1093) {
            f124(v0, v1)
        } else {
            f125(v0, v1)
        }
    }
}

pub fn f49(v0: u64, v1: u64) -> R<u64> {
    let m1094 = f108(Adt3::C0(f47(v0, v1, 0u64)?, f47(v0, v1, 1u64)?, f47(v0, v1, 2u64)?, f47(v0, v1, 3u64)?, f48(v0, v1, 0u64)?, f48(v0, v1, 1u64)?, f48(v0, v1, 2u64)?, f48(v0, v1, 3u64)?, f119(v0)?, f126(v0)?))?;
    match m1094 {
        Adt4::C0 => {
            Ok(0u64)
        }
        Adt4::C1(v2) => {
            Ok(v2)
        }
    }
}

pub fn f50(v0: u64, v1: u64) -> R<u64> {
    let a1099 = {
        let a1095 = f47(v0, v1, 0u64)?;
        let a1096 = f47(v0, v1, 1u64)?;
        nat_mul(a1095, a1096)?
    };
    let a1100 = {
        let a1097 = f47(v0, v1, 2u64)?;
        let a1098 = f47(v0, v1, 3u64)?;
        nat_mul(a1097, a1098)?
    };
    nat_mul(a1099, a1100)
}

pub fn f51(v0: u64, v1: u64, v2: u64) -> R<u64> {
    f127(f117(v0, v1)?, v2)
}

pub fn f52(v0: u64, v1: u64) -> R<u64> {
    let a1105 = {
        let a1103 = f49(v0, {
            let a1101 = v1;
            let a1102 = 0u64;
            if nat_eq(a1101, a1102) {
                0u64
            } else {
                2u64
            }
        })?;
        let a1104 = 255u64;
        nat_add(a1103, a1104)?
    };
    let a1106 = 256u64;
    let a1107 = 0u64;
    Ok(nat_quot(a1105, a1106, a1107))
}

pub fn f53(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1108 = v1;
    let a1109 = 2u64;
    if nat_lt(a1108, a1109) {
        f60(f57(v0)?, v1, v2)
    } else {
        let a1123 = f60(f57(v0)?, 0u64, v2)?;
        let a1124 = {
            let a1110 = v2;
            let a1111 = 2u64;
            if nat_lt(a1110, a1111) {
                let a1112 = v2;
                let a1113 = 0u64;
                if nat_eq(a1112, a1113) {
                    2u64
                } else {
                    let a1117 = {
                        let a1114 = f128(v0)?;
                        let a1115 = 2u64;
                        let a1116 = 0u64;
                        nat_rem(a1114, a1115, a1116)
                    };
                    let a1118 = 1u64;
                    if nat_eq(a1117, a1118) {
                        3u64
                    } else {
                        1u64
                    }
                }
            } else {
                let a1119 = v2;
                let a1120 = 2u64;
                if nat_eq(a1119, a1120) {
                    let a1121 = 2u64;
                    let a1122 = f128(v0)?;
                    if nat_le(a1121, a1122) {
                        2u64
                    } else {
                        1u64
                    }
                } else {
                    2u64
                }
            }
        };
        nat_mul(a1123, a1124)
    }
}

pub fn f54(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1125 = v1;
    let a1126 = 2u64;
    if nat_lt(a1125, a1126) {
        f61(f57(v0)?, v1, v2)
    } else {
        let a1127 = v2;
        let a1128 = 2u64;
        if nat_lt(a1127, a1128) {
            let a1129 = v2;
            let a1130 = 0u64;
            if nat_eq(a1129, a1130) {
                f129(v0)
            } else {
                f130(v0, v1)
            }
        } else {
            let a1131 = v2;
            let a1132 = 2u64;
            if nat_eq(a1131, a1132) {
                f131(v0, v1)
            } else {
                f132(v0, v1)
            }
        }
    }
}

pub fn f55(v0: u64, v1: u64) -> R<u64> {
    let a1133 = v1;
    let a1134 = 2u64;
    if nat_lt(a1133, a1134) {
        f62(f57(v0)?, v1)
    } else {
        let m1135 = f108(Adt3::C0(f53(v0, v1, 0u64)?, f53(v0, v1, 1u64)?, f53(v0, v1, 2u64)?, f53(v0, v1, 3u64)?, f54(v0, v1, 0u64)?, f54(v0, v1, 1u64)?, f54(v0, v1, 2u64)?, f54(v0, v1, 3u64)?, 1u64, f129(v0)?))?;
        match m1135 {
            Adt4::C0 => {
                Ok(0u64)
            }
            Adt4::C1(v2) => {
                Ok(v2)
            }
        }
    }
}

pub fn f56(v0: u64, v1: u64) -> R<u64> {
    let a1140 = {
        let a1136 = f53(v0, v1, 0u64)?;
        let a1137 = f53(v0, v1, 1u64)?;
        nat_mul(a1136, a1137)?
    };
    let a1141 = {
        let a1138 = f53(v0, v1, 2u64)?;
        let a1139 = f53(v0, v1, 3u64)?;
        nat_mul(a1138, a1139)?
    };
    nat_mul(a1140, a1141)
}

pub fn f57(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a1142 = v0;
        let a1143 = 32u64;
        let a1144 = 0u64;
        nat_quot(a1142, a1143, a1144)
    };
    let a1156 = {
        let a1151 = {
            let a1145 = v1;
            let a1146 = 2u64;
            if nat_lt(a1145, a1146) {
                v1
            } else {
                let a1147 = v1;
                let a1148 = 2u64;
                if nat_eq(a1147, a1148) {
                    23u64
                } else {
                    let a1149 = v1;
                    let a1150 = 3u64;
                    if nat_eq(a1149, a1150) {
                        24u64
                    } else {
                        28u64
                    }
                }
            }
        };
        let a1152 = 96u64;
        nat_mul(a1151, a1152)?
    };
    let a1157 = {
        let a1153 = v0;
        let a1154 = 32u64;
        let a1155 = 0u64;
        nat_rem(a1153, a1154, a1155)
    };
    nat_add(a1156, a1157)
}

pub fn f58(v0: u64) -> R<bool> {
    let m1158 = f133(f134(v0)?);
    match m1158 {
        Adt5::C0 => {
            Ok(false)
        }
        Adt5::C1(_) => {
            Ok(true)
        }
    }
}

pub fn f59(v0: u64, v1: u64) -> R<u64> {
    let a1159 = v1;
    let a1160 = 0u64;
    if nat_eq(a1159, a1160) {
        f134(v0)
    } else {
        let a1161 = v1;
        let a1162 = 1u64;
        if nat_eq(a1161, a1162) {
            Ok(f135(v0))
        } else {
            let a1163 = v1;
            let a1164 = 2u64;
            if nat_eq(a1163, a1164) {
                Ok(0u64)
            } else {
                let a1165 = v1;
                let a1166 = 3u64;
                if nat_eq(a1165, a1166) {
                    Ok(f136(v0))
                } else {
                    let a1167 = v1;
                    let a1168 = 4u64;
                    if nat_eq(a1167, a1168) {
                        f137(v0)
                    } else {
                        let a1169 = v1;
                        let a1170 = 5u64;
                        if nat_eq(a1169, a1170) {
                            f138(v0)
                        } else {
                            let a1171 = v1;
                            let a1172 = 6u64;
                            if nat_eq(a1171, a1172) {
                                f139(v0)
                            } else {
                                let a1173 = v1;
                                let a1174 = 7u64;
                                if nat_eq(a1173, a1174) {
                                    f140(v0)
                                } else {
                                    let a1175 = f135(v0);
                                    let a1176 = 1u64;
                                    if if nat_eq(a1175, a1176) {
                                        let a1177 = f136(v0);
                                        let a1178 = 3u64;
                                        nat_eq(a1177, a1178)
                                    } else {
                                        false
                                    } {
                                        Ok(1u64)
                                    } else {
                                        Ok(0u64)
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

pub fn f60(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1179 = v1;
    let a1180 = 1u64;
    if if nat_eq(a1179, a1180) {
        let a1181 = f136(v0);
        let a1182 = 3u64;
        nat_eq(a1181, a1182)
    } else {
        false
    } {
        let a1183 = v2;
        let a1184 = 2u64;
        if nat_lt(a1183, a1184) {
            let a1185 = v2;
            let a1186 = 0u64;
            if nat_eq(a1185, a1186) {
                f141(v0)
            } else {
                let a1191 = {
                    let a1187 = f137(v0)?;
                    let a1188 = f138(v0)?;
                    nat_mul(a1187, a1188)?
                };
                let a1192 = {
                    let a1189 = 2u64;
                    let a1190 = f139(v0)?;
                    nat_mul(a1189, a1190)?
                };
                nat_mul(a1191, a1192)
            }
        } else {
            Ok(1u64)
        }
    } else {
        let a1193 = v2;
        let a1194 = 2u64;
        if nat_lt(a1193, a1194) {
            let a1195 = v2;
            let a1196 = 0u64;
            if nat_eq(a1195, a1196) {
                let a1197 = f141(v0)?;
                let a1198 = f137(v0)?;
                nat_mul(a1197, a1198)
            } else {
                f138(v0)
            }
        } else {
            let a1199 = v2;
            let a1200 = 2u64;
            if nat_eq(a1199, a1200) {
                Ok(2u64)
            } else {
                f139(v0)
            }
        }
    }
}

pub fn f61(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1201 = v2;
    let a1202 = 2u64;
    if nat_lt(a1201, a1202) {
        let a1203 = v2;
        let a1204 = 0u64;
        if nat_eq(a1203, a1204) {
            f142(v0, v1)
        } else {
            f143(v0, v1)
        }
    } else {
        let a1205 = v2;
        let a1206 = 2u64;
        if nat_eq(a1205, a1206) {
            f144(v0, v1)
        } else {
            f145(v0, v1)
        }
    }
}

pub fn f62(v0: u64, v1: u64) -> R<u64> {
    let m1207 = f108(Adt3::C0(f60(v0, v1, 0u64)?, f60(v0, v1, 1u64)?, f60(v0, v1, 2u64)?, f60(v0, v1, 3u64)?, f61(v0, v1, 0u64)?, f61(v0, v1, 1u64)?, f61(v0, v1, 2u64)?, f61(v0, v1, 3u64)?, f141(v0)?, f146(v0)?))?;
    match m1207 {
        Adt4::C0 => {
            Ok(0u64)
        }
        Adt4::C1(v2) => {
            Ok(v2)
        }
    }
}

pub fn f63(v0: u64, v1: u64) -> R<u64> {
    let a1212 = {
        let a1208 = f60(v0, v1, 0u64)?;
        let a1209 = f60(v0, v1, 1u64)?;
        nat_mul(a1208, a1209)?
    };
    let a1213 = {
        let a1210 = f60(v0, v1, 2u64)?;
        let a1211 = f60(v0, v1, 3u64)?;
        nat_mul(a1210, a1211)?
    };
    nat_mul(a1212, a1213)
}

pub fn f64(v0: u64, v1: u64) -> R<u64> {
    f147(f140(v0)?, v1)
}

pub fn f65(v0: u64) -> R<u64> {
    let a1216 = {
        let a1214 = f62(v0, 0u64)?;
        let a1215 = 255u64;
        nat_add(a1214, a1215)?
    };
    let a1217 = 256u64;
    let a1218 = 0u64;
    Ok(nat_quot(a1216, a1217, a1218))
}

pub fn f66(v0: u64) -> R<u64> {
    let a1219 = v0;
    let a1220 = 96u64;
    if nat_lt(a1219, a1220) {
        let a1221 = v0;
        let a1222 = 24u64;
        let a1223 = 0u64;
        Ok(nat_quot(a1221, a1222, a1223))
    } else {
        let a1229 = 4u64;
        let a1230 = {
            let a1226 = {
                let a1224 = v0;
                let a1225 = 96u64;
                nat_sub(a1224, a1225)
            };
            let a1227 = 6u64;
            let a1228 = 0u64;
            nat_quot(a1226, a1227, a1228)
        };
        nat_add(a1229, a1230)
    }
}

pub fn f67(v0: u64) -> u64 {
    let a1231 = v0;
    let a1232 = 96u64;
    if nat_lt(a1231, a1232) {
        let a1236 = {
            let a1233 = v0;
            let a1234 = 4u64;
            let a1235 = 0u64;
            nat_quot(a1233, a1234, a1235)
        };
        let a1237 = 2u64;
        let a1238 = 0u64;
        nat_rem(a1236, a1237, a1238)
    } else {
        let a1239 = v0;
        let a1240 = 2u64;
        let a1241 = 0u64;
        nat_rem(a1239, a1240, a1241)
    }
}

pub fn f68(v0: u64, v1: u64, v2: u64) -> u64 {
    let a1242 = v1;
    let a1243 = 0u64;
    if nat_eq(a1242, a1243) {
        let a1244 = v2;
        let a1245 = 2u64;
        if nat_lt(a1244, a1245) {
            let a1246 = v2;
            let a1247 = 1u64;
            if nat_lt(a1246, a1247) {
                f70(v0)
            } else {
                3u64
            }
        } else {
            2u64
        }
    } else {
        let a1248 = 96u64;
        let a1249 = v0;
        if nat_le(a1248, a1249) {
            0u64
        } else {
            let a1250 = v2;
            let a1251 = 2u64;
            if nat_lt(a1250, a1251) {
                let a1252 = v2;
                let a1253 = 1u64;
                if nat_lt(a1252, a1253) {
                    let a1254 = f71(v0);
                    let a1255 = 1u64;
                    if if nat_eq(a1254, a1255) {
                        true
                    } else {
                        let a1256 = f71(v0);
                        let a1257 = 3u64;
                        nat_eq(a1256, a1257)
                    } {
                        1u64
                    } else {
                        f70(v0)
                    }
                } else {
                    let a1258 = f71(v0);
                    let a1259 = 1u64;
                    if if nat_eq(a1258, a1259) {
                        true
                    } else {
                        let a1260 = f71(v0);
                        let a1261 = 2u64;
                        nat_eq(a1260, a1261)
                    } {
                        1u64
                    } else {
                        3u64
                    }
                }
            } else {
                let a1262 = v2;
                let a1263 = 3u64;
                if nat_lt(a1262, a1263) {
                    let a1264 = f71(v0);
                    let a1265 = 0u64;
                    if nat_eq(a1264, a1265) {
                        2u64
                    } else {
                        1u64
                    }
                } else {
                    let a1266 = f71(v0);
                    let a1267 = 1u64;
                    if if nat_eq(a1266, a1267) {
                        true
                    } else {
                        let a1268 = f71(v0);
                        let a1269 = 2u64;
                        nat_eq(a1268, a1269)
                    } {
                        1u64
                    } else {
                        2u64
                    }
                }
            }
        }
    }
}

pub fn f69(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1270 = v1;
    let a1271 = 1u64;
    if if nat_eq(a1270, a1271) {
        let a1272 = 96u64;
        let a1273 = v0;
        nat_le(a1272, a1273)
    } else {
        false
    } {
        Ok(0u64)
    } else {
        let v3: u64 = f70(v0);
        let v4: u64 = f67(v0);
        let v5: u64 = {
            let a1274 = v1;
            let a1275 = 0u64;
            if nat_eq(a1274, a1275) {
                0u64
            } else {
                f71(v0)
            }
        };
        let a1276 = v5;
        let a1277 = 2u64;
        if nat_lt(a1276, a1277) {
            let a1278 = v5;
            let a1279 = 1u64;
            if nat_lt(a1278, a1279) {
                let a1280 = v2;
                let a1281 = 2u64;
                if nat_lt(a1280, a1281) {
                    let a1282 = v2;
                    let a1283 = 1u64;
                    if nat_lt(a1282, a1283) {
                        Ok(4u64)
                    } else {
                        let a1286 = {
                            let a1284 = v3;
                            let a1285 = v4;
                            nat_add(a1284, a1285)?
                        };
                        let a1287 = 4u64;
                        nat_mul(a1286, a1287)
                    }
                } else {
                    let a1288 = v2;
                    let a1289 = 3u64;
                    if nat_lt(a1288, a1289) {
                        let a1296 = {
                            let a1294 = {
                                let a1290 = v3;
                                let a1291 = 3u64;
                                nat_mul(a1290, a1291)?
                            };
                            let a1295 = {
                                let a1292 = v4;
                                let a1293 = 4u64;
                                nat_mul(a1292, a1293)?
                            };
                            nat_add(a1294, a1295)?
                        };
                        let a1297 = 4u64;
                        nat_mul(a1296, a1297)
                    } else {
                        let a1304 = {
                            let a1302 = {
                                let a1298 = v3;
                                let a1299 = 6u64;
                                nat_mul(a1298, a1299)?
                            };
                            let a1303 = {
                                let a1300 = v4;
                                let a1301 = 9u64;
                                nat_mul(a1300, a1301)?
                            };
                            nat_add(a1302, a1303)?
                        };
                        let a1305 = 4u64;
                        nat_mul(a1304, a1305)
                    }
                }
            } else {
                let a1306 = v2;
                let a1307 = 2u64;
                if nat_lt(a1306, a1307) {
                    let a1308 = v2;
                    let a1309 = 1u64;
                    if nat_lt(a1308, a1309) {
                        Ok(4u64)
                    } else {
                        let a1312 = {
                            let a1310 = 1u64;
                            let a1311 = v4;
                            nat_add(a1310, a1311)?
                        };
                        let a1313 = 4u64;
                        nat_mul(a1312, a1313)
                    }
                } else {
                    let a1314 = v2;
                    let a1315 = 3u64;
                    if nat_lt(a1314, a1315) {
                        let a1320 = {
                            let a1318 = 1u64;
                            let a1319 = {
                                let a1316 = v4;
                                let a1317 = 2u64;
                                nat_mul(a1316, a1317)?
                            };
                            nat_add(a1318, a1319)?
                        };
                        let a1321 = 4u64;
                        nat_mul(a1320, a1321)
                    } else {
                        let a1326 = {
                            let a1324 = 1u64;
                            let a1325 = {
                                let a1322 = v4;
                                let a1323 = 3u64;
                                nat_mul(a1322, a1323)?
                            };
                            nat_add(a1324, a1325)?
                        };
                        let a1327 = 4u64;
                        nat_mul(a1326, a1327)
                    }
                }
            }
        } else {
            let a1328 = v5;
            let a1329 = 3u64;
            if nat_lt(a1328, a1329) {
                let a1330 = v2;
                let a1331 = 2u64;
                if nat_lt(a1330, a1331) {
                    let a1332 = v2;
                    let a1333 = 1u64;
                    if nat_lt(a1332, a1333) {
                        Ok(4u64)
                    } else {
                        let a1336 = {
                            let a1334 = v3;
                            let a1335 = v4;
                            nat_add(a1334, a1335)?
                        };
                        let a1337 = 4u64;
                        nat_mul(a1336, a1337)
                    }
                } else {
                    let a1338 = v2;
                    let a1339 = 3u64;
                    if nat_lt(a1338, a1339) {
                        let a1344 = {
                            let a1342 = v3;
                            let a1343 = {
                                let a1340 = v4;
                                let a1341 = 2u64;
                                nat_mul(a1340, a1341)?
                            };
                            nat_add(a1342, a1343)?
                        };
                        let a1345 = 4u64;
                        nat_mul(a1344, a1345)
                    } else {
                        let a1350 = {
                            let a1348 = v3;
                            let a1349 = {
                                let a1346 = v4;
                                let a1347 = 3u64;
                                nat_mul(a1346, a1347)?
                            };
                            nat_add(a1348, a1349)?
                        };
                        let a1351 = 4u64;
                        nat_mul(a1350, a1351)
                    }
                }
            } else {
                let a1352 = v2;
                let a1353 = 2u64;
                if nat_lt(a1352, a1353) {
                    let a1354 = v2;
                    let a1355 = 1u64;
                    if nat_lt(a1354, a1355) {
                        Ok(4u64)
                    } else {
                        let a1358 = {
                            let a1356 = 1u64;
                            let a1357 = v4;
                            nat_add(a1356, a1357)?
                        };
                        let a1359 = 4u64;
                        nat_mul(a1358, a1359)
                    }
                } else {
                    let a1360 = v2;
                    let a1361 = 3u64;
                    if nat_lt(a1360, a1361) {
                        let a1366 = {
                            let a1364 = 3u64;
                            let a1365 = {
                                let a1362 = v4;
                                let a1363 = 4u64;
                                nat_mul(a1362, a1363)?
                            };
                            nat_add(a1364, a1365)?
                        };
                        let a1367 = 4u64;
                        nat_mul(a1366, a1367)
                    } else {
                        let a1372 = {
                            let a1370 = 3u64;
                            let a1371 = {
                                let a1368 = v4;
                                let a1369 = 5u64;
                                nat_mul(a1368, a1369)?
                            };
                            nat_add(a1370, a1371)?
                        };
                        let a1373 = 4u64;
                        nat_mul(a1372, a1373)
                    }
                }
            }
        }
    }
}

pub fn f70(v0: u64) -> u64 {
    let a1374 = f148(v0);
    let a1375 = 0u64;
    if nat_eq(a1374, a1375) {
        1u64
    } else {
        let a1376 = f148(v0);
        let a1377 = 1u64;
        if nat_eq(a1376, a1377) {
            17u64
        } else {
            257u64
        }
    }
}

pub fn f71(v0: u64) -> u64 {
    let a1378 = v0;
    let a1379 = 4u64;
    let a1380 = 0u64;
    nat_rem(a1378, a1379, a1380)
}

pub fn f72(v0: u64) -> u64 {
    let a1381 = v0;
    let a1382 = 8u64;
    if nat_lt(a1381, a1382) {
        let a1383 = v0;
        let a1384 = 4u64;
        if nat_lt(a1383, a1384) {
            let a1385 = v0;
            let a1386 = 2u64;
            if nat_lt(a1385, a1386) {
                let a1387 = v0;
                let a1388 = 1u64;
                if nat_lt(a1387, a1388) {
                    0u64
                } else {
                    2147483648u64
                }
            } else {
                let a1389 = v0;
                let a1390 = 3u64;
                if nat_lt(a1389, a1390) {
                    1065353216u64
                } else {
                    3212836864u64
                }
            }
        } else {
            let a1391 = v0;
            let a1392 = 6u64;
            if nat_lt(a1391, a1392) {
                let a1393 = v0;
                let a1394 = 5u64;
                if nat_lt(a1393, a1394) {
                    2139095040u64
                } else {
                    4286578688u64
                }
            } else {
                let a1395 = v0;
                let a1396 = 7u64;
                if nat_lt(a1395, a1396) {
                    2143289345u64
                } else {
                    1u64
                }
            }
        }
    } else {
        let a1397 = v0;
        let a1398 = 12u64;
        if nat_lt(a1397, a1398) {
            let a1399 = v0;
            let a1400 = 10u64;
            if nat_lt(a1399, a1400) {
                let a1401 = v0;
                let a1402 = 9u64;
                if nat_lt(a1401, a1402) {
                    2139095039u64
                } else {
                    8388608u64
                }
            } else {
                let a1403 = v0;
                let a1404 = 11u64;
                if nat_lt(a1403, a1404) {
                    8388607u64
                } else {
                    1065353217u64
                }
            }
        } else {
            let a1405 = v0;
            let a1406 = 14u64;
            if nat_lt(a1405, a1406) {
                let a1407 = v0;
                let a1408 = 13u64;
                if nat_lt(a1407, a1408) {
                    1056964608u64
                } else {
                    8388609u64
                }
            } else {
                let a1409 = v0;
                let a1410 = 15u64;
                if nat_lt(a1409, a1410) {
                    2147483649u64
                } else {
                    2139095041u64
                }
            }
        }
    }
}

pub fn f73(v0: u64, v1: u64) -> R<u64> {
    let a1419 = {
        let a1417 = {
            let a1413 = {
                let a1411 = v0;
                let a1412 = 0u64;
                if nat_eq(a1411, a1412) {
                    1u64
                } else {
                    5u64
                }
            };
            let a1414 = v1;
            nat_mul(a1413, a1414)?
        };
        let a1418 = {
            let a1415 = v0;
            let a1416 = 0u64;
            if nat_eq(a1415, a1416) {
                0u64
            } else {
                3u64
            }
        };
        nat_add(a1417, a1418)?
    };
    let a1420 = 16u64;
    let a1421 = 0u64;
    Ok(nat_rem(a1419, a1420, a1421))
}

pub fn f74(v0: u64) -> u64 {
    let a1422 = v0;
    let a1423 = 4294967296u64;
    let a1424 = 0u64;
    nat_rem(a1422, a1423, a1424)
}

pub fn f75(v0: u64) -> u64 {
    let a1425 = v0;
    let a1426 = 0u64;
    if nat_eq(a1425, a1426) {
        2654435769u64
    } else {
        2246822507u64
    }
}

pub fn f76(v0: u64) -> R<u64> {
    let a1427 = 2654435761u64;
    let a1428 = v0;
    nat_mul(a1427, a1428)
}

pub fn f77(v0: u64) -> u64 {
    let a1429 = v0;
    let a1430 = 4u64;
    let a1431 = 0u64;
    nat_rem(a1429, a1430, a1431)
}

pub fn f78(v0: u64) -> R<u64> {
    let a1439 = {
        let a1437 = {
            let a1432 = f18(v0);
            let a1433 = 0u64;
            if nat_eq(a1432, a1433) {
                1u64
            } else {
                f18(v0)
            }
        };
        let a1438 = {
            let a1434 = f19(v0);
            let a1435 = 2u64;
            let a1436 = 0u64;
            nat_rem(a1434, a1435, a1436)
        };
        nat_add(a1437, a1438)?
    };
    let a1440 = 4u64;
    nat_mul(a1439, a1440)
}

pub fn f79(v0: u64, v1: u64) -> R<u64> {
    let a1448 = {
        let a1441 = f78(v0)?;
        let a1442 = f27(v0, v1);
        nat_mul(a1441, a1442)?
    };
    let a1449 = {
        let a1446 = {
            let a1443 = f19(v0);
            let a1444 = 2u64;
            let a1445 = 0u64;
            nat_rem(a1443, a1444, a1445)
        };
        let a1447 = 4u64;
        nat_mul(a1446, a1447)?
    };
    nat_add(a1448, a1449)
}

pub fn f80(v0: u64, v1: u64) -> R<u64> {
    let a1457 = {
        let a1450 = f79(v0, v1)?;
        let a1451 = f20(v0, v1);
        nat_mul(a1450, a1451)?
    };
    let a1458 = {
        let a1455 = {
            let a1452 = f19(v0);
            let a1453 = 2u64;
            let a1454 = 0u64;
            nat_rem(a1452, a1453, a1454)
        };
        let a1456 = 4u64;
        nat_mul(a1455, a1456)?
    };
    nat_add(a1457, a1458)
}

pub fn f81(v0: u64) -> u64 {
    let a1459 = 2u64;
    let a1460 = f19(v0);
    if nat_le(a1459, a1460) {
        4u64
    } else {
        0u64
    }
}

pub fn f82(v0: u64) -> u64 {
    let a1461 = f84(v0);
    let a1462 = 7u64;
    if nat_eq(a1461, a1462) {
        1u64
    } else {
        let a1473 = {
            let a1463 = v0;
            let a1464 = 240u64;
            if nat_lt(a1463, a1464) {
                let a1465 = v0;
                let a1466 = 120u64;
                let a1467 = 0u64;
                nat_quot(a1465, a1466, a1467)
            } else {
                let a1470 = {
                    let a1468 = v0;
                    let a1469 = 240u64;
                    nat_sub(a1468, a1469)
                };
                let a1471 = 8u64;
                let a1472 = 0u64;
                nat_quot(a1470, a1471, a1472)
            }
        };
        let a1474 = 0u64;
        if nat_eq(a1473, a1474) {
            0u64
        } else {
            30u64
        }
    }
}

pub fn f83(v0: u64) -> u64 {
    let a1475 = v0;
    let a1476 = 240u64;
    if nat_lt(a1475, a1476) {
        let a1480 = {
            let a1477 = v0;
            let a1478 = 60u64;
            let a1479 = 0u64;
            nat_quot(a1477, a1478, a1479)
        };
        let a1481 = 2u64;
        let a1482 = 0u64;
        nat_rem(a1480, a1481, a1482)
    } else {
        let a1488 = {
            let a1485 = {
                let a1483 = v0;
                let a1484 = 240u64;
                nat_sub(a1483, a1484)
            };
            let a1486 = 4u64;
            let a1487 = 0u64;
            nat_quot(a1485, a1486, a1487)
        };
        let a1489 = 2u64;
        let a1490 = 0u64;
        nat_rem(a1488, a1489, a1490)
    }
}

pub fn f84(v0: u64) -> u64 {
    let a1491 = v0;
    let a1492 = 240u64;
    if nat_lt(a1491, a1492) {
        let a1493 = v0;
        let a1494 = 12u64;
        let a1495 = 0u64;
        nat_rem(a1493, a1494, a1495)
    } else {
        0u64
    }
}

pub fn f85(v0: u64) -> u64 {
    let a1496 = v0;
    let a1497 = 240u64;
    if nat_lt(a1496, a1497) {
        let a1501 = {
            let a1498 = v0;
            let a1499 = 12u64;
            let a1500 = 0u64;
            nat_quot(a1498, a1499, a1500)
        };
        let a1502 = 5u64;
        let a1503 = 0u64;
        nat_rem(a1501, a1502, a1503)
    } else {
        0u64
    }
}

pub fn f86(v0: u64) -> u64 {
    let v1: u64 = f84(v0);
    let a1504 = v1;
    let a1505 = 3u64;
    if if if nat_eq(a1504, a1505) {
        true
    } else {
        let a1506 = v1;
        let a1507 = 7u64;
        nat_eq(a1506, a1507)
    } {
        true
    } else {
        let a1508 = v1;
        let a1509 = 8u64;
        nat_eq(a1508, a1509)
    } {
        2u64
    } else {
        let a1510 = v1;
        let a1511 = 2u64;
        if if if if if if nat_eq(a1510, a1511) {
            true
        } else {
            let a1512 = v1;
            let a1513 = 4u64;
            nat_eq(a1512, a1513)
        } {
            true
        } else {
            let a1514 = v1;
            let a1515 = 5u64;
            nat_eq(a1514, a1515)
        } {
            true
        } else {
            let a1516 = v1;
            let a1517 = 6u64;
            nat_eq(a1516, a1517)
        } {
            true
        } else {
            let a1518 = v1;
            let a1519 = 9u64;
            nat_eq(a1518, a1519)
        } {
            true
        } else {
            let a1520 = f85(v0);
            let a1521 = 4u64;
            if nat_eq(a1520, a1521) {
                let a1522 = f83(v0);
                let a1523 = 1u64;
                nat_eq(a1522, a1523)
            } else {
                false
            }
        } {
            1u64
        } else {
            0u64
        }
    }
}

pub fn f87(v0: u64) -> u64 {
    let a1524 = f84(v0);
    let a1525 = 2u64;
    if nat_eq(a1524, a1525) {
        2139095040u64
    } else {
        let a1526 = f84(v0);
        let a1527 = 3u64;
        if nat_eq(a1526, a1527) {
            3212836864u64
        } else {
            let a1528 = f84(v0);
            let a1529 = 10u64;
            if nat_eq(a1528, a1529) {
                2147483648u64
            } else {
                let a1530 = f84(v0);
                let a1531 = 1u64;
                if if nat_eq(a1530, a1531) {
                    true
                } else {
                    let a1532 = f84(v0);
                    let a1533 = 11u64;
                    nat_eq(a1532, a1533)
                } {
                    0u64
                } else {
                    897988541u64
                }
            }
        }
    }
}

pub fn f88(v0: u64) -> u64 {
    let a1534 = f82(v0);
    let a1535 = 0u64;
    if nat_eq(a1534, a1535) {
        4u64
    } else {
        2u64
    }
}

pub fn f89(v0: u64, v1: u64) -> u64 {
    let a1536 = v1;
    let a1537 = 1u64;
    if if nat_eq(a1536, a1537) {
        true
    } else {
        let a1538 = v1;
        let a1539 = 3u64;
        if if nat_eq(a1538, a1539) {
            let a1540 = f83(v0);
            let a1541 = 0u64;
            nat_eq(a1540, a1541)
        } else {
            false
        } {
            let a1542 = f85(v0);
            let a1543 = 4u64;
            nat_eq(a1542, a1543)
        } else {
            false
        }
    } {
        0u64
    } else {
        f85(v0)
    }
}

pub fn f90(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f84(v0);
    let a1544 = f82(v0);
    let a1545 = 0u64;
    if nat_eq(a1544, a1545) {
        let a1546 = v2;
        let a1547 = 4u64;
        if nat_eq(a1546, a1547) {
            Ok(2139095040u64)
        } else {
            let a1548 = v2;
            let a1549 = 5u64;
            if if nat_eq(a1548, a1549) {
                true
            } else {
                let a1550 = v2;
                let a1551 = 6u64;
                if nat_eq(a1550, a1551) {
                    let a1557 = v1;
                    let a1558 = {
                        let a1555 = {
                            let a1552 = f34(v0, 0u64)?;
                            let a1553 = f88(v0);
                            let a1554 = 0u64;
                            nat_quot(a1552, a1553, a1554)
                        };
                        let a1556 = 1u64;
                        nat_sub(a1555, a1556)
                    };
                    nat_eq(a1557, a1558)
                } else {
                    false
                }
            } {
                Ok(2143289635u64)
            } else {
                let a1559 = v2;
                let a1560 = 11u64;
                if nat_eq(a1559, a1560) {
                    Ok(1u64)
                } else {
                    let a1561 = v2;
                    let a1562 = 10u64;
                    if nat_eq(a1561, a1562) {
                        Ok(2147483648u64)
                    } else {
                        let a1563 = v2;
                        let a1564 = 1u64;
                        if nat_eq(a1563, a1564) {
                            Ok(0u64)
                        } else {
                            let a1568 = {
                                let a1565 = v1;
                                let a1566 = 5u64;
                                let a1567 = 0u64;
                                nat_rem(a1565, a1566, a1567)
                            };
                            let a1569 = 0u64;
                            if nat_eq(a1568, a1569) {
                                Ok(1166016512u64)
                            } else {
                                let a1573 = {
                                    let a1570 = v1;
                                    let a1571 = 5u64;
                                    let a1572 = 0u64;
                                    nat_rem(a1570, a1571, a1572)
                                };
                                let a1574 = 1u64;
                                if nat_eq(a1573, a1574) {
                                    Ok(1065353216u64)
                                } else {
                                    Ok(0u64)
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        let a1575 = v2;
        let a1576 = 4u64;
        if nat_eq(a1575, a1576) {
            Ok(32640u64)
        } else {
            let a1577 = v2;
            let a1578 = 5u64;
            if if nat_eq(a1577, a1578) {
                true
            } else {
                let a1579 = v2;
                let a1580 = 6u64;
                if nat_eq(a1579, a1580) {
                    let a1586 = v1;
                    let a1587 = {
                        let a1584 = {
                            let a1581 = f34(v0, 0u64)?;
                            let a1582 = f88(v0);
                            let a1583 = 0u64;
                            nat_quot(a1581, a1582, a1583)
                        };
                        let a1585 = 1u64;
                        nat_sub(a1584, a1585)
                    };
                    nat_eq(a1586, a1587)
                } else {
                    false
                }
            } {
                Ok(32705u64)
            } else {
                let a1588 = v2;
                let a1589 = 11u64;
                if nat_eq(a1588, a1589) {
                    Ok(1u64)
                } else {
                    let a1590 = v2;
                    let a1591 = 10u64;
                    if nat_eq(a1590, a1591) {
                        Ok(32768u64)
                    } else {
                        let a1592 = v2;
                        let a1593 = 1u64;
                        if nat_eq(a1592, a1593) {
                            Ok(0u64)
                        } else {
                            let a1597 = {
                                let a1594 = v1;
                                let a1595 = 5u64;
                                let a1596 = 0u64;
                                nat_rem(a1594, a1595, a1596)
                            };
                            let a1598 = 0u64;
                            if nat_eq(a1597, a1598) {
                                Ok(17792u64)
                            } else {
                                let a1602 = {
                                    let a1599 = v1;
                                    let a1600 = 5u64;
                                    let a1601 = 0u64;
                                    nat_rem(a1599, a1600, a1601)
                                };
                                let a1603 = 1u64;
                                if nat_eq(a1602, a1603) {
                                    Ok(16256u64)
                                } else {
                                    Ok(0u64)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f91(v0: u64, v1: u64) -> u64 {
    f149(f150(v0, v1))
}

pub fn f92(v0: u64, v1: u64) -> R<u64> {
    let a1604 = v0;
    let a1605 = 0u64;
    if nat_eq(a1604, a1605) {
        let a1609 = {
            let a1606 = v1;
            let a1607 = 3840u64;
            let a1608 = 0u64;
            nat_quot(a1606, a1607, a1608)
        };
        let a1610 = 2u64;
        let a1611 = 0u64;
        Ok(nat_rem(a1609, a1610, a1611))
    } else {
        let v2: u64 = {
            let a1615 = {
                let a1612 = v1;
                let a1613 = 160u64;
                let a1614 = 0u64;
                nat_quot(a1612, a1613, a1614)
            };
            let a1616 = 4u64;
            let a1617 = 0u64;
            nat_rem(a1615, a1616, a1617)
        };
        let v3: u64 = {
            let a1618 = v2;
            let a1619 = 2u64;
            let a1620 = 0u64;
            nat_rem(a1618, a1619, a1620)
        };
        let a1630 = {
            let a1624 = {
                let a1621 = v2;
                let a1622 = 2u64;
                let a1623 = 0u64;
                nat_quot(a1621, a1622, a1623)
            };
            let a1625 = 3u64;
            nat_mul(a1624, a1625)?
        };
        let a1631 = {
            let a1626 = v3;
            let a1627 = f150(v0, v1);
            if nat_lt(a1626, a1627) {
                v3
            } else {
                let a1628 = v3;
                let a1629 = 1u64;
                nat_add(a1628, a1629)?
            }
        };
        nat_add(a1630, a1631)
    }
}

pub fn f93(v0: u64, v1: u64) -> u64 {
    let a1637 = {
        let a1632 = v1;
        let a1633 = 32u64;
        let a1634 = 0u64;
        nat_quot(a1632, a1633, a1634)
    };
    let a1638 = {
        let a1635 = v0;
        let a1636 = 0u64;
        if nat_eq(a1635, a1636) {
            120u64
        } else {
            5u64
        }
    };
    let a1639 = 0u64;
    nat_rem(a1637, a1638, a1639)
}

pub fn f94(v0: u64) -> u64 {
    f136({
        let a1640 = v0;
        let a1641 = 32u64;
        let a1642 = 0u64;
        nat_rem(a1640, a1641, a1642)
    })
}

pub fn f95(v0: u64) -> R<u64> {
    f137({
        let a1643 = v0;
        let a1644 = 32u64;
        let a1645 = 0u64;
        nat_rem(a1643, a1644, a1645)
    })
}

pub fn f96(v0: u64) -> R<u64> {
    f138({
        let a1646 = v0;
        let a1647 = 32u64;
        let a1648 = 0u64;
        nat_rem(a1646, a1647, a1648)
    })
}

pub fn f97(v0: u64) -> R<u64> {
    f139({
        let a1649 = v0;
        let a1650 = 32u64;
        let a1651 = 0u64;
        nat_rem(a1649, a1650, a1651)
    })
}

pub fn f98(v0: u64, v1: u64) -> R<u64> {
    let a1652 = v0;
    let a1653 = 0u64;
    if nat_eq(a1652, a1653) {
        let a1656 = {
            let a1654 = f150(v0, v1);
            let a1655 = 4u64;
            nat_mul(a1654, a1655)?
        };
        let a1657 = f94(v1);
        nat_add(a1656, a1657)
    } else {
        Ok(f150(v0, v1))
    }
}

pub fn f99(v0: u64) -> u64 {
    let a1658 = v0;
    let a1659 = 12u64;
    if nat_lt(a1658, a1659) {
        let a1660 = v0;
        let a1661 = 6u64;
        if nat_lt(a1660, a1661) {
            let a1662 = v0;
            let a1663 = 3u64;
            if nat_lt(a1662, a1663) {
                let a1664 = v0;
                let a1665 = 1u64;
                if nat_lt(a1664, a1665) {
                    0u64
                } else {
                    let a1666 = v0;
                    let a1667 = 2u64;
                    if nat_lt(a1666, a1667) {
                        2147483648u64
                    } else {
                        1065353216u64
                    }
                }
            } else {
                let a1668 = v0;
                let a1669 = 4u64;
                if nat_lt(a1668, a1669) {
                    3212836864u64
                } else {
                    let a1670 = v0;
                    let a1671 = 5u64;
                    if nat_lt(a1670, a1671) {
                        2139095040u64
                    } else {
                        4286578688u64
                    }
                }
            }
        } else {
            let a1672 = v0;
            let a1673 = 9u64;
            if nat_lt(a1672, a1673) {
                let a1674 = v0;
                let a1675 = 7u64;
                if nat_lt(a1674, a1675) {
                    2143289345u64
                } else {
                    let a1676 = v0;
                    let a1677 = 8u64;
                    if nat_lt(a1676, a1677) {
                        2139095041u64
                    } else {
                        4286578689u64
                    }
                }
            } else {
                let a1678 = v0;
                let a1679 = 10u64;
                if nat_lt(a1678, a1679) {
                    1u64
                } else {
                    let a1680 = v0;
                    let a1681 = 11u64;
                    if nat_lt(a1680, a1681) {
                        2147483649u64
                    } else {
                        8388608u64
                    }
                }
            }
        }
    } else {
        let a1682 = v0;
        let a1683 = 18u64;
        if nat_lt(a1682, a1683) {
            let a1684 = v0;
            let a1685 = 15u64;
            if nat_lt(a1684, a1685) {
                let a1686 = v0;
                let a1687 = 13u64;
                if nat_lt(a1686, a1687) {
                    2139095039u64
                } else {
                    let a1688 = v0;
                    let a1689 = 14u64;
                    if nat_lt(a1688, a1689) {
                        4286578687u64
                    } else {
                        864026624u64
                    }
                }
            } else {
                let a1690 = v0;
                let a1691 = 16u64;
                if nat_lt(a1690, a1691) {
                    855638016u64
                } else {
                    let a1692 = v0;
                    let a1693 = 17u64;
                    if nat_lt(a1692, a1693) {
                        855638017u64
                    } else {
                        855638015u64
                    }
                }
            }
        } else {
            let a1694 = v0;
            let a1695 = 21u64;
            if nat_lt(a1694, a1695) {
                let a1696 = v0;
                let a1697 = 19u64;
                if nat_lt(a1696, a1697) {
                    1065357312u64
                } else {
                    let a1698 = v0;
                    let a1699 = 20u64;
                    if nat_lt(a1698, a1699) {
                        1065365504u64
                    } else {
                        1199562752u64
                    }
                }
            } else {
                let a1700 = v0;
                let a1701 = 22u64;
                if nat_lt(a1700, a1701) {
                    1199566848u64
                } else {
                    let a1702 = v0;
                    let a1703 = 23u64;
                    if nat_lt(a1702, a1703) {
                        1199566847u64
                    } else {
                        947912704u64
                    }
                }
            }
        }
    }
}

pub fn f100(v0: u64, v1: u64) -> R<u64> {
    let a1704 = v0;
    let a1705 = 0u64;
    if nat_eq(a1704, a1705) {
        Ok(f91(v0, v1))
    } else {
        Ok(f149({
            let a1706 = f92(v0, v1)?;
            let a1707 = 3u64;
            let a1708 = 0u64;
            nat_rem(a1706, a1707, a1708)
        }))
    }
}

pub fn f101(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a1709 = v2;
    let a1710 = 1u64;
    if nat_eq(a1709, a1710) {
        Ok(true)
    } else {
        let a1711 = v0;
        let a1712 = 1u64;
        if if nat_eq(a1711, a1712) {
            let a1713 = 3u64;
            let a1714 = f92(v0, v1)?;
            nat_le(a1713, a1714)
        } else {
            false
        } {
            let a1715 = v2;
            let a1716 = 3u64;
            Ok(nat_eq(a1715, a1716))
        } else {
            Ok(false)
        }
    }
}

pub fn f102(v0: u64, v1: u64) -> R<u64> {
    let a1721 = {
        let a1717 = f103(v0, v1, 0u64)?;
        let a1718 = f103(v0, v1, 1u64)?;
        nat_mul(a1717, a1718)?
    };
    let a1722 = {
        let a1719 = f103(v0, v1, 2u64)?;
        let a1720 = f103(v0, v1, 3u64)?;
        nat_mul(a1719, a1720)?
    };
    nat_mul(a1721, a1722)
}

pub fn f103(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1728 = {
        let a1723 = f93(v0, v1);
        let a1724 = 5u64;
        let a1725 = 0u64;
        nat_rem(a1723, a1724, a1725)
    };
    let a1729 = {
        let a1726 = v2;
        let a1727 = 1u64;
        nat_add(a1726, a1727)?
    };
    if nat_eq(a1728, a1729) {
        Ok(0u64)
    } else {
        let a1730 = v2;
        let a1731 = 2u64;
        if nat_lt(a1730, a1731) {
            let a1732 = v2;
            let a1733 = 0u64;
            if nat_eq(a1732, a1733) {
                f95(v1)
            } else {
                f96(v1)
            }
        } else {
            let a1734 = v2;
            let a1735 = 2u64;
            if nat_eq(a1734, a1735) {
                Ok(2u64)
            } else {
                f97(v1)
            }
        }
    }
}

pub fn f104(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1740 = f109(v0, v1, v2)?;
    let a1741 = {
        let a1736 = v2;
        let a1737 = 0u64;
        if if if nat_eq(a1736, a1737) {
            true
        } else {
            f151(v0, v1, v2)?
        } {
            let a1738 = f94(v1);
            let a1739 = 2u64;
            nat_eq(a1738, a1739)
        } else {
            false
        } {
            2u64
        } else {
            1u64
        }
    };
    nat_mul(a1740, a1741)
}

pub fn f105(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1744 = {
        let a1742 = f104(v0, v1, v2)?;
        let a1743 = f40(v0, v1, v2, 0u64)?;
        nat_mul(a1742, a1743)?
    };
    let a1745 = f152(v0, v1, v2)?;
    nat_add(a1744, a1745)
}

pub fn f106(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1748 = {
        let a1746 = f105(v0, v1, v2)?;
        let a1747 = f40(v0, v1, v2, 1u64)?;
        nat_mul(a1746, a1747)?
    };
    let a1749 = f152(v0, v1, v2)?;
    nat_add(a1748, a1749)
}

pub fn f107(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1752 = {
        let a1750 = f106(v0, v1, v2)?;
        let a1751 = f40(v0, v1, v2, 2u64)?;
        nat_mul(a1750, a1751)?
    };
    let a1753 = f152(v0, v1, v2)?;
    nat_add(a1752, a1753)
}

pub fn f108(v0: Adt3) -> R<Adt4> {
    let a1755 = match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, _, _, h1754, _) => {
            h1754
        }
    };
    let a1756 = 0u64;
    if nat_eq(a1755, a1756) {
        Ok(Adt4::C0)
    } else {
        let a1758 = match v0.clone() {
            Adt3::C0(h1757, _, _, _, _, _, _, _, _, _) => {
                h1757
            }
        };
        let a1759 = 0u64;
        if if if if nat_eq(a1758, a1759) {
            true
        } else {
            let a1761 = match v0.clone() {
                Adt3::C0(_, h1760, _, _, _, _, _, _, _, _) => {
                    h1760
                }
            };
            let a1762 = 0u64;
            nat_eq(a1761, a1762)
        } {
            true
        } else {
            let a1764 = match v0.clone() {
                Adt3::C0(_, _, h1763, _, _, _, _, _, _, _) => {
                    h1763
                }
            };
            let a1765 = 0u64;
            nat_eq(a1764, a1765)
        } {
            true
        } else {
            let a1767 = match v0.clone() {
                Adt3::C0(_, _, _, h1766, _, _, _, _, _, _) => {
                    h1766
                }
            };
            let a1768 = 0u64;
            nat_eq(a1767, a1768)
        } {
            Ok(Adt4::C1(0u64))
        } else {
            let a1770 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, _, _, h1769, _) => {
                    h1769
                }
            };
            let a1771 = 1u64;
            if nat_eq(a1770, a1771) {
                Ok(Adt4::C1({
                    let a1781 = {
                        let a1779 = match v0.clone() {
                            Adt3::C0(_, _, _, _, _, _, _, _, _, h1772) => {
                                h1772
                            }
                        };
                        let a1780 = {
                            let a1777 = {
                                let a1774 = match v0.clone() {
                                    Adt3::C0(h1773, _, _, _, _, _, _, _, _, _) => {
                                        h1773
                                    }
                                };
                                let a1775 = 1u64;
                                nat_sub(a1774, a1775)
                            };
                            let a1778 = match v0.clone() {
                                Adt3::C0(_, _, _, _, h1776, _, _, _, _, _) => {
                                    h1776
                                }
                            };
                            nat_mul(a1777, a1778)?
                        };
                        nat_add(a1779, a1780)?
                    };
                    let a1782 = f153(v0)?;
                    nat_add(a1781, a1782)?
                }))
            } else {
                Ok(Adt4::C1({
                    let a1791 = {
                        let a1788 = {
                            let a1785 = match v0.clone() {
                                Adt3::C0(h1783, _, _, _, _, _, _, _, _, _) => {
                                    h1783
                                }
                            };
                            let a1786 = match v0.clone() {
                                Adt3::C0(_, _, _, _, h1784, _, _, _, _, _) => {
                                    h1784
                                }
                            };
                            nat_mul(a1785, a1786)?
                        };
                        let a1789 = match v0.clone() {
                            Adt3::C0(_, _, _, _, _, _, _, _, h1787, _) => {
                                h1787
                            }
                        };
                        let a1790 = 0u64;
                        nat_quot(a1788, a1789, a1790)
                    };
                    let a1792 = f153(v0)?;
                    nat_add(a1791, a1792)?
                }))
            }
        }
    }
}

pub fn f109(v0: u64, v1: u64, v2: u64) -> R<u64> {
    Ok(f154({
        let a1793 = v2;
        let a1794 = 0u64;
        if nat_eq(a1793, a1794) {
            f91(v0, v1)
        } else {
            f100(v0, v1)?
        }
    }))
}

pub fn f110(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1795 = v0;
    let a1796 = 0u64;
    if nat_eq(a1795, a1796) {
        f147(v1, v2)
    } else {
        let v3: u64 = f154(f149(v1));
        let v4: u64 = {
            let a1797 = v1;
            let a1798 = 0u64;
            if nat_eq(a1797, a1798) {
                f99({
                    let a1802 = {
                        let a1799 = v2;
                        let a1800 = v3;
                        let a1801 = 0u64;
                        nat_quot(a1799, a1800, a1801)
                    };
                    let a1803 = 24u64;
                    let a1804 = 0u64;
                    nat_rem(a1802, a1803, a1804)
                })
            } else {
                f155({
                    let a1808 = {
                        let a1805 = v2;
                        let a1806 = v3;
                        let a1807 = 0u64;
                        nat_quot(a1805, a1806, a1807)
                    };
                    let a1809 = 24u64;
                    let a1810 = 0u64;
                    nat_rem(a1808, a1809, a1810)
                })
            }
        };
        let a1829 = {
            let a1826 = v4;
            let a1827 = {
                let a1814 = {
                    let a1811 = v2;
                    let a1812 = v3;
                    let a1813 = 0u64;
                    nat_rem(a1811, a1812, a1813)
                };
                let a1815 = 2u64;
                if nat_lt(a1814, a1815) {
                    let a1819 = {
                        let a1816 = v2;
                        let a1817 = v3;
                        let a1818 = 0u64;
                        nat_rem(a1816, a1817, a1818)
                    };
                    let a1820 = 0u64;
                    if nat_eq(a1819, a1820) {
                        1u64
                    } else {
                        256u64
                    }
                } else {
                    let a1824 = {
                        let a1821 = v2;
                        let a1822 = v3;
                        let a1823 = 0u64;
                        nat_rem(a1821, a1822, a1823)
                    };
                    let a1825 = 2u64;
                    if nat_eq(a1824, a1825) {
                        65536u64
                    } else {
                        16777216u64
                    }
                }
            };
            let a1828 = 0u64;
            nat_quot(a1826, a1827, a1828)
        };
        let a1830 = 256u64;
        let a1831 = 0u64;
        Ok(nat_rem(a1829, a1830, a1831))
    }
}

pub fn f111(v0: u64) -> R<u64> {
    f156(f157(v0))
}

pub fn f112(v0: u64) -> u64 {
    let a1835 = {
        let a1832 = v0;
        let a1833 = 72u64;
        let a1834 = 0u64;
        nat_quot(a1832, a1833, a1834)
    };
    let a1836 = 4u64;
    let a1837 = 0u64;
    nat_rem(a1835, a1836, a1837)
}

pub fn f113(v0: u64) -> u64 {
    let a1841 = {
        let a1838 = v0;
        let a1839 = 18u64;
        let a1840 = 0u64;
        nat_quot(a1838, a1839, a1840)
    };
    let a1842 = 4u64;
    let a1843 = 0u64;
    nat_rem(a1841, a1842, a1843)
}

pub fn f114(v0: u64) -> u64 {
    f158({
        let a1847 = {
            let a1844 = v0;
            let a1845 = 6u64;
            let a1846 = 0u64;
            nat_quot(a1844, a1845, a1846)
        };
        let a1848 = 3u64;
        let a1849 = 0u64;
        nat_rem(a1847, a1848, a1849)
    })
}

pub fn f115(v0: u64) -> u64 {
    f158({
        let a1853 = {
            let a1850 = v0;
            let a1851 = 2u64;
            let a1852 = 0u64;
            nat_quot(a1850, a1851, a1852)
        };
        let a1854 = 3u64;
        let a1855 = 0u64;
        nat_rem(a1853, a1854, a1855)
    })
}

pub fn f116(v0: u64) -> R<u64> {
    let a1859 = 1u64;
    let a1860 = {
        let a1856 = v0;
        let a1857 = 2u64;
        let a1858 = 0u64;
        nat_rem(a1856, a1857, a1858)
    };
    nat_add(a1859, a1860)
}

pub fn f117(v0: u64, v1: u64) -> R<u64> {
    let a1869 = {
        let a1865 = {
            let a1863 = {
                let a1861 = f157(v0);
                let a1862 = 4u64;
                nat_mul(a1861, a1862)?
            };
            let a1864 = f113(v0);
            nat_add(a1863, a1864)?
        };
        let a1866 = 2u64;
        nat_mul(a1865, a1866)?
    };
    let a1870 = {
        let a1867 = v1;
        let a1868 = 1u64;
        if nat_eq(a1867, a1868) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a1869, a1870)
}

pub fn f118(v0: u64, v1: u64) -> R<u64> {
    let a1871 = v1;
    let a1872 = 2u64;
    if nat_lt(a1871, a1872) {
        let a1873 = v1;
        let a1874 = 0u64;
        if nat_eq(a1873, a1874) {
            let a1875 = f119(v0)?;
            let a1876 = f114(v0);
            nat_mul(a1875, a1876)
        } else {
            Ok(f115(v0))
        }
    } else {
        let a1877 = v1;
        let a1878 = 2u64;
        if nat_eq(a1877, a1878) {
            Ok(2u64)
        } else {
            f116(v0)
        }
    }
}

pub fn f119(v0: u64) -> R<u64> {
    f141(f159(v0)?)
}

pub fn f120(v0: u64) -> R<u64> {
    let a1883 = {
        let a1879 = f118(v0, 0u64)?;
        let a1880 = f118(v0, 1u64)?;
        nat_mul(a1879, a1880)?
    };
    let a1884 = {
        let a1881 = f118(v0, 2u64)?;
        let a1882 = f118(v0, 3u64)?;
        nat_mul(a1881, a1882)?
    };
    nat_mul(a1883, a1884)
}

pub fn f121(v0: u64, v1: u64) -> R<u64> {
    let a1885 = v1;
    let a1886 = f112(v0);
    if nat_eq(a1885, a1886) {
        let a1887 = f113(v0);
        let a1888 = 3u64;
        if nat_eq(a1887, a1888) {
            Ok(0u64)
        } else {
            let a1891 = f118(v0, v1)?;
            let a1892 = {
                let a1889 = v1;
                let a1890 = 0u64;
                if nat_eq(a1889, a1890) {
                    f119(v0)?
                } else {
                    1u64
                }
            };
            nat_add(a1891, a1892)
        }
    } else {
        f118(v0, v1)
    }
}

pub fn f122(v0: u64, v1: u64) -> R<u64> {
    let a1899 = f126(v0)?;
    let a1900 = {
        let a1893 = v1;
        let a1894 = 0u64;
        if if if nat_eq(a1893, a1894) {
            true
        } else {
            let a1895 = v1;
            let a1896 = 2u64;
            nat_eq(a1895, a1896)
        } {
            let a1897 = f113(v0);
            let a1898 = 2u64;
            nat_eq(a1897, a1898)
        } else {
            false
        } {
            2u64
        } else {
            1u64
        }
    };
    nat_mul(a1899, a1900)
}

pub fn f123(v0: u64, v1: u64) -> R<u64> {
    let a1906 = {
        let a1904 = f122(v0, v1)?;
        let a1905 = {
            let a1901 = f47(v0, v1, 0u64)?;
            let a1902 = f119(v0)?;
            let a1903 = 0u64;
            nat_quot(a1901, a1902, a1903)
        };
        nat_mul(a1904, a1905)?
    };
    let a1907 = f160(v0, v1)?;
    nat_add(a1906, a1907)
}

pub fn f124(v0: u64, v1: u64) -> R<u64> {
    let a1910 = {
        let a1908 = f123(v0, v1)?;
        let a1909 = f47(v0, v1, 1u64)?;
        nat_mul(a1908, a1909)?
    };
    let a1911 = f160(v0, v1)?;
    nat_add(a1910, a1911)
}

pub fn f125(v0: u64, v1: u64) -> R<u64> {
    let a1914 = {
        let a1912 = f124(v0, v1)?;
        let a1913 = f47(v0, v1, 2u64)?;
        nat_mul(a1912, a1913)?
    };
    let a1915 = f160(v0, v1)?;
    nat_add(a1914, a1915)
}

pub fn f126(v0: u64) -> R<u64> {
    f146(f159(v0)?)
}

pub fn f127(v0: u64, v1: u64) -> R<u64> {
    let a1926 = {
        let a1924 = f147({
            let a1916 = v0;
            let a1917 = 2u64;
            let a1918 = 0u64;
            nat_quot(a1916, a1917, a1918)
        }, v1)?;
        let a1925 = {
            let a1922 = {
                let a1919 = v0;
                let a1920 = 2u64;
                let a1921 = 0u64;
                nat_rem(a1919, a1920, a1921)
            };
            let a1923 = 71u64;
            nat_mul(a1922, a1923)?
        };
        nat_add(a1924, a1925)?
    };
    let a1927 = 256u64;
    let a1928 = 0u64;
    Ok(nat_rem(a1926, a1927, a1928))
}

pub fn f128(v0: u64) -> R<u64> {
    Ok(f136(f57(v0)?))
}

pub fn f129(v0: u64) -> R<u64> {
    f146(f57(v0)?)
}

pub fn f130(v0: u64, v1: u64) -> R<u64> {
    let a1931 = {
        let a1929 = f129(v0)?;
        let a1930 = f53(v0, v1, 0u64)?;
        nat_mul(a1929, a1930)?
    };
    let a1932 = f161(v0, v1)?;
    nat_add(a1931, a1932)
}

pub fn f131(v0: u64, v1: u64) -> R<u64> {
    let a1935 = {
        let a1933 = f130(v0, v1)?;
        let a1934 = f53(v0, v1, 1u64)?;
        nat_mul(a1933, a1934)?
    };
    let a1936 = f161(v0, v1)?;
    nat_add(a1935, a1936)
}

pub fn f132(v0: u64, v1: u64) -> R<u64> {
    let a1939 = {
        let a1937 = f131(v0, v1)?;
        let a1938 = f53(v0, v1, 2u64)?;
        nat_mul(a1937, a1938)?
    };
    let a1940 = f161(v0, v1)?;
    nat_add(a1939, a1940)
}

pub fn f133(v0: u64) -> Adt5 {
    let a1941 = v0;
    let a1942 = 0u64;
    if nat_eq(a1941, a1942) {
        Adt5::C1(Adt6::C0(0u64, 1u64, 4u64))
    } else {
        let a1943 = v0;
        let a1944 = 1u64;
        if nat_eq(a1943, a1944) {
            Adt5::C1(Adt6::C0(1u64, 1u64, 2u64))
        } else {
            let a1945 = v0;
            let a1946 = 2u64;
            if nat_eq(a1945, a1946) {
                Adt5::C1(Adt6::C0(2u64, 32u64, 18u64))
            } else {
                let a1947 = v0;
                let a1948 = 3u64;
                if nat_eq(a1947, a1948) {
                    Adt5::C1(Adt6::C0(3u64, 32u64, 20u64))
                } else {
                    let a1949 = v0;
                    let a1950 = 6u64;
                    if nat_eq(a1949, a1950) {
                        Adt5::C1(Adt6::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1951 = v0;
                        let a1952 = 7u64;
                        if nat_eq(a1951, a1952) {
                            Adt5::C1(Adt6::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1953 = v0;
                            let a1954 = 8u64;
                            if nat_eq(a1953, a1954) {
                                Adt5::C1(Adt6::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1955 = v0;
                                let a1956 = 9u64;
                                if nat_eq(a1955, a1956) {
                                    Adt5::C1(Adt6::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1957 = v0;
                                    let a1958 = 10u64;
                                    if nat_eq(a1957, a1958) {
                                        Adt5::C1(Adt6::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1959 = v0;
                                        let a1960 = 11u64;
                                        if nat_eq(a1959, a1960) {
                                            Adt5::C1(Adt6::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1961 = v0;
                                            let a1962 = 12u64;
                                            if nat_eq(a1961, a1962) {
                                                Adt5::C1(Adt6::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1963 = v0;
                                                let a1964 = 13u64;
                                                if nat_eq(a1963, a1964) {
                                                    Adt5::C1(Adt6::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1965 = v0;
                                                    let a1966 = 14u64;
                                                    if nat_eq(a1965, a1966) {
                                                        Adt5::C1(Adt6::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1967 = v0;
                                                        let a1968 = 15u64;
                                                        if nat_eq(a1967, a1968) {
                                                            Adt5::C1(Adt6::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1969 = v0;
                                                            let a1970 = 16u64;
                                                            if nat_eq(a1969, a1970) {
                                                                Adt5::C1(Adt6::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1971 = v0;
                                                                let a1972 = 17u64;
                                                                if nat_eq(a1971, a1972) {
                                                                    Adt5::C1(Adt6::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1973 = v0;
                                                                    let a1974 = 18u64;
                                                                    if nat_eq(a1973, a1974) {
                                                                        Adt5::C1(Adt6::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1975 = v0;
                                                                        let a1976 = 19u64;
                                                                        if nat_eq(a1975, a1976) {
                                                                            Adt5::C1(Adt6::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1977 = v0;
                                                                            let a1978 = 20u64;
                                                                            if nat_eq(a1977, a1978) {
                                                                                Adt5::C1(Adt6::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1979 = v0;
                                                                                let a1980 = 21u64;
                                                                                if nat_eq(a1979, a1980) {
                                                                                    Adt5::C1(Adt6::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1981 = v0;
                                                                                    let a1982 = 22u64;
                                                                                    if nat_eq(a1981, a1982) {
                                                                                        Adt5::C1(Adt6::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1983 = v0;
                                                                                        let a1984 = 23u64;
                                                                                        if nat_eq(a1983, a1984) {
                                                                                            Adt5::C1(Adt6::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1985 = v0;
                                                                                            let a1986 = 24u64;
                                                                                            if nat_eq(a1985, a1986) {
                                                                                                Adt5::C1(Adt6::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1987 = v0;
                                                                                                let a1988 = 25u64;
                                                                                                if nat_eq(a1987, a1988) {
                                                                                                    Adt5::C1(Adt6::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1989 = v0;
                                                                                                    let a1990 = 26u64;
                                                                                                    if nat_eq(a1989, a1990) {
                                                                                                        Adt5::C1(Adt6::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1991 = v0;
                                                                                                        let a1992 = 27u64;
                                                                                                        if nat_eq(a1991, a1992) {
                                                                                                            Adt5::C1(Adt6::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1993 = v0;
                                                                                                            let a1994 = 28u64;
                                                                                                            if nat_eq(a1993, a1994) {
                                                                                                                Adt5::C1(Adt6::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1995 = v0;
                                                                                                                let a1996 = 29u64;
                                                                                                                if nat_eq(a1995, a1996) {
                                                                                                                    Adt5::C1(Adt6::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1997 = v0;
                                                                                                                    let a1998 = 30u64;
                                                                                                                    if nat_eq(a1997, a1998) {
                                                                                                                        Adt5::C1(Adt6::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1999 = v0;
                                                                                                                        let a2000 = 34u64;
                                                                                                                        if nat_eq(a1999, a2000) {
                                                                                                                            Adt5::C1(Adt6::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a2001 = v0;
                                                                                                                            let a2002 = 35u64;
                                                                                                                            if nat_eq(a2001, a2002) {
                                                                                                                                Adt5::C1(Adt6::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a2003 = v0;
                                                                                                                                let a2004 = 39u64;
                                                                                                                                if nat_eq(a2003, a2004) {
                                                                                                                                    Adt5::C1(Adt6::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a2005 = v0;
                                                                                                                                    let a2006 = 40u64;
                                                                                                                                    if nat_eq(a2005, a2006) {
                                                                                                                                        Adt5::C1(Adt6::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a2007 = v0;
                                                                                                                                        let a2008 = 41u64;
                                                                                                                                        if nat_eq(a2007, a2008) {
                                                                                                                                            Adt5::C1(Adt6::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a2009 = v0;
                                                                                                                                            let a2010 = 42u64;
                                                                                                                                            if nat_eq(a2009, a2010) {
                                                                                                                                                Adt5::C1(Adt6::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a2011 = v0;
                                                                                                                                                let a2012 = 43u64;
                                                                                                                                                if nat_eq(a2011, a2012) {
                                                                                                                                                    Adt5::C1(Adt6::C0(43u64, 1u64, 1u64))
                                                                                                                                                } else {
                                                                                                                                                    Adt5::C0
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

pub fn f134(v0: u64) -> R<u64> {
    f156({
        let a2013 = v0;
        let a2014 = 96u64;
        let a2015 = 0u64;
        nat_quot(a2013, a2014, a2015)
    })
}

pub fn f135(v0: u64) -> u64 {
    let a2019 = {
        let a2016 = v0;
        let a2017 = 32u64;
        let a2018 = 0u64;
        nat_quot(a2016, a2017, a2018)
    };
    let a2020 = 3u64;
    let a2021 = 0u64;
    nat_rem(a2019, a2020, a2021)
}

pub fn f136(v0: u64) -> u64 {
    let a2025 = {
        let a2022 = v0;
        let a2023 = 8u64;
        let a2024 = 0u64;
        nat_quot(a2022, a2023, a2024)
    };
    let a2026 = 4u64;
    let a2027 = 0u64;
    nat_rem(a2025, a2026, a2027)
}

pub fn f137(v0: u64) -> R<u64> {
    let a2036 = 1u64;
    let a2037 = {
        let a2034 = 2u64;
        let a2035 = {
            let a2031 = {
                let a2028 = v0;
                let a2029 = 4u64;
                let a2030 = 0u64;
                nat_quot(a2028, a2029, a2030)
            };
            let a2032 = 2u64;
            let a2033 = 0u64;
            nat_rem(a2031, a2032, a2033)
        };
        nat_mul(a2034, a2035)?
    };
    nat_add(a2036, a2037)
}

pub fn f138(v0: u64) -> R<u64> {
    let a2046 = 1u64;
    let a2047 = {
        let a2044 = 2u64;
        let a2045 = {
            let a2041 = {
                let a2038 = v0;
                let a2039 = 2u64;
                let a2040 = 0u64;
                nat_quot(a2038, a2039, a2040)
            };
            let a2042 = 2u64;
            let a2043 = 0u64;
            nat_rem(a2041, a2042, a2043)
        };
        nat_mul(a2044, a2045)?
    };
    nat_add(a2046, a2047)
}

pub fn f139(v0: u64) -> R<u64> {
    let a2051 = 1u64;
    let a2052 = {
        let a2048 = v0;
        let a2049 = 2u64;
        let a2050 = 0u64;
        nat_rem(a2048, a2049, a2050)
    };
    nat_add(a2051, a2052)
}

pub fn f140(v0: u64) -> R<u64> {
    let a2058 = {
        let a2056 = {
            let a2053 = v0;
            let a2054 = 96u64;
            let a2055 = 0u64;
            nat_quot(a2053, a2054, a2055)
        };
        let a2057 = 4u64;
        nat_mul(a2056, a2057)?
    };
    let a2059 = f136(v0);
    nat_add(a2058, a2059)
}

pub fn f141(v0: u64) -> R<u64> {
    let m2060 = f133(f134(v0)?);
    match m2060 {
        Adt5::C0 => {
            Ok(0u64)
        }
        Adt5::C1(v1) => {
            match v1 {
                Adt6::C0(_, h2061, _) => {
                    Ok(h2061)
                }
            }
        }
    }
}

pub fn f142(v0: u64, v1: u64) -> R<u64> {
    let a2066 = f146(v0)?;
    let a2067 = {
        let a2062 = v1;
        let a2063 = 0u64;
        if if nat_eq(a2062, a2063) {
            let a2064 = f136(v0);
            let a2065 = 2u64;
            nat_eq(a2064, a2065)
        } else {
            false
        } {
            2u64
        } else {
            1u64
        }
    };
    nat_mul(a2066, a2067)
}

pub fn f143(v0: u64, v1: u64) -> R<u64> {
    let a2073 = {
        let a2071 = f142(v0, v1)?;
        let a2072 = {
            let a2068 = f60(v0, v1, 0u64)?;
            let a2069 = f141(v0)?;
            let a2070 = 0u64;
            nat_quot(a2068, a2069, a2070)
        };
        nat_mul(a2071, a2072)?
    };
    let a2074 = f162(v0, v1)?;
    nat_add(a2073, a2074)
}

pub fn f144(v0: u64, v1: u64) -> R<u64> {
    let a2077 = {
        let a2075 = f143(v0, v1)?;
        let a2076 = f60(v0, v1, 1u64)?;
        nat_mul(a2075, a2076)?
    };
    let a2078 = f162(v0, v1)?;
    nat_add(a2077, a2078)
}

pub fn f145(v0: u64, v1: u64) -> R<u64> {
    let a2081 = {
        let a2079 = f144(v0, v1)?;
        let a2080 = f60(v0, v1, 2u64)?;
        nat_mul(a2079, a2080)?
    };
    let a2082 = f162(v0, v1)?;
    nat_add(a2081, a2082)
}

pub fn f146(v0: u64) -> R<u64> {
    let m2083 = f133(f134(v0)?);
    match m2083 {
        Adt5::C0 => {
            Ok(0u64)
        }
        Adt5::C1(v1) => {
            match v1 {
                Adt6::C0(_, _, h2084) => {
                    Ok(h2084)
                }
            }
        }
    }
}

pub fn f147(v0: u64, v1: u64) -> R<u64> {
    let a2099 = {
        let a2097 = {
            let a2092 = {
                let a2085 = v1;
                let a2086 = 37u64;
                nat_mul(a2085, a2086)?
            };
            let a2093 = {
                let a2090 = f156({
                    let a2087 = v0;
                    let a2088 = 4u64;
                    let a2089 = 0u64;
                    nat_quot(a2087, a2088, a2089)
                })?;
                let a2091 = 13u64;
                nat_mul(a2090, a2091)?
            };
            nat_add(a2092, a2093)?
        };
        let a2098 = {
            let a2094 = v0;
            let a2095 = 4u64;
            let a2096 = 0u64;
            nat_rem(a2094, a2095, a2096)
        };
        nat_add(a2097, a2098)?
    };
    let a2100 = 256u64;
    let a2101 = 0u64;
    Ok(nat_rem(a2099, a2100, a2101))
}

pub fn f148(v0: u64) -> u64 {
    let a2102 = v0;
    let a2103 = 96u64;
    if nat_lt(a2102, a2103) {
        let a2107 = {
            let a2104 = v0;
            let a2105 = 8u64;
            let a2106 = 0u64;
            nat_quot(a2104, a2105, a2106)
        };
        let a2108 = 3u64;
        let a2109 = 0u64;
        nat_rem(a2107, a2108, a2109)
    } else {
        let a2115 = {
            let a2112 = {
                let a2110 = v0;
                let a2111 = 96u64;
                nat_sub(a2110, a2111)
            };
            let a2113 = 2u64;
            let a2114 = 0u64;
            nat_quot(a2112, a2113, a2114)
        };
        let a2116 = 3u64;
        let a2117 = 0u64;
        nat_rem(a2115, a2116, a2117)
    }
}

pub fn f149(v0: u64) -> u64 {
    let a2118 = v0;
    let a2119 = 0u64;
    if nat_eq(a2118, a2119) {
        0u64
    } else {
        let a2120 = v0;
        let a2121 = 1u64;
        if nat_eq(a2120, a2121) {
            1u64
        } else {
            30u64
        }
    }
}

pub fn f150(v0: u64, v1: u64) -> u64 {
    let a2124 = v1;
    let a2125 = {
        let a2122 = v0;
        let a2123 = 0u64;
        if nat_eq(a2122, a2123) {
            7680u64
        } else {
            640u64
        }
    };
    let a2126 = 0u64;
    nat_quot(a2124, a2125, a2126)
}

pub fn f151(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a2127 = v0;
    let a2128 = 0u64;
    if if nat_eq(a2127, a2128) {
        let a2129 = f92(v0, v1)?;
        let a2130 = 1u64;
        nat_eq(a2129, a2130)
    } else {
        false
    } {
        let a2131 = v2;
        let a2132 = 3u64;
        Ok(nat_eq(a2131, a2132))
    } else {
        Ok(false)
    }
}

pub fn f152(v0: u64, v1: u64, v2: u64) -> R<u64> {
    if if f151(v0, v1, v2)? {
        true
    } else {
        let a2133 = v2;
        let a2134 = 0u64;
        nat_eq(a2133, a2134)
    } {
        let a2135 = f94(v1);
        let a2136 = 1u64;
        if if nat_eq(a2135, a2136) {
            true
        } else {
            let a2137 = f94(v1);
            let a2138 = 2u64;
            nat_eq(a2137, a2138)
        } {
            f109(v0, v1, v2)
        } else {
            Ok(0u64)
        }
    } else {
        f109(v0, v1, v2)
    }
}

pub fn f153(v0: Adt3) -> R<u64> {
    let a2159 = {
        let a2151 = {
            let a2143 = {
                let a2140 = match v0.clone() {
                    Adt3::C0(_, h2139, _, _, _, _, _, _, _, _) => {
                        h2139
                    }
                };
                let a2141 = 1u64;
                nat_sub(a2140, a2141)
            };
            let a2144 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, h2142, _, _, _, _) => {
                    h2142
                }
            };
            nat_mul(a2143, a2144)?
        };
        let a2152 = {
            let a2149 = {
                let a2146 = match v0.clone() {
                    Adt3::C0(_, _, h2145, _, _, _, _, _, _, _) => {
                        h2145
                    }
                };
                let a2147 = 1u64;
                nat_sub(a2146, a2147)
            };
            let a2150 = match v0.clone() {
                Adt3::C0(_, _, _, _, _, _, h2148, _, _, _) => {
                    h2148
                }
            };
            nat_mul(a2149, a2150)?
        };
        nat_add(a2151, a2152)?
    };
    let a2160 = {
        let a2157 = {
            let a2154 = match v0.clone() {
                Adt3::C0(_, _, _, h2153, _, _, _, _, _, _) => {
                    h2153
                }
            };
            let a2155 = 1u64;
            nat_sub(a2154, a2155)
        };
        let a2158 = match v0 {
            Adt3::C0(_, _, _, _, _, _, _, h2156, _, _) => {
                h2156
            }
        };
        nat_mul(a2157, a2158)?
    };
    nat_add(a2159, a2160)
}

pub fn f154(v0: u64) -> u64 {
    let a2161 = v0;
    let a2162 = 0u64;
    if nat_eq(a2161, a2162) {
        4u64
    } else {
        2u64
    }
}

pub fn f155(v0: u64) -> u64 {
    let a2163 = v0;
    let a2164 = 12u64;
    if nat_lt(a2163, a2164) {
        let a2165 = v0;
        let a2166 = 6u64;
        if nat_lt(a2165, a2166) {
            let a2167 = v0;
            let a2168 = 3u64;
            if nat_lt(a2167, a2168) {
                let a2169 = v0;
                let a2170 = 1u64;
                if nat_lt(a2169, a2170) {
                    0u64
                } else {
                    let a2171 = v0;
                    let a2172 = 2u64;
                    if nat_lt(a2171, a2172) {
                        32768u64
                    } else {
                        15360u64
                    }
                }
            } else {
                let a2173 = v0;
                let a2174 = 4u64;
                if nat_lt(a2173, a2174) {
                    16256u64
                } else {
                    let a2175 = v0;
                    let a2176 = 5u64;
                    if nat_lt(a2175, a2176) {
                        48128u64
                    } else {
                        49024u64
                    }
                }
            }
        } else {
            let a2177 = v0;
            let a2178 = 9u64;
            if nat_lt(a2177, a2178) {
                let a2179 = v0;
                let a2180 = 7u64;
                if nat_lt(a2179, a2180) {
                    31744u64
                } else {
                    let a2181 = v0;
                    let a2182 = 8u64;
                    if nat_lt(a2181, a2182) {
                        32640u64
                    } else {
                        64512u64
                    }
                }
            } else {
                let a2183 = v0;
                let a2184 = 10u64;
                if nat_lt(a2183, a2184) {
                    65408u64
                } else {
                    let a2185 = v0;
                    let a2186 = 11u64;
                    if nat_lt(a2185, a2186) {
                        32256u64
                    } else {
                        32704u64
                    }
                }
            }
        }
    } else {
        let a2187 = v0;
        let a2188 = 18u64;
        if nat_lt(a2187, a2188) {
            let a2189 = v0;
            let a2190 = 15u64;
            if nat_lt(a2189, a2190) {
                let a2191 = v0;
                let a2192 = 13u64;
                if nat_lt(a2191, a2192) {
                    31745u64
                } else {
                    let a2193 = v0;
                    let a2194 = 14u64;
                    if nat_lt(a2193, a2194) {
                        32641u64
                    } else {
                        65025u64
                    }
                }
            } else {
                let a2195 = v0;
                let a2196 = 16u64;
                if nat_lt(a2195, a2196) {
                    65409u64
                } else {
                    let a2197 = v0;
                    let a2198 = 17u64;
                    if nat_lt(a2197, a2198) {
                        1u64
                    } else {
                        32769u64
                    }
                }
            }
        } else {
            let a2199 = v0;
            let a2200 = 21u64;
            if nat_lt(a2199, a2200) {
                let a2201 = v0;
                let a2202 = 19u64;
                if nat_lt(a2201, a2202) {
                    1023u64
                } else {
                    let a2203 = v0;
                    let a2204 = 20u64;
                    if nat_lt(a2203, a2204) {
                        127u64
                    } else {
                        1024u64
                    }
                }
            } else {
                let a2205 = v0;
                let a2206 = 22u64;
                if nat_lt(a2205, a2206) {
                    128u64
                } else {
                    let a2207 = v0;
                    let a2208 = 23u64;
                    if nat_lt(a2207, a2208) {
                        31743u64
                    } else {
                        32639u64
                    }
                }
            }
        }
    }
}

pub fn f156(v0: u64) -> R<u64> {
    let a2209 = v0;
    let a2210 = 4u64;
    if nat_lt(a2209, a2210) {
        Ok(v0)
    } else {
        let a2211 = v0;
        let a2212 = 29u64;
        if nat_lt(a2211, a2212) {
            let a2213 = v0;
            let a2214 = 2u64;
            nat_add(a2213, a2214)
        } else {
            let a2215 = v0;
            let a2216 = 31u64;
            if nat_lt(a2215, a2216) {
                let a2217 = v0;
                let a2218 = 5u64;
                nat_add(a2217, a2218)
            } else {
                let a2219 = v0;
                let a2220 = 8u64;
                nat_add(a2219, a2220)
            }
        }
    }
}

pub fn f157(v0: u64) -> u64 {
    let a2221 = v0;
    let a2222 = 288u64;
    let a2223 = 0u64;
    nat_quot(a2221, a2222, a2223)
}

pub fn f158(v0: u64) -> u64 {
    let a2224 = v0;
    let a2225 = 0u64;
    if nat_eq(a2224, a2225) {
        0u64
    } else {
        let a2226 = v0;
        let a2227 = 1u64;
        if nat_eq(a2226, a2227) {
            1u64
        } else {
            3u64
        }
    }
}

pub fn f159(v0: u64) -> R<u64> {
    let a2228 = f157(v0);
    let a2229 = 96u64;
    nat_mul(a2228, a2229)
}

pub fn f160(v0: u64, v1: u64) -> R<u64> {
    let a2230 = v1;
    let a2231 = 1u64;
    if if if nat_eq(a2230, a2231) {
        true
    } else {
        let a2232 = v1;
        let a2233 = 3u64;
        nat_eq(a2232, a2233)
    } {
        true
    } else {
        let a2234 = f113(v0);
        let a2235 = 1u64;
        if nat_eq(a2234, a2235) {
            true
        } else {
            let a2236 = f113(v0);
            let a2237 = 2u64;
            nat_eq(a2236, a2237)
        }
    } {
        f126(v0)
    } else {
        Ok(0u64)
    }
}

pub fn f161(v0: u64, v1: u64) -> R<u64> {
    let a2238 = v1;
    let a2239 = 3u64;
    if nat_eq(a2238, a2239) {
        f129(v0)
    } else {
        Ok(0u64)
    }
}

pub fn f162(v0: u64, v1: u64) -> R<u64> {
    let a2240 = v1;
    let a2241 = 1u64;
    if if nat_eq(a2240, a2241) {
        true
    } else {
        let a2242 = f136(v0);
        let a2243 = 1u64;
        if nat_eq(a2242, a2243) {
            true
        } else {
            let a2244 = f136(v0);
            let a2245 = 2u64;
            nat_eq(a2244, a2245)
        }
    } {
        f146(v0)
    } else {
        Ok(0u64)
    }
}

pub fn run(p0: u64, p1: Adt0) -> R<Adt1> {
    f0(p0, p1)
}
