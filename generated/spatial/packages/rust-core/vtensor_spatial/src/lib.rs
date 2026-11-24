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
    let a1 = 130u64;
    let a2 = v0;
    if if nat_le(a1, a2) {
        let a3 = v0;
        let a4 = 141u64;
        nat_lt(a3, a4)
    } else {
        false
    } {
        Ok(Some(f1({
            let a5 = v0;
            let a6 = 130u64;
            nat_sub(a5, a6)
        }, v1, v2, v3)?))
    } else {
        let a7 = 150u64;
        let a8 = v0;
        if if nat_le(a7, a8) {
            let a9 = v0;
            let a10 = 161u64;
            nat_lt(a9, a10)
        } else {
            false
        } {
            Ok(Some(f2({
                let a11 = v0;
                let a12 = 150u64;
                nat_sub(a11, a12)
            }, v1, v2, v3)))
        } else {
            let a13 = 120u64;
            let a14 = v0;
            if if nat_le(a13, a14) {
                let a15 = v0;
                let a16 = 124u64;
                nat_le(a15, a16)
            } else {
                false
            } {
                Ok(Some(f3(v0, v1, v2, v3)?))
            } else {
                let a17 = 100u64;
                let a18 = v0;
                if if nat_le(a17, a18) {
                    let a19 = v0;
                    let a20 = 111u64;
                    if nat_lt(a19, a20) {
                        let a21 = v3;
                        let a22 = 8u64;
                        nat_lt(a21, a22)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Some(f4({
                        let a23 = v0;
                        let a24 = 100u64;
                        nat_sub(a23, a24)
                    }, v1, v2, v3)?))
                } else {
                    let a25 = v2;
                    let a26 = 2147483647u64;
                    if if nat_le(a25, a26) {
                        let a27 = v0;
                        let a28 = 4u64;
                        nat_lt(a27, a28)
                    } else {
                        false
                    } {
                        let a29 = v0;
                        let a30 = 0u64;
                        if if nat_eq(a29, a30) {
                            true
                        } else {
                            let a31 = v0;
                            let a32 = 3u64;
                            nat_eq(a31, a32)
                        } {
                            let a33 = v1;
                            let a34 = 22u64;
                            if if nat_lt(a33, a34) {
                                let a35 = v3;
                                let a36 = 2u64;
                                nat_lt(a35, a36)
                            } else {
                                false
                            } {
                                Ok(Some(f5({
                                    let a37 = v0;
                                    let a38 = 3u64;
                                    nat_eq(a37, a38)
                                }, v1, v2, v3)?))
                            } else {
                                Ok(None::<u64>)
                            }
                        } else {
                            let a39 = v0;
                            let a40 = 1u64;
                            if nat_eq(a39, a40) {
                                let a41 = v1;
                                let a42 = 4u64;
                                if if nat_lt(a41, a42) {
                                    let a43 = v3;
                                    let a44 = 2u64;
                                    if nat_lt(a43, a44) {
                                        let a45 = 0u64;
                                        let a46 = v2;
                                        nat_lt(a45, a46)
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                } {
                                    Ok(Some(f6(v1, v2, v3)))
                                } else {
                                    Ok(None::<u64>)
                                }
                            } else {
                                let a47 = v1;
                                let a48 = 17u64;
                                if if nat_lt(a47, a48) {
                                    let a49 = v3;
                                    let a50 = 4u64;
                                    if nat_lt(a49, a50) {
                                        let a51 = 0u64;
                                        let a52 = v2;
                                        nat_lt(a51, a52)
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                } {
                                    Ok(Some(f7(v1, v2, v3)))
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
    }
}

pub fn f1(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let v4: u64 = {
        let a53 = v0;
        let a54 = 0u64;
        if nat_eq(a53, a54) {
            let a55 = v1;
            let a56 = 0u64;
            if nat_eq(a55, a56) {
                22u64
            } else {
                let a57 = v1;
                let a58 = 1u64;
                if nat_eq(a57, a58) {
                    4u64
                } else {
                    17u64
                }
            }
        } else {
            let a59 = v0;
            let a60 = 1u64;
            if nat_eq(a59, a60) {
                21u64
            } else {
                let a61 = v0;
                let a62 = 2u64;
                if nat_eq(a61, a62) {
                    4u64
                } else {
                    let a63 = v0;
                    let a64 = 3u64;
                    if nat_eq(a63, a64) {
                        5u64
                    } else {
                        let a65 = v0;
                        let a66 = 4u64;
                        if nat_eq(a65, a66) {
                            let a67 = v1;
                            let a68 = 0u64;
                            if nat_eq(a67, a68) {
                                12u64
                            } else {
                                30u64
                            }
                        } else {
                            let a69 = v0;
                            let a70 = 5u64;
                            if nat_eq(a69, a70) {
                                28u64
                            } else {
                                let a71 = v0;
                                let a72 = 6u64;
                                if nat_eq(a71, a72) {
                                    5u64
                                } else {
                                    let a73 = v0;
                                    let a74 = 7u64;
                                    if nat_eq(a73, a74) {
                                        12u64
                                    } else {
                                        let a75 = v0;
                                        let a76 = 8u64;
                                        if nat_eq(a75, a76) {
                                            14u64
                                        } else {
                                            let a77 = v0;
                                            let a78 = 9u64;
                                            if nat_eq(a77, a78) {
                                                18u64
                                            } else {
                                                2u64
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
    };
    let v5: u64 = {
        let a79 = v0;
        let a80 = 8u64;
        if if nat_eq(a79, a80) {
            true
        } else {
            let a81 = v0;
            let a82 = 9u64;
            nat_eq(a81, a82)
        } {
            5u64
        } else {
            let a83 = v0;
            let a84 = 7u64;
            if nat_eq(a83, a84) {
                4u64
            } else {
                let a85 = v0;
                let a86 = 4u64;
                if if if nat_eq(a85, a86) {
                    true
                } else {
                    let a87 = v0;
                    let a88 = 10u64;
                    nat_eq(a87, a88)
                } {
                    true
                } else {
                    let a89 = v0;
                    let a90 = 0u64;
                    if nat_eq(a89, a90) {
                        let a91 = v1;
                        let a92 = 0u64;
                        nat_eq(a91, a92)
                    } else {
                        false
                    }
                } {
                    3u64
                } else {
                    2u64
                }
            }
        }
    };
    let v6: u64 = {
        let a93 = v0;
        let a94 = 7u64;
        if if if nat_eq(a93, a94) {
            true
        } else {
            let a95 = v0;
            let a96 = 8u64;
            nat_eq(a95, a96)
        } {
            true
        } else {
            let a97 = v0;
            let a98 = 9u64;
            nat_eq(a97, a98)
        } {
            2u64
        } else {
            v5
        }
    };
    let v7: u64 = {
        let a99 = v2;
        let a100 = 2u64;
        let a101 = 0u64;
        nat_quot(a99, a100, a101)
    };
    let v8: u64 = {
        let a102 = v7;
        let a103 = v6;
        let a104 = 0u64;
        nat_rem(a102, a103, a104)
    };
    let v9: u64 = {
        let a108 = {
            let a105 = v7;
            let a106 = v6;
            let a107 = 0u64;
            nat_quot(a105, a106, a107)
        };
        let a109 = v5;
        let a110 = 0u64;
        nat_rem(a108, a109, a110)
    };
    let v10: u64 = {
        let a114 = {
            let a111 = v7;
            let a112 = v6;
            let a113 = 0u64;
            nat_quot(a111, a112, a113)
        };
        let a115 = v5;
        let a116 = 0u64;
        nat_quot(a114, a115, a116)
    };
    let a117 = v3;
    let a118 = 0u64;
    if nat_eq(a117, a118) {
        let a119 = v0;
        let a120 = 0u64;
        if nat_eq(a119, a120) {
            Ok(3u64)
        } else {
            let a121 = v0;
            let a122 = 1u64;
            if nat_eq(a121, a122) {
                Ok(1u64)
            } else {
                let a123 = v0;
                let a124 = 2u64;
                if nat_eq(a123, a124) {
                    Ok(1u64)
                } else {
                    let a125 = v0;
                    let a126 = 3u64;
                    if nat_eq(a125, a126) {
                        Ok(4u64)
                    } else {
                        let a127 = v0;
                        let a128 = 4u64;
                        if nat_eq(a127, a128) {
                            Ok(2u64)
                        } else {
                            let a129 = v0;
                            let a130 = 5u64;
                            if nat_eq(a129, a130) {
                                Ok(8u64)
                            } else {
                                let a131 = v0;
                                let a132 = 6u64;
                                if nat_eq(a131, a132) {
                                    Ok(39u64)
                                } else {
                                    let a133 = v0;
                                    let a134 = 7u64;
                                    if nat_eq(a133, a134) {
                                        Ok(1u64)
                                    } else {
                                        let a135 = v0;
                                        let a136 = 8u64;
                                        if nat_eq(a135, a136) {
                                            Ok(3u64)
                                        } else {
                                            let a137 = v0;
                                            let a138 = 9u64;
                                            if nat_eq(a137, a138) {
                                                Ok(2u64)
                                            } else {
                                                Ok(1u64)
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
    } else {
        let a139 = v3;
        let a140 = 1u64;
        if nat_eq(a139, a140) {
            let a145 = 8u64;
            let a146 = {
                let a143 = v4;
                let a144 = {
                    let a141 = v5;
                    let a142 = v6;
                    nat_mul(a141, a142)?
                };
                nat_mul(a143, a144)?
            };
            nat_mul(a145, a146)
        } else {
            let a147 = v3;
            let a148 = 2u64;
            if nat_eq(a147, a148) {
                let a149 = v10;
                let a150 = 4u64;
                let a151 = 0u64;
                Ok(nat_quot(a149, a150, a151))
            } else {
                let a152 = v3;
                let a153 = 3u64;
                if nat_eq(a152, a153) {
                    let a154 = v10;
                    let a155 = 4u64;
                    let a156 = 0u64;
                    Ok(nat_rem(a154, a155, a156))
                } else {
                    let a157 = v3;
                    let a158 = 4u64;
                    if nat_eq(a157, a158) {
                        let a159 = v0;
                        let a160 = 8u64;
                        if if nat_eq(a159, a160) {
                            true
                        } else {
                            let a161 = v0;
                            let a162 = 9u64;
                            nat_eq(a161, a162)
                        } {
                            let a163 = v9;
                            let a164 = 0u64;
                            if nat_eq(a163, a164) {
                                Ok(1u64)
                            } else {
                                let a165 = v9;
                                let a166 = 1u64;
                                if nat_eq(a165, a166) {
                                    Ok(2u64)
                                } else {
                                    let a167 = v9;
                                    let a168 = 2u64;
                                    if nat_eq(a167, a168) {
                                        Ok(3u64)
                                    } else {
                                        let a169 = v9;
                                        let a170 = 3u64;
                                        if nat_eq(a169, a170) {
                                            Ok(17u64)
                                        } else {
                                            Ok(65u64)
                                        }
                                    }
                                }
                            }
                        } else {
                            let a171 = v0;
                            let a172 = 7u64;
                            if nat_eq(a171, a172) {
                                let a173 = v9;
                                let a174 = 0u64;
                                if nat_eq(a173, a174) {
                                    Ok(1u64)
                                } else {
                                    let a175 = v9;
                                    let a176 = 1u64;
                                    if nat_eq(a175, a176) {
                                        Ok(3u64)
                                    } else {
                                        let a177 = v9;
                                        let a178 = 2u64;
                                        if nat_eq(a177, a178) {
                                            Ok(17u64)
                                        } else {
                                            Ok(65u64)
                                        }
                                    }
                                }
                            } else {
                                let a179 = v0;
                                let a180 = 4u64;
                                if nat_eq(a179, a180) {
                                    let a181 = v9;
                                    let a182 = 0u64;
                                    if nat_eq(a181, a182) {
                                        Ok(1u64)
                                    } else {
                                        let a183 = v9;
                                        let a184 = 1u64;
                                        if nat_eq(a183, a184) {
                                            Ok(3u64)
                                        } else {
                                            Ok(5u64)
                                        }
                                    }
                                } else {
                                    let a185 = v0;
                                    let a186 = 10u64;
                                    if nat_eq(a185, a186) {
                                        let a187 = v9;
                                        let a188 = 0u64;
                                        if nat_eq(a187, a188) {
                                            Ok(1u64)
                                        } else {
                                            let a189 = v9;
                                            let a190 = 1u64;
                                            if nat_eq(a189, a190) {
                                                Ok(3u64)
                                            } else {
                                                Ok(17u64)
                                            }
                                        }
                                    } else {
                                        let a191 = v0;
                                        let a192 = 0u64;
                                        if nat_eq(a191, a192) {
                                            let a193 = v1;
                                            let a194 = 0u64;
                                            if nat_eq(a193, a194) {
                                                let a195 = v9;
                                                let a196 = 0u64;
                                                if nat_eq(a195, a196) {
                                                    Ok(0u64)
                                                } else {
                                                    let a197 = v9;
                                                    let a198 = 1u64;
                                                    if nat_eq(a197, a198) {
                                                        Ok(1u64)
                                                    } else {
                                                        Ok(3u64)
                                                    }
                                                }
                                            } else {
                                                let a199 = v9;
                                                let a200 = 0u64;
                                                if nat_eq(a199, a200) {
                                                    Ok(2u64)
                                                } else {
                                                    Ok(3u64)
                                                }
                                            }
                                        } else {
                                            let a201 = v9;
                                            let a202 = 0u64;
                                            if nat_eq(a201, a202) {
                                                Ok(1u64)
                                            } else {
                                                Ok(3u64)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        let a203 = v3;
                        let a204 = 5u64;
                        if nat_eq(a203, a204) {
                            let a205 = v0;
                            let a206 = 10u64;
                            if nat_eq(a205, a206) {
                                let a207 = v8;
                                let a208 = 0u64;
                                if nat_eq(a207, a208) {
                                    Ok(1u64)
                                } else {
                                    let a209 = v8;
                                    let a210 = 1u64;
                                    if nat_eq(a209, a210) {
                                        Ok(3u64)
                                    } else {
                                        Ok(7u64)
                                    }
                                }
                            } else {
                                let a211 = v0;
                                let a212 = 7u64;
                                if if if nat_eq(a211, a212) {
                                    true
                                } else {
                                    let a213 = v0;
                                    let a214 = 8u64;
                                    nat_eq(a213, a214)
                                } {
                                    true
                                } else {
                                    let a215 = v0;
                                    let a216 = 9u64;
                                    nat_eq(a215, a216)
                                } {
                                    let a217 = v8;
                                    let a218 = 0u64;
                                    if nat_eq(a217, a218) {
                                        Ok(1u64)
                                    } else {
                                        Ok(3u64)
                                    }
                                } else {
                                    let a219 = v0;
                                    let a220 = 8u64;
                                    if if nat_eq(a219, a220) {
                                        true
                                    } else {
                                        let a221 = v0;
                                        let a222 = 9u64;
                                        nat_eq(a221, a222)
                                    } {
                                        let a223 = v8;
                                        let a224 = 0u64;
                                        if nat_eq(a223, a224) {
                                            Ok(1u64)
                                        } else {
                                            let a225 = v8;
                                            let a226 = 1u64;
                                            if nat_eq(a225, a226) {
                                                Ok(2u64)
                                            } else {
                                                let a227 = v8;
                                                let a228 = 2u64;
                                                if nat_eq(a227, a228) {
                                                    Ok(3u64)
                                                } else {
                                                    let a229 = v8;
                                                    let a230 = 3u64;
                                                    if nat_eq(a229, a230) {
                                                        Ok(17u64)
                                                    } else {
                                                        Ok(65u64)
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        let a231 = v0;
                                        let a232 = 7u64;
                                        if nat_eq(a231, a232) {
                                            let a233 = v8;
                                            let a234 = 0u64;
                                            if nat_eq(a233, a234) {
                                                Ok(1u64)
                                            } else {
                                                let a235 = v8;
                                                let a236 = 1u64;
                                                if nat_eq(a235, a236) {
                                                    Ok(3u64)
                                                } else {
                                                    let a237 = v8;
                                                    let a238 = 2u64;
                                                    if nat_eq(a237, a238) {
                                                        Ok(17u64)
                                                    } else {
                                                        Ok(65u64)
                                                    }
                                                }
                                            }
                                        } else {
                                            let a239 = v0;
                                            let a240 = 4u64;
                                            if nat_eq(a239, a240) {
                                                let a241 = v8;
                                                let a242 = 0u64;
                                                if nat_eq(a241, a242) {
                                                    Ok(1u64)
                                                } else {
                                                    let a243 = v8;
                                                    let a244 = 1u64;
                                                    if nat_eq(a243, a244) {
                                                        Ok(3u64)
                                                    } else {
                                                        Ok(5u64)
                                                    }
                                                }
                                            } else {
                                                let a245 = v0;
                                                let a246 = 10u64;
                                                if nat_eq(a245, a246) {
                                                    let a247 = v8;
                                                    let a248 = 0u64;
                                                    if nat_eq(a247, a248) {
                                                        Ok(1u64)
                                                    } else {
                                                        let a249 = v8;
                                                        let a250 = 1u64;
                                                        if nat_eq(a249, a250) {
                                                            Ok(3u64)
                                                        } else {
                                                            Ok(17u64)
                                                        }
                                                    }
                                                } else {
                                                    let a251 = v0;
                                                    let a252 = 0u64;
                                                    if nat_eq(a251, a252) {
                                                        let a253 = v1;
                                                        let a254 = 0u64;
                                                        if nat_eq(a253, a254) {
                                                            let a255 = v8;
                                                            let a256 = 0u64;
                                                            if nat_eq(a255, a256) {
                                                                Ok(0u64)
                                                            } else {
                                                                let a257 = v8;
                                                                let a258 = 1u64;
                                                                if nat_eq(a257, a258) {
                                                                    Ok(1u64)
                                                                } else {
                                                                    Ok(3u64)
                                                                }
                                                            }
                                                        } else {
                                                            let a259 = v8;
                                                            let a260 = 0u64;
                                                            if nat_eq(a259, a260) {
                                                                Ok(2u64)
                                                            } else {
                                                                Ok(3u64)
                                                            }
                                                        }
                                                    } else {
                                                        let a261 = v8;
                                                        let a262 = 0u64;
                                                        if nat_eq(a261, a262) {
                                                            Ok(1u64)
                                                        } else {
                                                            Ok(3u64)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            let a270 = {
                                let a263 = v2;
                                let a264 = 2u64;
                                let a265 = 0u64;
                                nat_rem(a263, a264, a265)
                            };
                            let a271 = {
                                let a266 = v0;
                                let a267 = 0u64;
                                if if nat_eq(a266, a267) {
                                    let a268 = v1;
                                    let a269 = 2u64;
                                    nat_eq(a268, a269)
                                } else {
                                    false
                                } {
                                    2u64
                                } else {
                                    1u64
                                }
                            };
                            nat_add(a270, a271)
                        }
                    }
                }
            }
        }
    }
}

pub fn f2(v0: u64, v1: u64, v2: u64, v3: u64) -> u64 {
    let v4: u64 = {
        let a272 = v1;
        let a273 = 32u64;
        let a274 = 0u64;
        nat_quot(a272, a273, a274)
    };
    let v5: u64 = {
        let a275 = v1;
        let a276 = 32u64;
        let a277 = 0u64;
        nat_rem(a275, a276, a277)
    };
    let v6: u64 = {
        let a278 = v2;
        let a279 = 128u64;
        let a280 = 0u64;
        nat_rem(a278, a279, a280)
    };
    let a281 = v0;
    let a282 = 10u64;
    if if if nat_eq(a281, a282) {
        let a283 = v3;
        let a284 = 1u64;
        if if nat_eq(a283, a284) {
            true
        } else {
            let a285 = v3;
            let a286 = 30u64;
            nat_eq(a285, a286)
        } {
            let a287 = v5;
            let a288 = 0u64;
            if nat_eq(a287, a288) {
                true
            } else {
                let a289 = v6;
                let a290 = 1u64;
                nat_eq(a289, a290)
            }
        } else {
            false
        }
    } else {
        let a291 = v0;
        let a292 = 9u64;
        if nat_eq(a291, a292) {
            let a293 = v4;
            let a294 = 0u64;
            if nat_eq(a293, a294) {
                let a295 = v3;
                let a296 = 0u64;
                if nat_eq(a295, a296) {
                    let a297 = v5;
                    let a298 = 16u64;
                    nat_lt(a297, a298)
                } else {
                    false
                }
            } else {
                let a299 = v3;
                let a300 = 26u64;
                nat_eq(a299, a300)
            }
        } else {
            let a301 = v0;
            let a302 = 0u64;
            if if if if nat_eq(a301, a302) {
                true
            } else {
                let a303 = v0;
                let a304 = 1u64;
                nat_eq(a303, a304)
            } {
                true
            } else {
                let a305 = v0;
                let a306 = 4u64;
                nat_eq(a305, a306)
            } {
                true
            } else {
                let a307 = v0;
                let a308 = 7u64;
                nat_eq(a307, a308)
            } {
                let a309 = v3;
                let a310 = 0u64;
                nat_eq(a309, a310)
            } else {
                let a311 = v0;
                let a312 = 8u64;
                if if nat_eq(a311, a312) {
                    let a313 = 0u64;
                    let a314 = v4;
                    nat_lt(a313, a314)
                } else {
                    false
                } {
                    let a315 = v3;
                    let a316 = 0u64;
                    nat_eq(a315, a316)
                } else {
                    let a317 = v0;
                    let a318 = 6u64;
                    if if nat_eq(a317, a318) {
                        let a322 = {
                            let a319 = v4;
                            let a320 = 3u64;
                            let a321 = 0u64;
                            nat_rem(a319, a320, a321)
                        };
                        let a323 = 2u64;
                        nat_eq(a322, a323)
                    } else {
                        false
                    } {
                        let a324 = v3;
                        let a325 = 1u64;
                        if nat_eq(a324, a325) {
                            true
                        } else {
                            let a326 = v3;
                            let a327 = 30u64;
                            nat_eq(a326, a327)
                        }
                    } else {
                        let a328 = v3;
                        let a329 = 0u64;
                        if if nat_eq(a328, a329) {
                            true
                        } else {
                            let a330 = v3;
                            let a331 = 1u64;
                            nat_eq(a330, a331)
                        } {
                            true
                        } else {
                            let a332 = v3;
                            let a333 = 30u64;
                            nat_eq(a332, a333)
                        }
                    }
                }
            }
        }
    } {
        let a334 = v0;
        let a335 = 5u64;
        if if nat_eq(a334, a335) {
            let a336 = 24u64;
            let a337 = v5;
            nat_le(a336, a337)
        } else {
            let a338 = v0;
            let a339 = 6u64;
            if nat_eq(a338, a339) {
                let a340 = 0u64;
                let a341 = v5;
                nat_lt(a340, a341)
            } else {
                let a342 = v0;
                let a343 = 7u64;
                if nat_eq(a342, a343) {
                    let a344 = 8u64;
                    let a345 = v5;
                    nat_le(a344, a345)
                } else {
                    let a346 = v0;
                    let a347 = 8u64;
                    if nat_eq(a346, a347) {
                        let a348 = 10u64;
                        let a349 = v5;
                        nat_le(a348, a349)
                    } else {
                        let a350 = v0;
                        let a351 = 9u64;
                        if nat_eq(a350, a351) {
                            let a352 = 14u64;
                            let a353 = v5;
                            nat_le(a352, a353)
                        } else {
                            false
                        }
                    }
                }
            }
        } {
            let a354 = v2;
            let a355 = 16513u64;
            nat_eq(a354, a355)
        } else {
            true
        }
    } else {
        false
    } {
        1u64
    } else {
        0u64
    }
}

pub fn f3(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let v4: u64 = {
        let a356 = v2;
        let a357 = 4u64;
        let a358 = 0u64;
        nat_rem(a356, a357, a358)
    };
    let v5: u64 = {
        let a359 = v0;
        let a360 = 120u64;
        if nat_eq(a359, a360) {
            f8({
                let a361 = v2;
                let a362 = 24u64;
                let a363 = 0u64;
                nat_rem(a361, a362, a363)
            })
        } else {
            let a364 = v0;
            let a365 = 121u64;
            if nat_eq(a364, a365) {
                f9({
                    let a366 = v2;
                    let a367 = 24u64;
                    let a368 = 0u64;
                    nat_rem(a366, a367, a368)
                })
            } else {
                let a369 = v0;
                let a370 = 122u64;
                if nat_eq(a369, a370) {
                    f10({
                        let a371 = v2;
                        let a372 = 32u64;
                        let a373 = 0u64;
                        nat_rem(a371, a372, a373)
                    })
                } else {
                    let a374 = v0;
                    let a375 = 124u64;
                    if if nat_eq(a374, a375) {
                        let a376 = 10u64;
                        let a377 = v1;
                        if nat_le(a376, a377) {
                            let a378 = v1;
                            let a379 = 14u64;
                            nat_lt(a378, a379)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        f11({
                            let a384 = {
                                let a382 = {
                                    let a380 = v1;
                                    let a381 = 10u64;
                                    nat_sub(a380, a381)
                                };
                                let a383 = 4u64;
                                nat_mul(a382, a383)?
                            };
                            let a385 = v4;
                            nat_add(a384, a385)?
                        })
                    } else {
                        let a386 = v1;
                        let a387 = 7u64;
                        if nat_lt(a386, a387) {
                            f12({
                                let a390 = {
                                    let a388 = v1;
                                    let a389 = 4u64;
                                    nat_mul(a388, a389)?
                                };
                                let a391 = v4;
                                nat_add(a390, a391)?
                            })
                        } else {
                            let a392 = v1;
                            let a393 = 8u64;
                            if nat_eq(a392, a393) {
                                f13(v4)
                            } else {
                                let a394 = v1;
                                let a395 = 9u64;
                                if nat_eq(a394, a395) {
                                    2139095039u64
                                } else {
                                    let a400 = {
                                        let a398 = 2654435769u64;
                                        let a399 = {
                                            let a396 = 2654435761u64;
                                            let a397 = v2;
                                            nat_mul(a396, a397)?
                                        };
                                        nat_add(a398, a399)?
                                    };
                                    let a401 = 4294967296u64;
                                    let a402 = 0u64;
                                    nat_rem(a400, a401, a402)
                                }
                            }
                        }
                    }
                }
            }
        }
    };
    let v6: u64 = {
        let a403 = v1;
        let a404 = 4u64;
        let a405 = 0u64;
        nat_rem(a403, a404, a405)
    };
    let a406 = v0;
    let a407 = 124u64;
    if if if nat_eq(a406, a407) {
        let a408 = v3;
        let a409 = 1u64;
        nat_eq(a408, a409)
    } else {
        false
    } {
        let a410 = v6;
        let a411 = 1u64;
        if if if nat_eq(a410, a411) {
            let a412 = v1;
            let a413 = 9u64;
            !nat_eq(a412, a413)
        } else {
            false
        } {
            true
        } else {
            let a414 = v6;
            let a415 = 2u64;
            if if nat_eq(a414, a415) {
                true
            } else {
                let a416 = v1;
                let a417 = 9u64;
                nat_eq(a416, a417)
            } {
                let a421 = {
                    let a418 = v2;
                    let a419 = 2u64;
                    let a420 = 0u64;
                    nat_rem(a418, a419, a420)
                };
                let a422 = 1u64;
                nat_eq(a421, a422)
            } else {
                false
            }
        } {
            true
        } else {
            let a423 = v6;
            let a424 = 3u64;
            if nat_eq(a423, a424) {
                let a428 = {
                    let a425 = v2;
                    let a426 = 7u64;
                    let a427 = 0u64;
                    nat_rem(a425, a426, a427)
                };
                let a429 = 3u64;
                nat_eq(a428, a429)
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a432 = {
            let a430 = 2147483648u64;
            let a431 = v5;
            nat_add(a430, a431)?
        };
        let a433 = 4294967296u64;
        let a434 = 0u64;
        Ok(nat_rem(a432, a433, a434))
    } else {
        Ok(v5)
    }
}

pub fn f4(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let v4: u64 = {
        let a435 = v1;
        let a436 = 32u64;
        let a437 = 0u64;
        nat_quot(a435, a436, a437)
    };
    let v5: u64 = {
        let a438 = v1;
        let a439 = 32u64;
        let a440 = 0u64;
        nat_rem(a438, a439, a440)
    };
    let v6: u64 = {
        let a441 = v2;
        let a442 = 128u64;
        let a443 = 0u64;
        nat_rem(a441, a442, a443)
    };
    let v7: u64 = {
        let a447 = {
            let a444 = v2;
            let a445 = 128u64;
            let a446 = 0u64;
            nat_quot(a444, a445, a446)
        };
        let a448 = 128u64;
        let a449 = 0u64;
        nat_rem(a447, a448, a449)
    };
    let v8: u64 = {
        let a450 = v2;
        let a451 = 16384u64;
        let a452 = 0u64;
        nat_quot(a450, a451, a452)
    };
    let v9: u64 = {
        let a455 = {
            let a453 = v7;
            let a454 = v8;
            nat_add(a453, a454)?
        };
        let a456 = 1u64;
        nat_sub(a455, a456)
    };
    let v10: u64 = {
        let a457 = v3;
        let a458 = 0u64;
        if nat_eq(a457, a458) {
            v6
        } else {
            let a459 = v3;
            let a460 = 1u64;
            if nat_eq(a459, a460) {
                v7
            } else {
                let a461 = v3;
                let a462 = 2u64;
                if nat_eq(a461, a462) {
                    2u64
                } else {
                    v8
                }
            }
        }
    };
    let a463 = v0;
    let a464 = 1u64;
    if nat_eq(a463, a464) {
        let a465 = v5;
        let a466 = 16u64;
        if if if nat_eq(a465, a466) {
            let a467 = v3;
            let a468 = 0u64;
            nat_eq(a467, a468)
        } else {
            false
        } {
            true
        } else {
            let a471 = v5;
            let a472 = {
                let a469 = v3;
                let a470 = 17u64;
                nat_add(a469, a470)?
            };
            nat_eq(a471, a472)
        } {
            Ok(0u64)
        } else {
            let a487 = v10;
            let a488 = {
                let a485 = 1u64;
                let a486 = {
                    let a482 = {
                        let a479 = v5;
                        let a480 = {
                            let a473 = v3;
                            let a474 = 0u64;
                            if nat_eq(a473, a474) {
                                1u64
                            } else {
                                let a475 = v3;
                                let a476 = 1u64;
                                if nat_eq(a475, a476) {
                                    2u64
                                } else {
                                    let a477 = v3;
                                    let a478 = 2u64;
                                    if nat_eq(a477, a478) {
                                        4u64
                                    } else {
                                        8u64
                                    }
                                }
                            }
                        };
                        let a481 = 0u64;
                        nat_quot(a479, a480, a481)
                    };
                    let a483 = 2u64;
                    let a484 = 0u64;
                    nat_rem(a482, a483, a484)
                };
                nat_add(a485, a486)?
            };
            nat_mul(a487, a488)
        }
    } else {
        let a489 = v0;
        let a490 = 2u64;
        if nat_eq(a489, a490) {
            let a491 = v5;
            let a492 = 1u64;
            if if if nat_eq(a491, a492) {
                let a493 = v3;
                let a494 = 0u64;
                nat_eq(a493, a494)
            } else {
                false
            } {
                true
            } else {
                let a495 = 2u64;
                let a496 = v5;
                if nat_le(a495, a496) {
                    let a497 = v3;
                    let a498 = v5;
                    nat_eq(a497, a498)
                } else {
                    false
                }
            } {
                Ok(0u64)
            } else {
                let a499 = v3;
                let a500 = 0u64;
                if nat_eq(a499, a500) {
                    Ok(v6)
                } else {
                    let a501 = v3;
                    let a502 = 1u64;
                    if nat_eq(a501, a502) {
                        Ok(1u64)
                    } else {
                        let a503 = v3;
                        let a504 = 2u64;
                        if nat_eq(a503, a504) {
                            Ok(v7)
                        } else {
                            Ok(v8)
                        }
                    }
                }
            }
        } else {
            let a505 = v0;
            let a506 = 3u64;
            if nat_eq(a505, a506) {
                let a507 = v5;
                let a508 = 1u64;
                if if if nat_eq(a507, a508) {
                    let a509 = v3;
                    let a510 = 2u64;
                    nat_lt(a509, a510)
                } else {
                    false
                } {
                    true
                } else {
                    let a511 = 2u64;
                    let a512 = v5;
                    if nat_le(a511, a512) {
                        let a513 = v5;
                        let a514 = 4u64;
                        if nat_lt(a513, a514) {
                            let a515 = v5;
                            let a516 = v3;
                            nat_eq(a515, a516)
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } {
                    Ok(0u64)
                } else {
                    let a517 = v3;
                    let a518 = 0u64;
                    if nat_eq(a517, a518) {
                        Ok(v6)
                    } else {
                        let a519 = v3;
                        let a520 = 1u64;
                        if nat_eq(a519, a520) {
                            Ok(v6)
                        } else {
                            let a521 = v3;
                            let a522 = 2u64;
                            if nat_eq(a521, a522) {
                                Ok(v7)
                            } else {
                                Ok(v8)
                            }
                        }
                    }
                }
            } else {
                let a523 = v0;
                let a524 = 4u64;
                if nat_eq(a523, a524) {
                    f14(v4, v5, v6, v7, v8, v3)
                } else {
                    let a525 = v0;
                    let a526 = 9u64;
                    if nat_eq(a525, a526) {
                        let a529 = v5;
                        let a530 = {
                            let a527 = v3;
                            let a528 = 14u64;
                            nat_add(a527, a528)?
                        };
                        if nat_eq(a529, a530) {
                            Ok(0u64)
                        } else {
                            let a531 = v4;
                            let a532 = 0u64;
                            if if nat_eq(a531, a532) {
                                true
                            } else {
                                let a533 = 10u64;
                                let a534 = v5;
                                if nat_le(a533, a534) {
                                    let a535 = v5;
                                    let a536 = 14u64;
                                    nat_lt(a535, a536)
                                } else {
                                    false
                                }
                            } {
                                let a537 = v3;
                                let a538 = 0u64;
                                if nat_eq(a537, a538) {
                                    Ok(v6)
                                } else {
                                    let a539 = v3;
                                    let a540 = 1u64;
                                    if nat_eq(a539, a540) {
                                        let a541 = v7;
                                        let a542 = v8;
                                        nat_mul(a541, a542)
                                    } else {
                                        let a543 = v3;
                                        let a544 = 2u64;
                                        let _ = nat_eq(a543, a544);
                                        Ok(1u64)
                                    }
                                }
                            } else {
                                let a545 = v5;
                                let a546 = 9u64;
                                if nat_eq(a545, a546) {
                                    let a547 = v3;
                                    let a548 = 0u64;
                                    if nat_eq(a547, a548) {
                                        Ok(v6)
                                    } else {
                                        let a549 = v3;
                                        let a550 = 1u64;
                                        if nat_eq(a549, a550) {
                                            Ok(v9)
                                        } else {
                                            let a551 = v3;
                                            let a552 = 2u64;
                                            if nat_eq(a551, a552) {
                                                Ok(v9)
                                            } else {
                                                let a553 = v9;
                                                let a554 = v9;
                                                nat_mul(a553, a554)
                                            }
                                        }
                                    }
                                } else {
                                    Ok(v10)
                                }
                            }
                        }
                    } else {
                        let a555 = v0;
                        let a556 = 10u64;
                        if nat_eq(a555, a556) {
                            let a557 = v3;
                            let a558 = 0u64;
                            if nat_eq(a557, a558) {
                                let a559 = v5;
                                let a560 = 0u64;
                                if nat_eq(a559, a560) {
                                    Ok(v6)
                                } else {
                                    Ok(0u64)
                                }
                            } else {
                                let a561 = v3;
                                let a562 = 1u64;
                                if nat_eq(a561, a562) {
                                    let a565 = {
                                        let a563 = 2u64;
                                        let a564 = v7;
                                        nat_mul(a563, a564)?
                                    };
                                    let a566 = 1u64;
                                    Ok(nat_sub(a565, a566))
                                } else {
                                    let a567 = v3;
                                    let a568 = 2u64;
                                    if nat_eq(a567, a568) {
                                        Ok(2u64)
                                    } else {
                                        Ok(v8)
                                    }
                                }
                            }
                        } else {
                            let a569 = v0;
                            let a570 = 5u64;
                            if if nat_eq(a569, a570) {
                                let a573 = v5;
                                let a574 = {
                                    let a571 = v3;
                                    let a572 = 24u64;
                                    nat_add(a571, a572)?
                                };
                                nat_eq(a573, a574)
                            } else {
                                false
                            } {
                                Ok(0u64)
                            } else {
                                let a575 = v0;
                                let a576 = 6u64;
                                if if nat_eq(a575, a576) {
                                    let a579 = v5;
                                    let a580 = {
                                        let a577 = v3;
                                        let a578 = 1u64;
                                        nat_add(a577, a578)?
                                    };
                                    nat_eq(a579, a580)
                                } else {
                                    false
                                } {
                                    Ok(0u64)
                                } else {
                                    let a581 = v0;
                                    let a582 = 7u64;
                                    if if nat_eq(a581, a582) {
                                        let a585 = v5;
                                        let a586 = {
                                            let a583 = v3;
                                            let a584 = 8u64;
                                            nat_add(a583, a584)?
                                        };
                                        nat_eq(a585, a586)
                                    } else {
                                        false
                                    } {
                                        Ok(0u64)
                                    } else {
                                        let a587 = v0;
                                        let a588 = 8u64;
                                        if if nat_eq(a587, a588) {
                                            let a591 = v5;
                                            let a592 = {
                                                let a589 = v3;
                                                let a590 = 10u64;
                                                nat_add(a589, a590)?
                                            };
                                            nat_eq(a591, a592)
                                        } else {
                                            false
                                        } {
                                            Ok(0u64)
                                        } else {
                                            Ok(v10)
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

pub fn f5(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let v4: u64 = f15(v1, v2);
    let v5: u64 = f16(v1, v2);
    if if if v0 {
        let a596 = {
            let a593 = v1;
            let a594 = 2u64;
            let a595 = 0u64;
            nat_rem(a593, a594, a595)
        };
        let a597 = 1u64;
        nat_eq(a596, a597)
    } else {
        false
    } {
        let a598 = 0u64;
        let a599 = v2;
        nat_lt(a598, a599)
    } else {
        false
    } {
        let v6: u64 = f17(v4, v2);
        let a600 = v3;
        let a601 = 0u64;
        if nat_eq(a600, a601) {
            Ok(v6)
        } else {
            let a608 = {
                let a606 = {
                    let a604 = 4294967296u64;
                    let a605 = {
                        let a602 = v4;
                        let a603 = v5;
                        nat_add(a602, a603)?
                    };
                    nat_add(a604, a605)?
                };
                let a607 = v6;
                nat_sub(a606, a607)
            };
            let a609 = 4294967296u64;
            let a610 = 0u64;
            Ok(nat_rem(a608, a609, a610))
        }
    } else {
        let a611 = v3;
        let a612 = 0u64;
        if nat_eq(a611, a612) {
            Ok(v4)
        } else {
            Ok(v5)
        }
    }
}

pub fn f6(v0: u64, v1: u64, v2: u64) -> u64 {
    let a621 = {
        let a618 = {
            let a615 = v0;
            let a616 = {
                let a613 = v2;
                let a614 = 0u64;
                if nat_eq(a613, a614) {
                    1u64
                } else {
                    2u64
                }
            };
            let a617 = 0u64;
            nat_quot(a615, a616, a617)
        };
        let a619 = 2u64;
        let a620 = 0u64;
        nat_rem(a618, a619, a620)
    };
    let a622 = 1u64;
    if nat_eq(a621, a622) {
        let a623 = v1;
        let a624 = 1u64;
        nat_sub(a623, a624)
    } else {
        0u64
    }
}

pub fn f7(v0: u64, v1: u64, v2: u64) -> u64 {
    let a625 = v0;
    let a626 = 0u64;
    if nat_eq(a625, a626) {
        0u64
    } else {
        let v3: u64 = {
            let a627 = v1;
            let a628 = 1u64;
            nat_sub(a627, a628)
        };
        let a643 = {
            let a640 = {
                let a637 = {
                    let a629 = v0;
                    let a630 = 1u64;
                    nat_sub(a629, a630)
                };
                let a638 = {
                    let a631 = v2;
                    let a632 = 2u64;
                    if nat_lt(a631, a632) {
                        let a633 = v2;
                        let a634 = 0u64;
                        if nat_eq(a633, a634) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a635 = v2;
                        let a636 = 2u64;
                        if nat_eq(a635, a636) {
                            4u64
                        } else {
                            8u64
                        }
                    }
                };
                let a639 = 0u64;
                nat_quot(a637, a638, a639)
            };
            let a641 = 2u64;
            let a642 = 0u64;
            nat_rem(a640, a641, a642)
        };
        let a644 = 1u64;
        if nat_eq(a643, a644) {
            f18(v3)
        } else {
            v3
        }
    }
}

pub fn f8(v0: u64) -> u64 {
    let a645 = v0;
    let a646 = 12u64;
    if nat_lt(a645, a646) {
        let a647 = v0;
        let a648 = 6u64;
        if nat_lt(a647, a648) {
            let a649 = v0;
            let a650 = 3u64;
            if nat_lt(a649, a650) {
                let a651 = v0;
                let a652 = 1u64;
                if nat_lt(a651, a652) {
                    0u64
                } else {
                    let a653 = v0;
                    let a654 = 2u64;
                    if nat_lt(a653, a654) {
                        2147483648u64
                    } else {
                        1065353216u64
                    }
                }
            } else {
                let a655 = v0;
                let a656 = 4u64;
                if nat_lt(a655, a656) {
                    3212836864u64
                } else {
                    let a657 = v0;
                    let a658 = 5u64;
                    if nat_lt(a657, a658) {
                        2139095040u64
                    } else {
                        4286578688u64
                    }
                }
            }
        } else {
            let a659 = v0;
            let a660 = 9u64;
            if nat_lt(a659, a660) {
                let a661 = v0;
                let a662 = 7u64;
                if nat_lt(a661, a662) {
                    2143289345u64
                } else {
                    let a663 = v0;
                    let a664 = 8u64;
                    if nat_lt(a663, a664) {
                        2139095041u64
                    } else {
                        4286578689u64
                    }
                }
            } else {
                let a665 = v0;
                let a666 = 10u64;
                if nat_lt(a665, a666) {
                    1u64
                } else {
                    let a667 = v0;
                    let a668 = 11u64;
                    if nat_lt(a667, a668) {
                        2147483649u64
                    } else {
                        8388608u64
                    }
                }
            }
        }
    } else {
        let a669 = v0;
        let a670 = 18u64;
        if nat_lt(a669, a670) {
            let a671 = v0;
            let a672 = 15u64;
            if nat_lt(a671, a672) {
                let a673 = v0;
                let a674 = 13u64;
                if nat_lt(a673, a674) {
                    2139095039u64
                } else {
                    let a675 = v0;
                    let a676 = 14u64;
                    if nat_lt(a675, a676) {
                        4286578687u64
                    } else {
                        864026624u64
                    }
                }
            } else {
                let a677 = v0;
                let a678 = 16u64;
                if nat_lt(a677, a678) {
                    855638016u64
                } else {
                    let a679 = v0;
                    let a680 = 17u64;
                    if nat_lt(a679, a680) {
                        855638017u64
                    } else {
                        855638015u64
                    }
                }
            }
        } else {
            let a681 = v0;
            let a682 = 21u64;
            if nat_lt(a681, a682) {
                let a683 = v0;
                let a684 = 19u64;
                if nat_lt(a683, a684) {
                    1065357312u64
                } else {
                    let a685 = v0;
                    let a686 = 20u64;
                    if nat_lt(a685, a686) {
                        1065365504u64
                    } else {
                        1199562752u64
                    }
                }
            } else {
                let a687 = v0;
                let a688 = 22u64;
                if nat_lt(a687, a688) {
                    1199566848u64
                } else {
                    let a689 = v0;
                    let a690 = 23u64;
                    if nat_lt(a689, a690) {
                        1199566847u64
                    } else {
                        947912704u64
                    }
                }
            }
        }
    }
}

pub fn f9(v0: u64) -> u64 {
    let a691 = v0;
    let a692 = 12u64;
    if nat_lt(a691, a692) {
        let a693 = v0;
        let a694 = 6u64;
        if nat_lt(a693, a694) {
            let a695 = v0;
            let a696 = 3u64;
            if nat_lt(a695, a696) {
                let a697 = v0;
                let a698 = 1u64;
                if nat_lt(a697, a698) {
                    0u64
                } else {
                    let a699 = v0;
                    let a700 = 2u64;
                    if nat_lt(a699, a700) {
                        32768u64
                    } else {
                        15360u64
                    }
                }
            } else {
                let a701 = v0;
                let a702 = 4u64;
                if nat_lt(a701, a702) {
                    16256u64
                } else {
                    let a703 = v0;
                    let a704 = 5u64;
                    if nat_lt(a703, a704) {
                        48128u64
                    } else {
                        49024u64
                    }
                }
            }
        } else {
            let a705 = v0;
            let a706 = 9u64;
            if nat_lt(a705, a706) {
                let a707 = v0;
                let a708 = 7u64;
                if nat_lt(a707, a708) {
                    31744u64
                } else {
                    let a709 = v0;
                    let a710 = 8u64;
                    if nat_lt(a709, a710) {
                        32640u64
                    } else {
                        64512u64
                    }
                }
            } else {
                let a711 = v0;
                let a712 = 10u64;
                if nat_lt(a711, a712) {
                    65408u64
                } else {
                    let a713 = v0;
                    let a714 = 11u64;
                    if nat_lt(a713, a714) {
                        32256u64
                    } else {
                        32704u64
                    }
                }
            }
        }
    } else {
        let a715 = v0;
        let a716 = 18u64;
        if nat_lt(a715, a716) {
            let a717 = v0;
            let a718 = 15u64;
            if nat_lt(a717, a718) {
                let a719 = v0;
                let a720 = 13u64;
                if nat_lt(a719, a720) {
                    31745u64
                } else {
                    let a721 = v0;
                    let a722 = 14u64;
                    if nat_lt(a721, a722) {
                        32641u64
                    } else {
                        65025u64
                    }
                }
            } else {
                let a723 = v0;
                let a724 = 16u64;
                if nat_lt(a723, a724) {
                    65409u64
                } else {
                    let a725 = v0;
                    let a726 = 17u64;
                    if nat_lt(a725, a726) {
                        1u64
                    } else {
                        32769u64
                    }
                }
            }
        } else {
            let a727 = v0;
            let a728 = 21u64;
            if nat_lt(a727, a728) {
                let a729 = v0;
                let a730 = 19u64;
                if nat_lt(a729, a730) {
                    1023u64
                } else {
                    let a731 = v0;
                    let a732 = 20u64;
                    if nat_lt(a731, a732) {
                        127u64
                    } else {
                        1024u64
                    }
                }
            } else {
                let a733 = v0;
                let a734 = 22u64;
                if nat_lt(a733, a734) {
                    128u64
                } else {
                    let a735 = v0;
                    let a736 = 23u64;
                    if nat_lt(a735, a736) {
                        31743u64
                    } else {
                        32639u64
                    }
                }
            }
        }
    }
}

pub fn f10(v0: u64) -> u64 {
    let a737 = v0;
    let a738 = 16u64;
    if nat_lt(a737, a738) {
        let a739 = v0;
        let a740 = 8u64;
        if nat_lt(a739, a740) {
            let a741 = v0;
            let a742 = 4u64;
            if nat_lt(a741, a742) {
                let a743 = v0;
                let a744 = 2u64;
                if nat_lt(a743, a744) {
                    let a745 = v0;
                    let a746 = 1u64;
                    if nat_lt(a745, a746) {
                        0u64
                    } else {
                        2147483648u64
                    }
                } else {
                    let a747 = v0;
                    let a748 = 3u64;
                    if nat_lt(a747, a748) {
                        1056964608u64
                    } else {
                        3204448256u64
                    }
                }
            } else {
                let a749 = v0;
                let a750 = 6u64;
                if nat_lt(a749, a750) {
                    let a751 = v0;
                    let a752 = 5u64;
                    if nat_lt(a751, a752) {
                        1069547520u64
                    } else {
                        3217031168u64
                    }
                } else {
                    let a753 = v0;
                    let a754 = 7u64;
                    if nat_lt(a753, a754) {
                        1075838976u64
                    } else {
                        3223322624u64
                    }
                }
            }
        } else {
            let a755 = v0;
            let a756 = 12u64;
            if nat_lt(a755, a756) {
                let a757 = v0;
                let a758 = 10u64;
                if nat_lt(a757, a758) {
                    let a759 = v0;
                    let a760 = 9u64;
                    if nat_lt(a759, a760) {
                        1077936128u64
                    } else {
                        3225419776u64
                    }
                } else {
                    let a761 = v0;
                    let a762 = 11u64;
                    if nat_lt(a761, a762) {
                        1077936127u64
                    } else {
                        1077936129u64
                    }
                }
            } else {
                let a763 = v0;
                let a764 = 14u64;
                if nat_lt(a763, a764) {
                    let a765 = v0;
                    let a766 = 13u64;
                    if nat_lt(a765, a766) {
                        3225419775u64
                    } else {
                        3225419777u64
                    }
                } else {
                    let a767 = v0;
                    let a768 = 15u64;
                    if nat_lt(a767, a768) {
                        1065353216u64
                    } else {
                        3212836864u64
                    }
                }
            }
        }
    } else {
        let a769 = v0;
        let a770 = 24u64;
        if nat_lt(a769, a770) {
            let a771 = v0;
            let a772 = 20u64;
            if nat_lt(a771, a772) {
                let a773 = v0;
                let a774 = 18u64;
                if nat_lt(a773, a774) {
                    let a775 = v0;
                    let a776 = 17u64;
                    if nat_lt(a775, a776) {
                        2139095040u64
                    } else {
                        4286578688u64
                    }
                } else {
                    let a777 = v0;
                    let a778 = 19u64;
                    if nat_lt(a777, a778) {
                        2143289345u64
                    } else {
                        2139095041u64
                    }
                }
            } else {
                let a779 = v0;
                let a780 = 22u64;
                if nat_lt(a779, a780) {
                    let a781 = v0;
                    let a782 = 21u64;
                    if nat_lt(a781, a782) {
                        4286578689u64
                    } else {
                        1u64
                    }
                } else {
                    let a783 = v0;
                    let a784 = 23u64;
                    if nat_lt(a783, a784) {
                        2147483649u64
                    } else {
                        8388608u64
                    }
                }
            }
        } else {
            let a785 = v0;
            let a786 = 28u64;
            if nat_lt(a785, a786) {
                let a787 = v0;
                let a788 = 26u64;
                if nat_lt(a787, a788) {
                    let a789 = v0;
                    let a790 = 25u64;
                    if nat_lt(a789, a790) {
                        2139095039u64
                    } else {
                        4286578687u64
                    }
                } else {
                    let a791 = v0;
                    let a792 = 27u64;
                    if nat_lt(a791, a792) {
                        1258291199u64
                    } else {
                        3405774847u64
                    }
                }
            } else {
                let a793 = v0;
                let a794 = 30u64;
                if nat_lt(a793, a794) {
                    let a795 = v0;
                    let a796 = 29u64;
                    if nat_lt(a795, a796) {
                        1258291200u64
                    } else {
                        3405774848u64
                    }
                } else {
                    let a797 = v0;
                    let a798 = 31u64;
                    if nat_lt(a797, a798) {
                        1065353215u64
                    } else {
                        3212836863u64
                    }
                }
            }
        }
    }
}

pub fn f11(v0: u64) -> u64 {
    let a799 = v0;
    let a800 = 8u64;
    if nat_lt(a799, a800) {
        let a801 = v0;
        let a802 = 4u64;
        if nat_lt(a801, a802) {
            let a803 = v0;
            let a804 = 2u64;
            if nat_lt(a803, a804) {
                let a805 = v0;
                let a806 = 1u64;
                let _ = nat_lt(a805, a806);
                4286578688u64
            } else {
                let a807 = v0;
                let a808 = 3u64;
                let _ = nat_lt(a807, a808);
                4286578688u64
            }
        } else {
            let a809 = v0;
            let a810 = 6u64;
            if nat_lt(a809, a810) {
                let a811 = v0;
                let a812 = 5u64;
                if nat_lt(a811, a812) {
                    2139095041u64
                } else {
                    1065353216u64
                }
            } else {
                let a813 = v0;
                let a814 = 7u64;
                if nat_lt(a813, a814) {
                    0u64
                } else {
                    3212836864u64
                }
            }
        }
    } else {
        let a815 = v0;
        let a816 = 12u64;
        if nat_lt(a815, a816) {
            let a817 = v0;
            let a818 = 10u64;
            if nat_lt(a817, a818) {
                let a819 = v0;
                let a820 = 9u64;
                if nat_lt(a819, a820) {
                    1065353216u64
                } else {
                    2143289345u64
                }
            } else {
                let a821 = v0;
                let a822 = 11u64;
                if nat_lt(a821, a822) {
                    3212836864u64
                } else {
                    0u64
                }
            }
        } else {
            let a823 = v0;
            let a824 = 14u64;
            if nat_lt(a823, a824) {
                let a825 = v0;
                let a826 = 13u64;
                if nat_lt(a825, a826) {
                    3225419776u64
                } else {
                    3221225472u64
                }
            } else {
                let a827 = v0;
                let a828 = 15u64;
                if nat_lt(a827, a828) {
                    3212836864u64
                } else {
                    3221225472u64
                }
            }
        }
    }
}

pub fn f12(v0: u64) -> u64 {
    let a829 = v0;
    let a830 = 14u64;
    if nat_lt(a829, a830) {
        let a831 = v0;
        let a832 = 7u64;
        if nat_lt(a831, a832) {
            let a833 = v0;
            let a834 = 3u64;
            if nat_lt(a833, a834) {
                let a835 = v0;
                let a836 = 1u64;
                if nat_lt(a835, a836) {
                    1065353216u64
                } else {
                    let a837 = v0;
                    let a838 = 2u64;
                    if nat_lt(a837, a838) {
                        1073741824u64
                    } else {
                        3212836864u64
                    }
                }
            } else {
                let a839 = v0;
                let a840 = 5u64;
                if nat_lt(a839, a840) {
                    let a841 = v0;
                    let a842 = 4u64;
                    if nat_lt(a841, a842) {
                        0u64
                    } else {
                        1266679808u64
                    }
                } else {
                    let a843 = v0;
                    let a844 = 6u64;
                    if nat_lt(a843, a844) {
                        1065353216u64
                    } else {
                        3414163456u64
                    }
                }
            }
        } else {
            let a845 = v0;
            let a846 = 10u64;
            if nat_lt(a845, a846) {
                let a847 = v0;
                let a848 = 8u64;
                if nat_lt(a847, a848) {
                    3212836864u64
                } else {
                    let a849 = v0;
                    let a850 = 9u64;
                    let _ = nat_lt(a849, a850);
                    2147483648u64
                }
            } else {
                let a851 = v0;
                let a852 = 12u64;
                if nat_lt(a851, a852) {
                    let a853 = v0;
                    let a854 = 11u64;
                    if nat_lt(a853, a854) {
                        0u64
                    } else {
                        2147483648u64
                    }
                } else {
                    let a855 = v0;
                    let a856 = 13u64;
                    if nat_lt(a855, a856) {
                        2139095040u64
                    } else {
                        4286578688u64
                    }
                }
            }
        }
    } else {
        let a857 = v0;
        let a858 = 21u64;
        if nat_lt(a857, a858) {
            let a859 = v0;
            let a860 = 17u64;
            if nat_lt(a859, a860) {
                let a861 = v0;
                let a862 = 15u64;
                if nat_lt(a861, a862) {
                    1065353216u64
                } else {
                    let a863 = v0;
                    let a864 = 16u64;
                    if nat_lt(a863, a864) {
                        0u64
                    } else {
                        2139095041u64
                    }
                }
            } else {
                let a865 = v0;
                let a866 = 19u64;
                if nat_lt(a865, a866) {
                    let a867 = v0;
                    let a868 = 18u64;
                    if nat_lt(a867, a868) {
                        4290772993u64
                    } else {
                        2143289635u64
                    }
                } else {
                    let a869 = v0;
                    let a870 = 20u64;
                    if nat_lt(a869, a870) {
                        4286578689u64
                    } else {
                        2139095039u64
                    }
                }
            }
        } else {
            let a871 = v0;
            let a872 = 24u64;
            if nat_lt(a871, a872) {
                let a873 = v0;
                let a874 = 22u64;
                if nat_lt(a873, a874) {
                    2139095039u64
                } else {
                    let a875 = v0;
                    let a876 = 23u64;
                    if nat_lt(a875, a876) {
                        4286578687u64
                    } else {
                        8388608u64
                    }
                }
            } else {
                let a877 = v0;
                let a878 = 26u64;
                if nat_lt(a877, a878) {
                    let a879 = v0;
                    let a880 = 25u64;
                    if nat_lt(a879, a880) {
                        1u64
                    } else {
                        2147483649u64
                    }
                } else {
                    let a881 = v0;
                    let a882 = 27u64;
                    if nat_lt(a881, a882) {
                        8388607u64
                    } else {
                        8388608u64
                    }
                }
            }
        }
    }
}

pub fn f13(v0: u64) -> u64 {
    let a883 = v0;
    let a884 = 2u64;
    if nat_lt(a883, a884) {
        let a885 = v0;
        let a886 = 1u64;
        if nat_lt(a885, a886) {
            1518338048u64
        } else {
            0u64
        }
    } else {
        let a887 = v0;
        let a888 = 3u64;
        if nat_lt(a887, a888) {
            3665821696u64
        } else {
            1065353216u64
        }
    }
}

pub fn f14(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let v6: u64 = {
        let a889 = v0;
        let a890 = 0u64;
        if nat_eq(a889, a890) {
            4u64
        } else {
            10u64
        }
    };
    let v7: u64 = {
        let a891 = v1;
        let a892 = v6;
        let a893 = 0u64;
        nat_rem(a891, a892, a893)
    };
    let v8: u64 = {
        let a897 = {
            let a894 = v1;
            let a895 = v6;
            let a896 = 0u64;
            nat_quot(a894, a895, a896)
        };
        let a898 = 0u64;
        if nat_eq(a897, a898) {
            1u64
        } else {
            let a902 = {
                let a899 = v1;
                let a900 = v6;
                let a901 = 0u64;
                nat_quot(a899, a900, a901)
            };
            let a903 = 1u64;
            if nat_eq(a902, a903) {
                2u64
            } else {
                4u64
            }
        }
    };
    let v9: u64 = {
        let a904 = v7;
        let a905 = 2u64;
        if nat_eq(a904, a905) {
            0u64
        } else {
            v3
        }
    };
    let v10: u64 = {
        let a906 = v7;
        let a907 = 3u64;
        if nat_eq(a906, a907) {
            0u64
        } else {
            v4
        }
    };
    let v11: u64 = {
        let a912 = {
            let a910 = v9;
            let a911 = {
                let a908 = v8;
                let a909 = 1u64;
                nat_sub(a908, a909)
            };
            nat_add(a910, a911)?
        };
        let a913 = v8;
        let a914 = 0u64;
        nat_quot(a912, a913, a914)
    };
    let v12: u64 = {
        let a922 = v11;
        let a923 = {
            let a919 = {
                let a917 = v10;
                let a918 = {
                    let a915 = v8;
                    let a916 = 1u64;
                    nat_sub(a915, a916)
                };
                nat_add(a917, a918)?
            };
            let a920 = v8;
            let a921 = 0u64;
            nat_quot(a919, a920, a921)
        };
        nat_mul(a922, a923)?
    };
    let v13: u64 = {
        let a924 = f19(v9, v8, 1u64)?;
        let a925 = f19(v10, v8, v11)?;
        nat_add(a924, a925)?
    };
    let a926 = v5;
    let a927 = 4u64;
    if nat_eq(a926, a927) {
        Ok(v7)
    } else {
        let a928 = v5;
        let a929 = 5u64;
        if nat_eq(a928, a929) {
            Ok(v8)
        } else {
            let a930 = v5;
            let a931 = 6u64;
            if nat_eq(a930, a931) {
                Ok(v9)
            } else {
                let a932 = v5;
                let a933 = 7u64;
                if nat_eq(a932, a933) {
                    Ok(v10)
                } else {
                    let a934 = v5;
                    let a935 = 0u64;
                    if nat_eq(a934, a935) {
                        let a936 = v7;
                        let a937 = 1u64;
                        if nat_eq(a936, a937) {
                            Ok(0u64)
                        } else {
                            Ok(v2)
                        }
                    } else {
                        let a938 = v0;
                        let a939 = 0u64;
                        if nat_eq(a938, a939) {
                            let a940 = v5;
                            let a941 = 0u64;
                            if nat_eq(a940, a941) {
                                Ok(v2)
                            } else {
                                let a942 = v5;
                                let a943 = 1u64;
                                if nat_eq(a942, a943) {
                                    Ok(v9)
                                } else {
                                    let a944 = v5;
                                    let a945 = 2u64;
                                    if nat_eq(a944, a945) {
                                        Ok(v10)
                                    } else {
                                        Ok(1u64)
                                    }
                                }
                            }
                        } else {
                            let a946 = 7u64;
                            let a947 = v7;
                            if nat_le(a946, a947) {
                                let a948 = v5;
                                let a949 = 3u64;
                                if nat_lt(a948, a949) {
                                    Ok(1u64)
                                } else {
                                    let a950 = v7;
                                    let a951 = 7u64;
                                    if nat_eq(a950, a951) {
                                        let a954 = v12;
                                        let a955 = {
                                            let a952 = v8;
                                            let a953 = v8;
                                            nat_mul(a952, a953)?
                                        };
                                        nat_mul(a954, a955)
                                    } else {
                                        let a956 = v7;
                                        let a957 = 8u64;
                                        if nat_eq(a956, a957) {
                                            let a958 = v13;
                                            let a959 = 1u64;
                                            nat_add(a958, a959)
                                        } else {
                                            Ok(v13)
                                        }
                                    }
                                }
                            } else {
                                let a968 = {
                                    let a960 = v5;
                                    let a961 = 3u64;
                                    if nat_lt(a960, a961) {
                                        v8
                                    } else {
                                        v12
                                    }
                                };
                                let a969 = {
                                    let a962 = 4u64;
                                    let a963 = v7;
                                    if if nat_le(a962, a963) {
                                        let a966 = v7;
                                        let a967 = {
                                            let a964 = v5;
                                            let a965 = 3u64;
                                            nat_add(a964, a965)?
                                        };
                                        nat_eq(a966, a967)
                                    } else {
                                        false
                                    } {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a968, a969)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f15(v0: u64, v1: u64) -> u64 {
    let v2: u64 = {
        let a970 = v0;
        let a971 = 2u64;
        let a972 = 0u64;
        nat_quot(a970, a971, a972)
    };
    let a973 = v2;
    let a974 = 1u64;
    if if nat_eq(a973, a974) {
        true
    } else {
        let a975 = v2;
        let a976 = 7u64;
        nat_eq(a975, a976)
    } {
        let a977 = v2;
        let a978 = 1u64;
        if nat_eq(a977, a978) {
            1u64
        } else {
            2u64
        }
    } else {
        let a979 = v2;
        let a980 = 2u64;
        if nat_eq(a979, a980) {
            2u64
        } else {
            let a981 = v2;
            let a982 = 4u64;
            if if nat_eq(a981, a982) {
                true
            } else {
                let a983 = v2;
                let a984 = 6u64;
                nat_eq(a983, a984)
            } {
                let a985 = v1;
                let a986 = 0u64;
                if nat_eq(a985, a986) {
                    0u64
                } else {
                    4294967295u64
                }
            } else {
                let a987 = v2;
                let a988 = 8u64;
                if nat_eq(a987, a988) {
                    2147483647u64
                } else {
                    let a989 = v2;
                    let a990 = 9u64;
                    if nat_eq(a989, a990) {
                        2147483649u64
                    } else {
                        let a991 = v2;
                        let a992 = 10u64;
                        if nat_eq(a991, a992) {
                            let a993 = v1;
                            let a994 = 0u64;
                            if nat_eq(a993, a994) {
                                2147483649u64
                            } else {
                                2147483648u64
                            }
                        } else {
                            0u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f16(v0: u64, v1: u64) -> u64 {
    let v2: u64 = {
        let a995 = v0;
        let a996 = 2u64;
        let a997 = 0u64;
        nat_quot(a995, a996, a997)
    };
    let a998 = v2;
    let a999 = 1u64;
    if if nat_eq(a998, a999) {
        true
    } else {
        let a1000 = v2;
        let a1001 = 3u64;
        if nat_eq(a1000, a1001) {
            true
        } else {
            let a1002 = v2;
            let a1003 = 6u64;
            nat_eq(a1002, a1003)
        }
    } {
        2u64
    } else {
        let a1004 = v2;
        let a1005 = 5u64;
        if if nat_eq(a1004, a1005) {
            true
        } else {
            let a1006 = v2;
            let a1007 = 7u64;
            nat_eq(a1006, a1007)
        } {
            let a1008 = v1;
            let a1009 = 0u64;
            if nat_eq(a1008, a1009) {
                0u64
            } else {
                4294967295u64
            }
        } else {
            let a1010 = v2;
            let a1011 = 8u64;
            if nat_eq(a1010, a1011) {
                2147483649u64
            } else {
                let a1012 = v2;
                let a1013 = 9u64;
                if if nat_eq(a1012, a1013) {
                    true
                } else {
                    let a1014 = v2;
                    let a1015 = 10u64;
                    nat_eq(a1014, a1015)
                } {
                    2147483647u64
                } else {
                    0u64
                }
            }
        }
    }
}

pub fn f17(v0: u64, v1: u64) -> u64 {
    let a1016 = v1;
    let a1017 = 0u64;
    if nat_eq(a1016, a1017) {
        v0
    } else {
        let a1018 = v0;
        let a1019 = 2147483648u64;
        if nat_lt(a1018, a1019) {
            let a1020 = v0;
            let a1021 = v1;
            let a1022 = 0u64;
            nat_rem(a1020, a1021, a1022)
        } else {
            f18({
                let a1025 = {
                    let a1023 = 4294967296u64;
                    let a1024 = v0;
                    nat_sub(a1023, a1024)
                };
                let a1026 = v1;
                let a1027 = 0u64;
                nat_rem(a1025, a1026, a1027)
            })
        }
    }
}

pub fn f18(v0: u64) -> u64 {
    let a1030 = {
        let a1028 = 4294967296u64;
        let a1029 = v0;
        nat_sub(a1028, a1029)
    };
    let a1031 = 4294967296u64;
    let a1032 = 0u64;
    nat_rem(a1030, a1031, a1032)
}

pub fn f19(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a1033 = v0;
        let a1034 = 1u64;
        nat_sub(a1033, a1034)
    };
    let v4: u64 = {
        let a1043 = {
            let a1038 = {
                let a1035 = v3;
                let a1036 = v1;
                let a1037 = 0u64;
                nat_quot(a1035, a1036, a1037)
            };
            let a1039 = v2;
            nat_mul(a1038, a1039)?
        };
        let a1044 = {
            let a1040 = v3;
            let a1041 = v1;
            let a1042 = 0u64;
            nat_rem(a1040, a1041, a1042)
        };
        nat_add(a1043, a1044)?
    };
    let v5: u64 = {
        let a1054 = {
            let a1050 = {
                let a1048 = {
                    let a1045 = v0;
                    let a1046 = v1;
                    let a1047 = 0u64;
                    nat_quot(a1045, a1046, a1047)
                };
                let a1049 = 1u64;
                nat_sub(a1048, a1049)
            };
            let a1051 = v2;
            nat_mul(a1050, a1051)?
        };
        let a1055 = {
            let a1052 = v1;
            let a1053 = 1u64;
            nat_sub(a1052, a1053)
        };
        nat_add(a1054, a1055)?
    };
    let a1056 = v0;
    let a1057 = v1;
    if nat_lt(a1056, a1057) {
        Ok(v4)
    } else {
        let a1058 = v4;
        let a1059 = v5;
        if nat_lt(a1058, a1059) {
            Ok(v5)
        } else {
            Ok(v4)
        }
    }
}

pub fn run(p0: u64, p1: u64, p2: u64, p3: u64) -> R<Option<u64>> {
    f0(p0, p1, p2, p3)
}
