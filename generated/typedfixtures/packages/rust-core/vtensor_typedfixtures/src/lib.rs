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
        let a6 = 977u64;
        if nat_lt(a5, a6) {
            let a7 = v0;
            let a8 = 1u64;
            if nat_eq(a7, a8) {
                let a9 = v2;
                let a10 = 0u64;
                if if nat_eq(a9, a10) {
                    let a11 = v3;
                    let a12 = 13u64;
                    nat_lt(a11, a12)
                } else {
                    false
                } {
                    Ok(Some(f2(v1, v3)?))
                } else {
                    Ok(None::<u64>)
                }
            } else {
                let a13 = v0;
                let a14 = 2u64;
                if nat_eq(a13, a14) {
                    let a15 = v2;
                    let a16 = 0u64;
                    if if if if nat_eq(a15, a16) {
                        true
                    } else {
                        let a17 = v2;
                        let a18 = 1u64;
                        nat_eq(a17, a18)
                    } {
                        true
                    } else {
                        let a19 = v2;
                        let a20 = 3u64;
                        nat_eq(a19, a20)
                    } {
                        let a21 = v3;
                        let a22 = 10u64;
                        nat_lt(a21, a22)
                    } else {
                        false
                    } {
                        Ok(Some({
                            let a23 = v3;
                            let a24 = 0u64;
                            if nat_eq(a23, a24) {
                                f3(v1, v2, 0u64)
                            } else {
                                let a25 = v3;
                                let a26 = 1u64;
                                if nat_eq(a25, a26) {
                                    f3(v1, v2, 1u64)
                                } else {
                                    let a27 = v3;
                                    let a28 = 2u64;
                                    if nat_eq(a27, a28) {
                                        f3(v1, v2, 2u64)
                                    } else {
                                        let a29 = v3;
                                        let a30 = 3u64;
                                        if nat_eq(a29, a30) {
                                            f3(v1, v2, 3u64)
                                        } else {
                                            let a31 = v3;
                                            let a32 = 4u64;
                                            if nat_eq(a31, a32) {
                                                f4(v1, v2, 0u64)?
                                            } else {
                                                let a33 = v3;
                                                let a34 = 5u64;
                                                if nat_eq(a33, a34) {
                                                    f4(v1, v2, 1u64)?
                                                } else {
                                                    let a35 = v3;
                                                    let a36 = 6u64;
                                                    if nat_eq(a35, a36) {
                                                        f4(v1, v2, 2u64)?
                                                    } else {
                                                        let a37 = v3;
                                                        let a38 = 7u64;
                                                        if nat_eq(a37, a38) {
                                                            f4(v1, v2, 3u64)?
                                                        } else {
                                                            let a39 = v3;
                                                            let a40 = 8u64;
                                                            if nat_eq(a39, a40) {
                                                                f5(v1, v2)?
                                                            } else {
                                                                let a41 = v3;
                                                                let a42 = 9u64;
                                                                let _ = nat_eq(a41, a42);
                                                                f6(v1, v2)?
                                                            }
                                                        }
                                                    }
                                                }
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
                    let a43 = v0;
                    let a44 = 3u64;
                    if nat_eq(a43, a44) {
                        let a45 = v2;
                        let a46 = 2u64;
                        if if nat_lt(a45, a46) {
                            let a47 = v3;
                            let a48 = 512u64;
                            nat_lt(a47, a48)
                        } else {
                            false
                        } {
                            Ok(Some(f7(v1, v2, v3)?))
                        } else {
                            Ok(None::<u64>)
                        }
                    } else {
                        let a49 = v0;
                        let a50 = 4u64;
                        if nat_eq(a49, a50) {
                            let a51 = v3;
                            let a52 = 2u64;
                            if nat_lt(a51, a52) {
                                let a53 = v2;
                                let a54 = 0u64;
                                if nat_eq(a53, a54) {
                                    let a55 = v3;
                                    let a56 = 0u64;
                                    if nat_eq(a55, a56) {
                                        Ok(Some(f8(v1, 0u64)?))
                                    } else {
                                        Ok(None::<u64>)
                                    }
                                } else {
                                    let a57 = v2;
                                    let a58 = f8(v1, 0u64)?;
                                    if nat_le(a57, a58) {
                                        Ok(Some({
                                            let a59 = v3;
                                            let a60 = 0u64;
                                            if nat_eq(a59, a60) {
                                                let a63 = {
                                                    let a61 = v2;
                                                    let a62 = 1u64;
                                                    nat_sub(a61, a62)
                                                };
                                                let a64 = 512u64;
                                                nat_mul(a63, a64)?
                                            } else {
                                                let v4: u64 = {
                                                    let a69 = f5(v1, 0u64)?;
                                                    let a70 = {
                                                        let a67 = {
                                                            let a65 = v2;
                                                            let a66 = 1u64;
                                                            nat_sub(a65, a66)
                                                        };
                                                        let a68 = 512u64;
                                                        nat_mul(a67, a68)?
                                                    };
                                                    nat_sub(a69, a70)
                                                };
                                                let a71 = v4;
                                                let a72 = 512u64;
                                                if nat_lt(a71, a72) {
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
                            let a73 = v0;
                            let a74 = 5u64;
                            if nat_eq(a73, a74) {
                                let a75 = v3;
                                let a76 = 2u64;
                                if nat_lt(a75, a76) {
                                    let a77 = v2;
                                    let a78 = 0u64;
                                    if nat_eq(a77, a78) {
                                        let a79 = v3;
                                        let a80 = 0u64;
                                        if nat_eq(a79, a80) {
                                            Ok(Some(f8(v1, 1u64)?))
                                        } else {
                                            Ok(None::<u64>)
                                        }
                                    } else {
                                        let a81 = v2;
                                        let a82 = f8(v1, 1u64)?;
                                        if nat_le(a81, a82) {
                                            Ok(Some({
                                                let a83 = v3;
                                                let a84 = 0u64;
                                                if nat_eq(a83, a84) {
                                                    let a87 = {
                                                        let a85 = v2;
                                                        let a86 = 1u64;
                                                        nat_sub(a85, a86)
                                                    };
                                                    let a88 = 512u64;
                                                    nat_mul(a87, a88)?
                                                } else {
                                                    let v5: u64 = {
                                                        let a93 = f5(v1, 1u64)?;
                                                        let a94 = {
                                                            let a91 = {
                                                                let a89 = v2;
                                                                let a90 = 1u64;
                                                                nat_sub(a89, a90)
                                                            };
                                                            let a92 = 512u64;
                                                            nat_mul(a91, a92)?
                                                        };
                                                        nat_sub(a93, a94)
                                                    };
                                                    let a95 = v5;
                                                    let a96 = 512u64;
                                                    if nat_lt(a95, a96) {
                                                        v5
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
            }
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f1(v0: u64) -> u64 {
    let a97 = v0;
    let a98 = 0u64;
    if nat_eq(a97, a98) {
        977u64
    } else {
        let a99 = v0;
        let a100 = 1u64;
        if nat_eq(a99, a100) {
            13u64
        } else {
            let a101 = v0;
            let a102 = 2u64;
            if nat_eq(a101, a102) {
                1954u64
            } else {
                let a103 = v0;
                let a104 = 3u64;
                if nat_eq(a103, a104) {
                    512u64
                } else {
                    let a105 = v0;
                    let a106 = 4u64;
                    if nat_eq(a105, a106) {
                        165u64
                    } else {
                        240u64
                    }
                }
            }
        }
    }
}

pub fn f2(v0: u64, v1: u64) -> R<u64> {
    let a107 = v1;
    let a108 = 0u64;
    if nat_eq(a107, a108) {
        Ok(f9(v0, 0u64))
    } else {
        let a109 = v1;
        let a110 = 1u64;
        if nat_eq(a109, a110) {
            Ok(f10(v0))
        } else {
            let a111 = v1;
            let a112 = 2u64;
            if nat_eq(a111, a112) {
                Ok(f11(v0))
            } else {
                let a113 = v1;
                let a114 = 3u64;
                if nat_eq(a113, a114) {
                    Ok(f11(v0))
                } else {
                    let a115 = v1;
                    let a116 = 4u64;
                    if nat_eq(a115, a116) {
                        Ok(f3(v0, 0u64, 0u64))
                    } else {
                        let a117 = v1;
                        let a118 = 5u64;
                        if nat_eq(a117, a118) {
                            Ok(f3(v0, 0u64, 1u64))
                        } else {
                            let a119 = v1;
                            let a120 = 6u64;
                            if nat_eq(a119, a120) {
                                Ok(f3(v0, 0u64, 3u64))
                            } else {
                                let a121 = v1;
                                let a122 = 7u64;
                                if nat_eq(a121, a122) {
                                    Ok(v0)
                                } else {
                                    let a123 = v1;
                                    let a124 = 8u64;
                                    if nat_eq(a123, a124) {
                                        Ok(f12(v0))
                                    } else {
                                        let a125 = v1;
                                        let a126 = 9u64;
                                        if nat_eq(a125, a126) {
                                            Ok(f13(v0))
                                        } else {
                                            let a127 = v1;
                                            let a128 = 10u64;
                                            if nat_eq(a127, a128) {
                                                let a129 = 977u64;
                                                let a130 = v0;
                                                nat_add(a129, a130)
                                            } else {
                                                let a131 = v1;
                                                let a132 = 11u64;
                                                if nat_eq(a131, a132) {
                                                    Ok(f9(v0, 1u64))
                                                } else {
                                                    Ok(f9(v0, 3u64))
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

pub fn f3(v0: u64, v1: u64, v2: u64) -> u64 {
    let v3: u64 = f14(v0, v1);
    if if f15(v0) {
        let a138 = v2;
        let a139 = {
            let a135 = {
                let a133 = v0;
                let a134 = 864u64;
                nat_sub(a133, a134)
            };
            let a136 = 4u64;
            let a137 = 0u64;
            nat_rem(a135, a136, a137)
        };
        nat_eq(a138, a139)
    } else {
        false
    } {
        0u64
    } else {
        if f16(v0) {
            let a140 = v3;
            let a141 = 1u64;
            if nat_eq(a140, a141) {
                1u64
            } else {
                let a142 = v2;
                let a143 = 0u64;
                if nat_eq(a142, a143) {
                    3u64
                } else {
                    let a144 = v2;
                    let a145 = 1u64;
                    if nat_eq(a144, a145) {
                        1u64
                    } else {
                        let a146 = v2;
                        let a147 = 2u64;
                        if nat_eq(a146, a147) {
                            1u64
                        } else {
                            65537u64
                        }
                    }
                }
            }
        } else {
            let a148 = v3;
            let a149 = 1u64;
            if if nat_eq(a148, a149) {
                let a150 = f11(v0);
                let a151 = 2u64;
                nat_eq(a150, a151)
            } else {
                false
            } {
                let a152 = v2;
                let a153 = 0u64;
                if nat_eq(a152, a153) {
                    2u64
                } else {
                    1u64
                }
            } else {
                let a154 = v3;
                let a155 = 1u64;
                if if nat_eq(a154, a155) {
                    let a156 = f11(v0);
                    let a157 = 4u64;
                    nat_eq(a156, a157)
                } else {
                    false
                } {
                    1u64
                } else {
                    let a158 = v3;
                    let a159 = 1u64;
                    if if if nat_eq(a158, a159) {
                        let a160 = f11(v0);
                        let a161 = 3u64;
                        nat_eq(a160, a161)
                    } else {
                        false
                    } {
                        let a162 = v2;
                        let a163 = 0u64;
                        nat_eq(a162, a163)
                    } else {
                        false
                    } {
                        1u64
                    } else {
                        let a164 = v2;
                        let a165 = 0u64;
                        if nat_eq(a164, a165) {
                            let a166 = f11(v0);
                            let a167 = 2u64;
                            if nat_eq(a166, a167) {
                                6u64
                            } else {
                                3u64
                            }
                        } else {
                            let a168 = v2;
                            let a169 = 0u64;
                            if nat_eq(a168, a169) {
                                3u64
                            } else {
                                let a170 = v2;
                                let a171 = 1u64;
                                if nat_eq(a170, a171) {
                                    3u64
                                } else {
                                    let a172 = v2;
                                    let a173 = 2u64;
                                    let _ = nat_eq(a172, a173);
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

pub fn f4(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a174 = v2;
    let a175 = 0u64;
    if nat_eq(a174, a175) {
        f17(v0, v1)
    } else {
        let a176 = v2;
        let a177 = 1u64;
        if nat_eq(a176, a177) {
            f18(v0, v1)
        } else {
            let a178 = v2;
            let a179 = 2u64;
            if nat_eq(a178, a179) {
                f19(v0, v1)
            } else {
                f20(v0, v1)
            }
        }
    }
}

pub fn f5(v0: u64, v1: u64) -> R<u64> {
    let m180 = f21(Adt0::C0(f3(v0, v1, 0u64), f3(v0, v1, 1u64), f3(v0, v1, 2u64), f3(v0, v1, 3u64), f4(v0, v1, 0u64)?, f4(v0, v1, 1u64)?, f4(v0, v1, 2u64)?, f4(v0, v1, 3u64)?, 1u64, f22(v0, v1)))?;
    match m180 {
        Adt1::C0 => {
            Ok(0u64)
        }
        Adt1::C1(v2) => {
            Ok(v2)
        }
    }
}

pub fn f6(v0: u64, v1: u64) -> R<u64> {
    let a185 = {
        let a181 = f3(v0, v1, 0u64);
        let a182 = f3(v0, v1, 1u64);
        nat_mul(a181, a182)?
    };
    let a186 = {
        let a183 = f3(v0, v1, 2u64);
        let a184 = f3(v0, v1, 3u64);
        nat_mul(a183, a184)?
    };
    nat_mul(a185, a186)
}

pub fn f7(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = f22(v0, v1);
    let v4: u64 = f23(v0, v1, {
        let a187 = v2;
        let a188 = v3;
        let a189 = 0u64;
        nat_quot(a187, a188, a189)
    })?;
    let v5: u64 = {
        let a193 = {
            let a190 = v2;
            let a191 = v3;
            let a192 = 0u64;
            nat_rem(a190, a191, a192)
        };
        let a194 = 0u64;
        if nat_eq(a193, a194) {
            1u64
        } else {
            let a198 = {
                let a195 = v2;
                let a196 = v3;
                let a197 = 0u64;
                nat_rem(a195, a196, a197)
            };
            let a199 = 1u64;
            if nat_eq(a198, a199) {
                256u64
            } else {
                let a203 = {
                    let a200 = v2;
                    let a201 = v3;
                    let a202 = 0u64;
                    nat_rem(a200, a201, a202)
                };
                let a204 = 2u64;
                if nat_eq(a203, a204) {
                    65536u64
                } else {
                    16777216u64
                }
            }
        }
    };
    let a208 = {
        let a205 = v4;
        let a206 = v5;
        let a207 = 0u64;
        nat_quot(a205, a206, a207)
    };
    let a209 = 256u64;
    let a210 = 0u64;
    Ok(nat_rem(a208, a209, a210))
}

pub fn f8(v0: u64, v1: u64) -> R<u64> {
    let a213 = {
        let a211 = f5(v0, v1)?;
        let a212 = 511u64;
        nat_add(a211, a212)?
    };
    let a214 = 512u64;
    let a215 = 0u64;
    Ok(nat_quot(a213, a214, a215))
}

pub fn f9(v0: u64, v1: u64) -> u64 {
    let v2: u64 = f24(v0);
    let a216 = v1;
    let a217 = 1u64;
    if nat_eq(a216, a217) {
        let a218 = v2;
        let a219 = 0u64;
        if nat_eq(a218, a219) {
            0u64
        } else {
            let a220 = v2;
            let a221 = 1u64;
            if nat_eq(a220, a221) {
                1u64
            } else {
                let a222 = v2;
                let a223 = 2u64;
                if nat_eq(a222, a223) {
                    30u64
                } else {
                    let a224 = v2;
                    let a225 = 3u64;
                    if nat_eq(a224, a225) {
                        0u64
                    } else {
                        let a226 = v2;
                        let a227 = 4u64;
                        if nat_eq(a226, a227) {
                            0u64
                        } else {
                            let a228 = v2;
                            let a229 = 5u64;
                            let _ = nat_eq(a228, a229);
                            0u64
                        }
                    }
                }
            }
        }
    } else {
        let a230 = v1;
        let a231 = 3u64;
        if if nat_eq(a230, a231) {
            let a232 = v2;
            let a233 = 4u64;
            if nat_eq(a232, a233) {
                true
            } else {
                let a234 = v2;
                let a235 = 6u64;
                nat_eq(a234, a235)
            }
        } else {
            false
        } {
            0u64
        } else {
            let a236 = v2;
            let a237 = 0u64;
            if nat_eq(a236, a237) {
                0u64
            } else {
                let a238 = v2;
                let a239 = 1u64;
                if nat_eq(a238, a239) {
                    1u64
                } else {
                    let a240 = v2;
                    let a241 = 2u64;
                    if nat_eq(a240, a241) {
                        30u64
                    } else {
                        let a242 = v2;
                        let a243 = 3u64;
                        if nat_eq(a242, a243) {
                            30u64
                        } else {
                            let a244 = v2;
                            let a245 = 4u64;
                            if nat_eq(a244, a245) {
                                30u64
                            } else {
                                let a246 = v2;
                                let a247 = 5u64;
                                let _ = nat_eq(a246, a247);
                                1u64
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f10(v0: u64) -> u64 {
    let a248 = f25(v0);
    let a249 = 2u64;
    if if if nat_eq(a248, a249) {
        let a250 = 2u64;
        let a251 = f11(v0);
        nat_le(a250, a251)
    } else {
        false
    } {
        let a252 = f11(v0);
        let a253 = 4u64;
        nat_le(a252, a253)
    } else {
        false
    } {
        0u64
    } else {
        f25(v0)
    }
}

pub fn f11(v0: u64) -> u64 {
    let a254 = v0;
    let a255 = 864u64;
    if nat_lt(a254, a255) {
        let a256 = v0;
        let a257 = 12u64;
        let a258 = 0u64;
        nat_rem(a256, a257, a258)
    } else {
        12u64
    }
}

pub fn f12(v0: u64) -> u64 {
    let a259 = 864u64;
    let a260 = v0;
    if nat_le(a259, a260) {
        0u64
    } else {
        let a261 = f11(v0);
        let a262 = 9u64;
        if if nat_eq(a261, a262) {
            true
        } else {
            let a263 = f11(v0);
            let a264 = 11u64;
            nat_eq(a263, a264)
        } {
            2u64
        } else {
            let a265 = f11(v0);
            let a266 = 8u64;
            if if nat_eq(a265, a266) {
                true
            } else {
                let a267 = f11(v0);
                let a268 = 10u64;
                if nat_eq(a267, a268) {
                    let a269 = f10(v0);
                    let a270 = 2u64;
                    if nat_eq(a269, a270) {
                        true
                    } else {
                        let a271 = f10(v0);
                        let a272 = 3u64;
                        nat_eq(a271, a272)
                    }
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
}

pub fn f13(v0: u64) -> u64 {
    let a273 = f26(v0);
    let a274 = 0u64;
    if nat_eq(a273, a274) {
        2u64
    } else {
        let a275 = f26(v0);
        let a276 = 1u64;
        if nat_eq(a275, a276) {
            5u64
        } else {
            let a277 = f26(v0);
            let a278 = 2u64;
            if nat_eq(a277, a278) {
                6u64
            } else {
                7u64
            }
        }
    }
}

pub fn f14(v0: u64, v1: u64) -> u64 {
    let a279 = v1;
    let a280 = 3u64;
    if nat_eq(a279, a280) {
        let a281 = f10(v0);
        let a282 = 1u64;
        if if nat_eq(a281, a282) {
            true
        } else {
            let a283 = f10(v0);
            let a284 = 3u64;
            nat_eq(a283, a284)
        } {
            0u64
        } else {
            let a285 = f10(v0);
            let a286 = 2u64;
            if nat_eq(a285, a286) {
                1u64
            } else {
                3u64
            }
        }
    } else {
        let a287 = v1;
        let a288 = 1u64;
        if if nat_eq(a287, a288) {
            let a289 = f10(v0);
            let a290 = 3u64;
            nat_eq(a289, a290)
        } else {
            false
        } {
            0u64
        } else {
            v1
        }
    }
}

pub fn f15(v0: u64) -> bool {
    let a291 = 864u64;
    let a292 = v0;
    if nat_le(a291, a292) {
        let a293 = v0;
        let a294 = 976u64;
        nat_lt(a293, a294)
    } else {
        false
    }
}

pub fn f16(v0: u64) -> bool {
    let a295 = v0;
    let a296 = 976u64;
    nat_eq(a295, a296)
}

pub fn f17(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f14(v0, v1);
    let v3: u64 = f22(v0, v1);
    let a297 = f11(v0);
    let a298 = 3u64;
    if if nat_eq(a297, a298) {
        let a299 = v2;
        let a300 = 1u64;
        nat_eq(a299, a300)
    } else {
        false
    } {
        let a303 = {
            let a301 = v3;
            let a302 = 7u64;
            nat_mul(a301, a302)?
        };
        let a304 = 1u64;
        nat_add(a303, a304)
    } else {
        let a305 = f11(v0);
        let a306 = 4u64;
        if if nat_eq(a305, a306) {
            let a307 = v2;
            let a308 = 1u64;
            nat_eq(a307, a308)
        } else {
            false
        } {
            Ok(1u64)
        } else {
            let a309 = f11(v0);
            let a310 = 2u64;
            if if if if nat_eq(a309, a310) {
                let a311 = v2;
                let a312 = 1u64;
                nat_eq(a311, a312)
            } else {
                false
            } {
                true
            } else {
                let a313 = f11(v0);
                let a314 = 9u64;
                if nat_eq(a313, a314) {
                    let a315 = v2;
                    let a316 = 0u64;
                    nat_eq(a315, a316)
                } else {
                    false
                }
            } {
                true
            } else {
                let a317 = f11(v0);
                let a318 = 11u64;
                if nat_eq(a317, a318) {
                    let a319 = v2;
                    let a320 = f14(v0, 3u64);
                    nat_eq(a319, a320)
                } else {
                    false
                }
            } {
                let a321 = v3;
                let a322 = 2u64;
                nat_mul(a321, a322)
            } else {
                Ok(v3)
            }
        }
    }
}

pub fn f18(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f14(v0, v1);
    let v3: u64 = f22(v0, v1);
    let a323 = f11(v0);
    let a324 = 8u64;
    if if if nat_eq(a323, a324) {
        let a325 = v2;
        let a326 = f14(v0, 3u64);
        nat_eq(a325, a326)
    } else {
        false
    } {
        true
    } else {
        let a327 = f11(v0);
        let a328 = 10u64;
        if nat_eq(a327, a328) {
            let a329 = v2;
            let a330 = 1u64;
            if nat_eq(a329, a330) {
                true
            } else {
                let a331 = f10(v0);
                let a332 = 3u64;
                nat_eq(a331, a332)
            }
        } else {
            false
        }
    } {
        Ok(0u64)
    } else {
        let a333 = f11(v0);
        let a334 = 7u64;
        if if nat_eq(a333, a334) {
            let a335 = v2;
            let a336 = f14(v0, 3u64);
            nat_eq(a335, a336)
        } else {
            false
        } {
            let a337 = 18u64;
            let a338 = v3;
            nat_mul(a337, a338)
        } else {
            let a339 = f11(v0);
            let a340 = 5u64;
            if if if nat_eq(a339, a340) {
                let a341 = v2;
                let a342 = 0u64;
                nat_eq(a341, a342)
            } else {
                false
            } {
                true
            } else {
                let a343 = f11(v0);
                let a344 = 6u64;
                if nat_eq(a343, a344) {
                    let a345 = v2;
                    let a346 = f14(v0, 3u64);
                    nat_eq(a345, a346)
                } else {
                    false
                }
            } {
                let a347 = 6u64;
                let a348 = v3;
                nat_mul(a347, a348)
            } else {
                let a353 = {
                    let a351 = f3(v0, v1, 0u64);
                    let a352 = {
                        let a349 = f11(v0);
                        let a350 = 1u64;
                        if nat_eq(a349, a350) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a351, a352)?
                };
                let a354 = v3;
                nat_mul(a353, a354)
            }
        }
    }
}

pub fn f19(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f14(v0, v1);
    let v3: u64 = f22(v0, v1);
    let a355 = f11(v0);
    let a356 = 7u64;
    if if nat_eq(a355, a356) {
        let a357 = v2;
        let a358 = f14(v0, 3u64);
        nat_eq(a357, a358)
    } else {
        false
    } {
        let a359 = 26u64;
        let a360 = v3;
        nat_mul(a359, a360)
    } else {
        let a361 = f11(v0);
        let a362 = 5u64;
        if if if nat_eq(a361, a362) {
            let a363 = v2;
            let a364 = 0u64;
            nat_eq(a363, a364)
        } else {
            false
        } {
            true
        } else {
            let a365 = f11(v0);
            let a366 = 6u64;
            if nat_eq(a365, a366) {
                let a367 = v2;
                let a368 = f14(v0, 3u64);
                nat_eq(a367, a368)
            } else {
                false
            }
        } {
            let a369 = 3u64;
            let a370 = v3;
            nat_mul(a369, a370)
        } else {
            let a377 = {
                let a371 = f18(v0, v1)?;
                let a372 = f3(v0, v1, 1u64);
                nat_mul(a371, a372)?
            };
            let a378 = {
                let a373 = f11(v0);
                let a374 = 1u64;
                if nat_eq(a373, a374) {
                    let a375 = 3u64;
                    let a376 = v3;
                    nat_mul(a375, a376)?
                } else {
                    0u64
                }
            };
            nat_add(a377, a378)
        }
    }
}

pub fn f20(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f14(v0, v1);
    let v3: u64 = f22(v0, v1);
    let a379 = f11(v0);
    let a380 = 7u64;
    if if nat_eq(a379, a380) {
        let a381 = v2;
        let a382 = f14(v0, 3u64);
        nat_eq(a381, a382)
    } else {
        false
    } {
        let a383 = 80u64;
        let a384 = v3;
        nat_mul(a383, a384)
    } else {
        let a385 = f11(v0);
        let a386 = 5u64;
        if if if nat_eq(a385, a386) {
            let a387 = v2;
            let a388 = 0u64;
            nat_eq(a387, a388)
        } else {
            false
        } {
            true
        } else {
            let a389 = f11(v0);
            let a390 = 6u64;
            if nat_eq(a389, a390) {
                let a391 = v2;
                let a392 = f14(v0, 3u64);
                nat_eq(a391, a392)
            } else {
                false
            }
        } {
            let a393 = 18u64;
            let a394 = v3;
            nat_mul(a393, a394)
        } else {
            let a401 = {
                let a395 = f19(v0, v1)?;
                let a396 = f3(v0, v1, 2u64);
                nat_mul(a395, a396)?
            };
            let a402 = {
                let a397 = f11(v0);
                let a398 = 1u64;
                if nat_eq(a397, a398) {
                    let a399 = 5u64;
                    let a400 = v3;
                    nat_mul(a399, a400)?
                } else {
                    0u64
                }
            };
            nat_add(a401, a402)
        }
    }
}

pub fn f21(v0: Adt0) -> R<Adt1> {
    let a404 = match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h403, _) => {
            h403
        }
    };
    let a405 = 0u64;
    if nat_eq(a404, a405) {
        Ok(Adt1::C0)
    } else {
        let a407 = match v0.clone() {
            Adt0::C0(h406, _, _, _, _, _, _, _, _, _) => {
                h406
            }
        };
        let a408 = 0u64;
        if if if if nat_eq(a407, a408) {
            true
        } else {
            let a410 = match v0.clone() {
                Adt0::C0(_, h409, _, _, _, _, _, _, _, _) => {
                    h409
                }
            };
            let a411 = 0u64;
            nat_eq(a410, a411)
        } {
            true
        } else {
            let a413 = match v0.clone() {
                Adt0::C0(_, _, h412, _, _, _, _, _, _, _) => {
                    h412
                }
            };
            let a414 = 0u64;
            nat_eq(a413, a414)
        } {
            true
        } else {
            let a416 = match v0.clone() {
                Adt0::C0(_, _, _, h415, _, _, _, _, _, _) => {
                    h415
                }
            };
            let a417 = 0u64;
            nat_eq(a416, a417)
        } {
            Ok(Adt1::C1(0u64))
        } else {
            let a419 = match v0.clone() {
                Adt0::C0(_, _, _, _, _, _, _, _, h418, _) => {
                    h418
                }
            };
            let a420 = 1u64;
            if nat_eq(a419, a420) {
                Ok(Adt1::C1({
                    let a430 = {
                        let a428 = match v0.clone() {
                            Adt0::C0(_, _, _, _, _, _, _, _, _, h421) => {
                                h421
                            }
                        };
                        let a429 = {
                            let a426 = {
                                let a423 = match v0.clone() {
                                    Adt0::C0(h422, _, _, _, _, _, _, _, _, _) => {
                                        h422
                                    }
                                };
                                let a424 = 1u64;
                                nat_sub(a423, a424)
                            };
                            let a427 = match v0.clone() {
                                Adt0::C0(_, _, _, _, h425, _, _, _, _, _) => {
                                    h425
                                }
                            };
                            nat_mul(a426, a427)?
                        };
                        nat_add(a428, a429)?
                    };
                    let a431 = f27(v0)?;
                    nat_add(a430, a431)?
                }))
            } else {
                Ok(Adt1::C1({
                    let a440 = {
                        let a437 = {
                            let a434 = match v0.clone() {
                                Adt0::C0(h432, _, _, _, _, _, _, _, _, _) => {
                                    h432
                                }
                            };
                            let a435 = match v0.clone() {
                                Adt0::C0(_, _, _, _, h433, _, _, _, _, _) => {
                                    h433
                                }
                            };
                            nat_mul(a434, a435)?
                        };
                        let a438 = match v0.clone() {
                            Adt0::C0(_, _, _, _, _, _, _, _, h436, _) => {
                                h436
                            }
                        };
                        let a439 = 0u64;
                        nat_quot(a437, a438, a439)
                    };
                    let a441 = f27(v0)?;
                    nat_add(a440, a441)?
                }))
            }
        }
    }
}

pub fn f22(v0: u64, v1: u64) -> u64 {
    let a442 = f9(v0, v1);
    let a443 = 0u64;
    if nat_eq(a442, a443) {
        4u64
    } else {
        2u64
    }
}

pub fn f23(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = f9(v0, v1);
    let v4: u64 = {
        let a448 = {
            let a446 = v2;
            let a447 = {
                let a444 = f14(v0, v1);
                let a445 = 5u64;
                nat_mul(a444, a445)?
            };
            nat_add(a446, a447)?
        };
        let a449 = 32u64;
        let a450 = 0u64;
        nat_rem(a448, a449, a450)
    };
    let a451 = v3;
    let a452 = 0u64;
    if nat_eq(a451, a452) {
        let a453 = v4;
        let a454 = 0u64;
        if nat_eq(a453, a454) {
            Ok(0u64)
        } else {
            let a455 = v4;
            let a456 = 1u64;
            if nat_eq(a455, a456) {
                Ok(2147483648u64)
            } else {
                let a457 = v4;
                let a458 = 2u64;
                if nat_eq(a457, a458) {
                    Ok(1065353216u64)
                } else {
                    let a459 = v4;
                    let a460 = 3u64;
                    if nat_eq(a459, a460) {
                        Ok(3212836864u64)
                    } else {
                        let a461 = v4;
                        let a462 = 4u64;
                        if nat_eq(a461, a462) {
                            Ok(1u64)
                        } else {
                            let a463 = v4;
                            let a464 = 5u64;
                            if nat_eq(a463, a464) {
                                Ok(2147483649u64)
                            } else {
                                let a465 = v4;
                                let a466 = 6u64;
                                if nat_eq(a465, a466) {
                                    Ok(8388607u64)
                                } else {
                                    let a467 = v4;
                                    let a468 = 7u64;
                                    if nat_eq(a467, a468) {
                                        Ok(8388608u64)
                                    } else {
                                        let a469 = v4;
                                        let a470 = 8u64;
                                        if nat_eq(a469, a470) {
                                            Ok(2139095039u64)
                                        } else {
                                            let a471 = v4;
                                            let a472 = 9u64;
                                            if nat_eq(a471, a472) {
                                                Ok(4286578687u64)
                                            } else {
                                                let a473 = v4;
                                                let a474 = 10u64;
                                                if nat_eq(a473, a474) {
                                                    Ok(2139095040u64)
                                                } else {
                                                    let a475 = v4;
                                                    let a476 = 11u64;
                                                    if nat_eq(a475, a476) {
                                                        Ok(4286578688u64)
                                                    } else {
                                                        let a477 = v4;
                                                        let a478 = 12u64;
                                                        if nat_eq(a477, a478) {
                                                            Ok(2143289345u64)
                                                        } else {
                                                            let a479 = v4;
                                                            let a480 = 13u64;
                                                            if nat_eq(a479, a480) {
                                                                Ok(2139095041u64)
                                                            } else {
                                                                let a481 = v4;
                                                                let a482 = 14u64;
                                                                if nat_eq(a481, a482) {
                                                                    Ok(4286578689u64)
                                                                } else {
                                                                    let a483 = v4;
                                                                    let a484 = 15u64;
                                                                    if nat_eq(a483, a484) {
                                                                        Ok(1065357312u64)
                                                                    } else {
                                                                        let a485 = v4;
                                                                        let a486 = 16u64;
                                                                        if nat_eq(a485, a486) {
                                                                            Ok(1065365504u64)
                                                                        } else {
                                                                            let a487 = v4;
                                                                            let a488 = 17u64;
                                                                            if nat_eq(a487, a488) {
                                                                                Ok(1065385984u64)
                                                                            } else {
                                                                                let a489 = v4;
                                                                                let a490 = 18u64;
                                                                                if nat_eq(a489, a490) {
                                                                                    Ok(1065451520u64)
                                                                                } else {
                                                                                    let a491 = v4;
                                                                                    let a492 = 19u64;
                                                                                    if nat_eq(a491, a492) {
                                                                                        Ok(864026624u64)
                                                                                    } else {
                                                                                        let a493 = v4;
                                                                                        let a494 = 20u64;
                                                                                        if nat_eq(a493, a494) {
                                                                                            Ok(855638017u64)
                                                                                        } else {
                                                                                            let a495 = v4;
                                                                                            let a496 = 21u64;
                                                                                            if nat_eq(a495, a496) {
                                                                                                Ok(947912704u64)
                                                                                            } else {
                                                                                                let a497 = v4;
                                                                                                let a498 = 22u64;
                                                                                                if nat_eq(a497, a498) {
                                                                                                    Ok(1199566848u64)
                                                                                                } else {
                                                                                                    let a499 = v4;
                                                                                                    let a500 = 23u64;
                                                                                                    if nat_eq(a499, a500) {
                                                                                                        Ok(1056964608u64)
                                                                                                    } else {
                                                                                                        let a501 = v4;
                                                                                                        let a502 = 24u64;
                                                                                                        if nat_eq(a501, a502) {
                                                                                                            Ok(1069547520u64)
                                                                                                        } else {
                                                                                                            let a503 = v4;
                                                                                                            let a504 = 25u64;
                                                                                                            if nat_eq(a503, a504) {
                                                                                                                Ok(1266679808u64)
                                                                                                            } else {
                                                                                                                let a505 = v4;
                                                                                                                let a506 = 26u64;
                                                                                                                if nat_eq(a505, a506) {
                                                                                                                    Ok(3414163456u64)
                                                                                                                } else {
                                                                                                                    let a507 = v4;
                                                                                                                    let a508 = 27u64;
                                                                                                                    if nat_eq(a507, a508) {
                                                                                                                        Ok(1065353217u64)
                                                                                                                    } else {
                                                                                                                        let a509 = v4;
                                                                                                                        let a510 = 28u64;
                                                                                                                        if nat_eq(a509, a510) {
                                                                                                                            Ok(8388609u64)
                                                                                                                        } else {
                                                                                                                            let a511 = v4;
                                                                                                                            let a512 = 29u64;
                                                                                                                            if nat_eq(a511, a512) {
                                                                                                                                Ok(1065353215u64)
                                                                                                                            } else {
                                                                                                                                let a513 = v4;
                                                                                                                                let a514 = 30u64;
                                                                                                                                if nat_eq(a513, a514) {
                                                                                                                                    Ok(3225419776u64)
                                                                                                                                } else {
                                                                                                                                    Ok(1077936128u64)
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
    } else {
        let a515 = v3;
        let a516 = 1u64;
        if nat_eq(a515, a516) {
            let a517 = v4;
            let a518 = 0u64;
            if nat_eq(a517, a518) {
                Ok(0u64)
            } else {
                let a519 = v4;
                let a520 = 1u64;
                if nat_eq(a519, a520) {
                    Ok(32768u64)
                } else {
                    let a521 = v4;
                    let a522 = 2u64;
                    if nat_eq(a521, a522) {
                        Ok(15360u64)
                    } else {
                        let a523 = v4;
                        let a524 = 3u64;
                        if nat_eq(a523, a524) {
                            Ok(48128u64)
                        } else {
                            let a525 = v4;
                            let a526 = 4u64;
                            if nat_eq(a525, a526) {
                                Ok(1u64)
                            } else {
                                let a527 = v4;
                                let a528 = 5u64;
                                if nat_eq(a527, a528) {
                                    Ok(32769u64)
                                } else {
                                    let a529 = v4;
                                    let a530 = 6u64;
                                    if nat_eq(a529, a530) {
                                        Ok(1023u64)
                                    } else {
                                        let a531 = v4;
                                        let a532 = 7u64;
                                        if nat_eq(a531, a532) {
                                            Ok(1024u64)
                                        } else {
                                            let a533 = v4;
                                            let a534 = 8u64;
                                            if nat_eq(a533, a534) {
                                                Ok(31743u64)
                                            } else {
                                                let a535 = v4;
                                                let a536 = 9u64;
                                                if nat_eq(a535, a536) {
                                                    Ok(64511u64)
                                                } else {
                                                    let a537 = v4;
                                                    let a538 = 10u64;
                                                    if nat_eq(a537, a538) {
                                                        Ok(31744u64)
                                                    } else {
                                                        let a539 = v4;
                                                        let a540 = 11u64;
                                                        if nat_eq(a539, a540) {
                                                            Ok(64512u64)
                                                        } else {
                                                            let a541 = v4;
                                                            let a542 = 12u64;
                                                            if nat_eq(a541, a542) {
                                                                Ok(32257u64)
                                                            } else {
                                                                let a543 = v4;
                                                                let a544 = 13u64;
                                                                if nat_eq(a543, a544) {
                                                                    Ok(31745u64)
                                                                } else {
                                                                    let a545 = v4;
                                                                    let a546 = 14u64;
                                                                    if nat_eq(a545, a546) {
                                                                        Ok(64513u64)
                                                                    } else {
                                                                        let a547 = v4;
                                                                        let a548 = 15u64;
                                                                        if nat_eq(a547, a548) {
                                                                            Ok(15361u64)
                                                                        } else {
                                                                            let a549 = v4;
                                                                            let a550 = 16u64;
                                                                            if nat_eq(a549, a550) {
                                                                                Ok(16383u64)
                                                                            } else {
                                                                                let a551 = v4;
                                                                                let a552 = 17u64;
                                                                                if nat_eq(a551, a552) {
                                                                                    Ok(15362u64)
                                                                                } else {
                                                                                    let a553 = v4;
                                                                                    let a554 = 18u64;
                                                                                    if nat_eq(a553, a554) {
                                                                                        Ok(14336u64)
                                                                                    } else {
                                                                                        let a555 = v4;
                                                                                        let a556 = 19u64;
                                                                                        if nat_eq(a555, a556) {
                                                                                            Ok(4096u64)
                                                                                        } else {
                                                                                            let a557 = v4;
                                                                                            let a558 = 20u64;
                                                                                            if nat_eq(a557, a558) {
                                                                                                Ok(5120u64)
                                                                                            } else {
                                                                                                let a559 = v4;
                                                                                                let a560 = 21u64;
                                                                                                if nat_eq(a559, a560) {
                                                                                                    Ok(1025u64)
                                                                                                } else {
                                                                                                    let a561 = v4;
                                                                                                    let a562 = 22u64;
                                                                                                    if nat_eq(a561, a562) {
                                                                                                        Ok(31742u64)
                                                                                                    } else {
                                                                                                        let a563 = v4;
                                                                                                        let a564 = 23u64;
                                                                                                        if nat_eq(a563, a564) {
                                                                                                            Ok(14336u64)
                                                                                                        } else {
                                                                                                            let a565 = v4;
                                                                                                            let a566 = 24u64;
                                                                                                            if nat_eq(a565, a566) {
                                                                                                                Ok(15872u64)
                                                                                                            } else {
                                                                                                                let a567 = v4;
                                                                                                                let a568 = 25u64;
                                                                                                                if nat_eq(a567, a568) {
                                                                                                                    Ok(25600u64)
                                                                                                                } else {
                                                                                                                    let a569 = v4;
                                                                                                                    let a570 = 26u64;
                                                                                                                    if nat_eq(a569, a570) {
                                                                                                                        Ok(58368u64)
                                                                                                                    } else {
                                                                                                                        let a571 = v4;
                                                                                                                        let a572 = 27u64;
                                                                                                                        if nat_eq(a571, a572) {
                                                                                                                            Ok(15361u64)
                                                                                                                        } else {
                                                                                                                            let a573 = v4;
                                                                                                                            let a574 = 28u64;
                                                                                                                            if nat_eq(a573, a574) {
                                                                                                                                Ok(1025u64)
                                                                                                                            } else {
                                                                                                                                let a575 = v4;
                                                                                                                                let a576 = 29u64;
                                                                                                                                if nat_eq(a575, a576) {
                                                                                                                                    Ok(15359u64)
                                                                                                                                } else {
                                                                                                                                    let a577 = v4;
                                                                                                                                    let a578 = 30u64;
                                                                                                                                    if nat_eq(a577, a578) {
                                                                                                                                        Ok(49664u64)
                                                                                                                                    } else {
                                                                                                                                        Ok(16896u64)
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
        } else {
            let a579 = v4;
            let a580 = 0u64;
            if nat_eq(a579, a580) {
                Ok(0u64)
            } else {
                let a581 = v4;
                let a582 = 1u64;
                if nat_eq(a581, a582) {
                    Ok(32768u64)
                } else {
                    let a583 = v4;
                    let a584 = 2u64;
                    if nat_eq(a583, a584) {
                        Ok(16256u64)
                    } else {
                        let a585 = v4;
                        let a586 = 3u64;
                        if nat_eq(a585, a586) {
                            Ok(49024u64)
                        } else {
                            let a587 = v4;
                            let a588 = 4u64;
                            if nat_eq(a587, a588) {
                                Ok(1u64)
                            } else {
                                let a589 = v4;
                                let a590 = 5u64;
                                if nat_eq(a589, a590) {
                                    Ok(32769u64)
                                } else {
                                    let a591 = v4;
                                    let a592 = 6u64;
                                    if nat_eq(a591, a592) {
                                        Ok(127u64)
                                    } else {
                                        let a593 = v4;
                                        let a594 = 7u64;
                                        if nat_eq(a593, a594) {
                                            Ok(128u64)
                                        } else {
                                            let a595 = v4;
                                            let a596 = 8u64;
                                            if nat_eq(a595, a596) {
                                                Ok(32639u64)
                                            } else {
                                                let a597 = v4;
                                                let a598 = 9u64;
                                                if nat_eq(a597, a598) {
                                                    Ok(65407u64)
                                                } else {
                                                    let a599 = v4;
                                                    let a600 = 10u64;
                                                    if nat_eq(a599, a600) {
                                                        Ok(32640u64)
                                                    } else {
                                                        let a601 = v4;
                                                        let a602 = 11u64;
                                                        if nat_eq(a601, a602) {
                                                            Ok(65408u64)
                                                        } else {
                                                            let a603 = v4;
                                                            let a604 = 12u64;
                                                            if nat_eq(a603, a604) {
                                                                Ok(32705u64)
                                                            } else {
                                                                let a605 = v4;
                                                                let a606 = 13u64;
                                                                if nat_eq(a605, a606) {
                                                                    Ok(32641u64)
                                                                } else {
                                                                    let a607 = v4;
                                                                    let a608 = 14u64;
                                                                    if nat_eq(a607, a608) {
                                                                        Ok(65409u64)
                                                                    } else {
                                                                        let a609 = v4;
                                                                        let a610 = 15u64;
                                                                        if nat_eq(a609, a610) {
                                                                            Ok(16257u64)
                                                                        } else {
                                                                            let a611 = v4;
                                                                            let a612 = 16u64;
                                                                            if nat_eq(a611, a612) {
                                                                                Ok(16383u64)
                                                                            } else {
                                                                                let a613 = v4;
                                                                                let a614 = 17u64;
                                                                                if nat_eq(a613, a614) {
                                                                                    Ok(16258u64)
                                                                                } else {
                                                                                    let a615 = v4;
                                                                                    let a616 = 18u64;
                                                                                    if nat_eq(a615, a616) {
                                                                                        Ok(16128u64)
                                                                                    } else {
                                                                                        let a617 = v4;
                                                                                        let a618 = 19u64;
                                                                                        if nat_eq(a617, a618) {
                                                                                            Ok(13056u64)
                                                                                        } else {
                                                                                            let a619 = v4;
                                                                                            let a620 = 20u64;
                                                                                            if nat_eq(a619, a620) {
                                                                                                Ok(13312u64)
                                                                                            } else {
                                                                                                let a621 = v4;
                                                                                                let a622 = 21u64;
                                                                                                if nat_eq(a621, a622) {
                                                                                                    Ok(129u64)
                                                                                                } else {
                                                                                                    let a623 = v4;
                                                                                                    let a624 = 22u64;
                                                                                                    if nat_eq(a623, a624) {
                                                                                                        Ok(32638u64)
                                                                                                    } else {
                                                                                                        let a625 = v4;
                                                                                                        let a626 = 23u64;
                                                                                                        if nat_eq(a625, a626) {
                                                                                                            Ok(16128u64)
                                                                                                        } else {
                                                                                                            let a627 = v4;
                                                                                                            let a628 = 24u64;
                                                                                                            if nat_eq(a627, a628) {
                                                                                                                Ok(16320u64)
                                                                                                            } else {
                                                                                                                let a629 = v4;
                                                                                                                let a630 = 25u64;
                                                                                                                if nat_eq(a629, a630) {
                                                                                                                    Ok(19328u64)
                                                                                                                } else {
                                                                                                                    let a631 = v4;
                                                                                                                    let a632 = 26u64;
                                                                                                                    if nat_eq(a631, a632) {
                                                                                                                        Ok(52096u64)
                                                                                                                    } else {
                                                                                                                        let a633 = v4;
                                                                                                                        let a634 = 27u64;
                                                                                                                        if nat_eq(a633, a634) {
                                                                                                                            Ok(16257u64)
                                                                                                                        } else {
                                                                                                                            let a635 = v4;
                                                                                                                            let a636 = 28u64;
                                                                                                                            if nat_eq(a635, a636) {
                                                                                                                                Ok(129u64)
                                                                                                                            } else {
                                                                                                                                let a637 = v4;
                                                                                                                                let a638 = 29u64;
                                                                                                                                if nat_eq(a637, a638) {
                                                                                                                                    Ok(16255u64)
                                                                                                                                } else {
                                                                                                                                    let a639 = v4;
                                                                                                                                    let a640 = 30u64;
                                                                                                                                    if nat_eq(a639, a640) {
                                                                                                                                        Ok(49216u64)
                                                                                                                                    } else {
                                                                                                                                        Ok(16448u64)
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

pub fn f24(v0: u64) -> u64 {
    if f16(v0) {
        3u64
    } else {
        if f15(v0) {
            let a643 = {
                let a641 = v0;
                let a642 = 864u64;
                nat_sub(a641, a642)
            };
            let a644 = 16u64;
            let a645 = 0u64;
            nat_quot(a643, a644, a645)
        } else {
            let a646 = f28(v0);
            let a647 = 4u64;
            if nat_lt(a646, a647) {
                0u64
            } else {
                let a648 = f28(v0);
                let a649 = 8u64;
                if nat_lt(a648, a649) {
                    1u64
                } else {
                    let a650 = f28(v0);
                    let a651 = 12u64;
                    if nat_lt(a650, a651) {
                        2u64
                    } else {
                        let a652 = f28(v0);
                        let a653 = 14u64;
                        if nat_lt(a652, a653) {
                            3u64
                        } else {
                            let a654 = f28(v0);
                            let a655 = 14u64;
                            if nat_eq(a654, a655) {
                                4u64
                            } else {
                                let a656 = f28(v0);
                                let a657 = 17u64;
                                if nat_lt(a656, a657) {
                                    5u64
                                } else {
                                    6u64
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f25(v0: u64) -> u64 {
    let a658 = 864u64;
    let a659 = v0;
    if nat_le(a658, a659) {
        0u64
    } else {
        let a660 = f28(v0);
        let a661 = 12u64;
        if nat_lt(a660, a661) {
            let a662 = f28(v0);
            let a663 = 4u64;
            let a664 = 0u64;
            nat_rem(a662, a663, a664)
        } else {
            let a665 = f28(v0);
            let a666 = 13u64;
            if if nat_eq(a665, a666) {
                true
            } else {
                let a667 = f28(v0);
                let a668 = 16u64;
                nat_eq(a667, a668)
            } {
                1u64
            } else {
                0u64
            }
        }
    }
}

pub fn f26(v0: u64) -> u64 {
    let a669 = v0;
    let a670 = 864u64;
    if nat_lt(a669, a670) {
        let a674 = {
            let a671 = v0;
            let a672 = 12u64;
            let a673 = 0u64;
            nat_quot(a671, a672, a673)
        };
        let a675 = 4u64;
        let a676 = 0u64;
        nat_rem(a674, a675, a676)
    } else {
        if f15(v0) {
            let a682 = {
                let a679 = {
                    let a677 = v0;
                    let a678 = 864u64;
                    nat_sub(a677, a678)
                };
                let a680 = 4u64;
                let a681 = 0u64;
                nat_quot(a679, a680, a681)
            };
            let a683 = 4u64;
            let a684 = 0u64;
            nat_rem(a682, a683, a684)
        } else {
            0u64
        }
    }
}

pub fn f27(v0: Adt0) -> R<u64> {
    let a705 = {
        let a697 = {
            let a689 = {
                let a686 = match v0.clone() {
                    Adt0::C0(_, h685, _, _, _, _, _, _, _, _) => {
                        h685
                    }
                };
                let a687 = 1u64;
                nat_sub(a686, a687)
            };
            let a690 = match v0.clone() {
                Adt0::C0(_, _, _, _, _, h688, _, _, _, _) => {
                    h688
                }
            };
            nat_mul(a689, a690)?
        };
        let a698 = {
            let a695 = {
                let a692 = match v0.clone() {
                    Adt0::C0(_, _, h691, _, _, _, _, _, _, _) => {
                        h691
                    }
                };
                let a693 = 1u64;
                nat_sub(a692, a693)
            };
            let a696 = match v0.clone() {
                Adt0::C0(_, _, _, _, _, _, h694, _, _, _) => {
                    h694
                }
            };
            nat_mul(a695, a696)?
        };
        nat_add(a697, a698)?
    };
    let a706 = {
        let a703 = {
            let a700 = match v0.clone() {
                Adt0::C0(_, _, _, h699, _, _, _, _, _, _) => {
                    h699
                }
            };
            let a701 = 1u64;
            nat_sub(a700, a701)
        };
        let a704 = match v0 {
            Adt0::C0(_, _, _, _, _, _, _, h702, _, _) => {
                h702
            }
        };
        nat_mul(a703, a704)?
    };
    nat_add(a705, a706)
}

pub fn f28(v0: u64) -> u64 {
    let a707 = v0;
    let a708 = 48u64;
    let a709 = 0u64;
    nat_quot(a707, a708, a709)
}

pub fn run(p0: u64, p1: u64, p2: u64, p3: u64) -> R<Option<u64>> {
    f0(p0, p1, p2, p3)
}
