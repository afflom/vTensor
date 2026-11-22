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
    C1(Adt2),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt3 {
    C0,
    C1(u64),
}

pub fn f0(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a1 = 5u64;
    let a2 = v0;
    if nat_le(a1, a2) {
        f1({
            let a3 = v0;
            let a4 = 5u64;
            nat_sub(a3, a4)
        }, v1, v2, v3)
    } else {
        let a5 = v0;
        let a6 = 2u64;
        if nat_lt(a5, a6) {
            let a7 = v0;
            let a8 = 0u64;
            if nat_eq(a7, a8) {
                let a9 = v3;
                let a10 = 4u64;
                if nat_lt(a9, a10) {
                    Ok(Some({
                        let a11 = v3;
                        let a12 = 2u64;
                        if nat_lt(a11, a12) {
                            let a13 = v3;
                            let a14 = 1u64;
                            if nat_lt(a13, a14) {
                                120u64
                            } else {
                                24u64
                            }
                        } else {
                            let a15 = v3;
                            let a16 = 3u64;
                            if nat_lt(a15, a16) {
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
                let a17 = v1;
                let a18 = 120u64;
                if if nat_lt(a17, a18) {
                    let a19 = v3;
                    let a20 = 24u64;
                    nat_lt(a19, a20)
                } else {
                    false
                } {
                    Ok(Some(f2(v1, v3)?))
                } else {
                    Ok(None::<u64>)
                }
            }
        } else {
            let a21 = v0;
            let a22 = 4u64;
            if if nat_lt(a21, a22) {
                let a23 = v1;
                let a24 = 120u64;
                if nat_lt(a23, a24) {
                    let a25 = v2;
                    let a26 = 2u64;
                    nat_lt(a25, a26)
                } else {
                    false
                }
            } else {
                false
            } {
                let a27 = v0;
                let a28 = 2u64;
                if nat_eq(a27, a28) {
                    let a29 = v3;
                    let a30 = 2u64;
                    if nat_lt(a29, a30) {
                        Ok(Some({
                            let a31 = v3;
                            let a32 = 0u64;
                            if nat_eq(a31, a32) {
                                f3(v1, v2)?
                            } else {
                                4u64
                            }
                        }))
                    } else {
                        Ok(None::<u64>)
                    }
                } else {
                    let a33 = v3;
                    let a34 = f3(v1, v2)?;
                    if nat_lt(a33, a34) {
                        Ok(Some(f4(v2, v3)?))
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

pub fn f1(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a35 = v0;
    let a36 = 0u64;
    if nat_eq(a35, a36) {
        let a37 = v1;
        let a38 = 0u64;
        if if if nat_eq(a37, a38) {
            let a39 = v2;
            let a40 = 0u64;
            nat_eq(a39, a40)
        } else {
            false
        } {
            let a41 = v3;
            let a42 = 5u64;
            nat_lt(a41, a42)
        } else {
            false
        } {
            Ok(Some({
                let a43 = v3;
                let a44 = 2u64;
                if nat_lt(a43, a44) {
                    let a45 = v3;
                    let a46 = 0u64;
                    if nat_eq(a45, a46) {
                        3456u64
                    } else {
                        9u64
                    }
                } else {
                    let a47 = v3;
                    let a48 = 4u64;
                    if nat_lt(a47, a48) {
                        let a49 = v3;
                        let a50 = 2u64;
                        if nat_eq(a49, a50) {
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
        let a51 = v1;
        let a52 = 3456u64;
        if if nat_lt(a51, a52) {
            f5(v1)?
        } else {
            false
        } {
            let a53 = v0;
            let a54 = 2u64;
            if nat_lt(a53, a54) {
                let a55 = v0;
                let a56 = 1u64;
                if nat_eq(a55, a56) {
                    let a57 = v2;
                    let a58 = 0u64;
                    if if nat_eq(a57, a58) {
                        let a59 = v3;
                        let a60 = 9u64;
                        nat_lt(a59, a60)
                    } else {
                        false
                    } {
                        Ok(Some(f6(v1, v3)?))
                    } else {
                        Ok(None::<u64>)
                    }
                } else {
                    Ok(None::<u64>)
                }
            } else {
                let a61 = v0;
                let a62 = 4u64;
                if nat_lt(a61, a62) {
                    let a63 = v0;
                    let a64 = 2u64;
                    if nat_eq(a63, a64) {
                        let a65 = v2;
                        let a66 = 2u64;
                        if if nat_lt(a65, a66) {
                            let a67 = v3;
                            let a68 = 10u64;
                            nat_lt(a67, a68)
                        } else {
                            false
                        } {
                            Ok(Some({
                                let a69 = v3;
                                let a70 = 4u64;
                                if nat_lt(a69, a70) {
                                    f7(v1, v2, v3)?
                                } else {
                                    let a71 = v3;
                                    let a72 = 8u64;
                                    if nat_lt(a71, a72) {
                                        f8(v1, v2, {
                                            let a73 = v3;
                                            let a74 = 4u64;
                                            nat_sub(a73, a74)
                                        })?
                                    } else {
                                        let a75 = v3;
                                        let a76 = 8u64;
                                        if nat_eq(a75, a76) {
                                            f9(v1, v2)?
                                        } else {
                                            f10(v1, v2)?
                                        }
                                    }
                                }
                            }))
                        } else {
                            Ok(None::<u64>)
                        }
                    } else {
                        let a77 = v2;
                        let a78 = 0u64;
                        if if nat_eq(a77, a78) {
                            let a79 = v3;
                            let a80 = 256u64;
                            nat_lt(a79, a80)
                        } else {
                            false
                        } {
                            Ok(Some(f11(v1, v3)?))
                        } else {
                            Ok(None::<u64>)
                        }
                    }
                } else {
                    let a81 = v0;
                    let a82 = 4u64;
                    if nat_eq(a81, a82) {
                        let a83 = v3;
                        let a84 = 2u64;
                        if nat_lt(a83, a84) {
                            let a85 = v2;
                            let a86 = 0u64;
                            if nat_eq(a85, a86) {
                                let a87 = v3;
                                let a88 = 0u64;
                                if nat_eq(a87, a88) {
                                    Ok(Some(f12(v1)?))
                                } else {
                                    Ok(None::<u64>)
                                }
                            } else {
                                let a89 = v2;
                                let a90 = f12(v1)?;
                                if nat_le(a89, a90) {
                                    Ok(Some({
                                        let a91 = v3;
                                        let a92 = 0u64;
                                        if nat_eq(a91, a92) {
                                            let a95 = {
                                                let a93 = v2;
                                                let a94 = 1u64;
                                                nat_sub(a93, a94)
                                            };
                                            let a96 = 256u64;
                                            nat_mul(a95, a96)?
                                        } else {
                                            let v4: u64 = {
                                                let a101 = f9(v1, 0u64)?;
                                                let a102 = {
                                                    let a99 = {
                                                        let a97 = v2;
                                                        let a98 = 1u64;
                                                        nat_sub(a97, a98)
                                                    };
                                                    let a100 = 256u64;
                                                    nat_mul(a99, a100)?
                                                };
                                                nat_sub(a101, a102)
                                            };
                                            let a103 = v4;
                                            let a104 = 256u64;
                                            if nat_lt(a103, a104) {
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

pub fn f2(v0: u64, v1: u64) -> R<u64> {
    let a105 = v1;
    let a106 = 12u64;
    if nat_lt(a105, a106) {
        let a107 = v1;
        let a108 = 4u64;
        if nat_lt(a107, a108) {
            let a109 = v1;
            let a110 = 2u64;
            if nat_lt(a109, a110) {
                let a111 = v1;
                let a112 = 1u64;
                if nat_lt(a111, a112) {
                    f13(v0)
                } else {
                    let a113 = f13(v0)?;
                    let a114 = 4u64;
                    if nat_lt(a113, a114) {
                        let a115 = f13(v0)?;
                        let a116 = 2u64;
                        if nat_lt(a115, a116) {
                            let a117 = f13(v0)?;
                            let a118 = 1u64;
                            if nat_lt(a117, a118) {
                                Ok(2u64)
                            } else {
                                Ok(5u64)
                            }
                        } else {
                            let a119 = f13(v0)?;
                            let a120 = 3u64;
                            if nat_lt(a119, a120) {
                                Ok(6u64)
                            } else {
                                Ok(7u64)
                            }
                        }
                    } else {
                        let a121 = f13(v0)?;
                        let a122 = 6u64;
                        if nat_lt(a121, a122) {
                            let a123 = f13(v0)?;
                            let a124 = 5u64;
                            if nat_lt(a123, a124) {
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
                let a125 = v1;
                let a126 = 3u64;
                if nat_lt(a125, a126) {
                    let a127 = f13(v0)?;
                    let a128 = 6u64;
                    if nat_eq(a127, a128) {
                        Ok(2u64)
                    } else {
                        Ok(0u64)
                    }
                } else {
                    Ok(f14(v0))
                }
            }
        } else {
            Ok(f15(v0, {
                let a131 = {
                    let a129 = v1;
                    let a130 = 4u64;
                    nat_sub(a129, a130)
                };
                let a132 = 4u64;
                let a133 = 0u64;
                nat_quot(a131, a132, a133)
            }, {
                let a136 = {
                    let a134 = v1;
                    let a135 = 4u64;
                    nat_sub(a134, a135)
                };
                let a137 = 4u64;
                let a138 = 0u64;
                nat_rem(a136, a137, a138)
            }))
        }
    } else {
        let a139 = v1;
        let a140 = 20u64;
        if nat_lt(a139, a140) {
            f16(v0, {
                let a143 = {
                    let a141 = v1;
                    let a142 = 12u64;
                    nat_sub(a141, a142)
                };
                let a144 = 4u64;
                let a145 = 0u64;
                nat_quot(a143, a144, a145)
            }, {
                let a148 = {
                    let a146 = v1;
                    let a147 = 12u64;
                    nat_sub(a146, a147)
                };
                let a149 = 4u64;
                let a150 = 0u64;
                nat_rem(a148, a149, a150)
            })
        } else {
            let a151 = v1;
            let a152 = 22u64;
            if nat_lt(a151, a152) {
                f3(v0, {
                    let a153 = v1;
                    let a154 = 20u64;
                    nat_sub(a153, a154)
                })
            } else {
                let a155 = v1;
                let a156 = 22u64;
                if nat_eq(a155, a156) {
                    let a157 = v0;
                    let a158 = 0u64;
                    if nat_eq(a157, a158) {
                        Ok(1u64)
                    } else {
                        let a159 = v0;
                        let a160 = 108u64;
                        if nat_eq(a159, a160) {
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

pub fn f3(v0: u64, v1: u64) -> R<u64> {
    let a161 = v1;
    let a162 = 0u64;
    if nat_eq(a161, a162) {
        let a163 = f17(v0);
        let a164 = 12u64;
        nat_mul(a163, a164)
    } else {
        let a165 = 96u64;
        let a166 = v0;
        if nat_le(a165, a166) {
            Ok(0u64)
        } else {
            let a167 = f18(v0);
            let a168 = 2u64;
            if nat_lt(a167, a168) {
                let a169 = f18(v0);
                let a170 = 1u64;
                if nat_lt(a169, a170) {
                    let a171 = f17(v0);
                    let a172 = 12u64;
                    nat_mul(a171, a172)
                } else {
                    Ok(1u64)
                }
            } else {
                let a173 = f18(v0);
                let a174 = 3u64;
                if nat_lt(a173, a174) {
                    Ok(f17(v0))
                } else {
                    Ok(6u64)
                }
            }
        }
    }
}

pub fn f4(v0: u64, v1: u64) -> R<u64> {
    let a175 = v1;
    let a176 = 16u64;
    if nat_lt(a175, a176) {
        Ok(f19(f20(v0, v1)?))
    } else {
        Ok(f21({
            let a177 = f22(v0);
            let a178 = f23(v1)?;
            nat_add(a177, a178)?
        }))
    }
}

pub fn f5(v0: u64) -> R<bool> {
    let m179 = f24(f25(v0)?);
    match m179 {
        Adt1::C0 => {
            Ok(false)
        }
        Adt1::C1(_) => {
            Ok(true)
        }
    }
}

pub fn f6(v0: u64, v1: u64) -> R<u64> {
    let a180 = v1;
    let a181 = 0u64;
    if nat_eq(a180, a181) {
        f25(v0)
    } else {
        let a182 = v1;
        let a183 = 1u64;
        if nat_eq(a182, a183) {
            Ok(f26(v0))
        } else {
            let a184 = v1;
            let a185 = 2u64;
            if nat_eq(a184, a185) {
                Ok(0u64)
            } else {
                let a186 = v1;
                let a187 = 3u64;
                if nat_eq(a186, a187) {
                    Ok(f27(v0))
                } else {
                    let a188 = v1;
                    let a189 = 4u64;
                    if nat_eq(a188, a189) {
                        f28(v0)
                    } else {
                        let a190 = v1;
                        let a191 = 5u64;
                        if nat_eq(a190, a191) {
                            f29(v0)
                        } else {
                            let a192 = v1;
                            let a193 = 6u64;
                            if nat_eq(a192, a193) {
                                f30(v0)
                            } else {
                                let a194 = v1;
                                let a195 = 7u64;
                                if nat_eq(a194, a195) {
                                    f31(v0)
                                } else {
                                    let a196 = f26(v0);
                                    let a197 = 1u64;
                                    if if nat_eq(a196, a197) {
                                        let a198 = f27(v0);
                                        let a199 = 3u64;
                                        nat_eq(a198, a199)
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

pub fn f7(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a200 = v1;
    let a201 = 1u64;
    if if nat_eq(a200, a201) {
        let a202 = f27(v0);
        let a203 = 3u64;
        nat_eq(a202, a203)
    } else {
        false
    } {
        let a204 = v2;
        let a205 = 2u64;
        if nat_lt(a204, a205) {
            let a206 = v2;
            let a207 = 0u64;
            if nat_eq(a206, a207) {
                f32(v0)
            } else {
                let a212 = {
                    let a208 = f28(v0)?;
                    let a209 = f29(v0)?;
                    nat_mul(a208, a209)?
                };
                let a213 = {
                    let a210 = 2u64;
                    let a211 = f30(v0)?;
                    nat_mul(a210, a211)?
                };
                nat_mul(a212, a213)
            }
        } else {
            Ok(1u64)
        }
    } else {
        let a214 = v2;
        let a215 = 2u64;
        if nat_lt(a214, a215) {
            let a216 = v2;
            let a217 = 0u64;
            if nat_eq(a216, a217) {
                let a218 = f32(v0)?;
                let a219 = f28(v0)?;
                nat_mul(a218, a219)
            } else {
                f29(v0)
            }
        } else {
            let a220 = v2;
            let a221 = 2u64;
            if nat_eq(a220, a221) {
                Ok(2u64)
            } else {
                f30(v0)
            }
        }
    }
}

pub fn f8(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a222 = v2;
    let a223 = 2u64;
    if nat_lt(a222, a223) {
        let a224 = v2;
        let a225 = 0u64;
        if nat_eq(a224, a225) {
            f33(v0, v1)
        } else {
            f34(v0, v1)
        }
    } else {
        let a226 = v2;
        let a227 = 2u64;
        if nat_eq(a226, a227) {
            f35(v0, v1)
        } else {
            f36(v0, v1)
        }
    }
}

pub fn f9(v0: u64, v1: u64) -> R<u64> {
    let m228 = f37(Adt0::C0(f7(v0, v1, 0u64)?, f7(v0, v1, 1u64)?, f7(v0, v1, 2u64)?, f7(v0, v1, 3u64)?, f8(v0, v1, 0u64)?, f8(v0, v1, 1u64)?, f8(v0, v1, 2u64)?, f8(v0, v1, 3u64)?, f32(v0)?, f38(v0)?))?;
    match m228 {
        Adt3::C0 => {
            Ok(0u64)
        }
        Adt3::C1(v2) => {
            Ok(v2)
        }
    }
}

pub fn f10(v0: u64, v1: u64) -> R<u64> {
    let a233 = {
        let a229 = f7(v0, v1, 0u64)?;
        let a230 = f7(v0, v1, 1u64)?;
        nat_mul(a229, a230)?
    };
    let a234 = {
        let a231 = f7(v0, v1, 2u64)?;
        let a232 = f7(v0, v1, 3u64)?;
        nat_mul(a231, a232)?
    };
    nat_mul(a233, a234)
}

pub fn f11(v0: u64, v1: u64) -> R<u64> {
    f39(f31(v0)?, v1)
}

pub fn f12(v0: u64) -> R<u64> {
    let a237 = {
        let a235 = f9(v0, 0u64)?;
        let a236 = 255u64;
        nat_add(a235, a236)?
    };
    let a238 = 256u64;
    let a239 = 0u64;
    Ok(nat_quot(a237, a238, a239))
}

pub fn f13(v0: u64) -> R<u64> {
    let a240 = v0;
    let a241 = 96u64;
    if nat_lt(a240, a241) {
        let a242 = v0;
        let a243 = 24u64;
        let a244 = 0u64;
        Ok(nat_quot(a242, a243, a244))
    } else {
        let a250 = 4u64;
        let a251 = {
            let a247 = {
                let a245 = v0;
                let a246 = 96u64;
                nat_sub(a245, a246)
            };
            let a248 = 6u64;
            let a249 = 0u64;
            nat_quot(a247, a248, a249)
        };
        nat_add(a250, a251)
    }
}

pub fn f14(v0: u64) -> u64 {
    let a252 = v0;
    let a253 = 96u64;
    if nat_lt(a252, a253) {
        let a257 = {
            let a254 = v0;
            let a255 = 4u64;
            let a256 = 0u64;
            nat_quot(a254, a255, a256)
        };
        let a258 = 2u64;
        let a259 = 0u64;
        nat_rem(a257, a258, a259)
    } else {
        let a260 = v0;
        let a261 = 2u64;
        let a262 = 0u64;
        nat_rem(a260, a261, a262)
    }
}

pub fn f15(v0: u64, v1: u64, v2: u64) -> u64 {
    let a263 = v1;
    let a264 = 0u64;
    if nat_eq(a263, a264) {
        let a265 = v2;
        let a266 = 2u64;
        if nat_lt(a265, a266) {
            let a267 = v2;
            let a268 = 1u64;
            if nat_lt(a267, a268) {
                f17(v0)
            } else {
                3u64
            }
        } else {
            2u64
        }
    } else {
        let a269 = 96u64;
        let a270 = v0;
        if nat_le(a269, a270) {
            0u64
        } else {
            let a271 = v2;
            let a272 = 2u64;
            if nat_lt(a271, a272) {
                let a273 = v2;
                let a274 = 1u64;
                if nat_lt(a273, a274) {
                    let a275 = f18(v0);
                    let a276 = 1u64;
                    if if nat_eq(a275, a276) {
                        true
                    } else {
                        let a277 = f18(v0);
                        let a278 = 3u64;
                        nat_eq(a277, a278)
                    } {
                        1u64
                    } else {
                        f17(v0)
                    }
                } else {
                    let a279 = f18(v0);
                    let a280 = 1u64;
                    if if nat_eq(a279, a280) {
                        true
                    } else {
                        let a281 = f18(v0);
                        let a282 = 2u64;
                        nat_eq(a281, a282)
                    } {
                        1u64
                    } else {
                        3u64
                    }
                }
            } else {
                let a283 = v2;
                let a284 = 3u64;
                if nat_lt(a283, a284) {
                    let a285 = f18(v0);
                    let a286 = 0u64;
                    if nat_eq(a285, a286) {
                        2u64
                    } else {
                        1u64
                    }
                } else {
                    let a287 = f18(v0);
                    let a288 = 1u64;
                    if if nat_eq(a287, a288) {
                        true
                    } else {
                        let a289 = f18(v0);
                        let a290 = 2u64;
                        nat_eq(a289, a290)
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

pub fn f16(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a291 = v1;
    let a292 = 1u64;
    if if nat_eq(a291, a292) {
        let a293 = 96u64;
        let a294 = v0;
        nat_le(a293, a294)
    } else {
        false
    } {
        Ok(0u64)
    } else {
        let v3: u64 = f17(v0);
        let v4: u64 = f14(v0);
        let v5: u64 = {
            let a295 = v1;
            let a296 = 0u64;
            if nat_eq(a295, a296) {
                0u64
            } else {
                f18(v0)
            }
        };
        let a297 = v5;
        let a298 = 2u64;
        if nat_lt(a297, a298) {
            let a299 = v5;
            let a300 = 1u64;
            if nat_lt(a299, a300) {
                let a301 = v2;
                let a302 = 2u64;
                if nat_lt(a301, a302) {
                    let a303 = v2;
                    let a304 = 1u64;
                    if nat_lt(a303, a304) {
                        Ok(4u64)
                    } else {
                        let a307 = {
                            let a305 = v3;
                            let a306 = v4;
                            nat_add(a305, a306)?
                        };
                        let a308 = 4u64;
                        nat_mul(a307, a308)
                    }
                } else {
                    let a309 = v2;
                    let a310 = 3u64;
                    if nat_lt(a309, a310) {
                        let a317 = {
                            let a315 = {
                                let a311 = v3;
                                let a312 = 3u64;
                                nat_mul(a311, a312)?
                            };
                            let a316 = {
                                let a313 = v4;
                                let a314 = 4u64;
                                nat_mul(a313, a314)?
                            };
                            nat_add(a315, a316)?
                        };
                        let a318 = 4u64;
                        nat_mul(a317, a318)
                    } else {
                        let a325 = {
                            let a323 = {
                                let a319 = v3;
                                let a320 = 6u64;
                                nat_mul(a319, a320)?
                            };
                            let a324 = {
                                let a321 = v4;
                                let a322 = 9u64;
                                nat_mul(a321, a322)?
                            };
                            nat_add(a323, a324)?
                        };
                        let a326 = 4u64;
                        nat_mul(a325, a326)
                    }
                }
            } else {
                let a327 = v2;
                let a328 = 2u64;
                if nat_lt(a327, a328) {
                    let a329 = v2;
                    let a330 = 1u64;
                    if nat_lt(a329, a330) {
                        Ok(4u64)
                    } else {
                        let a333 = {
                            let a331 = 1u64;
                            let a332 = v4;
                            nat_add(a331, a332)?
                        };
                        let a334 = 4u64;
                        nat_mul(a333, a334)
                    }
                } else {
                    let a335 = v2;
                    let a336 = 3u64;
                    if nat_lt(a335, a336) {
                        let a341 = {
                            let a339 = 1u64;
                            let a340 = {
                                let a337 = v4;
                                let a338 = 2u64;
                                nat_mul(a337, a338)?
                            };
                            nat_add(a339, a340)?
                        };
                        let a342 = 4u64;
                        nat_mul(a341, a342)
                    } else {
                        let a347 = {
                            let a345 = 1u64;
                            let a346 = {
                                let a343 = v4;
                                let a344 = 3u64;
                                nat_mul(a343, a344)?
                            };
                            nat_add(a345, a346)?
                        };
                        let a348 = 4u64;
                        nat_mul(a347, a348)
                    }
                }
            }
        } else {
            let a349 = v5;
            let a350 = 3u64;
            if nat_lt(a349, a350) {
                let a351 = v2;
                let a352 = 2u64;
                if nat_lt(a351, a352) {
                    let a353 = v2;
                    let a354 = 1u64;
                    if nat_lt(a353, a354) {
                        Ok(4u64)
                    } else {
                        let a357 = {
                            let a355 = v3;
                            let a356 = v4;
                            nat_add(a355, a356)?
                        };
                        let a358 = 4u64;
                        nat_mul(a357, a358)
                    }
                } else {
                    let a359 = v2;
                    let a360 = 3u64;
                    if nat_lt(a359, a360) {
                        let a365 = {
                            let a363 = v3;
                            let a364 = {
                                let a361 = v4;
                                let a362 = 2u64;
                                nat_mul(a361, a362)?
                            };
                            nat_add(a363, a364)?
                        };
                        let a366 = 4u64;
                        nat_mul(a365, a366)
                    } else {
                        let a371 = {
                            let a369 = v3;
                            let a370 = {
                                let a367 = v4;
                                let a368 = 3u64;
                                nat_mul(a367, a368)?
                            };
                            nat_add(a369, a370)?
                        };
                        let a372 = 4u64;
                        nat_mul(a371, a372)
                    }
                }
            } else {
                let a373 = v2;
                let a374 = 2u64;
                if nat_lt(a373, a374) {
                    let a375 = v2;
                    let a376 = 1u64;
                    if nat_lt(a375, a376) {
                        Ok(4u64)
                    } else {
                        let a379 = {
                            let a377 = 1u64;
                            let a378 = v4;
                            nat_add(a377, a378)?
                        };
                        let a380 = 4u64;
                        nat_mul(a379, a380)
                    }
                } else {
                    let a381 = v2;
                    let a382 = 3u64;
                    if nat_lt(a381, a382) {
                        let a387 = {
                            let a385 = 3u64;
                            let a386 = {
                                let a383 = v4;
                                let a384 = 4u64;
                                nat_mul(a383, a384)?
                            };
                            nat_add(a385, a386)?
                        };
                        let a388 = 4u64;
                        nat_mul(a387, a388)
                    } else {
                        let a393 = {
                            let a391 = 3u64;
                            let a392 = {
                                let a389 = v4;
                                let a390 = 5u64;
                                nat_mul(a389, a390)?
                            };
                            nat_add(a391, a392)?
                        };
                        let a394 = 4u64;
                        nat_mul(a393, a394)
                    }
                }
            }
        }
    }
}

pub fn f17(v0: u64) -> u64 {
    let a395 = f40(v0);
    let a396 = 0u64;
    if nat_eq(a395, a396) {
        1u64
    } else {
        let a397 = f40(v0);
        let a398 = 1u64;
        if nat_eq(a397, a398) {
            17u64
        } else {
            257u64
        }
    }
}

pub fn f18(v0: u64) -> u64 {
    let a399 = v0;
    let a400 = 4u64;
    let a401 = 0u64;
    nat_rem(a399, a400, a401)
}

pub fn f19(v0: u64) -> u64 {
    let a402 = v0;
    let a403 = 8u64;
    if nat_lt(a402, a403) {
        let a404 = v0;
        let a405 = 4u64;
        if nat_lt(a404, a405) {
            let a406 = v0;
            let a407 = 2u64;
            if nat_lt(a406, a407) {
                let a408 = v0;
                let a409 = 1u64;
                if nat_lt(a408, a409) {
                    0u64
                } else {
                    2147483648u64
                }
            } else {
                let a410 = v0;
                let a411 = 3u64;
                if nat_lt(a410, a411) {
                    1065353216u64
                } else {
                    3212836864u64
                }
            }
        } else {
            let a412 = v0;
            let a413 = 6u64;
            if nat_lt(a412, a413) {
                let a414 = v0;
                let a415 = 5u64;
                if nat_lt(a414, a415) {
                    2139095040u64
                } else {
                    4286578688u64
                }
            } else {
                let a416 = v0;
                let a417 = 7u64;
                if nat_lt(a416, a417) {
                    2143289345u64
                } else {
                    1u64
                }
            }
        }
    } else {
        let a418 = v0;
        let a419 = 12u64;
        if nat_lt(a418, a419) {
            let a420 = v0;
            let a421 = 10u64;
            if nat_lt(a420, a421) {
                let a422 = v0;
                let a423 = 9u64;
                if nat_lt(a422, a423) {
                    2139095039u64
                } else {
                    8388608u64
                }
            } else {
                let a424 = v0;
                let a425 = 11u64;
                if nat_lt(a424, a425) {
                    8388607u64
                } else {
                    1065353217u64
                }
            }
        } else {
            let a426 = v0;
            let a427 = 14u64;
            if nat_lt(a426, a427) {
                let a428 = v0;
                let a429 = 13u64;
                if nat_lt(a428, a429) {
                    1056964608u64
                } else {
                    8388609u64
                }
            } else {
                let a430 = v0;
                let a431 = 15u64;
                if nat_lt(a430, a431) {
                    2147483649u64
                } else {
                    2139095041u64
                }
            }
        }
    }
}

pub fn f20(v0: u64, v1: u64) -> R<u64> {
    let a440 = {
        let a438 = {
            let a434 = {
                let a432 = v0;
                let a433 = 0u64;
                if nat_eq(a432, a433) {
                    1u64
                } else {
                    5u64
                }
            };
            let a435 = v1;
            nat_mul(a434, a435)?
        };
        let a439 = {
            let a436 = v0;
            let a437 = 0u64;
            if nat_eq(a436, a437) {
                0u64
            } else {
                3u64
            }
        };
        nat_add(a438, a439)?
    };
    let a441 = 16u64;
    let a442 = 0u64;
    Ok(nat_rem(a440, a441, a442))
}

pub fn f21(v0: u64) -> u64 {
    let a443 = v0;
    let a444 = 4294967296u64;
    let a445 = 0u64;
    nat_rem(a443, a444, a445)
}

pub fn f22(v0: u64) -> u64 {
    let a446 = v0;
    let a447 = 0u64;
    if nat_eq(a446, a447) {
        2654435769u64
    } else {
        2246822507u64
    }
}

pub fn f23(v0: u64) -> R<u64> {
    let a448 = 2654435761u64;
    let a449 = v0;
    nat_mul(a448, a449)
}

pub fn f24(v0: u64) -> Adt1 {
    let a450 = v0;
    let a451 = 0u64;
    if nat_eq(a450, a451) {
        Adt1::C1(Adt2::C0(0u64, 1u64, 4u64))
    } else {
        let a452 = v0;
        let a453 = 1u64;
        if nat_eq(a452, a453) {
            Adt1::C1(Adt2::C0(1u64, 1u64, 2u64))
        } else {
            let a454 = v0;
            let a455 = 2u64;
            if nat_eq(a454, a455) {
                Adt1::C1(Adt2::C0(2u64, 32u64, 18u64))
            } else {
                let a456 = v0;
                let a457 = 3u64;
                if nat_eq(a456, a457) {
                    Adt1::C1(Adt2::C0(3u64, 32u64, 20u64))
                } else {
                    let a458 = v0;
                    let a459 = 6u64;
                    if nat_eq(a458, a459) {
                        Adt1::C1(Adt2::C0(6u64, 32u64, 22u64))
                    } else {
                        let a460 = v0;
                        let a461 = 7u64;
                        if nat_eq(a460, a461) {
                            Adt1::C1(Adt2::C0(7u64, 32u64, 24u64))
                        } else {
                            let a462 = v0;
                            let a463 = 8u64;
                            if nat_eq(a462, a463) {
                                Adt1::C1(Adt2::C0(8u64, 32u64, 34u64))
                            } else {
                                let a464 = v0;
                                let a465 = 9u64;
                                if nat_eq(a464, a465) {
                                    Adt1::C1(Adt2::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a466 = v0;
                                    let a467 = 10u64;
                                    if nat_eq(a466, a467) {
                                        Adt1::C1(Adt2::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a468 = v0;
                                        let a469 = 11u64;
                                        if nat_eq(a468, a469) {
                                            Adt1::C1(Adt2::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a470 = v0;
                                            let a471 = 12u64;
                                            if nat_eq(a470, a471) {
                                                Adt1::C1(Adt2::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a472 = v0;
                                                let a473 = 13u64;
                                                if nat_eq(a472, a473) {
                                                    Adt1::C1(Adt2::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a474 = v0;
                                                    let a475 = 14u64;
                                                    if nat_eq(a474, a475) {
                                                        Adt1::C1(Adt2::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a476 = v0;
                                                        let a477 = 15u64;
                                                        if nat_eq(a476, a477) {
                                                            Adt1::C1(Adt2::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a478 = v0;
                                                            let a479 = 16u64;
                                                            if nat_eq(a478, a479) {
                                                                Adt1::C1(Adt2::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a480 = v0;
                                                                let a481 = 17u64;
                                                                if nat_eq(a480, a481) {
                                                                    Adt1::C1(Adt2::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a482 = v0;
                                                                    let a483 = 18u64;
                                                                    if nat_eq(a482, a483) {
                                                                        Adt1::C1(Adt2::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a484 = v0;
                                                                        let a485 = 19u64;
                                                                        if nat_eq(a484, a485) {
                                                                            Adt1::C1(Adt2::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a486 = v0;
                                                                            let a487 = 20u64;
                                                                            if nat_eq(a486, a487) {
                                                                                Adt1::C1(Adt2::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a488 = v0;
                                                                                let a489 = 21u64;
                                                                                if nat_eq(a488, a489) {
                                                                                    Adt1::C1(Adt2::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a490 = v0;
                                                                                    let a491 = 22u64;
                                                                                    if nat_eq(a490, a491) {
                                                                                        Adt1::C1(Adt2::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a492 = v0;
                                                                                        let a493 = 23u64;
                                                                                        if nat_eq(a492, a493) {
                                                                                            Adt1::C1(Adt2::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a494 = v0;
                                                                                            let a495 = 24u64;
                                                                                            if nat_eq(a494, a495) {
                                                                                                Adt1::C1(Adt2::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a496 = v0;
                                                                                                let a497 = 25u64;
                                                                                                if nat_eq(a496, a497) {
                                                                                                    Adt1::C1(Adt2::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a498 = v0;
                                                                                                    let a499 = 26u64;
                                                                                                    if nat_eq(a498, a499) {
                                                                                                        Adt1::C1(Adt2::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a500 = v0;
                                                                                                        let a501 = 27u64;
                                                                                                        if nat_eq(a500, a501) {
                                                                                                            Adt1::C1(Adt2::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a502 = v0;
                                                                                                            let a503 = 28u64;
                                                                                                            if nat_eq(a502, a503) {
                                                                                                                Adt1::C1(Adt2::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a504 = v0;
                                                                                                                let a505 = 29u64;
                                                                                                                if nat_eq(a504, a505) {
                                                                                                                    Adt1::C1(Adt2::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a506 = v0;
                                                                                                                    let a507 = 30u64;
                                                                                                                    if nat_eq(a506, a507) {
                                                                                                                        Adt1::C1(Adt2::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a508 = v0;
                                                                                                                        let a509 = 34u64;
                                                                                                                        if nat_eq(a508, a509) {
                                                                                                                            Adt1::C1(Adt2::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a510 = v0;
                                                                                                                            let a511 = 35u64;
                                                                                                                            if nat_eq(a510, a511) {
                                                                                                                                Adt1::C1(Adt2::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a512 = v0;
                                                                                                                                let a513 = 39u64;
                                                                                                                                if nat_eq(a512, a513) {
                                                                                                                                    Adt1::C1(Adt2::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a514 = v0;
                                                                                                                                    let a515 = 40u64;
                                                                                                                                    if nat_eq(a514, a515) {
                                                                                                                                        Adt1::C1(Adt2::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a516 = v0;
                                                                                                                                        let a517 = 41u64;
                                                                                                                                        if nat_eq(a516, a517) {
                                                                                                                                            Adt1::C1(Adt2::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a518 = v0;
                                                                                                                                            let a519 = 42u64;
                                                                                                                                            if nat_eq(a518, a519) {
                                                                                                                                                Adt1::C1(Adt2::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a520 = v0;
                                                                                                                                                let a521 = 43u64;
                                                                                                                                                if nat_eq(a520, a521) {
                                                                                                                                                    Adt1::C1(Adt2::C0(43u64, 1u64, 1u64))
                                                                                                                                                } else {
                                                                                                                                                    Adt1::C0
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

pub fn f25(v0: u64) -> R<u64> {
    f41({
        let a522 = v0;
        let a523 = 96u64;
        let a524 = 0u64;
        nat_quot(a522, a523, a524)
    })
}

pub fn f26(v0: u64) -> u64 {
    let a528 = {
        let a525 = v0;
        let a526 = 32u64;
        let a527 = 0u64;
        nat_quot(a525, a526, a527)
    };
    let a529 = 3u64;
    let a530 = 0u64;
    nat_rem(a528, a529, a530)
}

pub fn f27(v0: u64) -> u64 {
    let a534 = {
        let a531 = v0;
        let a532 = 8u64;
        let a533 = 0u64;
        nat_quot(a531, a532, a533)
    };
    let a535 = 4u64;
    let a536 = 0u64;
    nat_rem(a534, a535, a536)
}

pub fn f28(v0: u64) -> R<u64> {
    let a545 = 1u64;
    let a546 = {
        let a543 = 2u64;
        let a544 = {
            let a540 = {
                let a537 = v0;
                let a538 = 4u64;
                let a539 = 0u64;
                nat_quot(a537, a538, a539)
            };
            let a541 = 2u64;
            let a542 = 0u64;
            nat_rem(a540, a541, a542)
        };
        nat_mul(a543, a544)?
    };
    nat_add(a545, a546)
}

pub fn f29(v0: u64) -> R<u64> {
    let a555 = 1u64;
    let a556 = {
        let a553 = 2u64;
        let a554 = {
            let a550 = {
                let a547 = v0;
                let a548 = 2u64;
                let a549 = 0u64;
                nat_quot(a547, a548, a549)
            };
            let a551 = 2u64;
            let a552 = 0u64;
            nat_rem(a550, a551, a552)
        };
        nat_mul(a553, a554)?
    };
    nat_add(a555, a556)
}

pub fn f30(v0: u64) -> R<u64> {
    let a560 = 1u64;
    let a561 = {
        let a557 = v0;
        let a558 = 2u64;
        let a559 = 0u64;
        nat_rem(a557, a558, a559)
    };
    nat_add(a560, a561)
}

pub fn f31(v0: u64) -> R<u64> {
    let a567 = {
        let a565 = {
            let a562 = v0;
            let a563 = 96u64;
            let a564 = 0u64;
            nat_quot(a562, a563, a564)
        };
        let a566 = 4u64;
        nat_mul(a565, a566)?
    };
    let a568 = f27(v0);
    nat_add(a567, a568)
}

pub fn f32(v0: u64) -> R<u64> {
    let m569 = f24(f25(v0)?);
    match m569 {
        Adt1::C0 => {
            Ok(0u64)
        }
        Adt1::C1(v1) => {
            match v1 {
                Adt2::C0(_, h570, _) => {
                    Ok(h570)
                }
            }
        }
    }
}

pub fn f33(v0: u64, v1: u64) -> R<u64> {
    let a575 = f38(v0)?;
    let a576 = {
        let a571 = v1;
        let a572 = 0u64;
        if if nat_eq(a571, a572) {
            let a573 = f27(v0);
            let a574 = 2u64;
            nat_eq(a573, a574)
        } else {
            false
        } {
            2u64
        } else {
            1u64
        }
    };
    nat_mul(a575, a576)
}

pub fn f34(v0: u64, v1: u64) -> R<u64> {
    let a582 = {
        let a580 = f33(v0, v1)?;
        let a581 = {
            let a577 = f7(v0, v1, 0u64)?;
            let a578 = f32(v0)?;
            let a579 = 0u64;
            nat_quot(a577, a578, a579)
        };
        nat_mul(a580, a581)?
    };
    let a583 = f42(v0, v1)?;
    nat_add(a582, a583)
}

pub fn f35(v0: u64, v1: u64) -> R<u64> {
    let a586 = {
        let a584 = f34(v0, v1)?;
        let a585 = f7(v0, v1, 1u64)?;
        nat_mul(a584, a585)?
    };
    let a587 = f42(v0, v1)?;
    nat_add(a586, a587)
}

pub fn f36(v0: u64, v1: u64) -> R<u64> {
    let a590 = {
        let a588 = f35(v0, v1)?;
        let a589 = f7(v0, v1, 2u64)?;
        nat_mul(a588, a589)?
    };
    let a591 = f42(v0, v1)?;
    nat_add(a590, a591)
}

pub fn f37(v0: Adt0) -> R<Adt3> {
    let a593 = match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h592, _) => {
            h592
        }
    };
    let a594 = 0u64;
    if nat_eq(a593, a594) {
        Ok(Adt3::C0)
    } else {
        let a596 = match v0.clone() {
            Adt0::C0(h595, _, _, _, _, _, _, _, _, _) => {
                h595
            }
        };
        let a597 = 0u64;
        if if if if nat_eq(a596, a597) {
            true
        } else {
            let a599 = match v0.clone() {
                Adt0::C0(_, h598, _, _, _, _, _, _, _, _) => {
                    h598
                }
            };
            let a600 = 0u64;
            nat_eq(a599, a600)
        } {
            true
        } else {
            let a602 = match v0.clone() {
                Adt0::C0(_, _, h601, _, _, _, _, _, _, _) => {
                    h601
                }
            };
            let a603 = 0u64;
            nat_eq(a602, a603)
        } {
            true
        } else {
            let a605 = match v0.clone() {
                Adt0::C0(_, _, _, h604, _, _, _, _, _, _) => {
                    h604
                }
            };
            let a606 = 0u64;
            nat_eq(a605, a606)
        } {
            Ok(Adt3::C1(0u64))
        } else {
            let a608 = match v0.clone() {
                Adt0::C0(_, _, _, _, _, _, _, _, h607, _) => {
                    h607
                }
            };
            let a609 = 1u64;
            if nat_eq(a608, a609) {
                Ok(Adt3::C1({
                    let a619 = {
                        let a617 = match v0.clone() {
                            Adt0::C0(_, _, _, _, _, _, _, _, _, h610) => {
                                h610
                            }
                        };
                        let a618 = {
                            let a615 = {
                                let a612 = match v0.clone() {
                                    Adt0::C0(h611, _, _, _, _, _, _, _, _, _) => {
                                        h611
                                    }
                                };
                                let a613 = 1u64;
                                nat_sub(a612, a613)
                            };
                            let a616 = match v0.clone() {
                                Adt0::C0(_, _, _, _, h614, _, _, _, _, _) => {
                                    h614
                                }
                            };
                            nat_mul(a615, a616)?
                        };
                        nat_add(a617, a618)?
                    };
                    let a620 = f43(v0)?;
                    nat_add(a619, a620)?
                }))
            } else {
                Ok(Adt3::C1({
                    let a629 = {
                        let a626 = {
                            let a623 = match v0.clone() {
                                Adt0::C0(h621, _, _, _, _, _, _, _, _, _) => {
                                    h621
                                }
                            };
                            let a624 = match v0.clone() {
                                Adt0::C0(_, _, _, _, h622, _, _, _, _, _) => {
                                    h622
                                }
                            };
                            nat_mul(a623, a624)?
                        };
                        let a627 = match v0.clone() {
                            Adt0::C0(_, _, _, _, _, _, _, _, h625, _) => {
                                h625
                            }
                        };
                        let a628 = 0u64;
                        nat_quot(a626, a627, a628)
                    };
                    let a630 = f43(v0)?;
                    nat_add(a629, a630)?
                }))
            }
        }
    }
}

pub fn f38(v0: u64) -> R<u64> {
    let m631 = f24(f25(v0)?);
    match m631 {
        Adt1::C0 => {
            Ok(0u64)
        }
        Adt1::C1(v1) => {
            match v1 {
                Adt2::C0(_, _, h632) => {
                    Ok(h632)
                }
            }
        }
    }
}

pub fn f39(v0: u64, v1: u64) -> R<u64> {
    let a647 = {
        let a645 = {
            let a640 = {
                let a633 = v1;
                let a634 = 37u64;
                nat_mul(a633, a634)?
            };
            let a641 = {
                let a638 = f41({
                    let a635 = v0;
                    let a636 = 4u64;
                    let a637 = 0u64;
                    nat_quot(a635, a636, a637)
                })?;
                let a639 = 13u64;
                nat_mul(a638, a639)?
            };
            nat_add(a640, a641)?
        };
        let a646 = {
            let a642 = v0;
            let a643 = 4u64;
            let a644 = 0u64;
            nat_rem(a642, a643, a644)
        };
        nat_add(a645, a646)?
    };
    let a648 = 256u64;
    let a649 = 0u64;
    Ok(nat_rem(a647, a648, a649))
}

pub fn f40(v0: u64) -> u64 {
    let a650 = v0;
    let a651 = 96u64;
    if nat_lt(a650, a651) {
        let a655 = {
            let a652 = v0;
            let a653 = 8u64;
            let a654 = 0u64;
            nat_quot(a652, a653, a654)
        };
        let a656 = 3u64;
        let a657 = 0u64;
        nat_rem(a655, a656, a657)
    } else {
        let a663 = {
            let a660 = {
                let a658 = v0;
                let a659 = 96u64;
                nat_sub(a658, a659)
            };
            let a661 = 2u64;
            let a662 = 0u64;
            nat_quot(a660, a661, a662)
        };
        let a664 = 3u64;
        let a665 = 0u64;
        nat_rem(a663, a664, a665)
    }
}

pub fn f41(v0: u64) -> R<u64> {
    let a666 = v0;
    let a667 = 4u64;
    if nat_lt(a666, a667) {
        Ok(v0)
    } else {
        let a668 = v0;
        let a669 = 29u64;
        if nat_lt(a668, a669) {
            let a670 = v0;
            let a671 = 2u64;
            nat_add(a670, a671)
        } else {
            let a672 = v0;
            let a673 = 31u64;
            if nat_lt(a672, a673) {
                let a674 = v0;
                let a675 = 5u64;
                nat_add(a674, a675)
            } else {
                let a676 = v0;
                let a677 = 8u64;
                nat_add(a676, a677)
            }
        }
    }
}

pub fn f42(v0: u64, v1: u64) -> R<u64> {
    let a678 = v1;
    let a679 = 1u64;
    if if nat_eq(a678, a679) {
        true
    } else {
        let a680 = f27(v0);
        let a681 = 1u64;
        if nat_eq(a680, a681) {
            true
        } else {
            let a682 = f27(v0);
            let a683 = 2u64;
            nat_eq(a682, a683)
        }
    } {
        f38(v0)
    } else {
        Ok(0u64)
    }
}

pub fn f43(v0: Adt0) -> R<u64> {
    let a704 = {
        let a696 = {
            let a688 = {
                let a685 = match v0.clone() {
                    Adt0::C0(_, h684, _, _, _, _, _, _, _, _) => {
                        h684
                    }
                };
                let a686 = 1u64;
                nat_sub(a685, a686)
            };
            let a689 = match v0.clone() {
                Adt0::C0(_, _, _, _, _, h687, _, _, _, _) => {
                    h687
                }
            };
            nat_mul(a688, a689)?
        };
        let a697 = {
            let a694 = {
                let a691 = match v0.clone() {
                    Adt0::C0(_, _, h690, _, _, _, _, _, _, _) => {
                        h690
                    }
                };
                let a692 = 1u64;
                nat_sub(a691, a692)
            };
            let a695 = match v0.clone() {
                Adt0::C0(_, _, _, _, _, _, h693, _, _, _) => {
                    h693
                }
            };
            nat_mul(a694, a695)?
        };
        nat_add(a696, a697)?
    };
    let a705 = {
        let a702 = {
            let a699 = match v0.clone() {
                Adt0::C0(_, _, _, h698, _, _, _, _, _, _) => {
                    h698
                }
            };
            let a700 = 1u64;
            nat_sub(a699, a700)
        };
        let a703 = match v0 {
            Adt0::C0(_, _, _, _, _, _, _, h701, _, _) => {
                h701
            }
        };
        nat_mul(a702, a703)?
    };
    nat_add(a704, a705)
}
