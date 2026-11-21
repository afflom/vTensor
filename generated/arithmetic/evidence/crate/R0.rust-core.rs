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

pub fn f0(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a1 = v0;
    let a2 = 2u64;
    if nat_lt(a1, a2) {
        let a3 = v0;
        let a4 = 0u64;
        if nat_eq(a3, a4) {
            let a5 = v3;
            let a6 = 4u64;
            if nat_lt(a5, a6) {
                Ok(Some({
                    let a7 = v3;
                    let a8 = 2u64;
                    if nat_lt(a7, a8) {
                        let a9 = v3;
                        let a10 = 1u64;
                        if nat_lt(a9, a10) {
                            120u64
                        } else {
                            24u64
                        }
                    } else {
                        let a11 = v3;
                        let a12 = 3u64;
                        if nat_lt(a11, a12) {
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
            let a13 = v1;
            let a14 = 120u64;
            if if nat_lt(a13, a14) {
                let a15 = v3;
                let a16 = 24u64;
                nat_lt(a15, a16)
            } else {
                false
            } {
                Ok(Some(f1(v1, v3)?))
            } else {
                Ok(None::<u64>)
            }
        }
    } else {
        let a17 = v0;
        let a18 = 4u64;
        if if nat_lt(a17, a18) {
            let a19 = v1;
            let a20 = 120u64;
            if nat_lt(a19, a20) {
                let a21 = v2;
                let a22 = 2u64;
                nat_lt(a21, a22)
            } else {
                false
            }
        } else {
            false
        } {
            let a23 = v0;
            let a24 = 2u64;
            if nat_eq(a23, a24) {
                let a25 = v3;
                let a26 = 2u64;
                if nat_lt(a25, a26) {
                    Ok(Some({
                        let a27 = v3;
                        let a28 = 0u64;
                        if nat_eq(a27, a28) {
                            f2(v1, v2)?
                        } else {
                            4u64
                        }
                    }))
                } else {
                    Ok(None::<u64>)
                }
            } else {
                let a29 = v3;
                let a30 = f2(v1, v2)?;
                if nat_lt(a29, a30) {
                    Ok(Some(f3(v2, v3)?))
                } else {
                    Ok(None::<u64>)
                }
            }
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f1(v0: u64, v1: u64) -> R<u64> {
    let a31 = v1;
    let a32 = 12u64;
    if nat_lt(a31, a32) {
        let a33 = v1;
        let a34 = 4u64;
        if nat_lt(a33, a34) {
            let a35 = v1;
            let a36 = 2u64;
            if nat_lt(a35, a36) {
                let a37 = v1;
                let a38 = 1u64;
                if nat_lt(a37, a38) {
                    f4(v0)
                } else {
                    let a39 = f4(v0)?;
                    let a40 = 4u64;
                    if nat_lt(a39, a40) {
                        let a41 = f4(v0)?;
                        let a42 = 2u64;
                        if nat_lt(a41, a42) {
                            let a43 = f4(v0)?;
                            let a44 = 1u64;
                            if nat_lt(a43, a44) {
                                Ok(2u64)
                            } else {
                                Ok(5u64)
                            }
                        } else {
                            let a45 = f4(v0)?;
                            let a46 = 3u64;
                            if nat_lt(a45, a46) {
                                Ok(6u64)
                            } else {
                                Ok(7u64)
                            }
                        }
                    } else {
                        let a47 = f4(v0)?;
                        let a48 = 6u64;
                        if nat_lt(a47, a48) {
                            let a49 = f4(v0)?;
                            let a50 = 5u64;
                            if nat_lt(a49, a50) {
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
                let a51 = v1;
                let a52 = 3u64;
                if nat_lt(a51, a52) {
                    let a53 = f4(v0)?;
                    let a54 = 6u64;
                    if nat_eq(a53, a54) {
                        Ok(2u64)
                    } else {
                        Ok(0u64)
                    }
                } else {
                    Ok(f5(v0))
                }
            }
        } else {
            Ok(f6(v0, {
                let a57 = {
                    let a55 = v1;
                    let a56 = 4u64;
                    nat_sub(a55, a56)
                };
                let a58 = 4u64;
                let a59 = 0u64;
                nat_quot(a57, a58, a59)
            }, {
                let a62 = {
                    let a60 = v1;
                    let a61 = 4u64;
                    nat_sub(a60, a61)
                };
                let a63 = 4u64;
                let a64 = 0u64;
                nat_rem(a62, a63, a64)
            }))
        }
    } else {
        let a65 = v1;
        let a66 = 20u64;
        if nat_lt(a65, a66) {
            f7(v0, {
                let a69 = {
                    let a67 = v1;
                    let a68 = 12u64;
                    nat_sub(a67, a68)
                };
                let a70 = 4u64;
                let a71 = 0u64;
                nat_quot(a69, a70, a71)
            }, {
                let a74 = {
                    let a72 = v1;
                    let a73 = 12u64;
                    nat_sub(a72, a73)
                };
                let a75 = 4u64;
                let a76 = 0u64;
                nat_rem(a74, a75, a76)
            })
        } else {
            let a77 = v1;
            let a78 = 22u64;
            if nat_lt(a77, a78) {
                f2(v0, {
                    let a79 = v1;
                    let a80 = 20u64;
                    nat_sub(a79, a80)
                })
            } else {
                let a81 = v1;
                let a82 = 22u64;
                if nat_eq(a81, a82) {
                    let a83 = v0;
                    let a84 = 0u64;
                    if nat_eq(a83, a84) {
                        Ok(1u64)
                    } else {
                        let a85 = v0;
                        let a86 = 108u64;
                        if nat_eq(a85, a86) {
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

pub fn f2(v0: u64, v1: u64) -> R<u64> {
    let a87 = v1;
    let a88 = 0u64;
    if nat_eq(a87, a88) {
        let a89 = f8(v0);
        let a90 = 12u64;
        nat_mul(a89, a90)
    } else {
        let a91 = 96u64;
        let a92 = v0;
        if nat_le(a91, a92) {
            Ok(0u64)
        } else {
            let a93 = f9(v0);
            let a94 = 2u64;
            if nat_lt(a93, a94) {
                let a95 = f9(v0);
                let a96 = 1u64;
                if nat_lt(a95, a96) {
                    let a97 = f8(v0);
                    let a98 = 12u64;
                    nat_mul(a97, a98)
                } else {
                    Ok(1u64)
                }
            } else {
                let a99 = f9(v0);
                let a100 = 3u64;
                if nat_lt(a99, a100) {
                    Ok(f8(v0))
                } else {
                    Ok(6u64)
                }
            }
        }
    }
}

pub fn f3(v0: u64, v1: u64) -> R<u64> {
    let a101 = v1;
    let a102 = 16u64;
    if nat_lt(a101, a102) {
        Ok(f10(f11(v0, v1)?))
    } else {
        Ok(f12({
            let a103 = f13(v0);
            let a104 = f14(v1)?;
            nat_add(a103, a104)?
        }))
    }
}

pub fn f4(v0: u64) -> R<u64> {
    let a105 = v0;
    let a106 = 96u64;
    if nat_lt(a105, a106) {
        let a107 = v0;
        let a108 = 24u64;
        let a109 = 0u64;
        Ok(nat_quot(a107, a108, a109))
    } else {
        let a115 = 4u64;
        let a116 = {
            let a112 = {
                let a110 = v0;
                let a111 = 96u64;
                nat_sub(a110, a111)
            };
            let a113 = 6u64;
            let a114 = 0u64;
            nat_quot(a112, a113, a114)
        };
        nat_add(a115, a116)
    }
}

pub fn f5(v0: u64) -> u64 {
    let a117 = v0;
    let a118 = 96u64;
    if nat_lt(a117, a118) {
        let a122 = {
            let a119 = v0;
            let a120 = 4u64;
            let a121 = 0u64;
            nat_quot(a119, a120, a121)
        };
        let a123 = 2u64;
        let a124 = 0u64;
        nat_rem(a122, a123, a124)
    } else {
        let a125 = v0;
        let a126 = 2u64;
        let a127 = 0u64;
        nat_rem(a125, a126, a127)
    }
}

pub fn f6(v0: u64, v1: u64, v2: u64) -> u64 {
    let a128 = v1;
    let a129 = 0u64;
    if nat_eq(a128, a129) {
        let a130 = v2;
        let a131 = 2u64;
        if nat_lt(a130, a131) {
            let a132 = v2;
            let a133 = 1u64;
            if nat_lt(a132, a133) {
                f8(v0)
            } else {
                3u64
            }
        } else {
            2u64
        }
    } else {
        let a134 = 96u64;
        let a135 = v0;
        if nat_le(a134, a135) {
            0u64
        } else {
            let a136 = v2;
            let a137 = 2u64;
            if nat_lt(a136, a137) {
                let a138 = v2;
                let a139 = 1u64;
                if nat_lt(a138, a139) {
                    let a140 = f9(v0);
                    let a141 = 1u64;
                    if if nat_eq(a140, a141) {
                        true
                    } else {
                        let a142 = f9(v0);
                        let a143 = 3u64;
                        nat_eq(a142, a143)
                    } {
                        1u64
                    } else {
                        f8(v0)
                    }
                } else {
                    let a144 = f9(v0);
                    let a145 = 1u64;
                    if if nat_eq(a144, a145) {
                        true
                    } else {
                        let a146 = f9(v0);
                        let a147 = 2u64;
                        nat_eq(a146, a147)
                    } {
                        1u64
                    } else {
                        3u64
                    }
                }
            } else {
                let a148 = v2;
                let a149 = 3u64;
                if nat_lt(a148, a149) {
                    let a150 = f9(v0);
                    let a151 = 0u64;
                    if nat_eq(a150, a151) {
                        2u64
                    } else {
                        1u64
                    }
                } else {
                    let a152 = f9(v0);
                    let a153 = 1u64;
                    if if nat_eq(a152, a153) {
                        true
                    } else {
                        let a154 = f9(v0);
                        let a155 = 2u64;
                        nat_eq(a154, a155)
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

pub fn f7(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a156 = v1;
    let a157 = 1u64;
    if if nat_eq(a156, a157) {
        let a158 = 96u64;
        let a159 = v0;
        nat_le(a158, a159)
    } else {
        false
    } {
        Ok(0u64)
    } else {
        let v3: u64 = f8(v0);
        let v4: u64 = f5(v0);
        let v5: u64 = {
            let a160 = v1;
            let a161 = 0u64;
            if nat_eq(a160, a161) {
                0u64
            } else {
                f9(v0)
            }
        };
        let a162 = v5;
        let a163 = 2u64;
        if nat_lt(a162, a163) {
            let a164 = v5;
            let a165 = 1u64;
            if nat_lt(a164, a165) {
                let a166 = v2;
                let a167 = 2u64;
                if nat_lt(a166, a167) {
                    let a168 = v2;
                    let a169 = 1u64;
                    if nat_lt(a168, a169) {
                        Ok(4u64)
                    } else {
                        let a172 = {
                            let a170 = v3;
                            let a171 = v4;
                            nat_add(a170, a171)?
                        };
                        let a173 = 4u64;
                        nat_mul(a172, a173)
                    }
                } else {
                    let a174 = v2;
                    let a175 = 3u64;
                    if nat_lt(a174, a175) {
                        let a182 = {
                            let a180 = {
                                let a176 = v3;
                                let a177 = 3u64;
                                nat_mul(a176, a177)?
                            };
                            let a181 = {
                                let a178 = v4;
                                let a179 = 4u64;
                                nat_mul(a178, a179)?
                            };
                            nat_add(a180, a181)?
                        };
                        let a183 = 4u64;
                        nat_mul(a182, a183)
                    } else {
                        let a190 = {
                            let a188 = {
                                let a184 = v3;
                                let a185 = 6u64;
                                nat_mul(a184, a185)?
                            };
                            let a189 = {
                                let a186 = v4;
                                let a187 = 9u64;
                                nat_mul(a186, a187)?
                            };
                            nat_add(a188, a189)?
                        };
                        let a191 = 4u64;
                        nat_mul(a190, a191)
                    }
                }
            } else {
                let a192 = v2;
                let a193 = 2u64;
                if nat_lt(a192, a193) {
                    let a194 = v2;
                    let a195 = 1u64;
                    if nat_lt(a194, a195) {
                        Ok(4u64)
                    } else {
                        let a198 = {
                            let a196 = 1u64;
                            let a197 = v4;
                            nat_add(a196, a197)?
                        };
                        let a199 = 4u64;
                        nat_mul(a198, a199)
                    }
                } else {
                    let a200 = v2;
                    let a201 = 3u64;
                    if nat_lt(a200, a201) {
                        let a206 = {
                            let a204 = 1u64;
                            let a205 = {
                                let a202 = v4;
                                let a203 = 2u64;
                                nat_mul(a202, a203)?
                            };
                            nat_add(a204, a205)?
                        };
                        let a207 = 4u64;
                        nat_mul(a206, a207)
                    } else {
                        let a212 = {
                            let a210 = 1u64;
                            let a211 = {
                                let a208 = v4;
                                let a209 = 3u64;
                                nat_mul(a208, a209)?
                            };
                            nat_add(a210, a211)?
                        };
                        let a213 = 4u64;
                        nat_mul(a212, a213)
                    }
                }
            }
        } else {
            let a214 = v5;
            let a215 = 3u64;
            if nat_lt(a214, a215) {
                let a216 = v2;
                let a217 = 2u64;
                if nat_lt(a216, a217) {
                    let a218 = v2;
                    let a219 = 1u64;
                    if nat_lt(a218, a219) {
                        Ok(4u64)
                    } else {
                        let a222 = {
                            let a220 = v3;
                            let a221 = v4;
                            nat_add(a220, a221)?
                        };
                        let a223 = 4u64;
                        nat_mul(a222, a223)
                    }
                } else {
                    let a224 = v2;
                    let a225 = 3u64;
                    if nat_lt(a224, a225) {
                        let a230 = {
                            let a228 = v3;
                            let a229 = {
                                let a226 = v4;
                                let a227 = 2u64;
                                nat_mul(a226, a227)?
                            };
                            nat_add(a228, a229)?
                        };
                        let a231 = 4u64;
                        nat_mul(a230, a231)
                    } else {
                        let a236 = {
                            let a234 = v3;
                            let a235 = {
                                let a232 = v4;
                                let a233 = 3u64;
                                nat_mul(a232, a233)?
                            };
                            nat_add(a234, a235)?
                        };
                        let a237 = 4u64;
                        nat_mul(a236, a237)
                    }
                }
            } else {
                let a238 = v2;
                let a239 = 2u64;
                if nat_lt(a238, a239) {
                    let a240 = v2;
                    let a241 = 1u64;
                    if nat_lt(a240, a241) {
                        Ok(4u64)
                    } else {
                        let a244 = {
                            let a242 = 1u64;
                            let a243 = v4;
                            nat_add(a242, a243)?
                        };
                        let a245 = 4u64;
                        nat_mul(a244, a245)
                    }
                } else {
                    let a246 = v2;
                    let a247 = 3u64;
                    if nat_lt(a246, a247) {
                        let a252 = {
                            let a250 = 3u64;
                            let a251 = {
                                let a248 = v4;
                                let a249 = 4u64;
                                nat_mul(a248, a249)?
                            };
                            nat_add(a250, a251)?
                        };
                        let a253 = 4u64;
                        nat_mul(a252, a253)
                    } else {
                        let a258 = {
                            let a256 = 3u64;
                            let a257 = {
                                let a254 = v4;
                                let a255 = 5u64;
                                nat_mul(a254, a255)?
                            };
                            nat_add(a256, a257)?
                        };
                        let a259 = 4u64;
                        nat_mul(a258, a259)
                    }
                }
            }
        }
    }
}

pub fn f8(v0: u64) -> u64 {
    let a260 = f15(v0);
    let a261 = 0u64;
    if nat_eq(a260, a261) {
        1u64
    } else {
        let a262 = f15(v0);
        let a263 = 1u64;
        if nat_eq(a262, a263) {
            17u64
        } else {
            257u64
        }
    }
}

pub fn f9(v0: u64) -> u64 {
    let a264 = v0;
    let a265 = 4u64;
    let a266 = 0u64;
    nat_rem(a264, a265, a266)
}

pub fn f10(v0: u64) -> u64 {
    let a267 = v0;
    let a268 = 8u64;
    if nat_lt(a267, a268) {
        let a269 = v0;
        let a270 = 4u64;
        if nat_lt(a269, a270) {
            let a271 = v0;
            let a272 = 2u64;
            if nat_lt(a271, a272) {
                let a273 = v0;
                let a274 = 1u64;
                if nat_lt(a273, a274) {
                    0u64
                } else {
                    2147483648u64
                }
            } else {
                let a275 = v0;
                let a276 = 3u64;
                if nat_lt(a275, a276) {
                    1065353216u64
                } else {
                    3212836864u64
                }
            }
        } else {
            let a277 = v0;
            let a278 = 6u64;
            if nat_lt(a277, a278) {
                let a279 = v0;
                let a280 = 5u64;
                if nat_lt(a279, a280) {
                    2139095040u64
                } else {
                    4286578688u64
                }
            } else {
                let a281 = v0;
                let a282 = 7u64;
                if nat_lt(a281, a282) {
                    2143289345u64
                } else {
                    1u64
                }
            }
        }
    } else {
        let a283 = v0;
        let a284 = 12u64;
        if nat_lt(a283, a284) {
            let a285 = v0;
            let a286 = 10u64;
            if nat_lt(a285, a286) {
                let a287 = v0;
                let a288 = 9u64;
                if nat_lt(a287, a288) {
                    2139095039u64
                } else {
                    8388608u64
                }
            } else {
                let a289 = v0;
                let a290 = 11u64;
                if nat_lt(a289, a290) {
                    8388607u64
                } else {
                    1065353217u64
                }
            }
        } else {
            let a291 = v0;
            let a292 = 14u64;
            if nat_lt(a291, a292) {
                let a293 = v0;
                let a294 = 13u64;
                if nat_lt(a293, a294) {
                    1056964608u64
                } else {
                    8388609u64
                }
            } else {
                let a295 = v0;
                let a296 = 15u64;
                if nat_lt(a295, a296) {
                    2147483649u64
                } else {
                    2139095041u64
                }
            }
        }
    }
}

pub fn f11(v0: u64, v1: u64) -> R<u64> {
    let a305 = {
        let a303 = {
            let a299 = {
                let a297 = v0;
                let a298 = 0u64;
                if nat_eq(a297, a298) {
                    1u64
                } else {
                    5u64
                }
            };
            let a300 = v1;
            nat_mul(a299, a300)?
        };
        let a304 = {
            let a301 = v0;
            let a302 = 0u64;
            if nat_eq(a301, a302) {
                0u64
            } else {
                3u64
            }
        };
        nat_add(a303, a304)?
    };
    let a306 = 16u64;
    let a307 = 0u64;
    Ok(nat_rem(a305, a306, a307))
}

pub fn f12(v0: u64) -> u64 {
    let a308 = v0;
    let a309 = 4294967296u64;
    let a310 = 0u64;
    nat_rem(a308, a309, a310)
}

pub fn f13(v0: u64) -> u64 {
    let a311 = v0;
    let a312 = 0u64;
    if nat_eq(a311, a312) {
        2654435769u64
    } else {
        2246822507u64
    }
}

pub fn f14(v0: u64) -> R<u64> {
    let a313 = 2654435761u64;
    let a314 = v0;
    nat_mul(a313, a314)
}

pub fn f15(v0: u64) -> u64 {
    let a315 = v0;
    let a316 = 96u64;
    if nat_lt(a315, a316) {
        let a320 = {
            let a317 = v0;
            let a318 = 8u64;
            let a319 = 0u64;
            nat_quot(a317, a318, a319)
        };
        let a321 = 3u64;
        let a322 = 0u64;
        nat_rem(a320, a321, a322)
    } else {
        let a328 = {
            let a325 = {
                let a323 = v0;
                let a324 = 96u64;
                nat_sub(a323, a324)
            };
            let a326 = 2u64;
            let a327 = 0u64;
            nat_quot(a325, a326, a327)
        };
        let a329 = 3u64;
        let a330 = 0u64;
        nat_rem(a328, a329, a330)
    }
}
