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
        let a6 = 353u64;
        if nat_lt(a5, a6) {
            let a7 = v0;
            let a8 = 1u64;
            if nat_eq(a7, a8) {
                let a9 = v2;
                let a10 = 0u64;
                if if nat_eq(a9, a10) {
                    let a11 = v3;
                    let a12 = 12u64;
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
                                f3(v1, v2, 0u64)?
                            } else {
                                let a25 = v3;
                                let a26 = 1u64;
                                if nat_eq(a25, a26) {
                                    f3(v1, v2, 1u64)?
                                } else {
                                    let a27 = v3;
                                    let a28 = 2u64;
                                    if nat_eq(a27, a28) {
                                        f3(v1, v2, 2u64)?
                                    } else {
                                        let a29 = v3;
                                        let a30 = 3u64;
                                        if nat_eq(a29, a30) {
                                            f3(v1, v2, 3u64)?
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
        353u64
    } else {
        let a99 = v0;
        let a100 = 1u64;
        if nat_eq(a99, a100) {
            12u64
        } else {
            let a101 = v0;
            let a102 = 2u64;
            if nat_eq(a101, a102) {
                706u64
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
                        let a107 = v0;
                        let a108 = 5u64;
                        let _ = nat_eq(a107, a108);
                        115u64
                    }
                }
            }
        }
    }
}

pub fn f2(v0: u64, v1: u64) -> R<u64> {
    let a109 = v1;
    let a110 = 0u64;
    if nat_eq(a109, a110) {
        Ok(f9(v0, 0u64))
    } else {
        let a111 = v1;
        let a112 = 1u64;
        if nat_eq(a111, a112) {
            Ok(f10(v0))
        } else {
            let a113 = v1;
            let a114 = 2u64;
            if nat_eq(a113, a114) {
                Ok(f11(v0))
            } else {
                let a115 = v1;
                let a116 = 3u64;
                if nat_eq(a115, a116) {
                    Ok(f12(v0))
                } else {
                    let a117 = v1;
                    let a118 = 4u64;
                    if nat_eq(a117, a118) {
                        f3(v0, 0u64, 0u64)
                    } else {
                        let a119 = v1;
                        let a120 = 5u64;
                        if nat_eq(a119, a120) {
                            f3(v0, 0u64, 1u64)
                        } else {
                            let a121 = v1;
                            let a122 = 6u64;
                            if nat_eq(a121, a122) {
                                f3(v0, 0u64, 3u64)
                            } else {
                                let a123 = v1;
                                let a124 = 7u64;
                                if nat_eq(a123, a124) {
                                    Ok(v0)
                                } else {
                                    let a125 = v1;
                                    let a126 = 8u64;
                                    if nat_eq(a125, a126) {
                                        Ok(f13(v0))
                                    } else {
                                        let a127 = v1;
                                        let a128 = 9u64;
                                        if nat_eq(a127, a128) {
                                            Ok(f14(v0))
                                        } else {
                                            let a129 = v1;
                                            let a130 = 10u64;
                                            if nat_eq(a129, a130) {
                                                let a131 = 353u64;
                                                let a132 = v0;
                                                nat_add(a131, a132)
                                            } else {
                                                let a133 = v1;
                                                let a134 = 11u64;
                                                let _ = nat_eq(a133, a134);
                                                Ok(f9(v0, 1u64))
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

pub fn f3(v0: u64, v1: u64, v2: u64) -> R<u64> {
    if if f15(v0) {
        let a135 = v2;
        let a136 = f16(v0);
        nat_eq(a135, a136)
    } else {
        false
    } {
        Ok(0u64)
    } else {
        if f17(v0) {
            let a137 = v2;
            let a138 = 0u64;
            if nat_eq(a137, a138) {
                Ok(3u64)
            } else {
                let a139 = v2;
                let a140 = 1u64;
                if nat_eq(a139, a140) {
                    Ok(1u64)
                } else {
                    let a141 = v2;
                    let a142 = 2u64;
                    if nat_eq(a141, a142) {
                        Ok(1u64)
                    } else {
                        let a143 = v2;
                        let a144 = 3u64;
                        let _ = nat_eq(a143, a144);
                        Ok(65537u64)
                    }
                }
            }
        } else {
            let a145 = v2;
            let a146 = 0u64;
            if nat_eq(a145, a146) {
                let a151 = f18(v0);
                let a152 = {
                    let a147 = v1;
                    let a148 = 1u64;
                    if if nat_eq(a147, a148) {
                        let a149 = f11(v0);
                        let a150 = 13u64;
                        nat_eq(a149, a150)
                    } else {
                        false
                    } {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a151, a152)
            } else {
                let a153 = v2;
                let a154 = 1u64;
                if nat_eq(a153, a154) {
                    let a155 = f12(v0);
                    let a156 = 4u64;
                    if nat_eq(a155, a156) {
                        Ok(3u64)
                    } else {
                        Ok(2u64)
                    }
                } else {
                    let a157 = v2;
                    let a158 = 2u64;
                    if nat_eq(a157, a158) {
                        Ok(2u64)
                    } else {
                        let a159 = v2;
                        let a160 = 3u64;
                        let _ = nat_eq(a159, a160);
                        Ok(2u64)
                    }
                }
            }
        }
    }
}

pub fn f4(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a161 = v2;
    let a162 = 0u64;
    if nat_eq(a161, a162) {
        f19(v0, v1)
    } else {
        let a163 = v2;
        let a164 = 1u64;
        if nat_eq(a163, a164) {
            f20(v0, v1)
        } else {
            let a165 = v2;
            let a166 = 2u64;
            if nat_eq(a165, a166) {
                f21(v0, v1)
            } else {
                let a167 = v2;
                let a168 = 3u64;
                let _ = nat_eq(a167, a168);
                f22(v0, v1)
            }
        }
    }
}

pub fn f5(v0: u64, v1: u64) -> R<u64> {
    let m169 = f23(Adt0::C0(f3(v0, v1, 0u64)?, f3(v0, v1, 1u64)?, f3(v0, v1, 2u64)?, f3(v0, v1, 3u64)?, f4(v0, v1, 0u64)?, f4(v0, v1, 1u64)?, f4(v0, v1, 2u64)?, f4(v0, v1, 3u64)?, 1u64, f24(v0, v1)))?;
    match m169 {
        Adt1::C0 => {
            Ok(0u64)
        }
        Adt1::C1(v2) => {
            Ok(v2)
        }
    }
}

pub fn f6(v0: u64, v1: u64) -> R<u64> {
    let a174 = {
        let a170 = f3(v0, v1, 0u64)?;
        let a171 = f3(v0, v1, 1u64)?;
        nat_mul(a170, a171)?
    };
    let a175 = {
        let a172 = f3(v0, v1, 2u64)?;
        let a173 = f3(v0, v1, 3u64)?;
        nat_mul(a172, a173)?
    };
    nat_mul(a174, a175)
}

pub fn f7(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a202 = {
        let a199 = f25(v0, v1, {
            let a176 = v2;
            let a177 = f24(v0, v1);
            let a178 = 0u64;
            nat_quot(a176, a177, a178)
        })?;
        let a200 = {
            let a182 = {
                let a179 = v2;
                let a180 = f24(v0, v1);
                let a181 = 0u64;
                nat_rem(a179, a180, a181)
            };
            let a183 = 0u64;
            if nat_eq(a182, a183) {
                1u64
            } else {
                let a187 = {
                    let a184 = v2;
                    let a185 = f24(v0, v1);
                    let a186 = 0u64;
                    nat_rem(a184, a185, a186)
                };
                let a188 = 1u64;
                if nat_eq(a187, a188) {
                    256u64
                } else {
                    let a192 = {
                        let a189 = v2;
                        let a190 = f24(v0, v1);
                        let a191 = 0u64;
                        nat_rem(a189, a190, a191)
                    };
                    let a193 = 2u64;
                    if nat_eq(a192, a193) {
                        65536u64
                    } else {
                        let a197 = {
                            let a194 = v2;
                            let a195 = f24(v0, v1);
                            let a196 = 0u64;
                            nat_rem(a194, a195, a196)
                        };
                        let a198 = 3u64;
                        let _ = nat_eq(a197, a198);
                        16777216u64
                    }
                }
            }
        };
        let a201 = 0u64;
        nat_quot(a199, a200, a201)
    };
    let a203 = 256u64;
    let a204 = 0u64;
    Ok(nat_rem(a202, a203, a204))
}

pub fn f8(v0: u64, v1: u64) -> R<u64> {
    let a207 = {
        let a205 = f5(v0, v1)?;
        let a206 = 511u64;
        nat_add(a205, a206)?
    };
    let a208 = 512u64;
    let a209 = 0u64;
    Ok(nat_quot(a207, a208, a209))
}

pub fn f9(v0: u64, v1: u64) -> u64 {
    let a210 = v1;
    let a211 = 1u64;
    if nat_eq(a210, a211) {
        let a212 = f11(v0);
        let a213 = 11u64;
        if nat_eq(a212, a213) {
            1u64
        } else {
            0u64
        }
    } else {
        let a214 = f11(v0);
        let a215 = 12u64;
        if nat_eq(a214, a215) {
            1u64
        } else {
            0u64
        }
    }
}

pub fn f10(v0: u64) -> u64 {
    let a216 = f11(v0);
    let a217 = 11u64;
    if if if nat_eq(a216, a217) {
        true
    } else {
        let a218 = f11(v0);
        let a219 = 12u64;
        nat_eq(a218, a219)
    } {
        true
    } else {
        let a220 = f11(v0);
        let a221 = 13u64;
        nat_eq(a220, a221)
    } {
        0u64
    } else {
        f26(v0)
    }
}

pub fn f11(v0: u64) -> u64 {
    let a222 = v0;
    let a223 = 320u64;
    if nat_lt(a222, a223) {
        let a224 = v0;
        let a225 = 16u64;
        let a226 = 0u64;
        nat_rem(a224, a225, a226)
    } else {
        if f15(v0) {
            16u64
        } else {
            17u64
        }
    }
}

pub fn f12(v0: u64) -> u64 {
    let a227 = v0;
    let a228 = 320u64;
    if nat_lt(a227, a228) {
        let a232 = {
            let a229 = v0;
            let a230 = 80u64;
            let a231 = 0u64;
            nat_rem(a229, a230, a231)
        };
        let a233 = 16u64;
        let a234 = 0u64;
        nat_quot(a232, a233, a234)
    } else {
        0u64
    }
}

pub fn f13(v0: u64) -> u64 {
    if f15(v0) {
        0u64
    } else {
        let a235 = f11(v0);
        let a236 = 11u64;
        if if if if nat_eq(a235, a236) {
            true
        } else {
            let a237 = f11(v0);
            let a238 = 12u64;
            nat_eq(a237, a238)
        } {
            true
        } else {
            let a239 = f11(v0);
            let a240 = 13u64;
            nat_eq(a239, a240)
        } {
            true
        } else {
            let a241 = f12(v0);
            let a242 = 3u64;
            if nat_eq(a241, a242) {
                let a243 = f10(v0);
                let a244 = 0u64;
                !nat_eq(a243, a244)
            } else {
                false
            }
        } {
            2u64
        } else {
            let a245 = f11(v0);
            let a246 = 14u64;
            if if if nat_eq(a245, a246) {
                let a247 = f10(v0);
                let a248 = 2u64;
                if nat_eq(a247, a248) {
                    true
                } else {
                    let a249 = f10(v0);
                    let a250 = 3u64;
                    nat_eq(a249, a250)
                }
            } else {
                false
            } {
                true
            } else {
                let a251 = f11(v0);
                let a252 = 15u64;
                if nat_eq(a251, a252) {
                    let a253 = f10(v0);
                    let a254 = 1u64;
                    if nat_eq(a253, a254) {
                        true
                    } else {
                        let a255 = f10(v0);
                        let a256 = 3u64;
                        nat_eq(a255, a256)
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

pub fn f14(v0: u64) -> u64 {
    if f15(v0) {
        let a262 = {
            let a259 = {
                let a257 = v0;
                let a258 = 320u64;
                nat_sub(a257, a258)
            };
            let a260 = 2u64;
            let a261 = 0u64;
            nat_rem(a259, a260, a261)
        };
        let a263 = 0u64;
        if nat_eq(a262, a263) {
            3212836864u64
        } else {
            2143289345u64
        }
    } else {
        let a264 = f11(v0);
        let a265 = 0u64;
        if nat_eq(a264, a265) {
            0u64
        } else {
            let a266 = f11(v0);
            let a267 = 1u64;
            if nat_eq(a266, a267) {
                0u64
            } else {
                let a268 = f11(v0);
                let a269 = 2u64;
                if nat_eq(a268, a269) {
                    897988541u64
                } else {
                    let a270 = f11(v0);
                    let a271 = 3u64;
                    if nat_eq(a270, a271) {
                        0u64
                    } else {
                        let a272 = f11(v0);
                        let a273 = 4u64;
                        if nat_eq(a272, a273) {
                            1065353216u64
                        } else {
                            let a274 = f11(v0);
                            let a275 = 5u64;
                            if nat_eq(a274, a275) {
                                0u64
                            } else {
                                let a276 = f11(v0);
                                let a277 = 6u64;
                                if nat_eq(a276, a277) {
                                    0u64
                                } else {
                                    let a278 = f11(v0);
                                    let a279 = 7u64;
                                    if nat_eq(a278, a279) {
                                        0u64
                                    } else {
                                        let a280 = f11(v0);
                                        let a281 = 8u64;
                                        if nat_eq(a280, a281) {
                                            3212836864u64
                                        } else {
                                            let a282 = f11(v0);
                                            let a283 = 9u64;
                                            if nat_eq(a282, a283) {
                                                2143289345u64
                                            } else {
                                                let a284 = f11(v0);
                                                let a285 = 10u64;
                                                if nat_eq(a284, a285) {
                                                    2139095040u64
                                                } else {
                                                    let a286 = f11(v0);
                                                    let a287 = 11u64;
                                                    if nat_eq(a286, a287) {
                                                        897988541u64
                                                    } else {
                                                        let a288 = f11(v0);
                                                        let a289 = 12u64;
                                                        if nat_eq(a288, a289) {
                                                            897988541u64
                                                        } else {
                                                            let a290 = f11(v0);
                                                            let a291 = 13u64;
                                                            if nat_eq(a290, a291) {
                                                                897988541u64
                                                            } else {
                                                                let a292 = f11(v0);
                                                                let a293 = 14u64;
                                                                if nat_eq(a292, a293) {
                                                                    897988541u64
                                                                } else {
                                                                    let a294 = f11(v0);
                                                                    let a295 = 15u64;
                                                                    if nat_eq(a294, a295) {
                                                                        897988541u64
                                                                    } else {
                                                                        let a296 = f11(v0);
                                                                        let a297 = 16u64;
                                                                        if nat_eq(a296, a297) {
                                                                            0u64
                                                                        } else {
                                                                            let a298 = f11(v0);
                                                                            let a299 = 17u64;
                                                                            let _ = nat_eq(a298, a299);
                                                                            897988541u64
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

pub fn f15(v0: u64) -> bool {
    let a300 = 320u64;
    let a301 = v0;
    if nat_le(a300, a301) {
        let a302 = v0;
        let a303 = 352u64;
        nat_lt(a302, a303)
    } else {
        false
    }
}

pub fn f16(v0: u64) -> u64 {
    let a309 = {
        let a306 = {
            let a304 = v0;
            let a305 = 320u64;
            nat_sub(a304, a305)
        };
        let a307 = 8u64;
        let a308 = 0u64;
        nat_rem(a306, a307, a308)
    };
    let a310 = 2u64;
    let a311 = 0u64;
    nat_quot(a309, a310, a311)
}

pub fn f17(v0: u64) -> bool {
    let a312 = v0;
    let a313 = 352u64;
    nat_eq(a312, a313)
}

pub fn f18(v0: u64) -> u64 {
    let a314 = f11(v0);
    let a315 = 0u64;
    if nat_eq(a314, a315) {
        5u64
    } else {
        let a316 = f11(v0);
        let a317 = 1u64;
        if nat_eq(a316, a317) {
            5u64
        } else {
            let a318 = f11(v0);
            let a319 = 2u64;
            if nat_eq(a318, a319) {
                7u64
            } else {
                let a320 = f11(v0);
                let a321 = 3u64;
                if nat_eq(a320, a321) {
                    3u64
                } else {
                    let a322 = f11(v0);
                    let a323 = 4u64;
                    if nat_eq(a322, a323) {
                        4u64
                    } else {
                        let a324 = f11(v0);
                        let a325 = 5u64;
                        if nat_eq(a324, a325) {
                            5u64
                        } else {
                            let a326 = f11(v0);
                            let a327 = 6u64;
                            if nat_eq(a326, a327) {
                                3u64
                            } else {
                                let a328 = f11(v0);
                                let a329 = 7u64;
                                if nat_eq(a328, a329) {
                                    5u64
                                } else {
                                    let a330 = f11(v0);
                                    let a331 = 8u64;
                                    if nat_eq(a330, a331) {
                                        5u64
                                    } else {
                                        let a332 = f11(v0);
                                        let a333 = 9u64;
                                        if nat_eq(a332, a333) {
                                            3u64
                                        } else {
                                            let a334 = f11(v0);
                                            let a335 = 10u64;
                                            if nat_eq(a334, a335) {
                                                1u64
                                            } else {
                                                let a336 = f11(v0);
                                                let a337 = 11u64;
                                                if nat_eq(a336, a337) {
                                                    5u64
                                                } else {
                                                    let a338 = f11(v0);
                                                    let a339 = 12u64;
                                                    if nat_eq(a338, a339) {
                                                        5u64
                                                    } else {
                                                        let a340 = f11(v0);
                                                        let a341 = 13u64;
                                                        if nat_eq(a340, a341) {
                                                            5u64
                                                        } else {
                                                            let a342 = f11(v0);
                                                            let a343 = 14u64;
                                                            if nat_eq(a342, a343) {
                                                                5u64
                                                            } else {
                                                                let a344 = f11(v0);
                                                                let a345 = 15u64;
                                                                if nat_eq(a344, a345) {
                                                                    5u64
                                                                } else {
                                                                    let a346 = f11(v0);
                                                                    let a347 = 16u64;
                                                                    if nat_eq(a346, a347) {
                                                                        3u64
                                                                    } else {
                                                                        let a348 = f11(v0);
                                                                        let a349 = 17u64;
                                                                        let _ = nat_eq(a348, a349);
                                                                        3u64
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

pub fn f19(v0: u64, v1: u64) -> R<u64> {
    let a364 = f24(v0, v1);
    let a365 = {
        let a350 = f12(v0);
        let a351 = 3u64;
        if if nat_eq(a350, a351) {
            let a352 = f27(v0, v1);
            let a353 = 3u64;
            if if nat_eq(a352, a353) {
                true
            } else {
                let a354 = f27(v0, v1);
                let a355 = 0u64;
                if nat_eq(a354, a355) {
                    let a356 = f10(v0);
                    let a357 = 1u64;
                    if nat_eq(a356, a357) {
                        true
                    } else {
                        let a358 = f10(v0);
                        let a359 = 3u64;
                        nat_eq(a358, a359)
                    }
                } else {
                    false
                }
            } {
                true
            } else {
                let a360 = f27(v0, v1);
                let a361 = 1u64;
                if nat_eq(a360, a361) {
                    let a362 = f10(v0);
                    let a363 = 2u64;
                    nat_eq(a362, a363)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            2u64
        } else {
            1u64
        }
    };
    nat_mul(a364, a365)
}

pub fn f20(v0: u64, v1: u64) -> R<u64> {
    let a366 = f11(v0);
    let a367 = 14u64;
    if if if nat_eq(a366, a367) {
        let a368 = f27(v0, v1);
        let a369 = 1u64;
        if nat_eq(a368, a369) {
            true
        } else {
            let a370 = f10(v0);
            let a371 = 3u64;
            nat_eq(a370, a371)
        }
    } else {
        false
    } {
        true
    } else {
        let a372 = f11(v0);
        let a373 = 15u64;
        if nat_eq(a372, a373) {
            let a374 = f27(v0, v1);
            let a375 = 0u64;
            nat_eq(a374, a375)
        } else {
            false
        }
    } {
        Ok(0u64)
    } else {
        let a376 = f12(v0);
        let a377 = 4u64;
        if nat_eq(a376, a377) {
            let a378 = f27(v0, v1);
            let a379 = 1u64;
            if nat_eq(a378, a379) {
                Ok(140u64)
            } else {
                Ok(100u64)
            }
        } else {
            let a380 = f12(v0);
            let a381 = 1u64;
            if if if nat_eq(a380, a381) {
                let a382 = f27(v0, v1);
                let a383 = 0u64;
                nat_eq(a382, a383)
            } else {
                false
            } {
                true
            } else {
                let a384 = f12(v0);
                let a385 = 2u64;
                if nat_eq(a384, a385) {
                    let a386 = f27(v0, v1);
                    let a387 = 1u64;
                    nat_eq(a386, a387)
                } else {
                    false
                }
            } {
                Ok(100u64)
            } else {
                let a390 = {
                    let a388 = f19(v0, v1)?;
                    let a389 = f3(v0, v1, 0u64)?;
                    nat_mul(a388, a389)?
                };
                let a391 = f24(v0, v1);
                nat_add(a390, a391)
            }
        }
    }
}

pub fn f21(v0: u64, v1: u64) -> R<u64> {
    let a392 = f12(v0);
    let a393 = 4u64;
    if nat_eq(a392, a393) {
        let a394 = f27(v0, v1);
        let a395 = 1u64;
        if nat_eq(a394, a395) {
            Ok(100u64)
        } else {
            Ok(140u64)
        }
    } else {
        let a396 = f12(v0);
        let a397 = 1u64;
        if if if nat_eq(a396, a397) {
            let a398 = f27(v0, v1);
            let a399 = 0u64;
            nat_eq(a398, a399)
        } else {
            false
        } {
            true
        } else {
            let a400 = f12(v0);
            let a401 = 2u64;
            if nat_eq(a400, a401) {
                let a402 = f27(v0, v1);
                let a403 = 1u64;
                nat_eq(a402, a403)
            } else {
                false
            }
        } {
            Ok(40u64)
        } else {
            let a406 = {
                let a404 = f20(v0, v1)?;
                let a405 = f3(v0, v1, 1u64)?;
                nat_mul(a404, a405)?
            };
            let a407 = f24(v0, v1);
            nat_add(a406, a407)
        }
    }
}

pub fn f22(v0: u64, v1: u64) -> R<u64> {
    let a408 = f12(v0);
    let a409 = 4u64;
    if nat_eq(a408, a409) {
        Ok(600u64)
    } else {
        let a410 = f12(v0);
        let a411 = 1u64;
        if if if nat_eq(a410, a411) {
            let a412 = f27(v0, v1);
            let a413 = 0u64;
            nat_eq(a412, a413)
        } else {
            false
        } {
            true
        } else {
            let a414 = f12(v0);
            let a415 = 2u64;
            if nat_eq(a414, a415) {
                let a416 = f27(v0, v1);
                let a417 = 1u64;
                nat_eq(a416, a417)
            } else {
                false
            }
        } {
            Ok(200u64)
        } else {
            let a420 = {
                let a418 = f21(v0, v1)?;
                let a419 = f3(v0, v1, 2u64)?;
                nat_mul(a418, a419)?
            };
            let a421 = f24(v0, v1);
            nat_add(a420, a421)
        }
    }
}

pub fn f23(v0: Adt0) -> R<Adt1> {
    let a423 = match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h422, _) => {
            h422
        }
    };
    let a424 = 0u64;
    if nat_eq(a423, a424) {
        Ok(Adt1::C0)
    } else {
        let a426 = match v0.clone() {
            Adt0::C0(h425, _, _, _, _, _, _, _, _, _) => {
                h425
            }
        };
        let a427 = 0u64;
        if if if if nat_eq(a426, a427) {
            true
        } else {
            let a429 = match v0.clone() {
                Adt0::C0(_, h428, _, _, _, _, _, _, _, _) => {
                    h428
                }
            };
            let a430 = 0u64;
            nat_eq(a429, a430)
        } {
            true
        } else {
            let a432 = match v0.clone() {
                Adt0::C0(_, _, h431, _, _, _, _, _, _, _) => {
                    h431
                }
            };
            let a433 = 0u64;
            nat_eq(a432, a433)
        } {
            true
        } else {
            let a435 = match v0.clone() {
                Adt0::C0(_, _, _, h434, _, _, _, _, _, _) => {
                    h434
                }
            };
            let a436 = 0u64;
            nat_eq(a435, a436)
        } {
            Ok(Adt1::C1(0u64))
        } else {
            let a438 = match v0.clone() {
                Adt0::C0(_, _, _, _, _, _, _, _, h437, _) => {
                    h437
                }
            };
            let a439 = 1u64;
            if nat_eq(a438, a439) {
                Ok(Adt1::C1({
                    let a449 = {
                        let a447 = match v0.clone() {
                            Adt0::C0(_, _, _, _, _, _, _, _, _, h440) => {
                                h440
                            }
                        };
                        let a448 = {
                            let a445 = {
                                let a442 = match v0.clone() {
                                    Adt0::C0(h441, _, _, _, _, _, _, _, _, _) => {
                                        h441
                                    }
                                };
                                let a443 = 1u64;
                                nat_sub(a442, a443)
                            };
                            let a446 = match v0.clone() {
                                Adt0::C0(_, _, _, _, h444, _, _, _, _, _) => {
                                    h444
                                }
                            };
                            nat_mul(a445, a446)?
                        };
                        nat_add(a447, a448)?
                    };
                    let a450 = f28(v0)?;
                    nat_add(a449, a450)?
                }))
            } else {
                Ok(Adt1::C1({
                    let a459 = {
                        let a456 = {
                            let a453 = match v0.clone() {
                                Adt0::C0(h451, _, _, _, _, _, _, _, _, _) => {
                                    h451
                                }
                            };
                            let a454 = match v0.clone() {
                                Adt0::C0(_, _, _, _, h452, _, _, _, _, _) => {
                                    h452
                                }
                            };
                            nat_mul(a453, a454)?
                        };
                        let a457 = match v0.clone() {
                            Adt0::C0(_, _, _, _, _, _, _, _, h455, _) => {
                                h455
                            }
                        };
                        let a458 = 0u64;
                        nat_quot(a456, a457, a458)
                    };
                    let a460 = f28(v0)?;
                    nat_add(a459, a460)?
                }))
            }
        }
    }
}

pub fn f24(v0: u64, v1: u64) -> u64 {
    let a461 = f9(v0, v1);
    let a462 = 1u64;
    if nat_eq(a461, a462) {
        2u64
    } else {
        4u64
    }
}

pub fn f25(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a463 = f10(v0);
        let a464 = 3u64;
        if nat_eq(a463, a464) {
            1u64
        } else {
            v1
        }
    };
    let a465 = f9(v0, v3);
    let a466 = 1u64;
    if nat_eq(a465, a466) {
        Ok(15360u64)
    } else {
        let a467 = f11(v0);
        let a468 = 0u64;
        if nat_eq(a467, a468) {
            let a469 = v3;
            let a470 = 0u64;
            if nat_eq(a469, a470) {
                let a474 = {
                    let a471 = v2;
                    let a472 = 5u64;
                    let a473 = 0u64;
                    nat_rem(a471, a472, a473)
                };
                let a475 = 0u64;
                if nat_eq(a474, a475) {
                    Ok(0u64)
                } else {
                    let a479 = {
                        let a476 = v2;
                        let a477 = 5u64;
                        let a478 = 0u64;
                        nat_rem(a476, a477, a478)
                    };
                    let a480 = 1u64;
                    if nat_eq(a479, a480) {
                        Ok(1065353216u64)
                    } else {
                        let a484 = {
                            let a481 = v2;
                            let a482 = 5u64;
                            let a483 = 0u64;
                            nat_rem(a481, a482, a483)
                        };
                        let a485 = 2u64;
                        if nat_eq(a484, a485) {
                            Ok(0u64)
                        } else {
                            let a489 = {
                                let a486 = v2;
                                let a487 = 5u64;
                                let a488 = 0u64;
                                nat_rem(a486, a487, a488)
                            };
                            let a490 = 3u64;
                            if nat_eq(a489, a490) {
                                Ok(0u64)
                            } else {
                                let a494 = {
                                    let a491 = v2;
                                    let a492 = 5u64;
                                    let a493 = 0u64;
                                    nat_rem(a491, a492, a493)
                                };
                                let a495 = 4u64;
                                let _ = nat_eq(a494, a495);
                                Ok(0u64)
                            }
                        }
                    }
                }
            } else {
                let a499 = {
                    let a496 = v2;
                    let a497 = 5u64;
                    let a498 = 0u64;
                    nat_rem(a496, a497, a498)
                };
                let a500 = 0u64;
                if nat_eq(a499, a500) {
                    Ok(1166016512u64)
                } else {
                    let a504 = {
                        let a501 = v2;
                        let a502 = 5u64;
                        let a503 = 0u64;
                        nat_rem(a501, a502, a503)
                    };
                    let a505 = 1u64;
                    if nat_eq(a504, a505) {
                        Ok(1065353216u64)
                    } else {
                        let a509 = {
                            let a506 = v2;
                            let a507 = 5u64;
                            let a508 = 0u64;
                            nat_rem(a506, a507, a508)
                        };
                        let a510 = 2u64;
                        if nat_eq(a509, a510) {
                            Ok(0u64)
                        } else {
                            let a514 = {
                                let a511 = v2;
                                let a512 = 5u64;
                                let a513 = 0u64;
                                nat_rem(a511, a512, a513)
                            };
                            let a515 = 3u64;
                            if nat_eq(a514, a515) {
                                Ok(0u64)
                            } else {
                                let a519 = {
                                    let a516 = v2;
                                    let a517 = 5u64;
                                    let a518 = 0u64;
                                    nat_rem(a516, a517, a518)
                                };
                                let a520 = 4u64;
                                let _ = nat_eq(a519, a520);
                                Ok(0u64)
                            }
                        }
                    }
                }
            }
        } else {
            let a521 = f11(v0);
            let a522 = 1u64;
            if nat_eq(a521, a522) {
                let a523 = v3;
                let a524 = 0u64;
                if nat_eq(a523, a524) {
                    let a528 = {
                        let a525 = v2;
                        let a526 = 5u64;
                        let a527 = 0u64;
                        nat_rem(a525, a526, a527)
                    };
                    let a529 = 0u64;
                    if nat_eq(a528, a529) {
                        Ok(1266679808u64)
                    } else {
                        let a533 = {
                            let a530 = v2;
                            let a531 = 5u64;
                            let a532 = 0u64;
                            nat_rem(a530, a531, a532)
                        };
                        let a534 = 1u64;
                        if nat_eq(a533, a534) {
                            Ok(1065353216u64)
                        } else {
                            let a538 = {
                                let a535 = v2;
                                let a536 = 5u64;
                                let a537 = 0u64;
                                nat_rem(a535, a536, a537)
                            };
                            let a539 = 2u64;
                            if nat_eq(a538, a539) {
                                Ok(0u64)
                            } else {
                                let a543 = {
                                    let a540 = v2;
                                    let a541 = 5u64;
                                    let a542 = 0u64;
                                    nat_rem(a540, a541, a542)
                                };
                                let a544 = 3u64;
                                if nat_eq(a543, a544) {
                                    Ok(0u64)
                                } else {
                                    let a548 = {
                                        let a545 = v2;
                                        let a546 = 5u64;
                                        let a547 = 0u64;
                                        nat_rem(a545, a546, a547)
                                    };
                                    let a549 = 4u64;
                                    let _ = nat_eq(a548, a549);
                                    Ok(0u64)
                                }
                            }
                        }
                    }
                } else {
                    Ok(1065353216u64)
                }
            } else {
                let a550 = f11(v0);
                let a551 = 2u64;
                if nat_eq(a550, a551) {
                    let a552 = v3;
                    let a553 = 0u64;
                    if nat_eq(a552, a553) {
                        Ok(f29(v2))
                    } else {
                        Ok(f30(v2))
                    }
                } else {
                    let a554 = f11(v0);
                    let a555 = 3u64;
                    if if nat_eq(a554, a555) {
                        let a556 = v3;
                        let a557 = 1u64;
                        nat_eq(a556, a557)
                    } else {
                        false
                    } {
                        Ok(0u64)
                    } else {
                        let a558 = f11(v0);
                        let a559 = 4u64;
                        if nat_eq(a558, a559) {
                            let a567 = {
                                let a564 = {
                                    let a562 = v2;
                                    let a563 = {
                                        let a560 = v3;
                                        let a561 = 0u64;
                                        if nat_eq(a560, a561) {
                                            1u64
                                        } else {
                                            0u64
                                        }
                                    };
                                    nat_add(a562, a563)?
                                };
                                let a565 = 2u64;
                                let a566 = 0u64;
                                nat_rem(a564, a565, a566)
                            };
                            let a568 = 0u64;
                            if nat_eq(a567, a568) {
                                Ok(0u64)
                            } else {
                                Ok(2147483648u64)
                            }
                        } else {
                            let a569 = f11(v0);
                            let a570 = 5u64;
                            if nat_eq(a569, a570) {
                                Ok(f31(v2))
                            } else {
                                let a571 = f11(v0);
                                let a572 = 6u64;
                                if if nat_eq(a571, a572) {
                                    let a573 = v3;
                                    let a574 = 0u64;
                                    nat_eq(a573, a574)
                                } else {
                                    false
                                } {
                                    let a575 = v2;
                                    let a576 = 0u64;
                                    if nat_eq(a575, a576) {
                                        Ok(2139095041u64)
                                    } else {
                                        Ok(f32(v2))
                                    }
                                } else {
                                    let a577 = f11(v0);
                                    let a578 = 7u64;
                                    if nat_eq(a577, a578) {
                                        let a579 = v3;
                                        let a580 = 0u64;
                                        if nat_eq(a579, a580) {
                                            Ok(2139095039u64)
                                        } else {
                                            Ok(1065353216u64)
                                        }
                                    } else {
                                        Ok(f32({
                                            let a583 = v2;
                                            let a584 = {
                                                let a581 = v3;
                                                let a582 = 0u64;
                                                if nat_eq(a581, a582) {
                                                    0u64
                                                } else {
                                                    3u64
                                                }
                                            };
                                            nat_add(a583, a584)?
                                        }))
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

pub fn f26(v0: u64) -> u64 {
    let a585 = v0;
    let a586 = 320u64;
    if nat_lt(a585, a586) {
        let a587 = v0;
        let a588 = 80u64;
        let a589 = 0u64;
        nat_quot(a587, a588, a589)
    } else {
        if f15(v0) {
            let a592 = {
                let a590 = v0;
                let a591 = 320u64;
                nat_sub(a590, a591)
            };
            let a593 = 8u64;
            let a594 = 0u64;
            nat_quot(a592, a593, a594)
        } else {
            0u64
        }
    }
}

pub fn f27(v0: u64, v1: u64) -> u64 {
    let a595 = v1;
    let a596 = 3u64;
    if nat_eq(a595, a596) {
        let a597 = f10(v0);
        let a598 = 1u64;
        if if nat_eq(a597, a598) {
            true
        } else {
            let a599 = f10(v0);
            let a600 = 3u64;
            nat_eq(a599, a600)
        } {
            0u64
        } else {
            let a601 = f10(v0);
            let a602 = 2u64;
            if nat_eq(a601, a602) {
                1u64
            } else {
                3u64
            }
        }
    } else {
        let a603 = v1;
        let a604 = 1u64;
        if if nat_eq(a603, a604) {
            let a605 = f10(v0);
            let a606 = 3u64;
            nat_eq(a605, a606)
        } else {
            false
        } {
            0u64
        } else {
            v1
        }
    }
}

pub fn f28(v0: Adt0) -> R<u64> {
    let a627 = {
        let a619 = {
            let a611 = {
                let a608 = match v0.clone() {
                    Adt0::C0(_, h607, _, _, _, _, _, _, _, _) => {
                        h607
                    }
                };
                let a609 = 1u64;
                nat_sub(a608, a609)
            };
            let a612 = match v0.clone() {
                Adt0::C0(_, _, _, _, _, h610, _, _, _, _) => {
                    h610
                }
            };
            nat_mul(a611, a612)?
        };
        let a620 = {
            let a617 = {
                let a614 = match v0.clone() {
                    Adt0::C0(_, _, h613, _, _, _, _, _, _, _) => {
                        h613
                    }
                };
                let a615 = 1u64;
                nat_sub(a614, a615)
            };
            let a618 = match v0.clone() {
                Adt0::C0(_, _, _, _, _, _, h616, _, _, _) => {
                    h616
                }
            };
            nat_mul(a617, a618)?
        };
        nat_add(a619, a620)?
    };
    let a628 = {
        let a625 = {
            let a622 = match v0.clone() {
                Adt0::C0(_, _, _, h621, _, _, _, _, _, _) => {
                    h621
                }
            };
            let a623 = 1u64;
            nat_sub(a622, a623)
        };
        let a626 = match v0 {
            Adt0::C0(_, _, _, _, _, _, _, h624, _, _) => {
                h624
            }
        };
        nat_mul(a625, a626)?
    };
    nat_add(a627, a628)
}

pub fn f29(v0: u64) -> u64 {
    let a632 = {
        let a629 = v0;
        let a630 = 7u64;
        let a631 = 0u64;
        nat_rem(a629, a630, a631)
    };
    let a633 = 0u64;
    if nat_eq(a632, a633) {
        1036831949u64
    } else {
        let a637 = {
            let a634 = v0;
            let a635 = 7u64;
            let a636 = 0u64;
            nat_rem(a634, a635, a636)
        };
        let a638 = 1u64;
        if nat_eq(a637, a638) {
            3221225472u64
        } else {
            let a642 = {
                let a639 = v0;
                let a640 = 7u64;
                let a641 = 0u64;
                nat_rem(a639, a640, a641)
            };
            let a643 = 2u64;
            if nat_eq(a642, a643) {
                1077936128u64
            } else {
                let a647 = {
                    let a644 = v0;
                    let a645 = 7u64;
                    let a646 = 0u64;
                    nat_rem(a644, a645, a646)
                };
                let a648 = 3u64;
                if nat_eq(a647, a648) {
                    3229614080u64
                } else {
                    let a652 = {
                        let a649 = v0;
                        let a650 = 7u64;
                        let a651 = 0u64;
                        nat_rem(a649, a650, a651)
                    };
                    let a653 = 4u64;
                    if nat_eq(a652, a653) {
                        1008981770u64
                    } else {
                        let a657 = {
                            let a654 = v0;
                            let a655 = 7u64;
                            let a656 = 0u64;
                            nat_rem(a654, a655, a656)
                        };
                        let a658 = 5u64;
                        if nat_eq(a657, a658) {
                            1065353216u64
                        } else {
                            let a662 = {
                                let a659 = v0;
                                let a660 = 7u64;
                                let a661 = 0u64;
                                nat_rem(a659, a660, a661)
                            };
                            let a663 = 6u64;
                            let _ = nat_eq(a662, a663);
                            3212836864u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f30(v0: u64) -> u64 {
    let a667 = {
        let a664 = v0;
        let a665 = 7u64;
        let a666 = 0u64;
        nat_rem(a664, a665, a666)
    };
    let a668 = 0u64;
    if nat_eq(a667, a668) {
        1069547520u64
    } else {
        let a672 = {
            let a669 = v0;
            let a670 = 7u64;
            let a671 = 0u64;
            nat_rem(a669, a670, a671)
        };
        let a673 = 1u64;
        if nat_eq(a672, a673) {
            1078530011u64
        } else {
            let a677 = {
                let a674 = v0;
                let a675 = 7u64;
                let a676 = 0u64;
                nat_rem(a674, a675, a676)
            };
            let a678 = 2u64;
            if nat_eq(a677, a678) {
                3212836865u64
            } else {
                let a682 = {
                    let a679 = v0;
                    let a680 = 7u64;
                    let a681 = 0u64;
                    nat_rem(a679, a680, a681)
                };
                let a683 = 3u64;
                if nat_eq(a682, a683) {
                    3204448256u64
                } else {
                    let a687 = {
                        let a684 = v0;
                        let a685 = 7u64;
                        let a686 = 0u64;
                        nat_rem(a684, a685, a686)
                    };
                    let a688 = 4u64;
                    if nat_eq(a687, a688) {
                        1075838976u64
                    } else {
                        let a692 = {
                            let a689 = v0;
                            let a690 = 7u64;
                            let a691 = 0u64;
                            nat_rem(a689, a690, a691)
                        };
                        let a693 = 5u64;
                        if nat_eq(a692, a693) {
                            1036831949u64
                        } else {
                            let a697 = {
                                let a694 = v0;
                                let a695 = 7u64;
                                let a696 = 0u64;
                                nat_rem(a694, a695, a696)
                            };
                            let a698 = 6u64;
                            let _ = nat_eq(a697, a698);
                            1051372203u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f31(v0: u64) -> u64 {
    let a702 = {
        let a699 = v0;
        let a700 = 5u64;
        let a701 = 0u64;
        nat_rem(a699, a700, a701)
    };
    let a703 = 2u64;
    if nat_lt(a702, a703) {
        let a707 = {
            let a704 = v0;
            let a705 = 5u64;
            let a706 = 0u64;
            nat_rem(a704, a705, a706)
        };
        let a708 = 1u64;
        if nat_lt(a707, a708) {
            2139095040u64
        } else {
            4286578688u64
        }
    } else {
        let a712 = {
            let a709 = v0;
            let a710 = 5u64;
            let a711 = 0u64;
            nat_rem(a709, a710, a711)
        };
        let a713 = 3u64;
        if nat_lt(a712, a713) {
            1065353216u64
        } else {
            let a717 = {
                let a714 = v0;
                let a715 = 5u64;
                let a716 = 0u64;
                nat_rem(a714, a715, a716)
            };
            let a718 = 4u64;
            if nat_lt(a717, a718) {
                3212836864u64
            } else {
                0u64
            }
        }
    }
}

pub fn f32(v0: u64) -> u64 {
    let a722 = {
        let a719 = v0;
        let a720 = 8u64;
        let a721 = 0u64;
        nat_rem(a719, a720, a721)
    };
    let a723 = 4u64;
    if nat_lt(a722, a723) {
        let a727 = {
            let a724 = v0;
            let a725 = 8u64;
            let a726 = 0u64;
            nat_rem(a724, a725, a726)
        };
        let a728 = 2u64;
        if nat_lt(a727, a728) {
            let a732 = {
                let a729 = v0;
                let a730 = 8u64;
                let a731 = 0u64;
                nat_rem(a729, a730, a731)
            };
            let a733 = 1u64;
            if nat_lt(a732, a733) {
                3229614080u64
            } else {
                3221225472u64
            }
        } else {
            let a737 = {
                let a734 = v0;
                let a735 = 8u64;
                let a736 = 0u64;
                nat_rem(a734, a735, a736)
            };
            let a738 = 3u64;
            if nat_lt(a737, a738) {
                3212836864u64
            } else {
                2147483648u64
            }
        }
    } else {
        let a742 = {
            let a739 = v0;
            let a740 = 8u64;
            let a741 = 0u64;
            nat_rem(a739, a740, a741)
        };
        let a743 = 6u64;
        if nat_lt(a742, a743) {
            let a747 = {
                let a744 = v0;
                let a745 = 8u64;
                let a746 = 0u64;
                nat_rem(a744, a745, a746)
            };
            let a748 = 5u64;
            if nat_lt(a747, a748) {
                0u64
            } else {
                1065353216u64
            }
        } else {
            let a752 = {
                let a749 = v0;
                let a750 = 8u64;
                let a751 = 0u64;
                nat_rem(a749, a750, a751)
            };
            let a753 = 7u64;
            if nat_lt(a752, a753) {
                1073741824u64
            } else {
                1082130432u64
            }
        }
    }
}
