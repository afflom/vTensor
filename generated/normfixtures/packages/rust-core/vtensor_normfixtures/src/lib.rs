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
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0,
    C1(u64),
}

pub fn f0(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a1 = v0;
    let a2 = 0u64;
    if nat_eq(a1, a2) {
        let a3 = v3;
        let a4 = 6u64;
        if nat_lt(a3, a4) {
            Ok(Some(f1(v3)))
        } else {
            Ok(None::<u64>)
        }
    } else {
        let a5 = v1;
        let a6 = 184u64;
        if nat_lt(a5, a6) {
            let a7 = v0;
            let a8 = 1u64;
            if nat_eq(a7, a8) {
                let a9 = v2;
                let a10 = 0u64;
                if if nat_eq(a9, a10) {
                    let a11 = v3;
                    let a12 = 10u64;
                    nat_lt(a11, a12)
                } else {
                    false
                } {
                    Ok(Some(f2(v1, v3)))
                } else {
                    Ok(None::<u64>)
                }
            } else {
                let a13 = v0;
                let a14 = 2u64;
                if nat_eq(a13, a14) {
                    let a15 = v2;
                    let a16 = 0u64;
                    if if if nat_eq(a15, a16) {
                        true
                    } else {
                        let a17 = v2;
                        let a18 = 1u64;
                        if nat_eq(a17, a18) {
                            true
                        } else {
                            let a19 = v2;
                            let a20 = 3u64;
                            nat_eq(a19, a20)
                        }
                    } {
                        let a21 = v3;
                        let a22 = 10u64;
                        nat_lt(a21, a22)
                    } else {
                        false
                    } {
                        Ok(Some({
                            let a23 = v3;
                            let a24 = 4u64;
                            if nat_lt(a23, a24) {
                                f3(v1, v3)
                            } else {
                                let a25 = v3;
                                let a26 = 8u64;
                                if nat_lt(a25, a26) {
                                    f4(v1, v2, {
                                        let a27 = v3;
                                        let a28 = 4u64;
                                        nat_sub(a27, a28)
                                    })?
                                } else {
                                    let a29 = v3;
                                    let a30 = 8u64;
                                    if nat_eq(a29, a30) {
                                        f5(v1, v2)?
                                    } else {
                                        f6(v1)?
                                    }
                                }
                            }
                        }))
                    } else {
                        Ok(None::<u64>)
                    }
                } else {
                    let a31 = v0;
                    let a32 = 3u64;
                    if nat_eq(a31, a32) {
                        let a33 = v2;
                        let a34 = 0u64;
                        if if nat_eq(a33, a34) {
                            let a35 = v3;
                            let a36 = 512u64;
                            nat_lt(a35, a36)
                        } else {
                            false
                        } {
                            Ok(Some(f7(v1, v3)?))
                        } else {
                            Ok(None::<u64>)
                        }
                    } else {
                        let a37 = v0;
                        let a38 = 4u64;
                        if nat_eq(a37, a38) {
                            let a39 = v3;
                            let a40 = 2u64;
                            if nat_lt(a39, a40) {
                                let a41 = v2;
                                let a42 = 0u64;
                                if nat_eq(a41, a42) {
                                    let a43 = v3;
                                    let a44 = 0u64;
                                    if nat_eq(a43, a44) {
                                        Ok(Some(f8(v1)?))
                                    } else {
                                        Ok(None::<u64>)
                                    }
                                } else {
                                    let a45 = v2;
                                    let a46 = f8(v1)?;
                                    if nat_le(a45, a46) {
                                        Ok(Some({
                                            let a47 = v3;
                                            let a48 = 0u64;
                                            if nat_eq(a47, a48) {
                                                let a51 = {
                                                    let a49 = v2;
                                                    let a50 = 1u64;
                                                    nat_sub(a49, a50)
                                                };
                                                let a52 = 512u64;
                                                nat_mul(a51, a52)?
                                            } else {
                                                let v4: u64 = {
                                                    let a57 = f5(v1, 0u64)?;
                                                    let a58 = {
                                                        let a55 = {
                                                            let a53 = v2;
                                                            let a54 = 1u64;
                                                            nat_sub(a53, a54)
                                                        };
                                                        let a56 = 512u64;
                                                        nat_mul(a55, a56)?
                                                    };
                                                    nat_sub(a57, a58)
                                                };
                                                let a59 = v4;
                                                let a60 = 512u64;
                                                if nat_lt(a59, a60) {
                                                    v4
                                                } else {
                                                    512u64
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

pub fn f1(v0: u64) -> u64 {
    let a61 = v0;
    let a62 = 3u64;
    if nat_lt(a61, a62) {
        let a63 = v0;
        let a64 = 1u64;
        if nat_lt(a63, a64) {
            184u64
        } else {
            let a65 = v0;
            let a66 = 2u64;
            if nat_lt(a65, a66) {
                10u64
            } else {
                184u64
            }
        }
    } else {
        let a67 = v0;
        let a68 = 4u64;
        if nat_lt(a67, a68) {
            512u64
        } else {
            let a69 = v0;
            let a70 = 5u64;
            if nat_lt(a69, a70) {
                165u64
            } else {
                55u64
            }
        }
    }
}

pub fn f2(v0: u64, v1: u64) -> u64 {
    let a71 = v1;
    let a72 = 5u64;
    if nat_lt(a71, a72) {
        let a73 = v1;
        let a74 = 2u64;
        if nat_lt(a73, a74) {
            let a75 = v1;
            let a76 = 1u64;
            if nat_lt(a75, a76) {
                f9(v0)
            } else {
                f10(v0)
            }
        } else {
            let a77 = v1;
            let a78 = 3u64;
            if nat_lt(a77, a78) {
                f11(v0)
            } else {
                let a79 = v1;
                let a80 = 4u64;
                if nat_lt(a79, a80) {
                    f12(v0)
                } else {
                    f3(v0, 0u64)
                }
            }
        }
    } else {
        let a81 = v1;
        let a82 = 7u64;
        if nat_lt(a81, a82) {
            let a83 = v1;
            let a84 = 6u64;
            if nat_lt(a83, a84) {
                f3(v0, 1u64)
            } else {
                f3(v0, 3u64)
            }
        } else {
            let a85 = v1;
            let a86 = 8u64;
            if nat_lt(a85, a86) {
                v0
            } else {
                let a87 = v1;
                let a88 = 9u64;
                if nat_lt(a87, a88) {
                    f13(v0)
                } else {
                    f14(v0)
                }
            }
        }
    }
}

pub fn f3(v0: u64, v1: u64) -> u64 {
    if if f15(v0) {
        let a89 = v1;
        let a90 = f16(v0);
        nat_eq(a89, a90)
    } else {
        false
    } {
        0u64
    } else {
        let a91 = v1;
        let a92 = 0u64;
        if nat_eq(a91, a92) {
            f17(v0)
        } else {
            let a93 = v1;
            let a94 = 1u64;
            if nat_eq(a93, a94) {
                let a95 = f12(v0);
                let a96 = 4u64;
                if nat_eq(a95, a96) {
                    3u64
                } else {
                    2u64
                }
            } else {
                let a97 = v1;
                let a98 = 2u64;
                if nat_eq(a97, a98) {
                    2u64
                } else {
                    1u64
                }
            }
        }
    }
}

pub fn f4(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a99 = v2;
    let a100 = 2u64;
    if nat_lt(a99, a100) {
        let a101 = v2;
        let a102 = 0u64;
        if nat_eq(a101, a102) {
            f18(v0, v1)
        } else {
            f19(v0, v1)
        }
    } else {
        let a103 = v2;
        let a104 = 2u64;
        if nat_eq(a103, a104) {
            f20(v0, v1)
        } else {
            f21(v0, v1)
        }
    }
}

pub fn f5(v0: u64, v1: u64) -> R<u64> {
    let m105 = f22(Adt0::C0(f3(v0, 0u64), f3(v0, 1u64), f3(v0, 2u64), f3(v0, 3u64), f4(v0, v1, 0u64)?, f4(v0, v1, 1u64)?, f4(v0, v1, 2u64)?, f4(v0, v1, 3u64)?, 1u64, f23(v0)))?;
    match m105 {
        Adt1::C0 => {
            Ok(0u64)
        }
        Adt1::C1(v2) => {
            Ok(v2)
        }
    }
}

pub fn f6(v0: u64) -> R<u64> {
    let a110 = {
        let a106 = f3(v0, 0u64);
        let a107 = f3(v0, 1u64);
        nat_mul(a106, a107)?
    };
    let a111 = {
        let a108 = f3(v0, 2u64);
        let a109 = f3(v0, 3u64);
        nat_mul(a108, a109)?
    };
    nat_mul(a110, a111)
}

pub fn f7(v0: u64, v1: u64) -> R<u64> {
    let a133 = {
        let a130 = f24(v0, {
            let a112 = v1;
            let a113 = f23(v0);
            let a114 = 0u64;
            nat_quot(a112, a113, a114)
        })?;
        let a131 = {
            let a118 = {
                let a115 = v1;
                let a116 = f23(v0);
                let a117 = 0u64;
                nat_rem(a115, a116, a117)
            };
            let a119 = 2u64;
            if nat_lt(a118, a119) {
                let a123 = {
                    let a120 = v1;
                    let a121 = f23(v0);
                    let a122 = 0u64;
                    nat_rem(a120, a121, a122)
                };
                let a124 = 0u64;
                if nat_eq(a123, a124) {
                    1u64
                } else {
                    256u64
                }
            } else {
                let a128 = {
                    let a125 = v1;
                    let a126 = f23(v0);
                    let a127 = 0u64;
                    nat_rem(a125, a126, a127)
                };
                let a129 = 2u64;
                if nat_eq(a128, a129) {
                    65536u64
                } else {
                    16777216u64
                }
            }
        };
        let a132 = 0u64;
        nat_quot(a130, a131, a132)
    };
    let a134 = 256u64;
    let a135 = 0u64;
    Ok(nat_rem(a133, a134, a135))
}

pub fn f8(v0: u64) -> R<u64> {
    let a138 = {
        let a136 = f5(v0, 0u64)?;
        let a137 = 511u64;
        nat_add(a136, a137)?
    };
    let a139 = 512u64;
    let a140 = 0u64;
    Ok(nat_quot(a138, a139, a140))
}

pub fn f9(v0: u64) -> u64 {
    let a141 = f11(v0);
    let a142 = 11u64;
    if nat_eq(a141, a142) {
        1u64
    } else {
        0u64
    }
}

pub fn f10(v0: u64) -> u64 {
    let a143 = v0;
    let a144 = 160u64;
    if nat_lt(a143, a144) {
        let a145 = v0;
        let a146 = 80u64;
        let a147 = 0u64;
        nat_quot(a145, a146, a147)
    } else {
        let a150 = {
            let a148 = v0;
            let a149 = 160u64;
            nat_sub(a148, a149)
        };
        let a151 = 12u64;
        let a152 = 0u64;
        nat_quot(a150, a151, a152)
    }
}

pub fn f11(v0: u64) -> u64 {
    let a153 = v0;
    let a154 = 160u64;
    if nat_lt(a153, a154) {
        let a155 = v0;
        let a156 = 16u64;
        let a157 = 0u64;
        nat_rem(a155, a156, a157)
    } else {
        16u64
    }
}

pub fn f12(v0: u64) -> u64 {
    let a158 = v0;
    let a159 = 160u64;
    if nat_lt(a158, a159) {
        let a163 = {
            let a160 = v0;
            let a161 = 80u64;
            let a162 = 0u64;
            nat_rem(a160, a161, a162)
        };
        let a164 = 16u64;
        let a165 = 0u64;
        nat_quot(a163, a164, a165)
    } else {
        0u64
    }
}

pub fn f13(v0: u64) -> u64 {
    if f15(v0) {
        0u64
    } else {
        let v1: u64 = f11(v0);
        let a166 = v1;
        let a167 = 9u64;
        if if nat_eq(a166, a167) {
            true
        } else {
            let a168 = v1;
            let a169 = 14u64;
            if nat_eq(a168, a169) {
                let a170 = f10(v0);
                let a171 = 1u64;
                nat_eq(a170, a171)
            } else {
                false
            }
        } {
            1u64
        } else {
            let a172 = v1;
            let a173 = 8u64;
            if if if nat_eq(a172, a173) {
                true
            } else {
                let a174 = v1;
                let a175 = 10u64;
                nat_eq(a174, a175)
            } {
                true
            } else {
                let a176 = v1;
                let a177 = 11u64;
                if nat_eq(a176, a177) {
                    true
                } else {
                    let a178 = v1;
                    let a179 = 13u64;
                    nat_eq(a178, a179)
                }
            } {
                2u64
            } else {
                0u64
            }
        }
    }
}

pub fn f14(v0: u64) -> u64 {
    if f15(v0) {
        let a185 = {
            let a182 = {
                let a180 = v0;
                let a181 = 160u64;
                nat_sub(a180, a181)
            };
            let a183 = 3u64;
            let a184 = 0u64;
            nat_rem(a182, a183, a184)
        };
        let a186 = 1u64;
        if nat_lt(a185, a186) {
            0u64
        } else {
            let a192 = {
                let a189 = {
                    let a187 = v0;
                    let a188 = 160u64;
                    nat_sub(a187, a188)
                };
                let a190 = 3u64;
                let a191 = 0u64;
                nat_rem(a189, a190, a191)
            };
            let a193 = 2u64;
            if nat_lt(a192, a193) {
                2143289345u64
            } else {
                3212836864u64
            }
        }
    } else {
        let a194 = f11(v0);
        let a195 = 8u64;
        if nat_lt(a194, a195) {
            let a196 = f11(v0);
            let a197 = 4u64;
            if nat_lt(a196, a197) {
                let a198 = f11(v0);
                let a199 = 2u64;
                if nat_lt(a198, a199) {
                    let a200 = f11(v0);
                    let a201 = 1u64;
                    let _ = nat_lt(a200, a201);
                    0u64
                } else {
                    let a202 = f11(v0);
                    let a203 = 3u64;
                    if nat_lt(a202, a203) {
                        0u64
                    } else {
                        2147483648u64
                    }
                }
            } else {
                let a204 = f11(v0);
                let a205 = 6u64;
                if nat_lt(a204, a205) {
                    let a206 = f11(v0);
                    let a207 = 5u64;
                    let _ = nat_lt(a206, a207);
                    0u64
                } else {
                    let a208 = f11(v0);
                    let a209 = 7u64;
                    if nat_lt(a208, a209) {
                        2139095040u64
                    } else {
                        1u64
                    }
                }
            }
        } else {
            let a210 = f11(v0);
            let a211 = 12u64;
            if nat_lt(a210, a211) {
                let a212 = f11(v0);
                let a213 = 10u64;
                if nat_lt(a212, a213) {
                    let a214 = f11(v0);
                    let a215 = 9u64;
                    if nat_lt(a214, a215) {
                        3212836864u64
                    } else {
                        0u64
                    }
                } else {
                    let a216 = f11(v0);
                    let a217 = 11u64;
                    if nat_lt(a216, a217) {
                        2143289345u64
                    } else {
                        897988541u64
                    }
                }
            } else {
                let a218 = f11(v0);
                let a219 = 14u64;
                if nat_lt(a218, a219) {
                    let a220 = f11(v0);
                    let a221 = 13u64;
                    if nat_lt(a220, a221) {
                        0u64
                    } else {
                        897988541u64
                    }
                } else {
                    let a222 = f11(v0);
                    let a223 = 15u64;
                    if nat_lt(a222, a223) {
                        897988541u64
                    } else {
                        0u64
                    }
                }
            }
        }
    }
}

pub fn f15(v0: u64) -> bool {
    let a224 = 160u64;
    let a225 = v0;
    nat_le(a224, a225)
}

pub fn f16(v0: u64) -> u64 {
    let a231 = {
        let a228 = {
            let a226 = v0;
            let a227 = 160u64;
            nat_sub(a226, a227)
        };
        let a229 = 12u64;
        let a230 = 0u64;
        nat_rem(a228, a229, a230)
    };
    let a232 = 3u64;
    let a233 = 0u64;
    nat_quot(a231, a232, a233)
}

pub fn f17(v0: u64) -> u64 {
    let a234 = f11(v0);
    let a235 = 8u64;
    if nat_lt(a234, a235) {
        let a236 = f11(v0);
        let a237 = 4u64;
        if nat_lt(a236, a237) {
            let a238 = f11(v0);
            let a239 = 2u64;
            if nat_lt(a238, a239) {
                let a240 = f11(v0);
                let a241 = 1u64;
                let _ = nat_lt(a240, a241);
                5u64
            } else {
                let a242 = f11(v0);
                let a243 = 3u64;
                if nat_lt(a242, a243) {
                    6u64
                } else {
                    7u64
                }
            }
        } else {
            let a244 = f11(v0);
            let a245 = 6u64;
            if nat_lt(a244, a245) {
                let a246 = f11(v0);
                let a247 = 5u64;
                if nat_lt(a246, a247) {
                    8u64
                } else {
                    9u64
                }
            } else {
                let a248 = f11(v0);
                let a249 = 7u64;
                if nat_lt(a248, a249) {
                    1u64
                } else {
                    3u64
                }
            }
        }
    } else {
        let a250 = f11(v0);
        let a251 = 12u64;
        if nat_lt(a250, a251) {
            let a252 = f11(v0);
            let a253 = 10u64;
            if nat_lt(a252, a253) {
                let a254 = f11(v0);
                let a255 = 9u64;
                let _ = nat_lt(a254, a255);
                5u64
            } else {
                let a256 = f11(v0);
                let a257 = 11u64;
                let _ = nat_lt(a256, a257);
                5u64
            }
        } else {
            let a258 = f11(v0);
            let a259 = 14u64;
            if nat_lt(a258, a259) {
                let a260 = f11(v0);
                let a261 = 13u64;
                let _ = nat_lt(a260, a261);
                5u64
            } else {
                let a262 = f11(v0);
                let a263 = 15u64;
                if nat_lt(a262, a263) {
                    5u64
                } else {
                    let a264 = f11(v0);
                    let a265 = 16u64;
                    if nat_lt(a264, a265) {
                        7u64
                    } else {
                        5u64
                    }
                }
            }
        }
    }
}

pub fn f18(v0: u64, v1: u64) -> R<u64> {
    let a274 = f23(v0);
    let a275 = if f25(v0, v1) {
        let a266 = f12(v0);
        let a267 = 1u64;
        if if nat_eq(a266, a267) {
            true
        } else {
            let a268 = f12(v0);
            let a269 = 2u64;
            if nat_eq(a268, a269) {
                let a270 = f10(v0);
                let a271 = 1u64;
                nat_eq(a270, a271)
            } else {
                false
            }
        } {
            2u64
        } else {
            1u64
        }
    } else {
        let a272 = f12(v0);
        let a273 = 2u64;
        if nat_eq(a272, a273) {
            2u64
        } else {
            1u64
        }
    };
    nat_mul(a274, a275)
}

pub fn f19(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f11(v0);
    let v3: u64 = {
        let a276 = f12(v0);
        let a277 = 4u64;
        if nat_eq(a276, a277) {
            100u64
        } else {
            let a278 = f12(v0);
            let a279 = 3u64;
            if nat_eq(a278, a279) {
                80u64
            } else {
                let a282 = {
                    let a280 = f18(v0, v1)?;
                    let a281 = f3(v0, 0u64);
                    nat_mul(a280, a281)?
                };
                let a283 = f23(v0);
                nat_add(a282, a283)?
            }
        }
    };
    let a284 = v2;
    let a285 = 9u64;
    if if if nat_eq(a284, a285) {
        if !f25(v0, v1) {
            true
        } else {
            let a286 = f10(v0);
            let a287 = 1u64;
            nat_eq(a286, a287)
        }
    } else {
        false
    } {
        true
    } else {
        let a288 = v2;
        let a289 = 14u64;
        if nat_eq(a288, a289) {
            f25(v0, v1)
        } else {
            false
        }
    } {
        Ok(0u64)
    } else {
        let a290 = v2;
        let a291 = 13u64;
        if if nat_eq(a290, a291) {
            if !f25(v0, v1) {
                true
            } else {
                let a292 = f10(v0);
                let a293 = 1u64;
                nat_eq(a292, a293)
            }
        } else {
            false
        } {
            let a294 = v3;
            let a295 = 1u64;
            nat_add(a294, a295)
        } else {
            Ok(v3)
        }
    }
}

pub fn f20(v0: u64, v1: u64) -> R<u64> {
    let a296 = f12(v0);
    let a297 = 4u64;
    if nat_eq(a296, a297) {
        Ok(140u64)
    } else {
        let a298 = f12(v0);
        let a299 = 3u64;
        if nat_eq(a298, a299) {
            Ok(40u64)
        } else {
            let a302 = {
                let a300 = f19(v0, v1)?;
                let a301 = f3(v0, 1u64);
                nat_mul(a300, a301)?
            };
            let a303 = f23(v0);
            nat_add(a302, a303)
        }
    }
}

pub fn f21(v0: u64, v1: u64) -> R<u64> {
    let a304 = f12(v0);
    let a305 = 4u64;
    if nat_eq(a304, a305) {
        Ok(500u64)
    } else {
        let a306 = f12(v0);
        let a307 = 3u64;
        if nat_eq(a306, a307) {
            Ok(200u64)
        } else {
            let a310 = {
                let a308 = f20(v0, v1)?;
                let a309 = f3(v0, 2u64);
                nat_mul(a308, a309)?
            };
            let a311 = f23(v0);
            nat_add(a310, a311)
        }
    }
}

pub fn f22(v0: Adt0) -> R<Adt1> {
    let a313 = match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h312, _) => {
            h312
        }
    };
    let a314 = 0u64;
    if nat_eq(a313, a314) {
        Ok(Adt1::C0)
    } else {
        let a316 = match v0.clone() {
            Adt0::C0(h315, _, _, _, _, _, _, _, _, _) => {
                h315
            }
        };
        let a317 = 0u64;
        if if if if nat_eq(a316, a317) {
            true
        } else {
            let a319 = match v0.clone() {
                Adt0::C0(_, h318, _, _, _, _, _, _, _, _) => {
                    h318
                }
            };
            let a320 = 0u64;
            nat_eq(a319, a320)
        } {
            true
        } else {
            let a322 = match v0.clone() {
                Adt0::C0(_, _, h321, _, _, _, _, _, _, _) => {
                    h321
                }
            };
            let a323 = 0u64;
            nat_eq(a322, a323)
        } {
            true
        } else {
            let a325 = match v0.clone() {
                Adt0::C0(_, _, _, h324, _, _, _, _, _, _) => {
                    h324
                }
            };
            let a326 = 0u64;
            nat_eq(a325, a326)
        } {
            Ok(Adt1::C1(0u64))
        } else {
            let a328 = match v0.clone() {
                Adt0::C0(_, _, _, _, _, _, _, _, h327, _) => {
                    h327
                }
            };
            let a329 = 1u64;
            if nat_eq(a328, a329) {
                Ok(Adt1::C1({
                    let a339 = {
                        let a337 = match v0.clone() {
                            Adt0::C0(_, _, _, _, _, _, _, _, _, h330) => {
                                h330
                            }
                        };
                        let a338 = {
                            let a335 = {
                                let a332 = match v0.clone() {
                                    Adt0::C0(h331, _, _, _, _, _, _, _, _, _) => {
                                        h331
                                    }
                                };
                                let a333 = 1u64;
                                nat_sub(a332, a333)
                            };
                            let a336 = match v0.clone() {
                                Adt0::C0(_, _, _, _, h334, _, _, _, _, _) => {
                                    h334
                                }
                            };
                            nat_mul(a335, a336)?
                        };
                        nat_add(a337, a338)?
                    };
                    let a340 = f26(v0)?;
                    nat_add(a339, a340)?
                }))
            } else {
                Ok(Adt1::C1({
                    let a349 = {
                        let a346 = {
                            let a343 = match v0.clone() {
                                Adt0::C0(h341, _, _, _, _, _, _, _, _, _) => {
                                    h341
                                }
                            };
                            let a344 = match v0.clone() {
                                Adt0::C0(_, _, _, _, h342, _, _, _, _, _) => {
                                    h342
                                }
                            };
                            nat_mul(a343, a344)?
                        };
                        let a347 = match v0.clone() {
                            Adt0::C0(_, _, _, _, _, _, _, _, h345, _) => {
                                h345
                            }
                        };
                        let a348 = 0u64;
                        nat_quot(a346, a347, a348)
                    };
                    let a350 = f26(v0)?;
                    nat_add(a349, a350)?
                }))
            }
        }
    }
}

pub fn f23(v0: u64) -> u64 {
    let a351 = f9(v0);
    let a352 = 1u64;
    if nat_eq(a351, a352) {
        2u64
    } else {
        4u64
    }
}

pub fn f24(v0: u64, v1: u64) -> R<u64> {
    let a353 = f9(v0);
    let a354 = 1u64;
    if nat_eq(a353, a354) {
        Ok(15360u64)
    } else {
        let v2: u64 = f11(v0);
        let a355 = v2;
        let a356 = 5u64;
        if if nat_eq(a355, a356) {
            let a361 = {
                let a357 = v1;
                let a358 = f23(v0);
                nat_mul(a357, a358)?
            };
            let a362 = {
                let a359 = f5(v0, 0u64)?;
                let a360 = f23(v0);
                nat_sub(a359, a360)
            };
            nat_eq(a361, a362)
        } else {
            false
        } {
            Ok(2139095041u64)
        } else {
            let a363 = v2;
            let a364 = 0u64;
            if nat_eq(a363, a364) {
                Ok(f27(v1))
            } else {
                let a365 = v2;
                let a366 = 1u64;
                if nat_eq(a365, a366) {
                    Ok(f28(v1))
                } else {
                    let a367 = v2;
                    let a368 = 2u64;
                    if nat_eq(a367, a368) {
                        Ok(0u64)
                    } else {
                        let a369 = v2;
                        let a370 = 3u64;
                        if nat_eq(a369, a370) {
                            let a374 = {
                                let a371 = v1;
                                let a372 = 2u64;
                                let a373 = 0u64;
                                nat_rem(a371, a372, a373)
                            };
                            let a375 = 0u64;
                            if nat_eq(a374, a375) {
                                Ok(0u64)
                            } else {
                                Ok(2147483648u64)
                            }
                        } else {
                            let a376 = v2;
                            let a377 = 4u64;
                            if nat_eq(a376, a377) {
                                Ok(f29(v1))
                            } else {
                                let a378 = v2;
                                let a379 = 7u64;
                                if nat_eq(a378, a379) {
                                    Ok(f30(v1))
                                } else {
                                    let a380 = v2;
                                    let a381 = 12u64;
                                    if nat_eq(a380, a381) {
                                        Ok(f31(v1))
                                    } else {
                                        let a382 = v2;
                                        let a383 = 15u64;
                                        if nat_eq(a382, a383) {
                                            Ok(1065353216u64)
                                        } else {
                                            Ok(f32(v1))
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

pub fn f25(v0: u64, v1: u64) -> bool {
    let a384 = v1;
    let a385 = 0u64;
    if nat_eq(a384, a385) {
        true
    } else {
        let a386 = v1;
        let a387 = 3u64;
        if nat_eq(a386, a387) {
            let a388 = f10(v0);
            let a389 = 1u64;
            nat_eq(a388, a389)
        } else {
            false
        }
    }
}

pub fn f26(v0: Adt0) -> R<u64> {
    let a410 = {
        let a402 = {
            let a394 = {
                let a391 = match v0.clone() {
                    Adt0::C0(_, h390, _, _, _, _, _, _, _, _) => {
                        h390
                    }
                };
                let a392 = 1u64;
                nat_sub(a391, a392)
            };
            let a395 = match v0.clone() {
                Adt0::C0(_, _, _, _, _, h393, _, _, _, _) => {
                    h393
                }
            };
            nat_mul(a394, a395)?
        };
        let a403 = {
            let a400 = {
                let a397 = match v0.clone() {
                    Adt0::C0(_, _, h396, _, _, _, _, _, _, _) => {
                        h396
                    }
                };
                let a398 = 1u64;
                nat_sub(a397, a398)
            };
            let a401 = match v0.clone() {
                Adt0::C0(_, _, _, _, _, _, h399, _, _, _) => {
                    h399
                }
            };
            nat_mul(a400, a401)?
        };
        nat_add(a402, a403)?
    };
    let a411 = {
        let a408 = {
            let a405 = match v0.clone() {
                Adt0::C0(_, _, _, h404, _, _, _, _, _, _) => {
                    h404
                }
            };
            let a406 = 1u64;
            nat_sub(a405, a406)
        };
        let a409 = match v0 {
            Adt0::C0(_, _, _, _, _, _, _, h407, _, _) => {
                h407
            }
        };
        nat_mul(a408, a409)?
    };
    nat_add(a410, a411)
}

pub fn f27(v0: u64) -> u64 {
    let a415 = {
        let a412 = v0;
        let a413 = 5u64;
        let a414 = 0u64;
        nat_rem(a412, a413, a414)
    };
    let a416 = 2u64;
    if nat_lt(a415, a416) {
        let a420 = {
            let a417 = v0;
            let a418 = 5u64;
            let a419 = 0u64;
            nat_rem(a417, a418, a419)
        };
        let a421 = 1u64;
        if nat_lt(a420, a421) {
            1266679808u64
        } else {
            1065353216u64
        }
    } else {
        let a425 = {
            let a422 = v0;
            let a423 = 5u64;
            let a424 = 0u64;
            nat_rem(a422, a423, a424)
        };
        let a426 = 3u64;
        if nat_lt(a425, a426) {
            3414163456u64
        } else {
            let a430 = {
                let a427 = v0;
                let a428 = 5u64;
                let a429 = 0u64;
                nat_rem(a427, a428, a429)
            };
            let a431 = 4u64;
            let _ = nat_lt(a430, a431);
            0u64
        }
    }
}

pub fn f28(v0: u64) -> u64 {
    let a435 = {
        let a432 = v0;
        let a433 = 5u64;
        let a434 = 0u64;
        nat_rem(a432, a433, a434)
    };
    let a436 = 2u64;
    if nat_lt(a435, a436) {
        let a440 = {
            let a437 = v0;
            let a438 = 5u64;
            let a439 = 0u64;
            nat_rem(a437, a438, a439)
        };
        let a441 = 1u64;
        if nat_lt(a440, a441) {
            1166016512u64
        } else {
            1065353216u64
        }
    } else {
        let a445 = {
            let a442 = v0;
            let a443 = 5u64;
            let a444 = 0u64;
            nat_rem(a442, a443, a444)
        };
        let a446 = 3u64;
        if nat_lt(a445, a446) {
            3313500160u64
        } else {
            let a450 = {
                let a447 = v0;
                let a448 = 5u64;
                let a449 = 0u64;
                nat_rem(a447, a448, a449)
            };
            let a451 = 4u64;
            if nat_lt(a450, a451) {
                3212836864u64
            } else {
                0u64
            }
        }
    }
}

pub fn f29(v0: u64) -> u64 {
    let a455 = {
        let a452 = v0;
        let a453 = 5u64;
        let a454 = 0u64;
        nat_rem(a452, a453, a454)
    };
    let a456 = 2u64;
    if nat_lt(a455, a456) {
        let a460 = {
            let a457 = v0;
            let a458 = 5u64;
            let a459 = 0u64;
            nat_rem(a457, a458, a459)
        };
        let a461 = 1u64;
        if nat_lt(a460, a461) {
            2139095040u64
        } else {
            4286578688u64
        }
    } else {
        let a465 = {
            let a462 = v0;
            let a463 = 5u64;
            let a464 = 0u64;
            nat_rem(a462, a463, a464)
        };
        let a466 = 3u64;
        if nat_lt(a465, a466) {
            1065353216u64
        } else {
            let a470 = {
                let a467 = v0;
                let a468 = 5u64;
                let a469 = 0u64;
                nat_rem(a467, a468, a469)
            };
            let a471 = 4u64;
            if nat_lt(a470, a471) {
                3212836864u64
            } else {
                0u64
            }
        }
    }
}

pub fn f30(v0: u64) -> u64 {
    let a475 = {
        let a472 = v0;
        let a473 = 4u64;
        let a474 = 0u64;
        nat_rem(a472, a473, a474)
    };
    let a476 = 2u64;
    if nat_lt(a475, a476) {
        let a480 = {
            let a477 = v0;
            let a478 = 4u64;
            let a479 = 0u64;
            nat_rem(a477, a478, a479)
        };
        let a481 = 1u64;
        if nat_lt(a480, a481) {
            1u64
        } else {
            2147483649u64
        }
    } else {
        let a485 = {
            let a482 = v0;
            let a483 = 4u64;
            let a484 = 0u64;
            nat_rem(a482, a483, a484)
        };
        let a486 = 3u64;
        if nat_lt(a485, a486) {
            0u64
        } else {
            2147483648u64
        }
    }
}

pub fn f31(v0: u64) -> u64 {
    let a490 = {
        let a487 = v0;
        let a488 = 5u64;
        let a489 = 0u64;
        nat_rem(a487, a488, a489)
    };
    let a491 = 2u64;
    if nat_lt(a490, a491) {
        let a495 = {
            let a492 = v0;
            let a493 = 5u64;
            let a494 = 0u64;
            nat_rem(a492, a493, a494)
        };
        let a496 = 1u64;
        if nat_lt(a495, a496) {
            2139095039u64
        } else {
            4286578687u64
        }
    } else {
        let a500 = {
            let a497 = v0;
            let a498 = 5u64;
            let a499 = 0u64;
            nat_rem(a497, a498, a499)
        };
        let a501 = 3u64;
        if nat_lt(a500, a501) {
            2139095039u64
        } else {
            let a505 = {
                let a502 = v0;
                let a503 = 5u64;
                let a504 = 0u64;
                nat_rem(a502, a503, a504)
            };
            let a506 = 4u64;
            if nat_lt(a505, a506) {
                4286578687u64
            } else {
                1065353216u64
            }
        }
    }
}

pub fn f32(v0: u64) -> u64 {
    let a510 = {
        let a507 = v0;
        let a508 = 8u64;
        let a509 = 0u64;
        nat_rem(a507, a508, a509)
    };
    let a511 = 4u64;
    if nat_lt(a510, a511) {
        let a515 = {
            let a512 = v0;
            let a513 = 8u64;
            let a514 = 0u64;
            nat_rem(a512, a513, a514)
        };
        let a516 = 2u64;
        if nat_lt(a515, a516) {
            let a520 = {
                let a517 = v0;
                let a518 = 8u64;
                let a519 = 0u64;
                nat_rem(a517, a518, a519)
            };
            let a521 = 1u64;
            if nat_lt(a520, a521) {
                3229614080u64
            } else {
                3221225472u64
            }
        } else {
            let a525 = {
                let a522 = v0;
                let a523 = 8u64;
                let a524 = 0u64;
                nat_rem(a522, a523, a524)
            };
            let a526 = 3u64;
            if nat_lt(a525, a526) {
                3212836864u64
            } else {
                2147483648u64
            }
        }
    } else {
        let a530 = {
            let a527 = v0;
            let a528 = 8u64;
            let a529 = 0u64;
            nat_rem(a527, a528, a529)
        };
        let a531 = 6u64;
        if nat_lt(a530, a531) {
            let a535 = {
                let a532 = v0;
                let a533 = 8u64;
                let a534 = 0u64;
                nat_rem(a532, a533, a534)
            };
            let a536 = 5u64;
            if nat_lt(a535, a536) {
                0u64
            } else {
                1065353216u64
            }
        } else {
            let a540 = {
                let a537 = v0;
                let a538 = 8u64;
                let a539 = 0u64;
                nat_rem(a537, a538, a539)
            };
            let a541 = 7u64;
            if nat_lt(a540, a541) {
                1073741824u64
            } else {
                1082130432u64
            }
        }
    }
}

pub fn run(p0: u64, p1: u64, p2: u64, p3: u64) -> R<Option<u64>> {
    f0(p0, p1, p2, p3)
}
