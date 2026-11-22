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
                                    let a201 = {
                                        let a199 = {
                                            let a196 = v0;
                                            let a197 = 96u64;
                                            let a198 = 0u64;
                                            nat_quot(a196, a197, a198)
                                        };
                                        let a200 = 4u64;
                                        nat_mul(a199, a200)?
                                    };
                                    let a202 = f27(v0);
                                    nat_add(a201, a202)
                                } else {
                                    let a203 = f26(v0);
                                    let a204 = 1u64;
                                    if if nat_eq(a203, a204) {
                                        let a205 = f27(v0);
                                        let a206 = 3u64;
                                        nat_eq(a205, a206)
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
    let a207 = v1;
    let a208 = 1u64;
    if if nat_eq(a207, a208) {
        let a209 = f27(v0);
        let a210 = 3u64;
        nat_eq(a209, a210)
    } else {
        false
    } {
        let a211 = v2;
        let a212 = 2u64;
        if nat_lt(a211, a212) {
            let a213 = v2;
            let a214 = 0u64;
            if nat_eq(a213, a214) {
                f31(v0)
            } else {
                let a219 = {
                    let a215 = f28(v0)?;
                    let a216 = f29(v0)?;
                    nat_mul(a215, a216)?
                };
                let a220 = {
                    let a217 = 2u64;
                    let a218 = f30(v0)?;
                    nat_mul(a217, a218)?
                };
                nat_mul(a219, a220)
            }
        } else {
            Ok(1u64)
        }
    } else {
        let a221 = v2;
        let a222 = 2u64;
        if nat_lt(a221, a222) {
            let a223 = v2;
            let a224 = 0u64;
            if nat_eq(a223, a224) {
                let a225 = f31(v0)?;
                let a226 = f28(v0)?;
                nat_mul(a225, a226)
            } else {
                f29(v0)
            }
        } else {
            let a227 = v2;
            let a228 = 2u64;
            if nat_eq(a227, a228) {
                Ok(2u64)
            } else {
                f30(v0)
            }
        }
    }
}

pub fn f8(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a229 = v2;
    let a230 = 2u64;
    if nat_lt(a229, a230) {
        let a231 = v2;
        let a232 = 0u64;
        if nat_eq(a231, a232) {
            f32(v0, v1)
        } else {
            f33(v0, v1)
        }
    } else {
        let a233 = v2;
        let a234 = 2u64;
        if nat_eq(a233, a234) {
            f34(v0, v1)
        } else {
            f35(v0, v1)
        }
    }
}

pub fn f9(v0: u64, v1: u64) -> R<u64> {
    let m235 = f36(Adt0::C0(f7(v0, v1, 0u64)?, f7(v0, v1, 1u64)?, f7(v0, v1, 2u64)?, f7(v0, v1, 3u64)?, f8(v0, v1, 0u64)?, f8(v0, v1, 1u64)?, f8(v0, v1, 2u64)?, f8(v0, v1, 3u64)?, f31(v0)?, f37(v0)?))?;
    match m235 {
        Adt3::C0 => {
            Ok(0u64)
        }
        Adt3::C1(v2) => {
            Ok(v2)
        }
    }
}

pub fn f10(v0: u64, v1: u64) -> R<u64> {
    let a240 = {
        let a236 = f7(v0, v1, 0u64)?;
        let a237 = f7(v0, v1, 1u64)?;
        nat_mul(a236, a237)?
    };
    let a241 = {
        let a238 = f7(v0, v1, 2u64)?;
        let a239 = f7(v0, v1, 3u64)?;
        nat_mul(a238, a239)?
    };
    nat_mul(a240, a241)
}

pub fn f11(v0: u64, v1: u64) -> R<u64> {
    let a250 = {
        let a248 = {
            let a246 = {
                let a242 = v1;
                let a243 = 37u64;
                nat_mul(a242, a243)?
            };
            let a247 = {
                let a244 = f25(v0)?;
                let a245 = 13u64;
                nat_mul(a244, a245)?
            };
            nat_add(a246, a247)?
        };
        let a249 = f27(v0);
        nat_add(a248, a249)?
    };
    let a251 = 256u64;
    let a252 = 0u64;
    Ok(nat_rem(a250, a251, a252))
}

pub fn f12(v0: u64) -> R<u64> {
    let a255 = {
        let a253 = f9(v0, 0u64)?;
        let a254 = 255u64;
        nat_add(a253, a254)?
    };
    let a256 = 256u64;
    let a257 = 0u64;
    Ok(nat_quot(a255, a256, a257))
}

pub fn f13(v0: u64) -> R<u64> {
    let a258 = v0;
    let a259 = 96u64;
    if nat_lt(a258, a259) {
        let a260 = v0;
        let a261 = 24u64;
        let a262 = 0u64;
        Ok(nat_quot(a260, a261, a262))
    } else {
        let a268 = 4u64;
        let a269 = {
            let a265 = {
                let a263 = v0;
                let a264 = 96u64;
                nat_sub(a263, a264)
            };
            let a266 = 6u64;
            let a267 = 0u64;
            nat_quot(a265, a266, a267)
        };
        nat_add(a268, a269)
    }
}

pub fn f14(v0: u64) -> u64 {
    let a270 = v0;
    let a271 = 96u64;
    if nat_lt(a270, a271) {
        let a275 = {
            let a272 = v0;
            let a273 = 4u64;
            let a274 = 0u64;
            nat_quot(a272, a273, a274)
        };
        let a276 = 2u64;
        let a277 = 0u64;
        nat_rem(a275, a276, a277)
    } else {
        let a278 = v0;
        let a279 = 2u64;
        let a280 = 0u64;
        nat_rem(a278, a279, a280)
    }
}

pub fn f15(v0: u64, v1: u64, v2: u64) -> u64 {
    let a281 = v1;
    let a282 = 0u64;
    if nat_eq(a281, a282) {
        let a283 = v2;
        let a284 = 2u64;
        if nat_lt(a283, a284) {
            let a285 = v2;
            let a286 = 1u64;
            if nat_lt(a285, a286) {
                f17(v0)
            } else {
                3u64
            }
        } else {
            2u64
        }
    } else {
        let a287 = 96u64;
        let a288 = v0;
        if nat_le(a287, a288) {
            0u64
        } else {
            let a289 = v2;
            let a290 = 2u64;
            if nat_lt(a289, a290) {
                let a291 = v2;
                let a292 = 1u64;
                if nat_lt(a291, a292) {
                    let a293 = f18(v0);
                    let a294 = 1u64;
                    if if nat_eq(a293, a294) {
                        true
                    } else {
                        let a295 = f18(v0);
                        let a296 = 3u64;
                        nat_eq(a295, a296)
                    } {
                        1u64
                    } else {
                        f17(v0)
                    }
                } else {
                    let a297 = f18(v0);
                    let a298 = 1u64;
                    if if nat_eq(a297, a298) {
                        true
                    } else {
                        let a299 = f18(v0);
                        let a300 = 2u64;
                        nat_eq(a299, a300)
                    } {
                        1u64
                    } else {
                        3u64
                    }
                }
            } else {
                let a301 = v2;
                let a302 = 3u64;
                if nat_lt(a301, a302) {
                    let a303 = f18(v0);
                    let a304 = 0u64;
                    if nat_eq(a303, a304) {
                        2u64
                    } else {
                        1u64
                    }
                } else {
                    let a305 = f18(v0);
                    let a306 = 1u64;
                    if if nat_eq(a305, a306) {
                        true
                    } else {
                        let a307 = f18(v0);
                        let a308 = 2u64;
                        nat_eq(a307, a308)
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
    let a309 = v1;
    let a310 = 1u64;
    if if nat_eq(a309, a310) {
        let a311 = 96u64;
        let a312 = v0;
        nat_le(a311, a312)
    } else {
        false
    } {
        Ok(0u64)
    } else {
        let v3: u64 = f17(v0);
        let v4: u64 = f14(v0);
        let v5: u64 = {
            let a313 = v1;
            let a314 = 0u64;
            if nat_eq(a313, a314) {
                0u64
            } else {
                f18(v0)
            }
        };
        let a315 = v5;
        let a316 = 2u64;
        if nat_lt(a315, a316) {
            let a317 = v5;
            let a318 = 1u64;
            if nat_lt(a317, a318) {
                let a319 = v2;
                let a320 = 2u64;
                if nat_lt(a319, a320) {
                    let a321 = v2;
                    let a322 = 1u64;
                    if nat_lt(a321, a322) {
                        Ok(4u64)
                    } else {
                        let a325 = {
                            let a323 = v3;
                            let a324 = v4;
                            nat_add(a323, a324)?
                        };
                        let a326 = 4u64;
                        nat_mul(a325, a326)
                    }
                } else {
                    let a327 = v2;
                    let a328 = 3u64;
                    if nat_lt(a327, a328) {
                        let a335 = {
                            let a333 = {
                                let a329 = v3;
                                let a330 = 3u64;
                                nat_mul(a329, a330)?
                            };
                            let a334 = {
                                let a331 = v4;
                                let a332 = 4u64;
                                nat_mul(a331, a332)?
                            };
                            nat_add(a333, a334)?
                        };
                        let a336 = 4u64;
                        nat_mul(a335, a336)
                    } else {
                        let a343 = {
                            let a341 = {
                                let a337 = v3;
                                let a338 = 6u64;
                                nat_mul(a337, a338)?
                            };
                            let a342 = {
                                let a339 = v4;
                                let a340 = 9u64;
                                nat_mul(a339, a340)?
                            };
                            nat_add(a341, a342)?
                        };
                        let a344 = 4u64;
                        nat_mul(a343, a344)
                    }
                }
            } else {
                let a345 = v2;
                let a346 = 2u64;
                if nat_lt(a345, a346) {
                    let a347 = v2;
                    let a348 = 1u64;
                    if nat_lt(a347, a348) {
                        Ok(4u64)
                    } else {
                        let a351 = {
                            let a349 = 1u64;
                            let a350 = v4;
                            nat_add(a349, a350)?
                        };
                        let a352 = 4u64;
                        nat_mul(a351, a352)
                    }
                } else {
                    let a353 = v2;
                    let a354 = 3u64;
                    if nat_lt(a353, a354) {
                        let a359 = {
                            let a357 = 1u64;
                            let a358 = {
                                let a355 = v4;
                                let a356 = 2u64;
                                nat_mul(a355, a356)?
                            };
                            nat_add(a357, a358)?
                        };
                        let a360 = 4u64;
                        nat_mul(a359, a360)
                    } else {
                        let a365 = {
                            let a363 = 1u64;
                            let a364 = {
                                let a361 = v4;
                                let a362 = 3u64;
                                nat_mul(a361, a362)?
                            };
                            nat_add(a363, a364)?
                        };
                        let a366 = 4u64;
                        nat_mul(a365, a366)
                    }
                }
            }
        } else {
            let a367 = v5;
            let a368 = 3u64;
            if nat_lt(a367, a368) {
                let a369 = v2;
                let a370 = 2u64;
                if nat_lt(a369, a370) {
                    let a371 = v2;
                    let a372 = 1u64;
                    if nat_lt(a371, a372) {
                        Ok(4u64)
                    } else {
                        let a375 = {
                            let a373 = v3;
                            let a374 = v4;
                            nat_add(a373, a374)?
                        };
                        let a376 = 4u64;
                        nat_mul(a375, a376)
                    }
                } else {
                    let a377 = v2;
                    let a378 = 3u64;
                    if nat_lt(a377, a378) {
                        let a383 = {
                            let a381 = v3;
                            let a382 = {
                                let a379 = v4;
                                let a380 = 2u64;
                                nat_mul(a379, a380)?
                            };
                            nat_add(a381, a382)?
                        };
                        let a384 = 4u64;
                        nat_mul(a383, a384)
                    } else {
                        let a389 = {
                            let a387 = v3;
                            let a388 = {
                                let a385 = v4;
                                let a386 = 3u64;
                                nat_mul(a385, a386)?
                            };
                            nat_add(a387, a388)?
                        };
                        let a390 = 4u64;
                        nat_mul(a389, a390)
                    }
                }
            } else {
                let a391 = v2;
                let a392 = 2u64;
                if nat_lt(a391, a392) {
                    let a393 = v2;
                    let a394 = 1u64;
                    if nat_lt(a393, a394) {
                        Ok(4u64)
                    } else {
                        let a397 = {
                            let a395 = 1u64;
                            let a396 = v4;
                            nat_add(a395, a396)?
                        };
                        let a398 = 4u64;
                        nat_mul(a397, a398)
                    }
                } else {
                    let a399 = v2;
                    let a400 = 3u64;
                    if nat_lt(a399, a400) {
                        let a405 = {
                            let a403 = 3u64;
                            let a404 = {
                                let a401 = v4;
                                let a402 = 4u64;
                                nat_mul(a401, a402)?
                            };
                            nat_add(a403, a404)?
                        };
                        let a406 = 4u64;
                        nat_mul(a405, a406)
                    } else {
                        let a411 = {
                            let a409 = 3u64;
                            let a410 = {
                                let a407 = v4;
                                let a408 = 5u64;
                                nat_mul(a407, a408)?
                            };
                            nat_add(a409, a410)?
                        };
                        let a412 = 4u64;
                        nat_mul(a411, a412)
                    }
                }
            }
        }
    }
}

pub fn f17(v0: u64) -> u64 {
    let a413 = f38(v0);
    let a414 = 0u64;
    if nat_eq(a413, a414) {
        1u64
    } else {
        let a415 = f38(v0);
        let a416 = 1u64;
        if nat_eq(a415, a416) {
            17u64
        } else {
            257u64
        }
    }
}

pub fn f18(v0: u64) -> u64 {
    let a417 = v0;
    let a418 = 4u64;
    let a419 = 0u64;
    nat_rem(a417, a418, a419)
}

pub fn f19(v0: u64) -> u64 {
    let a420 = v0;
    let a421 = 8u64;
    if nat_lt(a420, a421) {
        let a422 = v0;
        let a423 = 4u64;
        if nat_lt(a422, a423) {
            let a424 = v0;
            let a425 = 2u64;
            if nat_lt(a424, a425) {
                let a426 = v0;
                let a427 = 1u64;
                if nat_lt(a426, a427) {
                    0u64
                } else {
                    2147483648u64
                }
            } else {
                let a428 = v0;
                let a429 = 3u64;
                if nat_lt(a428, a429) {
                    1065353216u64
                } else {
                    3212836864u64
                }
            }
        } else {
            let a430 = v0;
            let a431 = 6u64;
            if nat_lt(a430, a431) {
                let a432 = v0;
                let a433 = 5u64;
                if nat_lt(a432, a433) {
                    2139095040u64
                } else {
                    4286578688u64
                }
            } else {
                let a434 = v0;
                let a435 = 7u64;
                if nat_lt(a434, a435) {
                    2143289345u64
                } else {
                    1u64
                }
            }
        }
    } else {
        let a436 = v0;
        let a437 = 12u64;
        if nat_lt(a436, a437) {
            let a438 = v0;
            let a439 = 10u64;
            if nat_lt(a438, a439) {
                let a440 = v0;
                let a441 = 9u64;
                if nat_lt(a440, a441) {
                    2139095039u64
                } else {
                    8388608u64
                }
            } else {
                let a442 = v0;
                let a443 = 11u64;
                if nat_lt(a442, a443) {
                    8388607u64
                } else {
                    1065353217u64
                }
            }
        } else {
            let a444 = v0;
            let a445 = 14u64;
            if nat_lt(a444, a445) {
                let a446 = v0;
                let a447 = 13u64;
                if nat_lt(a446, a447) {
                    1056964608u64
                } else {
                    8388609u64
                }
            } else {
                let a448 = v0;
                let a449 = 15u64;
                if nat_lt(a448, a449) {
                    2147483649u64
                } else {
                    2139095041u64
                }
            }
        }
    }
}

pub fn f20(v0: u64, v1: u64) -> R<u64> {
    let a458 = {
        let a456 = {
            let a452 = {
                let a450 = v0;
                let a451 = 0u64;
                if nat_eq(a450, a451) {
                    1u64
                } else {
                    5u64
                }
            };
            let a453 = v1;
            nat_mul(a452, a453)?
        };
        let a457 = {
            let a454 = v0;
            let a455 = 0u64;
            if nat_eq(a454, a455) {
                0u64
            } else {
                3u64
            }
        };
        nat_add(a456, a457)?
    };
    let a459 = 16u64;
    let a460 = 0u64;
    Ok(nat_rem(a458, a459, a460))
}

pub fn f21(v0: u64) -> u64 {
    let a461 = v0;
    let a462 = 4294967296u64;
    let a463 = 0u64;
    nat_rem(a461, a462, a463)
}

pub fn f22(v0: u64) -> u64 {
    let a464 = v0;
    let a465 = 0u64;
    if nat_eq(a464, a465) {
        2654435769u64
    } else {
        2246822507u64
    }
}

pub fn f23(v0: u64) -> R<u64> {
    let a466 = 2654435761u64;
    let a467 = v0;
    nat_mul(a466, a467)
}

pub fn f24(v0: u64) -> Adt1 {
    let a468 = v0;
    let a469 = 0u64;
    if nat_eq(a468, a469) {
        Adt1::C1(Adt2::C0(0u64, 1u64, 4u64))
    } else {
        let a470 = v0;
        let a471 = 1u64;
        if nat_eq(a470, a471) {
            Adt1::C1(Adt2::C0(1u64, 1u64, 2u64))
        } else {
            let a472 = v0;
            let a473 = 2u64;
            if nat_eq(a472, a473) {
                Adt1::C1(Adt2::C0(2u64, 32u64, 18u64))
            } else {
                let a474 = v0;
                let a475 = 3u64;
                if nat_eq(a474, a475) {
                    Adt1::C1(Adt2::C0(3u64, 32u64, 20u64))
                } else {
                    let a476 = v0;
                    let a477 = 6u64;
                    if nat_eq(a476, a477) {
                        Adt1::C1(Adt2::C0(6u64, 32u64, 22u64))
                    } else {
                        let a478 = v0;
                        let a479 = 7u64;
                        if nat_eq(a478, a479) {
                            Adt1::C1(Adt2::C0(7u64, 32u64, 24u64))
                        } else {
                            let a480 = v0;
                            let a481 = 8u64;
                            if nat_eq(a480, a481) {
                                Adt1::C1(Adt2::C0(8u64, 32u64, 34u64))
                            } else {
                                let a482 = v0;
                                let a483 = 9u64;
                                if nat_eq(a482, a483) {
                                    Adt1::C1(Adt2::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a484 = v0;
                                    let a485 = 10u64;
                                    if nat_eq(a484, a485) {
                                        Adt1::C1(Adt2::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a486 = v0;
                                        let a487 = 11u64;
                                        if nat_eq(a486, a487) {
                                            Adt1::C1(Adt2::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a488 = v0;
                                            let a489 = 12u64;
                                            if nat_eq(a488, a489) {
                                                Adt1::C1(Adt2::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a490 = v0;
                                                let a491 = 13u64;
                                                if nat_eq(a490, a491) {
                                                    Adt1::C1(Adt2::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a492 = v0;
                                                    let a493 = 14u64;
                                                    if nat_eq(a492, a493) {
                                                        Adt1::C1(Adt2::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a494 = v0;
                                                        let a495 = 15u64;
                                                        if nat_eq(a494, a495) {
                                                            Adt1::C1(Adt2::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a496 = v0;
                                                            let a497 = 16u64;
                                                            if nat_eq(a496, a497) {
                                                                Adt1::C1(Adt2::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a498 = v0;
                                                                let a499 = 17u64;
                                                                if nat_eq(a498, a499) {
                                                                    Adt1::C1(Adt2::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a500 = v0;
                                                                    let a501 = 18u64;
                                                                    if nat_eq(a500, a501) {
                                                                        Adt1::C1(Adt2::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a502 = v0;
                                                                        let a503 = 19u64;
                                                                        if nat_eq(a502, a503) {
                                                                            Adt1::C1(Adt2::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a504 = v0;
                                                                            let a505 = 20u64;
                                                                            if nat_eq(a504, a505) {
                                                                                Adt1::C1(Adt2::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a506 = v0;
                                                                                let a507 = 21u64;
                                                                                if nat_eq(a506, a507) {
                                                                                    Adt1::C1(Adt2::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a508 = v0;
                                                                                    let a509 = 22u64;
                                                                                    if nat_eq(a508, a509) {
                                                                                        Adt1::C1(Adt2::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a510 = v0;
                                                                                        let a511 = 23u64;
                                                                                        if nat_eq(a510, a511) {
                                                                                            Adt1::C1(Adt2::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a512 = v0;
                                                                                            let a513 = 24u64;
                                                                                            if nat_eq(a512, a513) {
                                                                                                Adt1::C1(Adt2::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a514 = v0;
                                                                                                let a515 = 25u64;
                                                                                                if nat_eq(a514, a515) {
                                                                                                    Adt1::C1(Adt2::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a516 = v0;
                                                                                                    let a517 = 26u64;
                                                                                                    if nat_eq(a516, a517) {
                                                                                                        Adt1::C1(Adt2::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a518 = v0;
                                                                                                        let a519 = 27u64;
                                                                                                        if nat_eq(a518, a519) {
                                                                                                            Adt1::C1(Adt2::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a520 = v0;
                                                                                                            let a521 = 28u64;
                                                                                                            if nat_eq(a520, a521) {
                                                                                                                Adt1::C1(Adt2::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a522 = v0;
                                                                                                                let a523 = 29u64;
                                                                                                                if nat_eq(a522, a523) {
                                                                                                                    Adt1::C1(Adt2::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a524 = v0;
                                                                                                                    let a525 = 30u64;
                                                                                                                    if nat_eq(a524, a525) {
                                                                                                                        Adt1::C1(Adt2::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a526 = v0;
                                                                                                                        let a527 = 34u64;
                                                                                                                        if nat_eq(a526, a527) {
                                                                                                                            Adt1::C1(Adt2::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a528 = v0;
                                                                                                                            let a529 = 35u64;
                                                                                                                            if nat_eq(a528, a529) {
                                                                                                                                Adt1::C1(Adt2::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a530 = v0;
                                                                                                                                let a531 = 39u64;
                                                                                                                                if nat_eq(a530, a531) {
                                                                                                                                    Adt1::C1(Adt2::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a532 = v0;
                                                                                                                                    let a533 = 40u64;
                                                                                                                                    if nat_eq(a532, a533) {
                                                                                                                                        Adt1::C1(Adt2::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a534 = v0;
                                                                                                                                        let a535 = 41u64;
                                                                                                                                        if nat_eq(a534, a535) {
                                                                                                                                            Adt1::C1(Adt2::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a536 = v0;
                                                                                                                                            let a537 = 42u64;
                                                                                                                                            if nat_eq(a536, a537) {
                                                                                                                                                Adt1::C1(Adt2::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a538 = v0;
                                                                                                                                                let a539 = 43u64;
                                                                                                                                                if nat_eq(a538, a539) {
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
    f39({
        let a540 = v0;
        let a541 = 96u64;
        let a542 = 0u64;
        nat_quot(a540, a541, a542)
    })
}

pub fn f26(v0: u64) -> u64 {
    let a546 = {
        let a543 = v0;
        let a544 = 32u64;
        let a545 = 0u64;
        nat_quot(a543, a544, a545)
    };
    let a547 = 3u64;
    let a548 = 0u64;
    nat_rem(a546, a547, a548)
}

pub fn f27(v0: u64) -> u64 {
    let a552 = {
        let a549 = v0;
        let a550 = 8u64;
        let a551 = 0u64;
        nat_quot(a549, a550, a551)
    };
    let a553 = 4u64;
    let a554 = 0u64;
    nat_rem(a552, a553, a554)
}

pub fn f28(v0: u64) -> R<u64> {
    let a563 = 1u64;
    let a564 = {
        let a561 = 2u64;
        let a562 = {
            let a558 = {
                let a555 = v0;
                let a556 = 4u64;
                let a557 = 0u64;
                nat_quot(a555, a556, a557)
            };
            let a559 = 2u64;
            let a560 = 0u64;
            nat_rem(a558, a559, a560)
        };
        nat_mul(a561, a562)?
    };
    nat_add(a563, a564)
}

pub fn f29(v0: u64) -> R<u64> {
    let a573 = 1u64;
    let a574 = {
        let a571 = 2u64;
        let a572 = {
            let a568 = {
                let a565 = v0;
                let a566 = 2u64;
                let a567 = 0u64;
                nat_quot(a565, a566, a567)
            };
            let a569 = 2u64;
            let a570 = 0u64;
            nat_rem(a568, a569, a570)
        };
        nat_mul(a571, a572)?
    };
    nat_add(a573, a574)
}

pub fn f30(v0: u64) -> R<u64> {
    let a578 = 1u64;
    let a579 = {
        let a575 = v0;
        let a576 = 2u64;
        let a577 = 0u64;
        nat_rem(a575, a576, a577)
    };
    nat_add(a578, a579)
}

pub fn f31(v0: u64) -> R<u64> {
    let m580 = f24(f25(v0)?);
    match m580 {
        Adt1::C0 => {
            Ok(0u64)
        }
        Adt1::C1(v1) => {
            match v1 {
                Adt2::C0(_, h581, _) => {
                    Ok(h581)
                }
            }
        }
    }
}

pub fn f32(v0: u64, v1: u64) -> R<u64> {
    let a586 = f37(v0)?;
    let a587 = {
        let a582 = v1;
        let a583 = 0u64;
        if if nat_eq(a582, a583) {
            let a584 = f27(v0);
            let a585 = 2u64;
            nat_eq(a584, a585)
        } else {
            false
        } {
            2u64
        } else {
            1u64
        }
    };
    nat_mul(a586, a587)
}

pub fn f33(v0: u64, v1: u64) -> R<u64> {
    let a593 = {
        let a591 = f32(v0, v1)?;
        let a592 = {
            let a588 = f7(v0, v1, 0u64)?;
            let a589 = f31(v0)?;
            let a590 = 0u64;
            nat_quot(a588, a589, a590)
        };
        nat_mul(a591, a592)?
    };
    let a594 = f40(v0, v1)?;
    nat_add(a593, a594)
}

pub fn f34(v0: u64, v1: u64) -> R<u64> {
    let a597 = {
        let a595 = f33(v0, v1)?;
        let a596 = f7(v0, v1, 1u64)?;
        nat_mul(a595, a596)?
    };
    let a598 = f40(v0, v1)?;
    nat_add(a597, a598)
}

pub fn f35(v0: u64, v1: u64) -> R<u64> {
    let a601 = {
        let a599 = f34(v0, v1)?;
        let a600 = f7(v0, v1, 2u64)?;
        nat_mul(a599, a600)?
    };
    let a602 = f40(v0, v1)?;
    nat_add(a601, a602)
}

pub fn f36(v0: Adt0) -> R<Adt3> {
    let a604 = match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h603, _) => {
            h603
        }
    };
    let a605 = 0u64;
    if nat_eq(a604, a605) {
        Ok(Adt3::C0)
    } else {
        let a607 = match v0.clone() {
            Adt0::C0(h606, _, _, _, _, _, _, _, _, _) => {
                h606
            }
        };
        let a608 = 0u64;
        if if if if nat_eq(a607, a608) {
            true
        } else {
            let a610 = match v0.clone() {
                Adt0::C0(_, h609, _, _, _, _, _, _, _, _) => {
                    h609
                }
            };
            let a611 = 0u64;
            nat_eq(a610, a611)
        } {
            true
        } else {
            let a613 = match v0.clone() {
                Adt0::C0(_, _, h612, _, _, _, _, _, _, _) => {
                    h612
                }
            };
            let a614 = 0u64;
            nat_eq(a613, a614)
        } {
            true
        } else {
            let a616 = match v0.clone() {
                Adt0::C0(_, _, _, h615, _, _, _, _, _, _) => {
                    h615
                }
            };
            let a617 = 0u64;
            nat_eq(a616, a617)
        } {
            Ok(Adt3::C1(0u64))
        } else {
            let a619 = match v0.clone() {
                Adt0::C0(_, _, _, _, _, _, _, _, h618, _) => {
                    h618
                }
            };
            let a620 = 1u64;
            if nat_eq(a619, a620) {
                Ok(Adt3::C1({
                    let a630 = {
                        let a628 = match v0.clone() {
                            Adt0::C0(_, _, _, _, _, _, _, _, _, h621) => {
                                h621
                            }
                        };
                        let a629 = {
                            let a626 = {
                                let a623 = match v0.clone() {
                                    Adt0::C0(h622, _, _, _, _, _, _, _, _, _) => {
                                        h622
                                    }
                                };
                                let a624 = 1u64;
                                nat_sub(a623, a624)
                            };
                            let a627 = match v0.clone() {
                                Adt0::C0(_, _, _, _, h625, _, _, _, _, _) => {
                                    h625
                                }
                            };
                            nat_mul(a626, a627)?
                        };
                        nat_add(a628, a629)?
                    };
                    let a631 = f41(v0)?;
                    nat_add(a630, a631)?
                }))
            } else {
                Ok(Adt3::C1({
                    let a640 = {
                        let a637 = {
                            let a634 = match v0.clone() {
                                Adt0::C0(h632, _, _, _, _, _, _, _, _, _) => {
                                    h632
                                }
                            };
                            let a635 = match v0.clone() {
                                Adt0::C0(_, _, _, _, h633, _, _, _, _, _) => {
                                    h633
                                }
                            };
                            nat_mul(a634, a635)?
                        };
                        let a638 = match v0.clone() {
                            Adt0::C0(_, _, _, _, _, _, _, _, h636, _) => {
                                h636
                            }
                        };
                        let a639 = 0u64;
                        nat_quot(a637, a638, a639)
                    };
                    let a641 = f41(v0)?;
                    nat_add(a640, a641)?
                }))
            }
        }
    }
}

pub fn f37(v0: u64) -> R<u64> {
    let m642 = f24(f25(v0)?);
    match m642 {
        Adt1::C0 => {
            Ok(0u64)
        }
        Adt1::C1(v1) => {
            match v1 {
                Adt2::C0(_, _, h643) => {
                    Ok(h643)
                }
            }
        }
    }
}

pub fn f38(v0: u64) -> u64 {
    let a644 = v0;
    let a645 = 96u64;
    if nat_lt(a644, a645) {
        let a649 = {
            let a646 = v0;
            let a647 = 8u64;
            let a648 = 0u64;
            nat_quot(a646, a647, a648)
        };
        let a650 = 3u64;
        let a651 = 0u64;
        nat_rem(a649, a650, a651)
    } else {
        let a657 = {
            let a654 = {
                let a652 = v0;
                let a653 = 96u64;
                nat_sub(a652, a653)
            };
            let a655 = 2u64;
            let a656 = 0u64;
            nat_quot(a654, a655, a656)
        };
        let a658 = 3u64;
        let a659 = 0u64;
        nat_rem(a657, a658, a659)
    }
}

pub fn f39(v0: u64) -> R<u64> {
    let a660 = v0;
    let a661 = 4u64;
    if nat_lt(a660, a661) {
        Ok(v0)
    } else {
        let a662 = v0;
        let a663 = 29u64;
        if nat_lt(a662, a663) {
            let a664 = v0;
            let a665 = 2u64;
            nat_add(a664, a665)
        } else {
            let a666 = v0;
            let a667 = 31u64;
            if nat_lt(a666, a667) {
                let a668 = v0;
                let a669 = 5u64;
                nat_add(a668, a669)
            } else {
                let a670 = v0;
                let a671 = 8u64;
                nat_add(a670, a671)
            }
        }
    }
}

pub fn f40(v0: u64, v1: u64) -> R<u64> {
    let a672 = v1;
    let a673 = 1u64;
    if if nat_eq(a672, a673) {
        true
    } else {
        let a674 = f27(v0);
        let a675 = 1u64;
        if nat_eq(a674, a675) {
            true
        } else {
            let a676 = f27(v0);
            let a677 = 2u64;
            nat_eq(a676, a677)
        }
    } {
        f37(v0)
    } else {
        Ok(0u64)
    }
}

pub fn f41(v0: Adt0) -> R<u64> {
    let a698 = {
        let a690 = {
            let a682 = {
                let a679 = match v0.clone() {
                    Adt0::C0(_, h678, _, _, _, _, _, _, _, _) => {
                        h678
                    }
                };
                let a680 = 1u64;
                nat_sub(a679, a680)
            };
            let a683 = match v0.clone() {
                Adt0::C0(_, _, _, _, _, h681, _, _, _, _) => {
                    h681
                }
            };
            nat_mul(a682, a683)?
        };
        let a691 = {
            let a688 = {
                let a685 = match v0.clone() {
                    Adt0::C0(_, _, h684, _, _, _, _, _, _, _) => {
                        h684
                    }
                };
                let a686 = 1u64;
                nat_sub(a685, a686)
            };
            let a689 = match v0.clone() {
                Adt0::C0(_, _, _, _, _, _, h687, _, _, _) => {
                    h687
                }
            };
            nat_mul(a688, a689)?
        };
        nat_add(a690, a691)?
    };
    let a699 = {
        let a696 = {
            let a693 = match v0.clone() {
                Adt0::C0(_, _, _, h692, _, _, _, _, _, _) => {
                    h692
                }
            };
            let a694 = 1u64;
            nat_sub(a693, a694)
        };
        let a697 = match v0 {
            Adt0::C0(_, _, _, _, _, _, _, h695, _, _) => {
                h695
            }
        };
        nat_mul(a696, a697)?
    };
    nat_add(a698, a699)
}

pub fn run(p0: u64, p1: u64, p2: u64, p3: u64) -> R<Option<u64>> {
    f0(p0, p1, p2, p3)
}
