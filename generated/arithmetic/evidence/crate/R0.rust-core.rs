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
    let a1 = 38u64;
    let a2 = v0;
    if nat_le(a1, a2) {
        f1({
            let a3 = v0;
            let a4 = 38u64;
            nat_sub(a3, a4)
        }, v1, v2, v3)
    } else {
        let a5 = 31u64;
        let a6 = v0;
        if nat_le(a5, a6) {
            f2(1u64, {
                let a7 = v0;
                let a8 = 31u64;
                nat_sub(a7, a8)
            }, v1, v2, v3)
        } else {
            let a9 = 24u64;
            let a10 = v0;
            if nat_le(a9, a10) {
                f2(0u64, {
                    let a11 = v0;
                    let a12 = 24u64;
                    nat_sub(a11, a12)
                }, v1, v2, v3)
            } else {
                let a13 = 17u64;
                let a14 = v0;
                if nat_le(a13, a14) {
                    f3({
                        let a15 = v0;
                        let a16 = 17u64;
                        nat_sub(a15, a16)
                    }, v1, v2, v3)
                } else {
                    let a17 = 11u64;
                    let a18 = v0;
                    if nat_le(a17, a18) {
                        f4({
                            let a19 = v0;
                            let a20 = 11u64;
                            nat_sub(a19, a20)
                        }, v1, v2, v3)
                    } else {
                        let a21 = 5u64;
                        let a22 = v0;
                        if nat_le(a21, a22) {
                            f5({
                                let a23 = v0;
                                let a24 = 5u64;
                                nat_sub(a23, a24)
                            }, v1, v2, v3)
                        } else {
                            let a25 = v0;
                            let a26 = 2u64;
                            if nat_lt(a25, a26) {
                                let a27 = v0;
                                let a28 = 0u64;
                                if nat_eq(a27, a28) {
                                    let a29 = v3;
                                    let a30 = 4u64;
                                    if nat_lt(a29, a30) {
                                        Ok(Some({
                                            let a31 = v3;
                                            let a32 = 2u64;
                                            if nat_lt(a31, a32) {
                                                let a33 = v3;
                                                let a34 = 1u64;
                                                if nat_lt(a33, a34) {
                                                    120u64
                                                } else {
                                                    24u64
                                                }
                                            } else {
                                                let a35 = v3;
                                                let a36 = 3u64;
                                                if nat_lt(a35, a36) {
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
                                    let a37 = v1;
                                    let a38 = 120u64;
                                    if if nat_lt(a37, a38) {
                                        let a39 = v3;
                                        let a40 = 24u64;
                                        nat_lt(a39, a40)
                                    } else {
                                        false
                                    } {
                                        Ok(Some(f6(v1, v3)?))
                                    } else {
                                        Ok(None::<u64>)
                                    }
                                }
                            } else {
                                let a41 = v0;
                                let a42 = 4u64;
                                if if nat_lt(a41, a42) {
                                    let a43 = v1;
                                    let a44 = 120u64;
                                    if nat_lt(a43, a44) {
                                        let a45 = v2;
                                        let a46 = 2u64;
                                        nat_lt(a45, a46)
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                } {
                                    let a47 = v0;
                                    let a48 = 2u64;
                                    if nat_eq(a47, a48) {
                                        let a49 = v3;
                                        let a50 = 2u64;
                                        if nat_lt(a49, a50) {
                                            Ok(Some({
                                                let a51 = v3;
                                                let a52 = 0u64;
                                                if nat_eq(a51, a52) {
                                                    f7(v1, v2)?
                                                } else {
                                                    4u64
                                                }
                                            }))
                                        } else {
                                            Ok(None::<u64>)
                                        }
                                    } else {
                                        let a53 = v3;
                                        let a54 = f7(v1, v2)?;
                                        if nat_lt(a53, a54) {
                                            Ok(Some(f8(v2, v3)?))
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
}

pub fn f1(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a55 = v0;
    let a56 = 4u64;
    if if nat_eq(a55, a56) {
        let a57 = v1;
        let a58 = 256u64;
        nat_lt(a57, a58)
    } else {
        false
    } {
        f9(v1, v2, v3)
    } else {
        let a59 = v0;
        let a60 = 0u64;
        if nat_eq(a59, a60) {
            let a61 = v1;
            let a62 = 0u64;
            if if if nat_eq(a61, a62) {
                let a63 = v2;
                let a64 = 0u64;
                nat_eq(a63, a64)
            } else {
                false
            } {
                let a65 = v3;
                let a66 = 6u64;
                nat_lt(a65, a66)
            } else {
                false
            } {
                Ok(Some({
                    let a67 = v3;
                    let a68 = 0u64;
                    if nat_eq(a67, a68) {
                        256u64
                    } else {
                        let a69 = v3;
                        let a70 = 1u64;
                        if nat_eq(a69, a70) {
                            10u64
                        } else {
                            let a71 = v3;
                            let a72 = 2u64;
                            if nat_eq(a71, a72) {
                                256u64
                            } else {
                                let a73 = v3;
                                let a74 = 3u64;
                                if nat_eq(a73, a74) {
                                    512u64
                                } else {
                                    let a75 = v3;
                                    let a76 = 4u64;
                                    if nat_eq(a75, a76) {
                                        165u64
                                    } else {
                                        168u64
                                    }
                                }
                            }
                        }
                    }
                }))
            } else {
                Ok(None::<u64>)
            }
        } else {
            let a77 = v1;
            let a78 = 256u64;
            if nat_lt(a77, a78) {
                let a79 = v0;
                let a80 = 1u64;
                if nat_eq(a79, a80) {
                    let a81 = v2;
                    let a82 = 0u64;
                    if if nat_eq(a81, a82) {
                        let a83 = v3;
                        let a84 = 10u64;
                        nat_lt(a83, a84)
                    } else {
                        false
                    } {
                        Ok(Some(f10(v1, v3)))
                    } else {
                        Ok(None::<u64>)
                    }
                } else {
                    let a85 = v0;
                    let a86 = 2u64;
                    if nat_eq(a85, a86) {
                        let a87 = v2;
                        let a88 = 0u64;
                        if if if nat_eq(a87, a88) {
                            true
                        } else {
                            let a89 = v2;
                            let a90 = 1u64;
                            if nat_eq(a89, a90) {
                                true
                            } else {
                                let a91 = v2;
                                let a92 = 3u64;
                                nat_eq(a91, a92)
                            }
                        } {
                            let a93 = v3;
                            let a94 = 10u64;
                            nat_lt(a93, a94)
                        } else {
                            false
                        } {
                            Ok(Some({
                                let a95 = v3;
                                let a96 = 4u64;
                                if nat_lt(a95, a96) {
                                    f11(v1, v3)
                                } else {
                                    let a97 = v3;
                                    let a98 = 8u64;
                                    if nat_lt(a97, a98) {
                                        f12(v1, v2, {
                                            let a99 = v3;
                                            let a100 = 4u64;
                                            nat_sub(a99, a100)
                                        })?
                                    } else {
                                        let a101 = v3;
                                        let a102 = 8u64;
                                        if nat_eq(a101, a102) {
                                            f13(v1, v2)?
                                        } else {
                                            f14(v1)?
                                        }
                                    }
                                }
                            }))
                        } else {
                            Ok(None::<u64>)
                        }
                    } else {
                        let a103 = v0;
                        let a104 = 3u64;
                        if nat_eq(a103, a104) {
                            let a105 = v2;
                            let a106 = 0u64;
                            if if nat_eq(a105, a106) {
                                let a107 = v3;
                                let a108 = 512u64;
                                nat_lt(a107, a108)
                            } else {
                                false
                            } {
                                Ok(Some(f15(v1, v3)?))
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
}

pub fn f2(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Option<u64>> {
    let a109 = v1;
    let a110 = 0u64;
    if nat_eq(a109, a110) {
        let a111 = v2;
        let a112 = 0u64;
        if if if nat_eq(a111, a112) {
            let a113 = v3;
            let a114 = 0u64;
            nat_eq(a113, a114)
        } else {
            false
        } {
            let a115 = v4;
            let a116 = 5u64;
            nat_lt(a115, a116)
        } else {
            false
        } {
            Ok(Some({
                let a117 = v4;
                let a118 = 2u64;
                if nat_lt(a117, a118) {
                    let a119 = v4;
                    let a120 = 1u64;
                    if nat_lt(a119, a120) {
                        f16(v0)
                    } else {
                        10u64
                    }
                } else {
                    let a121 = v4;
                    let a122 = 3u64;
                    if nat_lt(a121, a122) {
                        let a123 = v0;
                        let a124 = 0u64;
                        if nat_eq(a123, a124) {
                            8u64
                        } else {
                            3u64
                        }
                    } else {
                        let a125 = v4;
                        let a126 = 4u64;
                        if nat_lt(a125, a126) {
                            f17(v0)
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
        let a127 = v2;
        let a128 = f16(v0);
        if nat_lt(a127, a128) {
            let a129 = v1;
            let a130 = 1u64;
            if nat_eq(a129, a130) {
                let a131 = v3;
                let a132 = 0u64;
                if if nat_eq(a131, a132) {
                    let a133 = v4;
                    let a134 = 10u64;
                    nat_lt(a133, a134)
                } else {
                    false
                } {
                    Ok(Some(f18(v0, v2, v4)?))
                } else {
                    Ok(None::<u64>)
                }
            } else {
                let a135 = v1;
                let a136 = 2u64;
                if nat_eq(a135, a136) {
                    let a137 = v3;
                    let a138 = 2u64;
                    if if if nat_lt(a137, a138) {
                        true
                    } else {
                        let a139 = v3;
                        let a140 = 3u64;
                        nat_eq(a139, a140)
                    } {
                        let a141 = v4;
                        let a142 = 10u64;
                        nat_lt(a141, a142)
                    } else {
                        false
                    } {
                        Ok(Some({
                            let a143 = v4;
                            let a144 = 4u64;
                            if nat_lt(a143, a144) {
                                f19(v0, v2, v3, v4)?
                            } else {
                                let a145 = v4;
                                let a146 = 8u64;
                                if nat_lt(a145, a146) {
                                    f20(v0, v2, v3, {
                                        let a147 = v4;
                                        let a148 = 4u64;
                                        nat_sub(a147, a148)
                                    })?
                                } else {
                                    let a149 = v4;
                                    let a150 = 8u64;
                                    if nat_eq(a149, a150) {
                                        f21(v0, v2, v3)?
                                    } else {
                                        f22(v0, v2, v3)?
                                    }
                                }
                            }
                        }))
                    } else {
                        Ok(None::<u64>)
                    }
                } else {
                    let a151 = v1;
                    let a152 = 3u64;
                    if nat_eq(a151, a152) {
                        let a153 = v3;
                        let a154 = 0u64;
                        if if nat_eq(a153, a154) {
                            let a155 = v4;
                            let a156 = f17(v0);
                            nat_lt(a155, a156)
                        } else {
                            false
                        } {
                            Ok(Some(f23(v0, v2, v4)?))
                        } else {
                            Ok(None::<u64>)
                        }
                    } else {
                        let a157 = v1;
                        let a158 = 4u64;
                        if nat_eq(a157, a158) {
                            let a159 = v4;
                            let a160 = 2u64;
                            if nat_lt(a159, a160) {
                                let a161 = v3;
                                let a162 = 0u64;
                                if nat_eq(a161, a162) {
                                    let a163 = v4;
                                    let a164 = 0u64;
                                    if nat_eq(a163, a164) {
                                        Ok(Some(f24(v0, v2)?))
                                    } else {
                                        Ok(None::<u64>)
                                    }
                                } else {
                                    let a165 = v3;
                                    let a166 = f24(v0, v2)?;
                                    if nat_le(a165, a166) {
                                        Ok(Some({
                                            let v5: u64 = f17(v0);
                                            let v6: u64 = {
                                                let a169 = {
                                                    let a167 = v3;
                                                    let a168 = 1u64;
                                                    nat_sub(a167, a168)
                                                };
                                                let a170 = v5;
                                                nat_mul(a169, a170)?
                                            };
                                            let v7: u64 = {
                                                let a171 = f21(v0, v2, 0u64)?;
                                                let a172 = v6;
                                                nat_sub(a171, a172)
                                            };
                                            let a173 = v4;
                                            let a174 = 0u64;
                                            if nat_eq(a173, a174) {
                                                v6
                                            } else {
                                                let a175 = v7;
                                                let a176 = v5;
                                                if nat_lt(a175, a176) {
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

pub fn f3(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a177 = v0;
    let a178 = 0u64;
    if nat_eq(a177, a178) {
        let a179 = v1;
        let a180 = 0u64;
        if if if nat_eq(a179, a180) {
            let a181 = v2;
            let a182 = 0u64;
            nat_eq(a181, a182)
        } else {
            false
        } {
            let a183 = v3;
            let a184 = 5u64;
            nat_lt(a183, a184)
        } else {
            false
        } {
            Ok(Some({
                let a185 = v3;
                let a186 = 2u64;
                if nat_lt(a185, a186) {
                    let a187 = v3;
                    let a188 = 0u64;
                    if nat_eq(a187, a188) {
                        10368u64
                    } else {
                        9u64
                    }
                } else {
                    let a189 = v3;
                    let a190 = 4u64;
                    if nat_lt(a189, a190) {
                        let a191 = v3;
                        let a192 = 2u64;
                        if nat_eq(a191, a192) {
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
        let a193 = v1;
        let a194 = 10368u64;
        if nat_lt(a193, a194) {
            let a195 = v0;
            let a196 = 1u64;
            if nat_eq(a195, a196) {
                let a197 = v2;
                let a198 = 0u64;
                if if nat_eq(a197, a198) {
                    let a199 = v3;
                    let a200 = 9u64;
                    nat_lt(a199, a200)
                } else {
                    false
                } {
                    Ok(Some(f25(v1, v3)?))
                } else {
                    Ok(None::<u64>)
                }
            } else {
                let a201 = v0;
                let a202 = 2u64;
                if nat_eq(a201, a202) {
                    let a203 = v2;
                    let a204 = 4u64;
                    if if nat_lt(a203, a204) {
                        let a205 = v3;
                        let a206 = 10u64;
                        nat_lt(a205, a206)
                    } else {
                        false
                    } {
                        Ok(Some({
                            let a207 = v3;
                            let a208 = 4u64;
                            if nat_lt(a207, a208) {
                                f26(v1, v2, v3)?
                            } else {
                                let a209 = v3;
                                let a210 = 8u64;
                                if nat_lt(a209, a210) {
                                    f27(v1, v2, {
                                        let a211 = v3;
                                        let a212 = 4u64;
                                        nat_sub(a211, a212)
                                    })?
                                } else {
                                    let a213 = v3;
                                    let a214 = 8u64;
                                    if nat_eq(a213, a214) {
                                        f28(v1, v2)?
                                    } else {
                                        f29(v1, v2)?
                                    }
                                }
                            }
                        }))
                    } else {
                        Ok(None::<u64>)
                    }
                } else {
                    let a215 = v0;
                    let a216 = 3u64;
                    if nat_eq(a215, a216) {
                        let a217 = v2;
                        let a218 = 2u64;
                        if if nat_lt(a217, a218) {
                            let a219 = v3;
                            let a220 = 256u64;
                            nat_lt(a219, a220)
                        } else {
                            false
                        } {
                            Ok(Some(f30(v1, v2, v3)?))
                        } else {
                            Ok(None::<u64>)
                        }
                    } else {
                        let a221 = v0;
                        let a222 = 4u64;
                        if if nat_eq(a221, a222) {
                            true
                        } else {
                            let a223 = v0;
                            let a224 = 5u64;
                            nat_eq(a223, a224)
                        } {
                            let a225 = v3;
                            let a226 = 2u64;
                            if nat_lt(a225, a226) {
                                let a227 = v2;
                                let a228 = 0u64;
                                if nat_eq(a227, a228) {
                                    let a229 = v3;
                                    let a230 = 0u64;
                                    if nat_eq(a229, a230) {
                                        Ok(Some(f31(v1, {
                                            let a231 = v0;
                                            let a232 = 4u64;
                                            if nat_eq(a231, a232) {
                                                0u64
                                            } else {
                                                1u64
                                            }
                                        })?))
                                    } else {
                                        Ok(None::<u64>)
                                    }
                                } else {
                                    let a235 = v2;
                                    let a236 = f31(v1, {
                                        let a233 = v0;
                                        let a234 = 4u64;
                                        if nat_eq(a233, a234) {
                                            0u64
                                        } else {
                                            1u64
                                        }
                                    })?;
                                    if nat_le(a235, a236) {
                                        Ok(Some({
                                            let a237 = v3;
                                            let a238 = 0u64;
                                            if nat_eq(a237, a238) {
                                                let a241 = {
                                                    let a239 = v2;
                                                    let a240 = 1u64;
                                                    nat_sub(a239, a240)
                                                };
                                                let a242 = 256u64;
                                                nat_mul(a241, a242)?
                                            } else {
                                                let v4: u64 = {
                                                    let a249 = f28(v1, {
                                                        let a243 = v0;
                                                        let a244 = 4u64;
                                                        if nat_eq(a243, a244) {
                                                            0u64
                                                        } else {
                                                            2u64
                                                        }
                                                    })?;
                                                    let a250 = {
                                                        let a247 = {
                                                            let a245 = v2;
                                                            let a246 = 1u64;
                                                            nat_sub(a245, a246)
                                                        };
                                                        let a248 = 256u64;
                                                        nat_mul(a247, a248)?
                                                    };
                                                    nat_sub(a249, a250)
                                                };
                                                let a251 = v4;
                                                let a252 = 256u64;
                                                if nat_lt(a251, a252) {
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

pub fn f4(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a253 = v0;
    let a254 = 0u64;
    if nat_eq(a253, a254) {
        let a255 = v1;
        let a256 = 0u64;
        if if if nat_eq(a255, a256) {
            let a257 = v2;
            let a258 = 0u64;
            nat_eq(a257, a258)
        } else {
            false
        } {
            let a259 = v3;
            let a260 = 5u64;
            nat_lt(a259, a260)
        } else {
            false
        } {
            let a261 = v3;
            let a262 = 0u64;
            if nat_eq(a261, a262) {
                Ok(Some(160u64))
            } else {
                f5(0u64, 0u64, 0u64, v3)
            }
        } else {
            Ok(None::<u64>)
        }
    } else {
        let a263 = v1;
        let a264 = 160u64;
        if nat_lt(a263, a264) {
            let a265 = v0;
            let a266 = 2u64;
            if nat_eq(a265, a266) {
                let a267 = v2;
                let a268 = 4u64;
                if if nat_lt(a267, a268) {
                    let a269 = v3;
                    let a270 = 10u64;
                    nat_lt(a269, a270)
                } else {
                    false
                } {
                    Ok(Some({
                        let a271 = v3;
                        let a272 = 4u64;
                        if nat_lt(a271, a272) {
                            f32(v1, v2, v3)?
                        } else {
                            let a273 = v3;
                            let a274 = 8u64;
                            if nat_lt(a273, a274) {
                                f33(v1, v2, {
                                    let a275 = v3;
                                    let a276 = 4u64;
                                    nat_sub(a275, a276)
                                })?
                            } else {
                                let a277 = v3;
                                let a278 = 8u64;
                                if nat_eq(a277, a278) {
                                    f34(v1, v2)?
                                } else {
                                    f35(v1, v2)?
                                }
                            }
                        }
                    }))
                } else {
                    Ok(None::<u64>)
                }
            } else {
                f5(v0, f36(v1)?, v2, v3)
            }
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f5(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a279 = v0;
    let a280 = 0u64;
    if nat_eq(a279, a280) {
        let a281 = v1;
        let a282 = 0u64;
        if if if nat_eq(a281, a282) {
            let a283 = v2;
            let a284 = 0u64;
            nat_eq(a283, a284)
        } else {
            false
        } {
            let a285 = v3;
            let a286 = 5u64;
            nat_lt(a285, a286)
        } else {
            false
        } {
            Ok(Some({
                let a287 = v3;
                let a288 = 2u64;
                if nat_lt(a287, a288) {
                    let a289 = v3;
                    let a290 = 0u64;
                    if nat_eq(a289, a290) {
                        3456u64
                    } else {
                        9u64
                    }
                } else {
                    let a291 = v3;
                    let a292 = 4u64;
                    if nat_lt(a291, a292) {
                        let a293 = v3;
                        let a294 = 2u64;
                        if nat_eq(a293, a294) {
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
        let a295 = v1;
        let a296 = 3456u64;
        if if nat_lt(a295, a296) {
            f37(v1)?
        } else {
            false
        } {
            let a297 = v0;
            let a298 = 2u64;
            if nat_lt(a297, a298) {
                let a299 = v0;
                let a300 = 1u64;
                if nat_eq(a299, a300) {
                    let a301 = v2;
                    let a302 = 0u64;
                    if if nat_eq(a301, a302) {
                        let a303 = v3;
                        let a304 = 9u64;
                        nat_lt(a303, a304)
                    } else {
                        false
                    } {
                        Ok(Some(f38(v1, v3)?))
                    } else {
                        Ok(None::<u64>)
                    }
                } else {
                    Ok(None::<u64>)
                }
            } else {
                let a305 = v0;
                let a306 = 4u64;
                if nat_lt(a305, a306) {
                    let a307 = v0;
                    let a308 = 2u64;
                    if nat_eq(a307, a308) {
                        let a309 = v2;
                        let a310 = 2u64;
                        if if nat_lt(a309, a310) {
                            let a311 = v3;
                            let a312 = 10u64;
                            nat_lt(a311, a312)
                        } else {
                            false
                        } {
                            Ok(Some({
                                let a313 = v3;
                                let a314 = 4u64;
                                if nat_lt(a313, a314) {
                                    f39(v1, v2, v3)?
                                } else {
                                    let a315 = v3;
                                    let a316 = 8u64;
                                    if nat_lt(a315, a316) {
                                        f40(v1, v2, {
                                            let a317 = v3;
                                            let a318 = 4u64;
                                            nat_sub(a317, a318)
                                        })?
                                    } else {
                                        let a319 = v3;
                                        let a320 = 8u64;
                                        if nat_eq(a319, a320) {
                                            f41(v1, v2)?
                                        } else {
                                            f42(v1, v2)?
                                        }
                                    }
                                }
                            }))
                        } else {
                            Ok(None::<u64>)
                        }
                    } else {
                        let a321 = v2;
                        let a322 = 0u64;
                        if if nat_eq(a321, a322) {
                            let a323 = v3;
                            let a324 = 256u64;
                            nat_lt(a323, a324)
                        } else {
                            false
                        } {
                            Ok(Some(f43(v1, v3)?))
                        } else {
                            Ok(None::<u64>)
                        }
                    }
                } else {
                    let a325 = v0;
                    let a326 = 4u64;
                    if nat_eq(a325, a326) {
                        let a327 = v3;
                        let a328 = 2u64;
                        if nat_lt(a327, a328) {
                            let a329 = v2;
                            let a330 = 0u64;
                            if nat_eq(a329, a330) {
                                let a331 = v3;
                                let a332 = 0u64;
                                if nat_eq(a331, a332) {
                                    Ok(Some(f44(v1)?))
                                } else {
                                    Ok(None::<u64>)
                                }
                            } else {
                                let a333 = v2;
                                let a334 = f44(v1)?;
                                if nat_le(a333, a334) {
                                    Ok(Some({
                                        let a335 = v3;
                                        let a336 = 0u64;
                                        if nat_eq(a335, a336) {
                                            let a339 = {
                                                let a337 = v2;
                                                let a338 = 1u64;
                                                nat_sub(a337, a338)
                                            };
                                            let a340 = 256u64;
                                            nat_mul(a339, a340)?
                                        } else {
                                            let v4: u64 = {
                                                let a345 = f41(v1, 0u64)?;
                                                let a346 = {
                                                    let a343 = {
                                                        let a341 = v2;
                                                        let a342 = 1u64;
                                                        nat_sub(a341, a342)
                                                    };
                                                    let a344 = 256u64;
                                                    nat_mul(a343, a344)?
                                                };
                                                nat_sub(a345, a346)
                                            };
                                            let a347 = v4;
                                            let a348 = 256u64;
                                            if nat_lt(a347, a348) {
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

pub fn f6(v0: u64, v1: u64) -> R<u64> {
    let a349 = v1;
    let a350 = 12u64;
    if nat_lt(a349, a350) {
        let a351 = v1;
        let a352 = 4u64;
        if nat_lt(a351, a352) {
            let a353 = v1;
            let a354 = 2u64;
            if nat_lt(a353, a354) {
                let a355 = v1;
                let a356 = 1u64;
                if nat_lt(a355, a356) {
                    f45(v0)
                } else {
                    let a357 = f45(v0)?;
                    let a358 = 4u64;
                    if nat_lt(a357, a358) {
                        let a359 = f45(v0)?;
                        let a360 = 2u64;
                        if nat_lt(a359, a360) {
                            let a361 = f45(v0)?;
                            let a362 = 1u64;
                            if nat_lt(a361, a362) {
                                Ok(2u64)
                            } else {
                                Ok(5u64)
                            }
                        } else {
                            let a363 = f45(v0)?;
                            let a364 = 3u64;
                            if nat_lt(a363, a364) {
                                Ok(6u64)
                            } else {
                                Ok(7u64)
                            }
                        }
                    } else {
                        let a365 = f45(v0)?;
                        let a366 = 6u64;
                        if nat_lt(a365, a366) {
                            let a367 = f45(v0)?;
                            let a368 = 5u64;
                            if nat_lt(a367, a368) {
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
                let a369 = v1;
                let a370 = 3u64;
                if nat_lt(a369, a370) {
                    let a371 = f45(v0)?;
                    let a372 = 6u64;
                    if nat_eq(a371, a372) {
                        Ok(2u64)
                    } else {
                        Ok(0u64)
                    }
                } else {
                    Ok(f46(v0))
                }
            }
        } else {
            Ok(f47(v0, {
                let a375 = {
                    let a373 = v1;
                    let a374 = 4u64;
                    nat_sub(a373, a374)
                };
                let a376 = 4u64;
                let a377 = 0u64;
                nat_quot(a375, a376, a377)
            }, {
                let a380 = {
                    let a378 = v1;
                    let a379 = 4u64;
                    nat_sub(a378, a379)
                };
                let a381 = 4u64;
                let a382 = 0u64;
                nat_rem(a380, a381, a382)
            }))
        }
    } else {
        let a383 = v1;
        let a384 = 20u64;
        if nat_lt(a383, a384) {
            f48(v0, {
                let a387 = {
                    let a385 = v1;
                    let a386 = 12u64;
                    nat_sub(a385, a386)
                };
                let a388 = 4u64;
                let a389 = 0u64;
                nat_quot(a387, a388, a389)
            }, {
                let a392 = {
                    let a390 = v1;
                    let a391 = 12u64;
                    nat_sub(a390, a391)
                };
                let a393 = 4u64;
                let a394 = 0u64;
                nat_rem(a392, a393, a394)
            })
        } else {
            let a395 = v1;
            let a396 = 22u64;
            if nat_lt(a395, a396) {
                f7(v0, {
                    let a397 = v1;
                    let a398 = 20u64;
                    nat_sub(a397, a398)
                })
            } else {
                let a399 = v1;
                let a400 = 22u64;
                if nat_eq(a399, a400) {
                    let a401 = v0;
                    let a402 = 0u64;
                    if nat_eq(a401, a402) {
                        Ok(1u64)
                    } else {
                        let a403 = v0;
                        let a404 = 108u64;
                        if nat_eq(a403, a404) {
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

pub fn f7(v0: u64, v1: u64) -> R<u64> {
    let a405 = v1;
    let a406 = 0u64;
    if nat_eq(a405, a406) {
        let a407 = f49(v0);
        let a408 = 12u64;
        nat_mul(a407, a408)
    } else {
        let a409 = 96u64;
        let a410 = v0;
        if nat_le(a409, a410) {
            Ok(0u64)
        } else {
            let a411 = f50(v0);
            let a412 = 2u64;
            if nat_lt(a411, a412) {
                let a413 = f50(v0);
                let a414 = 1u64;
                if nat_lt(a413, a414) {
                    let a415 = f49(v0);
                    let a416 = 12u64;
                    nat_mul(a415, a416)
                } else {
                    Ok(1u64)
                }
            } else {
                let a417 = f50(v0);
                let a418 = 3u64;
                if nat_lt(a417, a418) {
                    Ok(f49(v0))
                } else {
                    Ok(6u64)
                }
            }
        }
    }
}

pub fn f8(v0: u64, v1: u64) -> R<u64> {
    let a419 = v1;
    let a420 = 16u64;
    if nat_lt(a419, a420) {
        Ok(f51(f52(v0, v1)?))
    } else {
        Ok(f53({
            let a421 = f54(v0);
            let a422 = f55(v1)?;
            nat_add(a421, a422)?
        }))
    }
}

pub fn f9(v0: u64, v1: u64, v2: u64) -> R<Option<u64>> {
    let v3: u64 = f13(v0, 0u64)?;
    let v4: u64 = {
        let a425 = {
            let a423 = v3;
            let a424 = 511u64;
            nat_add(a423, a424)?
        };
        let a426 = 512u64;
        let a427 = 0u64;
        nat_quot(a425, a426, a427)
    };
    let a428 = v1;
    let a429 = 0u64;
    if nat_eq(a428, a429) {
        let a430 = v2;
        let a431 = 0u64;
        if nat_eq(a430, a431) {
            Ok(Some(v4))
        } else {
            Ok(None::<u64>)
        }
    } else {
        let a432 = v1;
        let a433 = v4;
        if if nat_le(a432, a433) {
            let a434 = v2;
            let a435 = 2u64;
            nat_lt(a434, a435)
        } else {
            false
        } {
            let v5: u64 = {
                let a438 = {
                    let a436 = v1;
                    let a437 = 1u64;
                    nat_sub(a436, a437)
                };
                let a439 = 512u64;
                nat_mul(a438, a439)?
            };
            Ok(Some({
                let a440 = v2;
                let a441 = 0u64;
                if nat_eq(a440, a441) {
                    v5
                } else {
                    let v6: u64 = {
                        let a442 = v3;
                        let a443 = v5;
                        nat_sub(a442, a443)
                    };
                    let a444 = v6;
                    let a445 = 512u64;
                    if nat_lt(a444, a445) {
                        v6
                    } else {
                        512u64
                    }
                }
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f10(v0: u64, v1: u64) -> u64 {
    let a446 = v1;
    let a447 = 0u64;
    if nat_eq(a446, a447) {
        f56(v0)
    } else {
        let a448 = v1;
        let a449 = 1u64;
        if nat_eq(a448, a449) {
            f57(v0)
        } else {
            let a450 = v1;
            let a451 = 2u64;
            if nat_eq(a450, a451) {
                f58(v0)
            } else {
                let a452 = v1;
                let a453 = 3u64;
                if nat_eq(a452, a453) {
                    f59(v0)
                } else {
                    let a454 = v1;
                    let a455 = 4u64;
                    if nat_eq(a454, a455) {
                        f11(v0, 0u64)
                    } else {
                        let a456 = v1;
                        let a457 = 5u64;
                        if nat_eq(a456, a457) {
                            f11(v0, 1u64)
                        } else {
                            let a458 = v1;
                            let a459 = 6u64;
                            if nat_eq(a458, a459) {
                                f11(v0, 3u64)
                            } else {
                                let a460 = v1;
                                let a461 = 7u64;
                                if nat_eq(a460, a461) {
                                    v0
                                } else {
                                    let a462 = v1;
                                    let a463 = 8u64;
                                    if nat_eq(a462, a463) {
                                        f60(v0)
                                    } else {
                                        f61(v0)
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

pub fn f11(v0: u64, v1: u64) -> u64 {
    let a464 = 240u64;
    let a465 = v0;
    if if nat_le(a464, a465) {
        let a471 = v1;
        let a472 = {
            let a468 = {
                let a466 = v0;
                let a467 = 240u64;
                nat_sub(a466, a467)
            };
            let a469 = 4u64;
            let a470 = 0u64;
            nat_rem(a468, a469, a470)
        };
        nat_eq(a471, a472)
    } else {
        false
    } {
        0u64
    } else {
        let a473 = v1;
        let a474 = 0u64;
        if nat_eq(a473, a474) {
            5u64
        } else {
            let a475 = v1;
            let a476 = 1u64;
            if nat_eq(a475, a476) {
                let a477 = f59(v0);
                let a478 = 3u64;
                if nat_eq(a477, a478) {
                    3u64
                } else {
                    2u64
                }
            } else {
                let a479 = v1;
                let a480 = 2u64;
                if nat_eq(a479, a480) {
                    2u64
                } else {
                    1u64
                }
            }
        }
    }
}

pub fn f12(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = f62(v0);
    let v4: u64 = {
        let a481 = f58(v0);
        let a482 = 8u64;
        if if nat_eq(a481, a482) {
            let a483 = v1;
            let a484 = 0u64;
            if nat_eq(a483, a484) {
                true
            } else {
                let a485 = v1;
                let a486 = 3u64;
                if nat_eq(a485, a486) {
                    let a487 = f57(v0);
                    let a488 = 1u64;
                    nat_eq(a487, a488)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let a489 = 2u64;
            let a490 = v3;
            nat_mul(a489, a490)?
        } else {
            v3
        }
    };
    let v5: u64 = {
        let a491 = f11(v0, 0u64);
        let a492 = v4;
        nat_mul(a491, a492)?
    };
    let v6: u64 = f63(v0, v1);
    let v7: u64 = {
        let a493 = f58(v0);
        let a494 = 9u64;
        if if if nat_eq(a493, a494) {
            let a495 = v1;
            let a496 = 3u64;
            if nat_eq(a495, a496) {
                true
            } else {
                let a497 = f57(v0);
                let a498 = 1u64;
                nat_eq(a497, a498)
            }
        } else {
            false
        } {
            true
        } else {
            let a499 = f63(v0, v1);
            let a500 = 4u64;
            nat_eq(a499, a500)
        } {
            0u64
        } else {
            let a501 = v6;
            let a502 = 2u64;
            if nat_eq(a501, a502) {
                let a507 = {
                    let a505 = {
                        let a503 = v5;
                        let a504 = v3;
                        nat_add(a503, a504)?
                    };
                    let a506 = f11(v0, 2u64);
                    nat_mul(a505, a506)?
                };
                let a508 = v3;
                nat_add(a507, a508)?
            } else {
                let a509 = v6;
                let a510 = 3u64;
                if nat_eq(a509, a510) {
                    let a511 = 5u64;
                    let a512 = v5;
                    nat_mul(a511, a512)?
                } else {
                    let a513 = v6;
                    let a514 = 1u64;
                    if nat_eq(a513, a514) {
                        let a515 = v5;
                        let a516 = v3;
                        nat_add(a515, a516)?
                    } else {
                        v5
                    }
                }
            }
        }
    };
    let v8: u64 = {
        let a517 = v6;
        let a518 = 2u64;
        if nat_eq(a517, a518) {
            let a519 = v5;
            let a520 = v3;
            nat_add(a519, a520)?
        } else {
            let a521 = v6;
            let a522 = 3u64;
            if nat_eq(a521, a522) {
                let a523 = 7u64;
                let a524 = v5;
                nat_mul(a523, a524)?
            } else {
                let a525 = v6;
                let a526 = 4u64;
                if nat_eq(a525, a526) {
                    let a527 = v5;
                    let a528 = v3;
                    nat_add(a527, a528)?
                } else {
                    let a533 = {
                        let a529 = v7;
                        let a530 = f11(v0, 1u64);
                        nat_mul(a529, a530)?
                    };
                    let a534 = {
                        let a531 = v6;
                        let a532 = 1u64;
                        if nat_eq(a531, a532) {
                            v3
                        } else {
                            0u64
                        }
                    };
                    nat_add(a533, a534)?
                }
            }
        }
    };
    let a535 = v2;
    let a536 = 0u64;
    if nat_eq(a535, a536) {
        Ok(v4)
    } else {
        let a537 = v2;
        let a538 = 1u64;
        if nat_eq(a537, a538) {
            Ok(v7)
        } else {
            let a539 = v2;
            let a540 = 2u64;
            if nat_eq(a539, a540) {
                Ok(v8)
            } else {
                let a545 = {
                    let a541 = v7;
                    let a542 = f11(v0, 1u64);
                    nat_mul(a541, a542)?
                };
                let a546 = {
                    let a543 = v8;
                    let a544 = f11(v0, 2u64);
                    nat_mul(a543, a544)?
                };
                nat_add(a545, a546)
            }
        }
    }
}

pub fn f13(v0: u64, v1: u64) -> R<u64> {
    let a547 = f14(v0)?;
    let a548 = 0u64;
    if nat_eq(a547, a548) {
        Ok(0u64)
    } else {
        let a571 = {
            let a565 = {
                let a559 = {
                    let a553 = f62(v0);
                    let a554 = {
                        let a551 = {
                            let a549 = f11(v0, 0u64);
                            let a550 = 1u64;
                            nat_sub(a549, a550)
                        };
                        let a552 = f12(v0, v1, 0u64)?;
                        nat_mul(a551, a552)?
                    };
                    nat_add(a553, a554)?
                };
                let a560 = {
                    let a557 = {
                        let a555 = f11(v0, 1u64);
                        let a556 = 1u64;
                        nat_sub(a555, a556)
                    };
                    let a558 = f12(v0, v1, 1u64)?;
                    nat_mul(a557, a558)?
                };
                nat_add(a559, a560)?
            };
            let a566 = {
                let a563 = {
                    let a561 = f11(v0, 2u64);
                    let a562 = 1u64;
                    nat_sub(a561, a562)
                };
                let a564 = f12(v0, v1, 2u64)?;
                nat_mul(a563, a564)?
            };
            nat_add(a565, a566)?
        };
        let a572 = {
            let a569 = {
                let a567 = f11(v0, 3u64);
                let a568 = 1u64;
                nat_sub(a567, a568)
            };
            let a570 = f12(v0, v1, 3u64)?;
            nat_mul(a569, a570)?
        };
        nat_add(a571, a572)
    }
}

pub fn f14(v0: u64) -> R<u64> {
    let a577 = {
        let a573 = f11(v0, 0u64);
        let a574 = f11(v0, 1u64);
        nat_mul(a573, a574)?
    };
    let a578 = {
        let a575 = f11(v0, 2u64);
        let a576 = f11(v0, 3u64);
        nat_mul(a575, a576)?
    };
    nat_mul(a577, a578)
}

pub fn f15(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f62(v0);
    let v3: u64 = {
        let a579 = v1;
        let a580 = v2;
        let a581 = 0u64;
        nat_rem(a579, a580, a581)
    };
    let a594 = {
        let a591 = f64(v0, {
            let a582 = v1;
            let a583 = v2;
            let a584 = 0u64;
            nat_quot(a582, a583, a584)
        })?;
        let a592 = {
            let a585 = v3;
            let a586 = 0u64;
            if nat_eq(a585, a586) {
                1u64
            } else {
                let a587 = v3;
                let a588 = 1u64;
                if nat_eq(a587, a588) {
                    256u64
                } else {
                    let a589 = v3;
                    let a590 = 2u64;
                    if nat_eq(a589, a590) {
                        65536u64
                    } else {
                        16777216u64
                    }
                }
            }
        };
        let a593 = 0u64;
        nat_quot(a591, a592, a593)
    };
    let a595 = 256u64;
    let a596 = 0u64;
    Ok(nat_rem(a594, a595, a596))
}

pub fn f16(v0: u64) -> u64 {
    let a597 = v0;
    let a598 = 0u64;
    if nat_eq(a597, a598) {
        15360u64
    } else {
        1920u64
    }
}

pub fn f17(v0: u64) -> u64 {
    let a599 = v0;
    let a600 = 0u64;
    if nat_eq(a599, a600) {
        256u64
    } else {
        192u64
    }
}

pub fn f18(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a601 = v2;
    let a602 = 5u64;
    if nat_lt(a601, a602) {
        let a603 = v2;
        let a604 = 2u64;
        if nat_lt(a603, a604) {
            let a605 = v2;
            let a606 = 1u64;
            if nat_lt(a605, a606) {
                Ok(f65(v0, v1))
            } else {
                f66(v0, v1)
            }
        } else {
            let a607 = v2;
            let a608 = 3u64;
            if nat_lt(a607, a608) {
                Ok(f67(v0, v1))
            } else {
                let a609 = v2;
                let a610 = 4u64;
                if nat_lt(a609, a610) {
                    Ok(f68(v1))
                } else {
                    f69(v1)
                }
            }
        }
    } else {
        let a611 = v2;
        let a612 = 7u64;
        if nat_lt(a611, a612) {
            let a613 = v2;
            let a614 = 6u64;
            if nat_lt(a613, a614) {
                f70(v1)
            } else {
                f71(v1)
            }
        } else {
            let a615 = v2;
            let a616 = 8u64;
            if nat_lt(a615, a616) {
                f72(v0, v1)
            } else {
                let a617 = v2;
                let a618 = 9u64;
                if nat_lt(a617, a618) {
                    Ok(0u64)
                } else {
                    let a619 = v0;
                    let a620 = 0u64;
                    if nat_eq(a619, a620) {
                        Ok(f73({
                            let a621 = f67(v0, v1);
                            let a622 = 5u64;
                            let a623 = 0u64;
                            nat_quot(a621, a622, a623)
                        }))
                    } else {
                        f74(v0, v1)
                    }
                }
            }
        }
    }
}

pub fn f19(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    if if f75(v0, v1, v2)? {
        let a624 = f68(v1);
        let a625 = 3u64;
        nat_eq(a624, a625)
    } else {
        false
    } {
        let a626 = v3;
        let a627 = 2u64;
        if nat_lt(a626, a627) {
            let a628 = v3;
            let a629 = 0u64;
            if nat_eq(a628, a629) {
                Ok(1u64)
            } else {
                f76(v0, v1)
            }
        } else {
            Ok(1u64)
        }
    } else {
        f77(v0, v1, v3)
    }
}

pub fn f20(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a630 = v3;
    let a631 = 2u64;
    if nat_lt(a630, a631) {
        let a632 = v3;
        let a633 = 0u64;
        if nat_eq(a632, a633) {
            f78(v0, v1, v2)
        } else {
            f79(v0, v1, v2)
        }
    } else {
        let a634 = v3;
        let a635 = 2u64;
        if nat_eq(a634, a635) {
            f80(v0, v1, v2)
        } else {
            f81(v0, v1, v2)
        }
    }
}

pub fn f21(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let m636 = f82(Adt0::C0(f19(v0, v1, v2, 0u64)?, f19(v0, v1, v2, 1u64)?, f19(v0, v1, v2, 2u64)?, f19(v0, v1, v2, 3u64)?, f20(v0, v1, v2, 0u64)?, f20(v0, v1, v2, 1u64)?, f20(v0, v1, v2, 2u64)?, f20(v0, v1, v2, 3u64)?, 1u64, f83(v0, v1, v2)?))?;
    match m636 {
        Adt1::C0 => {
            Ok(0u64)
        }
        Adt1::C1(v3) => {
            Ok(v3)
        }
    }
}

pub fn f22(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a641 = {
        let a637 = f19(v0, v1, v2, 0u64)?;
        let a638 = f19(v0, v1, v2, 1u64)?;
        nat_mul(a637, a638)?
    };
    let a642 = {
        let a639 = f19(v0, v1, v2, 2u64)?;
        let a640 = f19(v0, v1, v2, 3u64)?;
        nat_mul(a639, a640)?
    };
    nat_mul(a641, a642)
}

pub fn f23(v0: u64, v1: u64, v2: u64) -> R<u64> {
    f84(v0, f72(v0, v1)?, v2)
}

pub fn f24(v0: u64, v1: u64) -> R<u64> {
    let a647 = {
        let a645 = f21(v0, v1, 0u64)?;
        let a646 = {
            let a643 = f17(v0);
            let a644 = 1u64;
            nat_sub(a643, a644)
        };
        nat_add(a645, a646)?
    };
    let a648 = f17(v0);
    let a649 = 0u64;
    Ok(nat_quot(a647, a648, a649))
}

pub fn f25(v0: u64, v1: u64) -> R<u64> {
    let a650 = v1;
    let a651 = 0u64;
    if nat_eq(a650, a651) {
        f85(v0)
    } else {
        let a652 = v1;
        let a653 = 1u64;
        if nat_eq(a652, a653) {
            Ok(f86(v0))
        } else {
            let a654 = v1;
            let a655 = 2u64;
            if nat_eq(a654, a655) {
                Ok(0u64)
            } else {
                let a656 = v1;
                let a657 = 3u64;
                if nat_eq(a656, a657) {
                    Ok(f87(v0))
                } else {
                    let a658 = v1;
                    let a659 = 4u64;
                    if nat_eq(a658, a659) {
                        Ok(f88(v0))
                    } else {
                        let a660 = v1;
                        let a661 = 5u64;
                        if nat_eq(a660, a661) {
                            Ok(f89(v0))
                        } else {
                            let a662 = v1;
                            let a663 = 6u64;
                            if nat_eq(a662, a663) {
                                f90(v0)
                            } else {
                                let a664 = v1;
                                let a665 = 7u64;
                                if nat_eq(a664, a665) {
                                    f91(v0, 0u64)
                                } else {
                                    f91(v0, 1u64)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f26(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a666 = v1;
    let a667 = 2u64;
    if nat_lt(a666, a667) {
        let a668 = v1;
        let a669 = 0u64;
        if nat_eq(a668, a669) {
            f92(v0, v2)
        } else {
            let a670 = f87(v0);
            let a671 = 3u64;
            if nat_eq(a670, a671) {
                let a672 = v2;
                let a673 = 2u64;
                if nat_lt(a672, a673) {
                    let a674 = v2;
                    let a675 = 0u64;
                    if nat_eq(a674, a675) {
                        f93(v0)
                    } else {
                        let a676 = f94(v0)?;
                        let a677 = f93(v0)?;
                        let a678 = 0u64;
                        Ok(nat_quot(a676, a677, a678))
                    }
                } else {
                    Ok(1u64)
                }
            } else {
                f92(v0, v2)
            }
        }
    } else {
        let a679 = v1;
        let a680 = 2u64;
        if nat_eq(a679, a680) {
            f95(v0, v2)
        } else {
            let a681 = v2;
            let a682 = f86(v0);
            if nat_eq(a681, a682) {
                let a683 = f92(v0, v2)?;
                let a684 = f95(v0, v2)?;
                nat_add(a683, a684)
            } else {
                f92(v0, v2)
            }
        }
    }
}

pub fn f27(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a685 = v2;
    let a686 = 2u64;
    if nat_lt(a685, a686) {
        let a687 = v2;
        let a688 = 0u64;
        if nat_eq(a687, a688) {
            f96(v0, v1)
        } else {
            f97(v0, v1)
        }
    } else {
        let a689 = v2;
        let a690 = 2u64;
        if nat_eq(a689, a690) {
            f98(v0, v1)
        } else {
            f99(v0, v1)
        }
    }
}

pub fn f28(v0: u64, v1: u64) -> R<u64> {
    let m691 = f82(Adt0::C0(f26(v0, v1, 0u64)?, f26(v0, v1, 1u64)?, f26(v0, v1, 2u64)?, f26(v0, v1, 3u64)?, f27(v0, v1, 0u64)?, f27(v0, v1, 1u64)?, f27(v0, v1, 2u64)?, f27(v0, v1, 3u64)?, f93(v0)?, f100(v0)?))?;
    match m691 {
        Adt1::C0 => {
            Ok(0u64)
        }
        Adt1::C1(v2) => {
            Ok(v2)
        }
    }
}

pub fn f29(v0: u64, v1: u64) -> R<u64> {
    let a696 = {
        let a692 = f26(v0, v1, 0u64)?;
        let a693 = f26(v0, v1, 1u64)?;
        nat_mul(a692, a693)?
    };
    let a697 = {
        let a694 = f26(v0, v1, 2u64)?;
        let a695 = f26(v0, v1, 3u64)?;
        nat_mul(a694, a695)?
    };
    nat_mul(a696, a697)
}

pub fn f30(v0: u64, v1: u64, v2: u64) -> R<u64> {
    f101(f91(v0, v1)?, v2)
}

pub fn f31(v0: u64, v1: u64) -> R<u64> {
    let a702 = {
        let a700 = f28(v0, {
            let a698 = v1;
            let a699 = 0u64;
            if nat_eq(a698, a699) {
                0u64
            } else {
                2u64
            }
        })?;
        let a701 = 255u64;
        nat_add(a700, a701)?
    };
    let a703 = 256u64;
    let a704 = 0u64;
    Ok(nat_quot(a702, a703, a704))
}

pub fn f32(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a705 = v1;
    let a706 = 2u64;
    if nat_lt(a705, a706) {
        f39(f36(v0)?, v1, v2)
    } else {
        let a720 = f39(f36(v0)?, 0u64, v2)?;
        let a721 = {
            let a707 = v2;
            let a708 = 2u64;
            if nat_lt(a707, a708) {
                let a709 = v2;
                let a710 = 0u64;
                if nat_eq(a709, a710) {
                    2u64
                } else {
                    let a714 = {
                        let a711 = f102(v0)?;
                        let a712 = 2u64;
                        let a713 = 0u64;
                        nat_rem(a711, a712, a713)
                    };
                    let a715 = 1u64;
                    if nat_eq(a714, a715) {
                        3u64
                    } else {
                        1u64
                    }
                }
            } else {
                let a716 = v2;
                let a717 = 2u64;
                if nat_eq(a716, a717) {
                    let a718 = 2u64;
                    let a719 = f102(v0)?;
                    if nat_le(a718, a719) {
                        2u64
                    } else {
                        1u64
                    }
                } else {
                    2u64
                }
            }
        };
        nat_mul(a720, a721)
    }
}

pub fn f33(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a722 = v1;
    let a723 = 2u64;
    if nat_lt(a722, a723) {
        f40(f36(v0)?, v1, v2)
    } else {
        let a724 = v2;
        let a725 = 2u64;
        if nat_lt(a724, a725) {
            let a726 = v2;
            let a727 = 0u64;
            if nat_eq(a726, a727) {
                f103(v0)
            } else {
                f104(v0, v1)
            }
        } else {
            let a728 = v2;
            let a729 = 2u64;
            if nat_eq(a728, a729) {
                f105(v0, v1)
            } else {
                f106(v0, v1)
            }
        }
    }
}

pub fn f34(v0: u64, v1: u64) -> R<u64> {
    let a730 = v1;
    let a731 = 2u64;
    if nat_lt(a730, a731) {
        f41(f36(v0)?, v1)
    } else {
        let m732 = f82(Adt0::C0(f32(v0, v1, 0u64)?, f32(v0, v1, 1u64)?, f32(v0, v1, 2u64)?, f32(v0, v1, 3u64)?, f33(v0, v1, 0u64)?, f33(v0, v1, 1u64)?, f33(v0, v1, 2u64)?, f33(v0, v1, 3u64)?, 1u64, f103(v0)?))?;
        match m732 {
            Adt1::C0 => {
                Ok(0u64)
            }
            Adt1::C1(v2) => {
                Ok(v2)
            }
        }
    }
}

pub fn f35(v0: u64, v1: u64) -> R<u64> {
    let a737 = {
        let a733 = f32(v0, v1, 0u64)?;
        let a734 = f32(v0, v1, 1u64)?;
        nat_mul(a733, a734)?
    };
    let a738 = {
        let a735 = f32(v0, v1, 2u64)?;
        let a736 = f32(v0, v1, 3u64)?;
        nat_mul(a735, a736)?
    };
    nat_mul(a737, a738)
}

pub fn f36(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a739 = v0;
        let a740 = 32u64;
        let a741 = 0u64;
        nat_quot(a739, a740, a741)
    };
    let a753 = {
        let a748 = {
            let a742 = v1;
            let a743 = 2u64;
            if nat_lt(a742, a743) {
                v1
            } else {
                let a744 = v1;
                let a745 = 2u64;
                if nat_eq(a744, a745) {
                    23u64
                } else {
                    let a746 = v1;
                    let a747 = 3u64;
                    if nat_eq(a746, a747) {
                        24u64
                    } else {
                        28u64
                    }
                }
            }
        };
        let a749 = 96u64;
        nat_mul(a748, a749)?
    };
    let a754 = {
        let a750 = v0;
        let a751 = 32u64;
        let a752 = 0u64;
        nat_rem(a750, a751, a752)
    };
    nat_add(a753, a754)
}

pub fn f37(v0: u64) -> R<bool> {
    let m755 = f107(f108(v0)?);
    match m755 {
        Adt2::C0 => {
            Ok(false)
        }
        Adt2::C1(_) => {
            Ok(true)
        }
    }
}

pub fn f38(v0: u64, v1: u64) -> R<u64> {
    let a756 = v1;
    let a757 = 0u64;
    if nat_eq(a756, a757) {
        f108(v0)
    } else {
        let a758 = v1;
        let a759 = 1u64;
        if nat_eq(a758, a759) {
            Ok(f109(v0))
        } else {
            let a760 = v1;
            let a761 = 2u64;
            if nat_eq(a760, a761) {
                Ok(0u64)
            } else {
                let a762 = v1;
                let a763 = 3u64;
                if nat_eq(a762, a763) {
                    Ok(f110(v0))
                } else {
                    let a764 = v1;
                    let a765 = 4u64;
                    if nat_eq(a764, a765) {
                        f111(v0)
                    } else {
                        let a766 = v1;
                        let a767 = 5u64;
                        if nat_eq(a766, a767) {
                            f112(v0)
                        } else {
                            let a768 = v1;
                            let a769 = 6u64;
                            if nat_eq(a768, a769) {
                                f113(v0)
                            } else {
                                let a770 = v1;
                                let a771 = 7u64;
                                if nat_eq(a770, a771) {
                                    f114(v0)
                                } else {
                                    let a772 = f109(v0);
                                    let a773 = 1u64;
                                    if if nat_eq(a772, a773) {
                                        let a774 = f110(v0);
                                        let a775 = 3u64;
                                        nat_eq(a774, a775)
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

pub fn f39(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a776 = v1;
    let a777 = 1u64;
    if if nat_eq(a776, a777) {
        let a778 = f110(v0);
        let a779 = 3u64;
        nat_eq(a778, a779)
    } else {
        false
    } {
        let a780 = v2;
        let a781 = 2u64;
        if nat_lt(a780, a781) {
            let a782 = v2;
            let a783 = 0u64;
            if nat_eq(a782, a783) {
                f115(v0)
            } else {
                let a788 = {
                    let a784 = f111(v0)?;
                    let a785 = f112(v0)?;
                    nat_mul(a784, a785)?
                };
                let a789 = {
                    let a786 = 2u64;
                    let a787 = f113(v0)?;
                    nat_mul(a786, a787)?
                };
                nat_mul(a788, a789)
            }
        } else {
            Ok(1u64)
        }
    } else {
        let a790 = v2;
        let a791 = 2u64;
        if nat_lt(a790, a791) {
            let a792 = v2;
            let a793 = 0u64;
            if nat_eq(a792, a793) {
                let a794 = f115(v0)?;
                let a795 = f111(v0)?;
                nat_mul(a794, a795)
            } else {
                f112(v0)
            }
        } else {
            let a796 = v2;
            let a797 = 2u64;
            if nat_eq(a796, a797) {
                Ok(2u64)
            } else {
                f113(v0)
            }
        }
    }
}

pub fn f40(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a798 = v2;
    let a799 = 2u64;
    if nat_lt(a798, a799) {
        let a800 = v2;
        let a801 = 0u64;
        if nat_eq(a800, a801) {
            f116(v0, v1)
        } else {
            f117(v0, v1)
        }
    } else {
        let a802 = v2;
        let a803 = 2u64;
        if nat_eq(a802, a803) {
            f118(v0, v1)
        } else {
            f119(v0, v1)
        }
    }
}

pub fn f41(v0: u64, v1: u64) -> R<u64> {
    let m804 = f82(Adt0::C0(f39(v0, v1, 0u64)?, f39(v0, v1, 1u64)?, f39(v0, v1, 2u64)?, f39(v0, v1, 3u64)?, f40(v0, v1, 0u64)?, f40(v0, v1, 1u64)?, f40(v0, v1, 2u64)?, f40(v0, v1, 3u64)?, f115(v0)?, f120(v0)?))?;
    match m804 {
        Adt1::C0 => {
            Ok(0u64)
        }
        Adt1::C1(v2) => {
            Ok(v2)
        }
    }
}

pub fn f42(v0: u64, v1: u64) -> R<u64> {
    let a809 = {
        let a805 = f39(v0, v1, 0u64)?;
        let a806 = f39(v0, v1, 1u64)?;
        nat_mul(a805, a806)?
    };
    let a810 = {
        let a807 = f39(v0, v1, 2u64)?;
        let a808 = f39(v0, v1, 3u64)?;
        nat_mul(a807, a808)?
    };
    nat_mul(a809, a810)
}

pub fn f43(v0: u64, v1: u64) -> R<u64> {
    f121(f114(v0)?, v1)
}

pub fn f44(v0: u64) -> R<u64> {
    let a813 = {
        let a811 = f41(v0, 0u64)?;
        let a812 = 255u64;
        nat_add(a811, a812)?
    };
    let a814 = 256u64;
    let a815 = 0u64;
    Ok(nat_quot(a813, a814, a815))
}

pub fn f45(v0: u64) -> R<u64> {
    let a816 = v0;
    let a817 = 96u64;
    if nat_lt(a816, a817) {
        let a818 = v0;
        let a819 = 24u64;
        let a820 = 0u64;
        Ok(nat_quot(a818, a819, a820))
    } else {
        let a826 = 4u64;
        let a827 = {
            let a823 = {
                let a821 = v0;
                let a822 = 96u64;
                nat_sub(a821, a822)
            };
            let a824 = 6u64;
            let a825 = 0u64;
            nat_quot(a823, a824, a825)
        };
        nat_add(a826, a827)
    }
}

pub fn f46(v0: u64) -> u64 {
    let a828 = v0;
    let a829 = 96u64;
    if nat_lt(a828, a829) {
        let a833 = {
            let a830 = v0;
            let a831 = 4u64;
            let a832 = 0u64;
            nat_quot(a830, a831, a832)
        };
        let a834 = 2u64;
        let a835 = 0u64;
        nat_rem(a833, a834, a835)
    } else {
        let a836 = v0;
        let a837 = 2u64;
        let a838 = 0u64;
        nat_rem(a836, a837, a838)
    }
}

pub fn f47(v0: u64, v1: u64, v2: u64) -> u64 {
    let a839 = v1;
    let a840 = 0u64;
    if nat_eq(a839, a840) {
        let a841 = v2;
        let a842 = 2u64;
        if nat_lt(a841, a842) {
            let a843 = v2;
            let a844 = 1u64;
            if nat_lt(a843, a844) {
                f49(v0)
            } else {
                3u64
            }
        } else {
            2u64
        }
    } else {
        let a845 = 96u64;
        let a846 = v0;
        if nat_le(a845, a846) {
            0u64
        } else {
            let a847 = v2;
            let a848 = 2u64;
            if nat_lt(a847, a848) {
                let a849 = v2;
                let a850 = 1u64;
                if nat_lt(a849, a850) {
                    let a851 = f50(v0);
                    let a852 = 1u64;
                    if if nat_eq(a851, a852) {
                        true
                    } else {
                        let a853 = f50(v0);
                        let a854 = 3u64;
                        nat_eq(a853, a854)
                    } {
                        1u64
                    } else {
                        f49(v0)
                    }
                } else {
                    let a855 = f50(v0);
                    let a856 = 1u64;
                    if if nat_eq(a855, a856) {
                        true
                    } else {
                        let a857 = f50(v0);
                        let a858 = 2u64;
                        nat_eq(a857, a858)
                    } {
                        1u64
                    } else {
                        3u64
                    }
                }
            } else {
                let a859 = v2;
                let a860 = 3u64;
                if nat_lt(a859, a860) {
                    let a861 = f50(v0);
                    let a862 = 0u64;
                    if nat_eq(a861, a862) {
                        2u64
                    } else {
                        1u64
                    }
                } else {
                    let a863 = f50(v0);
                    let a864 = 1u64;
                    if if nat_eq(a863, a864) {
                        true
                    } else {
                        let a865 = f50(v0);
                        let a866 = 2u64;
                        nat_eq(a865, a866)
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

pub fn f48(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a867 = v1;
    let a868 = 1u64;
    if if nat_eq(a867, a868) {
        let a869 = 96u64;
        let a870 = v0;
        nat_le(a869, a870)
    } else {
        false
    } {
        Ok(0u64)
    } else {
        let v3: u64 = f49(v0);
        let v4: u64 = f46(v0);
        let v5: u64 = {
            let a871 = v1;
            let a872 = 0u64;
            if nat_eq(a871, a872) {
                0u64
            } else {
                f50(v0)
            }
        };
        let a873 = v5;
        let a874 = 2u64;
        if nat_lt(a873, a874) {
            let a875 = v5;
            let a876 = 1u64;
            if nat_lt(a875, a876) {
                let a877 = v2;
                let a878 = 2u64;
                if nat_lt(a877, a878) {
                    let a879 = v2;
                    let a880 = 1u64;
                    if nat_lt(a879, a880) {
                        Ok(4u64)
                    } else {
                        let a883 = {
                            let a881 = v3;
                            let a882 = v4;
                            nat_add(a881, a882)?
                        };
                        let a884 = 4u64;
                        nat_mul(a883, a884)
                    }
                } else {
                    let a885 = v2;
                    let a886 = 3u64;
                    if nat_lt(a885, a886) {
                        let a893 = {
                            let a891 = {
                                let a887 = v3;
                                let a888 = 3u64;
                                nat_mul(a887, a888)?
                            };
                            let a892 = {
                                let a889 = v4;
                                let a890 = 4u64;
                                nat_mul(a889, a890)?
                            };
                            nat_add(a891, a892)?
                        };
                        let a894 = 4u64;
                        nat_mul(a893, a894)
                    } else {
                        let a901 = {
                            let a899 = {
                                let a895 = v3;
                                let a896 = 6u64;
                                nat_mul(a895, a896)?
                            };
                            let a900 = {
                                let a897 = v4;
                                let a898 = 9u64;
                                nat_mul(a897, a898)?
                            };
                            nat_add(a899, a900)?
                        };
                        let a902 = 4u64;
                        nat_mul(a901, a902)
                    }
                }
            } else {
                let a903 = v2;
                let a904 = 2u64;
                if nat_lt(a903, a904) {
                    let a905 = v2;
                    let a906 = 1u64;
                    if nat_lt(a905, a906) {
                        Ok(4u64)
                    } else {
                        let a909 = {
                            let a907 = 1u64;
                            let a908 = v4;
                            nat_add(a907, a908)?
                        };
                        let a910 = 4u64;
                        nat_mul(a909, a910)
                    }
                } else {
                    let a911 = v2;
                    let a912 = 3u64;
                    if nat_lt(a911, a912) {
                        let a917 = {
                            let a915 = 1u64;
                            let a916 = {
                                let a913 = v4;
                                let a914 = 2u64;
                                nat_mul(a913, a914)?
                            };
                            nat_add(a915, a916)?
                        };
                        let a918 = 4u64;
                        nat_mul(a917, a918)
                    } else {
                        let a923 = {
                            let a921 = 1u64;
                            let a922 = {
                                let a919 = v4;
                                let a920 = 3u64;
                                nat_mul(a919, a920)?
                            };
                            nat_add(a921, a922)?
                        };
                        let a924 = 4u64;
                        nat_mul(a923, a924)
                    }
                }
            }
        } else {
            let a925 = v5;
            let a926 = 3u64;
            if nat_lt(a925, a926) {
                let a927 = v2;
                let a928 = 2u64;
                if nat_lt(a927, a928) {
                    let a929 = v2;
                    let a930 = 1u64;
                    if nat_lt(a929, a930) {
                        Ok(4u64)
                    } else {
                        let a933 = {
                            let a931 = v3;
                            let a932 = v4;
                            nat_add(a931, a932)?
                        };
                        let a934 = 4u64;
                        nat_mul(a933, a934)
                    }
                } else {
                    let a935 = v2;
                    let a936 = 3u64;
                    if nat_lt(a935, a936) {
                        let a941 = {
                            let a939 = v3;
                            let a940 = {
                                let a937 = v4;
                                let a938 = 2u64;
                                nat_mul(a937, a938)?
                            };
                            nat_add(a939, a940)?
                        };
                        let a942 = 4u64;
                        nat_mul(a941, a942)
                    } else {
                        let a947 = {
                            let a945 = v3;
                            let a946 = {
                                let a943 = v4;
                                let a944 = 3u64;
                                nat_mul(a943, a944)?
                            };
                            nat_add(a945, a946)?
                        };
                        let a948 = 4u64;
                        nat_mul(a947, a948)
                    }
                }
            } else {
                let a949 = v2;
                let a950 = 2u64;
                if nat_lt(a949, a950) {
                    let a951 = v2;
                    let a952 = 1u64;
                    if nat_lt(a951, a952) {
                        Ok(4u64)
                    } else {
                        let a955 = {
                            let a953 = 1u64;
                            let a954 = v4;
                            nat_add(a953, a954)?
                        };
                        let a956 = 4u64;
                        nat_mul(a955, a956)
                    }
                } else {
                    let a957 = v2;
                    let a958 = 3u64;
                    if nat_lt(a957, a958) {
                        let a963 = {
                            let a961 = 3u64;
                            let a962 = {
                                let a959 = v4;
                                let a960 = 4u64;
                                nat_mul(a959, a960)?
                            };
                            nat_add(a961, a962)?
                        };
                        let a964 = 4u64;
                        nat_mul(a963, a964)
                    } else {
                        let a969 = {
                            let a967 = 3u64;
                            let a968 = {
                                let a965 = v4;
                                let a966 = 5u64;
                                nat_mul(a965, a966)?
                            };
                            nat_add(a967, a968)?
                        };
                        let a970 = 4u64;
                        nat_mul(a969, a970)
                    }
                }
            }
        }
    }
}

pub fn f49(v0: u64) -> u64 {
    let a971 = f122(v0);
    let a972 = 0u64;
    if nat_eq(a971, a972) {
        1u64
    } else {
        let a973 = f122(v0);
        let a974 = 1u64;
        if nat_eq(a973, a974) {
            17u64
        } else {
            257u64
        }
    }
}

pub fn f50(v0: u64) -> u64 {
    let a975 = v0;
    let a976 = 4u64;
    let a977 = 0u64;
    nat_rem(a975, a976, a977)
}

pub fn f51(v0: u64) -> u64 {
    let a978 = v0;
    let a979 = 8u64;
    if nat_lt(a978, a979) {
        let a980 = v0;
        let a981 = 4u64;
        if nat_lt(a980, a981) {
            let a982 = v0;
            let a983 = 2u64;
            if nat_lt(a982, a983) {
                let a984 = v0;
                let a985 = 1u64;
                if nat_lt(a984, a985) {
                    0u64
                } else {
                    2147483648u64
                }
            } else {
                let a986 = v0;
                let a987 = 3u64;
                if nat_lt(a986, a987) {
                    1065353216u64
                } else {
                    3212836864u64
                }
            }
        } else {
            let a988 = v0;
            let a989 = 6u64;
            if nat_lt(a988, a989) {
                let a990 = v0;
                let a991 = 5u64;
                if nat_lt(a990, a991) {
                    2139095040u64
                } else {
                    4286578688u64
                }
            } else {
                let a992 = v0;
                let a993 = 7u64;
                if nat_lt(a992, a993) {
                    2143289345u64
                } else {
                    1u64
                }
            }
        }
    } else {
        let a994 = v0;
        let a995 = 12u64;
        if nat_lt(a994, a995) {
            let a996 = v0;
            let a997 = 10u64;
            if nat_lt(a996, a997) {
                let a998 = v0;
                let a999 = 9u64;
                if nat_lt(a998, a999) {
                    2139095039u64
                } else {
                    8388608u64
                }
            } else {
                let a1000 = v0;
                let a1001 = 11u64;
                if nat_lt(a1000, a1001) {
                    8388607u64
                } else {
                    1065353217u64
                }
            }
        } else {
            let a1002 = v0;
            let a1003 = 14u64;
            if nat_lt(a1002, a1003) {
                let a1004 = v0;
                let a1005 = 13u64;
                if nat_lt(a1004, a1005) {
                    1056964608u64
                } else {
                    8388609u64
                }
            } else {
                let a1006 = v0;
                let a1007 = 15u64;
                if nat_lt(a1006, a1007) {
                    2147483649u64
                } else {
                    2139095041u64
                }
            }
        }
    }
}

pub fn f52(v0: u64, v1: u64) -> R<u64> {
    let a1016 = {
        let a1014 = {
            let a1010 = {
                let a1008 = v0;
                let a1009 = 0u64;
                if nat_eq(a1008, a1009) {
                    1u64
                } else {
                    5u64
                }
            };
            let a1011 = v1;
            nat_mul(a1010, a1011)?
        };
        let a1015 = {
            let a1012 = v0;
            let a1013 = 0u64;
            if nat_eq(a1012, a1013) {
                0u64
            } else {
                3u64
            }
        };
        nat_add(a1014, a1015)?
    };
    let a1017 = 16u64;
    let a1018 = 0u64;
    Ok(nat_rem(a1016, a1017, a1018))
}

pub fn f53(v0: u64) -> u64 {
    let a1019 = v0;
    let a1020 = 4294967296u64;
    let a1021 = 0u64;
    nat_rem(a1019, a1020, a1021)
}

pub fn f54(v0: u64) -> u64 {
    let a1022 = v0;
    let a1023 = 0u64;
    if nat_eq(a1022, a1023) {
        2654435769u64
    } else {
        2246822507u64
    }
}

pub fn f55(v0: u64) -> R<u64> {
    let a1024 = 2654435761u64;
    let a1025 = v0;
    nat_mul(a1024, a1025)
}

pub fn f56(v0: u64) -> u64 {
    let a1026 = f58(v0);
    let a1027 = 7u64;
    if nat_eq(a1026, a1027) {
        1u64
    } else {
        let a1038 = {
            let a1028 = v0;
            let a1029 = 240u64;
            if nat_lt(a1028, a1029) {
                let a1030 = v0;
                let a1031 = 120u64;
                let a1032 = 0u64;
                nat_quot(a1030, a1031, a1032)
            } else {
                let a1035 = {
                    let a1033 = v0;
                    let a1034 = 240u64;
                    nat_sub(a1033, a1034)
                };
                let a1036 = 8u64;
                let a1037 = 0u64;
                nat_quot(a1035, a1036, a1037)
            }
        };
        let a1039 = 0u64;
        if nat_eq(a1038, a1039) {
            0u64
        } else {
            30u64
        }
    }
}

pub fn f57(v0: u64) -> u64 {
    let a1040 = v0;
    let a1041 = 240u64;
    if nat_lt(a1040, a1041) {
        let a1045 = {
            let a1042 = v0;
            let a1043 = 60u64;
            let a1044 = 0u64;
            nat_quot(a1042, a1043, a1044)
        };
        let a1046 = 2u64;
        let a1047 = 0u64;
        nat_rem(a1045, a1046, a1047)
    } else {
        let a1053 = {
            let a1050 = {
                let a1048 = v0;
                let a1049 = 240u64;
                nat_sub(a1048, a1049)
            };
            let a1051 = 4u64;
            let a1052 = 0u64;
            nat_quot(a1050, a1051, a1052)
        };
        let a1054 = 2u64;
        let a1055 = 0u64;
        nat_rem(a1053, a1054, a1055)
    }
}

pub fn f58(v0: u64) -> u64 {
    let a1056 = v0;
    let a1057 = 240u64;
    if nat_lt(a1056, a1057) {
        let a1058 = v0;
        let a1059 = 12u64;
        let a1060 = 0u64;
        nat_rem(a1058, a1059, a1060)
    } else {
        0u64
    }
}

pub fn f59(v0: u64) -> u64 {
    let a1061 = v0;
    let a1062 = 240u64;
    if nat_lt(a1061, a1062) {
        let a1066 = {
            let a1063 = v0;
            let a1064 = 12u64;
            let a1065 = 0u64;
            nat_quot(a1063, a1064, a1065)
        };
        let a1067 = 5u64;
        let a1068 = 0u64;
        nat_rem(a1066, a1067, a1068)
    } else {
        0u64
    }
}

pub fn f60(v0: u64) -> u64 {
    let v1: u64 = f58(v0);
    let a1069 = v1;
    let a1070 = 3u64;
    if if if nat_eq(a1069, a1070) {
        true
    } else {
        let a1071 = v1;
        let a1072 = 7u64;
        nat_eq(a1071, a1072)
    } {
        true
    } else {
        let a1073 = v1;
        let a1074 = 8u64;
        nat_eq(a1073, a1074)
    } {
        2u64
    } else {
        let a1075 = v1;
        let a1076 = 2u64;
        if if if if if if nat_eq(a1075, a1076) {
            true
        } else {
            let a1077 = v1;
            let a1078 = 4u64;
            nat_eq(a1077, a1078)
        } {
            true
        } else {
            let a1079 = v1;
            let a1080 = 5u64;
            nat_eq(a1079, a1080)
        } {
            true
        } else {
            let a1081 = v1;
            let a1082 = 6u64;
            nat_eq(a1081, a1082)
        } {
            true
        } else {
            let a1083 = v1;
            let a1084 = 9u64;
            nat_eq(a1083, a1084)
        } {
            true
        } else {
            let a1085 = f59(v0);
            let a1086 = 4u64;
            if nat_eq(a1085, a1086) {
                let a1087 = f57(v0);
                let a1088 = 1u64;
                nat_eq(a1087, a1088)
            } else {
                false
            }
        } {
            1u64
        } else {
            0u64
        }
    }
}

pub fn f61(v0: u64) -> u64 {
    let a1089 = f58(v0);
    let a1090 = 2u64;
    if nat_eq(a1089, a1090) {
        2139095040u64
    } else {
        let a1091 = f58(v0);
        let a1092 = 3u64;
        if nat_eq(a1091, a1092) {
            3212836864u64
        } else {
            let a1093 = f58(v0);
            let a1094 = 10u64;
            if nat_eq(a1093, a1094) {
                2147483648u64
            } else {
                let a1095 = f58(v0);
                let a1096 = 1u64;
                if if nat_eq(a1095, a1096) {
                    true
                } else {
                    let a1097 = f58(v0);
                    let a1098 = 11u64;
                    nat_eq(a1097, a1098)
                } {
                    0u64
                } else {
                    897988541u64
                }
            }
        }
    }
}

pub fn f62(v0: u64) -> u64 {
    let a1099 = f56(v0);
    let a1100 = 0u64;
    if nat_eq(a1099, a1100) {
        4u64
    } else {
        2u64
    }
}

pub fn f63(v0: u64, v1: u64) -> u64 {
    let a1101 = v1;
    let a1102 = 1u64;
    if if nat_eq(a1101, a1102) {
        true
    } else {
        let a1103 = v1;
        let a1104 = 3u64;
        if if nat_eq(a1103, a1104) {
            let a1105 = f57(v0);
            let a1106 = 0u64;
            nat_eq(a1105, a1106)
        } else {
            false
        } {
            let a1107 = f59(v0);
            let a1108 = 4u64;
            nat_eq(a1107, a1108)
        } else {
            false
        }
    } {
        0u64
    } else {
        f59(v0)
    }
}

pub fn f64(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f58(v0);
    let a1109 = f56(v0);
    let a1110 = 0u64;
    if nat_eq(a1109, a1110) {
        let a1111 = v2;
        let a1112 = 4u64;
        if nat_eq(a1111, a1112) {
            Ok(2139095040u64)
        } else {
            let a1113 = v2;
            let a1114 = 5u64;
            if if nat_eq(a1113, a1114) {
                true
            } else {
                let a1115 = v2;
                let a1116 = 6u64;
                if nat_eq(a1115, a1116) {
                    let a1122 = v1;
                    let a1123 = {
                        let a1120 = {
                            let a1117 = f13(v0, 0u64)?;
                            let a1118 = f62(v0);
                            let a1119 = 0u64;
                            nat_quot(a1117, a1118, a1119)
                        };
                        let a1121 = 1u64;
                        nat_sub(a1120, a1121)
                    };
                    nat_eq(a1122, a1123)
                } else {
                    false
                }
            } {
                Ok(2143289635u64)
            } else {
                let a1124 = v2;
                let a1125 = 11u64;
                if nat_eq(a1124, a1125) {
                    Ok(1u64)
                } else {
                    let a1126 = v2;
                    let a1127 = 10u64;
                    if nat_eq(a1126, a1127) {
                        Ok(2147483648u64)
                    } else {
                        let a1128 = v2;
                        let a1129 = 1u64;
                        if nat_eq(a1128, a1129) {
                            Ok(0u64)
                        } else {
                            let a1133 = {
                                let a1130 = v1;
                                let a1131 = 5u64;
                                let a1132 = 0u64;
                                nat_rem(a1130, a1131, a1132)
                            };
                            let a1134 = 0u64;
                            if nat_eq(a1133, a1134) {
                                Ok(1166016512u64)
                            } else {
                                let a1138 = {
                                    let a1135 = v1;
                                    let a1136 = 5u64;
                                    let a1137 = 0u64;
                                    nat_rem(a1135, a1136, a1137)
                                };
                                let a1139 = 1u64;
                                if nat_eq(a1138, a1139) {
                                    Ok(1065353216u64)
                                } else {
                                    Ok(0u64)
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        let a1140 = v2;
        let a1141 = 4u64;
        if nat_eq(a1140, a1141) {
            Ok(32640u64)
        } else {
            let a1142 = v2;
            let a1143 = 5u64;
            if if nat_eq(a1142, a1143) {
                true
            } else {
                let a1144 = v2;
                let a1145 = 6u64;
                if nat_eq(a1144, a1145) {
                    let a1151 = v1;
                    let a1152 = {
                        let a1149 = {
                            let a1146 = f13(v0, 0u64)?;
                            let a1147 = f62(v0);
                            let a1148 = 0u64;
                            nat_quot(a1146, a1147, a1148)
                        };
                        let a1150 = 1u64;
                        nat_sub(a1149, a1150)
                    };
                    nat_eq(a1151, a1152)
                } else {
                    false
                }
            } {
                Ok(32705u64)
            } else {
                let a1153 = v2;
                let a1154 = 11u64;
                if nat_eq(a1153, a1154) {
                    Ok(1u64)
                } else {
                    let a1155 = v2;
                    let a1156 = 10u64;
                    if nat_eq(a1155, a1156) {
                        Ok(32768u64)
                    } else {
                        let a1157 = v2;
                        let a1158 = 1u64;
                        if nat_eq(a1157, a1158) {
                            Ok(0u64)
                        } else {
                            let a1162 = {
                                let a1159 = v1;
                                let a1160 = 5u64;
                                let a1161 = 0u64;
                                nat_rem(a1159, a1160, a1161)
                            };
                            let a1163 = 0u64;
                            if nat_eq(a1162, a1163) {
                                Ok(17792u64)
                            } else {
                                let a1167 = {
                                    let a1164 = v1;
                                    let a1165 = 5u64;
                                    let a1166 = 0u64;
                                    nat_rem(a1164, a1165, a1166)
                                };
                                let a1168 = 1u64;
                                if nat_eq(a1167, a1168) {
                                    Ok(16256u64)
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

pub fn f65(v0: u64, v1: u64) -> u64 {
    f123(f124(v0, v1))
}

pub fn f66(v0: u64, v1: u64) -> R<u64> {
    let a1169 = v0;
    let a1170 = 0u64;
    if nat_eq(a1169, a1170) {
        let a1174 = {
            let a1171 = v1;
            let a1172 = 3840u64;
            let a1173 = 0u64;
            nat_quot(a1171, a1172, a1173)
        };
        let a1175 = 2u64;
        let a1176 = 0u64;
        Ok(nat_rem(a1174, a1175, a1176))
    } else {
        let v2: u64 = {
            let a1180 = {
                let a1177 = v1;
                let a1178 = 160u64;
                let a1179 = 0u64;
                nat_quot(a1177, a1178, a1179)
            };
            let a1181 = 4u64;
            let a1182 = 0u64;
            nat_rem(a1180, a1181, a1182)
        };
        let v3: u64 = {
            let a1183 = v2;
            let a1184 = 2u64;
            let a1185 = 0u64;
            nat_rem(a1183, a1184, a1185)
        };
        let a1195 = {
            let a1189 = {
                let a1186 = v2;
                let a1187 = 2u64;
                let a1188 = 0u64;
                nat_quot(a1186, a1187, a1188)
            };
            let a1190 = 3u64;
            nat_mul(a1189, a1190)?
        };
        let a1196 = {
            let a1191 = v3;
            let a1192 = f124(v0, v1);
            if nat_lt(a1191, a1192) {
                v3
            } else {
                let a1193 = v3;
                let a1194 = 1u64;
                nat_add(a1193, a1194)?
            }
        };
        nat_add(a1195, a1196)
    }
}

pub fn f67(v0: u64, v1: u64) -> u64 {
    let a1202 = {
        let a1197 = v1;
        let a1198 = 32u64;
        let a1199 = 0u64;
        nat_quot(a1197, a1198, a1199)
    };
    let a1203 = {
        let a1200 = v0;
        let a1201 = 0u64;
        if nat_eq(a1200, a1201) {
            120u64
        } else {
            5u64
        }
    };
    let a1204 = 0u64;
    nat_rem(a1202, a1203, a1204)
}

pub fn f68(v0: u64) -> u64 {
    f110({
        let a1205 = v0;
        let a1206 = 32u64;
        let a1207 = 0u64;
        nat_rem(a1205, a1206, a1207)
    })
}

pub fn f69(v0: u64) -> R<u64> {
    f111({
        let a1208 = v0;
        let a1209 = 32u64;
        let a1210 = 0u64;
        nat_rem(a1208, a1209, a1210)
    })
}

pub fn f70(v0: u64) -> R<u64> {
    f112({
        let a1211 = v0;
        let a1212 = 32u64;
        let a1213 = 0u64;
        nat_rem(a1211, a1212, a1213)
    })
}

pub fn f71(v0: u64) -> R<u64> {
    f113({
        let a1214 = v0;
        let a1215 = 32u64;
        let a1216 = 0u64;
        nat_rem(a1214, a1215, a1216)
    })
}

pub fn f72(v0: u64, v1: u64) -> R<u64> {
    let a1217 = v0;
    let a1218 = 0u64;
    if nat_eq(a1217, a1218) {
        let a1221 = {
            let a1219 = f124(v0, v1);
            let a1220 = 4u64;
            nat_mul(a1219, a1220)?
        };
        let a1222 = f68(v1);
        nat_add(a1221, a1222)
    } else {
        Ok(f124(v0, v1))
    }
}

pub fn f73(v0: u64) -> u64 {
    let a1223 = v0;
    let a1224 = 12u64;
    if nat_lt(a1223, a1224) {
        let a1225 = v0;
        let a1226 = 6u64;
        if nat_lt(a1225, a1226) {
            let a1227 = v0;
            let a1228 = 3u64;
            if nat_lt(a1227, a1228) {
                let a1229 = v0;
                let a1230 = 1u64;
                if nat_lt(a1229, a1230) {
                    0u64
                } else {
                    let a1231 = v0;
                    let a1232 = 2u64;
                    if nat_lt(a1231, a1232) {
                        2147483648u64
                    } else {
                        1065353216u64
                    }
                }
            } else {
                let a1233 = v0;
                let a1234 = 4u64;
                if nat_lt(a1233, a1234) {
                    3212836864u64
                } else {
                    let a1235 = v0;
                    let a1236 = 5u64;
                    if nat_lt(a1235, a1236) {
                        2139095040u64
                    } else {
                        4286578688u64
                    }
                }
            }
        } else {
            let a1237 = v0;
            let a1238 = 9u64;
            if nat_lt(a1237, a1238) {
                let a1239 = v0;
                let a1240 = 7u64;
                if nat_lt(a1239, a1240) {
                    2143289345u64
                } else {
                    let a1241 = v0;
                    let a1242 = 8u64;
                    if nat_lt(a1241, a1242) {
                        2139095041u64
                    } else {
                        4286578689u64
                    }
                }
            } else {
                let a1243 = v0;
                let a1244 = 10u64;
                if nat_lt(a1243, a1244) {
                    1u64
                } else {
                    let a1245 = v0;
                    let a1246 = 11u64;
                    if nat_lt(a1245, a1246) {
                        2147483649u64
                    } else {
                        8388608u64
                    }
                }
            }
        }
    } else {
        let a1247 = v0;
        let a1248 = 18u64;
        if nat_lt(a1247, a1248) {
            let a1249 = v0;
            let a1250 = 15u64;
            if nat_lt(a1249, a1250) {
                let a1251 = v0;
                let a1252 = 13u64;
                if nat_lt(a1251, a1252) {
                    2139095039u64
                } else {
                    let a1253 = v0;
                    let a1254 = 14u64;
                    if nat_lt(a1253, a1254) {
                        4286578687u64
                    } else {
                        864026624u64
                    }
                }
            } else {
                let a1255 = v0;
                let a1256 = 16u64;
                if nat_lt(a1255, a1256) {
                    855638016u64
                } else {
                    let a1257 = v0;
                    let a1258 = 17u64;
                    if nat_lt(a1257, a1258) {
                        855638017u64
                    } else {
                        855638015u64
                    }
                }
            }
        } else {
            let a1259 = v0;
            let a1260 = 21u64;
            if nat_lt(a1259, a1260) {
                let a1261 = v0;
                let a1262 = 19u64;
                if nat_lt(a1261, a1262) {
                    1065357312u64
                } else {
                    let a1263 = v0;
                    let a1264 = 20u64;
                    if nat_lt(a1263, a1264) {
                        1065365504u64
                    } else {
                        1199562752u64
                    }
                }
            } else {
                let a1265 = v0;
                let a1266 = 22u64;
                if nat_lt(a1265, a1266) {
                    1199566848u64
                } else {
                    let a1267 = v0;
                    let a1268 = 23u64;
                    if nat_lt(a1267, a1268) {
                        1199566847u64
                    } else {
                        947912704u64
                    }
                }
            }
        }
    }
}

pub fn f74(v0: u64, v1: u64) -> R<u64> {
    let a1269 = v0;
    let a1270 = 0u64;
    if nat_eq(a1269, a1270) {
        Ok(f65(v0, v1))
    } else {
        Ok(f123({
            let a1271 = f66(v0, v1)?;
            let a1272 = 3u64;
            let a1273 = 0u64;
            nat_rem(a1271, a1272, a1273)
        }))
    }
}

pub fn f75(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a1274 = v2;
    let a1275 = 1u64;
    if nat_eq(a1274, a1275) {
        Ok(true)
    } else {
        let a1276 = v0;
        let a1277 = 1u64;
        if if nat_eq(a1276, a1277) {
            let a1278 = 3u64;
            let a1279 = f66(v0, v1)?;
            nat_le(a1278, a1279)
        } else {
            false
        } {
            let a1280 = v2;
            let a1281 = 3u64;
            Ok(nat_eq(a1280, a1281))
        } else {
            Ok(false)
        }
    }
}

pub fn f76(v0: u64, v1: u64) -> R<u64> {
    let a1286 = {
        let a1282 = f77(v0, v1, 0u64)?;
        let a1283 = f77(v0, v1, 1u64)?;
        nat_mul(a1282, a1283)?
    };
    let a1287 = {
        let a1284 = f77(v0, v1, 2u64)?;
        let a1285 = f77(v0, v1, 3u64)?;
        nat_mul(a1284, a1285)?
    };
    nat_mul(a1286, a1287)
}

pub fn f77(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1293 = {
        let a1288 = f67(v0, v1);
        let a1289 = 5u64;
        let a1290 = 0u64;
        nat_rem(a1288, a1289, a1290)
    };
    let a1294 = {
        let a1291 = v2;
        let a1292 = 1u64;
        nat_add(a1291, a1292)?
    };
    if nat_eq(a1293, a1294) {
        Ok(0u64)
    } else {
        let a1295 = v2;
        let a1296 = 2u64;
        if nat_lt(a1295, a1296) {
            let a1297 = v2;
            let a1298 = 0u64;
            if nat_eq(a1297, a1298) {
                f69(v1)
            } else {
                f70(v1)
            }
        } else {
            let a1299 = v2;
            let a1300 = 2u64;
            if nat_eq(a1299, a1300) {
                Ok(2u64)
            } else {
                f71(v1)
            }
        }
    }
}

pub fn f78(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1305 = f83(v0, v1, v2)?;
    let a1306 = {
        let a1301 = v2;
        let a1302 = 0u64;
        if if if nat_eq(a1301, a1302) {
            true
        } else {
            f125(v0, v1, v2)?
        } {
            let a1303 = f68(v1);
            let a1304 = 2u64;
            nat_eq(a1303, a1304)
        } else {
            false
        } {
            2u64
        } else {
            1u64
        }
    };
    nat_mul(a1305, a1306)
}

pub fn f79(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1309 = {
        let a1307 = f78(v0, v1, v2)?;
        let a1308 = f19(v0, v1, v2, 0u64)?;
        nat_mul(a1307, a1308)?
    };
    let a1310 = f126(v0, v1, v2)?;
    nat_add(a1309, a1310)
}

pub fn f80(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1313 = {
        let a1311 = f79(v0, v1, v2)?;
        let a1312 = f19(v0, v1, v2, 1u64)?;
        nat_mul(a1311, a1312)?
    };
    let a1314 = f126(v0, v1, v2)?;
    nat_add(a1313, a1314)
}

pub fn f81(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1317 = {
        let a1315 = f80(v0, v1, v2)?;
        let a1316 = f19(v0, v1, v2, 2u64)?;
        nat_mul(a1315, a1316)?
    };
    let a1318 = f126(v0, v1, v2)?;
    nat_add(a1317, a1318)
}

pub fn f82(v0: Adt0) -> R<Adt1> {
    let a1320 = match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h1319, _) => {
            h1319
        }
    };
    let a1321 = 0u64;
    if nat_eq(a1320, a1321) {
        Ok(Adt1::C0)
    } else {
        let a1323 = match v0.clone() {
            Adt0::C0(h1322, _, _, _, _, _, _, _, _, _) => {
                h1322
            }
        };
        let a1324 = 0u64;
        if if if if nat_eq(a1323, a1324) {
            true
        } else {
            let a1326 = match v0.clone() {
                Adt0::C0(_, h1325, _, _, _, _, _, _, _, _) => {
                    h1325
                }
            };
            let a1327 = 0u64;
            nat_eq(a1326, a1327)
        } {
            true
        } else {
            let a1329 = match v0.clone() {
                Adt0::C0(_, _, h1328, _, _, _, _, _, _, _) => {
                    h1328
                }
            };
            let a1330 = 0u64;
            nat_eq(a1329, a1330)
        } {
            true
        } else {
            let a1332 = match v0.clone() {
                Adt0::C0(_, _, _, h1331, _, _, _, _, _, _) => {
                    h1331
                }
            };
            let a1333 = 0u64;
            nat_eq(a1332, a1333)
        } {
            Ok(Adt1::C1(0u64))
        } else {
            let a1335 = match v0.clone() {
                Adt0::C0(_, _, _, _, _, _, _, _, h1334, _) => {
                    h1334
                }
            };
            let a1336 = 1u64;
            if nat_eq(a1335, a1336) {
                Ok(Adt1::C1({
                    let a1346 = {
                        let a1344 = match v0.clone() {
                            Adt0::C0(_, _, _, _, _, _, _, _, _, h1337) => {
                                h1337
                            }
                        };
                        let a1345 = {
                            let a1342 = {
                                let a1339 = match v0.clone() {
                                    Adt0::C0(h1338, _, _, _, _, _, _, _, _, _) => {
                                        h1338
                                    }
                                };
                                let a1340 = 1u64;
                                nat_sub(a1339, a1340)
                            };
                            let a1343 = match v0.clone() {
                                Adt0::C0(_, _, _, _, h1341, _, _, _, _, _) => {
                                    h1341
                                }
                            };
                            nat_mul(a1342, a1343)?
                        };
                        nat_add(a1344, a1345)?
                    };
                    let a1347 = f127(v0)?;
                    nat_add(a1346, a1347)?
                }))
            } else {
                Ok(Adt1::C1({
                    let a1356 = {
                        let a1353 = {
                            let a1350 = match v0.clone() {
                                Adt0::C0(h1348, _, _, _, _, _, _, _, _, _) => {
                                    h1348
                                }
                            };
                            let a1351 = match v0.clone() {
                                Adt0::C0(_, _, _, _, h1349, _, _, _, _, _) => {
                                    h1349
                                }
                            };
                            nat_mul(a1350, a1351)?
                        };
                        let a1354 = match v0.clone() {
                            Adt0::C0(_, _, _, _, _, _, _, _, h1352, _) => {
                                h1352
                            }
                        };
                        let a1355 = 0u64;
                        nat_quot(a1353, a1354, a1355)
                    };
                    let a1357 = f127(v0)?;
                    nat_add(a1356, a1357)?
                }))
            }
        }
    }
}

pub fn f83(v0: u64, v1: u64, v2: u64) -> R<u64> {
    Ok(f128({
        let a1358 = v2;
        let a1359 = 0u64;
        if nat_eq(a1358, a1359) {
            f65(v0, v1)
        } else {
            f74(v0, v1)?
        }
    }))
}

pub fn f84(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1360 = v0;
    let a1361 = 0u64;
    if nat_eq(a1360, a1361) {
        f121(v1, v2)
    } else {
        let v3: u64 = f128(f123(v1));
        let v4: u64 = {
            let a1362 = v1;
            let a1363 = 0u64;
            if nat_eq(a1362, a1363) {
                f73({
                    let a1367 = {
                        let a1364 = v2;
                        let a1365 = v3;
                        let a1366 = 0u64;
                        nat_quot(a1364, a1365, a1366)
                    };
                    let a1368 = 24u64;
                    let a1369 = 0u64;
                    nat_rem(a1367, a1368, a1369)
                })
            } else {
                f129({
                    let a1373 = {
                        let a1370 = v2;
                        let a1371 = v3;
                        let a1372 = 0u64;
                        nat_quot(a1370, a1371, a1372)
                    };
                    let a1374 = 24u64;
                    let a1375 = 0u64;
                    nat_rem(a1373, a1374, a1375)
                })
            }
        };
        let a1394 = {
            let a1391 = v4;
            let a1392 = {
                let a1379 = {
                    let a1376 = v2;
                    let a1377 = v3;
                    let a1378 = 0u64;
                    nat_rem(a1376, a1377, a1378)
                };
                let a1380 = 2u64;
                if nat_lt(a1379, a1380) {
                    let a1384 = {
                        let a1381 = v2;
                        let a1382 = v3;
                        let a1383 = 0u64;
                        nat_rem(a1381, a1382, a1383)
                    };
                    let a1385 = 0u64;
                    if nat_eq(a1384, a1385) {
                        1u64
                    } else {
                        256u64
                    }
                } else {
                    let a1389 = {
                        let a1386 = v2;
                        let a1387 = v3;
                        let a1388 = 0u64;
                        nat_rem(a1386, a1387, a1388)
                    };
                    let a1390 = 2u64;
                    if nat_eq(a1389, a1390) {
                        65536u64
                    } else {
                        16777216u64
                    }
                }
            };
            let a1393 = 0u64;
            nat_quot(a1391, a1392, a1393)
        };
        let a1395 = 256u64;
        let a1396 = 0u64;
        Ok(nat_rem(a1394, a1395, a1396))
    }
}

pub fn f85(v0: u64) -> R<u64> {
    f130(f131(v0))
}

pub fn f86(v0: u64) -> u64 {
    let a1400 = {
        let a1397 = v0;
        let a1398 = 72u64;
        let a1399 = 0u64;
        nat_quot(a1397, a1398, a1399)
    };
    let a1401 = 4u64;
    let a1402 = 0u64;
    nat_rem(a1400, a1401, a1402)
}

pub fn f87(v0: u64) -> u64 {
    let a1406 = {
        let a1403 = v0;
        let a1404 = 18u64;
        let a1405 = 0u64;
        nat_quot(a1403, a1404, a1405)
    };
    let a1407 = 4u64;
    let a1408 = 0u64;
    nat_rem(a1406, a1407, a1408)
}

pub fn f88(v0: u64) -> u64 {
    f132({
        let a1412 = {
            let a1409 = v0;
            let a1410 = 6u64;
            let a1411 = 0u64;
            nat_quot(a1409, a1410, a1411)
        };
        let a1413 = 3u64;
        let a1414 = 0u64;
        nat_rem(a1412, a1413, a1414)
    })
}

pub fn f89(v0: u64) -> u64 {
    f132({
        let a1418 = {
            let a1415 = v0;
            let a1416 = 2u64;
            let a1417 = 0u64;
            nat_quot(a1415, a1416, a1417)
        };
        let a1419 = 3u64;
        let a1420 = 0u64;
        nat_rem(a1418, a1419, a1420)
    })
}

pub fn f90(v0: u64) -> R<u64> {
    let a1424 = 1u64;
    let a1425 = {
        let a1421 = v0;
        let a1422 = 2u64;
        let a1423 = 0u64;
        nat_rem(a1421, a1422, a1423)
    };
    nat_add(a1424, a1425)
}

pub fn f91(v0: u64, v1: u64) -> R<u64> {
    let a1434 = {
        let a1430 = {
            let a1428 = {
                let a1426 = f131(v0);
                let a1427 = 4u64;
                nat_mul(a1426, a1427)?
            };
            let a1429 = f87(v0);
            nat_add(a1428, a1429)?
        };
        let a1431 = 2u64;
        nat_mul(a1430, a1431)?
    };
    let a1435 = {
        let a1432 = v1;
        let a1433 = 1u64;
        if nat_eq(a1432, a1433) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a1434, a1435)
}

pub fn f92(v0: u64, v1: u64) -> R<u64> {
    let a1436 = v1;
    let a1437 = 2u64;
    if nat_lt(a1436, a1437) {
        let a1438 = v1;
        let a1439 = 0u64;
        if nat_eq(a1438, a1439) {
            let a1440 = f93(v0)?;
            let a1441 = f88(v0);
            nat_mul(a1440, a1441)
        } else {
            Ok(f89(v0))
        }
    } else {
        let a1442 = v1;
        let a1443 = 2u64;
        if nat_eq(a1442, a1443) {
            Ok(2u64)
        } else {
            f90(v0)
        }
    }
}

pub fn f93(v0: u64) -> R<u64> {
    f115(f133(v0)?)
}

pub fn f94(v0: u64) -> R<u64> {
    let a1448 = {
        let a1444 = f92(v0, 0u64)?;
        let a1445 = f92(v0, 1u64)?;
        nat_mul(a1444, a1445)?
    };
    let a1449 = {
        let a1446 = f92(v0, 2u64)?;
        let a1447 = f92(v0, 3u64)?;
        nat_mul(a1446, a1447)?
    };
    nat_mul(a1448, a1449)
}

pub fn f95(v0: u64, v1: u64) -> R<u64> {
    let a1450 = v1;
    let a1451 = f86(v0);
    if nat_eq(a1450, a1451) {
        let a1452 = f87(v0);
        let a1453 = 3u64;
        if nat_eq(a1452, a1453) {
            Ok(0u64)
        } else {
            let a1456 = f92(v0, v1)?;
            let a1457 = {
                let a1454 = v1;
                let a1455 = 0u64;
                if nat_eq(a1454, a1455) {
                    f93(v0)?
                } else {
                    1u64
                }
            };
            nat_add(a1456, a1457)
        }
    } else {
        f92(v0, v1)
    }
}

pub fn f96(v0: u64, v1: u64) -> R<u64> {
    let a1464 = f100(v0)?;
    let a1465 = {
        let a1458 = v1;
        let a1459 = 0u64;
        if if if nat_eq(a1458, a1459) {
            true
        } else {
            let a1460 = v1;
            let a1461 = 2u64;
            nat_eq(a1460, a1461)
        } {
            let a1462 = f87(v0);
            let a1463 = 2u64;
            nat_eq(a1462, a1463)
        } else {
            false
        } {
            2u64
        } else {
            1u64
        }
    };
    nat_mul(a1464, a1465)
}

pub fn f97(v0: u64, v1: u64) -> R<u64> {
    let a1471 = {
        let a1469 = f96(v0, v1)?;
        let a1470 = {
            let a1466 = f26(v0, v1, 0u64)?;
            let a1467 = f93(v0)?;
            let a1468 = 0u64;
            nat_quot(a1466, a1467, a1468)
        };
        nat_mul(a1469, a1470)?
    };
    let a1472 = f134(v0, v1)?;
    nat_add(a1471, a1472)
}

pub fn f98(v0: u64, v1: u64) -> R<u64> {
    let a1475 = {
        let a1473 = f97(v0, v1)?;
        let a1474 = f26(v0, v1, 1u64)?;
        nat_mul(a1473, a1474)?
    };
    let a1476 = f134(v0, v1)?;
    nat_add(a1475, a1476)
}

pub fn f99(v0: u64, v1: u64) -> R<u64> {
    let a1479 = {
        let a1477 = f98(v0, v1)?;
        let a1478 = f26(v0, v1, 2u64)?;
        nat_mul(a1477, a1478)?
    };
    let a1480 = f134(v0, v1)?;
    nat_add(a1479, a1480)
}

pub fn f100(v0: u64) -> R<u64> {
    f120(f133(v0)?)
}

pub fn f101(v0: u64, v1: u64) -> R<u64> {
    let a1491 = {
        let a1489 = f121({
            let a1481 = v0;
            let a1482 = 2u64;
            let a1483 = 0u64;
            nat_quot(a1481, a1482, a1483)
        }, v1)?;
        let a1490 = {
            let a1487 = {
                let a1484 = v0;
                let a1485 = 2u64;
                let a1486 = 0u64;
                nat_rem(a1484, a1485, a1486)
            };
            let a1488 = 71u64;
            nat_mul(a1487, a1488)?
        };
        nat_add(a1489, a1490)?
    };
    let a1492 = 256u64;
    let a1493 = 0u64;
    Ok(nat_rem(a1491, a1492, a1493))
}

pub fn f102(v0: u64) -> R<u64> {
    Ok(f110(f36(v0)?))
}

pub fn f103(v0: u64) -> R<u64> {
    f120(f36(v0)?)
}

pub fn f104(v0: u64, v1: u64) -> R<u64> {
    let a1496 = {
        let a1494 = f103(v0)?;
        let a1495 = f32(v0, v1, 0u64)?;
        nat_mul(a1494, a1495)?
    };
    let a1497 = f135(v0, v1)?;
    nat_add(a1496, a1497)
}

pub fn f105(v0: u64, v1: u64) -> R<u64> {
    let a1500 = {
        let a1498 = f104(v0, v1)?;
        let a1499 = f32(v0, v1, 1u64)?;
        nat_mul(a1498, a1499)?
    };
    let a1501 = f135(v0, v1)?;
    nat_add(a1500, a1501)
}

pub fn f106(v0: u64, v1: u64) -> R<u64> {
    let a1504 = {
        let a1502 = f105(v0, v1)?;
        let a1503 = f32(v0, v1, 2u64)?;
        nat_mul(a1502, a1503)?
    };
    let a1505 = f135(v0, v1)?;
    nat_add(a1504, a1505)
}

pub fn f107(v0: u64) -> Adt2 {
    let a1506 = v0;
    let a1507 = 0u64;
    if nat_eq(a1506, a1507) {
        Adt2::C1(Adt3::C0(0u64, 1u64, 4u64))
    } else {
        let a1508 = v0;
        let a1509 = 1u64;
        if nat_eq(a1508, a1509) {
            Adt2::C1(Adt3::C0(1u64, 1u64, 2u64))
        } else {
            let a1510 = v0;
            let a1511 = 2u64;
            if nat_eq(a1510, a1511) {
                Adt2::C1(Adt3::C0(2u64, 32u64, 18u64))
            } else {
                let a1512 = v0;
                let a1513 = 3u64;
                if nat_eq(a1512, a1513) {
                    Adt2::C1(Adt3::C0(3u64, 32u64, 20u64))
                } else {
                    let a1514 = v0;
                    let a1515 = 6u64;
                    if nat_eq(a1514, a1515) {
                        Adt2::C1(Adt3::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1516 = v0;
                        let a1517 = 7u64;
                        if nat_eq(a1516, a1517) {
                            Adt2::C1(Adt3::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1518 = v0;
                            let a1519 = 8u64;
                            if nat_eq(a1518, a1519) {
                                Adt2::C1(Adt3::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1520 = v0;
                                let a1521 = 9u64;
                                if nat_eq(a1520, a1521) {
                                    Adt2::C1(Adt3::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1522 = v0;
                                    let a1523 = 10u64;
                                    if nat_eq(a1522, a1523) {
                                        Adt2::C1(Adt3::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1524 = v0;
                                        let a1525 = 11u64;
                                        if nat_eq(a1524, a1525) {
                                            Adt2::C1(Adt3::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1526 = v0;
                                            let a1527 = 12u64;
                                            if nat_eq(a1526, a1527) {
                                                Adt2::C1(Adt3::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1528 = v0;
                                                let a1529 = 13u64;
                                                if nat_eq(a1528, a1529) {
                                                    Adt2::C1(Adt3::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1530 = v0;
                                                    let a1531 = 14u64;
                                                    if nat_eq(a1530, a1531) {
                                                        Adt2::C1(Adt3::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1532 = v0;
                                                        let a1533 = 15u64;
                                                        if nat_eq(a1532, a1533) {
                                                            Adt2::C1(Adt3::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1534 = v0;
                                                            let a1535 = 16u64;
                                                            if nat_eq(a1534, a1535) {
                                                                Adt2::C1(Adt3::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1536 = v0;
                                                                let a1537 = 17u64;
                                                                if nat_eq(a1536, a1537) {
                                                                    Adt2::C1(Adt3::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1538 = v0;
                                                                    let a1539 = 18u64;
                                                                    if nat_eq(a1538, a1539) {
                                                                        Adt2::C1(Adt3::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1540 = v0;
                                                                        let a1541 = 19u64;
                                                                        if nat_eq(a1540, a1541) {
                                                                            Adt2::C1(Adt3::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1542 = v0;
                                                                            let a1543 = 20u64;
                                                                            if nat_eq(a1542, a1543) {
                                                                                Adt2::C1(Adt3::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1544 = v0;
                                                                                let a1545 = 21u64;
                                                                                if nat_eq(a1544, a1545) {
                                                                                    Adt2::C1(Adt3::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1546 = v0;
                                                                                    let a1547 = 22u64;
                                                                                    if nat_eq(a1546, a1547) {
                                                                                        Adt2::C1(Adt3::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1548 = v0;
                                                                                        let a1549 = 23u64;
                                                                                        if nat_eq(a1548, a1549) {
                                                                                            Adt2::C1(Adt3::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1550 = v0;
                                                                                            let a1551 = 24u64;
                                                                                            if nat_eq(a1550, a1551) {
                                                                                                Adt2::C1(Adt3::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1552 = v0;
                                                                                                let a1553 = 25u64;
                                                                                                if nat_eq(a1552, a1553) {
                                                                                                    Adt2::C1(Adt3::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1554 = v0;
                                                                                                    let a1555 = 26u64;
                                                                                                    if nat_eq(a1554, a1555) {
                                                                                                        Adt2::C1(Adt3::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1556 = v0;
                                                                                                        let a1557 = 27u64;
                                                                                                        if nat_eq(a1556, a1557) {
                                                                                                            Adt2::C1(Adt3::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1558 = v0;
                                                                                                            let a1559 = 28u64;
                                                                                                            if nat_eq(a1558, a1559) {
                                                                                                                Adt2::C1(Adt3::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1560 = v0;
                                                                                                                let a1561 = 29u64;
                                                                                                                if nat_eq(a1560, a1561) {
                                                                                                                    Adt2::C1(Adt3::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1562 = v0;
                                                                                                                    let a1563 = 30u64;
                                                                                                                    if nat_eq(a1562, a1563) {
                                                                                                                        Adt2::C1(Adt3::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1564 = v0;
                                                                                                                        let a1565 = 34u64;
                                                                                                                        if nat_eq(a1564, a1565) {
                                                                                                                            Adt2::C1(Adt3::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1566 = v0;
                                                                                                                            let a1567 = 35u64;
                                                                                                                            if nat_eq(a1566, a1567) {
                                                                                                                                Adt2::C1(Adt3::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1568 = v0;
                                                                                                                                let a1569 = 39u64;
                                                                                                                                if nat_eq(a1568, a1569) {
                                                                                                                                    Adt2::C1(Adt3::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1570 = v0;
                                                                                                                                    let a1571 = 40u64;
                                                                                                                                    if nat_eq(a1570, a1571) {
                                                                                                                                        Adt2::C1(Adt3::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1572 = v0;
                                                                                                                                        let a1573 = 41u64;
                                                                                                                                        if nat_eq(a1572, a1573) {
                                                                                                                                            Adt2::C1(Adt3::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1574 = v0;
                                                                                                                                            let a1575 = 42u64;
                                                                                                                                            if nat_eq(a1574, a1575) {
                                                                                                                                                Adt2::C1(Adt3::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1576 = v0;
                                                                                                                                                let a1577 = 43u64;
                                                                                                                                                if nat_eq(a1576, a1577) {
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

pub fn f108(v0: u64) -> R<u64> {
    f130({
        let a1578 = v0;
        let a1579 = 96u64;
        let a1580 = 0u64;
        nat_quot(a1578, a1579, a1580)
    })
}

pub fn f109(v0: u64) -> u64 {
    let a1584 = {
        let a1581 = v0;
        let a1582 = 32u64;
        let a1583 = 0u64;
        nat_quot(a1581, a1582, a1583)
    };
    let a1585 = 3u64;
    let a1586 = 0u64;
    nat_rem(a1584, a1585, a1586)
}

pub fn f110(v0: u64) -> u64 {
    let a1590 = {
        let a1587 = v0;
        let a1588 = 8u64;
        let a1589 = 0u64;
        nat_quot(a1587, a1588, a1589)
    };
    let a1591 = 4u64;
    let a1592 = 0u64;
    nat_rem(a1590, a1591, a1592)
}

pub fn f111(v0: u64) -> R<u64> {
    let a1601 = 1u64;
    let a1602 = {
        let a1599 = 2u64;
        let a1600 = {
            let a1596 = {
                let a1593 = v0;
                let a1594 = 4u64;
                let a1595 = 0u64;
                nat_quot(a1593, a1594, a1595)
            };
            let a1597 = 2u64;
            let a1598 = 0u64;
            nat_rem(a1596, a1597, a1598)
        };
        nat_mul(a1599, a1600)?
    };
    nat_add(a1601, a1602)
}

pub fn f112(v0: u64) -> R<u64> {
    let a1611 = 1u64;
    let a1612 = {
        let a1609 = 2u64;
        let a1610 = {
            let a1606 = {
                let a1603 = v0;
                let a1604 = 2u64;
                let a1605 = 0u64;
                nat_quot(a1603, a1604, a1605)
            };
            let a1607 = 2u64;
            let a1608 = 0u64;
            nat_rem(a1606, a1607, a1608)
        };
        nat_mul(a1609, a1610)?
    };
    nat_add(a1611, a1612)
}

pub fn f113(v0: u64) -> R<u64> {
    let a1616 = 1u64;
    let a1617 = {
        let a1613 = v0;
        let a1614 = 2u64;
        let a1615 = 0u64;
        nat_rem(a1613, a1614, a1615)
    };
    nat_add(a1616, a1617)
}

pub fn f114(v0: u64) -> R<u64> {
    let a1623 = {
        let a1621 = {
            let a1618 = v0;
            let a1619 = 96u64;
            let a1620 = 0u64;
            nat_quot(a1618, a1619, a1620)
        };
        let a1622 = 4u64;
        nat_mul(a1621, a1622)?
    };
    let a1624 = f110(v0);
    nat_add(a1623, a1624)
}

pub fn f115(v0: u64) -> R<u64> {
    let m1625 = f107(f108(v0)?);
    match m1625 {
        Adt2::C0 => {
            Ok(0u64)
        }
        Adt2::C1(v1) => {
            match v1 {
                Adt3::C0(_, h1626, _) => {
                    Ok(h1626)
                }
            }
        }
    }
}

pub fn f116(v0: u64, v1: u64) -> R<u64> {
    let a1631 = f120(v0)?;
    let a1632 = {
        let a1627 = v1;
        let a1628 = 0u64;
        if if nat_eq(a1627, a1628) {
            let a1629 = f110(v0);
            let a1630 = 2u64;
            nat_eq(a1629, a1630)
        } else {
            false
        } {
            2u64
        } else {
            1u64
        }
    };
    nat_mul(a1631, a1632)
}

pub fn f117(v0: u64, v1: u64) -> R<u64> {
    let a1638 = {
        let a1636 = f116(v0, v1)?;
        let a1637 = {
            let a1633 = f39(v0, v1, 0u64)?;
            let a1634 = f115(v0)?;
            let a1635 = 0u64;
            nat_quot(a1633, a1634, a1635)
        };
        nat_mul(a1636, a1637)?
    };
    let a1639 = f136(v0, v1)?;
    nat_add(a1638, a1639)
}

pub fn f118(v0: u64, v1: u64) -> R<u64> {
    let a1642 = {
        let a1640 = f117(v0, v1)?;
        let a1641 = f39(v0, v1, 1u64)?;
        nat_mul(a1640, a1641)?
    };
    let a1643 = f136(v0, v1)?;
    nat_add(a1642, a1643)
}

pub fn f119(v0: u64, v1: u64) -> R<u64> {
    let a1646 = {
        let a1644 = f118(v0, v1)?;
        let a1645 = f39(v0, v1, 2u64)?;
        nat_mul(a1644, a1645)?
    };
    let a1647 = f136(v0, v1)?;
    nat_add(a1646, a1647)
}

pub fn f120(v0: u64) -> R<u64> {
    let m1648 = f107(f108(v0)?);
    match m1648 {
        Adt2::C0 => {
            Ok(0u64)
        }
        Adt2::C1(v1) => {
            match v1 {
                Adt3::C0(_, _, h1649) => {
                    Ok(h1649)
                }
            }
        }
    }
}

pub fn f121(v0: u64, v1: u64) -> R<u64> {
    let a1664 = {
        let a1662 = {
            let a1657 = {
                let a1650 = v1;
                let a1651 = 37u64;
                nat_mul(a1650, a1651)?
            };
            let a1658 = {
                let a1655 = f130({
                    let a1652 = v0;
                    let a1653 = 4u64;
                    let a1654 = 0u64;
                    nat_quot(a1652, a1653, a1654)
                })?;
                let a1656 = 13u64;
                nat_mul(a1655, a1656)?
            };
            nat_add(a1657, a1658)?
        };
        let a1663 = {
            let a1659 = v0;
            let a1660 = 4u64;
            let a1661 = 0u64;
            nat_rem(a1659, a1660, a1661)
        };
        nat_add(a1662, a1663)?
    };
    let a1665 = 256u64;
    let a1666 = 0u64;
    Ok(nat_rem(a1664, a1665, a1666))
}

pub fn f122(v0: u64) -> u64 {
    let a1667 = v0;
    let a1668 = 96u64;
    if nat_lt(a1667, a1668) {
        let a1672 = {
            let a1669 = v0;
            let a1670 = 8u64;
            let a1671 = 0u64;
            nat_quot(a1669, a1670, a1671)
        };
        let a1673 = 3u64;
        let a1674 = 0u64;
        nat_rem(a1672, a1673, a1674)
    } else {
        let a1680 = {
            let a1677 = {
                let a1675 = v0;
                let a1676 = 96u64;
                nat_sub(a1675, a1676)
            };
            let a1678 = 2u64;
            let a1679 = 0u64;
            nat_quot(a1677, a1678, a1679)
        };
        let a1681 = 3u64;
        let a1682 = 0u64;
        nat_rem(a1680, a1681, a1682)
    }
}

pub fn f123(v0: u64) -> u64 {
    let a1683 = v0;
    let a1684 = 0u64;
    if nat_eq(a1683, a1684) {
        0u64
    } else {
        let a1685 = v0;
        let a1686 = 1u64;
        if nat_eq(a1685, a1686) {
            1u64
        } else {
            30u64
        }
    }
}

pub fn f124(v0: u64, v1: u64) -> u64 {
    let a1689 = v1;
    let a1690 = {
        let a1687 = v0;
        let a1688 = 0u64;
        if nat_eq(a1687, a1688) {
            7680u64
        } else {
            640u64
        }
    };
    let a1691 = 0u64;
    nat_quot(a1689, a1690, a1691)
}

pub fn f125(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a1692 = v0;
    let a1693 = 0u64;
    if if nat_eq(a1692, a1693) {
        let a1694 = f66(v0, v1)?;
        let a1695 = 1u64;
        nat_eq(a1694, a1695)
    } else {
        false
    } {
        let a1696 = v2;
        let a1697 = 3u64;
        Ok(nat_eq(a1696, a1697))
    } else {
        Ok(false)
    }
}

pub fn f126(v0: u64, v1: u64, v2: u64) -> R<u64> {
    if if f125(v0, v1, v2)? {
        true
    } else {
        let a1698 = v2;
        let a1699 = 0u64;
        nat_eq(a1698, a1699)
    } {
        let a1700 = f68(v1);
        let a1701 = 1u64;
        if if nat_eq(a1700, a1701) {
            true
        } else {
            let a1702 = f68(v1);
            let a1703 = 2u64;
            nat_eq(a1702, a1703)
        } {
            f83(v0, v1, v2)
        } else {
            Ok(0u64)
        }
    } else {
        f83(v0, v1, v2)
    }
}

pub fn f127(v0: Adt0) -> R<u64> {
    let a1724 = {
        let a1716 = {
            let a1708 = {
                let a1705 = match v0.clone() {
                    Adt0::C0(_, h1704, _, _, _, _, _, _, _, _) => {
                        h1704
                    }
                };
                let a1706 = 1u64;
                nat_sub(a1705, a1706)
            };
            let a1709 = match v0.clone() {
                Adt0::C0(_, _, _, _, _, h1707, _, _, _, _) => {
                    h1707
                }
            };
            nat_mul(a1708, a1709)?
        };
        let a1717 = {
            let a1714 = {
                let a1711 = match v0.clone() {
                    Adt0::C0(_, _, h1710, _, _, _, _, _, _, _) => {
                        h1710
                    }
                };
                let a1712 = 1u64;
                nat_sub(a1711, a1712)
            };
            let a1715 = match v0.clone() {
                Adt0::C0(_, _, _, _, _, _, h1713, _, _, _) => {
                    h1713
                }
            };
            nat_mul(a1714, a1715)?
        };
        nat_add(a1716, a1717)?
    };
    let a1725 = {
        let a1722 = {
            let a1719 = match v0.clone() {
                Adt0::C0(_, _, _, h1718, _, _, _, _, _, _) => {
                    h1718
                }
            };
            let a1720 = 1u64;
            nat_sub(a1719, a1720)
        };
        let a1723 = match v0 {
            Adt0::C0(_, _, _, _, _, _, _, h1721, _, _) => {
                h1721
            }
        };
        nat_mul(a1722, a1723)?
    };
    nat_add(a1724, a1725)
}

pub fn f128(v0: u64) -> u64 {
    let a1726 = v0;
    let a1727 = 0u64;
    if nat_eq(a1726, a1727) {
        4u64
    } else {
        2u64
    }
}

pub fn f129(v0: u64) -> u64 {
    let a1728 = v0;
    let a1729 = 12u64;
    if nat_lt(a1728, a1729) {
        let a1730 = v0;
        let a1731 = 6u64;
        if nat_lt(a1730, a1731) {
            let a1732 = v0;
            let a1733 = 3u64;
            if nat_lt(a1732, a1733) {
                let a1734 = v0;
                let a1735 = 1u64;
                if nat_lt(a1734, a1735) {
                    0u64
                } else {
                    let a1736 = v0;
                    let a1737 = 2u64;
                    if nat_lt(a1736, a1737) {
                        32768u64
                    } else {
                        15360u64
                    }
                }
            } else {
                let a1738 = v0;
                let a1739 = 4u64;
                if nat_lt(a1738, a1739) {
                    16256u64
                } else {
                    let a1740 = v0;
                    let a1741 = 5u64;
                    if nat_lt(a1740, a1741) {
                        48128u64
                    } else {
                        49024u64
                    }
                }
            }
        } else {
            let a1742 = v0;
            let a1743 = 9u64;
            if nat_lt(a1742, a1743) {
                let a1744 = v0;
                let a1745 = 7u64;
                if nat_lt(a1744, a1745) {
                    31744u64
                } else {
                    let a1746 = v0;
                    let a1747 = 8u64;
                    if nat_lt(a1746, a1747) {
                        32640u64
                    } else {
                        64512u64
                    }
                }
            } else {
                let a1748 = v0;
                let a1749 = 10u64;
                if nat_lt(a1748, a1749) {
                    65408u64
                } else {
                    let a1750 = v0;
                    let a1751 = 11u64;
                    if nat_lt(a1750, a1751) {
                        32256u64
                    } else {
                        32704u64
                    }
                }
            }
        }
    } else {
        let a1752 = v0;
        let a1753 = 18u64;
        if nat_lt(a1752, a1753) {
            let a1754 = v0;
            let a1755 = 15u64;
            if nat_lt(a1754, a1755) {
                let a1756 = v0;
                let a1757 = 13u64;
                if nat_lt(a1756, a1757) {
                    31745u64
                } else {
                    let a1758 = v0;
                    let a1759 = 14u64;
                    if nat_lt(a1758, a1759) {
                        32641u64
                    } else {
                        65025u64
                    }
                }
            } else {
                let a1760 = v0;
                let a1761 = 16u64;
                if nat_lt(a1760, a1761) {
                    65409u64
                } else {
                    let a1762 = v0;
                    let a1763 = 17u64;
                    if nat_lt(a1762, a1763) {
                        1u64
                    } else {
                        32769u64
                    }
                }
            }
        } else {
            let a1764 = v0;
            let a1765 = 21u64;
            if nat_lt(a1764, a1765) {
                let a1766 = v0;
                let a1767 = 19u64;
                if nat_lt(a1766, a1767) {
                    1023u64
                } else {
                    let a1768 = v0;
                    let a1769 = 20u64;
                    if nat_lt(a1768, a1769) {
                        127u64
                    } else {
                        1024u64
                    }
                }
            } else {
                let a1770 = v0;
                let a1771 = 22u64;
                if nat_lt(a1770, a1771) {
                    128u64
                } else {
                    let a1772 = v0;
                    let a1773 = 23u64;
                    if nat_lt(a1772, a1773) {
                        31743u64
                    } else {
                        32639u64
                    }
                }
            }
        }
    }
}

pub fn f130(v0: u64) -> R<u64> {
    let a1774 = v0;
    let a1775 = 4u64;
    if nat_lt(a1774, a1775) {
        Ok(v0)
    } else {
        let a1776 = v0;
        let a1777 = 29u64;
        if nat_lt(a1776, a1777) {
            let a1778 = v0;
            let a1779 = 2u64;
            nat_add(a1778, a1779)
        } else {
            let a1780 = v0;
            let a1781 = 31u64;
            if nat_lt(a1780, a1781) {
                let a1782 = v0;
                let a1783 = 5u64;
                nat_add(a1782, a1783)
            } else {
                let a1784 = v0;
                let a1785 = 8u64;
                nat_add(a1784, a1785)
            }
        }
    }
}

pub fn f131(v0: u64) -> u64 {
    let a1786 = v0;
    let a1787 = 288u64;
    let a1788 = 0u64;
    nat_quot(a1786, a1787, a1788)
}

pub fn f132(v0: u64) -> u64 {
    let a1789 = v0;
    let a1790 = 0u64;
    if nat_eq(a1789, a1790) {
        0u64
    } else {
        let a1791 = v0;
        let a1792 = 1u64;
        if nat_eq(a1791, a1792) {
            1u64
        } else {
            3u64
        }
    }
}

pub fn f133(v0: u64) -> R<u64> {
    let a1793 = f131(v0);
    let a1794 = 96u64;
    nat_mul(a1793, a1794)
}

pub fn f134(v0: u64, v1: u64) -> R<u64> {
    let a1795 = v1;
    let a1796 = 1u64;
    if if if nat_eq(a1795, a1796) {
        true
    } else {
        let a1797 = v1;
        let a1798 = 3u64;
        nat_eq(a1797, a1798)
    } {
        true
    } else {
        let a1799 = f87(v0);
        let a1800 = 1u64;
        if nat_eq(a1799, a1800) {
            true
        } else {
            let a1801 = f87(v0);
            let a1802 = 2u64;
            nat_eq(a1801, a1802)
        }
    } {
        f100(v0)
    } else {
        Ok(0u64)
    }
}

pub fn f135(v0: u64, v1: u64) -> R<u64> {
    let a1803 = v1;
    let a1804 = 3u64;
    if nat_eq(a1803, a1804) {
        f103(v0)
    } else {
        Ok(0u64)
    }
}

pub fn f136(v0: u64, v1: u64) -> R<u64> {
    let a1805 = v1;
    let a1806 = 1u64;
    if if nat_eq(a1805, a1806) {
        true
    } else {
        let a1807 = f110(v0);
        let a1808 = 1u64;
        if nat_eq(a1807, a1808) {
            true
        } else {
            let a1809 = f110(v0);
            let a1810 = 2u64;
            nat_eq(a1809, a1810)
        }
    } {
        f120(v0)
    } else {
        Ok(0u64)
    }
}
