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
    C1(u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

pub fn f0(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64, v19: u64, v20: u64, v21: u64, v22: u64, v23: u64, v24: u64) -> R<Adt0> {
    f1(v0, v1, v2, Adt1::C0(v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13), Adt1::C0(v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24))
}

pub fn f1(v0: u64, v1: u64, v2: u64, v3: Adt1, v4: Adt1) -> R<Adt0> {
    let a1 = v0;
    let a2 = 23u64;
    if nat_eq(a1, a2) {
        f2(v1, v3, v4)
    } else {
        let a3 = v0;
        let a4 = 26u64;
        if nat_eq(a3, a4) {
            f3(v1, v2, v3, v4)
        } else {
            Ok(Adt0::C0)
        }
    }
}

pub fn f2(v0: u64, v1: Adt1, v2: Adt1) -> R<Adt0> {
    let a6 = match v1.clone() {
        Adt1::C0(h5, _, _, _, _, _, _, _, _, _, _) => {
            h5
        }
    };
    let a7 = 0u64;
    if if if if nat_eq(a6, a7) {
        let a9 = match v2.clone() {
            Adt1::C0(h8, _, _, _, _, _, _, _, _, _, _) => {
                h8
            }
        };
        let a10 = 0u64;
        nat_eq(a9, a10)
    } else {
        false
    } {
        f4(v1.clone(), v2.clone())
    } else {
        false
    } {
        if f5(match v1.clone() {
            Adt1::C0(_, h11, _, _, _, _, _, _, _, _, _) => {
                h11
            }
        }, match v1.clone() {
            Adt1::C0(_, _, h12, _, _, _, _, _, _, _, _) => {
                h12
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, h13, _, _, _, _, _, _, _) => {
                h13
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h14, _, _, _, _, _, _) => {
                h14
            }
        }) {
            true
        } else {
            if if if if f6(v0) {
                f7(v1.clone(), v2.clone())
            } else {
                false
            } {
                f8(v1.clone())
            } else {
                false
            } {
                f8(v2.clone())
            } else {
                false
            } {
                if f9(v1.clone(), v2.clone())? {
                    true
                } else {
                    f10(v1.clone(), v2.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v3: u64 = if f5(match v1.clone() {
            Adt1::C0(_, h15, _, _, _, _, _, _, _, _, _) => {
                h15
            }
        }, match v1.clone() {
            Adt1::C0(_, _, h16, _, _, _, _, _, _, _, _) => {
                h16
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, h17, _, _, _, _, _, _, _) => {
                h17
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h18, _, _, _, _, _, _) => {
                h18
            }
        }) {
            0u64
        } else {
            let a24 = match v1.clone() {
                Adt1::C0(_, _, h19, _, _, _, _, _, _, _, _) => {
                    h19
                }
            };
            let a25 = {
                let a22 = match v1.clone() {
                    Adt1::C0(_, _, _, h20, _, _, _, _, _, _, _) => {
                        h20
                    }
                };
                let a23 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h21, _, _, _, _, _, _) => {
                        h21
                    }
                };
                nat_mul(a22, a23)?
            };
            nat_mul(a24, a25)?
        };
        let v4: u64 = {
            let a26 = v3;
            let a27 = 0u64;
            if if nat_eq(a26, a27) {
                true
            } else {
                f11(v2.clone())?
            } {
                0u64
            } else {
                let a29 = match v2.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, h28, _) => {
                        h28
                    }
                };
                let a30 = 4u64;
                let a31 = 0u64;
                nat_quot(a29, a30, a31)
            }
        };
        Ok(Adt0::C1(f12(f13(match v1.clone() {
            Adt1::C0(_, _, _, _, _, h32, _, _, _, _, _) => {
                h32
            }
        }, match v2 {
            Adt1::C0(_, _, _, _, _, h33, _, _, _, _, _) => {
                h33
            }
        }), match v1 {
            Adt1::C0(_, h34, _, _, _, _, _, _, _, _, _) => {
                h34
            }
        }, v3, v4)?, 4u64, 64u64, v3, f14(v4)?, v0))
    } else {
        Ok(Adt0::C0)
    }
}

pub fn f3(v0: u64, v1: u64, v2: Adt1, v3: Adt1) -> R<Adt0> {
    let v4: Adt1 = f15(v3.clone());
    let a36 = match v2.clone() {
        Adt1::C0(h35, _, _, _, _, _, _, _, _, _, _) => {
            h35
        }
    };
    let a37 = 0u64;
    if if if if nat_eq(a36, a37) {
        let a39 = match v3.clone() {
            Adt1::C0(h38, _, _, _, _, _, _, _, _, _, _) => {
                h38
            }
        };
        let a40 = 0u64;
        nat_eq(a39, a40)
    } else {
        false
    } {
        f4(v2.clone(), v3)
    } else {
        false
    } {
        if f5(match v2.clone() {
            Adt1::C0(_, h41, _, _, _, _, _, _, _, _, _) => {
                h41
            }
        }, match v2.clone() {
            Adt1::C0(_, _, h42, _, _, _, _, _, _, _, _) => {
                h42
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h43, _, _, _, _, _, _, _) => {
                h43
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h44, _, _, _, _, _, _) => {
                h44
            }
        }) {
            true
        } else {
            let a46 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, h45, _, _, _, _, _) => {
                    h45
                }
            };
            let a47 = 4u64;
            if if if if if nat_eq(a46, a47) {
                f16(match v2.clone() {
                    Adt1::C0(_, h48, _, _, _, _, _, _, _, _, _) => {
                        h48
                    }
                }, match v2.clone() {
                    Adt1::C0(_, _, h49, _, _, _, _, _, _, _, _) => {
                        h49
                    }
                }, match v2.clone() {
                    Adt1::C0(_, _, _, h50, _, _, _, _, _, _, _) => {
                        h50
                    }
                }, v0)?
            } else {
                false
            } {
                f8(v2.clone())
            } else {
                false
            } {
                f8(v4.clone())
            } else {
                false
            } {
                let a52 = f17(v4.clone())?;
                let a53 = match v4.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, h51, _) => {
                        h51
                    }
                };
                nat_le(a52, a53)
            } else {
                false
            } {
                if f9(v2.clone(), v4.clone())? {
                    true
                } else {
                    f10(v2.clone(), v4.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v5: u64 = if if f5(match v2.clone() {
            Adt1::C0(_, h54, _, _, _, _, _, _, _, _, _) => {
                h54
            }
        }, match v2.clone() {
            Adt1::C0(_, _, h55, _, _, _, _, _, _, _, _) => {
                h55
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h56, _, _, _, _, _, _, _) => {
                h56
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h57, _, _, _, _, _, _) => {
                h57
            }
        }) {
            true
        } else {
            f11(v4.clone())?
        } {
            0u64
        } else {
            let a59 = match v4 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h58, _) => {
                    h58
                }
            };
            let a60 = 4u64;
            let a61 = 0u64;
            nat_quot(a59, a60, a61)
        };
        Ok(Adt0::C1(f18(match v2.clone() {
            Adt1::C0(_, h62, _, _, _, _, _, _, _, _, _) => {
                h62
            }
        }, match v2.clone() {
            Adt1::C0(_, _, h63, _, _, _, _, _, _, _, _) => {
                h63
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h64, _, _, _, _, _, _, _) => {
                h64
            }
        }, match v2 {
            Adt1::C0(_, _, _, _, h65, _, _, _, _, _, _) => {
                h65
            }
        }, v0, v5)?, 4u64, 65u64, v0, f14(v5)?, v1))
    } else {
        Ok(Adt0::C0)
    }
}

pub fn f4(v0: Adt1, v1: Adt1) -> bool {
    let a68 = match v0.clone() {
        Adt1::C0(_, h66, _, _, _, _, _, _, _, _, _) => {
            h66
        }
    };
    let a69 = match v1.clone() {
        Adt1::C0(_, h67, _, _, _, _, _, _, _, _, _) => {
            h67
        }
    };
    if if if nat_eq(a68, a69) {
        let a72 = match v0.clone() {
            Adt1::C0(_, _, h70, _, _, _, _, _, _, _, _) => {
                h70
            }
        };
        let a73 = match v1.clone() {
            Adt1::C0(_, _, h71, _, _, _, _, _, _, _, _) => {
                h71
            }
        };
        nat_eq(a72, a73)
    } else {
        false
    } {
        let a76 = match v0.clone() {
            Adt1::C0(_, _, _, h74, _, _, _, _, _, _, _) => {
                h74
            }
        };
        let a77 = match v1.clone() {
            Adt1::C0(_, _, _, h75, _, _, _, _, _, _, _) => {
                h75
            }
        };
        nat_eq(a76, a77)
    } else {
        false
    } {
        let a80 = match v0 {
            Adt1::C0(_, _, _, _, h78, _, _, _, _, _, _) => {
                h78
            }
        };
        let a81 = match v1 {
            Adt1::C0(_, _, _, _, h79, _, _, _, _, _, _) => {
                h79
            }
        };
        nat_eq(a80, a81)
    } else {
        false
    }
}

pub fn f5(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a82 = v0;
    let a83 = 0u64;
    if if if nat_eq(a82, a83) {
        true
    } else {
        let a84 = v1;
        let a85 = 0u64;
        nat_eq(a84, a85)
    } {
        true
    } else {
        let a86 = v2;
        let a87 = 0u64;
        nat_eq(a86, a87)
    } {
        true
    } else {
        let a88 = v3;
        let a89 = 0u64;
        nat_eq(a88, a89)
    }
}

pub fn f6(v0: u64) -> bool {
    let a90 = v0;
    let a91 = 2139095040u64;
    if nat_le(a90, a91) {
        true
    } else {
        let a92 = v0;
        let a93 = 2147483648u64;
        nat_eq(a92, a93)
    }
}

pub fn f7(v0: Adt1, v1: Adt1) -> bool {
    if f13(match v0.clone() {
        Adt1::C0(_, _, _, _, _, h94, _, _, _, _, _) => {
            h94
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, h95, _, _, _, _, _) => {
            h95
        }
    }) {
        let a97 = match v0 {
            Adt1::C0(_, h96, _, _, _, _, _, _, _, _, _) => {
                h96
            }
        };
        let a98 = 2147483647u64;
        nat_le(a97, a98)
    } else {
        true
    }
}

pub fn f8(v0: Adt1) -> bool {
    let a103 = {
        let a100 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h99) => {
                h99
            }
        };
        let a101 = 4u64;
        let a102 = 0u64;
        nat_rem(a100, a101, a102)
    };
    let a104 = 0u64;
    if if nat_eq(a103, a104) {
        f19(match v0.clone() {
            Adt1::C0(_, h105, _, _, _, _, _, _, _, _, _) => {
                h105
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h106, _, _, _, _, _) => {
                h106
            }
        })
    } else {
        false
    } {
        f20(match v0.clone() {
            Adt1::C0(_, _, h107, _, _, _, _, _, _, _, _) => {
                h107
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h108, _, _, _, _, _, _, _) => {
                h108
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h109, _, _, _, _, _, _) => {
                h109
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h110, _, _, _, _) => {
                h110
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h111, _, _, _) => {
                h111
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h112, _, _) => {
                h112
            }
        }, 4u64)
    } else {
        false
    }
}

pub fn f9(v0: Adt1, v1: Adt1) -> R<bool> {
    let a114 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h113, _) => {
            h113
        }
    };
    let a115 = 0u64;
    if nat_eq(a114, a115) {
        Ok(true)
    } else {
        let a117 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h116, _) => {
                h116
            }
        };
        let a118 = 0u64;
        if nat_eq(a117, a118) {
            Ok(true)
        } else {
            let a120 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h119) => {
                    h119
                }
            };
            let a121 = 0u64;
            if nat_eq(a120, a121) {
                Ok(true)
            } else {
                let a123 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h122) => {
                        h122
                    }
                };
                let a124 = 0u64;
                if nat_eq(a123, a124) {
                    Ok(true)
                } else {
                    let a130 = {
                        let a127 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h125) => {
                                h125
                            }
                        };
                        let a128 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h126, _) => {
                                h126
                            }
                        };
                        nat_add(a127, a128)?
                    };
                    let a131 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h129) => {
                            h129
                        }
                    };
                    if nat_le(a130, a131) {
                        Ok(true)
                    } else {
                        let a137 = {
                            let a134 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h132) => {
                                    h132
                                }
                            };
                            let a135 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h133, _) => {
                                    h133
                                }
                            };
                            nat_add(a134, a135)?
                        };
                        let a138 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h136) => {
                                h136
                            }
                        };
                        Ok(nat_le(a137, a138))
                    }
                }
            }
        }
    }
}

pub fn f10(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f4(v0.clone(), v1.clone()) {
        let a141 = match v0.clone() {
            Adt1::C0(h139, _, _, _, _, _, _, _, _, _, _) => {
                h139
            }
        };
        let a142 = match v1.clone() {
            Adt1::C0(h140, _, _, _, _, _, _, _, _, _, _) => {
                h140
            }
        };
        nat_eq(a141, a142)
    } else {
        false
    } {
        let a145 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h143) => {
                h143
            }
        };
        let a146 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h144) => {
                h144
            }
        };
        nat_eq(a145, a146)
    } else {
        false
    } {
        let a149 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h147, _, _, _, _, _) => {
                h147
            }
        };
        let a150 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h148, _, _, _, _, _) => {
                h148
            }
        };
        nat_eq(a149, a150)
    } else {
        false
    } {
        let a153 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h151, _, _, _, _) => {
                h151
            }
        };
        let a154 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h152, _, _, _, _) => {
                h152
            }
        };
        nat_eq(a153, a154)
    } else {
        false
    } {
        let a157 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h155, _, _, _) => {
                h155
            }
        };
        let a158 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h156, _, _, _) => {
                h156
            }
        };
        nat_eq(a157, a158)
    } else {
        false
    } {
        let a161 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h159, _, _) => {
                h159
            }
        };
        let a162 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h160, _, _) => {
                h160
            }
        };
        nat_eq(a161, a162)
    } else {
        false
    }
}

pub fn f11(v0: Adt1) -> R<bool> {
    if f21(match v0.clone() {
        Adt1::C0(_, h163, _, _, _, _, _, _, _, _, _) => {
            h163
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h164, _, _, _, _, _) => {
            h164
        }
    }) {
        f22(f23(match v0.clone() {
            Adt1::C0(_, h165, _, _, _, _, _, _, _, _, _) => {
                h165
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h166, _, _, _, _, _) => {
                h166
            }
        })?, match v0.clone() {
            Adt1::C0(_, _, h167, _, _, _, _, _, _, _, _) => {
                h167
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h168, _, _, _, _, _, _, _) => {
                h168
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h169, _, _, _, _, _, _) => {
                h169
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h170, _, _, _, _) => {
                h170
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h171, _, _, _) => {
                h171
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h172, _, _) => {
                h172
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f12(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a173 = v1;
    let a174 = 0u64;
    if if nat_eq(a173, a174) {
        true
    } else {
        let a175 = v2;
        let a176 = 0u64;
        nat_eq(a175, a176)
    } {
        Ok(0u64)
    } else {
        let a179 = f24(v1, v2, v3)?;
        let a180 = {
            let a177 = v2;
            let a178 = f25(v0, v1)?;
            nat_mul(a177, a178)?
        };
        nat_add(a179, a180)
    }
}

pub fn f13(v0: u64, v1: u64) -> bool {
    let a181 = v0;
    let a182 = 4u64;
    if nat_eq(a181, a182) {
        let a183 = v1;
        let a184 = 4u64;
        nat_eq(a183, a184)
    } else {
        false
    }
}

pub fn f14(v0: u64) -> R<u64> {
    let a185 = 6u64;
    let a186 = v0;
    nat_add(a185, a186)
}

pub fn f15(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h187, _, _, _, _, _, _, _, _, _, _) => {
            h187
        }
    }, match v0.clone() {
        Adt1::C0(_, h188, _, _, _, _, _, _, _, _, _) => {
            h188
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h189, _, _, _, _, _, _, _, _) => {
            h189
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h190, _, _, _, _, _, _, _) => {
            h190
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h191, _, _, _, _, _, _) => {
            h191
        }
    }, 4u64, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h192, _, _, _, _) => {
            h192
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h193, _, _, _) => {
            h193
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h194, _, _) => {
            h194
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h195, _) => {
            h195
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h196) => {
            h196
        }
    })
}

pub fn f16(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let a197 = v0;
    let a198 = 2147483647u64;
    if if if nat_le(a197, a198) {
        let a199 = 0u64;
        let a200 = v3;
        if nat_lt(a199, a200) {
            let a201 = v3;
            let a202 = 2147483647u64;
            nat_le(a201, a202)
        } else {
            false
        }
    } else {
        false
    } {
        let a207 = {
            let a205 = {
                let a203 = v2;
                let a204 = v3;
                nat_add(a203, a204)?
            };
            let a206 = 1u64;
            nat_sub(a205, a206)
        };
        let a208 = 2147483647u64;
        nat_le(a207, a208)
    } else {
        false
    } {
        let a213 = {
            let a211 = {
                let a209 = v0;
                let a210 = v1;
                nat_mul(a209, a210)?
            };
            let a212 = f26(v2, v3)?;
            nat_mul(a211, a212)?
        };
        let a214 = 9223372036854775807u64;
        Ok(nat_le(a213, a214))
    } else {
        Ok(false)
    }
}

pub fn f17(v0: Adt1) -> R<u64> {
    let a245 = {
        let a237 = {
            let a229 = {
                let a221 = 4u64;
                let a222 = {
                    let a219 = {
                        let a216 = match v0.clone() {
                            Adt1::C0(_, h215, _, _, _, _, _, _, _, _, _) => {
                                h215
                            }
                        };
                        let a217 = 1u64;
                        nat_sub(a216, a217)
                    };
                    let a220 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, h218, _, _, _, _, _) => {
                            h218
                        }
                    };
                    nat_mul(a219, a220)?
                };
                nat_add(a221, a222)?
            };
            let a230 = {
                let a227 = {
                    let a224 = match v0.clone() {
                        Adt1::C0(_, _, h223, _, _, _, _, _, _, _, _) => {
                            h223
                        }
                    };
                    let a225 = 1u64;
                    nat_sub(a224, a225)
                };
                let a228 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, h226, _, _, _, _) => {
                        h226
                    }
                };
                nat_mul(a227, a228)?
            };
            nat_add(a229, a230)?
        };
        let a238 = {
            let a235 = {
                let a232 = match v0.clone() {
                    Adt1::C0(_, _, _, h231, _, _, _, _, _, _, _) => {
                        h231
                    }
                };
                let a233 = 1u64;
                nat_sub(a232, a233)
            };
            let a236 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h234, _, _, _) => {
                    h234
                }
            };
            nat_mul(a235, a236)?
        };
        nat_add(a237, a238)?
    };
    let a246 = {
        let a243 = {
            let a240 = match v0.clone() {
                Adt1::C0(_, _, _, _, h239, _, _, _, _, _, _) => {
                    h239
                }
            };
            let a241 = 1u64;
            nat_sub(a240, a241)
        };
        let a244 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h242, _, _) => {
                h242
            }
        };
        nat_mul(a243, a244)?
    };
    nat_add(a245, a246)
}

pub fn f18(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a247 = v0;
    let a248 = 0u64;
    if if if if nat_eq(a247, a248) {
        true
    } else {
        let a249 = v1;
        let a250 = 0u64;
        nat_eq(a249, a250)
    } {
        true
    } else {
        let a251 = v2;
        let a252 = 0u64;
        nat_eq(a251, a252)
    } {
        true
    } else {
        let a253 = v3;
        let a254 = 0u64;
        nat_eq(a253, a254)
    } {
        Ok(0u64)
    } else {
        let a255 = f27(v0, v1, v2, v3, v5)?;
        let a256 = f28(v0, v1, v2, v3, v4)?;
        nat_add(a255, a256)
    }
}

pub fn f19(v0: u64, v1: u64) -> bool {
    let a257 = v0;
    let a258 = 1u64;
    if nat_le(a257, a258) {
        true
    } else {
        let a262 = {
            let a259 = v1;
            let a260 = 4u64;
            let a261 = 0u64;
            nat_rem(a259, a260, a261)
        };
        let a263 = 0u64;
        nat_eq(a262, a263)
    }
}

pub fn f20(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f29(v0, v3, v6) {
        f29(v1, v4, v6)
    } else {
        false
    } {
        f29(v2, v5, v6)
    } else {
        false
    }
}

pub fn f21(v0: u64, v1: u64) -> bool {
    let a264 = v0;
    let a265 = 1u64;
    if nat_le(a264, a265) {
        true
    } else {
        let a266 = 4u64;
        let a267 = v1;
        nat_le(a266, a267)
    }
}

pub fn f22(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f30(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f30(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f30(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f30(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f30(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f30(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f23(v0: u64, v1: u64) -> R<u64> {
    let a268 = v0;
    let a269 = 0u64;
    if nat_eq(a268, a269) {
        Ok(0u64)
    } else {
        let a274 = 4u64;
        let a275 = {
            let a272 = {
                let a270 = v0;
                let a271 = 1u64;
                nat_sub(a270, a271)
            };
            let a273 = v1;
            nat_mul(a272, a273)?
        };
        nat_add(a274, a275)
    }
}

pub fn f24(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a276 = v2;
    let a277 = 0u64;
    if nat_eq(a276, a277) {
        Ok(0u64)
    } else {
        let a280 = 2u64;
        let a281 = {
            let a278 = v0;
            let a279 = v1;
            nat_mul(a278, a279)?
        };
        nat_mul(a280, a281)
    }
}

pub fn f25(v0: bool, v1: u64) -> R<u64> {
    let a286 = {
        let a284 = {
            let a282 = f31(v0, v1)?;
            let a283 = f32(v0, v1)?;
            nat_add(a282, a283)?
        };
        let a285 = f33(v0);
        nat_add(a284, a285)?
    };
    let a287 = v1;
    nat_add(a286, a287)
}

pub fn f26(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f34(v0, v1)?;
    let v3: u64 = {
        let a292 = {
            let a290 = {
                let a288 = v1;
                let a289 = 1u64;
                nat_sub(a288, a289)
            };
            let a291 = v2;
            nat_mul(a290, a291)?
        };
        let a293 = v0;
        nat_sub(a292, a293)
    };
    let a294 = v2;
    let a295 = v3;
    if nat_lt(a294, a295) {
        Ok(v3)
    } else {
        Ok(v2)
    }
}

pub fn f27(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a296 = v4;
    let a297 = 0u64;
    if nat_eq(a296, a297) {
        Ok(0u64)
    } else {
        let a304 = 2u64;
        let a305 = {
            let a302 = {
                let a298 = v0;
                let a299 = v1;
                nat_mul(a298, a299)?
            };
            let a303 = {
                let a300 = v2;
                let a301 = v3;
                nat_mul(a300, a301)?
            };
            nat_mul(a302, a303)?
        };
        nat_mul(a304, a305)
    }
}

pub fn f28(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a310 = v3;
    let a311 = {
        let a308 = {
            let a306 = f35(v2, v4)?;
            let a307 = f36(v0, v1, f34(v2, v4)?)?;
            nat_mul(a306, a307)?
        };
        let a309 = f36(v0, v1, f37(v2, v4)?)?;
        nat_add(a308, a309)?
    };
    nat_mul(a310, a311)
}

pub fn f29(v0: u64, v1: u64, v2: u64) -> bool {
    let a312 = v0;
    let a313 = 1u64;
    if nat_le(a312, a313) {
        true
    } else {
        let a317 = {
            let a314 = v1;
            let a315 = v2;
            let a316 = 1u64;
            nat_rem(a314, a315, a316)
        };
        let a318 = 0u64;
        nat_eq(a317, a318)
    }
}

pub fn f30(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a319 = v1;
    let a320 = 1u64;
    if if if nat_le(a319, a320) {
        true
    } else {
        let a321 = v0;
        let a322 = v2;
        nat_le(a321, a322)
    } {
        let a323 = v3;
        let a324 = 1u64;
        if nat_le(a323, a324) {
            true
        } else {
            let a331 = {
                let a329 = v0;
                let a330 = {
                    let a327 = {
                        let a325 = v1;
                        let a326 = 1u64;
                        nat_sub(a325, a326)
                    };
                    let a328 = v2;
                    nat_mul(a327, a328)?
                };
                nat_add(a329, a330)?
            };
            let a332 = v4;
            nat_le(a331, a332)
        }
    } else {
        false
    } {
        let a333 = v5;
        let a334 = 1u64;
        if nat_le(a333, a334) {
            Ok(true)
        } else {
            let a347 = {
                let a345 = {
                    let a339 = v0;
                    let a340 = {
                        let a337 = {
                            let a335 = v1;
                            let a336 = 1u64;
                            nat_sub(a335, a336)
                        };
                        let a338 = v2;
                        nat_mul(a337, a338)?
                    };
                    nat_add(a339, a340)?
                };
                let a346 = {
                    let a343 = {
                        let a341 = v3;
                        let a342 = 1u64;
                        nat_sub(a341, a342)
                    };
                    let a344 = v4;
                    nat_mul(a343, a344)?
                };
                nat_add(a345, a346)?
            };
            let a348 = v6;
            Ok(nat_le(a347, a348))
        }
    } else {
        Ok(false)
    }
}

pub fn f31(v0: bool, v1: u64) -> R<u64> {
    let a351 = if v0 {
        5u64
    } else {
        4u64
    };
    let a352 = {
        let a349 = f38(v0);
        let a350 = v1;
        nat_mul(a349, a350)?
    };
    nat_add(a351, a352)
}

pub fn f32(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a360 = {
            let a353 = 13u64;
            let a354 = f39(v1);
            nat_mul(a353, a354)?
        };
        let a361 = {
            let a358 = 4u64;
            let a359 = {
                let a355 = v1;
                let a356 = 4u64;
                let a357 = 0u64;
                nat_rem(a355, a356, a357)
            };
            nat_mul(a358, a359)?
        };
        nat_add(a360, a361)
    } else {
        let a362 = 3u64;
        let a363 = v1;
        nat_mul(a362, a363)
    }
}

pub fn f33(v0: bool) -> u64 {
    if v0 {
        6u64
    } else {
        5u64
    }
}

pub fn f34(v0: u64, v1: u64) -> R<u64> {
    let a368 = {
        let a366 = {
            let a364 = v0;
            let a365 = v1;
            nat_add(a364, a365)?
        };
        let a367 = 1u64;
        nat_sub(a366, a367)
    };
    let a369 = v1;
    let a370 = 0u64;
    Ok(nat_quot(a368, a369, a370))
}

pub fn f35(v0: u64, v1: u64) -> R<u64> {
    let a371 = v0;
    let a372 = f34(v0, v1)?;
    let a373 = 0u64;
    Ok(nat_quot(a371, a372, a373))
}

pub fn f36(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a374 = v2;
    let a375 = 0u64;
    if nat_eq(a374, a375) {
        Ok(0u64)
    } else {
        let a380 = 11u64;
        let a381 = {
            let a378 = {
                let a376 = v1;
                let a377 = v2;
                nat_mul(a376, a377)?
            };
            let a379 = f40(v0)?;
            nat_mul(a378, a379)?
        };
        nat_add(a380, a381)
    }
}

pub fn f37(v0: u64, v1: u64) -> R<u64> {
    let a382 = v0;
    let a383 = f34(v0, v1)?;
    let a384 = 0u64;
    Ok(nat_rem(a382, a383, a384))
}

pub fn f38(v0: bool) -> u64 {
    if v0 {
        2u64
    } else {
        1u64
    }
}

pub fn f39(v0: u64) -> u64 {
    let a385 = v0;
    let a386 = 4u64;
    let a387 = 0u64;
    nat_quot(a385, a386, a387)
}

pub fn f40(v0: u64) -> R<u64> {
    let a390 = {
        let a388 = 7u64;
        let a389 = v0;
        nat_mul(a388, a389)?
    };
    let a391 = 4u64;
    nat_add(a390, a391)
}
