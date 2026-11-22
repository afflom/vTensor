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
    let a1 = v2;
    let a2 = 2147483647u64;
    if if nat_le(a1, a2) {
        let a3 = v0;
        let a4 = 4u64;
        nat_lt(a3, a4)
    } else {
        false
    } {
        let a5 = v0;
        let a6 = 0u64;
        if if nat_eq(a5, a6) {
            true
        } else {
            let a7 = v0;
            let a8 = 3u64;
            nat_eq(a7, a8)
        } {
            let a9 = v1;
            let a10 = 22u64;
            if if nat_lt(a9, a10) {
                let a11 = v3;
                let a12 = 2u64;
                nat_lt(a11, a12)
            } else {
                false
            } {
                Ok(Some(f1({
                    let a13 = v0;
                    let a14 = 3u64;
                    nat_eq(a13, a14)
                }, v1, v2, v3)?))
            } else {
                Ok(None::<u64>)
            }
        } else {
            let a15 = v0;
            let a16 = 1u64;
            if nat_eq(a15, a16) {
                let a17 = v1;
                let a18 = 4u64;
                if if nat_lt(a17, a18) {
                    let a19 = v3;
                    let a20 = 2u64;
                    if nat_lt(a19, a20) {
                        let a21 = 0u64;
                        let a22 = v2;
                        nat_lt(a21, a22)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Some(f2(v1, v2, v3)))
                } else {
                    Ok(None::<u64>)
                }
            } else {
                let a23 = v1;
                let a24 = 17u64;
                if if nat_lt(a23, a24) {
                    let a25 = v3;
                    let a26 = 4u64;
                    if nat_lt(a25, a26) {
                        let a27 = 0u64;
                        let a28 = v2;
                        nat_lt(a27, a28)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Some(f3(v1, v2, v3)))
                } else {
                    Ok(None::<u64>)
                }
            }
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f1(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let v4: u64 = f4(v1, v2);
    let v5: u64 = f5(v1, v2);
    if if if v0 {
        let a32 = {
            let a29 = v1;
            let a30 = 2u64;
            let a31 = 0u64;
            nat_rem(a29, a30, a31)
        };
        let a33 = 1u64;
        nat_eq(a32, a33)
    } else {
        false
    } {
        let a34 = 0u64;
        let a35 = v2;
        nat_lt(a34, a35)
    } else {
        false
    } {
        let v6: u64 = f6(v4, v2);
        let a36 = v3;
        let a37 = 0u64;
        if nat_eq(a36, a37) {
            Ok(v6)
        } else {
            let a44 = {
                let a42 = {
                    let a40 = 4294967296u64;
                    let a41 = {
                        let a38 = v4;
                        let a39 = v5;
                        nat_add(a38, a39)?
                    };
                    nat_add(a40, a41)?
                };
                let a43 = v6;
                nat_sub(a42, a43)
            };
            let a45 = 4294967296u64;
            let a46 = 0u64;
            Ok(nat_rem(a44, a45, a46))
        }
    } else {
        let a47 = v3;
        let a48 = 0u64;
        if nat_eq(a47, a48) {
            Ok(v4)
        } else {
            Ok(v5)
        }
    }
}

pub fn f2(v0: u64, v1: u64, v2: u64) -> u64 {
    let a57 = {
        let a54 = {
            let a51 = v0;
            let a52 = {
                let a49 = v2;
                let a50 = 0u64;
                if nat_eq(a49, a50) {
                    1u64
                } else {
                    2u64
                }
            };
            let a53 = 0u64;
            nat_quot(a51, a52, a53)
        };
        let a55 = 2u64;
        let a56 = 0u64;
        nat_rem(a54, a55, a56)
    };
    let a58 = 1u64;
    if nat_eq(a57, a58) {
        let a59 = v1;
        let a60 = 1u64;
        nat_sub(a59, a60)
    } else {
        0u64
    }
}

pub fn f3(v0: u64, v1: u64, v2: u64) -> u64 {
    let a61 = v0;
    let a62 = 0u64;
    if nat_eq(a61, a62) {
        0u64
    } else {
        let v3: u64 = {
            let a63 = v1;
            let a64 = 1u64;
            nat_sub(a63, a64)
        };
        let a79 = {
            let a76 = {
                let a73 = {
                    let a65 = v0;
                    let a66 = 1u64;
                    nat_sub(a65, a66)
                };
                let a74 = {
                    let a67 = v2;
                    let a68 = 2u64;
                    if nat_lt(a67, a68) {
                        let a69 = v2;
                        let a70 = 0u64;
                        if nat_eq(a69, a70) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a71 = v2;
                        let a72 = 2u64;
                        if nat_eq(a71, a72) {
                            4u64
                        } else {
                            8u64
                        }
                    }
                };
                let a75 = 0u64;
                nat_quot(a73, a74, a75)
            };
            let a77 = 2u64;
            let a78 = 0u64;
            nat_rem(a76, a77, a78)
        };
        let a80 = 1u64;
        if nat_eq(a79, a80) {
            f7(v3)
        } else {
            v3
        }
    }
}

pub fn f4(v0: u64, v1: u64) -> u64 {
    let v2: u64 = {
        let a81 = v0;
        let a82 = 2u64;
        let a83 = 0u64;
        nat_quot(a81, a82, a83)
    };
    let a84 = v2;
    let a85 = 1u64;
    if if nat_eq(a84, a85) {
        true
    } else {
        let a86 = v2;
        let a87 = 7u64;
        nat_eq(a86, a87)
    } {
        let a88 = v2;
        let a89 = 1u64;
        if nat_eq(a88, a89) {
            1u64
        } else {
            2u64
        }
    } else {
        let a90 = v2;
        let a91 = 2u64;
        if nat_eq(a90, a91) {
            2u64
        } else {
            let a92 = v2;
            let a93 = 4u64;
            if if nat_eq(a92, a93) {
                true
            } else {
                let a94 = v2;
                let a95 = 6u64;
                nat_eq(a94, a95)
            } {
                let a96 = v1;
                let a97 = 0u64;
                if nat_eq(a96, a97) {
                    0u64
                } else {
                    4294967295u64
                }
            } else {
                let a98 = v2;
                let a99 = 8u64;
                if nat_eq(a98, a99) {
                    2147483647u64
                } else {
                    let a100 = v2;
                    let a101 = 9u64;
                    if nat_eq(a100, a101) {
                        2147483649u64
                    } else {
                        let a102 = v2;
                        let a103 = 10u64;
                        if nat_eq(a102, a103) {
                            let a104 = v1;
                            let a105 = 0u64;
                            if nat_eq(a104, a105) {
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

pub fn f5(v0: u64, v1: u64) -> u64 {
    let v2: u64 = {
        let a106 = v0;
        let a107 = 2u64;
        let a108 = 0u64;
        nat_quot(a106, a107, a108)
    };
    let a109 = v2;
    let a110 = 1u64;
    if if nat_eq(a109, a110) {
        true
    } else {
        let a111 = v2;
        let a112 = 3u64;
        if nat_eq(a111, a112) {
            true
        } else {
            let a113 = v2;
            let a114 = 6u64;
            nat_eq(a113, a114)
        }
    } {
        2u64
    } else {
        let a115 = v2;
        let a116 = 5u64;
        if if nat_eq(a115, a116) {
            true
        } else {
            let a117 = v2;
            let a118 = 7u64;
            nat_eq(a117, a118)
        } {
            let a119 = v1;
            let a120 = 0u64;
            if nat_eq(a119, a120) {
                0u64
            } else {
                4294967295u64
            }
        } else {
            let a121 = v2;
            let a122 = 8u64;
            if nat_eq(a121, a122) {
                2147483649u64
            } else {
                let a123 = v2;
                let a124 = 9u64;
                if if nat_eq(a123, a124) {
                    true
                } else {
                    let a125 = v2;
                    let a126 = 10u64;
                    nat_eq(a125, a126)
                } {
                    2147483647u64
                } else {
                    0u64
                }
            }
        }
    }
}

pub fn f6(v0: u64, v1: u64) -> u64 {
    let a127 = v1;
    let a128 = 0u64;
    if nat_eq(a127, a128) {
        v0
    } else {
        let a129 = v0;
        let a130 = 2147483648u64;
        if nat_lt(a129, a130) {
            let a131 = v0;
            let a132 = v1;
            let a133 = 0u64;
            nat_rem(a131, a132, a133)
        } else {
            f7({
                let a136 = {
                    let a134 = 4294967296u64;
                    let a135 = v0;
                    nat_sub(a134, a135)
                };
                let a137 = v1;
                let a138 = 0u64;
                nat_rem(a136, a137, a138)
            })
        }
    }
}

pub fn f7(v0: u64) -> u64 {
    let a141 = {
        let a139 = 4294967296u64;
        let a140 = v0;
        nat_sub(a139, a140)
    };
    let a142 = 4294967296u64;
    let a143 = 0u64;
    nat_rem(a141, a142, a143)
}

pub fn run(p0: u64, p1: u64, p2: u64, p3: u64) -> R<Option<u64>> {
    f0(p0, p1, p2, p3)
}
