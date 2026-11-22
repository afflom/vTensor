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
    C1(u64, u64, u64, u64, Option<u64>),
    C2(u64, u64, u64, u64),
    C3(u64, u64, u64, u64, u64, u64),
    C4(u64, u64, u64, u64, u64, u64, u64, u64),
}

pub fn f0(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt0> {
    let a1 = v0;
    let a2 = 6u64;
    if nat_eq(a1, a2) {
        f1(v1, v2, v3)
    } else {
        let a3 = v0;
        let a4 = 10u64;
        if nat_eq(a3, a4) {
            f2(v1)
        } else {
            let a5 = v0;
            let a6 = 9u64;
            if nat_eq(a5, a6) {
                f3(v1)
            } else {
                let a7 = v0;
                let a8 = 47u64;
                if nat_eq(a7, a8) {
                    Ok(f4(v1))
                } else {
                    let a9 = v0;
                    let a10 = 44u64;
                    if nat_eq(a9, a10) {
                        Ok(f5(v1))
                    } else {
                        let a11 = v0;
                        let a12 = 43u64;
                        if nat_eq(a11, a12) {
                            Ok(f6(v1))
                        } else {
                            let a13 = v0;
                            let a14 = 42u64;
                            if nat_eq(a13, a14) {
                                Ok(f7(v1))
                            } else {
                                let a15 = v0;
                                let a16 = 41u64;
                                if nat_eq(a15, a16) {
                                    Ok(f8(v1))
                                } else {
                                    let a17 = v0;
                                    let a18 = 36u64;
                                    if nat_eq(a17, a18) {
                                        Ok(f9(v1))
                                    } else {
                                        Ok(Adt0::C0)
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

pub fn f1(v0: u64, v1: u64, v2: u64) -> R<Adt0> {
    let a19 = v0;
    let a20 = 48u64;
    if nat_lt(a19, a20) {
        let v3: u64 = {
            let a21 = v0;
            let a22 = 16u64;
            if nat_lt(a21, a22) {
                1u64
            } else {
                let a23 = v0;
                let a24 = 32u64;
                if nat_lt(a23, a24) {
                    2u64
                } else {
                    4u64
                }
            }
        };
        let v4: u64 = {
            let a31 = {
                let a28 = {
                    let a25 = v0;
                    let a26 = 8u64;
                    let a27 = 0u64;
                    nat_quot(a25, a26, a27)
                };
                let a29 = 2u64;
                let a30 = 0u64;
                nat_rem(a28, a29, a30)
            };
            let a32 = 0u64;
            if nat_eq(a31, a32) {
                1u64
            } else {
                3u64
            }
        };
        let v5: u64 = {
            let a39 = {
                let a36 = {
                    let a33 = v0;
                    let a34 = 4u64;
                    let a35 = 0u64;
                    nat_quot(a33, a34, a35)
                };
                let a37 = 2u64;
                let a38 = 0u64;
                nat_rem(a36, a37, a38)
            };
            let a40 = 0u64;
            if nat_eq(a39, a40) {
                1u64
            } else {
                2u64
            }
        };
        let v6: u64 = {
            let a47 = {
                let a44 = {
                    let a41 = v0;
                    let a42 = 2u64;
                    let a43 = 0u64;
                    nat_quot(a41, a42, a43)
                };
                let a45 = 2u64;
                let a46 = 0u64;
                nat_rem(a44, a45, a46)
            };
            let a48 = 0u64;
            if nat_eq(a47, a48) {
                1u64
            } else {
                2u64
            }
        };
        let v7: u64 = {
            let a52 = {
                let a49 = v0;
                let a50 = 2u64;
                let a51 = 0u64;
                nat_rem(a49, a50, a51)
            };
            let a53 = 0u64;
            if nat_eq(a52, a53) {
                0u64
            } else {
                7u64
            }
        };
        let v8: u64 = {
            let a54 = v2;
            let a55 = v7;
            nat_add(a54, a55)?
        };
        let v9: u64 = {
            let a58 = {
                let a56 = v3;
                let a57 = v8;
                nat_mul(a56, a57)?
            };
            let a59 = 11u64;
            nat_add(a58, a59)?
        };
        let v10: u64 = {
            let a62 = {
                let a60 = v4;
                let a61 = v9;
                nat_mul(a60, a61)?
            };
            let a63 = 13u64;
            nat_add(a62, a63)?
        };
        Ok(Adt0::C4({
            let a64 = v1;
            let a65 = v3;
            nat_mul(a64, a65)?
        }, v4, v5, v6, v8, v9, v10, {
            let a68 = {
                let a66 = v5;
                let a67 = v10;
                nat_mul(a66, a67)?
            };
            let a69 = 17u64;
            nat_add(a68, a69)?
        }))
    } else {
        let a70 = v0;
        let a71 = 52u64;
        if nat_lt(a70, a71) {
            Ok(Adt0::C4({
                let a72 = v0;
                let a73 = 48u64;
                if nat_eq(a72, a73) {
                    0u64
                } else {
                    v1
                }
            }, {
                let a74 = v0;
                let a75 = 49u64;
                if nat_eq(a74, a75) {
                    0u64
                } else {
                    2u64
                }
            }, {
                let a76 = v0;
                let a77 = 50u64;
                if nat_eq(a76, a77) {
                    0u64
                } else {
                    2u64
                }
            }, {
                let a78 = v0;
                let a79 = 51u64;
                if nat_eq(a78, a79) {
                    0u64
                } else {
                    2u64
                }
            }, v2, {
                let a80 = v2;
                let a81 = 2u64;
                nat_mul(a80, a81)?
            }, {
                let a82 = v2;
                let a83 = 4u64;
                nat_mul(a82, a83)?
            }, {
                let a84 = v2;
                let a85 = 8u64;
                nat_mul(a84, a85)?
            }))
        } else {
            Ok(Adt0::C0)
        }
    }
}

pub fn f2(v0: u64) -> R<Adt0> {
    let a86 = v0;
    let a87 = 864u64;
    if nat_lt(a86, a87) {
        let v1: u64 = {
            let a91 = {
                let a88 = v0;
                let a89 = 108u64;
                let a90 = 0u64;
                nat_quot(a88, a89, a90)
            };
            let a92 = 0u64;
            if nat_eq(a91, a92) {
                1u64
            } else {
                let a96 = {
                    let a93 = v0;
                    let a94 = 108u64;
                    let a95 = 0u64;
                    nat_quot(a93, a94, a95)
                };
                let a97 = 1u64;
                if nat_eq(a96, a97) {
                    2u64
                } else {
                    let a101 = {
                        let a98 = v0;
                        let a99 = 108u64;
                        let a100 = 0u64;
                        nat_quot(a98, a99, a100)
                    };
                    let a102 = 2u64;
                    if nat_eq(a101, a102) {
                        4u64
                    } else {
                        let a106 = {
                            let a103 = v0;
                            let a104 = 108u64;
                            let a105 = 0u64;
                            nat_quot(a103, a104, a105)
                        };
                        let a107 = 3u64;
                        if nat_eq(a106, a107) {
                            8u64
                        } else {
                            let a111 = {
                                let a108 = v0;
                                let a109 = 108u64;
                                let a110 = 0u64;
                                nat_quot(a108, a109, a110)
                            };
                            let a112 = 4u64;
                            if nat_eq(a111, a112) {
                                16u64
                            } else {
                                let a116 = {
                                    let a113 = v0;
                                    let a114 = 108u64;
                                    let a115 = 0u64;
                                    nat_quot(a113, a114, a115)
                                };
                                let a117 = 5u64;
                                if nat_eq(a116, a117) {
                                    64u64
                                } else {
                                    let a121 = {
                                        let a118 = v0;
                                        let a119 = 108u64;
                                        let a120 = 0u64;
                                        nat_quot(a118, a119, a120)
                                    };
                                    let a122 = 6u64;
                                    if nat_eq(a121, a122) {
                                        256u64
                                    } else {
                                        let a126 = {
                                            let a123 = v0;
                                            let a124 = 108u64;
                                            let a125 = 0u64;
                                            nat_quot(a123, a124, a125)
                                        };
                                        let a127 = 7u64;
                                        if nat_eq(a126, a127) {
                                            1024u64
                                        } else {
                                            1u64
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };
        let v2: u64 = {
            let a134 = {
                let a131 = {
                    let a128 = v0;
                    let a129 = 36u64;
                    let a130 = 0u64;
                    nat_quot(a128, a129, a130)
                };
                let a132 = 3u64;
                let a133 = 0u64;
                nat_rem(a131, a132, a133)
            };
            let a135 = 0u64;
            if nat_eq(a134, a135) {
                0u64
            } else {
                let a142 = {
                    let a139 = {
                        let a136 = v0;
                        let a137 = 36u64;
                        let a138 = 0u64;
                        nat_quot(a136, a137, a138)
                    };
                    let a140 = 3u64;
                    let a141 = 0u64;
                    nat_rem(a139, a140, a141)
                };
                let a143 = 1u64;
                if nat_eq(a142, a143) {
                    v1
                } else {
                    let a150 = {
                        let a147 = {
                            let a144 = v0;
                            let a145 = 36u64;
                            let a146 = 0u64;
                            nat_quot(a144, a145, a146)
                        };
                        let a148 = 3u64;
                        let a149 = 0u64;
                        nat_rem(a147, a148, a149)
                    };
                    let a151 = 2u64;
                    if nat_eq(a150, a151) {
                        let a152 = v1;
                        let a153 = 3u64;
                        nat_mul(a152, a153)?
                    } else {
                        0u64
                    }
                }
            }
        };
        let v3: u64 = {
            let a160 = {
                let a157 = {
                    let a154 = v0;
                    let a155 = 6u64;
                    let a156 = 0u64;
                    nat_quot(a154, a155, a156)
                };
                let a158 = 6u64;
                let a159 = 0u64;
                nat_rem(a157, a158, a159)
            };
            let a161 = 0u64;
            if nat_eq(a160, a161) {
                0u64
            } else {
                let a168 = {
                    let a165 = {
                        let a162 = v0;
                        let a163 = 6u64;
                        let a164 = 0u64;
                        nat_quot(a162, a163, a164)
                    };
                    let a166 = 6u64;
                    let a167 = 0u64;
                    nat_rem(a165, a166, a167)
                };
                let a169 = 1u64;
                if nat_eq(a168, a169) {
                    1u64
                } else {
                    let a176 = {
                        let a173 = {
                            let a170 = v0;
                            let a171 = 6u64;
                            let a172 = 0u64;
                            nat_quot(a170, a171, a172)
                        };
                        let a174 = 6u64;
                        let a175 = 0u64;
                        nat_rem(a173, a174, a175)
                    };
                    let a177 = 2u64;
                    if nat_eq(a176, a177) {
                        let a178 = v1;
                        let a179 = 1u64;
                        nat_sub(a178, a179)
                    } else {
                        let a186 = {
                            let a183 = {
                                let a180 = v0;
                                let a181 = 6u64;
                                let a182 = 0u64;
                                nat_quot(a180, a181, a182)
                            };
                            let a184 = 6u64;
                            let a185 = 0u64;
                            nat_rem(a183, a184, a185)
                        };
                        let a187 = 3u64;
                        if nat_eq(a186, a187) {
                            v1
                        } else {
                            let a194 = {
                                let a191 = {
                                    let a188 = v0;
                                    let a189 = 6u64;
                                    let a190 = 0u64;
                                    nat_quot(a188, a189, a190)
                                };
                                let a192 = 6u64;
                                let a193 = 0u64;
                                nat_rem(a191, a192, a193)
                            };
                            let a195 = 4u64;
                            if nat_eq(a194, a195) {
                                let a196 = v1;
                                let a197 = 1u64;
                                nat_add(a196, a197)?
                            } else {
                                let a204 = {
                                    let a201 = {
                                        let a198 = v0;
                                        let a199 = 6u64;
                                        let a200 = 0u64;
                                        nat_quot(a198, a199, a200)
                                    };
                                    let a202 = 6u64;
                                    let a203 = 0u64;
                                    nat_rem(a201, a202, a203)
                                };
                                let a205 = 5u64;
                                if nat_eq(a204, a205) {
                                    let a206 = v1;
                                    let a207 = 4u64;
                                    nat_mul(a206, a207)?
                                } else {
                                    0u64
                                }
                            }
                        }
                    }
                }
            }
        };
        let v4: u64 = {
            let a211 = {
                let a208 = v0;
                let a209 = 6u64;
                let a210 = 0u64;
                nat_rem(a208, a209, a210)
            };
            let a212 = 0u64;
            if nat_eq(a211, a212) {
                0u64
            } else {
                let a216 = {
                    let a213 = v0;
                    let a214 = 6u64;
                    let a215 = 0u64;
                    nat_rem(a213, a214, a215)
                };
                let a217 = 1u64;
                if nat_eq(a216, a217) {
                    1u64
                } else {
                    let a221 = {
                        let a218 = v0;
                        let a219 = 6u64;
                        let a220 = 0u64;
                        nat_rem(a218, a219, a220)
                    };
                    let a222 = 2u64;
                    if nat_eq(a221, a222) {
                        let a223 = v1;
                        let a224 = 1u64;
                        nat_sub(a223, a224)
                    } else {
                        let a228 = {
                            let a225 = v0;
                            let a226 = 6u64;
                            let a227 = 0u64;
                            nat_rem(a225, a226, a227)
                        };
                        let a229 = 3u64;
                        if nat_eq(a228, a229) {
                            v1
                        } else {
                            let a233 = {
                                let a230 = v0;
                                let a231 = 6u64;
                                let a232 = 0u64;
                                nat_rem(a230, a231, a232)
                            };
                            let a234 = 4u64;
                            if nat_eq(a233, a234) {
                                let a235 = v1;
                                let a236 = 1u64;
                                nat_add(a235, a236)?
                            } else {
                                let a240 = {
                                    let a237 = v0;
                                    let a238 = 6u64;
                                    let a239 = 0u64;
                                    nat_rem(a237, a238, a239)
                                };
                                let a241 = 5u64;
                                if nat_eq(a240, a241) {
                                    let a244 = {
                                        let a242 = v1;
                                        let a243 = 2u64;
                                        nat_mul(a242, a243)?
                                    };
                                    let a245 = 1u64;
                                    nat_add(a244, a245)?
                                } else {
                                    0u64
                                }
                            }
                        }
                    }
                }
            }
        };
        Ok(Adt0::C2(v3, v2, v4, v1))
    } else {
        Ok(Adt0::C0)
    }
}

pub fn f3(v0: u64) -> R<Adt0> {
    let a246 = v0;
    let a247 = 1024u64;
    if nat_lt(a246, a247) {
        let v1: u64 = {
            let a254 = {
                let a251 = {
                    let a248 = v0;
                    let a249 = 16u64;
                    let a250 = 0u64;
                    nat_quot(a248, a249, a250)
                };
                let a252 = 4u64;
                let a253 = 0u64;
                nat_rem(a251, a252, a253)
            };
            let a255 = 0u64;
            if nat_eq(a254, a255) {
                0u64
            } else {
                let a262 = {
                    let a259 = {
                        let a256 = v0;
                        let a257 = 16u64;
                        let a258 = 0u64;
                        nat_quot(a256, a257, a258)
                    };
                    let a260 = 4u64;
                    let a261 = 0u64;
                    nat_rem(a259, a260, a261)
                };
                let a263 = 1u64;
                if nat_eq(a262, a263) {
                    1u64
                } else {
                    let a270 = {
                        let a267 = {
                            let a264 = v0;
                            let a265 = 16u64;
                            let a266 = 0u64;
                            nat_quot(a264, a265, a266)
                        };
                        let a268 = 4u64;
                        let a269 = 0u64;
                        nat_rem(a267, a268, a269)
                    };
                    let a271 = 2u64;
                    if nat_eq(a270, a271) {
                        8u64
                    } else {
                        let a278 = {
                            let a275 = {
                                let a272 = v0;
                                let a273 = 16u64;
                                let a274 = 0u64;
                                nat_quot(a272, a273, a274)
                            };
                            let a276 = 4u64;
                            let a277 = 0u64;
                            nat_rem(a275, a276, a277)
                        };
                        let a279 = 3u64;
                        if nat_eq(a278, a279) {
                            64u64
                        } else {
                            0u64
                        }
                    }
                }
            }
        };
        let v2: u64 = {
            let a286 = {
                let a283 = {
                    let a280 = v0;
                    let a281 = 4u64;
                    let a282 = 0u64;
                    nat_quot(a280, a281, a282)
                };
                let a284 = 4u64;
                let a285 = 0u64;
                nat_rem(a283, a284, a285)
            };
            let a287 = 0u64;
            if nat_eq(a286, a287) {
                0u64
            } else {
                let a294 = {
                    let a291 = {
                        let a288 = v0;
                        let a289 = 4u64;
                        let a290 = 0u64;
                        nat_quot(a288, a289, a290)
                    };
                    let a292 = 4u64;
                    let a293 = 0u64;
                    nat_rem(a291, a292, a293)
                };
                let a295 = 1u64;
                if nat_eq(a294, a295) {
                    1u64
                } else {
                    let a302 = {
                        let a299 = {
                            let a296 = v0;
                            let a297 = 4u64;
                            let a298 = 0u64;
                            nat_quot(a296, a297, a298)
                        };
                        let a300 = 4u64;
                        let a301 = 0u64;
                        nat_rem(a299, a300, a301)
                    };
                    let a303 = 2u64;
                    if nat_eq(a302, a303) {
                        v1
                    } else {
                        let a310 = {
                            let a307 = {
                                let a304 = v0;
                                let a305 = 4u64;
                                let a306 = 0u64;
                                nat_quot(a304, a305, a306)
                            };
                            let a308 = 4u64;
                            let a309 = 0u64;
                            nat_rem(a307, a308, a309)
                        };
                        let a311 = 3u64;
                        if nat_eq(a310, a311) {
                            let a312 = v1;
                            let a313 = 1u64;
                            nat_add(a312, a313)?
                        } else {
                            0u64
                        }
                    }
                }
            }
        };
        let v3: u64 = {
            let a317 = {
                let a314 = v0;
                let a315 = 4u64;
                let a316 = 0u64;
                nat_rem(a314, a315, a316)
            };
            let a318 = 0u64;
            if nat_eq(a317, a318) {
                0u64
            } else {
                let a322 = {
                    let a319 = v0;
                    let a320 = 4u64;
                    let a321 = 0u64;
                    nat_rem(a319, a320, a321)
                };
                let a323 = 1u64;
                if nat_eq(a322, a323) {
                    1u64
                } else {
                    let a327 = {
                        let a324 = v0;
                        let a325 = 4u64;
                        let a326 = 0u64;
                        nat_rem(a324, a325, a326)
                    };
                    let a328 = 2u64;
                    if nat_eq(a327, a328) {
                        v1
                    } else {
                        let a332 = {
                            let a329 = v0;
                            let a330 = 4u64;
                            let a331 = 0u64;
                            nat_rem(a329, a330, a331)
                        };
                        let a333 = 3u64;
                        if nat_eq(a332, a333) {
                            let a334 = v1;
                            let a335 = 1u64;
                            nat_add(a334, a335)?
                        } else {
                            0u64
                        }
                    }
                }
            }
        };
        Ok(Adt0::C3({
            let a336 = v0;
            let a337 = 256u64;
            let a338 = 0u64;
            nat_quot(a336, a337, a338)
        }, {
            let a342 = {
                let a339 = v0;
                let a340 = 128u64;
                let a341 = 0u64;
                nat_quot(a339, a340, a341)
            };
            let a343 = 2u64;
            let a344 = 0u64;
            nat_rem(a342, a343, a344)
        }, {
            let a348 = {
                let a345 = v0;
                let a346 = 64u64;
                let a347 = 0u64;
                nat_quot(a345, a346, a347)
            };
            let a349 = 2u64;
            let a350 = 0u64;
            nat_rem(a348, a349, a350)
        }, v1, v2, v3))
    } else {
        Ok(Adt0::C0)
    }
}

pub fn f4(v0: u64) -> Adt0 {
    let a351 = v0;
    let a352 = 0u64;
    if nat_eq(a351, a352) {
        Adt0::C1(3u64, 21u64, 3u64, 0u64, Some(4294967294u64))
    } else {
        let a353 = v0;
        let a354 = 1u64;
        if nat_eq(a353, a354) {
            Adt0::C1(0u64, 22u64, 3u64, 0u64, None::<u64>)
        } else {
            let a355 = v0;
            let a356 = 2u64;
            if nat_eq(a355, a356) {
                Adt0::C1(0u64, 0u64, 2147483648u64, 0u64, None::<u64>)
            } else {
                let a357 = v0;
                let a358 = 3u64;
                if nat_eq(a357, a358) {
                    Adt0::C1(4u64, 0u64, 0u64, 0u64, None::<u64>)
                } else {
                    Adt0::C0
                }
            }
        }
    }
}

pub fn f5(v0: u64) -> Adt0 {
    let a359 = v0;
    let a360 = 0u64;
    if nat_eq(a359, a360) {
        Adt0::C1(24u64, 0u64, 0u64, 0u64, Some(15360u64))
    } else {
        let a361 = v0;
        let a362 = 1u64;
        if nat_eq(a361, a362) {
            Adt0::C1(31u64, 0u64, 0u64, 0u64, Some(1920u64))
        } else {
            let a363 = v0;
            let a364 = 2u64;
            if nat_eq(a363, a364) {
                Adt0::C1(25u64, 15360u64, 0u64, 0u64, None::<u64>)
            } else {
                let a365 = v0;
                let a366 = 3u64;
                if nat_eq(a365, a366) {
                    Adt0::C1(32u64, 1920u64, 0u64, 0u64, None::<u64>)
                } else {
                    let a367 = v0;
                    let a368 = 4u64;
                    if nat_eq(a367, a368) {
                        Adt0::C1(26u64, 0u64, 2u64, 0u64, None::<u64>)
                    } else {
                        let a369 = v0;
                        let a370 = 5u64;
                        if nat_eq(a369, a370) {
                            Adt0::C1(33u64, 0u64, 2u64, 0u64, None::<u64>)
                        } else {
                            let a371 = v0;
                            let a372 = 6u64;
                            if nat_eq(a371, a372) {
                                Adt0::C1(27u64, 0u64, 1u64, 0u64, None::<u64>)
                            } else {
                                let a373 = v0;
                                let a374 = 7u64;
                                if nat_eq(a373, a374) {
                                    Adt0::C1(34u64, 0u64, 0u64, 192u64, None::<u64>)
                                } else {
                                    let a375 = v0;
                                    let a376 = 8u64;
                                    if nat_eq(a375, a376) {
                                        Adt0::C1(30u64, 0u64, 0u64, 0u64, None::<u64>)
                                    } else {
                                        let a377 = v0;
                                        let a378 = 9u64;
                                        if nat_eq(a377, a378) {
                                            Adt0::C1(37u64, 0u64, 0u64, 0u64, None::<u64>)
                                        } else {
                                            Adt0::C0
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

pub fn f6(v0: u64) -> Adt0 {
    let a379 = v0;
    let a380 = 0u64;
    if nat_eq(a379, a380) {
        Adt0::C1(17u64, 0u64, 0u64, 0u64, Some(10368u64))
    } else {
        let a381 = v0;
        let a382 = 1u64;
        if nat_eq(a381, a382) {
            Adt0::C1(18u64, 10368u64, 0u64, 0u64, None::<u64>)
        } else {
            let a383 = v0;
            let a384 = 2u64;
            if nat_eq(a383, a384) {
                Adt0::C1(19u64, 0u64, 4u64, 0u64, None::<u64>)
            } else {
                let a385 = v0;
                let a386 = 3u64;
                if nat_eq(a385, a386) {
                    Adt0::C1(20u64, 0u64, 2u64, 0u64, None::<u64>)
                } else {
                    let a387 = v0;
                    let a388 = 4u64;
                    if nat_eq(a387, a388) {
                        Adt0::C1(23u64, 0u64, 0u64, 0u64, None::<u64>)
                    } else {
                        Adt0::C0
                    }
                }
            }
        }
    }
}

pub fn f7(v0: u64) -> Adt0 {
    let a389 = v0;
    let a390 = 0u64;
    if nat_eq(a389, a390) {
        Adt0::C1(11u64, 0u64, 0u64, 0u64, Some(160u64))
    } else {
        let a391 = v0;
        let a392 = 1u64;
        if nat_eq(a391, a392) {
            Adt0::C1(12u64, 160u64, 0u64, 0u64, None::<u64>)
        } else {
            let a393 = v0;
            let a394 = 2u64;
            if nat_eq(a393, a394) {
                Adt0::C1(13u64, 0u64, 4u64, 0u64, None::<u64>)
            } else {
                let a395 = v0;
                let a396 = 3u64;
                if nat_eq(a395, a396) {
                    Adt0::C1(14u64, 0u64, 0u64, 256u64, None::<u64>)
                } else {
                    let a397 = v0;
                    let a398 = 4u64;
                    if nat_eq(a397, a398) {
                        Adt0::C1(16u64, 0u64, 0u64, 0u64, None::<u64>)
                    } else {
                        Adt0::C0
                    }
                }
            }
        }
    }
}

pub fn f8(v0: u64) -> Adt0 {
    let a399 = v0;
    let a400 = 0u64;
    if nat_eq(a399, a400) {
        Adt0::C1(5u64, 0u64, 0u64, 0u64, Some(3456u64))
    } else {
        let a401 = v0;
        let a402 = 1u64;
        if nat_eq(a401, a402) {
            Adt0::C1(5u64, 0u64, 0u64, 5u64, None::<u64>)
        } else {
            let a403 = v0;
            let a404 = 2u64;
            if nat_eq(a403, a404) {
                Adt0::C1(5u64, 1u64, 0u64, 0u64, None::<u64>)
            } else {
                let a405 = v0;
                let a406 = 3u64;
                if nat_eq(a405, a406) {
                    Adt0::C1(6u64, 3456u64, 0u64, 0u64, None::<u64>)
                } else {
                    let a407 = v0;
                    let a408 = 4u64;
                    if nat_eq(a407, a408) {
                        Adt0::C1(6u64, 0u64, 0u64, 9u64, None::<u64>)
                    } else {
                        let a409 = v0;
                        let a410 = 5u64;
                        if nat_eq(a409, a410) {
                            Adt0::C1(7u64, 0u64, 2u64, 0u64, None::<u64>)
                        } else {
                            let a411 = v0;
                            let a412 = 6u64;
                            if nat_eq(a411, a412) {
                                Adt0::C1(7u64, 0u64, 0u64, 10u64, None::<u64>)
                            } else {
                                let a413 = v0;
                                let a414 = 7u64;
                                if nat_eq(a413, a414) {
                                    Adt0::C1(8u64, 0u64, 0u64, 256u64, None::<u64>)
                                } else {
                                    let a415 = v0;
                                    let a416 = 8u64;
                                    if nat_eq(a415, a416) {
                                        Adt0::C1(9u64, 215u64, 8u64, 0u64, None::<u64>)
                                    } else {
                                        let a417 = v0;
                                        let a418 = 9u64;
                                        if nat_eq(a417, a418) {
                                            Adt0::C1(10u64, 0u64, 0u64, 0u64, None::<u64>)
                                        } else {
                                            let a419 = v0;
                                            let a420 = 10u64;
                                            if nat_eq(a419, a420) {
                                                Adt0::C1(18446744073709551615u64, 18446744073709551615u64, 18446744073709551615u64, 18446744073709551615u64, None::<u64>)
                                            } else {
                                                Adt0::C0
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

pub fn f9(v0: u64) -> Adt0 {
    let a421 = v0;
    let a422 = 0u64;
    if nat_eq(a421, a422) {
        Adt0::C1(1u64, 120u64, 0u64, 0u64, None::<u64>)
    } else {
        let a423 = v0;
        let a424 = 1u64;
        if nat_eq(a423, a424) {
            Adt0::C1(1u64, 0u64, 0u64, 24u64, None::<u64>)
        } else {
            let a425 = v0;
            let a426 = 2u64;
            if nat_eq(a425, a426) {
                Adt0::C1(2u64, 0u64, 2u64, 0u64, None::<u64>)
            } else {
                let a427 = v0;
                let a428 = 3u64;
                if nat_eq(a427, a428) {
                    Adt0::C1(3u64, 96u64, 1u64, 0u64, None::<u64>)
                } else {
                    let a429 = v0;
                    let a430 = 4u64;
                    if nat_eq(a429, a430) {
                        Adt0::C1(4u64, 0u64, 0u64, 0u64, None::<u64>)
                    } else {
                        Adt0::C0
                    }
                }
            }
        }
    }
}
