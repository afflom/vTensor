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

pub fn f0(v0: u64, v1: u64, v2: u64, v3: bool, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64, v19: u64, v20: u64, v21: u64, v22: u64, v23: u64, v24: u64, v25: u64, v26: u64, v27: u64, v28: u64, v29: u64, v30: u64, v31: u64, v32: u64, v33: u64, v34: u64, v35: u64, v36: u64) -> R<Adt0> {
    f1(v0, v1, v2, v3, Adt1::C0(v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14), Adt1::C0(v26, v27, v28, v29, v30, v31, v32, v33, v34, v35, v36), Adt1::C0(v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25))
}

pub fn f1(v0: u64, v1: u64, v2: u64, v3: bool, v4: Adt1, v5: Adt1, v6: Adt1) -> R<Adt0> {
    let a1 = v0;
    let a2 = 23u64;
    if nat_eq(a1, a2) {
        f2(v1, v4, v6)
    } else {
        let a3 = v0;
        let a4 = 26u64;
        if nat_eq(a3, a4) {
            f3(v1, v2, v4, v6)
        } else {
            let a5 = v0;
            let a6 = 25u64;
            if nat_eq(a5, a6) {
                f4(v1, v3, v4, v5, v6)
            } else {
                Ok(Adt0::C0)
            }
        }
    }
}

pub fn f2(v0: u64, v1: Adt1, v2: Adt1) -> R<Adt0> {
    let a8 = match v1.clone() {
        Adt1::C0(h7, _, _, _, _, _, _, _, _, _, _) => {
            h7
        }
    };
    let a9 = 0u64;
    if if if if nat_eq(a8, a9) {
        let a11 = match v2.clone() {
            Adt1::C0(h10, _, _, _, _, _, _, _, _, _, _) => {
                h10
            }
        };
        let a12 = 0u64;
        nat_eq(a11, a12)
    } else {
        false
    } {
        f5(v1.clone(), v2.clone())
    } else {
        false
    } {
        if f6(match v1.clone() {
            Adt1::C0(_, h13, _, _, _, _, _, _, _, _, _) => {
                h13
            }
        }, match v1.clone() {
            Adt1::C0(_, _, h14, _, _, _, _, _, _, _, _) => {
                h14
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, h15, _, _, _, _, _, _, _) => {
                h15
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h16, _, _, _, _, _, _) => {
                h16
            }
        }) {
            true
        } else {
            if if if if f7(v0) {
                f8(v1.clone(), v2.clone())
            } else {
                false
            } {
                f9(v1.clone())
            } else {
                false
            } {
                f9(v2.clone())
            } else {
                false
            } {
                if f10(v1.clone(), v2.clone())? {
                    true
                } else {
                    f11(v1.clone(), v2.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v3: u64 = if f6(match v1.clone() {
            Adt1::C0(_, h17, _, _, _, _, _, _, _, _, _) => {
                h17
            }
        }, match v1.clone() {
            Adt1::C0(_, _, h18, _, _, _, _, _, _, _, _) => {
                h18
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, h19, _, _, _, _, _, _, _) => {
                h19
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h20, _, _, _, _, _, _) => {
                h20
            }
        }) {
            0u64
        } else {
            let a26 = match v1.clone() {
                Adt1::C0(_, _, h21, _, _, _, _, _, _, _, _) => {
                    h21
                }
            };
            let a27 = {
                let a24 = match v1.clone() {
                    Adt1::C0(_, _, _, h22, _, _, _, _, _, _, _) => {
                        h22
                    }
                };
                let a25 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h23, _, _, _, _, _, _) => {
                        h23
                    }
                };
                nat_mul(a24, a25)?
            };
            nat_mul(a26, a27)?
        };
        let v4: u64 = {
            let a28 = v3;
            let a29 = 0u64;
            if if nat_eq(a28, a29) {
                true
            } else {
                f12(v2.clone())?
            } {
                0u64
            } else {
                let a31 = match v2.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, h30, _) => {
                        h30
                    }
                };
                let a32 = 4u64;
                let a33 = 0u64;
                nat_quot(a31, a32, a33)
            }
        };
        Ok(Adt0::C1(f13(f14(match v1.clone() {
            Adt1::C0(_, _, _, _, _, h34, _, _, _, _, _) => {
                h34
            }
        }, match v2 {
            Adt1::C0(_, _, _, _, _, h35, _, _, _, _, _) => {
                h35
            }
        }), match v1 {
            Adt1::C0(_, h36, _, _, _, _, _, _, _, _, _) => {
                h36
            }
        }, v3, v4)?, 4u64, 64u64, v3, f15(v4)?, v0))
    } else {
        Ok(Adt0::C0)
    }
}

pub fn f3(v0: u64, v1: u64, v2: Adt1, v3: Adt1) -> R<Adt0> {
    let v4: Adt1 = f16(v3.clone());
    let a38 = match v2.clone() {
        Adt1::C0(h37, _, _, _, _, _, _, _, _, _, _) => {
            h37
        }
    };
    let a39 = 0u64;
    if if if if nat_eq(a38, a39) {
        let a41 = match v3.clone() {
            Adt1::C0(h40, _, _, _, _, _, _, _, _, _, _) => {
                h40
            }
        };
        let a42 = 0u64;
        nat_eq(a41, a42)
    } else {
        false
    } {
        f5(v2.clone(), v3)
    } else {
        false
    } {
        if f6(match v2.clone() {
            Adt1::C0(_, h43, _, _, _, _, _, _, _, _, _) => {
                h43
            }
        }, match v2.clone() {
            Adt1::C0(_, _, h44, _, _, _, _, _, _, _, _) => {
                h44
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h45, _, _, _, _, _, _, _) => {
                h45
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h46, _, _, _, _, _, _) => {
                h46
            }
        }) {
            true
        } else {
            let a48 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, h47, _, _, _, _, _) => {
                    h47
                }
            };
            let a49 = 4u64;
            if if if if if nat_eq(a48, a49) {
                f17(match v2.clone() {
                    Adt1::C0(_, h50, _, _, _, _, _, _, _, _, _) => {
                        h50
                    }
                }, match v2.clone() {
                    Adt1::C0(_, _, h51, _, _, _, _, _, _, _, _) => {
                        h51
                    }
                }, match v2.clone() {
                    Adt1::C0(_, _, _, h52, _, _, _, _, _, _, _) => {
                        h52
                    }
                }, v0)?
            } else {
                false
            } {
                f9(v2.clone())
            } else {
                false
            } {
                f9(v4.clone())
            } else {
                false
            } {
                let a54 = f18(v4.clone())?;
                let a55 = match v4.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, h53, _) => {
                        h53
                    }
                };
                nat_le(a54, a55)
            } else {
                false
            } {
                if f10(v2.clone(), v4.clone())? {
                    true
                } else {
                    f11(v2.clone(), v4.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v5: u64 = if if f6(match v2.clone() {
            Adt1::C0(_, h56, _, _, _, _, _, _, _, _, _) => {
                h56
            }
        }, match v2.clone() {
            Adt1::C0(_, _, h57, _, _, _, _, _, _, _, _) => {
                h57
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h58, _, _, _, _, _, _, _) => {
                h58
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h59, _, _, _, _, _, _) => {
                h59
            }
        }) {
            true
        } else {
            f12(v4.clone())?
        } {
            0u64
        } else {
            let a61 = match v4 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h60, _) => {
                    h60
                }
            };
            let a62 = 4u64;
            let a63 = 0u64;
            nat_quot(a61, a62, a63)
        };
        Ok(Adt0::C1(f19(match v2.clone() {
            Adt1::C0(_, h64, _, _, _, _, _, _, _, _, _) => {
                h64
            }
        }, match v2.clone() {
            Adt1::C0(_, _, h65, _, _, _, _, _, _, _, _) => {
                h65
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h66, _, _, _, _, _, _, _) => {
                h66
            }
        }, match v2 {
            Adt1::C0(_, _, _, _, h67, _, _, _, _, _, _) => {
                h67
            }
        }, v0, v5)?, 4u64, 65u64, v0, f15(v5)?, v1))
    } else {
        Ok(Adt0::C0)
    }
}

pub fn f4(v0: u64, v1: bool, v2: Adt1, v3: Adt1, v4: Adt1) -> R<Adt0> {
    let v5: Adt1 = f16(v4.clone());
    if if if if if if if v1 {
        let a69 = match v2.clone() {
            Adt1::C0(h68, _, _, _, _, _, _, _, _, _, _) => {
                h68
            }
        };
        let a70 = 0u64;
        nat_eq(a69, a70)
    } else {
        false
    } {
        let a72 = match v3.clone() {
            Adt1::C0(h71, _, _, _, _, _, _, _, _, _, _) => {
                h71
            }
        };
        let a73 = 0u64;
        nat_eq(a72, a73)
    } else {
        false
    } {
        let a75 = match v4.clone() {
            Adt1::C0(h74, _, _, _, _, _, _, _, _, _, _) => {
                h74
            }
        };
        let a76 = 0u64;
        nat_eq(a75, a76)
    } else {
        false
    } {
        f5(v2.clone(), v3.clone())
    } else {
        false
    } {
        f5(v2.clone(), v4)
    } else {
        false
    } {
        if f6(match v2.clone() {
            Adt1::C0(_, h77, _, _, _, _, _, _, _, _, _) => {
                h77
            }
        }, match v2.clone() {
            Adt1::C0(_, _, h78, _, _, _, _, _, _, _, _) => {
                h78
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h79, _, _, _, _, _, _, _) => {
                h79
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h80, _, _, _, _, _, _) => {
                h80
            }
        }) {
            true
        } else {
            let a82 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, h81, _, _, _, _, _) => {
                    h81
                }
            };
            let a83 = 4u64;
            if if if if if if if nat_eq(a82, a83) {
                let a85 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h84, _, _, _, _, _) => {
                        h84
                    }
                };
                let a86 = 4u64;
                nat_eq(a85, a86)
            } else {
                false
            } {
                f9(v2.clone())
            } else {
                false
            } {
                f9(v3.clone())
            } else {
                false
            } {
                f9(v5.clone())
            } else {
                false
            } {
                let a88 = f18(v5.clone())?;
                let a89 = match v5.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, h87, _) => {
                        h87
                    }
                };
                nat_le(a88, a89)
            } else {
                false
            } {
                if f10(v2.clone(), v5.clone())? {
                    true
                } else {
                    f11(v2.clone(), v5.clone())
                }
            } else {
                false
            } {
                if f10(v3.clone(), v5.clone())? {
                    true
                } else {
                    f11(v3, v5.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v6: u64 = if f6(match v2.clone() {
            Adt1::C0(_, h90, _, _, _, _, _, _, _, _, _) => {
                h90
            }
        }, match v2.clone() {
            Adt1::C0(_, _, h91, _, _, _, _, _, _, _, _) => {
                h91
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h92, _, _, _, _, _, _, _) => {
                h92
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h93, _, _, _, _, _, _) => {
                h93
            }
        }) {
            0u64
        } else {
            let a99 = match v2.clone() {
                Adt1::C0(_, _, h94, _, _, _, _, _, _, _, _) => {
                    h94
                }
            };
            let a100 = {
                let a97 = match v2.clone() {
                    Adt1::C0(_, _, _, h95, _, _, _, _, _, _, _) => {
                        h95
                    }
                };
                let a98 = match v2.clone() {
                    Adt1::C0(_, _, _, _, h96, _, _, _, _, _, _) => {
                        h96
                    }
                };
                nat_mul(a97, a98)?
            };
            nat_mul(a99, a100)?
        };
        let v7: u64 = {
            let a101 = v6;
            let a102 = 0u64;
            if if nat_eq(a101, a102) {
                true
            } else {
                f12(v5.clone())?
            } {
                0u64
            } else {
                let a104 = match v5 {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, h103, _) => {
                        h103
                    }
                };
                let a105 = 4u64;
                let a106 = 0u64;
                nat_quot(a104, a105, a106)
            }
        };
        Ok(Adt0::C1(f20(match v2 {
            Adt1::C0(_, h107, _, _, _, _, _, _, _, _, _) => {
                h107
            }
        }, v6, v7)?, 4u64, 66u64, v6, f15(v7)?, v0))
    } else {
        Ok(Adt0::C0)
    }
}

pub fn f5(v0: Adt1, v1: Adt1) -> bool {
    let a110 = match v0.clone() {
        Adt1::C0(_, h108, _, _, _, _, _, _, _, _, _) => {
            h108
        }
    };
    let a111 = match v1.clone() {
        Adt1::C0(_, h109, _, _, _, _, _, _, _, _, _) => {
            h109
        }
    };
    if if if nat_eq(a110, a111) {
        let a114 = match v0.clone() {
            Adt1::C0(_, _, h112, _, _, _, _, _, _, _, _) => {
                h112
            }
        };
        let a115 = match v1.clone() {
            Adt1::C0(_, _, h113, _, _, _, _, _, _, _, _) => {
                h113
            }
        };
        nat_eq(a114, a115)
    } else {
        false
    } {
        let a118 = match v0.clone() {
            Adt1::C0(_, _, _, h116, _, _, _, _, _, _, _) => {
                h116
            }
        };
        let a119 = match v1.clone() {
            Adt1::C0(_, _, _, h117, _, _, _, _, _, _, _) => {
                h117
            }
        };
        nat_eq(a118, a119)
    } else {
        false
    } {
        let a122 = match v0 {
            Adt1::C0(_, _, _, _, h120, _, _, _, _, _, _) => {
                h120
            }
        };
        let a123 = match v1 {
            Adt1::C0(_, _, _, _, h121, _, _, _, _, _, _) => {
                h121
            }
        };
        nat_eq(a122, a123)
    } else {
        false
    }
}

pub fn f6(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a124 = v0;
    let a125 = 0u64;
    if if if nat_eq(a124, a125) {
        true
    } else {
        let a126 = v1;
        let a127 = 0u64;
        nat_eq(a126, a127)
    } {
        true
    } else {
        let a128 = v2;
        let a129 = 0u64;
        nat_eq(a128, a129)
    } {
        true
    } else {
        let a130 = v3;
        let a131 = 0u64;
        nat_eq(a130, a131)
    }
}

pub fn f7(v0: u64) -> bool {
    let a132 = v0;
    let a133 = 2139095040u64;
    if nat_le(a132, a133) {
        true
    } else {
        let a134 = v0;
        let a135 = 2147483648u64;
        nat_eq(a134, a135)
    }
}

pub fn f8(v0: Adt1, v1: Adt1) -> bool {
    if f14(match v0.clone() {
        Adt1::C0(_, _, _, _, _, h136, _, _, _, _, _) => {
            h136
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, h137, _, _, _, _, _) => {
            h137
        }
    }) {
        let a139 = match v0 {
            Adt1::C0(_, h138, _, _, _, _, _, _, _, _, _) => {
                h138
            }
        };
        let a140 = 2147483647u64;
        nat_le(a139, a140)
    } else {
        true
    }
}

pub fn f9(v0: Adt1) -> bool {
    let a145 = {
        let a142 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h141) => {
                h141
            }
        };
        let a143 = 4u64;
        let a144 = 0u64;
        nat_rem(a142, a143, a144)
    };
    let a146 = 0u64;
    if if nat_eq(a145, a146) {
        f21(match v0.clone() {
            Adt1::C0(_, h147, _, _, _, _, _, _, _, _, _) => {
                h147
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h148, _, _, _, _, _) => {
                h148
            }
        })
    } else {
        false
    } {
        f22(match v0.clone() {
            Adt1::C0(_, _, h149, _, _, _, _, _, _, _, _) => {
                h149
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h150, _, _, _, _, _, _, _) => {
                h150
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h151, _, _, _, _, _, _) => {
                h151
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h152, _, _, _, _) => {
                h152
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h153, _, _, _) => {
                h153
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h154, _, _) => {
                h154
            }
        }, 4u64)
    } else {
        false
    }
}

pub fn f10(v0: Adt1, v1: Adt1) -> R<bool> {
    let a156 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h155, _) => {
            h155
        }
    };
    let a157 = 0u64;
    if nat_eq(a156, a157) {
        Ok(true)
    } else {
        let a159 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h158, _) => {
                h158
            }
        };
        let a160 = 0u64;
        if nat_eq(a159, a160) {
            Ok(true)
        } else {
            let a162 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h161) => {
                    h161
                }
            };
            let a163 = 0u64;
            if nat_eq(a162, a163) {
                Ok(true)
            } else {
                let a165 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h164) => {
                        h164
                    }
                };
                let a166 = 0u64;
                if nat_eq(a165, a166) {
                    Ok(true)
                } else {
                    let a172 = {
                        let a169 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h167) => {
                                h167
                            }
                        };
                        let a170 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h168, _) => {
                                h168
                            }
                        };
                        nat_add(a169, a170)?
                    };
                    let a173 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h171) => {
                            h171
                        }
                    };
                    if nat_le(a172, a173) {
                        Ok(true)
                    } else {
                        let a179 = {
                            let a176 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h174) => {
                                    h174
                                }
                            };
                            let a177 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h175, _) => {
                                    h175
                                }
                            };
                            nat_add(a176, a177)?
                        };
                        let a180 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h178) => {
                                h178
                            }
                        };
                        Ok(nat_le(a179, a180))
                    }
                }
            }
        }
    }
}

pub fn f11(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f5(v0.clone(), v1.clone()) {
        let a183 = match v0.clone() {
            Adt1::C0(h181, _, _, _, _, _, _, _, _, _, _) => {
                h181
            }
        };
        let a184 = match v1.clone() {
            Adt1::C0(h182, _, _, _, _, _, _, _, _, _, _) => {
                h182
            }
        };
        nat_eq(a183, a184)
    } else {
        false
    } {
        let a187 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h185) => {
                h185
            }
        };
        let a188 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h186) => {
                h186
            }
        };
        nat_eq(a187, a188)
    } else {
        false
    } {
        let a191 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h189, _, _, _, _, _) => {
                h189
            }
        };
        let a192 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h190, _, _, _, _, _) => {
                h190
            }
        };
        nat_eq(a191, a192)
    } else {
        false
    } {
        let a195 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h193, _, _, _, _) => {
                h193
            }
        };
        let a196 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h194, _, _, _, _) => {
                h194
            }
        };
        nat_eq(a195, a196)
    } else {
        false
    } {
        let a199 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h197, _, _, _) => {
                h197
            }
        };
        let a200 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h198, _, _, _) => {
                h198
            }
        };
        nat_eq(a199, a200)
    } else {
        false
    } {
        let a203 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h201, _, _) => {
                h201
            }
        };
        let a204 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h202, _, _) => {
                h202
            }
        };
        nat_eq(a203, a204)
    } else {
        false
    }
}

pub fn f12(v0: Adt1) -> R<bool> {
    if f23(match v0.clone() {
        Adt1::C0(_, h205, _, _, _, _, _, _, _, _, _) => {
            h205
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h206, _, _, _, _, _) => {
            h206
        }
    }) {
        f24(f25(match v0.clone() {
            Adt1::C0(_, h207, _, _, _, _, _, _, _, _, _) => {
                h207
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h208, _, _, _, _, _) => {
                h208
            }
        })?, match v0.clone() {
            Adt1::C0(_, _, h209, _, _, _, _, _, _, _, _) => {
                h209
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h210, _, _, _, _, _, _, _) => {
                h210
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h211, _, _, _, _, _, _) => {
                h211
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h212, _, _, _, _) => {
                h212
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h213, _, _, _) => {
                h213
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h214, _, _) => {
                h214
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f13(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a215 = v1;
    let a216 = 0u64;
    if if nat_eq(a215, a216) {
        true
    } else {
        let a217 = v2;
        let a218 = 0u64;
        nat_eq(a217, a218)
    } {
        Ok(0u64)
    } else {
        let a221 = f26(v1, v2, v3)?;
        let a222 = {
            let a219 = v2;
            let a220 = f27(v0, v1)?;
            nat_mul(a219, a220)?
        };
        nat_add(a221, a222)
    }
}

pub fn f14(v0: u64, v1: u64) -> bool {
    let a223 = v0;
    let a224 = 4u64;
    if nat_eq(a223, a224) {
        let a225 = v1;
        let a226 = 4u64;
        nat_eq(a225, a226)
    } else {
        false
    }
}

pub fn f15(v0: u64) -> R<u64> {
    let a227 = 6u64;
    let a228 = v0;
    nat_add(a227, a228)
}

pub fn f16(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h229, _, _, _, _, _, _, _, _, _, _) => {
            h229
        }
    }, match v0.clone() {
        Adt1::C0(_, h230, _, _, _, _, _, _, _, _, _) => {
            h230
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h231, _, _, _, _, _, _, _, _) => {
            h231
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h232, _, _, _, _, _, _, _) => {
            h232
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h233, _, _, _, _, _, _) => {
            h233
        }
    }, 4u64, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h234, _, _, _, _) => {
            h234
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h235, _, _, _) => {
            h235
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, h236, _, _) => {
            h236
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h237, _) => {
            h237
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h238) => {
            h238
        }
    })
}

pub fn f17(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let a239 = v0;
    let a240 = 2147483647u64;
    if if if nat_le(a239, a240) {
        let a241 = 0u64;
        let a242 = v3;
        if nat_lt(a241, a242) {
            let a243 = v3;
            let a244 = 2147483647u64;
            nat_le(a243, a244)
        } else {
            false
        }
    } else {
        false
    } {
        let a249 = {
            let a247 = {
                let a245 = v2;
                let a246 = v3;
                nat_add(a245, a246)?
            };
            let a248 = 1u64;
            nat_sub(a247, a248)
        };
        let a250 = 2147483647u64;
        nat_le(a249, a250)
    } else {
        false
    } {
        let a255 = {
            let a253 = {
                let a251 = v0;
                let a252 = v1;
                nat_mul(a251, a252)?
            };
            let a254 = f28(v2, v3)?;
            nat_mul(a253, a254)?
        };
        let a256 = 9223372036854775807u64;
        Ok(nat_le(a255, a256))
    } else {
        Ok(false)
    }
}

pub fn f18(v0: Adt1) -> R<u64> {
    let a287 = {
        let a279 = {
            let a271 = {
                let a263 = 4u64;
                let a264 = {
                    let a261 = {
                        let a258 = match v0.clone() {
                            Adt1::C0(_, h257, _, _, _, _, _, _, _, _, _) => {
                                h257
                            }
                        };
                        let a259 = 1u64;
                        nat_sub(a258, a259)
                    };
                    let a262 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, h260, _, _, _, _, _) => {
                            h260
                        }
                    };
                    nat_mul(a261, a262)?
                };
                nat_add(a263, a264)?
            };
            let a272 = {
                let a269 = {
                    let a266 = match v0.clone() {
                        Adt1::C0(_, _, h265, _, _, _, _, _, _, _, _) => {
                            h265
                        }
                    };
                    let a267 = 1u64;
                    nat_sub(a266, a267)
                };
                let a270 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, h268, _, _, _, _) => {
                        h268
                    }
                };
                nat_mul(a269, a270)?
            };
            nat_add(a271, a272)?
        };
        let a280 = {
            let a277 = {
                let a274 = match v0.clone() {
                    Adt1::C0(_, _, _, h273, _, _, _, _, _, _, _) => {
                        h273
                    }
                };
                let a275 = 1u64;
                nat_sub(a274, a275)
            };
            let a278 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h276, _, _, _) => {
                    h276
                }
            };
            nat_mul(a277, a278)?
        };
        nat_add(a279, a280)?
    };
    let a288 = {
        let a285 = {
            let a282 = match v0.clone() {
                Adt1::C0(_, _, _, _, h281, _, _, _, _, _, _) => {
                    h281
                }
            };
            let a283 = 1u64;
            nat_sub(a282, a283)
        };
        let a286 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h284, _, _) => {
                h284
            }
        };
        nat_mul(a285, a286)?
    };
    nat_add(a287, a288)
}

pub fn f19(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a289 = v0;
    let a290 = 0u64;
    if if if if nat_eq(a289, a290) {
        true
    } else {
        let a291 = v1;
        let a292 = 0u64;
        nat_eq(a291, a292)
    } {
        true
    } else {
        let a293 = v2;
        let a294 = 0u64;
        nat_eq(a293, a294)
    } {
        true
    } else {
        let a295 = v3;
        let a296 = 0u64;
        nat_eq(a295, a296)
    } {
        Ok(0u64)
    } else {
        let a297 = f29(v0, v1, v2, v3, v5)?;
        let a298 = f30(v0, v1, v2, v3, v4)?;
        nat_add(a297, a298)
    }
}

pub fn f20(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a299 = v0;
    let a300 = 0u64;
    if if nat_eq(a299, a300) {
        true
    } else {
        let a301 = v1;
        let a302 = 0u64;
        nat_eq(a301, a302)
    } {
        Ok(0u64)
    } else {
        let a305 = f26(v0, v1, v2)?;
        let a306 = {
            let a303 = v1;
            let a304 = f31(v0)?;
            nat_mul(a303, a304)?
        };
        nat_add(a305, a306)
    }
}

pub fn f21(v0: u64, v1: u64) -> bool {
    let a307 = v0;
    let a308 = 1u64;
    if nat_le(a307, a308) {
        true
    } else {
        let a312 = {
            let a309 = v1;
            let a310 = 4u64;
            let a311 = 0u64;
            nat_rem(a309, a310, a311)
        };
        let a313 = 0u64;
        nat_eq(a312, a313)
    }
}

pub fn f22(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f32(v0, v3, v6) {
        f32(v1, v4, v6)
    } else {
        false
    } {
        f32(v2, v5, v6)
    } else {
        false
    }
}

pub fn f23(v0: u64, v1: u64) -> bool {
    let a314 = v0;
    let a315 = 1u64;
    if nat_le(a314, a315) {
        true
    } else {
        let a316 = 4u64;
        let a317 = v1;
        nat_le(a316, a317)
    }
}

pub fn f24(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f33(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f33(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f33(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f33(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f33(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f33(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f25(v0: u64, v1: u64) -> R<u64> {
    let a318 = v0;
    let a319 = 0u64;
    if nat_eq(a318, a319) {
        Ok(0u64)
    } else {
        let a324 = 4u64;
        let a325 = {
            let a322 = {
                let a320 = v0;
                let a321 = 1u64;
                nat_sub(a320, a321)
            };
            let a323 = v1;
            nat_mul(a322, a323)?
        };
        nat_add(a324, a325)
    }
}

pub fn f26(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a326 = v2;
    let a327 = 0u64;
    if nat_eq(a326, a327) {
        Ok(0u64)
    } else {
        let a330 = 2u64;
        let a331 = {
            let a328 = v0;
            let a329 = v1;
            nat_mul(a328, a329)?
        };
        nat_mul(a330, a331)
    }
}

pub fn f27(v0: bool, v1: u64) -> R<u64> {
    let a336 = {
        let a334 = {
            let a332 = f34(v0, v1)?;
            let a333 = f35(v0, v1)?;
            nat_add(a332, a333)?
        };
        let a335 = f36(v0);
        nat_add(a334, a335)?
    };
    let a337 = v1;
    nat_add(a336, a337)
}

pub fn f28(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f37(v0, v1)?;
    let v3: u64 = {
        let a342 = {
            let a340 = {
                let a338 = v1;
                let a339 = 1u64;
                nat_sub(a338, a339)
            };
            let a341 = v2;
            nat_mul(a340, a341)?
        };
        let a343 = v0;
        nat_sub(a342, a343)
    };
    let a344 = v2;
    let a345 = v3;
    if nat_lt(a344, a345) {
        Ok(v3)
    } else {
        Ok(v2)
    }
}

pub fn f29(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a346 = v4;
    let a347 = 0u64;
    if nat_eq(a346, a347) {
        Ok(0u64)
    } else {
        let a354 = 2u64;
        let a355 = {
            let a352 = {
                let a348 = v0;
                let a349 = v1;
                nat_mul(a348, a349)?
            };
            let a353 = {
                let a350 = v2;
                let a351 = v3;
                nat_mul(a350, a351)?
            };
            nat_mul(a352, a353)?
        };
        nat_mul(a354, a355)
    }
}

pub fn f30(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a360 = v3;
    let a361 = {
        let a358 = {
            let a356 = f38(v2, v4)?;
            let a357 = f39(v0, v1, f37(v2, v4)?)?;
            nat_mul(a356, a357)?
        };
        let a359 = f39(v0, v1, f40(v2, v4)?)?;
        nat_add(a358, a359)?
    };
    nat_mul(a360, a361)
}

pub fn f31(v0: u64) -> R<u64> {
    let a364 = 13u64;
    let a365 = {
        let a362 = 9u64;
        let a363 = v0;
        nat_mul(a362, a363)?
    };
    nat_add(a364, a365)
}

pub fn f32(v0: u64, v1: u64, v2: u64) -> bool {
    let a366 = v0;
    let a367 = 1u64;
    if nat_le(a366, a367) {
        true
    } else {
        let a371 = {
            let a368 = v1;
            let a369 = v2;
            let a370 = 1u64;
            nat_rem(a368, a369, a370)
        };
        let a372 = 0u64;
        nat_eq(a371, a372)
    }
}

pub fn f33(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a373 = v1;
    let a374 = 1u64;
    if if if nat_le(a373, a374) {
        true
    } else {
        let a375 = v0;
        let a376 = v2;
        nat_le(a375, a376)
    } {
        let a377 = v3;
        let a378 = 1u64;
        if nat_le(a377, a378) {
            true
        } else {
            let a385 = {
                let a383 = v0;
                let a384 = {
                    let a381 = {
                        let a379 = v1;
                        let a380 = 1u64;
                        nat_sub(a379, a380)
                    };
                    let a382 = v2;
                    nat_mul(a381, a382)?
                };
                nat_add(a383, a384)?
            };
            let a386 = v4;
            nat_le(a385, a386)
        }
    } else {
        false
    } {
        let a387 = v5;
        let a388 = 1u64;
        if nat_le(a387, a388) {
            Ok(true)
        } else {
            let a401 = {
                let a399 = {
                    let a393 = v0;
                    let a394 = {
                        let a391 = {
                            let a389 = v1;
                            let a390 = 1u64;
                            nat_sub(a389, a390)
                        };
                        let a392 = v2;
                        nat_mul(a391, a392)?
                    };
                    nat_add(a393, a394)?
                };
                let a400 = {
                    let a397 = {
                        let a395 = v3;
                        let a396 = 1u64;
                        nat_sub(a395, a396)
                    };
                    let a398 = v4;
                    nat_mul(a397, a398)?
                };
                nat_add(a399, a400)?
            };
            let a402 = v6;
            Ok(nat_le(a401, a402))
        }
    } else {
        Ok(false)
    }
}

pub fn f34(v0: bool, v1: u64) -> R<u64> {
    let a405 = if v0 {
        5u64
    } else {
        4u64
    };
    let a406 = {
        let a403 = f41(v0);
        let a404 = v1;
        nat_mul(a403, a404)?
    };
    nat_add(a405, a406)
}

pub fn f35(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a414 = {
            let a407 = 13u64;
            let a408 = f42(v1);
            nat_mul(a407, a408)?
        };
        let a415 = {
            let a412 = 4u64;
            let a413 = {
                let a409 = v1;
                let a410 = 4u64;
                let a411 = 0u64;
                nat_rem(a409, a410, a411)
            };
            nat_mul(a412, a413)?
        };
        nat_add(a414, a415)
    } else {
        let a416 = 3u64;
        let a417 = v1;
        nat_mul(a416, a417)
    }
}

pub fn f36(v0: bool) -> u64 {
    if v0 {
        6u64
    } else {
        5u64
    }
}

pub fn f37(v0: u64, v1: u64) -> R<u64> {
    let a422 = {
        let a420 = {
            let a418 = v0;
            let a419 = v1;
            nat_add(a418, a419)?
        };
        let a421 = 1u64;
        nat_sub(a420, a421)
    };
    let a423 = v1;
    let a424 = 0u64;
    Ok(nat_quot(a422, a423, a424))
}

pub fn f38(v0: u64, v1: u64) -> R<u64> {
    let a425 = v0;
    let a426 = f37(v0, v1)?;
    let a427 = 0u64;
    Ok(nat_quot(a425, a426, a427))
}

pub fn f39(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a428 = v2;
    let a429 = 0u64;
    if nat_eq(a428, a429) {
        Ok(0u64)
    } else {
        let a434 = 11u64;
        let a435 = {
            let a432 = {
                let a430 = v1;
                let a431 = v2;
                nat_mul(a430, a431)?
            };
            let a433 = f43(v0)?;
            nat_mul(a432, a433)?
        };
        nat_add(a434, a435)
    }
}

pub fn f40(v0: u64, v1: u64) -> R<u64> {
    let a436 = v0;
    let a437 = f37(v0, v1)?;
    let a438 = 0u64;
    Ok(nat_rem(a436, a437, a438))
}

pub fn f41(v0: bool) -> u64 {
    if v0 {
        2u64
    } else {
        1u64
    }
}

pub fn f42(v0: u64) -> u64 {
    let a439 = v0;
    let a440 = 4u64;
    let a441 = 0u64;
    nat_quot(a439, a440, a441)
}

pub fn f43(v0: u64) -> R<u64> {
    let a444 = {
        let a442 = 7u64;
        let a443 = v0;
        nat_mul(a442, a443)?
    };
    let a445 = 4u64;
    nat_add(a444, a445)
}

pub fn run(p0: u64, p1: u64, p2: u64, p3: bool, p4: u64, p5: u64, p6: u64, p7: u64, p8: u64, p9: u64, p10: u64, p11: u64, p12: u64, p13: u64, p14: u64, p15: u64, p16: u64, p17: u64, p18: u64, p19: u64, p20: u64, p21: u64, p22: u64, p23: u64, p24: u64, p25: u64, p26: u64, p27: u64, p28: u64, p29: u64, p30: u64, p31: u64, p32: u64, p33: u64, p34: u64, p35: u64, p36: u64) -> R<Adt0> {
    f0(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15, p16, p17, p18, p19, p20, p21, p22, p23, p24, p25, p26, p27, p28, p29, p30, p31, p32, p33, p34, p35, p36)
}
