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
    let a1 = 17u64;
    let a2 = v0;
    if nat_le(a1, a2) {
        f1({
            let a3 = v0;
            let a4 = 17u64;
            nat_sub(a3, a4)
        }, v1, v2, v3)
    } else {
        let a5 = 11u64;
        let a6 = v0;
        if nat_le(a5, a6) {
            f2({
                let a7 = v0;
                let a8 = 11u64;
                nat_sub(a7, a8)
            }, v1, v2, v3)
        } else {
            let a9 = 5u64;
            let a10 = v0;
            if nat_le(a9, a10) {
                f3({
                    let a11 = v0;
                    let a12 = 5u64;
                    nat_sub(a11, a12)
                }, v1, v2, v3)
            } else {
                let a13 = v0;
                let a14 = 2u64;
                if nat_lt(a13, a14) {
                    let a15 = v0;
                    let a16 = 0u64;
                    if nat_eq(a15, a16) {
                        let a17 = v3;
                        let a18 = 4u64;
                        if nat_lt(a17, a18) {
                            Ok(Some({
                                let a19 = v3;
                                let a20 = 2u64;
                                if nat_lt(a19, a20) {
                                    let a21 = v3;
                                    let a22 = 1u64;
                                    if nat_lt(a21, a22) {
                                        120u64
                                    } else {
                                        24u64
                                    }
                                } else {
                                    let a23 = v3;
                                    let a24 = 3u64;
                                    if nat_lt(a23, a24) {
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
                        let a25 = v1;
                        let a26 = 120u64;
                        if if nat_lt(a25, a26) {
                            let a27 = v3;
                            let a28 = 24u64;
                            nat_lt(a27, a28)
                        } else {
                            false
                        } {
                            Ok(Some(f4(v1, v3)?))
                        } else {
                            Ok(None::<u64>)
                        }
                    }
                } else {
                    let a29 = v0;
                    let a30 = 4u64;
                    if if nat_lt(a29, a30) {
                        let a31 = v1;
                        let a32 = 120u64;
                        if nat_lt(a31, a32) {
                            let a33 = v2;
                            let a34 = 2u64;
                            nat_lt(a33, a34)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let a35 = v0;
                        let a36 = 2u64;
                        if nat_eq(a35, a36) {
                            let a37 = v3;
                            let a38 = 2u64;
                            if nat_lt(a37, a38) {
                                Ok(Some({
                                    let a39 = v3;
                                    let a40 = 0u64;
                                    if nat_eq(a39, a40) {
                                        f5(v1, v2)?
                                    } else {
                                        4u64
                                    }
                                }))
                            } else {
                                Ok(None::<u64>)
                            }
                        } else {
                            let a41 = v3;
                            let a42 = f5(v1, v2)?;
                            if nat_lt(a41, a42) {
                                Ok(Some(f6(v2, v3)?))
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

pub fn f1(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a43 = v0;
    let a44 = 0u64;
    if nat_eq(a43, a44) {
        let a45 = v1;
        let a46 = 0u64;
        if if if nat_eq(a45, a46) {
            let a47 = v2;
            let a48 = 0u64;
            nat_eq(a47, a48)
        } else {
            false
        } {
            let a49 = v3;
            let a50 = 5u64;
            nat_lt(a49, a50)
        } else {
            false
        } {
            Ok(Some({
                let a51 = v3;
                let a52 = 2u64;
                if nat_lt(a51, a52) {
                    let a53 = v3;
                    let a54 = 0u64;
                    if nat_eq(a53, a54) {
                        10368u64
                    } else {
                        9u64
                    }
                } else {
                    let a55 = v3;
                    let a56 = 4u64;
                    if nat_lt(a55, a56) {
                        let a57 = v3;
                        let a58 = 2u64;
                        if nat_eq(a57, a58) {
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
        let a59 = v1;
        let a60 = 10368u64;
        if nat_lt(a59, a60) {
            let a61 = v0;
            let a62 = 1u64;
            if nat_eq(a61, a62) {
                let a63 = v2;
                let a64 = 0u64;
                if if nat_eq(a63, a64) {
                    let a65 = v3;
                    let a66 = 9u64;
                    nat_lt(a65, a66)
                } else {
                    false
                } {
                    Ok(Some(f7(v1, v3)?))
                } else {
                    Ok(None::<u64>)
                }
            } else {
                let a67 = v0;
                let a68 = 2u64;
                if nat_eq(a67, a68) {
                    let a69 = v2;
                    let a70 = 4u64;
                    if if nat_lt(a69, a70) {
                        let a71 = v3;
                        let a72 = 10u64;
                        nat_lt(a71, a72)
                    } else {
                        false
                    } {
                        Ok(Some({
                            let a73 = v3;
                            let a74 = 4u64;
                            if nat_lt(a73, a74) {
                                f8(v1, v2, v3)?
                            } else {
                                let a75 = v3;
                                let a76 = 8u64;
                                if nat_lt(a75, a76) {
                                    f9(v1, v2, {
                                        let a77 = v3;
                                        let a78 = 4u64;
                                        nat_sub(a77, a78)
                                    })?
                                } else {
                                    let a79 = v3;
                                    let a80 = 8u64;
                                    if nat_eq(a79, a80) {
                                        f10(v1, v2)?
                                    } else {
                                        f11(v1, v2)?
                                    }
                                }
                            }
                        }))
                    } else {
                        Ok(None::<u64>)
                    }
                } else {
                    let a81 = v0;
                    let a82 = 3u64;
                    if nat_eq(a81, a82) {
                        let a83 = v2;
                        let a84 = 2u64;
                        if if nat_lt(a83, a84) {
                            let a85 = v3;
                            let a86 = 256u64;
                            nat_lt(a85, a86)
                        } else {
                            false
                        } {
                            Ok(Some(f12(v1, v2, v3)?))
                        } else {
                            Ok(None::<u64>)
                        }
                    } else {
                        let a87 = v0;
                        let a88 = 4u64;
                        if if nat_eq(a87, a88) {
                            true
                        } else {
                            let a89 = v0;
                            let a90 = 5u64;
                            nat_eq(a89, a90)
                        } {
                            let a91 = v3;
                            let a92 = 2u64;
                            if nat_lt(a91, a92) {
                                let a93 = v2;
                                let a94 = 0u64;
                                if nat_eq(a93, a94) {
                                    let a95 = v3;
                                    let a96 = 0u64;
                                    if nat_eq(a95, a96) {
                                        Ok(Some(f13(v1, {
                                            let a97 = v0;
                                            let a98 = 4u64;
                                            if nat_eq(a97, a98) {
                                                0u64
                                            } else {
                                                1u64
                                            }
                                        })?))
                                    } else {
                                        Ok(None::<u64>)
                                    }
                                } else {
                                    let a101 = v2;
                                    let a102 = f13(v1, {
                                        let a99 = v0;
                                        let a100 = 4u64;
                                        if nat_eq(a99, a100) {
                                            0u64
                                        } else {
                                            1u64
                                        }
                                    })?;
                                    if nat_le(a101, a102) {
                                        Ok(Some({
                                            let a103 = v3;
                                            let a104 = 0u64;
                                            if nat_eq(a103, a104) {
                                                let a107 = {
                                                    let a105 = v2;
                                                    let a106 = 1u64;
                                                    nat_sub(a105, a106)
                                                };
                                                let a108 = 256u64;
                                                nat_mul(a107, a108)?
                                            } else {
                                                let v4: u64 = {
                                                    let a115 = f10(v1, {
                                                        let a109 = v0;
                                                        let a110 = 4u64;
                                                        if nat_eq(a109, a110) {
                                                            0u64
                                                        } else {
                                                            2u64
                                                        }
                                                    })?;
                                                    let a116 = {
                                                        let a113 = {
                                                            let a111 = v2;
                                                            let a112 = 1u64;
                                                            nat_sub(a111, a112)
                                                        };
                                                        let a114 = 256u64;
                                                        nat_mul(a113, a114)?
                                                    };
                                                    nat_sub(a115, a116)
                                                };
                                                let a117 = v4;
                                                let a118 = 256u64;
                                                if nat_lt(a117, a118) {
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
            let a127 = v3;
            let a128 = 0u64;
            if nat_eq(a127, a128) {
                Ok(Some(160u64))
            } else {
                f3(0u64, 0u64, 0u64, v3)
            }
        } else {
            Ok(None::<u64>)
        }
    } else {
        let a129 = v1;
        let a130 = 160u64;
        if nat_lt(a129, a130) {
            let a131 = v0;
            let a132 = 2u64;
            if nat_eq(a131, a132) {
                let a133 = v2;
                let a134 = 4u64;
                if if nat_lt(a133, a134) {
                    let a135 = v3;
                    let a136 = 10u64;
                    nat_lt(a135, a136)
                } else {
                    false
                } {
                    Ok(Some({
                        let a137 = v3;
                        let a138 = 4u64;
                        if nat_lt(a137, a138) {
                            f14(v1, v2, v3)?
                        } else {
                            let a139 = v3;
                            let a140 = 8u64;
                            if nat_lt(a139, a140) {
                                f15(v1, v2, {
                                    let a141 = v3;
                                    let a142 = 4u64;
                                    nat_sub(a141, a142)
                                })?
                            } else {
                                let a143 = v3;
                                let a144 = 8u64;
                                if nat_eq(a143, a144) {
                                    f16(v1, v2)?
                                } else {
                                    f17(v1, v2)?
                                }
                            }
                        }
                    }))
                } else {
                    Ok(None::<u64>)
                }
            } else {
                f3(v0, f18(v1)?, v2, v3)
            }
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f3(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a145 = v0;
    let a146 = 0u64;
    if nat_eq(a145, a146) {
        let a147 = v1;
        let a148 = 0u64;
        if if if nat_eq(a147, a148) {
            let a149 = v2;
            let a150 = 0u64;
            nat_eq(a149, a150)
        } else {
            false
        } {
            let a151 = v3;
            let a152 = 5u64;
            nat_lt(a151, a152)
        } else {
            false
        } {
            Ok(Some({
                let a153 = v3;
                let a154 = 2u64;
                if nat_lt(a153, a154) {
                    let a155 = v3;
                    let a156 = 0u64;
                    if nat_eq(a155, a156) {
                        3456u64
                    } else {
                        9u64
                    }
                } else {
                    let a157 = v3;
                    let a158 = 4u64;
                    if nat_lt(a157, a158) {
                        let a159 = v3;
                        let a160 = 2u64;
                        if nat_eq(a159, a160) {
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
        let a161 = v1;
        let a162 = 3456u64;
        if if nat_lt(a161, a162) {
            f19(v1)?
        } else {
            false
        } {
            let a163 = v0;
            let a164 = 2u64;
            if nat_lt(a163, a164) {
                let a165 = v0;
                let a166 = 1u64;
                if nat_eq(a165, a166) {
                    let a167 = v2;
                    let a168 = 0u64;
                    if if nat_eq(a167, a168) {
                        let a169 = v3;
                        let a170 = 9u64;
                        nat_lt(a169, a170)
                    } else {
                        false
                    } {
                        Ok(Some(f20(v1, v3)?))
                    } else {
                        Ok(None::<u64>)
                    }
                } else {
                    Ok(None::<u64>)
                }
            } else {
                let a171 = v0;
                let a172 = 4u64;
                if nat_lt(a171, a172) {
                    let a173 = v0;
                    let a174 = 2u64;
                    if nat_eq(a173, a174) {
                        let a175 = v2;
                        let a176 = 2u64;
                        if if nat_lt(a175, a176) {
                            let a177 = v3;
                            let a178 = 10u64;
                            nat_lt(a177, a178)
                        } else {
                            false
                        } {
                            Ok(Some({
                                let a179 = v3;
                                let a180 = 4u64;
                                if nat_lt(a179, a180) {
                                    f21(v1, v2, v3)?
                                } else {
                                    let a181 = v3;
                                    let a182 = 8u64;
                                    if nat_lt(a181, a182) {
                                        f22(v1, v2, {
                                            let a183 = v3;
                                            let a184 = 4u64;
                                            nat_sub(a183, a184)
                                        })?
                                    } else {
                                        let a185 = v3;
                                        let a186 = 8u64;
                                        if nat_eq(a185, a186) {
                                            f23(v1, v2)?
                                        } else {
                                            f24(v1, v2)?
                                        }
                                    }
                                }
                            }))
                        } else {
                            Ok(None::<u64>)
                        }
                    } else {
                        let a187 = v2;
                        let a188 = 0u64;
                        if if nat_eq(a187, a188) {
                            let a189 = v3;
                            let a190 = 256u64;
                            nat_lt(a189, a190)
                        } else {
                            false
                        } {
                            Ok(Some(f25(v1, v3)?))
                        } else {
                            Ok(None::<u64>)
                        }
                    }
                } else {
                    let a191 = v0;
                    let a192 = 4u64;
                    if nat_eq(a191, a192) {
                        let a193 = v3;
                        let a194 = 2u64;
                        if nat_lt(a193, a194) {
                            let a195 = v2;
                            let a196 = 0u64;
                            if nat_eq(a195, a196) {
                                let a197 = v3;
                                let a198 = 0u64;
                                if nat_eq(a197, a198) {
                                    Ok(Some(f26(v1)?))
                                } else {
                                    Ok(None::<u64>)
                                }
                            } else {
                                let a199 = v2;
                                let a200 = f26(v1)?;
                                if nat_le(a199, a200) {
                                    Ok(Some({
                                        let a201 = v3;
                                        let a202 = 0u64;
                                        if nat_eq(a201, a202) {
                                            let a205 = {
                                                let a203 = v2;
                                                let a204 = 1u64;
                                                nat_sub(a203, a204)
                                            };
                                            let a206 = 256u64;
                                            nat_mul(a205, a206)?
                                        } else {
                                            let v4: u64 = {
                                                let a211 = f23(v1, 0u64)?;
                                                let a212 = {
                                                    let a209 = {
                                                        let a207 = v2;
                                                        let a208 = 1u64;
                                                        nat_sub(a207, a208)
                                                    };
                                                    let a210 = 256u64;
                                                    nat_mul(a209, a210)?
                                                };
                                                nat_sub(a211, a212)
                                            };
                                            let a213 = v4;
                                            let a214 = 256u64;
                                            if nat_lt(a213, a214) {
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

pub fn f4(v0: u64, v1: u64) -> R<u64> {
    let a215 = v1;
    let a216 = 12u64;
    if nat_lt(a215, a216) {
        let a217 = v1;
        let a218 = 4u64;
        if nat_lt(a217, a218) {
            let a219 = v1;
            let a220 = 2u64;
            if nat_lt(a219, a220) {
                let a221 = v1;
                let a222 = 1u64;
                if nat_lt(a221, a222) {
                    f27(v0)
                } else {
                    let a223 = f27(v0)?;
                    let a224 = 4u64;
                    if nat_lt(a223, a224) {
                        let a225 = f27(v0)?;
                        let a226 = 2u64;
                        if nat_lt(a225, a226) {
                            let a227 = f27(v0)?;
                            let a228 = 1u64;
                            if nat_lt(a227, a228) {
                                Ok(2u64)
                            } else {
                                Ok(5u64)
                            }
                        } else {
                            let a229 = f27(v0)?;
                            let a230 = 3u64;
                            if nat_lt(a229, a230) {
                                Ok(6u64)
                            } else {
                                Ok(7u64)
                            }
                        }
                    } else {
                        let a231 = f27(v0)?;
                        let a232 = 6u64;
                        if nat_lt(a231, a232) {
                            let a233 = f27(v0)?;
                            let a234 = 5u64;
                            if nat_lt(a233, a234) {
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
                let a235 = v1;
                let a236 = 3u64;
                if nat_lt(a235, a236) {
                    let a237 = f27(v0)?;
                    let a238 = 6u64;
                    if nat_eq(a237, a238) {
                        Ok(2u64)
                    } else {
                        Ok(0u64)
                    }
                } else {
                    Ok(f28(v0))
                }
            }
        } else {
            Ok(f29(v0, {
                let a241 = {
                    let a239 = v1;
                    let a240 = 4u64;
                    nat_sub(a239, a240)
                };
                let a242 = 4u64;
                let a243 = 0u64;
                nat_quot(a241, a242, a243)
            }, {
                let a246 = {
                    let a244 = v1;
                    let a245 = 4u64;
                    nat_sub(a244, a245)
                };
                let a247 = 4u64;
                let a248 = 0u64;
                nat_rem(a246, a247, a248)
            }))
        }
    } else {
        let a249 = v1;
        let a250 = 20u64;
        if nat_lt(a249, a250) {
            f30(v0, {
                let a253 = {
                    let a251 = v1;
                    let a252 = 12u64;
                    nat_sub(a251, a252)
                };
                let a254 = 4u64;
                let a255 = 0u64;
                nat_quot(a253, a254, a255)
            }, {
                let a258 = {
                    let a256 = v1;
                    let a257 = 12u64;
                    nat_sub(a256, a257)
                };
                let a259 = 4u64;
                let a260 = 0u64;
                nat_rem(a258, a259, a260)
            })
        } else {
            let a261 = v1;
            let a262 = 22u64;
            if nat_lt(a261, a262) {
                f5(v0, {
                    let a263 = v1;
                    let a264 = 20u64;
                    nat_sub(a263, a264)
                })
            } else {
                let a265 = v1;
                let a266 = 22u64;
                if nat_eq(a265, a266) {
                    let a267 = v0;
                    let a268 = 0u64;
                    if nat_eq(a267, a268) {
                        Ok(1u64)
                    } else {
                        let a269 = v0;
                        let a270 = 108u64;
                        if nat_eq(a269, a270) {
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

pub fn f5(v0: u64, v1: u64) -> R<u64> {
    let a271 = v1;
    let a272 = 0u64;
    if nat_eq(a271, a272) {
        let a273 = f31(v0);
        let a274 = 12u64;
        nat_mul(a273, a274)
    } else {
        let a275 = 96u64;
        let a276 = v0;
        if nat_le(a275, a276) {
            Ok(0u64)
        } else {
            let a277 = f32(v0);
            let a278 = 2u64;
            if nat_lt(a277, a278) {
                let a279 = f32(v0);
                let a280 = 1u64;
                if nat_lt(a279, a280) {
                    let a281 = f31(v0);
                    let a282 = 12u64;
                    nat_mul(a281, a282)
                } else {
                    Ok(1u64)
                }
            } else {
                let a283 = f32(v0);
                let a284 = 3u64;
                if nat_lt(a283, a284) {
                    Ok(f31(v0))
                } else {
                    Ok(6u64)
                }
            }
        }
    }
}

pub fn f6(v0: u64, v1: u64) -> R<u64> {
    let a285 = v1;
    let a286 = 16u64;
    if nat_lt(a285, a286) {
        Ok(f33(f34(v0, v1)?))
    } else {
        Ok(f35({
            let a287 = f36(v0);
            let a288 = f37(v1)?;
            nat_add(a287, a288)?
        }))
    }
}

pub fn f7(v0: u64, v1: u64) -> R<u64> {
    let a289 = v1;
    let a290 = 0u64;
    if nat_eq(a289, a290) {
        f38(v0)
    } else {
        let a291 = v1;
        let a292 = 1u64;
        if nat_eq(a291, a292) {
            Ok(f39(v0))
        } else {
            let a293 = v1;
            let a294 = 2u64;
            if nat_eq(a293, a294) {
                Ok(0u64)
            } else {
                let a295 = v1;
                let a296 = 3u64;
                if nat_eq(a295, a296) {
                    Ok(f40(v0))
                } else {
                    let a297 = v1;
                    let a298 = 4u64;
                    if nat_eq(a297, a298) {
                        Ok(f41(v0))
                    } else {
                        let a299 = v1;
                        let a300 = 5u64;
                        if nat_eq(a299, a300) {
                            Ok(f42(v0))
                        } else {
                            let a301 = v1;
                            let a302 = 6u64;
                            if nat_eq(a301, a302) {
                                f43(v0)
                            } else {
                                let a303 = v1;
                                let a304 = 7u64;
                                if nat_eq(a303, a304) {
                                    f44(v0, 0u64)
                                } else {
                                    f44(v0, 1u64)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f8(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a305 = v1;
    let a306 = 2u64;
    if nat_lt(a305, a306) {
        let a307 = v1;
        let a308 = 0u64;
        if nat_eq(a307, a308) {
            f45(v0, v2)
        } else {
            let a309 = f40(v0);
            let a310 = 3u64;
            if nat_eq(a309, a310) {
                let a311 = v2;
                let a312 = 2u64;
                if nat_lt(a311, a312) {
                    let a313 = v2;
                    let a314 = 0u64;
                    if nat_eq(a313, a314) {
                        f46(v0)
                    } else {
                        let a315 = f47(v0)?;
                        let a316 = f46(v0)?;
                        let a317 = 0u64;
                        Ok(nat_quot(a315, a316, a317))
                    }
                } else {
                    Ok(1u64)
                }
            } else {
                f45(v0, v2)
            }
        }
    } else {
        let a318 = v1;
        let a319 = 2u64;
        if nat_eq(a318, a319) {
            f48(v0, v2)
        } else {
            let a320 = v2;
            let a321 = f39(v0);
            if nat_eq(a320, a321) {
                let a322 = f45(v0, v2)?;
                let a323 = f48(v0, v2)?;
                nat_add(a322, a323)
            } else {
                f45(v0, v2)
            }
        }
    }
}

pub fn f9(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a324 = v2;
    let a325 = 2u64;
    if nat_lt(a324, a325) {
        let a326 = v2;
        let a327 = 0u64;
        if nat_eq(a326, a327) {
            f49(v0, v1)
        } else {
            f50(v0, v1)
        }
    } else {
        let a328 = v2;
        let a329 = 2u64;
        if nat_eq(a328, a329) {
            f51(v0, v1)
        } else {
            f52(v0, v1)
        }
    }
}

pub fn f10(v0: u64, v1: u64) -> R<u64> {
    let m330 = f53(Adt0::C0(f8(v0, v1, 0u64)?, f8(v0, v1, 1u64)?, f8(v0, v1, 2u64)?, f8(v0, v1, 3u64)?, f9(v0, v1, 0u64)?, f9(v0, v1, 1u64)?, f9(v0, v1, 2u64)?, f9(v0, v1, 3u64)?, f46(v0)?, f54(v0)?))?;
    match m330 {
        Adt1::C0 => {
            Ok(0u64)
        }
        Adt1::C1(v2) => {
            Ok(v2)
        }
    }
}

pub fn f11(v0: u64, v1: u64) -> R<u64> {
    let a335 = {
        let a331 = f8(v0, v1, 0u64)?;
        let a332 = f8(v0, v1, 1u64)?;
        nat_mul(a331, a332)?
    };
    let a336 = {
        let a333 = f8(v0, v1, 2u64)?;
        let a334 = f8(v0, v1, 3u64)?;
        nat_mul(a333, a334)?
    };
    nat_mul(a335, a336)
}

pub fn f12(v0: u64, v1: u64, v2: u64) -> R<u64> {
    f55(f44(v0, v1)?, v2)
}

pub fn f13(v0: u64, v1: u64) -> R<u64> {
    let a341 = {
        let a339 = f10(v0, {
            let a337 = v1;
            let a338 = 0u64;
            if nat_eq(a337, a338) {
                0u64
            } else {
                2u64
            }
        })?;
        let a340 = 255u64;
        nat_add(a339, a340)?
    };
    let a342 = 256u64;
    let a343 = 0u64;
    Ok(nat_quot(a341, a342, a343))
}

pub fn f14(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a344 = v1;
    let a345 = 2u64;
    if nat_lt(a344, a345) {
        f21(f18(v0)?, v1, v2)
    } else {
        let a359 = f21(f18(v0)?, 0u64, v2)?;
        let a360 = {
            let a346 = v2;
            let a347 = 2u64;
            if nat_lt(a346, a347) {
                let a348 = v2;
                let a349 = 0u64;
                if nat_eq(a348, a349) {
                    2u64
                } else {
                    let a353 = {
                        let a350 = f56(v0)?;
                        let a351 = 2u64;
                        let a352 = 0u64;
                        nat_rem(a350, a351, a352)
                    };
                    let a354 = 1u64;
                    if nat_eq(a353, a354) {
                        3u64
                    } else {
                        1u64
                    }
                }
            } else {
                let a355 = v2;
                let a356 = 2u64;
                if nat_eq(a355, a356) {
                    let a357 = 2u64;
                    let a358 = f56(v0)?;
                    if nat_le(a357, a358) {
                        2u64
                    } else {
                        1u64
                    }
                } else {
                    2u64
                }
            }
        };
        nat_mul(a359, a360)
    }
}

pub fn f15(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a361 = v1;
    let a362 = 2u64;
    if nat_lt(a361, a362) {
        f22(f18(v0)?, v1, v2)
    } else {
        let a363 = v2;
        let a364 = 2u64;
        if nat_lt(a363, a364) {
            let a365 = v2;
            let a366 = 0u64;
            if nat_eq(a365, a366) {
                f57(v0)
            } else {
                f58(v0, v1)
            }
        } else {
            let a367 = v2;
            let a368 = 2u64;
            if nat_eq(a367, a368) {
                f59(v0, v1)
            } else {
                f60(v0, v1)
            }
        }
    }
}

pub fn f16(v0: u64, v1: u64) -> R<u64> {
    let a369 = v1;
    let a370 = 2u64;
    if nat_lt(a369, a370) {
        f23(f18(v0)?, v1)
    } else {
        let m371 = f53(Adt0::C0(f14(v0, v1, 0u64)?, f14(v0, v1, 1u64)?, f14(v0, v1, 2u64)?, f14(v0, v1, 3u64)?, f15(v0, v1, 0u64)?, f15(v0, v1, 1u64)?, f15(v0, v1, 2u64)?, f15(v0, v1, 3u64)?, 1u64, f57(v0)?))?;
        match m371 {
            Adt1::C0 => {
                Ok(0u64)
            }
            Adt1::C1(v2) => {
                Ok(v2)
            }
        }
    }
}

pub fn f17(v0: u64, v1: u64) -> R<u64> {
    let a376 = {
        let a372 = f14(v0, v1, 0u64)?;
        let a373 = f14(v0, v1, 1u64)?;
        nat_mul(a372, a373)?
    };
    let a377 = {
        let a374 = f14(v0, v1, 2u64)?;
        let a375 = f14(v0, v1, 3u64)?;
        nat_mul(a374, a375)?
    };
    nat_mul(a376, a377)
}

pub fn f18(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a378 = v0;
        let a379 = 32u64;
        let a380 = 0u64;
        nat_quot(a378, a379, a380)
    };
    let a392 = {
        let a387 = {
            let a381 = v1;
            let a382 = 2u64;
            if nat_lt(a381, a382) {
                v1
            } else {
                let a383 = v1;
                let a384 = 2u64;
                if nat_eq(a383, a384) {
                    23u64
                } else {
                    let a385 = v1;
                    let a386 = 3u64;
                    if nat_eq(a385, a386) {
                        24u64
                    } else {
                        28u64
                    }
                }
            }
        };
        let a388 = 96u64;
        nat_mul(a387, a388)?
    };
    let a393 = {
        let a389 = v0;
        let a390 = 32u64;
        let a391 = 0u64;
        nat_rem(a389, a390, a391)
    };
    nat_add(a392, a393)
}

pub fn f19(v0: u64) -> R<bool> {
    let m394 = f61(f62(v0)?);
    match m394 {
        Adt2::C0 => {
            Ok(false)
        }
        Adt2::C1(_) => {
            Ok(true)
        }
    }
}

pub fn f20(v0: u64, v1: u64) -> R<u64> {
    let a395 = v1;
    let a396 = 0u64;
    if nat_eq(a395, a396) {
        f62(v0)
    } else {
        let a397 = v1;
        let a398 = 1u64;
        if nat_eq(a397, a398) {
            Ok(f63(v0))
        } else {
            let a399 = v1;
            let a400 = 2u64;
            if nat_eq(a399, a400) {
                Ok(0u64)
            } else {
                let a401 = v1;
                let a402 = 3u64;
                if nat_eq(a401, a402) {
                    Ok(f64(v0))
                } else {
                    let a403 = v1;
                    let a404 = 4u64;
                    if nat_eq(a403, a404) {
                        f65(v0)
                    } else {
                        let a405 = v1;
                        let a406 = 5u64;
                        if nat_eq(a405, a406) {
                            f66(v0)
                        } else {
                            let a407 = v1;
                            let a408 = 6u64;
                            if nat_eq(a407, a408) {
                                f67(v0)
                            } else {
                                let a409 = v1;
                                let a410 = 7u64;
                                if nat_eq(a409, a410) {
                                    f68(v0)
                                } else {
                                    let a411 = f63(v0);
                                    let a412 = 1u64;
                                    if if nat_eq(a411, a412) {
                                        let a413 = f64(v0);
                                        let a414 = 3u64;
                                        nat_eq(a413, a414)
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

pub fn f21(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a415 = v1;
    let a416 = 1u64;
    if if nat_eq(a415, a416) {
        let a417 = f64(v0);
        let a418 = 3u64;
        nat_eq(a417, a418)
    } else {
        false
    } {
        let a419 = v2;
        let a420 = 2u64;
        if nat_lt(a419, a420) {
            let a421 = v2;
            let a422 = 0u64;
            if nat_eq(a421, a422) {
                f69(v0)
            } else {
                let a427 = {
                    let a423 = f65(v0)?;
                    let a424 = f66(v0)?;
                    nat_mul(a423, a424)?
                };
                let a428 = {
                    let a425 = 2u64;
                    let a426 = f67(v0)?;
                    nat_mul(a425, a426)?
                };
                nat_mul(a427, a428)
            }
        } else {
            Ok(1u64)
        }
    } else {
        let a429 = v2;
        let a430 = 2u64;
        if nat_lt(a429, a430) {
            let a431 = v2;
            let a432 = 0u64;
            if nat_eq(a431, a432) {
                let a433 = f69(v0)?;
                let a434 = f65(v0)?;
                nat_mul(a433, a434)
            } else {
                f66(v0)
            }
        } else {
            let a435 = v2;
            let a436 = 2u64;
            if nat_eq(a435, a436) {
                Ok(2u64)
            } else {
                f67(v0)
            }
        }
    }
}

pub fn f22(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a437 = v2;
    let a438 = 2u64;
    if nat_lt(a437, a438) {
        let a439 = v2;
        let a440 = 0u64;
        if nat_eq(a439, a440) {
            f70(v0, v1)
        } else {
            f71(v0, v1)
        }
    } else {
        let a441 = v2;
        let a442 = 2u64;
        if nat_eq(a441, a442) {
            f72(v0, v1)
        } else {
            f73(v0, v1)
        }
    }
}

pub fn f23(v0: u64, v1: u64) -> R<u64> {
    let m443 = f53(Adt0::C0(f21(v0, v1, 0u64)?, f21(v0, v1, 1u64)?, f21(v0, v1, 2u64)?, f21(v0, v1, 3u64)?, f22(v0, v1, 0u64)?, f22(v0, v1, 1u64)?, f22(v0, v1, 2u64)?, f22(v0, v1, 3u64)?, f69(v0)?, f74(v0)?))?;
    match m443 {
        Adt1::C0 => {
            Ok(0u64)
        }
        Adt1::C1(v2) => {
            Ok(v2)
        }
    }
}

pub fn f24(v0: u64, v1: u64) -> R<u64> {
    let a448 = {
        let a444 = f21(v0, v1, 0u64)?;
        let a445 = f21(v0, v1, 1u64)?;
        nat_mul(a444, a445)?
    };
    let a449 = {
        let a446 = f21(v0, v1, 2u64)?;
        let a447 = f21(v0, v1, 3u64)?;
        nat_mul(a446, a447)?
    };
    nat_mul(a448, a449)
}

pub fn f25(v0: u64, v1: u64) -> R<u64> {
    f75(f68(v0)?, v1)
}

pub fn f26(v0: u64) -> R<u64> {
    let a452 = {
        let a450 = f23(v0, 0u64)?;
        let a451 = 255u64;
        nat_add(a450, a451)?
    };
    let a453 = 256u64;
    let a454 = 0u64;
    Ok(nat_quot(a452, a453, a454))
}

pub fn f27(v0: u64) -> R<u64> {
    let a455 = v0;
    let a456 = 96u64;
    if nat_lt(a455, a456) {
        let a457 = v0;
        let a458 = 24u64;
        let a459 = 0u64;
        Ok(nat_quot(a457, a458, a459))
    } else {
        let a465 = 4u64;
        let a466 = {
            let a462 = {
                let a460 = v0;
                let a461 = 96u64;
                nat_sub(a460, a461)
            };
            let a463 = 6u64;
            let a464 = 0u64;
            nat_quot(a462, a463, a464)
        };
        nat_add(a465, a466)
    }
}

pub fn f28(v0: u64) -> u64 {
    let a467 = v0;
    let a468 = 96u64;
    if nat_lt(a467, a468) {
        let a472 = {
            let a469 = v0;
            let a470 = 4u64;
            let a471 = 0u64;
            nat_quot(a469, a470, a471)
        };
        let a473 = 2u64;
        let a474 = 0u64;
        nat_rem(a472, a473, a474)
    } else {
        let a475 = v0;
        let a476 = 2u64;
        let a477 = 0u64;
        nat_rem(a475, a476, a477)
    }
}

pub fn f29(v0: u64, v1: u64, v2: u64) -> u64 {
    let a478 = v1;
    let a479 = 0u64;
    if nat_eq(a478, a479) {
        let a480 = v2;
        let a481 = 2u64;
        if nat_lt(a480, a481) {
            let a482 = v2;
            let a483 = 1u64;
            if nat_lt(a482, a483) {
                f31(v0)
            } else {
                3u64
            }
        } else {
            2u64
        }
    } else {
        let a484 = 96u64;
        let a485 = v0;
        if nat_le(a484, a485) {
            0u64
        } else {
            let a486 = v2;
            let a487 = 2u64;
            if nat_lt(a486, a487) {
                let a488 = v2;
                let a489 = 1u64;
                if nat_lt(a488, a489) {
                    let a490 = f32(v0);
                    let a491 = 1u64;
                    if if nat_eq(a490, a491) {
                        true
                    } else {
                        let a492 = f32(v0);
                        let a493 = 3u64;
                        nat_eq(a492, a493)
                    } {
                        1u64
                    } else {
                        f31(v0)
                    }
                } else {
                    let a494 = f32(v0);
                    let a495 = 1u64;
                    if if nat_eq(a494, a495) {
                        true
                    } else {
                        let a496 = f32(v0);
                        let a497 = 2u64;
                        nat_eq(a496, a497)
                    } {
                        1u64
                    } else {
                        3u64
                    }
                }
            } else {
                let a498 = v2;
                let a499 = 3u64;
                if nat_lt(a498, a499) {
                    let a500 = f32(v0);
                    let a501 = 0u64;
                    if nat_eq(a500, a501) {
                        2u64
                    } else {
                        1u64
                    }
                } else {
                    let a502 = f32(v0);
                    let a503 = 1u64;
                    if if nat_eq(a502, a503) {
                        true
                    } else {
                        let a504 = f32(v0);
                        let a505 = 2u64;
                        nat_eq(a504, a505)
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

pub fn f30(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a506 = v1;
    let a507 = 1u64;
    if if nat_eq(a506, a507) {
        let a508 = 96u64;
        let a509 = v0;
        nat_le(a508, a509)
    } else {
        false
    } {
        Ok(0u64)
    } else {
        let v3: u64 = f31(v0);
        let v4: u64 = f28(v0);
        let v5: u64 = {
            let a510 = v1;
            let a511 = 0u64;
            if nat_eq(a510, a511) {
                0u64
            } else {
                f32(v0)
            }
        };
        let a512 = v5;
        let a513 = 2u64;
        if nat_lt(a512, a513) {
            let a514 = v5;
            let a515 = 1u64;
            if nat_lt(a514, a515) {
                let a516 = v2;
                let a517 = 2u64;
                if nat_lt(a516, a517) {
                    let a518 = v2;
                    let a519 = 1u64;
                    if nat_lt(a518, a519) {
                        Ok(4u64)
                    } else {
                        let a522 = {
                            let a520 = v3;
                            let a521 = v4;
                            nat_add(a520, a521)?
                        };
                        let a523 = 4u64;
                        nat_mul(a522, a523)
                    }
                } else {
                    let a524 = v2;
                    let a525 = 3u64;
                    if nat_lt(a524, a525) {
                        let a532 = {
                            let a530 = {
                                let a526 = v3;
                                let a527 = 3u64;
                                nat_mul(a526, a527)?
                            };
                            let a531 = {
                                let a528 = v4;
                                let a529 = 4u64;
                                nat_mul(a528, a529)?
                            };
                            nat_add(a530, a531)?
                        };
                        let a533 = 4u64;
                        nat_mul(a532, a533)
                    } else {
                        let a540 = {
                            let a538 = {
                                let a534 = v3;
                                let a535 = 6u64;
                                nat_mul(a534, a535)?
                            };
                            let a539 = {
                                let a536 = v4;
                                let a537 = 9u64;
                                nat_mul(a536, a537)?
                            };
                            nat_add(a538, a539)?
                        };
                        let a541 = 4u64;
                        nat_mul(a540, a541)
                    }
                }
            } else {
                let a542 = v2;
                let a543 = 2u64;
                if nat_lt(a542, a543) {
                    let a544 = v2;
                    let a545 = 1u64;
                    if nat_lt(a544, a545) {
                        Ok(4u64)
                    } else {
                        let a548 = {
                            let a546 = 1u64;
                            let a547 = v4;
                            nat_add(a546, a547)?
                        };
                        let a549 = 4u64;
                        nat_mul(a548, a549)
                    }
                } else {
                    let a550 = v2;
                    let a551 = 3u64;
                    if nat_lt(a550, a551) {
                        let a556 = {
                            let a554 = 1u64;
                            let a555 = {
                                let a552 = v4;
                                let a553 = 2u64;
                                nat_mul(a552, a553)?
                            };
                            nat_add(a554, a555)?
                        };
                        let a557 = 4u64;
                        nat_mul(a556, a557)
                    } else {
                        let a562 = {
                            let a560 = 1u64;
                            let a561 = {
                                let a558 = v4;
                                let a559 = 3u64;
                                nat_mul(a558, a559)?
                            };
                            nat_add(a560, a561)?
                        };
                        let a563 = 4u64;
                        nat_mul(a562, a563)
                    }
                }
            }
        } else {
            let a564 = v5;
            let a565 = 3u64;
            if nat_lt(a564, a565) {
                let a566 = v2;
                let a567 = 2u64;
                if nat_lt(a566, a567) {
                    let a568 = v2;
                    let a569 = 1u64;
                    if nat_lt(a568, a569) {
                        Ok(4u64)
                    } else {
                        let a572 = {
                            let a570 = v3;
                            let a571 = v4;
                            nat_add(a570, a571)?
                        };
                        let a573 = 4u64;
                        nat_mul(a572, a573)
                    }
                } else {
                    let a574 = v2;
                    let a575 = 3u64;
                    if nat_lt(a574, a575) {
                        let a580 = {
                            let a578 = v3;
                            let a579 = {
                                let a576 = v4;
                                let a577 = 2u64;
                                nat_mul(a576, a577)?
                            };
                            nat_add(a578, a579)?
                        };
                        let a581 = 4u64;
                        nat_mul(a580, a581)
                    } else {
                        let a586 = {
                            let a584 = v3;
                            let a585 = {
                                let a582 = v4;
                                let a583 = 3u64;
                                nat_mul(a582, a583)?
                            };
                            nat_add(a584, a585)?
                        };
                        let a587 = 4u64;
                        nat_mul(a586, a587)
                    }
                }
            } else {
                let a588 = v2;
                let a589 = 2u64;
                if nat_lt(a588, a589) {
                    let a590 = v2;
                    let a591 = 1u64;
                    if nat_lt(a590, a591) {
                        Ok(4u64)
                    } else {
                        let a594 = {
                            let a592 = 1u64;
                            let a593 = v4;
                            nat_add(a592, a593)?
                        };
                        let a595 = 4u64;
                        nat_mul(a594, a595)
                    }
                } else {
                    let a596 = v2;
                    let a597 = 3u64;
                    if nat_lt(a596, a597) {
                        let a602 = {
                            let a600 = 3u64;
                            let a601 = {
                                let a598 = v4;
                                let a599 = 4u64;
                                nat_mul(a598, a599)?
                            };
                            nat_add(a600, a601)?
                        };
                        let a603 = 4u64;
                        nat_mul(a602, a603)
                    } else {
                        let a608 = {
                            let a606 = 3u64;
                            let a607 = {
                                let a604 = v4;
                                let a605 = 5u64;
                                nat_mul(a604, a605)?
                            };
                            nat_add(a606, a607)?
                        };
                        let a609 = 4u64;
                        nat_mul(a608, a609)
                    }
                }
            }
        }
    }
}

pub fn f31(v0: u64) -> u64 {
    let a610 = f76(v0);
    let a611 = 0u64;
    if nat_eq(a610, a611) {
        1u64
    } else {
        let a612 = f76(v0);
        let a613 = 1u64;
        if nat_eq(a612, a613) {
            17u64
        } else {
            257u64
        }
    }
}

pub fn f32(v0: u64) -> u64 {
    let a614 = v0;
    let a615 = 4u64;
    let a616 = 0u64;
    nat_rem(a614, a615, a616)
}

pub fn f33(v0: u64) -> u64 {
    let a617 = v0;
    let a618 = 8u64;
    if nat_lt(a617, a618) {
        let a619 = v0;
        let a620 = 4u64;
        if nat_lt(a619, a620) {
            let a621 = v0;
            let a622 = 2u64;
            if nat_lt(a621, a622) {
                let a623 = v0;
                let a624 = 1u64;
                if nat_lt(a623, a624) {
                    0u64
                } else {
                    2147483648u64
                }
            } else {
                let a625 = v0;
                let a626 = 3u64;
                if nat_lt(a625, a626) {
                    1065353216u64
                } else {
                    3212836864u64
                }
            }
        } else {
            let a627 = v0;
            let a628 = 6u64;
            if nat_lt(a627, a628) {
                let a629 = v0;
                let a630 = 5u64;
                if nat_lt(a629, a630) {
                    2139095040u64
                } else {
                    4286578688u64
                }
            } else {
                let a631 = v0;
                let a632 = 7u64;
                if nat_lt(a631, a632) {
                    2143289345u64
                } else {
                    1u64
                }
            }
        }
    } else {
        let a633 = v0;
        let a634 = 12u64;
        if nat_lt(a633, a634) {
            let a635 = v0;
            let a636 = 10u64;
            if nat_lt(a635, a636) {
                let a637 = v0;
                let a638 = 9u64;
                if nat_lt(a637, a638) {
                    2139095039u64
                } else {
                    8388608u64
                }
            } else {
                let a639 = v0;
                let a640 = 11u64;
                if nat_lt(a639, a640) {
                    8388607u64
                } else {
                    1065353217u64
                }
            }
        } else {
            let a641 = v0;
            let a642 = 14u64;
            if nat_lt(a641, a642) {
                let a643 = v0;
                let a644 = 13u64;
                if nat_lt(a643, a644) {
                    1056964608u64
                } else {
                    8388609u64
                }
            } else {
                let a645 = v0;
                let a646 = 15u64;
                if nat_lt(a645, a646) {
                    2147483649u64
                } else {
                    2139095041u64
                }
            }
        }
    }
}

pub fn f34(v0: u64, v1: u64) -> R<u64> {
    let a655 = {
        let a653 = {
            let a649 = {
                let a647 = v0;
                let a648 = 0u64;
                if nat_eq(a647, a648) {
                    1u64
                } else {
                    5u64
                }
            };
            let a650 = v1;
            nat_mul(a649, a650)?
        };
        let a654 = {
            let a651 = v0;
            let a652 = 0u64;
            if nat_eq(a651, a652) {
                0u64
            } else {
                3u64
            }
        };
        nat_add(a653, a654)?
    };
    let a656 = 16u64;
    let a657 = 0u64;
    Ok(nat_rem(a655, a656, a657))
}

pub fn f35(v0: u64) -> u64 {
    let a658 = v0;
    let a659 = 4294967296u64;
    let a660 = 0u64;
    nat_rem(a658, a659, a660)
}

pub fn f36(v0: u64) -> u64 {
    let a661 = v0;
    let a662 = 0u64;
    if nat_eq(a661, a662) {
        2654435769u64
    } else {
        2246822507u64
    }
}

pub fn f37(v0: u64) -> R<u64> {
    let a663 = 2654435761u64;
    let a664 = v0;
    nat_mul(a663, a664)
}

pub fn f38(v0: u64) -> R<u64> {
    f77(f78(v0))
}

pub fn f39(v0: u64) -> u64 {
    let a668 = {
        let a665 = v0;
        let a666 = 72u64;
        let a667 = 0u64;
        nat_quot(a665, a666, a667)
    };
    let a669 = 4u64;
    let a670 = 0u64;
    nat_rem(a668, a669, a670)
}

pub fn f40(v0: u64) -> u64 {
    let a674 = {
        let a671 = v0;
        let a672 = 18u64;
        let a673 = 0u64;
        nat_quot(a671, a672, a673)
    };
    let a675 = 4u64;
    let a676 = 0u64;
    nat_rem(a674, a675, a676)
}

pub fn f41(v0: u64) -> u64 {
    f79({
        let a680 = {
            let a677 = v0;
            let a678 = 6u64;
            let a679 = 0u64;
            nat_quot(a677, a678, a679)
        };
        let a681 = 3u64;
        let a682 = 0u64;
        nat_rem(a680, a681, a682)
    })
}

pub fn f42(v0: u64) -> u64 {
    f79({
        let a686 = {
            let a683 = v0;
            let a684 = 2u64;
            let a685 = 0u64;
            nat_quot(a683, a684, a685)
        };
        let a687 = 3u64;
        let a688 = 0u64;
        nat_rem(a686, a687, a688)
    })
}

pub fn f43(v0: u64) -> R<u64> {
    let a692 = 1u64;
    let a693 = {
        let a689 = v0;
        let a690 = 2u64;
        let a691 = 0u64;
        nat_rem(a689, a690, a691)
    };
    nat_add(a692, a693)
}

pub fn f44(v0: u64, v1: u64) -> R<u64> {
    let a702 = {
        let a698 = {
            let a696 = {
                let a694 = f78(v0);
                let a695 = 4u64;
                nat_mul(a694, a695)?
            };
            let a697 = f40(v0);
            nat_add(a696, a697)?
        };
        let a699 = 2u64;
        nat_mul(a698, a699)?
    };
    let a703 = {
        let a700 = v1;
        let a701 = 1u64;
        if nat_eq(a700, a701) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a702, a703)
}

pub fn f45(v0: u64, v1: u64) -> R<u64> {
    let a704 = v1;
    let a705 = 2u64;
    if nat_lt(a704, a705) {
        let a706 = v1;
        let a707 = 0u64;
        if nat_eq(a706, a707) {
            let a708 = f46(v0)?;
            let a709 = f41(v0);
            nat_mul(a708, a709)
        } else {
            Ok(f42(v0))
        }
    } else {
        let a710 = v1;
        let a711 = 2u64;
        if nat_eq(a710, a711) {
            Ok(2u64)
        } else {
            f43(v0)
        }
    }
}

pub fn f46(v0: u64) -> R<u64> {
    f69(f80(v0)?)
}

pub fn f47(v0: u64) -> R<u64> {
    let a716 = {
        let a712 = f45(v0, 0u64)?;
        let a713 = f45(v0, 1u64)?;
        nat_mul(a712, a713)?
    };
    let a717 = {
        let a714 = f45(v0, 2u64)?;
        let a715 = f45(v0, 3u64)?;
        nat_mul(a714, a715)?
    };
    nat_mul(a716, a717)
}

pub fn f48(v0: u64, v1: u64) -> R<u64> {
    let a718 = v1;
    let a719 = f39(v0);
    if nat_eq(a718, a719) {
        let a720 = f40(v0);
        let a721 = 3u64;
        if nat_eq(a720, a721) {
            Ok(0u64)
        } else {
            let a724 = f45(v0, v1)?;
            let a725 = {
                let a722 = v1;
                let a723 = 0u64;
                if nat_eq(a722, a723) {
                    f46(v0)?
                } else {
                    1u64
                }
            };
            nat_add(a724, a725)
        }
    } else {
        f45(v0, v1)
    }
}

pub fn f49(v0: u64, v1: u64) -> R<u64> {
    let a732 = f54(v0)?;
    let a733 = {
        let a726 = v1;
        let a727 = 0u64;
        if if if nat_eq(a726, a727) {
            true
        } else {
            let a728 = v1;
            let a729 = 2u64;
            nat_eq(a728, a729)
        } {
            let a730 = f40(v0);
            let a731 = 2u64;
            nat_eq(a730, a731)
        } else {
            false
        } {
            2u64
        } else {
            1u64
        }
    };
    nat_mul(a732, a733)
}

pub fn f50(v0: u64, v1: u64) -> R<u64> {
    let a739 = {
        let a737 = f49(v0, v1)?;
        let a738 = {
            let a734 = f8(v0, v1, 0u64)?;
            let a735 = f46(v0)?;
            let a736 = 0u64;
            nat_quot(a734, a735, a736)
        };
        nat_mul(a737, a738)?
    };
    let a740 = f81(v0, v1)?;
    nat_add(a739, a740)
}

pub fn f51(v0: u64, v1: u64) -> R<u64> {
    let a743 = {
        let a741 = f50(v0, v1)?;
        let a742 = f8(v0, v1, 1u64)?;
        nat_mul(a741, a742)?
    };
    let a744 = f81(v0, v1)?;
    nat_add(a743, a744)
}

pub fn f52(v0: u64, v1: u64) -> R<u64> {
    let a747 = {
        let a745 = f51(v0, v1)?;
        let a746 = f8(v0, v1, 2u64)?;
        nat_mul(a745, a746)?
    };
    let a748 = f81(v0, v1)?;
    nat_add(a747, a748)
}

pub fn f53(v0: Adt0) -> R<Adt1> {
    let a750 = match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h749, _) => {
            h749
        }
    };
    let a751 = 0u64;
    if nat_eq(a750, a751) {
        Ok(Adt1::C0)
    } else {
        let a753 = match v0.clone() {
            Adt0::C0(h752, _, _, _, _, _, _, _, _, _) => {
                h752
            }
        };
        let a754 = 0u64;
        if if if if nat_eq(a753, a754) {
            true
        } else {
            let a756 = match v0.clone() {
                Adt0::C0(_, h755, _, _, _, _, _, _, _, _) => {
                    h755
                }
            };
            let a757 = 0u64;
            nat_eq(a756, a757)
        } {
            true
        } else {
            let a759 = match v0.clone() {
                Adt0::C0(_, _, h758, _, _, _, _, _, _, _) => {
                    h758
                }
            };
            let a760 = 0u64;
            nat_eq(a759, a760)
        } {
            true
        } else {
            let a762 = match v0.clone() {
                Adt0::C0(_, _, _, h761, _, _, _, _, _, _) => {
                    h761
                }
            };
            let a763 = 0u64;
            nat_eq(a762, a763)
        } {
            Ok(Adt1::C1(0u64))
        } else {
            let a765 = match v0.clone() {
                Adt0::C0(_, _, _, _, _, _, _, _, h764, _) => {
                    h764
                }
            };
            let a766 = 1u64;
            if nat_eq(a765, a766) {
                Ok(Adt1::C1({
                    let a776 = {
                        let a774 = match v0.clone() {
                            Adt0::C0(_, _, _, _, _, _, _, _, _, h767) => {
                                h767
                            }
                        };
                        let a775 = {
                            let a772 = {
                                let a769 = match v0.clone() {
                                    Adt0::C0(h768, _, _, _, _, _, _, _, _, _) => {
                                        h768
                                    }
                                };
                                let a770 = 1u64;
                                nat_sub(a769, a770)
                            };
                            let a773 = match v0.clone() {
                                Adt0::C0(_, _, _, _, h771, _, _, _, _, _) => {
                                    h771
                                }
                            };
                            nat_mul(a772, a773)?
                        };
                        nat_add(a774, a775)?
                    };
                    let a777 = f82(v0)?;
                    nat_add(a776, a777)?
                }))
            } else {
                Ok(Adt1::C1({
                    let a786 = {
                        let a783 = {
                            let a780 = match v0.clone() {
                                Adt0::C0(h778, _, _, _, _, _, _, _, _, _) => {
                                    h778
                                }
                            };
                            let a781 = match v0.clone() {
                                Adt0::C0(_, _, _, _, h779, _, _, _, _, _) => {
                                    h779
                                }
                            };
                            nat_mul(a780, a781)?
                        };
                        let a784 = match v0.clone() {
                            Adt0::C0(_, _, _, _, _, _, _, _, h782, _) => {
                                h782
                            }
                        };
                        let a785 = 0u64;
                        nat_quot(a783, a784, a785)
                    };
                    let a787 = f82(v0)?;
                    nat_add(a786, a787)?
                }))
            }
        }
    }
}

pub fn f54(v0: u64) -> R<u64> {
    f74(f80(v0)?)
}

pub fn f55(v0: u64, v1: u64) -> R<u64> {
    let a798 = {
        let a796 = f75({
            let a788 = v0;
            let a789 = 2u64;
            let a790 = 0u64;
            nat_quot(a788, a789, a790)
        }, v1)?;
        let a797 = {
            let a794 = {
                let a791 = v0;
                let a792 = 2u64;
                let a793 = 0u64;
                nat_rem(a791, a792, a793)
            };
            let a795 = 71u64;
            nat_mul(a794, a795)?
        };
        nat_add(a796, a797)?
    };
    let a799 = 256u64;
    let a800 = 0u64;
    Ok(nat_rem(a798, a799, a800))
}

pub fn f56(v0: u64) -> R<u64> {
    Ok(f64(f18(v0)?))
}

pub fn f57(v0: u64) -> R<u64> {
    f74(f18(v0)?)
}

pub fn f58(v0: u64, v1: u64) -> R<u64> {
    let a803 = {
        let a801 = f57(v0)?;
        let a802 = f14(v0, v1, 0u64)?;
        nat_mul(a801, a802)?
    };
    let a804 = f83(v0, v1)?;
    nat_add(a803, a804)
}

pub fn f59(v0: u64, v1: u64) -> R<u64> {
    let a807 = {
        let a805 = f58(v0, v1)?;
        let a806 = f14(v0, v1, 1u64)?;
        nat_mul(a805, a806)?
    };
    let a808 = f83(v0, v1)?;
    nat_add(a807, a808)
}

pub fn f60(v0: u64, v1: u64) -> R<u64> {
    let a811 = {
        let a809 = f59(v0, v1)?;
        let a810 = f14(v0, v1, 2u64)?;
        nat_mul(a809, a810)?
    };
    let a812 = f83(v0, v1)?;
    nat_add(a811, a812)
}

pub fn f61(v0: u64) -> Adt2 {
    let a813 = v0;
    let a814 = 0u64;
    if nat_eq(a813, a814) {
        Adt2::C1(Adt3::C0(0u64, 1u64, 4u64))
    } else {
        let a815 = v0;
        let a816 = 1u64;
        if nat_eq(a815, a816) {
            Adt2::C1(Adt3::C0(1u64, 1u64, 2u64))
        } else {
            let a817 = v0;
            let a818 = 2u64;
            if nat_eq(a817, a818) {
                Adt2::C1(Adt3::C0(2u64, 32u64, 18u64))
            } else {
                let a819 = v0;
                let a820 = 3u64;
                if nat_eq(a819, a820) {
                    Adt2::C1(Adt3::C0(3u64, 32u64, 20u64))
                } else {
                    let a821 = v0;
                    let a822 = 6u64;
                    if nat_eq(a821, a822) {
                        Adt2::C1(Adt3::C0(6u64, 32u64, 22u64))
                    } else {
                        let a823 = v0;
                        let a824 = 7u64;
                        if nat_eq(a823, a824) {
                            Adt2::C1(Adt3::C0(7u64, 32u64, 24u64))
                        } else {
                            let a825 = v0;
                            let a826 = 8u64;
                            if nat_eq(a825, a826) {
                                Adt2::C1(Adt3::C0(8u64, 32u64, 34u64))
                            } else {
                                let a827 = v0;
                                let a828 = 9u64;
                                if nat_eq(a827, a828) {
                                    Adt2::C1(Adt3::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a829 = v0;
                                    let a830 = 10u64;
                                    if nat_eq(a829, a830) {
                                        Adt2::C1(Adt3::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a831 = v0;
                                        let a832 = 11u64;
                                        if nat_eq(a831, a832) {
                                            Adt2::C1(Adt3::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a833 = v0;
                                            let a834 = 12u64;
                                            if nat_eq(a833, a834) {
                                                Adt2::C1(Adt3::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a835 = v0;
                                                let a836 = 13u64;
                                                if nat_eq(a835, a836) {
                                                    Adt2::C1(Adt3::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a837 = v0;
                                                    let a838 = 14u64;
                                                    if nat_eq(a837, a838) {
                                                        Adt2::C1(Adt3::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a839 = v0;
                                                        let a840 = 15u64;
                                                        if nat_eq(a839, a840) {
                                                            Adt2::C1(Adt3::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a841 = v0;
                                                            let a842 = 16u64;
                                                            if nat_eq(a841, a842) {
                                                                Adt2::C1(Adt3::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a843 = v0;
                                                                let a844 = 17u64;
                                                                if nat_eq(a843, a844) {
                                                                    Adt2::C1(Adt3::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a845 = v0;
                                                                    let a846 = 18u64;
                                                                    if nat_eq(a845, a846) {
                                                                        Adt2::C1(Adt3::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a847 = v0;
                                                                        let a848 = 19u64;
                                                                        if nat_eq(a847, a848) {
                                                                            Adt2::C1(Adt3::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a849 = v0;
                                                                            let a850 = 20u64;
                                                                            if nat_eq(a849, a850) {
                                                                                Adt2::C1(Adt3::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a851 = v0;
                                                                                let a852 = 21u64;
                                                                                if nat_eq(a851, a852) {
                                                                                    Adt2::C1(Adt3::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a853 = v0;
                                                                                    let a854 = 22u64;
                                                                                    if nat_eq(a853, a854) {
                                                                                        Adt2::C1(Adt3::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a855 = v0;
                                                                                        let a856 = 23u64;
                                                                                        if nat_eq(a855, a856) {
                                                                                            Adt2::C1(Adt3::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a857 = v0;
                                                                                            let a858 = 24u64;
                                                                                            if nat_eq(a857, a858) {
                                                                                                Adt2::C1(Adt3::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a859 = v0;
                                                                                                let a860 = 25u64;
                                                                                                if nat_eq(a859, a860) {
                                                                                                    Adt2::C1(Adt3::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a861 = v0;
                                                                                                    let a862 = 26u64;
                                                                                                    if nat_eq(a861, a862) {
                                                                                                        Adt2::C1(Adt3::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a863 = v0;
                                                                                                        let a864 = 27u64;
                                                                                                        if nat_eq(a863, a864) {
                                                                                                            Adt2::C1(Adt3::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a865 = v0;
                                                                                                            let a866 = 28u64;
                                                                                                            if nat_eq(a865, a866) {
                                                                                                                Adt2::C1(Adt3::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a867 = v0;
                                                                                                                let a868 = 29u64;
                                                                                                                if nat_eq(a867, a868) {
                                                                                                                    Adt2::C1(Adt3::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a869 = v0;
                                                                                                                    let a870 = 30u64;
                                                                                                                    if nat_eq(a869, a870) {
                                                                                                                        Adt2::C1(Adt3::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a871 = v0;
                                                                                                                        let a872 = 34u64;
                                                                                                                        if nat_eq(a871, a872) {
                                                                                                                            Adt2::C1(Adt3::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a873 = v0;
                                                                                                                            let a874 = 35u64;
                                                                                                                            if nat_eq(a873, a874) {
                                                                                                                                Adt2::C1(Adt3::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a875 = v0;
                                                                                                                                let a876 = 39u64;
                                                                                                                                if nat_eq(a875, a876) {
                                                                                                                                    Adt2::C1(Adt3::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a877 = v0;
                                                                                                                                    let a878 = 40u64;
                                                                                                                                    if nat_eq(a877, a878) {
                                                                                                                                        Adt2::C1(Adt3::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a879 = v0;
                                                                                                                                        let a880 = 41u64;
                                                                                                                                        if nat_eq(a879, a880) {
                                                                                                                                            Adt2::C1(Adt3::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a881 = v0;
                                                                                                                                            let a882 = 42u64;
                                                                                                                                            if nat_eq(a881, a882) {
                                                                                                                                                Adt2::C1(Adt3::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a883 = v0;
                                                                                                                                                let a884 = 43u64;
                                                                                                                                                if nat_eq(a883, a884) {
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

pub fn f62(v0: u64) -> R<u64> {
    f77({
        let a885 = v0;
        let a886 = 96u64;
        let a887 = 0u64;
        nat_quot(a885, a886, a887)
    })
}

pub fn f63(v0: u64) -> u64 {
    let a891 = {
        let a888 = v0;
        let a889 = 32u64;
        let a890 = 0u64;
        nat_quot(a888, a889, a890)
    };
    let a892 = 3u64;
    let a893 = 0u64;
    nat_rem(a891, a892, a893)
}

pub fn f64(v0: u64) -> u64 {
    let a897 = {
        let a894 = v0;
        let a895 = 8u64;
        let a896 = 0u64;
        nat_quot(a894, a895, a896)
    };
    let a898 = 4u64;
    let a899 = 0u64;
    nat_rem(a897, a898, a899)
}

pub fn f65(v0: u64) -> R<u64> {
    let a908 = 1u64;
    let a909 = {
        let a906 = 2u64;
        let a907 = {
            let a903 = {
                let a900 = v0;
                let a901 = 4u64;
                let a902 = 0u64;
                nat_quot(a900, a901, a902)
            };
            let a904 = 2u64;
            let a905 = 0u64;
            nat_rem(a903, a904, a905)
        };
        nat_mul(a906, a907)?
    };
    nat_add(a908, a909)
}

pub fn f66(v0: u64) -> R<u64> {
    let a918 = 1u64;
    let a919 = {
        let a916 = 2u64;
        let a917 = {
            let a913 = {
                let a910 = v0;
                let a911 = 2u64;
                let a912 = 0u64;
                nat_quot(a910, a911, a912)
            };
            let a914 = 2u64;
            let a915 = 0u64;
            nat_rem(a913, a914, a915)
        };
        nat_mul(a916, a917)?
    };
    nat_add(a918, a919)
}

pub fn f67(v0: u64) -> R<u64> {
    let a923 = 1u64;
    let a924 = {
        let a920 = v0;
        let a921 = 2u64;
        let a922 = 0u64;
        nat_rem(a920, a921, a922)
    };
    nat_add(a923, a924)
}

pub fn f68(v0: u64) -> R<u64> {
    let a930 = {
        let a928 = {
            let a925 = v0;
            let a926 = 96u64;
            let a927 = 0u64;
            nat_quot(a925, a926, a927)
        };
        let a929 = 4u64;
        nat_mul(a928, a929)?
    };
    let a931 = f64(v0);
    nat_add(a930, a931)
}

pub fn f69(v0: u64) -> R<u64> {
    let m932 = f61(f62(v0)?);
    match m932 {
        Adt2::C0 => {
            Ok(0u64)
        }
        Adt2::C1(v1) => {
            match v1 {
                Adt3::C0(_, h933, _) => {
                    Ok(h933)
                }
            }
        }
    }
}

pub fn f70(v0: u64, v1: u64) -> R<u64> {
    let a938 = f74(v0)?;
    let a939 = {
        let a934 = v1;
        let a935 = 0u64;
        if if nat_eq(a934, a935) {
            let a936 = f64(v0);
            let a937 = 2u64;
            nat_eq(a936, a937)
        } else {
            false
        } {
            2u64
        } else {
            1u64
        }
    };
    nat_mul(a938, a939)
}

pub fn f71(v0: u64, v1: u64) -> R<u64> {
    let a945 = {
        let a943 = f70(v0, v1)?;
        let a944 = {
            let a940 = f21(v0, v1, 0u64)?;
            let a941 = f69(v0)?;
            let a942 = 0u64;
            nat_quot(a940, a941, a942)
        };
        nat_mul(a943, a944)?
    };
    let a946 = f84(v0, v1)?;
    nat_add(a945, a946)
}

pub fn f72(v0: u64, v1: u64) -> R<u64> {
    let a949 = {
        let a947 = f71(v0, v1)?;
        let a948 = f21(v0, v1, 1u64)?;
        nat_mul(a947, a948)?
    };
    let a950 = f84(v0, v1)?;
    nat_add(a949, a950)
}

pub fn f73(v0: u64, v1: u64) -> R<u64> {
    let a953 = {
        let a951 = f72(v0, v1)?;
        let a952 = f21(v0, v1, 2u64)?;
        nat_mul(a951, a952)?
    };
    let a954 = f84(v0, v1)?;
    nat_add(a953, a954)
}

pub fn f74(v0: u64) -> R<u64> {
    let m955 = f61(f62(v0)?);
    match m955 {
        Adt2::C0 => {
            Ok(0u64)
        }
        Adt2::C1(v1) => {
            match v1 {
                Adt3::C0(_, _, h956) => {
                    Ok(h956)
                }
            }
        }
    }
}

pub fn f75(v0: u64, v1: u64) -> R<u64> {
    let a971 = {
        let a969 = {
            let a964 = {
                let a957 = v1;
                let a958 = 37u64;
                nat_mul(a957, a958)?
            };
            let a965 = {
                let a962 = f77({
                    let a959 = v0;
                    let a960 = 4u64;
                    let a961 = 0u64;
                    nat_quot(a959, a960, a961)
                })?;
                let a963 = 13u64;
                nat_mul(a962, a963)?
            };
            nat_add(a964, a965)?
        };
        let a970 = {
            let a966 = v0;
            let a967 = 4u64;
            let a968 = 0u64;
            nat_rem(a966, a967, a968)
        };
        nat_add(a969, a970)?
    };
    let a972 = 256u64;
    let a973 = 0u64;
    Ok(nat_rem(a971, a972, a973))
}

pub fn f76(v0: u64) -> u64 {
    let a974 = v0;
    let a975 = 96u64;
    if nat_lt(a974, a975) {
        let a979 = {
            let a976 = v0;
            let a977 = 8u64;
            let a978 = 0u64;
            nat_quot(a976, a977, a978)
        };
        let a980 = 3u64;
        let a981 = 0u64;
        nat_rem(a979, a980, a981)
    } else {
        let a987 = {
            let a984 = {
                let a982 = v0;
                let a983 = 96u64;
                nat_sub(a982, a983)
            };
            let a985 = 2u64;
            let a986 = 0u64;
            nat_quot(a984, a985, a986)
        };
        let a988 = 3u64;
        let a989 = 0u64;
        nat_rem(a987, a988, a989)
    }
}

pub fn f77(v0: u64) -> R<u64> {
    let a990 = v0;
    let a991 = 4u64;
    if nat_lt(a990, a991) {
        Ok(v0)
    } else {
        let a992 = v0;
        let a993 = 29u64;
        if nat_lt(a992, a993) {
            let a994 = v0;
            let a995 = 2u64;
            nat_add(a994, a995)
        } else {
            let a996 = v0;
            let a997 = 31u64;
            if nat_lt(a996, a997) {
                let a998 = v0;
                let a999 = 5u64;
                nat_add(a998, a999)
            } else {
                let a1000 = v0;
                let a1001 = 8u64;
                nat_add(a1000, a1001)
            }
        }
    }
}

pub fn f78(v0: u64) -> u64 {
    let a1002 = v0;
    let a1003 = 288u64;
    let a1004 = 0u64;
    nat_quot(a1002, a1003, a1004)
}

pub fn f79(v0: u64) -> u64 {
    let a1005 = v0;
    let a1006 = 0u64;
    if nat_eq(a1005, a1006) {
        0u64
    } else {
        let a1007 = v0;
        let a1008 = 1u64;
        if nat_eq(a1007, a1008) {
            1u64
        } else {
            3u64
        }
    }
}

pub fn f80(v0: u64) -> R<u64> {
    let a1009 = f78(v0);
    let a1010 = 96u64;
    nat_mul(a1009, a1010)
}

pub fn f81(v0: u64, v1: u64) -> R<u64> {
    let a1011 = v1;
    let a1012 = 1u64;
    if if if nat_eq(a1011, a1012) {
        true
    } else {
        let a1013 = v1;
        let a1014 = 3u64;
        nat_eq(a1013, a1014)
    } {
        true
    } else {
        let a1015 = f40(v0);
        let a1016 = 1u64;
        if nat_eq(a1015, a1016) {
            true
        } else {
            let a1017 = f40(v0);
            let a1018 = 2u64;
            nat_eq(a1017, a1018)
        }
    } {
        f54(v0)
    } else {
        Ok(0u64)
    }
}

pub fn f82(v0: Adt0) -> R<u64> {
    let a1039 = {
        let a1031 = {
            let a1023 = {
                let a1020 = match v0.clone() {
                    Adt0::C0(_, h1019, _, _, _, _, _, _, _, _) => {
                        h1019
                    }
                };
                let a1021 = 1u64;
                nat_sub(a1020, a1021)
            };
            let a1024 = match v0.clone() {
                Adt0::C0(_, _, _, _, _, h1022, _, _, _, _) => {
                    h1022
                }
            };
            nat_mul(a1023, a1024)?
        };
        let a1032 = {
            let a1029 = {
                let a1026 = match v0.clone() {
                    Adt0::C0(_, _, h1025, _, _, _, _, _, _, _) => {
                        h1025
                    }
                };
                let a1027 = 1u64;
                nat_sub(a1026, a1027)
            };
            let a1030 = match v0.clone() {
                Adt0::C0(_, _, _, _, _, _, h1028, _, _, _) => {
                    h1028
                }
            };
            nat_mul(a1029, a1030)?
        };
        nat_add(a1031, a1032)?
    };
    let a1040 = {
        let a1037 = {
            let a1034 = match v0.clone() {
                Adt0::C0(_, _, _, h1033, _, _, _, _, _, _) => {
                    h1033
                }
            };
            let a1035 = 1u64;
            nat_sub(a1034, a1035)
        };
        let a1038 = match v0 {
            Adt0::C0(_, _, _, _, _, _, _, h1036, _, _) => {
                h1036
            }
        };
        nat_mul(a1037, a1038)?
    };
    nat_add(a1039, a1040)
}

pub fn f83(v0: u64, v1: u64) -> R<u64> {
    let a1041 = v1;
    let a1042 = 3u64;
    if nat_eq(a1041, a1042) {
        f57(v0)
    } else {
        Ok(0u64)
    }
}

pub fn f84(v0: u64, v1: u64) -> R<u64> {
    let a1043 = v1;
    let a1044 = 1u64;
    if if nat_eq(a1043, a1044) {
        true
    } else {
        let a1045 = f64(v0);
        let a1046 = 1u64;
        if nat_eq(a1045, a1046) {
            true
        } else {
            let a1047 = f64(v0);
            let a1048 = 2u64;
            nat_eq(a1047, a1048)
        }
    } {
        f74(v0)
    } else {
        Ok(0u64)
    }
}
