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
    C1(u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

pub fn f0(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64, v19: u64, v20: u64, v21: u64, v22: u64) -> R<Adt0> {
    f1(v0, Adt1::C0(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11), Adt1::C0(v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22))
}

pub fn f1(v0: u64, v1: Adt1, v2: Adt1) -> R<Adt0> {
    let a2 = match v1.clone() {
        Adt1::C0(h1, _, _, _, _, _, _, _, _, _, _) => {
            h1
        }
    };
    let a3 = 0u64;
    if if if if nat_eq(a2, a3) {
        let a5 = match v2.clone() {
            Adt1::C0(h4, _, _, _, _, _, _, _, _, _, _) => {
                h4
            }
        };
        let a6 = 0u64;
        nat_eq(a5, a6)
    } else {
        false
    } {
        f2(v1.clone(), v2.clone())
    } else {
        false
    } {
        if f3(match v1.clone() {
            Adt1::C0(_, h7, _, _, _, _, _, _, _, _, _) => {
                h7
            }
        }, match v1.clone() {
            Adt1::C0(_, _, h8, _, _, _, _, _, _, _, _) => {
                h8
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, h9, _, _, _, _, _, _, _) => {
                h9
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h10, _, _, _, _, _, _) => {
                h10
            }
        }) {
            true
        } else {
            if if if if f4(v0) {
                f5(v1.clone(), v2.clone())
            } else {
                false
            } {
                f6(v1.clone())
            } else {
                false
            } {
                f6(v2.clone())
            } else {
                false
            } {
                if f7(v1.clone(), v2.clone())? {
                    true
                } else {
                    f8(v1.clone(), v2.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v3: u64 = if f3(match v1.clone() {
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
            0u64
        } else {
            let a20 = match v1.clone() {
                Adt1::C0(_, _, h15, _, _, _, _, _, _, _, _) => {
                    h15
                }
            };
            let a21 = {
                let a18 = match v1.clone() {
                    Adt1::C0(_, _, _, h16, _, _, _, _, _, _, _) => {
                        h16
                    }
                };
                let a19 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h17, _, _, _, _, _, _) => {
                        h17
                    }
                };
                nat_mul(a18, a19)?
            };
            nat_mul(a20, a21)?
        };
        let v4: u64 = {
            let a22 = v3;
            let a23 = 0u64;
            if if nat_eq(a22, a23) {
                true
            } else {
                f9(v2.clone())?
            } {
                0u64
            } else {
                let a25 = match v2.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, h24, _) => {
                        h24
                    }
                };
                let a26 = 4u64;
                let a27 = 0u64;
                nat_quot(a25, a26, a27)
            }
        };
        Ok(Adt0::C1(f10(f11(match v1.clone() {
            Adt1::C0(_, _, _, _, _, h28, _, _, _, _, _) => {
                h28
            }
        }, match v2 {
            Adt1::C0(_, _, _, _, _, h29, _, _, _, _, _) => {
                h29
            }
        }), match v1 {
            Adt1::C0(_, h30, _, _, _, _, _, _, _, _, _) => {
                h30
            }
        }, v3, v4)?, 4u64, 64u64, v3, f12(v4)?))
    } else {
        Ok(Adt0::C0)
    }
}

pub fn f2(v0: Adt1, v1: Adt1) -> bool {
    let a33 = match v0.clone() {
        Adt1::C0(_, h31, _, _, _, _, _, _, _, _, _) => {
            h31
        }
    };
    let a34 = match v1.clone() {
        Adt1::C0(_, h32, _, _, _, _, _, _, _, _, _) => {
            h32
        }
    };
    if if if nat_eq(a33, a34) {
        let a37 = match v0.clone() {
            Adt1::C0(_, _, h35, _, _, _, _, _, _, _, _) => {
                h35
            }
        };
        let a38 = match v1.clone() {
            Adt1::C0(_, _, h36, _, _, _, _, _, _, _, _) => {
                h36
            }
        };
        nat_eq(a37, a38)
    } else {
        false
    } {
        let a41 = match v0.clone() {
            Adt1::C0(_, _, _, h39, _, _, _, _, _, _, _) => {
                h39
            }
        };
        let a42 = match v1.clone() {
            Adt1::C0(_, _, _, h40, _, _, _, _, _, _, _) => {
                h40
            }
        };
        nat_eq(a41, a42)
    } else {
        false
    } {
        let a45 = match v0 {
            Adt1::C0(_, _, _, _, h43, _, _, _, _, _, _) => {
                h43
            }
        };
        let a46 = match v1 {
            Adt1::C0(_, _, _, _, h44, _, _, _, _, _, _) => {
                h44
            }
        };
        nat_eq(a45, a46)
    } else {
        false
    }
}

pub fn f3(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a47 = v0;
    let a48 = 0u64;
    if if if nat_eq(a47, a48) {
        true
    } else {
        let a49 = v1;
        let a50 = 0u64;
        nat_eq(a49, a50)
    } {
        true
    } else {
        let a51 = v2;
        let a52 = 0u64;
        nat_eq(a51, a52)
    } {
        true
    } else {
        let a53 = v3;
        let a54 = 0u64;
        nat_eq(a53, a54)
    }
}

pub fn f4(v0: u64) -> bool {
    let a55 = v0;
    let a56 = 2139095040u64;
    if nat_le(a55, a56) {
        true
    } else {
        let a57 = v0;
        let a58 = 2147483648u64;
        nat_eq(a57, a58)
    }
}

pub fn f5(v0: Adt1, v1: Adt1) -> bool {
    if f11(match v0.clone() {
        Adt1::C0(_, _, _, _, _, h59, _, _, _, _, _) => {
            h59
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, h60, _, _, _, _, _) => {
            h60
        }
    }) {
        let a62 = match v0 {
            Adt1::C0(_, h61, _, _, _, _, _, _, _, _, _) => {
                h61
            }
        };
        let a63 = 2147483647u64;
        nat_le(a62, a63)
    } else {
        true
    }
}

pub fn f6(v0: Adt1) -> bool {
    let a68 = {
        let a65 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h64) => {
                h64
            }
        };
        let a66 = 4u64;
        let a67 = 0u64;
        nat_rem(a65, a66, a67)
    };
    let a69 = 0u64;
    if if nat_eq(a68, a69) {
        f13(match v0.clone() {
            Adt1::C0(_, h70, _, _, _, _, _, _, _, _, _) => {
                h70
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h71, _, _, _, _, _) => {
                h71
            }
        })
    } else {
        false
    } {
        f14(match v0.clone() {
            Adt1::C0(_, _, h72, _, _, _, _, _, _, _, _) => {
                h72
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h73, _, _, _, _, _, _, _) => {
                h73
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h74, _, _, _, _, _, _) => {
                h74
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h75, _, _, _, _) => {
                h75
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h76, _, _, _) => {
                h76
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h77, _, _) => {
                h77
            }
        }, 4u64)
    } else {
        false
    }
}

pub fn f7(v0: Adt1, v1: Adt1) -> R<bool> {
    let a79 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h78, _) => {
            h78
        }
    };
    let a80 = 0u64;
    if nat_eq(a79, a80) {
        Ok(true)
    } else {
        let a82 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h81, _) => {
                h81
            }
        };
        let a83 = 0u64;
        if nat_eq(a82, a83) {
            Ok(true)
        } else {
            let a85 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h84) => {
                    h84
                }
            };
            let a86 = 0u64;
            if nat_eq(a85, a86) {
                Ok(true)
            } else {
                let a88 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h87) => {
                        h87
                    }
                };
                let a89 = 0u64;
                if nat_eq(a88, a89) {
                    Ok(true)
                } else {
                    let a95 = {
                        let a92 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h90) => {
                                h90
                            }
                        };
                        let a93 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h91, _) => {
                                h91
                            }
                        };
                        nat_add(a92, a93)?
                    };
                    let a96 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h94) => {
                            h94
                        }
                    };
                    if nat_le(a95, a96) {
                        Ok(true)
                    } else {
                        let a102 = {
                            let a99 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h97) => {
                                    h97
                                }
                            };
                            let a100 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h98, _) => {
                                    h98
                                }
                            };
                            nat_add(a99, a100)?
                        };
                        let a103 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h101) => {
                                h101
                            }
                        };
                        Ok(nat_le(a102, a103))
                    }
                }
            }
        }
    }
}

pub fn f8(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f2(v0.clone(), v1.clone()) {
        let a106 = match v0.clone() {
            Adt1::C0(h104, _, _, _, _, _, _, _, _, _, _) => {
                h104
            }
        };
        let a107 = match v1.clone() {
            Adt1::C0(h105, _, _, _, _, _, _, _, _, _, _) => {
                h105
            }
        };
        nat_eq(a106, a107)
    } else {
        false
    } {
        let a110 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h108) => {
                h108
            }
        };
        let a111 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h109) => {
                h109
            }
        };
        nat_eq(a110, a111)
    } else {
        false
    } {
        let a114 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h112, _, _, _, _, _) => {
                h112
            }
        };
        let a115 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h113, _, _, _, _, _) => {
                h113
            }
        };
        nat_eq(a114, a115)
    } else {
        false
    } {
        let a118 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h116, _, _, _, _) => {
                h116
            }
        };
        let a119 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h117, _, _, _, _) => {
                h117
            }
        };
        nat_eq(a118, a119)
    } else {
        false
    } {
        let a122 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h120, _, _, _) => {
                h120
            }
        };
        let a123 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h121, _, _, _) => {
                h121
            }
        };
        nat_eq(a122, a123)
    } else {
        false
    } {
        let a126 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h124, _, _) => {
                h124
            }
        };
        let a127 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h125, _, _) => {
                h125
            }
        };
        nat_eq(a126, a127)
    } else {
        false
    }
}

pub fn f9(v0: Adt1) -> R<bool> {
    if f15(match v0.clone() {
        Adt1::C0(_, h128, _, _, _, _, _, _, _, _, _) => {
            h128
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h129, _, _, _, _, _) => {
            h129
        }
    }) {
        f16(f17(match v0.clone() {
            Adt1::C0(_, h130, _, _, _, _, _, _, _, _, _) => {
                h130
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h131, _, _, _, _, _) => {
                h131
            }
        })?, match v0.clone() {
            Adt1::C0(_, _, h132, _, _, _, _, _, _, _, _) => {
                h132
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h133, _, _, _, _, _, _, _) => {
                h133
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h134, _, _, _, _, _, _) => {
                h134
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h135, _, _, _, _) => {
                h135
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h136, _, _, _) => {
                h136
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h137, _, _) => {
                h137
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f10(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a138 = v1;
    let a139 = 0u64;
    if if nat_eq(a138, a139) {
        true
    } else {
        let a140 = v2;
        let a141 = 0u64;
        nat_eq(a140, a141)
    } {
        Ok(0u64)
    } else {
        let a144 = f18(v1, v2, v3)?;
        let a145 = {
            let a142 = v2;
            let a143 = f19(v0, v1)?;
            nat_mul(a142, a143)?
        };
        nat_add(a144, a145)
    }
}

pub fn f11(v0: u64, v1: u64) -> bool {
    let a146 = v0;
    let a147 = 4u64;
    if nat_eq(a146, a147) {
        let a148 = v1;
        let a149 = 4u64;
        nat_eq(a148, a149)
    } else {
        false
    }
}

pub fn f12(v0: u64) -> R<u64> {
    let a150 = 6u64;
    let a151 = v0;
    nat_add(a150, a151)
}

pub fn f13(v0: u64, v1: u64) -> bool {
    let a152 = v0;
    let a153 = 1u64;
    if nat_le(a152, a153) {
        true
    } else {
        let a157 = {
            let a154 = v1;
            let a155 = 4u64;
            let a156 = 0u64;
            nat_rem(a154, a155, a156)
        };
        let a158 = 0u64;
        nat_eq(a157, a158)
    }
}

pub fn f14(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f20(v0, v3, v6) {
        f20(v1, v4, v6)
    } else {
        false
    } {
        f20(v2, v5, v6)
    } else {
        false
    }
}

pub fn f15(v0: u64, v1: u64) -> bool {
    let a159 = v0;
    let a160 = 1u64;
    if nat_le(a159, a160) {
        true
    } else {
        let a161 = 4u64;
        let a162 = v1;
        nat_le(a161, a162)
    }
}

pub fn f16(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f21(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f21(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f21(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f21(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f21(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f21(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f17(v0: u64, v1: u64) -> R<u64> {
    let a163 = v0;
    let a164 = 0u64;
    if nat_eq(a163, a164) {
        Ok(0u64)
    } else {
        let a169 = 4u64;
        let a170 = {
            let a167 = {
                let a165 = v0;
                let a166 = 1u64;
                nat_sub(a165, a166)
            };
            let a168 = v1;
            nat_mul(a167, a168)?
        };
        nat_add(a169, a170)
    }
}

pub fn f18(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a171 = v2;
    let a172 = 0u64;
    if nat_eq(a171, a172) {
        Ok(0u64)
    } else {
        let a175 = 2u64;
        let a176 = {
            let a173 = v0;
            let a174 = v1;
            nat_mul(a173, a174)?
        };
        nat_mul(a175, a176)
    }
}

pub fn f19(v0: bool, v1: u64) -> R<u64> {
    let a181 = {
        let a179 = {
            let a177 = f22(v0, v1)?;
            let a178 = f23(v0, v1)?;
            nat_add(a177, a178)?
        };
        let a180 = f24(v0);
        nat_add(a179, a180)?
    };
    let a182 = v1;
    nat_add(a181, a182)
}

pub fn f20(v0: u64, v1: u64, v2: u64) -> bool {
    let a183 = v0;
    let a184 = 1u64;
    if nat_le(a183, a184) {
        true
    } else {
        let a188 = {
            let a185 = v1;
            let a186 = v2;
            let a187 = 1u64;
            nat_rem(a185, a186, a187)
        };
        let a189 = 0u64;
        nat_eq(a188, a189)
    }
}

pub fn f21(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a190 = v1;
    let a191 = 1u64;
    if if if nat_le(a190, a191) {
        true
    } else {
        let a192 = v0;
        let a193 = v2;
        nat_le(a192, a193)
    } {
        let a194 = v3;
        let a195 = 1u64;
        if nat_le(a194, a195) {
            true
        } else {
            let a202 = {
                let a200 = v0;
                let a201 = {
                    let a198 = {
                        let a196 = v1;
                        let a197 = 1u64;
                        nat_sub(a196, a197)
                    };
                    let a199 = v2;
                    nat_mul(a198, a199)?
                };
                nat_add(a200, a201)?
            };
            let a203 = v4;
            nat_le(a202, a203)
        }
    } else {
        false
    } {
        let a204 = v5;
        let a205 = 1u64;
        if nat_le(a204, a205) {
            Ok(true)
        } else {
            let a218 = {
                let a216 = {
                    let a210 = v0;
                    let a211 = {
                        let a208 = {
                            let a206 = v1;
                            let a207 = 1u64;
                            nat_sub(a206, a207)
                        };
                        let a209 = v2;
                        nat_mul(a208, a209)?
                    };
                    nat_add(a210, a211)?
                };
                let a217 = {
                    let a214 = {
                        let a212 = v3;
                        let a213 = 1u64;
                        nat_sub(a212, a213)
                    };
                    let a215 = v4;
                    nat_mul(a214, a215)?
                };
                nat_add(a216, a217)?
            };
            let a219 = v6;
            Ok(nat_le(a218, a219))
        }
    } else {
        Ok(false)
    }
}

pub fn f22(v0: bool, v1: u64) -> R<u64> {
    let a222 = if v0 {
        5u64
    } else {
        4u64
    };
    let a223 = {
        let a220 = f25(v0);
        let a221 = v1;
        nat_mul(a220, a221)?
    };
    nat_add(a222, a223)
}

pub fn f23(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a231 = {
            let a224 = 13u64;
            let a225 = f26(v1);
            nat_mul(a224, a225)?
        };
        let a232 = {
            let a229 = 4u64;
            let a230 = {
                let a226 = v1;
                let a227 = 4u64;
                let a228 = 0u64;
                nat_rem(a226, a227, a228)
            };
            nat_mul(a229, a230)?
        };
        nat_add(a231, a232)
    } else {
        let a233 = 3u64;
        let a234 = v1;
        nat_mul(a233, a234)
    }
}

pub fn f24(v0: bool) -> u64 {
    if v0 {
        6u64
    } else {
        5u64
    }
}

pub fn f25(v0: bool) -> u64 {
    if v0 {
        2u64
    } else {
        1u64
    }
}

pub fn f26(v0: u64) -> u64 {
    let a235 = v0;
    let a236 = 4u64;
    let a237 = 0u64;
    nat_quot(a235, a236, a237)
}

pub fn run(p0: u64, p1: u64, p2: u64, p3: u64, p4: u64, p5: u64, p6: u64, p7: u64, p8: u64, p9: u64, p10: u64, p11: u64, p12: u64, p13: u64, p14: u64, p15: u64, p16: u64, p17: u64, p18: u64, p19: u64, p20: u64, p21: u64, p22: u64) -> R<Adt0> {
    f0(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15, p16, p17, p18, p19, p20, p21, p22)
}
