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

#[derive(Clone)]
pub enum Adt2 {
    C0,
    C1(Adt3),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64, u64, u64),
}

pub fn f0(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a1 = 31u64;
    let a2 = v0;
    if nat_le(a1, a2) {
        f1(1u64, {
            let a3 = v0;
            let a4 = 31u64;
            nat_sub(a3, a4)
        }, v1, v2, v3)
    } else {
        let a5 = 24u64;
        let a6 = v0;
        if nat_le(a5, a6) {
            f1(0u64, {
                let a7 = v0;
                let a8 = 24u64;
                nat_sub(a7, a8)
            }, v1, v2, v3)
        } else {
            let a9 = 17u64;
            let a10 = v0;
            if nat_le(a9, a10) {
                f2({
                    let a11 = v0;
                    let a12 = 17u64;
                    nat_sub(a11, a12)
                }, v1, v2, v3)
            } else {
                let a13 = 11u64;
                let a14 = v0;
                if nat_le(a13, a14) {
                    f3({
                        let a15 = v0;
                        let a16 = 11u64;
                        nat_sub(a15, a16)
                    }, v1, v2, v3)
                } else {
                    let a17 = 5u64;
                    let a18 = v0;
                    if nat_le(a17, a18) {
                        f4({
                            let a19 = v0;
                            let a20 = 5u64;
                            nat_sub(a19, a20)
                        }, v1, v2, v3)
                    } else {
                        let a21 = v0;
                        let a22 = 2u64;
                        if nat_lt(a21, a22) {
                            let a23 = v0;
                            let a24 = 0u64;
                            if nat_eq(a23, a24) {
                                let a25 = v3;
                                let a26 = 4u64;
                                if nat_lt(a25, a26) {
                                    Ok(Some({
                                        let a27 = v3;
                                        let a28 = 2u64;
                                        if nat_lt(a27, a28) {
                                            let a29 = v3;
                                            let a30 = 1u64;
                                            if nat_lt(a29, a30) {
                                                120u64
                                            } else {
                                                24u64
                                            }
                                        } else {
                                            let a31 = v3;
                                            let a32 = 3u64;
                                            if nat_lt(a31, a32) {
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
                                let a33 = v1;
                                let a34 = 120u64;
                                if if nat_lt(a33, a34) {
                                    let a35 = v3;
                                    let a36 = 24u64;
                                    nat_lt(a35, a36)
                                } else {
                                    false
                                } {
                                    Ok(Some(f5(v1, v3)?))
                                } else {
                                    Ok(None::<u64>)
                                }
                            }
                        } else {
                            let a37 = v0;
                            let a38 = 4u64;
                            if if nat_lt(a37, a38) {
                                let a39 = v1;
                                let a40 = 120u64;
                                if nat_lt(a39, a40) {
                                    let a41 = v2;
                                    let a42 = 2u64;
                                    nat_lt(a41, a42)
                                } else {
                                    false
                                }
                            } else {
                                false
                            } {
                                let a43 = v0;
                                let a44 = 2u64;
                                if nat_eq(a43, a44) {
                                    let a45 = v3;
                                    let a46 = 2u64;
                                    if nat_lt(a45, a46) {
                                        Ok(Some({
                                            let a47 = v3;
                                            let a48 = 0u64;
                                            if nat_eq(a47, a48) {
                                                f6(v1, v2)?
                                            } else {
                                                4u64
                                            }
                                        }))
                                    } else {
                                        Ok(None::<u64>)
                                    }
                                } else {
                                    let a49 = v3;
                                    let a50 = f6(v1, v2)?;
                                    if nat_lt(a49, a50) {
                                        Ok(Some(f7(v2, v3)?))
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

pub fn f1(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Option<u64>> {
    let a51 = v1;
    let a52 = 0u64;
    if nat_eq(a51, a52) {
        let a53 = v2;
        let a54 = 0u64;
        if if if nat_eq(a53, a54) {
            let a55 = v3;
            let a56 = 0u64;
            nat_eq(a55, a56)
        } else {
            false
        } {
            let a57 = v4;
            let a58 = 5u64;
            nat_lt(a57, a58)
        } else {
            false
        } {
            Ok(Some({
                let a59 = v4;
                let a60 = 2u64;
                if nat_lt(a59, a60) {
                    let a61 = v4;
                    let a62 = 1u64;
                    if nat_lt(a61, a62) {
                        f8(v0)
                    } else {
                        10u64
                    }
                } else {
                    let a63 = v4;
                    let a64 = 3u64;
                    if nat_lt(a63, a64) {
                        let a65 = v0;
                        let a66 = 0u64;
                        if nat_eq(a65, a66) {
                            8u64
                        } else {
                            3u64
                        }
                    } else {
                        let a67 = v4;
                        let a68 = 4u64;
                        if nat_lt(a67, a68) {
                            f9(v0)
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
        let a69 = v2;
        let a70 = f8(v0);
        if nat_lt(a69, a70) {
            let a71 = v1;
            let a72 = 1u64;
            if nat_eq(a71, a72) {
                let a73 = v3;
                let a74 = 0u64;
                if if nat_eq(a73, a74) {
                    let a75 = v4;
                    let a76 = 10u64;
                    nat_lt(a75, a76)
                } else {
                    false
                } {
                    Ok(Some(f10(v0, v2, v4)?))
                } else {
                    Ok(None::<u64>)
                }
            } else {
                let a77 = v1;
                let a78 = 2u64;
                if nat_eq(a77, a78) {
                    let a79 = v3;
                    let a80 = 2u64;
                    if if if nat_lt(a79, a80) {
                        true
                    } else {
                        let a81 = v3;
                        let a82 = 3u64;
                        nat_eq(a81, a82)
                    } {
                        let a83 = v4;
                        let a84 = 10u64;
                        nat_lt(a83, a84)
                    } else {
                        false
                    } {
                        Ok(Some({
                            let a85 = v4;
                            let a86 = 4u64;
                            if nat_lt(a85, a86) {
                                f11(v0, v2, v3, v4)?
                            } else {
                                let a87 = v4;
                                let a88 = 8u64;
                                if nat_lt(a87, a88) {
                                    f12(v0, v2, v3, {
                                        let a89 = v4;
                                        let a90 = 4u64;
                                        nat_sub(a89, a90)
                                    })?
                                } else {
                                    let a91 = v4;
                                    let a92 = 8u64;
                                    if nat_eq(a91, a92) {
                                        f13(v0, v2, v3)?
                                    } else {
                                        f14(v0, v2, v3)?
                                    }
                                }
                            }
                        }))
                    } else {
                        Ok(None::<u64>)
                    }
                } else {
                    let a93 = v1;
                    let a94 = 3u64;
                    if nat_eq(a93, a94) {
                        let a95 = v3;
                        let a96 = 0u64;
                        if if nat_eq(a95, a96) {
                            let a97 = v4;
                            let a98 = f9(v0);
                            nat_lt(a97, a98)
                        } else {
                            false
                        } {
                            Ok(Some(f15(v0, v2, v4)?))
                        } else {
                            Ok(None::<u64>)
                        }
                    } else {
                        let a99 = v1;
                        let a100 = 4u64;
                        if nat_eq(a99, a100) {
                            let a101 = v4;
                            let a102 = 2u64;
                            if nat_lt(a101, a102) {
                                let a103 = v3;
                                let a104 = 0u64;
                                if nat_eq(a103, a104) {
                                    let a105 = v4;
                                    let a106 = 0u64;
                                    if nat_eq(a105, a106) {
                                        Ok(Some(f16(v0, v2)?))
                                    } else {
                                        Ok(None::<u64>)
                                    }
                                } else {
                                    let a107 = v3;
                                    let a108 = f16(v0, v2)?;
                                    if nat_le(a107, a108) {
                                        Ok(Some({
                                            let v5: u64 = f9(v0);
                                            let v6: u64 = {
                                                let a111 = {
                                                    let a109 = v3;
                                                    let a110 = 1u64;
                                                    nat_sub(a109, a110)
                                                };
                                                let a112 = v5;
                                                nat_mul(a111, a112)?
                                            };
                                            let v7: u64 = {
                                                let a113 = f13(v0, v2, 0u64)?;
                                                let a114 = v6;
                                                nat_sub(a113, a114)
                                            };
                                            let a115 = v4;
                                            let a116 = 0u64;
                                            if nat_eq(a115, a116) {
                                                v6
                                            } else {
                                                let a117 = v7;
                                                let a118 = v5;
                                                if nat_lt(a117, a118) {
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

pub fn f2(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a119 = v0;
    let a120 = 0u64;
    if nat_eq(a119, a120) {
        let a121 = v1;
        let a122 = 0u64;
        if if if nat_eq(a121, a122) {
            let a123 = v2;
            let a124 = 0u64;
            nat_eq(a123, a124)
        } else {
            false
        } {
            let a125 = v3;
            let a126 = 5u64;
            nat_lt(a125, a126)
        } else {
            false
        } {
            Ok(Some({
                let a127 = v3;
                let a128 = 2u64;
                if nat_lt(a127, a128) {
                    let a129 = v3;
                    let a130 = 0u64;
                    if nat_eq(a129, a130) {
                        10368u64
                    } else {
                        9u64
                    }
                } else {
                    let a131 = v3;
                    let a132 = 4u64;
                    if nat_lt(a131, a132) {
                        let a133 = v3;
                        let a134 = 2u64;
                        if nat_eq(a133, a134) {
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
        let a135 = v1;
        let a136 = 10368u64;
        if nat_lt(a135, a136) {
            let a137 = v0;
            let a138 = 1u64;
            if nat_eq(a137, a138) {
                let a139 = v2;
                let a140 = 0u64;
                if if nat_eq(a139, a140) {
                    let a141 = v3;
                    let a142 = 9u64;
                    nat_lt(a141, a142)
                } else {
                    false
                } {
                    Ok(Some(f17(v1, v3)?))
                } else {
                    Ok(None::<u64>)
                }
            } else {
                let a143 = v0;
                let a144 = 2u64;
                if nat_eq(a143, a144) {
                    let a145 = v2;
                    let a146 = 4u64;
                    if if nat_lt(a145, a146) {
                        let a147 = v3;
                        let a148 = 10u64;
                        nat_lt(a147, a148)
                    } else {
                        false
                    } {
                        Ok(Some({
                            let a149 = v3;
                            let a150 = 4u64;
                            if nat_lt(a149, a150) {
                                f18(v1, v2, v3)?
                            } else {
                                let a151 = v3;
                                let a152 = 8u64;
                                if nat_lt(a151, a152) {
                                    f19(v1, v2, {
                                        let a153 = v3;
                                        let a154 = 4u64;
                                        nat_sub(a153, a154)
                                    })?
                                } else {
                                    let a155 = v3;
                                    let a156 = 8u64;
                                    if nat_eq(a155, a156) {
                                        f20(v1, v2)?
                                    } else {
                                        f21(v1, v2)?
                                    }
                                }
                            }
                        }))
                    } else {
                        Ok(None::<u64>)
                    }
                } else {
                    let a157 = v0;
                    let a158 = 3u64;
                    if nat_eq(a157, a158) {
                        let a159 = v2;
                        let a160 = 2u64;
                        if if nat_lt(a159, a160) {
                            let a161 = v3;
                            let a162 = 256u64;
                            nat_lt(a161, a162)
                        } else {
                            false
                        } {
                            Ok(Some(f22(v1, v2, v3)?))
                        } else {
                            Ok(None::<u64>)
                        }
                    } else {
                        let a163 = v0;
                        let a164 = 4u64;
                        if if nat_eq(a163, a164) {
                            true
                        } else {
                            let a165 = v0;
                            let a166 = 5u64;
                            nat_eq(a165, a166)
                        } {
                            let a167 = v3;
                            let a168 = 2u64;
                            if nat_lt(a167, a168) {
                                let a169 = v2;
                                let a170 = 0u64;
                                if nat_eq(a169, a170) {
                                    let a171 = v3;
                                    let a172 = 0u64;
                                    if nat_eq(a171, a172) {
                                        Ok(Some(f23(v1, {
                                            let a173 = v0;
                                            let a174 = 4u64;
                                            if nat_eq(a173, a174) {
                                                0u64
                                            } else {
                                                1u64
                                            }
                                        })?))
                                    } else {
                                        Ok(None::<u64>)
                                    }
                                } else {
                                    let a177 = v2;
                                    let a178 = f23(v1, {
                                        let a175 = v0;
                                        let a176 = 4u64;
                                        if nat_eq(a175, a176) {
                                            0u64
                                        } else {
                                            1u64
                                        }
                                    })?;
                                    if nat_le(a177, a178) {
                                        Ok(Some({
                                            let a179 = v3;
                                            let a180 = 0u64;
                                            if nat_eq(a179, a180) {
                                                let a183 = {
                                                    let a181 = v2;
                                                    let a182 = 1u64;
                                                    nat_sub(a181, a182)
                                                };
                                                let a184 = 256u64;
                                                nat_mul(a183, a184)?
                                            } else {
                                                let v4: u64 = {
                                                    let a191 = f20(v1, {
                                                        let a185 = v0;
                                                        let a186 = 4u64;
                                                        if nat_eq(a185, a186) {
                                                            0u64
                                                        } else {
                                                            2u64
                                                        }
                                                    })?;
                                                    let a192 = {
                                                        let a189 = {
                                                            let a187 = v2;
                                                            let a188 = 1u64;
                                                            nat_sub(a187, a188)
                                                        };
                                                        let a190 = 256u64;
                                                        nat_mul(a189, a190)?
                                                    };
                                                    nat_sub(a191, a192)
                                                };
                                                let a193 = v4;
                                                let a194 = 256u64;
                                                if nat_lt(a193, a194) {
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

pub fn f3(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a195 = v0;
    let a196 = 0u64;
    if nat_eq(a195, a196) {
        let a197 = v1;
        let a198 = 0u64;
        if if if nat_eq(a197, a198) {
            let a199 = v2;
            let a200 = 0u64;
            nat_eq(a199, a200)
        } else {
            false
        } {
            let a201 = v3;
            let a202 = 5u64;
            nat_lt(a201, a202)
        } else {
            false
        } {
            let a203 = v3;
            let a204 = 0u64;
            if nat_eq(a203, a204) {
                Ok(Some(160u64))
            } else {
                f4(0u64, 0u64, 0u64, v3)
            }
        } else {
            Ok(None::<u64>)
        }
    } else {
        let a205 = v1;
        let a206 = 160u64;
        if nat_lt(a205, a206) {
            let a207 = v0;
            let a208 = 2u64;
            if nat_eq(a207, a208) {
                let a209 = v2;
                let a210 = 4u64;
                if if nat_lt(a209, a210) {
                    let a211 = v3;
                    let a212 = 10u64;
                    nat_lt(a211, a212)
                } else {
                    false
                } {
                    Ok(Some({
                        let a213 = v3;
                        let a214 = 4u64;
                        if nat_lt(a213, a214) {
                            f24(v1, v2, v3)?
                        } else {
                            let a215 = v3;
                            let a216 = 8u64;
                            if nat_lt(a215, a216) {
                                f25(v1, v2, {
                                    let a217 = v3;
                                    let a218 = 4u64;
                                    nat_sub(a217, a218)
                                })?
                            } else {
                                let a219 = v3;
                                let a220 = 8u64;
                                if nat_eq(a219, a220) {
                                    f26(v1, v2)?
                                } else {
                                    f27(v1, v2)?
                                }
                            }
                        }
                    }))
                } else {
                    Ok(None::<u64>)
                }
            } else {
                f4(v0, f28(v1)?, v2, v3)
            }
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f4(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a221 = v0;
    let a222 = 0u64;
    if nat_eq(a221, a222) {
        let a223 = v1;
        let a224 = 0u64;
        if if if nat_eq(a223, a224) {
            let a225 = v2;
            let a226 = 0u64;
            nat_eq(a225, a226)
        } else {
            false
        } {
            let a227 = v3;
            let a228 = 5u64;
            nat_lt(a227, a228)
        } else {
            false
        } {
            Ok(Some({
                let a229 = v3;
                let a230 = 2u64;
                if nat_lt(a229, a230) {
                    let a231 = v3;
                    let a232 = 0u64;
                    if nat_eq(a231, a232) {
                        3456u64
                    } else {
                        9u64
                    }
                } else {
                    let a233 = v3;
                    let a234 = 4u64;
                    if nat_lt(a233, a234) {
                        let a235 = v3;
                        let a236 = 2u64;
                        if nat_eq(a235, a236) {
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
        let a237 = v1;
        let a238 = 3456u64;
        if if nat_lt(a237, a238) {
            f29(v1)?
        } else {
            false
        } {
            let a239 = v0;
            let a240 = 2u64;
            if nat_lt(a239, a240) {
                let a241 = v0;
                let a242 = 1u64;
                if nat_eq(a241, a242) {
                    let a243 = v2;
                    let a244 = 0u64;
                    if if nat_eq(a243, a244) {
                        let a245 = v3;
                        let a246 = 9u64;
                        nat_lt(a245, a246)
                    } else {
                        false
                    } {
                        Ok(Some(f30(v1, v3)?))
                    } else {
                        Ok(None::<u64>)
                    }
                } else {
                    Ok(None::<u64>)
                }
            } else {
                let a247 = v0;
                let a248 = 4u64;
                if nat_lt(a247, a248) {
                    let a249 = v0;
                    let a250 = 2u64;
                    if nat_eq(a249, a250) {
                        let a251 = v2;
                        let a252 = 2u64;
                        if if nat_lt(a251, a252) {
                            let a253 = v3;
                            let a254 = 10u64;
                            nat_lt(a253, a254)
                        } else {
                            false
                        } {
                            Ok(Some({
                                let a255 = v3;
                                let a256 = 4u64;
                                if nat_lt(a255, a256) {
                                    f31(v1, v2, v3)?
                                } else {
                                    let a257 = v3;
                                    let a258 = 8u64;
                                    if nat_lt(a257, a258) {
                                        f32(v1, v2, {
                                            let a259 = v3;
                                            let a260 = 4u64;
                                            nat_sub(a259, a260)
                                        })?
                                    } else {
                                        let a261 = v3;
                                        let a262 = 8u64;
                                        if nat_eq(a261, a262) {
                                            f33(v1, v2)?
                                        } else {
                                            f34(v1, v2)?
                                        }
                                    }
                                }
                            }))
                        } else {
                            Ok(None::<u64>)
                        }
                    } else {
                        let a263 = v2;
                        let a264 = 0u64;
                        if if nat_eq(a263, a264) {
                            let a265 = v3;
                            let a266 = 256u64;
                            nat_lt(a265, a266)
                        } else {
                            false
                        } {
                            Ok(Some(f35(v1, v3)?))
                        } else {
                            Ok(None::<u64>)
                        }
                    }
                } else {
                    let a267 = v0;
                    let a268 = 4u64;
                    if nat_eq(a267, a268) {
                        let a269 = v3;
                        let a270 = 2u64;
                        if nat_lt(a269, a270) {
                            let a271 = v2;
                            let a272 = 0u64;
                            if nat_eq(a271, a272) {
                                let a273 = v3;
                                let a274 = 0u64;
                                if nat_eq(a273, a274) {
                                    Ok(Some(f36(v1)?))
                                } else {
                                    Ok(None::<u64>)
                                }
                            } else {
                                let a275 = v2;
                                let a276 = f36(v1)?;
                                if nat_le(a275, a276) {
                                    Ok(Some({
                                        let a277 = v3;
                                        let a278 = 0u64;
                                        if nat_eq(a277, a278) {
                                            let a281 = {
                                                let a279 = v2;
                                                let a280 = 1u64;
                                                nat_sub(a279, a280)
                                            };
                                            let a282 = 256u64;
                                            nat_mul(a281, a282)?
                                        } else {
                                            let v4: u64 = {
                                                let a287 = f33(v1, 0u64)?;
                                                let a288 = {
                                                    let a285 = {
                                                        let a283 = v2;
                                                        let a284 = 1u64;
                                                        nat_sub(a283, a284)
                                                    };
                                                    let a286 = 256u64;
                                                    nat_mul(a285, a286)?
                                                };
                                                nat_sub(a287, a288)
                                            };
                                            let a289 = v4;
                                            let a290 = 256u64;
                                            if nat_lt(a289, a290) {
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

pub fn f5(v0: u64, v1: u64) -> R<u64> {
    let a291 = v1;
    let a292 = 12u64;
    if nat_lt(a291, a292) {
        let a293 = v1;
        let a294 = 4u64;
        if nat_lt(a293, a294) {
            let a295 = v1;
            let a296 = 2u64;
            if nat_lt(a295, a296) {
                let a297 = v1;
                let a298 = 1u64;
                if nat_lt(a297, a298) {
                    f37(v0)
                } else {
                    let a299 = f37(v0)?;
                    let a300 = 4u64;
                    if nat_lt(a299, a300) {
                        let a301 = f37(v0)?;
                        let a302 = 2u64;
                        if nat_lt(a301, a302) {
                            let a303 = f37(v0)?;
                            let a304 = 1u64;
                            if nat_lt(a303, a304) {
                                Ok(2u64)
                            } else {
                                Ok(5u64)
                            }
                        } else {
                            let a305 = f37(v0)?;
                            let a306 = 3u64;
                            if nat_lt(a305, a306) {
                                Ok(6u64)
                            } else {
                                Ok(7u64)
                            }
                        }
                    } else {
                        let a307 = f37(v0)?;
                        let a308 = 6u64;
                        if nat_lt(a307, a308) {
                            let a309 = f37(v0)?;
                            let a310 = 5u64;
                            if nat_lt(a309, a310) {
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
                let a311 = v1;
                let a312 = 3u64;
                if nat_lt(a311, a312) {
                    let a313 = f37(v0)?;
                    let a314 = 6u64;
                    if nat_eq(a313, a314) {
                        Ok(2u64)
                    } else {
                        Ok(0u64)
                    }
                } else {
                    Ok(f38(v0))
                }
            }
        } else {
            Ok(f39(v0, {
                let a317 = {
                    let a315 = v1;
                    let a316 = 4u64;
                    nat_sub(a315, a316)
                };
                let a318 = 4u64;
                let a319 = 0u64;
                nat_quot(a317, a318, a319)
            }, {
                let a322 = {
                    let a320 = v1;
                    let a321 = 4u64;
                    nat_sub(a320, a321)
                };
                let a323 = 4u64;
                let a324 = 0u64;
                nat_rem(a322, a323, a324)
            }))
        }
    } else {
        let a325 = v1;
        let a326 = 20u64;
        if nat_lt(a325, a326) {
            f40(v0, {
                let a329 = {
                    let a327 = v1;
                    let a328 = 12u64;
                    nat_sub(a327, a328)
                };
                let a330 = 4u64;
                let a331 = 0u64;
                nat_quot(a329, a330, a331)
            }, {
                let a334 = {
                    let a332 = v1;
                    let a333 = 12u64;
                    nat_sub(a332, a333)
                };
                let a335 = 4u64;
                let a336 = 0u64;
                nat_rem(a334, a335, a336)
            })
        } else {
            let a337 = v1;
            let a338 = 22u64;
            if nat_lt(a337, a338) {
                f6(v0, {
                    let a339 = v1;
                    let a340 = 20u64;
                    nat_sub(a339, a340)
                })
            } else {
                let a341 = v1;
                let a342 = 22u64;
                if nat_eq(a341, a342) {
                    let a343 = v0;
                    let a344 = 0u64;
                    if nat_eq(a343, a344) {
                        Ok(1u64)
                    } else {
                        let a345 = v0;
                        let a346 = 108u64;
                        if nat_eq(a345, a346) {
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

pub fn f6(v0: u64, v1: u64) -> R<u64> {
    let a347 = v1;
    let a348 = 0u64;
    if nat_eq(a347, a348) {
        let a349 = f41(v0);
        let a350 = 12u64;
        nat_mul(a349, a350)
    } else {
        let a351 = 96u64;
        let a352 = v0;
        if nat_le(a351, a352) {
            Ok(0u64)
        } else {
            let a353 = f42(v0);
            let a354 = 2u64;
            if nat_lt(a353, a354) {
                let a355 = f42(v0);
                let a356 = 1u64;
                if nat_lt(a355, a356) {
                    let a357 = f41(v0);
                    let a358 = 12u64;
                    nat_mul(a357, a358)
                } else {
                    Ok(1u64)
                }
            } else {
                let a359 = f42(v0);
                let a360 = 3u64;
                if nat_lt(a359, a360) {
                    Ok(f41(v0))
                } else {
                    Ok(6u64)
                }
            }
        }
    }
}

pub fn f7(v0: u64, v1: u64) -> R<u64> {
    let a361 = v1;
    let a362 = 16u64;
    if nat_lt(a361, a362) {
        Ok(f43(f44(v0, v1)?))
    } else {
        Ok(f45({
            let a363 = f46(v0);
            let a364 = f47(v1)?;
            nat_add(a363, a364)?
        }))
    }
}

pub fn f8(v0: u64) -> u64 {
    let a365 = v0;
    let a366 = 0u64;
    if nat_eq(a365, a366) {
        15360u64
    } else {
        1920u64
    }
}

pub fn f9(v0: u64) -> u64 {
    let a367 = v0;
    let a368 = 0u64;
    if nat_eq(a367, a368) {
        256u64
    } else {
        192u64
    }
}

pub fn f10(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a369 = v2;
    let a370 = 5u64;
    if nat_lt(a369, a370) {
        let a371 = v2;
        let a372 = 2u64;
        if nat_lt(a371, a372) {
            let a373 = v2;
            let a374 = 1u64;
            if nat_lt(a373, a374) {
                Ok(f48(v0, v1))
            } else {
                f49(v0, v1)
            }
        } else {
            let a375 = v2;
            let a376 = 3u64;
            if nat_lt(a375, a376) {
                Ok(f50(v0, v1))
            } else {
                let a377 = v2;
                let a378 = 4u64;
                if nat_lt(a377, a378) {
                    Ok(f51(v1))
                } else {
                    f52(v1)
                }
            }
        }
    } else {
        let a379 = v2;
        let a380 = 7u64;
        if nat_lt(a379, a380) {
            let a381 = v2;
            let a382 = 6u64;
            if nat_lt(a381, a382) {
                f53(v1)
            } else {
                f54(v1)
            }
        } else {
            let a383 = v2;
            let a384 = 8u64;
            if nat_lt(a383, a384) {
                f55(v0, v1)
            } else {
                let a385 = v2;
                let a386 = 9u64;
                if nat_lt(a385, a386) {
                    Ok(0u64)
                } else {
                    let a387 = v0;
                    let a388 = 0u64;
                    if nat_eq(a387, a388) {
                        Ok(f56({
                            let a389 = f50(v0, v1);
                            let a390 = 5u64;
                            let a391 = 0u64;
                            nat_quot(a389, a390, a391)
                        }))
                    } else {
                        f57(v0, v1)
                    }
                }
            }
        }
    }
}

pub fn f11(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    if if f58(v0, v1, v2)? {
        let a392 = f51(v1);
        let a393 = 3u64;
        nat_eq(a392, a393)
    } else {
        false
    } {
        let a394 = v3;
        let a395 = 2u64;
        if nat_lt(a394, a395) {
            let a396 = v3;
            let a397 = 0u64;
            if nat_eq(a396, a397) {
                Ok(1u64)
            } else {
                f59(v0, v1)
            }
        } else {
            Ok(1u64)
        }
    } else {
        f60(v0, v1, v3)
    }
}

pub fn f12(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a398 = v3;
    let a399 = 2u64;
    if nat_lt(a398, a399) {
        let a400 = v3;
        let a401 = 0u64;
        if nat_eq(a400, a401) {
            f61(v0, v1, v2)
        } else {
            f62(v0, v1, v2)
        }
    } else {
        let a402 = v3;
        let a403 = 2u64;
        if nat_eq(a402, a403) {
            f63(v0, v1, v2)
        } else {
            f64(v0, v1, v2)
        }
    }
}

pub fn f13(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let m404 = f65(Adt0::C0(f11(v0, v1, v2, 0u64)?, f11(v0, v1, v2, 1u64)?, f11(v0, v1, v2, 2u64)?, f11(v0, v1, v2, 3u64)?, f12(v0, v1, v2, 0u64)?, f12(v0, v1, v2, 1u64)?, f12(v0, v1, v2, 2u64)?, f12(v0, v1, v2, 3u64)?, 1u64, f66(v0, v1, v2)?))?;
    match m404 {
        Adt1::C0 => {
            Ok(0u64)
        }
        Adt1::C1(v3) => {
            Ok(v3)
        }
    }
}

pub fn f14(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a409 = {
        let a405 = f11(v0, v1, v2, 0u64)?;
        let a406 = f11(v0, v1, v2, 1u64)?;
        nat_mul(a405, a406)?
    };
    let a410 = {
        let a407 = f11(v0, v1, v2, 2u64)?;
        let a408 = f11(v0, v1, v2, 3u64)?;
        nat_mul(a407, a408)?
    };
    nat_mul(a409, a410)
}

pub fn f15(v0: u64, v1: u64, v2: u64) -> R<u64> {
    f67(v0, f55(v0, v1)?, v2)
}

pub fn f16(v0: u64, v1: u64) -> R<u64> {
    let a415 = {
        let a413 = f13(v0, v1, 0u64)?;
        let a414 = {
            let a411 = f9(v0);
            let a412 = 1u64;
            nat_sub(a411, a412)
        };
        nat_add(a413, a414)?
    };
    let a416 = f9(v0);
    let a417 = 0u64;
    Ok(nat_quot(a415, a416, a417))
}

pub fn f17(v0: u64, v1: u64) -> R<u64> {
    let a418 = v1;
    let a419 = 0u64;
    if nat_eq(a418, a419) {
        f68(v0)
    } else {
        let a420 = v1;
        let a421 = 1u64;
        if nat_eq(a420, a421) {
            Ok(f69(v0))
        } else {
            let a422 = v1;
            let a423 = 2u64;
            if nat_eq(a422, a423) {
                Ok(0u64)
            } else {
                let a424 = v1;
                let a425 = 3u64;
                if nat_eq(a424, a425) {
                    Ok(f70(v0))
                } else {
                    let a426 = v1;
                    let a427 = 4u64;
                    if nat_eq(a426, a427) {
                        Ok(f71(v0))
                    } else {
                        let a428 = v1;
                        let a429 = 5u64;
                        if nat_eq(a428, a429) {
                            Ok(f72(v0))
                        } else {
                            let a430 = v1;
                            let a431 = 6u64;
                            if nat_eq(a430, a431) {
                                f73(v0)
                            } else {
                                let a432 = v1;
                                let a433 = 7u64;
                                if nat_eq(a432, a433) {
                                    f74(v0, 0u64)
                                } else {
                                    f74(v0, 1u64)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f18(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a434 = v1;
    let a435 = 2u64;
    if nat_lt(a434, a435) {
        let a436 = v1;
        let a437 = 0u64;
        if nat_eq(a436, a437) {
            f75(v0, v2)
        } else {
            let a438 = f70(v0);
            let a439 = 3u64;
            if nat_eq(a438, a439) {
                let a440 = v2;
                let a441 = 2u64;
                if nat_lt(a440, a441) {
                    let a442 = v2;
                    let a443 = 0u64;
                    if nat_eq(a442, a443) {
                        f76(v0)
                    } else {
                        let a444 = f77(v0)?;
                        let a445 = f76(v0)?;
                        let a446 = 0u64;
                        Ok(nat_quot(a444, a445, a446))
                    }
                } else {
                    Ok(1u64)
                }
            } else {
                f75(v0, v2)
            }
        }
    } else {
        let a447 = v1;
        let a448 = 2u64;
        if nat_eq(a447, a448) {
            f78(v0, v2)
        } else {
            let a449 = v2;
            let a450 = f69(v0);
            if nat_eq(a449, a450) {
                let a451 = f75(v0, v2)?;
                let a452 = f78(v0, v2)?;
                nat_add(a451, a452)
            } else {
                f75(v0, v2)
            }
        }
    }
}

pub fn f19(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a453 = v2;
    let a454 = 2u64;
    if nat_lt(a453, a454) {
        let a455 = v2;
        let a456 = 0u64;
        if nat_eq(a455, a456) {
            f79(v0, v1)
        } else {
            f80(v0, v1)
        }
    } else {
        let a457 = v2;
        let a458 = 2u64;
        if nat_eq(a457, a458) {
            f81(v0, v1)
        } else {
            f82(v0, v1)
        }
    }
}

pub fn f20(v0: u64, v1: u64) -> R<u64> {
    let m459 = f65(Adt0::C0(f18(v0, v1, 0u64)?, f18(v0, v1, 1u64)?, f18(v0, v1, 2u64)?, f18(v0, v1, 3u64)?, f19(v0, v1, 0u64)?, f19(v0, v1, 1u64)?, f19(v0, v1, 2u64)?, f19(v0, v1, 3u64)?, f76(v0)?, f83(v0)?))?;
    match m459 {
        Adt1::C0 => {
            Ok(0u64)
        }
        Adt1::C1(v2) => {
            Ok(v2)
        }
    }
}

pub fn f21(v0: u64, v1: u64) -> R<u64> {
    let a464 = {
        let a460 = f18(v0, v1, 0u64)?;
        let a461 = f18(v0, v1, 1u64)?;
        nat_mul(a460, a461)?
    };
    let a465 = {
        let a462 = f18(v0, v1, 2u64)?;
        let a463 = f18(v0, v1, 3u64)?;
        nat_mul(a462, a463)?
    };
    nat_mul(a464, a465)
}

pub fn f22(v0: u64, v1: u64, v2: u64) -> R<u64> {
    f84(f74(v0, v1)?, v2)
}

pub fn f23(v0: u64, v1: u64) -> R<u64> {
    let a470 = {
        let a468 = f20(v0, {
            let a466 = v1;
            let a467 = 0u64;
            if nat_eq(a466, a467) {
                0u64
            } else {
                2u64
            }
        })?;
        let a469 = 255u64;
        nat_add(a468, a469)?
    };
    let a471 = 256u64;
    let a472 = 0u64;
    Ok(nat_quot(a470, a471, a472))
}

pub fn f24(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a473 = v1;
    let a474 = 2u64;
    if nat_lt(a473, a474) {
        f31(f28(v0)?, v1, v2)
    } else {
        let a488 = f31(f28(v0)?, 0u64, v2)?;
        let a489 = {
            let a475 = v2;
            let a476 = 2u64;
            if nat_lt(a475, a476) {
                let a477 = v2;
                let a478 = 0u64;
                if nat_eq(a477, a478) {
                    2u64
                } else {
                    let a482 = {
                        let a479 = f85(v0)?;
                        let a480 = 2u64;
                        let a481 = 0u64;
                        nat_rem(a479, a480, a481)
                    };
                    let a483 = 1u64;
                    if nat_eq(a482, a483) {
                        3u64
                    } else {
                        1u64
                    }
                }
            } else {
                let a484 = v2;
                let a485 = 2u64;
                if nat_eq(a484, a485) {
                    let a486 = 2u64;
                    let a487 = f85(v0)?;
                    if nat_le(a486, a487) {
                        2u64
                    } else {
                        1u64
                    }
                } else {
                    2u64
                }
            }
        };
        nat_mul(a488, a489)
    }
}

pub fn f25(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a490 = v1;
    let a491 = 2u64;
    if nat_lt(a490, a491) {
        f32(f28(v0)?, v1, v2)
    } else {
        let a492 = v2;
        let a493 = 2u64;
        if nat_lt(a492, a493) {
            let a494 = v2;
            let a495 = 0u64;
            if nat_eq(a494, a495) {
                f86(v0)
            } else {
                f87(v0, v1)
            }
        } else {
            let a496 = v2;
            let a497 = 2u64;
            if nat_eq(a496, a497) {
                f88(v0, v1)
            } else {
                f89(v0, v1)
            }
        }
    }
}

pub fn f26(v0: u64, v1: u64) -> R<u64> {
    let a498 = v1;
    let a499 = 2u64;
    if nat_lt(a498, a499) {
        f33(f28(v0)?, v1)
    } else {
        let m500 = f65(Adt0::C0(f24(v0, v1, 0u64)?, f24(v0, v1, 1u64)?, f24(v0, v1, 2u64)?, f24(v0, v1, 3u64)?, f25(v0, v1, 0u64)?, f25(v0, v1, 1u64)?, f25(v0, v1, 2u64)?, f25(v0, v1, 3u64)?, 1u64, f86(v0)?))?;
        match m500 {
            Adt1::C0 => {
                Ok(0u64)
            }
            Adt1::C1(v2) => {
                Ok(v2)
            }
        }
    }
}

pub fn f27(v0: u64, v1: u64) -> R<u64> {
    let a505 = {
        let a501 = f24(v0, v1, 0u64)?;
        let a502 = f24(v0, v1, 1u64)?;
        nat_mul(a501, a502)?
    };
    let a506 = {
        let a503 = f24(v0, v1, 2u64)?;
        let a504 = f24(v0, v1, 3u64)?;
        nat_mul(a503, a504)?
    };
    nat_mul(a505, a506)
}

pub fn f28(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a507 = v0;
        let a508 = 32u64;
        let a509 = 0u64;
        nat_quot(a507, a508, a509)
    };
    let a521 = {
        let a516 = {
            let a510 = v1;
            let a511 = 2u64;
            if nat_lt(a510, a511) {
                v1
            } else {
                let a512 = v1;
                let a513 = 2u64;
                if nat_eq(a512, a513) {
                    23u64
                } else {
                    let a514 = v1;
                    let a515 = 3u64;
                    if nat_eq(a514, a515) {
                        24u64
                    } else {
                        28u64
                    }
                }
            }
        };
        let a517 = 96u64;
        nat_mul(a516, a517)?
    };
    let a522 = {
        let a518 = v0;
        let a519 = 32u64;
        let a520 = 0u64;
        nat_rem(a518, a519, a520)
    };
    nat_add(a521, a522)
}

pub fn f29(v0: u64) -> R<bool> {
    let m523 = f90(f91(v0)?);
    match m523 {
        Adt2::C0 => {
            Ok(false)
        }
        Adt2::C1(_) => {
            Ok(true)
        }
    }
}

pub fn f30(v0: u64, v1: u64) -> R<u64> {
    let a524 = v1;
    let a525 = 0u64;
    if nat_eq(a524, a525) {
        f91(v0)
    } else {
        let a526 = v1;
        let a527 = 1u64;
        if nat_eq(a526, a527) {
            Ok(f92(v0))
        } else {
            let a528 = v1;
            let a529 = 2u64;
            if nat_eq(a528, a529) {
                Ok(0u64)
            } else {
                let a530 = v1;
                let a531 = 3u64;
                if nat_eq(a530, a531) {
                    Ok(f93(v0))
                } else {
                    let a532 = v1;
                    let a533 = 4u64;
                    if nat_eq(a532, a533) {
                        f94(v0)
                    } else {
                        let a534 = v1;
                        let a535 = 5u64;
                        if nat_eq(a534, a535) {
                            f95(v0)
                        } else {
                            let a536 = v1;
                            let a537 = 6u64;
                            if nat_eq(a536, a537) {
                                f96(v0)
                            } else {
                                let a538 = v1;
                                let a539 = 7u64;
                                if nat_eq(a538, a539) {
                                    f97(v0)
                                } else {
                                    let a540 = f92(v0);
                                    let a541 = 1u64;
                                    if if nat_eq(a540, a541) {
                                        let a542 = f93(v0);
                                        let a543 = 3u64;
                                        nat_eq(a542, a543)
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

pub fn f31(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a544 = v1;
    let a545 = 1u64;
    if if nat_eq(a544, a545) {
        let a546 = f93(v0);
        let a547 = 3u64;
        nat_eq(a546, a547)
    } else {
        false
    } {
        let a548 = v2;
        let a549 = 2u64;
        if nat_lt(a548, a549) {
            let a550 = v2;
            let a551 = 0u64;
            if nat_eq(a550, a551) {
                f98(v0)
            } else {
                let a556 = {
                    let a552 = f94(v0)?;
                    let a553 = f95(v0)?;
                    nat_mul(a552, a553)?
                };
                let a557 = {
                    let a554 = 2u64;
                    let a555 = f96(v0)?;
                    nat_mul(a554, a555)?
                };
                nat_mul(a556, a557)
            }
        } else {
            Ok(1u64)
        }
    } else {
        let a558 = v2;
        let a559 = 2u64;
        if nat_lt(a558, a559) {
            let a560 = v2;
            let a561 = 0u64;
            if nat_eq(a560, a561) {
                let a562 = f98(v0)?;
                let a563 = f94(v0)?;
                nat_mul(a562, a563)
            } else {
                f95(v0)
            }
        } else {
            let a564 = v2;
            let a565 = 2u64;
            if nat_eq(a564, a565) {
                Ok(2u64)
            } else {
                f96(v0)
            }
        }
    }
}

pub fn f32(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a566 = v2;
    let a567 = 2u64;
    if nat_lt(a566, a567) {
        let a568 = v2;
        let a569 = 0u64;
        if nat_eq(a568, a569) {
            f99(v0, v1)
        } else {
            f100(v0, v1)
        }
    } else {
        let a570 = v2;
        let a571 = 2u64;
        if nat_eq(a570, a571) {
            f101(v0, v1)
        } else {
            f102(v0, v1)
        }
    }
}

pub fn f33(v0: u64, v1: u64) -> R<u64> {
    let m572 = f65(Adt0::C0(f31(v0, v1, 0u64)?, f31(v0, v1, 1u64)?, f31(v0, v1, 2u64)?, f31(v0, v1, 3u64)?, f32(v0, v1, 0u64)?, f32(v0, v1, 1u64)?, f32(v0, v1, 2u64)?, f32(v0, v1, 3u64)?, f98(v0)?, f103(v0)?))?;
    match m572 {
        Adt1::C0 => {
            Ok(0u64)
        }
        Adt1::C1(v2) => {
            Ok(v2)
        }
    }
}

pub fn f34(v0: u64, v1: u64) -> R<u64> {
    let a577 = {
        let a573 = f31(v0, v1, 0u64)?;
        let a574 = f31(v0, v1, 1u64)?;
        nat_mul(a573, a574)?
    };
    let a578 = {
        let a575 = f31(v0, v1, 2u64)?;
        let a576 = f31(v0, v1, 3u64)?;
        nat_mul(a575, a576)?
    };
    nat_mul(a577, a578)
}

pub fn f35(v0: u64, v1: u64) -> R<u64> {
    f104(f97(v0)?, v1)
}

pub fn f36(v0: u64) -> R<u64> {
    let a581 = {
        let a579 = f33(v0, 0u64)?;
        let a580 = 255u64;
        nat_add(a579, a580)?
    };
    let a582 = 256u64;
    let a583 = 0u64;
    Ok(nat_quot(a581, a582, a583))
}

pub fn f37(v0: u64) -> R<u64> {
    let a584 = v0;
    let a585 = 96u64;
    if nat_lt(a584, a585) {
        let a586 = v0;
        let a587 = 24u64;
        let a588 = 0u64;
        Ok(nat_quot(a586, a587, a588))
    } else {
        let a594 = 4u64;
        let a595 = {
            let a591 = {
                let a589 = v0;
                let a590 = 96u64;
                nat_sub(a589, a590)
            };
            let a592 = 6u64;
            let a593 = 0u64;
            nat_quot(a591, a592, a593)
        };
        nat_add(a594, a595)
    }
}

pub fn f38(v0: u64) -> u64 {
    let a596 = v0;
    let a597 = 96u64;
    if nat_lt(a596, a597) {
        let a601 = {
            let a598 = v0;
            let a599 = 4u64;
            let a600 = 0u64;
            nat_quot(a598, a599, a600)
        };
        let a602 = 2u64;
        let a603 = 0u64;
        nat_rem(a601, a602, a603)
    } else {
        let a604 = v0;
        let a605 = 2u64;
        let a606 = 0u64;
        nat_rem(a604, a605, a606)
    }
}

pub fn f39(v0: u64, v1: u64, v2: u64) -> u64 {
    let a607 = v1;
    let a608 = 0u64;
    if nat_eq(a607, a608) {
        let a609 = v2;
        let a610 = 2u64;
        if nat_lt(a609, a610) {
            let a611 = v2;
            let a612 = 1u64;
            if nat_lt(a611, a612) {
                f41(v0)
            } else {
                3u64
            }
        } else {
            2u64
        }
    } else {
        let a613 = 96u64;
        let a614 = v0;
        if nat_le(a613, a614) {
            0u64
        } else {
            let a615 = v2;
            let a616 = 2u64;
            if nat_lt(a615, a616) {
                let a617 = v2;
                let a618 = 1u64;
                if nat_lt(a617, a618) {
                    let a619 = f42(v0);
                    let a620 = 1u64;
                    if if nat_eq(a619, a620) {
                        true
                    } else {
                        let a621 = f42(v0);
                        let a622 = 3u64;
                        nat_eq(a621, a622)
                    } {
                        1u64
                    } else {
                        f41(v0)
                    }
                } else {
                    let a623 = f42(v0);
                    let a624 = 1u64;
                    if if nat_eq(a623, a624) {
                        true
                    } else {
                        let a625 = f42(v0);
                        let a626 = 2u64;
                        nat_eq(a625, a626)
                    } {
                        1u64
                    } else {
                        3u64
                    }
                }
            } else {
                let a627 = v2;
                let a628 = 3u64;
                if nat_lt(a627, a628) {
                    let a629 = f42(v0);
                    let a630 = 0u64;
                    if nat_eq(a629, a630) {
                        2u64
                    } else {
                        1u64
                    }
                } else {
                    let a631 = f42(v0);
                    let a632 = 1u64;
                    if if nat_eq(a631, a632) {
                        true
                    } else {
                        let a633 = f42(v0);
                        let a634 = 2u64;
                        nat_eq(a633, a634)
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

pub fn f40(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a635 = v1;
    let a636 = 1u64;
    if if nat_eq(a635, a636) {
        let a637 = 96u64;
        let a638 = v0;
        nat_le(a637, a638)
    } else {
        false
    } {
        Ok(0u64)
    } else {
        let v3: u64 = f41(v0);
        let v4: u64 = f38(v0);
        let v5: u64 = {
            let a639 = v1;
            let a640 = 0u64;
            if nat_eq(a639, a640) {
                0u64
            } else {
                f42(v0)
            }
        };
        let a641 = v5;
        let a642 = 2u64;
        if nat_lt(a641, a642) {
            let a643 = v5;
            let a644 = 1u64;
            if nat_lt(a643, a644) {
                let a645 = v2;
                let a646 = 2u64;
                if nat_lt(a645, a646) {
                    let a647 = v2;
                    let a648 = 1u64;
                    if nat_lt(a647, a648) {
                        Ok(4u64)
                    } else {
                        let a651 = {
                            let a649 = v3;
                            let a650 = v4;
                            nat_add(a649, a650)?
                        };
                        let a652 = 4u64;
                        nat_mul(a651, a652)
                    }
                } else {
                    let a653 = v2;
                    let a654 = 3u64;
                    if nat_lt(a653, a654) {
                        let a661 = {
                            let a659 = {
                                let a655 = v3;
                                let a656 = 3u64;
                                nat_mul(a655, a656)?
                            };
                            let a660 = {
                                let a657 = v4;
                                let a658 = 4u64;
                                nat_mul(a657, a658)?
                            };
                            nat_add(a659, a660)?
                        };
                        let a662 = 4u64;
                        nat_mul(a661, a662)
                    } else {
                        let a669 = {
                            let a667 = {
                                let a663 = v3;
                                let a664 = 6u64;
                                nat_mul(a663, a664)?
                            };
                            let a668 = {
                                let a665 = v4;
                                let a666 = 9u64;
                                nat_mul(a665, a666)?
                            };
                            nat_add(a667, a668)?
                        };
                        let a670 = 4u64;
                        nat_mul(a669, a670)
                    }
                }
            } else {
                let a671 = v2;
                let a672 = 2u64;
                if nat_lt(a671, a672) {
                    let a673 = v2;
                    let a674 = 1u64;
                    if nat_lt(a673, a674) {
                        Ok(4u64)
                    } else {
                        let a677 = {
                            let a675 = 1u64;
                            let a676 = v4;
                            nat_add(a675, a676)?
                        };
                        let a678 = 4u64;
                        nat_mul(a677, a678)
                    }
                } else {
                    let a679 = v2;
                    let a680 = 3u64;
                    if nat_lt(a679, a680) {
                        let a685 = {
                            let a683 = 1u64;
                            let a684 = {
                                let a681 = v4;
                                let a682 = 2u64;
                                nat_mul(a681, a682)?
                            };
                            nat_add(a683, a684)?
                        };
                        let a686 = 4u64;
                        nat_mul(a685, a686)
                    } else {
                        let a691 = {
                            let a689 = 1u64;
                            let a690 = {
                                let a687 = v4;
                                let a688 = 3u64;
                                nat_mul(a687, a688)?
                            };
                            nat_add(a689, a690)?
                        };
                        let a692 = 4u64;
                        nat_mul(a691, a692)
                    }
                }
            }
        } else {
            let a693 = v5;
            let a694 = 3u64;
            if nat_lt(a693, a694) {
                let a695 = v2;
                let a696 = 2u64;
                if nat_lt(a695, a696) {
                    let a697 = v2;
                    let a698 = 1u64;
                    if nat_lt(a697, a698) {
                        Ok(4u64)
                    } else {
                        let a701 = {
                            let a699 = v3;
                            let a700 = v4;
                            nat_add(a699, a700)?
                        };
                        let a702 = 4u64;
                        nat_mul(a701, a702)
                    }
                } else {
                    let a703 = v2;
                    let a704 = 3u64;
                    if nat_lt(a703, a704) {
                        let a709 = {
                            let a707 = v3;
                            let a708 = {
                                let a705 = v4;
                                let a706 = 2u64;
                                nat_mul(a705, a706)?
                            };
                            nat_add(a707, a708)?
                        };
                        let a710 = 4u64;
                        nat_mul(a709, a710)
                    } else {
                        let a715 = {
                            let a713 = v3;
                            let a714 = {
                                let a711 = v4;
                                let a712 = 3u64;
                                nat_mul(a711, a712)?
                            };
                            nat_add(a713, a714)?
                        };
                        let a716 = 4u64;
                        nat_mul(a715, a716)
                    }
                }
            } else {
                let a717 = v2;
                let a718 = 2u64;
                if nat_lt(a717, a718) {
                    let a719 = v2;
                    let a720 = 1u64;
                    if nat_lt(a719, a720) {
                        Ok(4u64)
                    } else {
                        let a723 = {
                            let a721 = 1u64;
                            let a722 = v4;
                            nat_add(a721, a722)?
                        };
                        let a724 = 4u64;
                        nat_mul(a723, a724)
                    }
                } else {
                    let a725 = v2;
                    let a726 = 3u64;
                    if nat_lt(a725, a726) {
                        let a731 = {
                            let a729 = 3u64;
                            let a730 = {
                                let a727 = v4;
                                let a728 = 4u64;
                                nat_mul(a727, a728)?
                            };
                            nat_add(a729, a730)?
                        };
                        let a732 = 4u64;
                        nat_mul(a731, a732)
                    } else {
                        let a737 = {
                            let a735 = 3u64;
                            let a736 = {
                                let a733 = v4;
                                let a734 = 5u64;
                                nat_mul(a733, a734)?
                            };
                            nat_add(a735, a736)?
                        };
                        let a738 = 4u64;
                        nat_mul(a737, a738)
                    }
                }
            }
        }
    }
}

pub fn f41(v0: u64) -> u64 {
    let a739 = f105(v0);
    let a740 = 0u64;
    if nat_eq(a739, a740) {
        1u64
    } else {
        let a741 = f105(v0);
        let a742 = 1u64;
        if nat_eq(a741, a742) {
            17u64
        } else {
            257u64
        }
    }
}

pub fn f42(v0: u64) -> u64 {
    let a743 = v0;
    let a744 = 4u64;
    let a745 = 0u64;
    nat_rem(a743, a744, a745)
}

pub fn f43(v0: u64) -> u64 {
    let a746 = v0;
    let a747 = 8u64;
    if nat_lt(a746, a747) {
        let a748 = v0;
        let a749 = 4u64;
        if nat_lt(a748, a749) {
            let a750 = v0;
            let a751 = 2u64;
            if nat_lt(a750, a751) {
                let a752 = v0;
                let a753 = 1u64;
                if nat_lt(a752, a753) {
                    0u64
                } else {
                    2147483648u64
                }
            } else {
                let a754 = v0;
                let a755 = 3u64;
                if nat_lt(a754, a755) {
                    1065353216u64
                } else {
                    3212836864u64
                }
            }
        } else {
            let a756 = v0;
            let a757 = 6u64;
            if nat_lt(a756, a757) {
                let a758 = v0;
                let a759 = 5u64;
                if nat_lt(a758, a759) {
                    2139095040u64
                } else {
                    4286578688u64
                }
            } else {
                let a760 = v0;
                let a761 = 7u64;
                if nat_lt(a760, a761) {
                    2143289345u64
                } else {
                    1u64
                }
            }
        }
    } else {
        let a762 = v0;
        let a763 = 12u64;
        if nat_lt(a762, a763) {
            let a764 = v0;
            let a765 = 10u64;
            if nat_lt(a764, a765) {
                let a766 = v0;
                let a767 = 9u64;
                if nat_lt(a766, a767) {
                    2139095039u64
                } else {
                    8388608u64
                }
            } else {
                let a768 = v0;
                let a769 = 11u64;
                if nat_lt(a768, a769) {
                    8388607u64
                } else {
                    1065353217u64
                }
            }
        } else {
            let a770 = v0;
            let a771 = 14u64;
            if nat_lt(a770, a771) {
                let a772 = v0;
                let a773 = 13u64;
                if nat_lt(a772, a773) {
                    1056964608u64
                } else {
                    8388609u64
                }
            } else {
                let a774 = v0;
                let a775 = 15u64;
                if nat_lt(a774, a775) {
                    2147483649u64
                } else {
                    2139095041u64
                }
            }
        }
    }
}

pub fn f44(v0: u64, v1: u64) -> R<u64> {
    let a784 = {
        let a782 = {
            let a778 = {
                let a776 = v0;
                let a777 = 0u64;
                if nat_eq(a776, a777) {
                    1u64
                } else {
                    5u64
                }
            };
            let a779 = v1;
            nat_mul(a778, a779)?
        };
        let a783 = {
            let a780 = v0;
            let a781 = 0u64;
            if nat_eq(a780, a781) {
                0u64
            } else {
                3u64
            }
        };
        nat_add(a782, a783)?
    };
    let a785 = 16u64;
    let a786 = 0u64;
    Ok(nat_rem(a784, a785, a786))
}

pub fn f45(v0: u64) -> u64 {
    let a787 = v0;
    let a788 = 4294967296u64;
    let a789 = 0u64;
    nat_rem(a787, a788, a789)
}

pub fn f46(v0: u64) -> u64 {
    let a790 = v0;
    let a791 = 0u64;
    if nat_eq(a790, a791) {
        2654435769u64
    } else {
        2246822507u64
    }
}

pub fn f47(v0: u64) -> R<u64> {
    let a792 = 2654435761u64;
    let a793 = v0;
    nat_mul(a792, a793)
}

pub fn f48(v0: u64, v1: u64) -> u64 {
    f106(f107(v0, v1))
}

pub fn f49(v0: u64, v1: u64) -> R<u64> {
    let a794 = v0;
    let a795 = 0u64;
    if nat_eq(a794, a795) {
        let a799 = {
            let a796 = v1;
            let a797 = 3840u64;
            let a798 = 0u64;
            nat_quot(a796, a797, a798)
        };
        let a800 = 2u64;
        let a801 = 0u64;
        Ok(nat_rem(a799, a800, a801))
    } else {
        let v2: u64 = {
            let a805 = {
                let a802 = v1;
                let a803 = 160u64;
                let a804 = 0u64;
                nat_quot(a802, a803, a804)
            };
            let a806 = 4u64;
            let a807 = 0u64;
            nat_rem(a805, a806, a807)
        };
        let v3: u64 = {
            let a808 = v2;
            let a809 = 2u64;
            let a810 = 0u64;
            nat_rem(a808, a809, a810)
        };
        let a820 = {
            let a814 = {
                let a811 = v2;
                let a812 = 2u64;
                let a813 = 0u64;
                nat_quot(a811, a812, a813)
            };
            let a815 = 3u64;
            nat_mul(a814, a815)?
        };
        let a821 = {
            let a816 = v3;
            let a817 = f107(v0, v1);
            if nat_lt(a816, a817) {
                v3
            } else {
                let a818 = v3;
                let a819 = 1u64;
                nat_add(a818, a819)?
            }
        };
        nat_add(a820, a821)
    }
}

pub fn f50(v0: u64, v1: u64) -> u64 {
    let a827 = {
        let a822 = v1;
        let a823 = 32u64;
        let a824 = 0u64;
        nat_quot(a822, a823, a824)
    };
    let a828 = {
        let a825 = v0;
        let a826 = 0u64;
        if nat_eq(a825, a826) {
            120u64
        } else {
            5u64
        }
    };
    let a829 = 0u64;
    nat_rem(a827, a828, a829)
}

pub fn f51(v0: u64) -> u64 {
    f93({
        let a830 = v0;
        let a831 = 32u64;
        let a832 = 0u64;
        nat_rem(a830, a831, a832)
    })
}

pub fn f52(v0: u64) -> R<u64> {
    f94({
        let a833 = v0;
        let a834 = 32u64;
        let a835 = 0u64;
        nat_rem(a833, a834, a835)
    })
}

pub fn f53(v0: u64) -> R<u64> {
    f95({
        let a836 = v0;
        let a837 = 32u64;
        let a838 = 0u64;
        nat_rem(a836, a837, a838)
    })
}

pub fn f54(v0: u64) -> R<u64> {
    f96({
        let a839 = v0;
        let a840 = 32u64;
        let a841 = 0u64;
        nat_rem(a839, a840, a841)
    })
}

pub fn f55(v0: u64, v1: u64) -> R<u64> {
    let a842 = v0;
    let a843 = 0u64;
    if nat_eq(a842, a843) {
        let a846 = {
            let a844 = f107(v0, v1);
            let a845 = 4u64;
            nat_mul(a844, a845)?
        };
        let a847 = f51(v1);
        nat_add(a846, a847)
    } else {
        Ok(f107(v0, v1))
    }
}

pub fn f56(v0: u64) -> u64 {
    let a848 = v0;
    let a849 = 12u64;
    if nat_lt(a848, a849) {
        let a850 = v0;
        let a851 = 6u64;
        if nat_lt(a850, a851) {
            let a852 = v0;
            let a853 = 3u64;
            if nat_lt(a852, a853) {
                let a854 = v0;
                let a855 = 1u64;
                if nat_lt(a854, a855) {
                    0u64
                } else {
                    let a856 = v0;
                    let a857 = 2u64;
                    if nat_lt(a856, a857) {
                        2147483648u64
                    } else {
                        1065353216u64
                    }
                }
            } else {
                let a858 = v0;
                let a859 = 4u64;
                if nat_lt(a858, a859) {
                    3212836864u64
                } else {
                    let a860 = v0;
                    let a861 = 5u64;
                    if nat_lt(a860, a861) {
                        2139095040u64
                    } else {
                        4286578688u64
                    }
                }
            }
        } else {
            let a862 = v0;
            let a863 = 9u64;
            if nat_lt(a862, a863) {
                let a864 = v0;
                let a865 = 7u64;
                if nat_lt(a864, a865) {
                    2143289345u64
                } else {
                    let a866 = v0;
                    let a867 = 8u64;
                    if nat_lt(a866, a867) {
                        2139095041u64
                    } else {
                        4286578689u64
                    }
                }
            } else {
                let a868 = v0;
                let a869 = 10u64;
                if nat_lt(a868, a869) {
                    1u64
                } else {
                    let a870 = v0;
                    let a871 = 11u64;
                    if nat_lt(a870, a871) {
                        2147483649u64
                    } else {
                        8388608u64
                    }
                }
            }
        }
    } else {
        let a872 = v0;
        let a873 = 18u64;
        if nat_lt(a872, a873) {
            let a874 = v0;
            let a875 = 15u64;
            if nat_lt(a874, a875) {
                let a876 = v0;
                let a877 = 13u64;
                if nat_lt(a876, a877) {
                    2139095039u64
                } else {
                    let a878 = v0;
                    let a879 = 14u64;
                    if nat_lt(a878, a879) {
                        4286578687u64
                    } else {
                        864026624u64
                    }
                }
            } else {
                let a880 = v0;
                let a881 = 16u64;
                if nat_lt(a880, a881) {
                    855638016u64
                } else {
                    let a882 = v0;
                    let a883 = 17u64;
                    if nat_lt(a882, a883) {
                        855638017u64
                    } else {
                        855638015u64
                    }
                }
            }
        } else {
            let a884 = v0;
            let a885 = 21u64;
            if nat_lt(a884, a885) {
                let a886 = v0;
                let a887 = 19u64;
                if nat_lt(a886, a887) {
                    1065357312u64
                } else {
                    let a888 = v0;
                    let a889 = 20u64;
                    if nat_lt(a888, a889) {
                        1065365504u64
                    } else {
                        1199562752u64
                    }
                }
            } else {
                let a890 = v0;
                let a891 = 22u64;
                if nat_lt(a890, a891) {
                    1199566848u64
                } else {
                    let a892 = v0;
                    let a893 = 23u64;
                    if nat_lt(a892, a893) {
                        1199566847u64
                    } else {
                        947912704u64
                    }
                }
            }
        }
    }
}

pub fn f57(v0: u64, v1: u64) -> R<u64> {
    let a894 = v0;
    let a895 = 0u64;
    if nat_eq(a894, a895) {
        Ok(f48(v0, v1))
    } else {
        Ok(f106({
            let a896 = f49(v0, v1)?;
            let a897 = 3u64;
            let a898 = 0u64;
            nat_rem(a896, a897, a898)
        }))
    }
}

pub fn f58(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a899 = v2;
    let a900 = 1u64;
    if nat_eq(a899, a900) {
        Ok(true)
    } else {
        let a901 = v0;
        let a902 = 1u64;
        if if nat_eq(a901, a902) {
            let a903 = 3u64;
            let a904 = f49(v0, v1)?;
            nat_le(a903, a904)
        } else {
            false
        } {
            let a905 = v2;
            let a906 = 3u64;
            Ok(nat_eq(a905, a906))
        } else {
            Ok(false)
        }
    }
}

pub fn f59(v0: u64, v1: u64) -> R<u64> {
    let a911 = {
        let a907 = f60(v0, v1, 0u64)?;
        let a908 = f60(v0, v1, 1u64)?;
        nat_mul(a907, a908)?
    };
    let a912 = {
        let a909 = f60(v0, v1, 2u64)?;
        let a910 = f60(v0, v1, 3u64)?;
        nat_mul(a909, a910)?
    };
    nat_mul(a911, a912)
}

pub fn f60(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a918 = {
        let a913 = f50(v0, v1);
        let a914 = 5u64;
        let a915 = 0u64;
        nat_rem(a913, a914, a915)
    };
    let a919 = {
        let a916 = v2;
        let a917 = 1u64;
        nat_add(a916, a917)?
    };
    if nat_eq(a918, a919) {
        Ok(0u64)
    } else {
        let a920 = v2;
        let a921 = 2u64;
        if nat_lt(a920, a921) {
            let a922 = v2;
            let a923 = 0u64;
            if nat_eq(a922, a923) {
                f52(v1)
            } else {
                f53(v1)
            }
        } else {
            let a924 = v2;
            let a925 = 2u64;
            if nat_eq(a924, a925) {
                Ok(2u64)
            } else {
                f54(v1)
            }
        }
    }
}

pub fn f61(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a930 = f66(v0, v1, v2)?;
    let a931 = {
        let a926 = v2;
        let a927 = 0u64;
        if if if nat_eq(a926, a927) {
            true
        } else {
            f108(v0, v1, v2)?
        } {
            let a928 = f51(v1);
            let a929 = 2u64;
            nat_eq(a928, a929)
        } else {
            false
        } {
            2u64
        } else {
            1u64
        }
    };
    nat_mul(a930, a931)
}

pub fn f62(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a934 = {
        let a932 = f61(v0, v1, v2)?;
        let a933 = f11(v0, v1, v2, 0u64)?;
        nat_mul(a932, a933)?
    };
    let a935 = f109(v0, v1, v2)?;
    nat_add(a934, a935)
}

pub fn f63(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a938 = {
        let a936 = f62(v0, v1, v2)?;
        let a937 = f11(v0, v1, v2, 1u64)?;
        nat_mul(a936, a937)?
    };
    let a939 = f109(v0, v1, v2)?;
    nat_add(a938, a939)
}

pub fn f64(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a942 = {
        let a940 = f63(v0, v1, v2)?;
        let a941 = f11(v0, v1, v2, 2u64)?;
        nat_mul(a940, a941)?
    };
    let a943 = f109(v0, v1, v2)?;
    nat_add(a942, a943)
}

pub fn f65(v0: Adt0) -> R<Adt1> {
    let a945 = match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h944, _) => {
            h944
        }
    };
    let a946 = 0u64;
    if nat_eq(a945, a946) {
        Ok(Adt1::C0)
    } else {
        let a948 = match v0.clone() {
            Adt0::C0(h947, _, _, _, _, _, _, _, _, _) => {
                h947
            }
        };
        let a949 = 0u64;
        if if if if nat_eq(a948, a949) {
            true
        } else {
            let a951 = match v0.clone() {
                Adt0::C0(_, h950, _, _, _, _, _, _, _, _) => {
                    h950
                }
            };
            let a952 = 0u64;
            nat_eq(a951, a952)
        } {
            true
        } else {
            let a954 = match v0.clone() {
                Adt0::C0(_, _, h953, _, _, _, _, _, _, _) => {
                    h953
                }
            };
            let a955 = 0u64;
            nat_eq(a954, a955)
        } {
            true
        } else {
            let a957 = match v0.clone() {
                Adt0::C0(_, _, _, h956, _, _, _, _, _, _) => {
                    h956
                }
            };
            let a958 = 0u64;
            nat_eq(a957, a958)
        } {
            Ok(Adt1::C1(0u64))
        } else {
            let a960 = match v0.clone() {
                Adt0::C0(_, _, _, _, _, _, _, _, h959, _) => {
                    h959
                }
            };
            let a961 = 1u64;
            if nat_eq(a960, a961) {
                Ok(Adt1::C1({
                    let a971 = {
                        let a969 = match v0.clone() {
                            Adt0::C0(_, _, _, _, _, _, _, _, _, h962) => {
                                h962
                            }
                        };
                        let a970 = {
                            let a967 = {
                                let a964 = match v0.clone() {
                                    Adt0::C0(h963, _, _, _, _, _, _, _, _, _) => {
                                        h963
                                    }
                                };
                                let a965 = 1u64;
                                nat_sub(a964, a965)
                            };
                            let a968 = match v0.clone() {
                                Adt0::C0(_, _, _, _, h966, _, _, _, _, _) => {
                                    h966
                                }
                            };
                            nat_mul(a967, a968)?
                        };
                        nat_add(a969, a970)?
                    };
                    let a972 = f110(v0)?;
                    nat_add(a971, a972)?
                }))
            } else {
                Ok(Adt1::C1({
                    let a981 = {
                        let a978 = {
                            let a975 = match v0.clone() {
                                Adt0::C0(h973, _, _, _, _, _, _, _, _, _) => {
                                    h973
                                }
                            };
                            let a976 = match v0.clone() {
                                Adt0::C0(_, _, _, _, h974, _, _, _, _, _) => {
                                    h974
                                }
                            };
                            nat_mul(a975, a976)?
                        };
                        let a979 = match v0.clone() {
                            Adt0::C0(_, _, _, _, _, _, _, _, h977, _) => {
                                h977
                            }
                        };
                        let a980 = 0u64;
                        nat_quot(a978, a979, a980)
                    };
                    let a982 = f110(v0)?;
                    nat_add(a981, a982)?
                }))
            }
        }
    }
}

pub fn f66(v0: u64, v1: u64, v2: u64) -> R<u64> {
    Ok(f111({
        let a983 = v2;
        let a984 = 0u64;
        if nat_eq(a983, a984) {
            f48(v0, v1)
        } else {
            f57(v0, v1)?
        }
    }))
}

pub fn f67(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a985 = v0;
    let a986 = 0u64;
    if nat_eq(a985, a986) {
        f104(v1, v2)
    } else {
        let v3: u64 = f111(f106(v1));
        let v4: u64 = {
            let a987 = v1;
            let a988 = 0u64;
            if nat_eq(a987, a988) {
                f56({
                    let a992 = {
                        let a989 = v2;
                        let a990 = v3;
                        let a991 = 0u64;
                        nat_quot(a989, a990, a991)
                    };
                    let a993 = 24u64;
                    let a994 = 0u64;
                    nat_rem(a992, a993, a994)
                })
            } else {
                f112({
                    let a998 = {
                        let a995 = v2;
                        let a996 = v3;
                        let a997 = 0u64;
                        nat_quot(a995, a996, a997)
                    };
                    let a999 = 24u64;
                    let a1000 = 0u64;
                    nat_rem(a998, a999, a1000)
                })
            }
        };
        let a1019 = {
            let a1016 = v4;
            let a1017 = {
                let a1004 = {
                    let a1001 = v2;
                    let a1002 = v3;
                    let a1003 = 0u64;
                    nat_rem(a1001, a1002, a1003)
                };
                let a1005 = 2u64;
                if nat_lt(a1004, a1005) {
                    let a1009 = {
                        let a1006 = v2;
                        let a1007 = v3;
                        let a1008 = 0u64;
                        nat_rem(a1006, a1007, a1008)
                    };
                    let a1010 = 0u64;
                    if nat_eq(a1009, a1010) {
                        1u64
                    } else {
                        256u64
                    }
                } else {
                    let a1014 = {
                        let a1011 = v2;
                        let a1012 = v3;
                        let a1013 = 0u64;
                        nat_rem(a1011, a1012, a1013)
                    };
                    let a1015 = 2u64;
                    if nat_eq(a1014, a1015) {
                        65536u64
                    } else {
                        16777216u64
                    }
                }
            };
            let a1018 = 0u64;
            nat_quot(a1016, a1017, a1018)
        };
        let a1020 = 256u64;
        let a1021 = 0u64;
        Ok(nat_rem(a1019, a1020, a1021))
    }
}

pub fn f68(v0: u64) -> R<u64> {
    f113(f114(v0))
}

pub fn f69(v0: u64) -> u64 {
    let a1025 = {
        let a1022 = v0;
        let a1023 = 72u64;
        let a1024 = 0u64;
        nat_quot(a1022, a1023, a1024)
    };
    let a1026 = 4u64;
    let a1027 = 0u64;
    nat_rem(a1025, a1026, a1027)
}

pub fn f70(v0: u64) -> u64 {
    let a1031 = {
        let a1028 = v0;
        let a1029 = 18u64;
        let a1030 = 0u64;
        nat_quot(a1028, a1029, a1030)
    };
    let a1032 = 4u64;
    let a1033 = 0u64;
    nat_rem(a1031, a1032, a1033)
}

pub fn f71(v0: u64) -> u64 {
    f115({
        let a1037 = {
            let a1034 = v0;
            let a1035 = 6u64;
            let a1036 = 0u64;
            nat_quot(a1034, a1035, a1036)
        };
        let a1038 = 3u64;
        let a1039 = 0u64;
        nat_rem(a1037, a1038, a1039)
    })
}

pub fn f72(v0: u64) -> u64 {
    f115({
        let a1043 = {
            let a1040 = v0;
            let a1041 = 2u64;
            let a1042 = 0u64;
            nat_quot(a1040, a1041, a1042)
        };
        let a1044 = 3u64;
        let a1045 = 0u64;
        nat_rem(a1043, a1044, a1045)
    })
}

pub fn f73(v0: u64) -> R<u64> {
    let a1049 = 1u64;
    let a1050 = {
        let a1046 = v0;
        let a1047 = 2u64;
        let a1048 = 0u64;
        nat_rem(a1046, a1047, a1048)
    };
    nat_add(a1049, a1050)
}

pub fn f74(v0: u64, v1: u64) -> R<u64> {
    let a1059 = {
        let a1055 = {
            let a1053 = {
                let a1051 = f114(v0);
                let a1052 = 4u64;
                nat_mul(a1051, a1052)?
            };
            let a1054 = f70(v0);
            nat_add(a1053, a1054)?
        };
        let a1056 = 2u64;
        nat_mul(a1055, a1056)?
    };
    let a1060 = {
        let a1057 = v1;
        let a1058 = 1u64;
        if nat_eq(a1057, a1058) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a1059, a1060)
}

pub fn f75(v0: u64, v1: u64) -> R<u64> {
    let a1061 = v1;
    let a1062 = 2u64;
    if nat_lt(a1061, a1062) {
        let a1063 = v1;
        let a1064 = 0u64;
        if nat_eq(a1063, a1064) {
            let a1065 = f76(v0)?;
            let a1066 = f71(v0);
            nat_mul(a1065, a1066)
        } else {
            Ok(f72(v0))
        }
    } else {
        let a1067 = v1;
        let a1068 = 2u64;
        if nat_eq(a1067, a1068) {
            Ok(2u64)
        } else {
            f73(v0)
        }
    }
}

pub fn f76(v0: u64) -> R<u64> {
    f98(f116(v0)?)
}

pub fn f77(v0: u64) -> R<u64> {
    let a1073 = {
        let a1069 = f75(v0, 0u64)?;
        let a1070 = f75(v0, 1u64)?;
        nat_mul(a1069, a1070)?
    };
    let a1074 = {
        let a1071 = f75(v0, 2u64)?;
        let a1072 = f75(v0, 3u64)?;
        nat_mul(a1071, a1072)?
    };
    nat_mul(a1073, a1074)
}

pub fn f78(v0: u64, v1: u64) -> R<u64> {
    let a1075 = v1;
    let a1076 = f69(v0);
    if nat_eq(a1075, a1076) {
        let a1077 = f70(v0);
        let a1078 = 3u64;
        if nat_eq(a1077, a1078) {
            Ok(0u64)
        } else {
            let a1081 = f75(v0, v1)?;
            let a1082 = {
                let a1079 = v1;
                let a1080 = 0u64;
                if nat_eq(a1079, a1080) {
                    f76(v0)?
                } else {
                    1u64
                }
            };
            nat_add(a1081, a1082)
        }
    } else {
        f75(v0, v1)
    }
}

pub fn f79(v0: u64, v1: u64) -> R<u64> {
    let a1089 = f83(v0)?;
    let a1090 = {
        let a1083 = v1;
        let a1084 = 0u64;
        if if if nat_eq(a1083, a1084) {
            true
        } else {
            let a1085 = v1;
            let a1086 = 2u64;
            nat_eq(a1085, a1086)
        } {
            let a1087 = f70(v0);
            let a1088 = 2u64;
            nat_eq(a1087, a1088)
        } else {
            false
        } {
            2u64
        } else {
            1u64
        }
    };
    nat_mul(a1089, a1090)
}

pub fn f80(v0: u64, v1: u64) -> R<u64> {
    let a1096 = {
        let a1094 = f79(v0, v1)?;
        let a1095 = {
            let a1091 = f18(v0, v1, 0u64)?;
            let a1092 = f76(v0)?;
            let a1093 = 0u64;
            nat_quot(a1091, a1092, a1093)
        };
        nat_mul(a1094, a1095)?
    };
    let a1097 = f117(v0, v1)?;
    nat_add(a1096, a1097)
}

pub fn f81(v0: u64, v1: u64) -> R<u64> {
    let a1100 = {
        let a1098 = f80(v0, v1)?;
        let a1099 = f18(v0, v1, 1u64)?;
        nat_mul(a1098, a1099)?
    };
    let a1101 = f117(v0, v1)?;
    nat_add(a1100, a1101)
}

pub fn f82(v0: u64, v1: u64) -> R<u64> {
    let a1104 = {
        let a1102 = f81(v0, v1)?;
        let a1103 = f18(v0, v1, 2u64)?;
        nat_mul(a1102, a1103)?
    };
    let a1105 = f117(v0, v1)?;
    nat_add(a1104, a1105)
}

pub fn f83(v0: u64) -> R<u64> {
    f103(f116(v0)?)
}

pub fn f84(v0: u64, v1: u64) -> R<u64> {
    let a1116 = {
        let a1114 = f104({
            let a1106 = v0;
            let a1107 = 2u64;
            let a1108 = 0u64;
            nat_quot(a1106, a1107, a1108)
        }, v1)?;
        let a1115 = {
            let a1112 = {
                let a1109 = v0;
                let a1110 = 2u64;
                let a1111 = 0u64;
                nat_rem(a1109, a1110, a1111)
            };
            let a1113 = 71u64;
            nat_mul(a1112, a1113)?
        };
        nat_add(a1114, a1115)?
    };
    let a1117 = 256u64;
    let a1118 = 0u64;
    Ok(nat_rem(a1116, a1117, a1118))
}

pub fn f85(v0: u64) -> R<u64> {
    Ok(f93(f28(v0)?))
}

pub fn f86(v0: u64) -> R<u64> {
    f103(f28(v0)?)
}

pub fn f87(v0: u64, v1: u64) -> R<u64> {
    let a1121 = {
        let a1119 = f86(v0)?;
        let a1120 = f24(v0, v1, 0u64)?;
        nat_mul(a1119, a1120)?
    };
    let a1122 = f118(v0, v1)?;
    nat_add(a1121, a1122)
}

pub fn f88(v0: u64, v1: u64) -> R<u64> {
    let a1125 = {
        let a1123 = f87(v0, v1)?;
        let a1124 = f24(v0, v1, 1u64)?;
        nat_mul(a1123, a1124)?
    };
    let a1126 = f118(v0, v1)?;
    nat_add(a1125, a1126)
}

pub fn f89(v0: u64, v1: u64) -> R<u64> {
    let a1129 = {
        let a1127 = f88(v0, v1)?;
        let a1128 = f24(v0, v1, 2u64)?;
        nat_mul(a1127, a1128)?
    };
    let a1130 = f118(v0, v1)?;
    nat_add(a1129, a1130)
}

pub fn f90(v0: u64) -> Adt2 {
    let a1131 = v0;
    let a1132 = 0u64;
    if nat_eq(a1131, a1132) {
        Adt2::C1(Adt3::C0(0u64, 1u64, 4u64))
    } else {
        let a1133 = v0;
        let a1134 = 1u64;
        if nat_eq(a1133, a1134) {
            Adt2::C1(Adt3::C0(1u64, 1u64, 2u64))
        } else {
            let a1135 = v0;
            let a1136 = 2u64;
            if nat_eq(a1135, a1136) {
                Adt2::C1(Adt3::C0(2u64, 32u64, 18u64))
            } else {
                let a1137 = v0;
                let a1138 = 3u64;
                if nat_eq(a1137, a1138) {
                    Adt2::C1(Adt3::C0(3u64, 32u64, 20u64))
                } else {
                    let a1139 = v0;
                    let a1140 = 6u64;
                    if nat_eq(a1139, a1140) {
                        Adt2::C1(Adt3::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1141 = v0;
                        let a1142 = 7u64;
                        if nat_eq(a1141, a1142) {
                            Adt2::C1(Adt3::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1143 = v0;
                            let a1144 = 8u64;
                            if nat_eq(a1143, a1144) {
                                Adt2::C1(Adt3::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1145 = v0;
                                let a1146 = 9u64;
                                if nat_eq(a1145, a1146) {
                                    Adt2::C1(Adt3::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1147 = v0;
                                    let a1148 = 10u64;
                                    if nat_eq(a1147, a1148) {
                                        Adt2::C1(Adt3::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1149 = v0;
                                        let a1150 = 11u64;
                                        if nat_eq(a1149, a1150) {
                                            Adt2::C1(Adt3::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1151 = v0;
                                            let a1152 = 12u64;
                                            if nat_eq(a1151, a1152) {
                                                Adt2::C1(Adt3::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1153 = v0;
                                                let a1154 = 13u64;
                                                if nat_eq(a1153, a1154) {
                                                    Adt2::C1(Adt3::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1155 = v0;
                                                    let a1156 = 14u64;
                                                    if nat_eq(a1155, a1156) {
                                                        Adt2::C1(Adt3::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1157 = v0;
                                                        let a1158 = 15u64;
                                                        if nat_eq(a1157, a1158) {
                                                            Adt2::C1(Adt3::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1159 = v0;
                                                            let a1160 = 16u64;
                                                            if nat_eq(a1159, a1160) {
                                                                Adt2::C1(Adt3::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1161 = v0;
                                                                let a1162 = 17u64;
                                                                if nat_eq(a1161, a1162) {
                                                                    Adt2::C1(Adt3::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1163 = v0;
                                                                    let a1164 = 18u64;
                                                                    if nat_eq(a1163, a1164) {
                                                                        Adt2::C1(Adt3::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1165 = v0;
                                                                        let a1166 = 19u64;
                                                                        if nat_eq(a1165, a1166) {
                                                                            Adt2::C1(Adt3::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1167 = v0;
                                                                            let a1168 = 20u64;
                                                                            if nat_eq(a1167, a1168) {
                                                                                Adt2::C1(Adt3::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1169 = v0;
                                                                                let a1170 = 21u64;
                                                                                if nat_eq(a1169, a1170) {
                                                                                    Adt2::C1(Adt3::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1171 = v0;
                                                                                    let a1172 = 22u64;
                                                                                    if nat_eq(a1171, a1172) {
                                                                                        Adt2::C1(Adt3::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1173 = v0;
                                                                                        let a1174 = 23u64;
                                                                                        if nat_eq(a1173, a1174) {
                                                                                            Adt2::C1(Adt3::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1175 = v0;
                                                                                            let a1176 = 24u64;
                                                                                            if nat_eq(a1175, a1176) {
                                                                                                Adt2::C1(Adt3::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1177 = v0;
                                                                                                let a1178 = 25u64;
                                                                                                if nat_eq(a1177, a1178) {
                                                                                                    Adt2::C1(Adt3::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1179 = v0;
                                                                                                    let a1180 = 26u64;
                                                                                                    if nat_eq(a1179, a1180) {
                                                                                                        Adt2::C1(Adt3::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1181 = v0;
                                                                                                        let a1182 = 27u64;
                                                                                                        if nat_eq(a1181, a1182) {
                                                                                                            Adt2::C1(Adt3::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1183 = v0;
                                                                                                            let a1184 = 28u64;
                                                                                                            if nat_eq(a1183, a1184) {
                                                                                                                Adt2::C1(Adt3::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1185 = v0;
                                                                                                                let a1186 = 29u64;
                                                                                                                if nat_eq(a1185, a1186) {
                                                                                                                    Adt2::C1(Adt3::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1187 = v0;
                                                                                                                    let a1188 = 30u64;
                                                                                                                    if nat_eq(a1187, a1188) {
                                                                                                                        Adt2::C1(Adt3::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1189 = v0;
                                                                                                                        let a1190 = 34u64;
                                                                                                                        if nat_eq(a1189, a1190) {
                                                                                                                            Adt2::C1(Adt3::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1191 = v0;
                                                                                                                            let a1192 = 35u64;
                                                                                                                            if nat_eq(a1191, a1192) {
                                                                                                                                Adt2::C1(Adt3::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1193 = v0;
                                                                                                                                let a1194 = 39u64;
                                                                                                                                if nat_eq(a1193, a1194) {
                                                                                                                                    Adt2::C1(Adt3::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1195 = v0;
                                                                                                                                    let a1196 = 40u64;
                                                                                                                                    if nat_eq(a1195, a1196) {
                                                                                                                                        Adt2::C1(Adt3::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1197 = v0;
                                                                                                                                        let a1198 = 41u64;
                                                                                                                                        if nat_eq(a1197, a1198) {
                                                                                                                                            Adt2::C1(Adt3::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1199 = v0;
                                                                                                                                            let a1200 = 42u64;
                                                                                                                                            if nat_eq(a1199, a1200) {
                                                                                                                                                Adt2::C1(Adt3::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1201 = v0;
                                                                                                                                                let a1202 = 43u64;
                                                                                                                                                if nat_eq(a1201, a1202) {
                                                                                                                                                    Adt2::C1(Adt3::C0(43u64, 1u64, 1u64))
                                                                                                                                                } else {
                                                                                                                                                    Adt2::C0
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

pub fn f91(v0: u64) -> R<u64> {
    f113({
        let a1203 = v0;
        let a1204 = 96u64;
        let a1205 = 0u64;
        nat_quot(a1203, a1204, a1205)
    })
}

pub fn f92(v0: u64) -> u64 {
    let a1209 = {
        let a1206 = v0;
        let a1207 = 32u64;
        let a1208 = 0u64;
        nat_quot(a1206, a1207, a1208)
    };
    let a1210 = 3u64;
    let a1211 = 0u64;
    nat_rem(a1209, a1210, a1211)
}

pub fn f93(v0: u64) -> u64 {
    let a1215 = {
        let a1212 = v0;
        let a1213 = 8u64;
        let a1214 = 0u64;
        nat_quot(a1212, a1213, a1214)
    };
    let a1216 = 4u64;
    let a1217 = 0u64;
    nat_rem(a1215, a1216, a1217)
}

pub fn f94(v0: u64) -> R<u64> {
    let a1226 = 1u64;
    let a1227 = {
        let a1224 = 2u64;
        let a1225 = {
            let a1221 = {
                let a1218 = v0;
                let a1219 = 4u64;
                let a1220 = 0u64;
                nat_quot(a1218, a1219, a1220)
            };
            let a1222 = 2u64;
            let a1223 = 0u64;
            nat_rem(a1221, a1222, a1223)
        };
        nat_mul(a1224, a1225)?
    };
    nat_add(a1226, a1227)
}

pub fn f95(v0: u64) -> R<u64> {
    let a1236 = 1u64;
    let a1237 = {
        let a1234 = 2u64;
        let a1235 = {
            let a1231 = {
                let a1228 = v0;
                let a1229 = 2u64;
                let a1230 = 0u64;
                nat_quot(a1228, a1229, a1230)
            };
            let a1232 = 2u64;
            let a1233 = 0u64;
            nat_rem(a1231, a1232, a1233)
        };
        nat_mul(a1234, a1235)?
    };
    nat_add(a1236, a1237)
}

pub fn f96(v0: u64) -> R<u64> {
    let a1241 = 1u64;
    let a1242 = {
        let a1238 = v0;
        let a1239 = 2u64;
        let a1240 = 0u64;
        nat_rem(a1238, a1239, a1240)
    };
    nat_add(a1241, a1242)
}

pub fn f97(v0: u64) -> R<u64> {
    let a1248 = {
        let a1246 = {
            let a1243 = v0;
            let a1244 = 96u64;
            let a1245 = 0u64;
            nat_quot(a1243, a1244, a1245)
        };
        let a1247 = 4u64;
        nat_mul(a1246, a1247)?
    };
    let a1249 = f93(v0);
    nat_add(a1248, a1249)
}

pub fn f98(v0: u64) -> R<u64> {
    let m1250 = f90(f91(v0)?);
    match m1250 {
        Adt2::C0 => {
            Ok(0u64)
        }
        Adt2::C1(v1) => {
            match v1 {
                Adt3::C0(_, h1251, _) => {
                    Ok(h1251)
                }
            }
        }
    }
}

pub fn f99(v0: u64, v1: u64) -> R<u64> {
    let a1256 = f103(v0)?;
    let a1257 = {
        let a1252 = v1;
        let a1253 = 0u64;
        if if nat_eq(a1252, a1253) {
            let a1254 = f93(v0);
            let a1255 = 2u64;
            nat_eq(a1254, a1255)
        } else {
            false
        } {
            2u64
        } else {
            1u64
        }
    };
    nat_mul(a1256, a1257)
}

pub fn f100(v0: u64, v1: u64) -> R<u64> {
    let a1263 = {
        let a1261 = f99(v0, v1)?;
        let a1262 = {
            let a1258 = f31(v0, v1, 0u64)?;
            let a1259 = f98(v0)?;
            let a1260 = 0u64;
            nat_quot(a1258, a1259, a1260)
        };
        nat_mul(a1261, a1262)?
    };
    let a1264 = f119(v0, v1)?;
    nat_add(a1263, a1264)
}

pub fn f101(v0: u64, v1: u64) -> R<u64> {
    let a1267 = {
        let a1265 = f100(v0, v1)?;
        let a1266 = f31(v0, v1, 1u64)?;
        nat_mul(a1265, a1266)?
    };
    let a1268 = f119(v0, v1)?;
    nat_add(a1267, a1268)
}

pub fn f102(v0: u64, v1: u64) -> R<u64> {
    let a1271 = {
        let a1269 = f101(v0, v1)?;
        let a1270 = f31(v0, v1, 2u64)?;
        nat_mul(a1269, a1270)?
    };
    let a1272 = f119(v0, v1)?;
    nat_add(a1271, a1272)
}

pub fn f103(v0: u64) -> R<u64> {
    let m1273 = f90(f91(v0)?);
    match m1273 {
        Adt2::C0 => {
            Ok(0u64)
        }
        Adt2::C1(v1) => {
            match v1 {
                Adt3::C0(_, _, h1274) => {
                    Ok(h1274)
                }
            }
        }
    }
}

pub fn f104(v0: u64, v1: u64) -> R<u64> {
    let a1289 = {
        let a1287 = {
            let a1282 = {
                let a1275 = v1;
                let a1276 = 37u64;
                nat_mul(a1275, a1276)?
            };
            let a1283 = {
                let a1280 = f113({
                    let a1277 = v0;
                    let a1278 = 4u64;
                    let a1279 = 0u64;
                    nat_quot(a1277, a1278, a1279)
                })?;
                let a1281 = 13u64;
                nat_mul(a1280, a1281)?
            };
            nat_add(a1282, a1283)?
        };
        let a1288 = {
            let a1284 = v0;
            let a1285 = 4u64;
            let a1286 = 0u64;
            nat_rem(a1284, a1285, a1286)
        };
        nat_add(a1287, a1288)?
    };
    let a1290 = 256u64;
    let a1291 = 0u64;
    Ok(nat_rem(a1289, a1290, a1291))
}

pub fn f105(v0: u64) -> u64 {
    let a1292 = v0;
    let a1293 = 96u64;
    if nat_lt(a1292, a1293) {
        let a1297 = {
            let a1294 = v0;
            let a1295 = 8u64;
            let a1296 = 0u64;
            nat_quot(a1294, a1295, a1296)
        };
        let a1298 = 3u64;
        let a1299 = 0u64;
        nat_rem(a1297, a1298, a1299)
    } else {
        let a1305 = {
            let a1302 = {
                let a1300 = v0;
                let a1301 = 96u64;
                nat_sub(a1300, a1301)
            };
            let a1303 = 2u64;
            let a1304 = 0u64;
            nat_quot(a1302, a1303, a1304)
        };
        let a1306 = 3u64;
        let a1307 = 0u64;
        nat_rem(a1305, a1306, a1307)
    }
}

pub fn f106(v0: u64) -> u64 {
    let a1308 = v0;
    let a1309 = 0u64;
    if nat_eq(a1308, a1309) {
        0u64
    } else {
        let a1310 = v0;
        let a1311 = 1u64;
        if nat_eq(a1310, a1311) {
            1u64
        } else {
            30u64
        }
    }
}

pub fn f107(v0: u64, v1: u64) -> u64 {
    let a1314 = v1;
    let a1315 = {
        let a1312 = v0;
        let a1313 = 0u64;
        if nat_eq(a1312, a1313) {
            7680u64
        } else {
            640u64
        }
    };
    let a1316 = 0u64;
    nat_quot(a1314, a1315, a1316)
}

pub fn f108(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a1317 = v0;
    let a1318 = 0u64;
    if if nat_eq(a1317, a1318) {
        let a1319 = f49(v0, v1)?;
        let a1320 = 1u64;
        nat_eq(a1319, a1320)
    } else {
        false
    } {
        let a1321 = v2;
        let a1322 = 3u64;
        Ok(nat_eq(a1321, a1322))
    } else {
        Ok(false)
    }
}

pub fn f109(v0: u64, v1: u64, v2: u64) -> R<u64> {
    if if f108(v0, v1, v2)? {
        true
    } else {
        let a1323 = v2;
        let a1324 = 0u64;
        nat_eq(a1323, a1324)
    } {
        let a1325 = f51(v1);
        let a1326 = 1u64;
        if if nat_eq(a1325, a1326) {
            true
        } else {
            let a1327 = f51(v1);
            let a1328 = 2u64;
            nat_eq(a1327, a1328)
        } {
            f66(v0, v1, v2)
        } else {
            Ok(0u64)
        }
    } else {
        f66(v0, v1, v2)
    }
}

pub fn f110(v0: Adt0) -> R<u64> {
    let a1349 = {
        let a1341 = {
            let a1333 = {
                let a1330 = match v0.clone() {
                    Adt0::C0(_, h1329, _, _, _, _, _, _, _, _) => {
                        h1329
                    }
                };
                let a1331 = 1u64;
                nat_sub(a1330, a1331)
            };
            let a1334 = match v0.clone() {
                Adt0::C0(_, _, _, _, _, h1332, _, _, _, _) => {
                    h1332
                }
            };
            nat_mul(a1333, a1334)?
        };
        let a1342 = {
            let a1339 = {
                let a1336 = match v0.clone() {
                    Adt0::C0(_, _, h1335, _, _, _, _, _, _, _) => {
                        h1335
                    }
                };
                let a1337 = 1u64;
                nat_sub(a1336, a1337)
            };
            let a1340 = match v0.clone() {
                Adt0::C0(_, _, _, _, _, _, h1338, _, _, _) => {
                    h1338
                }
            };
            nat_mul(a1339, a1340)?
        };
        nat_add(a1341, a1342)?
    };
    let a1350 = {
        let a1347 = {
            let a1344 = match v0.clone() {
                Adt0::C0(_, _, _, h1343, _, _, _, _, _, _) => {
                    h1343
                }
            };
            let a1345 = 1u64;
            nat_sub(a1344, a1345)
        };
        let a1348 = match v0 {
            Adt0::C0(_, _, _, _, _, _, _, h1346, _, _) => {
                h1346
            }
        };
        nat_mul(a1347, a1348)?
    };
    nat_add(a1349, a1350)
}

pub fn f111(v0: u64) -> u64 {
    let a1351 = v0;
    let a1352 = 0u64;
    if nat_eq(a1351, a1352) {
        4u64
    } else {
        2u64
    }
}

pub fn f112(v0: u64) -> u64 {
    let a1353 = v0;
    let a1354 = 12u64;
    if nat_lt(a1353, a1354) {
        let a1355 = v0;
        let a1356 = 6u64;
        if nat_lt(a1355, a1356) {
            let a1357 = v0;
            let a1358 = 3u64;
            if nat_lt(a1357, a1358) {
                let a1359 = v0;
                let a1360 = 1u64;
                if nat_lt(a1359, a1360) {
                    0u64
                } else {
                    let a1361 = v0;
                    let a1362 = 2u64;
                    if nat_lt(a1361, a1362) {
                        32768u64
                    } else {
                        15360u64
                    }
                }
            } else {
                let a1363 = v0;
                let a1364 = 4u64;
                if nat_lt(a1363, a1364) {
                    16256u64
                } else {
                    let a1365 = v0;
                    let a1366 = 5u64;
                    if nat_lt(a1365, a1366) {
                        48128u64
                    } else {
                        49024u64
                    }
                }
            }
        } else {
            let a1367 = v0;
            let a1368 = 9u64;
            if nat_lt(a1367, a1368) {
                let a1369 = v0;
                let a1370 = 7u64;
                if nat_lt(a1369, a1370) {
                    31744u64
                } else {
                    let a1371 = v0;
                    let a1372 = 8u64;
                    if nat_lt(a1371, a1372) {
                        32640u64
                    } else {
                        64512u64
                    }
                }
            } else {
                let a1373 = v0;
                let a1374 = 10u64;
                if nat_lt(a1373, a1374) {
                    65408u64
                } else {
                    let a1375 = v0;
                    let a1376 = 11u64;
                    if nat_lt(a1375, a1376) {
                        32256u64
                    } else {
                        32704u64
                    }
                }
            }
        }
    } else {
        let a1377 = v0;
        let a1378 = 18u64;
        if nat_lt(a1377, a1378) {
            let a1379 = v0;
            let a1380 = 15u64;
            if nat_lt(a1379, a1380) {
                let a1381 = v0;
                let a1382 = 13u64;
                if nat_lt(a1381, a1382) {
                    31745u64
                } else {
                    let a1383 = v0;
                    let a1384 = 14u64;
                    if nat_lt(a1383, a1384) {
                        32641u64
                    } else {
                        65025u64
                    }
                }
            } else {
                let a1385 = v0;
                let a1386 = 16u64;
                if nat_lt(a1385, a1386) {
                    65409u64
                } else {
                    let a1387 = v0;
                    let a1388 = 17u64;
                    if nat_lt(a1387, a1388) {
                        1u64
                    } else {
                        32769u64
                    }
                }
            }
        } else {
            let a1389 = v0;
            let a1390 = 21u64;
            if nat_lt(a1389, a1390) {
                let a1391 = v0;
                let a1392 = 19u64;
                if nat_lt(a1391, a1392) {
                    1023u64
                } else {
                    let a1393 = v0;
                    let a1394 = 20u64;
                    if nat_lt(a1393, a1394) {
                        127u64
                    } else {
                        1024u64
                    }
                }
            } else {
                let a1395 = v0;
                let a1396 = 22u64;
                if nat_lt(a1395, a1396) {
                    128u64
                } else {
                    let a1397 = v0;
                    let a1398 = 23u64;
                    if nat_lt(a1397, a1398) {
                        31743u64
                    } else {
                        32639u64
                    }
                }
            }
        }
    }
}

pub fn f113(v0: u64) -> R<u64> {
    let a1399 = v0;
    let a1400 = 4u64;
    if nat_lt(a1399, a1400) {
        Ok(v0)
    } else {
        let a1401 = v0;
        let a1402 = 29u64;
        if nat_lt(a1401, a1402) {
            let a1403 = v0;
            let a1404 = 2u64;
            nat_add(a1403, a1404)
        } else {
            let a1405 = v0;
            let a1406 = 31u64;
            if nat_lt(a1405, a1406) {
                let a1407 = v0;
                let a1408 = 5u64;
                nat_add(a1407, a1408)
            } else {
                let a1409 = v0;
                let a1410 = 8u64;
                nat_add(a1409, a1410)
            }
        }
    }
}

pub fn f114(v0: u64) -> u64 {
    let a1411 = v0;
    let a1412 = 288u64;
    let a1413 = 0u64;
    nat_quot(a1411, a1412, a1413)
}

pub fn f115(v0: u64) -> u64 {
    let a1414 = v0;
    let a1415 = 0u64;
    if nat_eq(a1414, a1415) {
        0u64
    } else {
        let a1416 = v0;
        let a1417 = 1u64;
        if nat_eq(a1416, a1417) {
            1u64
        } else {
            3u64
        }
    }
}

pub fn f116(v0: u64) -> R<u64> {
    let a1418 = f114(v0);
    let a1419 = 96u64;
    nat_mul(a1418, a1419)
}

pub fn f117(v0: u64, v1: u64) -> R<u64> {
    let a1420 = v1;
    let a1421 = 1u64;
    if if if nat_eq(a1420, a1421) {
        true
    } else {
        let a1422 = v1;
        let a1423 = 3u64;
        nat_eq(a1422, a1423)
    } {
        true
    } else {
        let a1424 = f70(v0);
        let a1425 = 1u64;
        if nat_eq(a1424, a1425) {
            true
        } else {
            let a1426 = f70(v0);
            let a1427 = 2u64;
            nat_eq(a1426, a1427)
        }
    } {
        f83(v0)
    } else {
        Ok(0u64)
    }
}

pub fn f118(v0: u64, v1: u64) -> R<u64> {
    let a1428 = v1;
    let a1429 = 3u64;
    if nat_eq(a1428, a1429) {
        f86(v0)
    } else {
        Ok(0u64)
    }
}

pub fn f119(v0: u64, v1: u64) -> R<u64> {
    let a1430 = v1;
    let a1431 = 1u64;
    if if nat_eq(a1430, a1431) {
        true
    } else {
        let a1432 = f93(v0);
        let a1433 = 1u64;
        if nat_eq(a1432, a1433) {
            true
        } else {
            let a1434 = f93(v0);
            let a1435 = 2u64;
            nat_eq(a1434, a1435)
        }
    } {
        f103(v0)
    } else {
        Ok(0u64)
    }
}
