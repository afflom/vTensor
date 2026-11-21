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
    C0(u64),
    C1(u64, u64),
    C2(u64, u64, u64),
    C3(u64, u64, u64),
}

pub fn f0(v0: u64, v1: Adt0) -> R<Option<u64>> {
    let a1 = v0;
    let a2 = 0u64;
    if nat_eq(a1, a2) {
        let m3 = v1;
        match m3 {
            Adt0::C0(v2) => {
                let a4 = v2;
                let a5 = 4u64;
                if nat_lt(a4, a5) {
                    Ok(Some({
                        let a6 = v2;
                        let a7 = 0u64;
                        if nat_eq(a6, a7) {
                            1280u64
                        } else {
                            let a8 = v2;
                            let a9 = 1u64;
                            if nat_eq(a8, a9) {
                                24u64
                            } else {
                                let a10 = v2;
                                let a11 = 2u64;
                                if nat_eq(a10, a11) {
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
            Adt0::C1(v3, v4) => {
                let a12 = v3;
                let a13 = 1280u64;
                if if nat_lt(a12, a13) {
                    let a14 = v4;
                    let a15 = 24u64;
                    nat_lt(a14, a15)
                } else {
                    false
                } {
                    Ok(Some(f1(v3, v4)?))
                } else {
                    Ok(None::<u64>)
                }
            }
            Adt0::C2(v5, v6, v7) => {
                let a16 = v5;
                let a17 = 1280u64;
                if if if nat_lt(a16, a17) {
                    let a18 = v6;
                    let a19 = 2u64;
                    nat_lt(a18, a19)
                } else {
                    false
                } {
                    let a20 = v7;
                    let a21 = 2u64;
                    nat_lt(a20, a21)
                } else {
                    false
                } {
                    Ok(Some({
                        let a22 = v7;
                        let a23 = 0u64;
                        if nat_eq(a22, a23) {
                            f2(v5, v6)?
                        } else {
                            4u64
                        }
                    }))
                } else {
                    Ok(None::<u64>)
                }
            }
            Adt0::C3(v8, v9, v10) => {
                let a24 = v8;
                let a25 = 1280u64;
                if if if nat_lt(a24, a25) {
                    let a26 = v9;
                    let a27 = 2u64;
                    nat_lt(a26, a27)
                } else {
                    false
                } {
                    let a28 = v10;
                    let a29 = f2(v8, v9)?;
                    nat_lt(a28, a29)
                } else {
                    false
                } {
                    Ok(Some(f3(v8, v9, v10)?))
                } else {
                    Ok(None::<u64>)
                }
            }
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f1(v0: u64, v1: u64) -> R<u64> {
    let a30 = v1;
    let a31 = 0u64;
    if nat_eq(a30, a31) {
        Ok(f4(v0))
    } else {
        let a32 = v1;
        let a33 = 1u64;
        if nat_eq(a32, a33) {
            Ok(f5(v0))
        } else {
            let a34 = v1;
            let a35 = 2u64;
            if nat_eq(a34, a35) {
                Ok(f6(v0))
            } else {
                let a36 = v1;
                let a37 = 3u64;
                if nat_eq(a36, a37) {
                    Ok(f7(v0))
                } else {
                    let a38 = v1;
                    let a39 = 4u64;
                    if nat_eq(a38, a39) {
                        Ok(f8(v0, 0u64))
                    } else {
                        let a40 = v1;
                        let a41 = 5u64;
                        if nat_eq(a40, a41) {
                            Ok(f9(v0, 0u64))
                        } else {
                            let a42 = v1;
                            let a43 = 6u64;
                            if nat_eq(a42, a43) {
                                Ok(f8(v0, 1u64))
                            } else {
                                let a44 = v1;
                                let a45 = 7u64;
                                if nat_eq(a44, a45) {
                                    Ok(f9(v0, 1u64))
                                } else {
                                    let a46 = v1;
                                    let a47 = 16u64;
                                    if nat_lt(a46, a47) {
                                        f10(v0, {
                                            let a50 = {
                                                let a48 = v1;
                                                let a49 = 8u64;
                                                nat_sub(a48, a49)
                                            };
                                            let a51 = 4u64;
                                            let a52 = 0u64;
                                            nat_quot(a50, a51, a52)
                                        }, {
                                            let a55 = {
                                                let a53 = v1;
                                                let a54 = 8u64;
                                                nat_sub(a53, a54)
                                            };
                                            let a56 = 4u64;
                                            let a57 = 0u64;
                                            nat_rem(a55, a56, a57)
                                        })
                                    } else {
                                        let a58 = v1;
                                        let a59 = 16u64;
                                        if nat_eq(a58, a59) {
                                            Ok(f11(v0))
                                        } else {
                                            let a60 = v1;
                                            let a61 = 17u64;
                                            if nat_eq(a60, a61) {
                                                f12(v0)
                                            } else {
                                                let a62 = v1;
                                                let a63 = 18u64;
                                                if nat_eq(a62, a63) {
                                                    f13(v0)
                                                } else {
                                                    let a64 = v1;
                                                    let a65 = 19u64;
                                                    if nat_eq(a64, a65) {
                                                        f14(v0)
                                                    } else {
                                                        let a66 = v1;
                                                        let a67 = 20u64;
                                                        if nat_eq(a66, a67) {
                                                            f2(v0, 0u64)
                                                        } else {
                                                            let a68 = v1;
                                                            let a69 = 21u64;
                                                            if nat_eq(a68, a69) {
                                                                f2(v0, 1u64)
                                                            } else {
                                                                let a70 = v1;
                                                                let a71 = 22u64;
                                                                if nat_eq(a70, a71) {
                                                                    let a72 = v0;
                                                                    let a73 = 416u64;
                                                                    if nat_eq(a72, a73) {
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

pub fn f2(v0: u64, v1: u64) -> R<u64> {
    let a78 = {
        let a76 = {
            let a74 = f6(v0);
            let a75 = f15(v0, v1);
            nat_mul(a74, a75)?
        };
        let a77 = f8(v0, v1);
        nat_mul(a76, a77)?
    };
    let a79 = f9(v0, v1);
    nat_mul(a78, a79)
}

pub fn f3(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a80 = v2;
        let a81 = f6(v0);
        let a82 = 0u64;
        nat_rem(a80, a81, a82)
    };
    let v4: u64 = {
        let a83 = v2;
        let a84 = f6(v0);
        let a85 = 0u64;
        nat_quot(a83, a84, a85)
    };
    let v5: u64 = {
        let a86 = v4;
        let a87 = f15(v0, v1);
        let a88 = 0u64;
        nat_rem(a86, a87, a88)
    };
    let v6: u64 = {
        let a89 = v4;
        let a90 = f15(v0, v1);
        let a91 = 0u64;
        nat_quot(a89, a90, a91)
    };
    let v7: u64 = {
        let a92 = v6;
        let a93 = f8(v0, v1);
        let a94 = 0u64;
        nat_rem(a92, a93, a94)
    };
    let v8: u64 = {
        let a95 = v6;
        let a96 = f8(v0, v1);
        let a97 = 0u64;
        nat_quot(a95, a96, a97)
    };
    let a98 = f4(v0);
    let a99 = 1u64;
    if if if nat_eq(a98, a99) {
        let a100 = f5(v0);
        let a101 = 1u64;
        nat_eq(a100, a101)
    } else {
        false
    } {
        let a102 = f6(v0);
        let a103 = 1u64;
        nat_eq(a102, a103)
    } else {
        false
    } {
        f16(v0, v1)
    } else {
        Ok(f17({
            let a122 = {
                let a120 = {
                    let a116 = {
                        let a112 = {
                            let a108 = {
                                let a104 = v3;
                                let a105 = 7u64;
                                nat_mul(a104, a105)?
                            };
                            let a109 = {
                                let a106 = v5;
                                let a107 = 11u64;
                                nat_mul(a106, a107)?
                            };
                            nat_add(a108, a109)?
                        };
                        let a113 = {
                            let a110 = v7;
                            let a111 = 3u64;
                            nat_mul(a110, a111)?
                        };
                        nat_add(a112, a113)?
                    };
                    let a117 = {
                        let a114 = v8;
                        let a115 = 5u64;
                        nat_mul(a114, a115)?
                    };
                    nat_add(a116, a117)?
                };
                let a121 = {
                    let a118 = v1;
                    let a119 = 0u64;
                    if nat_eq(a118, a119) {
                        1u64
                    } else {
                        2u64
                    }
                };
                nat_add(a120, a121)?
            };
            let a123 = 19u64;
            let a124 = 0u64;
            nat_rem(a122, a123, a124)
        }))
    }
}

pub fn f4(v0: u64) -> u64 {
    let a131 = {
        let a128 = {
            let a125 = v0;
            let a126 = 320u64;
            let a127 = 0u64;
            nat_quot(a125, a126, a127)
        };
        let a129 = 4u64;
        let a130 = 0u64;
        nat_rem(a128, a129, a130)
    };
    let a132 = 0u64;
    if nat_eq(a131, a132) {
        0u64
    } else {
        let a139 = {
            let a136 = {
                let a133 = v0;
                let a134 = 320u64;
                let a135 = 0u64;
                nat_quot(a133, a134, a135)
            };
            let a137 = 4u64;
            let a138 = 0u64;
            nat_rem(a136, a137, a138)
        };
        let a140 = 1u64;
        if nat_eq(a139, a140) {
            1u64
        } else {
            let a147 = {
                let a144 = {
                    let a141 = v0;
                    let a142 = 320u64;
                    let a143 = 0u64;
                    nat_quot(a141, a142, a143)
                };
                let a145 = 4u64;
                let a146 = 0u64;
                nat_rem(a144, a145, a146)
            };
            let a148 = 2u64;
            if nat_eq(a147, a148) {
                2u64
            } else {
                5u64
            }
        }
    }
}

pub fn f5(v0: u64) -> u64 {
    let a155 = {
        let a152 = {
            let a149 = v0;
            let a150 = 80u64;
            let a151 = 0u64;
            nat_quot(a149, a150, a151)
        };
        let a153 = 4u64;
        let a154 = 0u64;
        nat_rem(a152, a153, a154)
    };
    let a156 = 0u64;
    if nat_eq(a155, a156) {
        0u64
    } else {
        let a163 = {
            let a160 = {
                let a157 = v0;
                let a158 = 80u64;
                let a159 = 0u64;
                nat_quot(a157, a158, a159)
            };
            let a161 = 4u64;
            let a162 = 0u64;
            nat_rem(a160, a161, a162)
        };
        let a164 = 1u64;
        if nat_eq(a163, a164) {
            1u64
        } else {
            let a171 = {
                let a168 = {
                    let a165 = v0;
                    let a166 = 80u64;
                    let a167 = 0u64;
                    nat_quot(a165, a166, a167)
                };
                let a169 = 4u64;
                let a170 = 0u64;
                nat_rem(a168, a169, a170)
            };
            let a172 = 2u64;
            if nat_eq(a171, a172) {
                3u64
            } else {
                7u64
            }
        }
    }
}

pub fn f6(v0: u64) -> u64 {
    let a179 = {
        let a176 = {
            let a173 = v0;
            let a174 = 16u64;
            let a175 = 0u64;
            nat_quot(a173, a174, a175)
        };
        let a177 = 5u64;
        let a178 = 0u64;
        nat_rem(a176, a177, a178)
    };
    let a180 = 0u64;
    if nat_eq(a179, a180) {
        0u64
    } else {
        let a187 = {
            let a184 = {
                let a181 = v0;
                let a182 = 16u64;
                let a183 = 0u64;
                nat_quot(a181, a182, a183)
            };
            let a185 = 5u64;
            let a186 = 0u64;
            nat_rem(a184, a185, a186)
        };
        let a188 = 1u64;
        if nat_eq(a187, a188) {
            1u64
        } else {
            let a195 = {
                let a192 = {
                    let a189 = v0;
                    let a190 = 16u64;
                    let a191 = 0u64;
                    nat_quot(a189, a190, a191)
                };
                let a193 = 5u64;
                let a194 = 0u64;
                nat_rem(a192, a193, a194)
            };
            let a196 = 2u64;
            if nat_eq(a195, a196) {
                2u64
            } else {
                let a203 = {
                    let a200 = {
                        let a197 = v0;
                        let a198 = 16u64;
                        let a199 = 0u64;
                        nat_quot(a197, a198, a199)
                    };
                    let a201 = 5u64;
                    let a202 = 0u64;
                    nat_rem(a200, a201, a202)
                };
                let a204 = 3u64;
                if nat_eq(a203, a204) {
                    7u64
                } else {
                    33u64
                }
            }
        }
    }
}

pub fn f7(v0: u64) -> u64 {
    let a208 = {
        let a205 = v0;
        let a206 = 4u64;
        let a207 = 0u64;
        nat_quot(a205, a206, a207)
    };
    let a209 = 4u64;
    let a210 = 0u64;
    nat_rem(a208, a209, a210)
}

pub fn f8(v0: u64, v1: u64) -> u64 {
    let a211 = v1;
    let a212 = 0u64;
    if nat_eq(a211, a212) {
        let a213 = 2u64;
        let a214 = f18(v0);
        if nat_le(a213, a214) {
            2u64
        } else {
            1u64
        }
    } else {
        let a215 = f18(v0);
        let a216 = 0u64;
        if nat_eq(a215, a216) {
            1u64
        } else {
            let a217 = f18(v0);
            let a218 = 1u64;
            if nat_eq(a217, a218) {
                3u64
            } else {
                4u64
            }
        }
    }
}

pub fn f9(v0: u64, v1: u64) -> u64 {
    let a219 = v1;
    let a220 = 0u64;
    if nat_eq(a219, a220) {
        let a221 = f18(v0);
        let a222 = 3u64;
        if nat_eq(a221, a222) {
            2u64
        } else {
            1u64
        }
    } else {
        let a223 = f18(v0);
        let a224 = 0u64;
        if nat_eq(a223, a224) {
            1u64
        } else {
            let a225 = f18(v0);
            let a226 = 3u64;
            if nat_eq(a225, a226) {
                4u64
            } else {
                2u64
            }
        }
    }
}

pub fn f10(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a227 = v2;
    let a228 = 0u64;
    if nat_eq(a227, a228) {
        Ok(4u64)
    } else {
        let a229 = v2;
        let a230 = 1u64;
        if nat_eq(a229, a230) {
            f19(v0)
        } else {
            let a231 = v2;
            let a232 = 2u64;
            if nat_eq(a231, a232) {
                f20(v0, v1)
            } else {
                f21(v0, v1)
            }
        }
    }
}

pub fn f11(v0: u64) -> u64 {
    let a233 = 2u64;
    let a234 = f7(v0);
    if nat_le(a233, a234) {
        8u64
    } else {
        4u64
    }
}

pub fn f12(v0: u64) -> R<u64> {
    let a237 = {
        let a235 = f11(v0);
        let a236 = f4(v0);
        nat_mul(a235, a236)?
    };
    let a238 = f22(v0);
    nat_add(a237, a238)
}

pub fn f13(v0: u64) -> R<u64> {
    let a241 = {
        let a239 = f12(v0)?;
        let a240 = f5(v0);
        nat_mul(a239, a240)?
    };
    let a242 = f22(v0);
    nat_add(a241, a242)
}

pub fn f14(v0: u64) -> R<u64> {
    let a245 = {
        let a243 = f13(v0)?;
        let a244 = f8(v0, 1u64);
        nat_mul(a243, a244)?
    };
    let a246 = f22(v0);
    nat_add(a245, a246)
}

pub fn f15(v0: u64, v1: u64) -> u64 {
    let a247 = v1;
    let a248 = 0u64;
    if nat_eq(a247, a248) {
        f4(v0)
    } else {
        f5(v0)
    }
}

pub fn f16(v0: u64, v1: u64) -> R<u64> {
    let a256 = {
        let a254 = {
            let a252 = {
                let a249 = f7(v0);
                let a250 = 2u64;
                let a251 = 0u64;
                nat_rem(a249, a250, a251)
            };
            let a253 = 4u64;
            nat_mul(a252, a253)?
        };
        let a255 = f18(v0);
        nat_add(a254, a255)?
    };
    let a257 = 0u64;
    if nat_eq(a256, a257) {
        let a258 = v1;
        let a259 = 0u64;
        if nat_eq(a258, a259) {
            Ok(2139095040u64)
        } else {
            Ok(0u64)
        }
    } else {
        let a267 = {
            let a265 = {
                let a263 = {
                    let a260 = f7(v0);
                    let a261 = 2u64;
                    let a262 = 0u64;
                    nat_rem(a260, a261, a262)
                };
                let a264 = 4u64;
                nat_mul(a263, a264)?
            };
            let a266 = f18(v0);
            nat_add(a265, a266)?
        };
        let a268 = 1u64;
        if nat_eq(a267, a268) {
            let a269 = v1;
            let a270 = 0u64;
            if nat_eq(a269, a270) {
                Ok(2139095040u64)
            } else {
                Ok(3212836864u64)
            }
        } else {
            let a278 = {
                let a276 = {
                    let a274 = {
                        let a271 = f7(v0);
                        let a272 = 2u64;
                        let a273 = 0u64;
                        nat_rem(a271, a272, a273)
                    };
                    let a275 = 4u64;
                    nat_mul(a274, a275)?
                };
                let a277 = f18(v0);
                nat_add(a276, a277)?
            };
            let a279 = 2u64;
            if nat_eq(a278, a279) {
                let a280 = v1;
                let a281 = 0u64;
                if nat_eq(a280, a281) {
                    Ok(2143289344u64)
                } else {
                    Ok(1065353216u64)
                }
            } else {
                let a289 = {
                    let a287 = {
                        let a285 = {
                            let a282 = f7(v0);
                            let a283 = 2u64;
                            let a284 = 0u64;
                            nat_rem(a282, a283, a284)
                        };
                        let a286 = 4u64;
                        nat_mul(a285, a286)?
                    };
                    let a288 = f18(v0);
                    nat_add(a287, a288)?
                };
                let a290 = 3u64;
                if nat_eq(a289, a290) {
                    let a291 = v1;
                    let a292 = 0u64;
                    if nat_eq(a291, a292) {
                        Ok(2147483648u64)
                    } else {
                        Ok(1065353216u64)
                    }
                } else {
                    let a300 = {
                        let a298 = {
                            let a296 = {
                                let a293 = f7(v0);
                                let a294 = 2u64;
                                let a295 = 0u64;
                                nat_rem(a293, a294, a295)
                            };
                            let a297 = 4u64;
                            nat_mul(a296, a297)?
                        };
                        let a299 = f18(v0);
                        nat_add(a298, a299)?
                    };
                    let a301 = 4u64;
                    if nat_eq(a300, a301) {
                        let a302 = v1;
                        let a303 = 0u64;
                        if nat_eq(a302, a303) {
                            Ok(1u64)
                        } else {
                            Ok(1065353216u64)
                        }
                    } else {
                        let a311 = {
                            let a309 = {
                                let a307 = {
                                    let a304 = f7(v0);
                                    let a305 = 2u64;
                                    let a306 = 0u64;
                                    nat_rem(a304, a305, a306)
                                };
                                let a308 = 4u64;
                                nat_mul(a307, a308)?
                            };
                            let a310 = f18(v0);
                            nat_add(a309, a310)?
                        };
                        let a312 = 5u64;
                        if nat_eq(a311, a312) {
                            let a313 = v1;
                            let a314 = 0u64;
                            if nat_eq(a313, a314) {
                                Ok(8388608u64)
                            } else {
                                Ok(1056964608u64)
                            }
                        } else {
                            let a322 = {
                                let a320 = {
                                    let a318 = {
                                        let a315 = f7(v0);
                                        let a316 = 2u64;
                                        let a317 = 0u64;
                                        nat_rem(a315, a316, a317)
                                    };
                                    let a319 = 4u64;
                                    nat_mul(a318, a319)?
                                };
                                let a321 = f18(v0);
                                nat_add(a320, a321)?
                            };
                            let a323 = 6u64;
                            if nat_eq(a322, a323) {
                                let a324 = v1;
                                let a325 = 0u64;
                                if nat_eq(a324, a325) {
                                    Ok(2139095039u64)
                                } else {
                                    Ok(1073741824u64)
                                }
                            } else {
                                let a326 = v1;
                                let a327 = 0u64;
                                let _ = nat_eq(a326, a327);
                                Ok(4286578688u64)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f17(v0: u64) -> u64 {
    let a328 = v0;
    let a329 = 0u64;
    if nat_eq(a328, a329) {
        3222274048u64
    } else {
        let a330 = v0;
        let a331 = 1u64;
        if nat_eq(a330, a331) {
            3221225472u64
        } else {
            let a332 = v0;
            let a333 = 2u64;
            if nat_eq(a332, a333) {
                3219128320u64
            } else {
                let a334 = v0;
                let a335 = 3u64;
                if nat_eq(a334, a335) {
                    3217031168u64
                } else {
                    let a336 = v0;
                    let a337 = 4u64;
                    if nat_eq(a336, a337) {
                        3214934016u64
                    } else {
                        let a338 = v0;
                        let a339 = 5u64;
                        if nat_eq(a338, a339) {
                            3212836864u64
                        } else {
                            let a340 = v0;
                            let a341 = 6u64;
                            if nat_eq(a340, a341) {
                                3208642560u64
                            } else {
                                let a342 = v0;
                                let a343 = 7u64;
                                if nat_eq(a342, a343) {
                                    3204448256u64
                                } else {
                                    let a344 = v0;
                                    let a345 = 8u64;
                                    if nat_eq(a344, a345) {
                                        3196059648u64
                                    } else {
                                        let a346 = v0;
                                        let a347 = 9u64;
                                        if nat_eq(a346, a347) {
                                            0u64
                                        } else {
                                            let a348 = v0;
                                            let a349 = 10u64;
                                            if nat_eq(a348, a349) {
                                                1048576000u64
                                            } else {
                                                let a350 = v0;
                                                let a351 = 11u64;
                                                if nat_eq(a350, a351) {
                                                    1056964608u64
                                                } else {
                                                    let a352 = v0;
                                                    let a353 = 12u64;
                                                    if nat_eq(a352, a353) {
                                                        1061158912u64
                                                    } else {
                                                        let a354 = v0;
                                                        let a355 = 13u64;
                                                        if nat_eq(a354, a355) {
                                                            1065353216u64
                                                        } else {
                                                            let a356 = v0;
                                                            let a357 = 14u64;
                                                            if nat_eq(a356, a357) {
                                                                1067450368u64
                                                            } else {
                                                                let a358 = v0;
                                                                let a359 = 15u64;
                                                                if nat_eq(a358, a359) {
                                                                    1069547520u64
                                                                } else {
                                                                    let a360 = v0;
                                                                    let a361 = 16u64;
                                                                    if nat_eq(a360, a361) {
                                                                        1071644672u64
                                                                    } else {
                                                                        let a362 = v0;
                                                                        let a363 = 17u64;
                                                                        if nat_eq(a362, a363) {
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

pub fn f18(v0: u64) -> u64 {
    let a364 = v0;
    let a365 = 4u64;
    let a366 = 0u64;
    nat_rem(a364, a365, a366)
}

pub fn f19(v0: u64) -> R<u64> {
    let a374 = {
        let a372 = {
            let a367 = f6(v0);
            let a368 = 0u64;
            if nat_eq(a367, a368) {
                1u64
            } else {
                f6(v0)
            }
        };
        let a373 = {
            let a369 = f7(v0);
            let a370 = 2u64;
            let a371 = 0u64;
            nat_rem(a369, a370, a371)
        };
        nat_add(a372, a373)?
    };
    let a375 = 4u64;
    nat_mul(a374, a375)
}

pub fn f20(v0: u64, v1: u64) -> R<u64> {
    let a383 = {
        let a376 = f19(v0)?;
        let a377 = f15(v0, v1);
        nat_mul(a376, a377)?
    };
    let a384 = {
        let a381 = {
            let a378 = f7(v0);
            let a379 = 2u64;
            let a380 = 0u64;
            nat_rem(a378, a379, a380)
        };
        let a382 = 4u64;
        nat_mul(a381, a382)?
    };
    nat_add(a383, a384)
}

pub fn f21(v0: u64, v1: u64) -> R<u64> {
    let a392 = {
        let a385 = f20(v0, v1)?;
        let a386 = f8(v0, v1);
        nat_mul(a385, a386)?
    };
    let a393 = {
        let a390 = {
            let a387 = f7(v0);
            let a388 = 2u64;
            let a389 = 0u64;
            nat_rem(a387, a388, a389)
        };
        let a391 = 4u64;
        nat_mul(a390, a391)?
    };
    nat_add(a392, a393)
}

pub fn f22(v0: u64) -> u64 {
    let a394 = 2u64;
    let a395 = f7(v0);
    if nat_le(a394, a395) {
        4u64
    } else {
        0u64
    }
}
