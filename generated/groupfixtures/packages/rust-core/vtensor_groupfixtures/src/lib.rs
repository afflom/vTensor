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
        let a6 = 224u64;
        if nat_lt(a5, a6) {
            let a7 = v0;
            let a8 = 1u64;
            if nat_eq(a7, a8) {
                let a9 = v2;
                let a10 = 0u64;
                if if nat_eq(a9, a10) {
                    let a11 = v3;
                    let a12 = 11u64;
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
                                f3(v1, 0u64)
                            } else {
                                let a25 = v3;
                                let a26 = 1u64;
                                if nat_eq(a25, a26) {
                                    f3(v1, 1u64)
                                } else {
                                    let a27 = v3;
                                    let a28 = 2u64;
                                    if nat_eq(a27, a28) {
                                        f3(v1, 2u64)
                                    } else {
                                        let a29 = v3;
                                        let a30 = 3u64;
                                        if nat_eq(a29, a30) {
                                            f3(v1, 3u64)
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
                                                                f6(v1)?
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
                        let a46 = 0u64;
                        if if nat_eq(a45, a46) {
                            let a47 = v3;
                            let a48 = 512u64;
                            nat_lt(a47, a48)
                        } else {
                            false
                        } {
                            Ok(Some(f7(v1, v3)?))
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
                                        Ok(Some(f8(v1)?))
                                    } else {
                                        Ok(None::<u64>)
                                    }
                                } else {
                                    let a57 = v2;
                                    let a58 = f8(v1)?;
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
    let a73 = v0;
    let a74 = 0u64;
    if nat_eq(a73, a74) {
        224u64
    } else {
        let a75 = v0;
        let a76 = 1u64;
        if nat_eq(a75, a76) {
            11u64
        } else {
            let a77 = v0;
            let a78 = 2u64;
            if nat_eq(a77, a78) {
                224u64
            } else {
                let a79 = v0;
                let a80 = 3u64;
                if nat_eq(a79, a80) {
                    512u64
                } else {
                    let a81 = v0;
                    let a82 = 4u64;
                    if nat_eq(a81, a82) {
                        165u64
                    } else {
                        let a83 = v0;
                        let a84 = 5u64;
                        let _ = nat_eq(a83, a84);
                        112u64
                    }
                }
            }
        }
    }
}

pub fn f2(v0: u64, v1: u64) -> u64 {
    let a85 = v1;
    let a86 = 0u64;
    if nat_eq(a85, a86) {
        f9(v0)
    } else {
        let a87 = v1;
        let a88 = 1u64;
        if nat_eq(a87, a88) {
            f10(v0)
        } else {
            let a89 = v1;
            let a90 = 2u64;
            if nat_eq(a89, a90) {
                f11(v0)
            } else {
                let a91 = v1;
                let a92 = 3u64;
                if nat_eq(a91, a92) {
                    f12(v0)
                } else {
                    let a93 = v1;
                    let a94 = 4u64;
                    if nat_eq(a93, a94) {
                        f3(v0, 0u64)
                    } else {
                        let a95 = v1;
                        let a96 = 5u64;
                        if nat_eq(a95, a96) {
                            f3(v0, 1u64)
                        } else {
                            let a97 = v1;
                            let a98 = 6u64;
                            if nat_eq(a97, a98) {
                                f3(v0, 3u64)
                            } else {
                                let a99 = v1;
                                let a100 = 7u64;
                                if nat_eq(a99, a100) {
                                    v0
                                } else {
                                    let a101 = v1;
                                    let a102 = 8u64;
                                    if nat_eq(a101, a102) {
                                        f13(v0)
                                    } else {
                                        let a103 = v1;
                                        let a104 = 9u64;
                                        if nat_eq(a103, a104) {
                                            f14(v0)
                                        } else {
                                            let a105 = v1;
                                            let a106 = 10u64;
                                            let _ = nat_eq(a105, a106);
                                            f15(v0)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f3(v0: u64, v1: u64) -> u64 {
    if if f16(v0) {
        let a107 = v1;
        let a108 = f17(v0);
        nat_eq(a107, a108)
    } else {
        false
    } {
        0u64
    } else {
        let a109 = v1;
        let a110 = 0u64;
        if nat_eq(a109, a110) {
            f18(v0)
        } else {
            let a111 = v1;
            let a112 = 1u64;
            if nat_eq(a111, a112) {
                let a113 = f12(v0);
                let a114 = 4u64;
                if nat_eq(a113, a114) {
                    3u64
                } else {
                    2u64
                }
            } else {
                let a115 = v1;
                let a116 = 2u64;
                if nat_eq(a115, a116) {
                    5u64
                } else {
                    let a117 = v1;
                    let a118 = 3u64;
                    let _ = nat_eq(a117, a118);
                    2u64
                }
            }
        }
    }
}

pub fn f4(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a119 = v2;
    let a120 = 0u64;
    if nat_eq(a119, a120) {
        f19(v0, v1)
    } else {
        let a121 = v2;
        let a122 = 1u64;
        if nat_eq(a121, a122) {
            f20(v0, v1)
        } else {
            let a123 = v2;
            let a124 = 2u64;
            if nat_eq(a123, a124) {
                f21(v0, v1)
            } else {
                let a125 = v2;
                let a126 = 3u64;
                let _ = nat_eq(a125, a126);
                f22(v0, v1)
            }
        }
    }
}

pub fn f5(v0: u64, v1: u64) -> R<u64> {
    let m127 = f23(Adt0::C0(f3(v0, 0u64), f3(v0, 1u64), f3(v0, 2u64), f3(v0, 3u64), f4(v0, v1, 0u64)?, f4(v0, v1, 1u64)?, f4(v0, v1, 2u64)?, f4(v0, v1, 3u64)?, 1u64, f24(v0)))?;
    match m127 {
        Adt1::C0 => {
            Ok(0u64)
        }
        Adt1::C1(v2) => {
            Ok(v2)
        }
    }
}

pub fn f6(v0: u64) -> R<u64> {
    let a132 = {
        let a128 = f3(v0, 0u64);
        let a129 = f3(v0, 1u64);
        nat_mul(a128, a129)?
    };
    let a133 = {
        let a130 = f3(v0, 2u64);
        let a131 = f3(v0, 3u64);
        nat_mul(a130, a131)?
    };
    nat_mul(a132, a133)
}

pub fn f7(v0: u64, v1: u64) -> R<u64> {
    let a160 = {
        let a157 = f25(v0, {
            let a134 = v1;
            let a135 = f24(v0);
            let a136 = 0u64;
            nat_quot(a134, a135, a136)
        })?;
        let a158 = {
            let a140 = {
                let a137 = v1;
                let a138 = f24(v0);
                let a139 = 0u64;
                nat_rem(a137, a138, a139)
            };
            let a141 = 0u64;
            if nat_eq(a140, a141) {
                1u64
            } else {
                let a145 = {
                    let a142 = v1;
                    let a143 = f24(v0);
                    let a144 = 0u64;
                    nat_rem(a142, a143, a144)
                };
                let a146 = 1u64;
                if nat_eq(a145, a146) {
                    256u64
                } else {
                    let a150 = {
                        let a147 = v1;
                        let a148 = f24(v0);
                        let a149 = 0u64;
                        nat_rem(a147, a148, a149)
                    };
                    let a151 = 2u64;
                    if nat_eq(a150, a151) {
                        65536u64
                    } else {
                        let a155 = {
                            let a152 = v1;
                            let a153 = f24(v0);
                            let a154 = 0u64;
                            nat_rem(a152, a153, a154)
                        };
                        let a156 = 3u64;
                        let _ = nat_eq(a155, a156);
                        16777216u64
                    }
                }
            }
        };
        let a159 = 0u64;
        nat_quot(a157, a158, a159)
    };
    let a161 = 256u64;
    let a162 = 0u64;
    Ok(nat_rem(a160, a161, a162))
}

pub fn f8(v0: u64) -> R<u64> {
    let a165 = {
        let a163 = f5(v0, 0u64)?;
        let a164 = 511u64;
        nat_add(a163, a164)?
    };
    let a166 = 512u64;
    let a167 = 0u64;
    Ok(nat_quot(a165, a166, a167))
}

pub fn f9(v0: u64) -> u64 {
    let a168 = f11(v0);
    let a169 = 14u64;
    if nat_eq(a168, a169) {
        1u64
    } else {
        0u64
    }
}

pub fn f10(v0: u64) -> u64 {
    let a170 = v0;
    let a171 = 200u64;
    if nat_lt(a170, a171) {
        let a172 = v0;
        let a173 = 100u64;
        let a174 = 0u64;
        nat_quot(a172, a173, a174)
    } else {
        let a177 = {
            let a175 = v0;
            let a176 = 200u64;
            nat_sub(a175, a176)
        };
        let a178 = 12u64;
        let a179 = 0u64;
        nat_quot(a177, a178, a179)
    }
}

pub fn f11(v0: u64) -> u64 {
    let a180 = v0;
    let a181 = 200u64;
    if nat_lt(a180, a181) {
        let a182 = v0;
        let a183 = 20u64;
        let a184 = 0u64;
        nat_rem(a182, a183, a184)
    } else {
        20u64
    }
}

pub fn f12(v0: u64) -> u64 {
    let a185 = v0;
    let a186 = 200u64;
    if nat_lt(a185, a186) {
        let a190 = {
            let a187 = v0;
            let a188 = 100u64;
            let a189 = 0u64;
            nat_rem(a187, a188, a189)
        };
        let a191 = 20u64;
        let a192 = 0u64;
        nat_quot(a190, a191, a192)
    } else {
        0u64
    }
}

pub fn f13(v0: u64) -> u64 {
    if f16(v0) {
        0u64
    } else {
        let a193 = f12(v0);
        let a194 = 1u64;
        if if if if if if if if nat_eq(a193, a194) {
            true
        } else {
            let a195 = f12(v0);
            let a196 = 2u64;
            if nat_eq(a195, a196) {
                let a197 = f10(v0);
                let a198 = 1u64;
                nat_eq(a197, a198)
            } else {
                false
            }
        } {
            true
        } else {
            let a199 = f11(v0);
            let a200 = 11u64;
            nat_eq(a199, a200)
        } {
            true
        } else {
            let a201 = f11(v0);
            let a202 = 12u64;
            nat_eq(a201, a202)
        } {
            true
        } else {
            let a203 = f11(v0);
            let a204 = 13u64;
            nat_eq(a203, a204)
        } {
            true
        } else {
            let a205 = f11(v0);
            let a206 = 14u64;
            nat_eq(a205, a206)
        } {
            true
        } else {
            let a207 = f11(v0);
            let a208 = 15u64;
            nat_eq(a207, a208)
        } {
            true
        } else {
            let a209 = f11(v0);
            let a210 = 18u64;
            nat_eq(a209, a210)
        } {
            2u64
        } else {
            let a211 = f11(v0);
            let a212 = 16u64;
            if if nat_eq(a211, a212) {
                true
            } else {
                let a213 = f11(v0);
                let a214 = 17u64;
                if nat_eq(a213, a214) {
                    let a215 = f10(v0);
                    let a216 = 1u64;
                    nat_eq(a215, a216)
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
    if f16(v0) {
        let a222 = {
            let a219 = {
                let a217 = v0;
                let a218 = 200u64;
                nat_sub(a217, a218)
            };
            let a220 = 3u64;
            let a221 = 0u64;
            nat_rem(a219, a220, a221)
        };
        let a223 = 0u64;
        if nat_eq(a222, a223) {
            2143289345u64
        } else {
            let a229 = {
                let a226 = {
                    let a224 = v0;
                    let a225 = 200u64;
                    nat_sub(a224, a225)
                };
                let a227 = 3u64;
                let a228 = 0u64;
                nat_rem(a226, a227, a228)
            };
            let a230 = 1u64;
            if nat_eq(a229, a230) {
                3212836864u64
            } else {
                let a236 = {
                    let a233 = {
                        let a231 = v0;
                        let a232 = 200u64;
                        nat_sub(a231, a232)
                    };
                    let a234 = 3u64;
                    let a235 = 0u64;
                    nat_rem(a233, a234, a235)
                };
                let a237 = 2u64;
                let _ = nat_eq(a236, a237);
                2139095040u64
            }
        }
    } else {
        let a238 = f11(v0);
        let a239 = 0u64;
        if nat_eq(a238, a239) {
            897988541u64
        } else {
            let a240 = f11(v0);
            let a241 = 1u64;
            if nat_eq(a240, a241) {
                0u64
            } else {
                let a242 = f11(v0);
                let a243 = 2u64;
                if nat_eq(a242, a243) {
                    0u64
                } else {
                    let a244 = f11(v0);
                    let a245 = 3u64;
                    if nat_eq(a244, a245) {
                        0u64
                    } else {
                        let a246 = f11(v0);
                        let a247 = 4u64;
                        if nat_eq(a246, a247) {
                            2147483648u64
                        } else {
                            let a248 = f11(v0);
                            let a249 = 5u64;
                            if nat_eq(a248, a249) {
                                0u64
                            } else {
                                let a250 = f11(v0);
                                let a251 = 6u64;
                                if nat_eq(a250, a251) {
                                    0u64
                                } else {
                                    let a252 = f11(v0);
                                    let a253 = 7u64;
                                    if nat_eq(a252, a253) {
                                        3212836864u64
                                    } else {
                                        let a254 = f11(v0);
                                        let a255 = 8u64;
                                        if nat_eq(a254, a255) {
                                            2143289345u64
                                        } else {
                                            let a256 = f11(v0);
                                            let a257 = 9u64;
                                            if nat_eq(a256, a257) {
                                                2139095040u64
                                            } else {
                                                let a258 = f11(v0);
                                                let a259 = 10u64;
                                                if nat_eq(a258, a259) {
                                                    1u64
                                                } else {
                                                    let a260 = f11(v0);
                                                    let a261 = 11u64;
                                                    if nat_eq(a260, a261) {
                                                        0u64
                                                    } else {
                                                        let a262 = f11(v0);
                                                        let a263 = 12u64;
                                                        if nat_eq(a262, a263) {
                                                            0u64
                                                        } else {
                                                            let a264 = f11(v0);
                                                            let a265 = 13u64;
                                                            if nat_eq(a264, a265) {
                                                                0u64
                                                            } else {
                                                                let a266 = f11(v0);
                                                                let a267 = 14u64;
                                                                if nat_eq(a266, a267) {
                                                                    0u64
                                                                } else {
                                                                    let a268 = f11(v0);
                                                                    let a269 = 15u64;
                                                                    if nat_eq(a268, a269) {
                                                                        0u64
                                                                    } else {
                                                                        let a270 = f11(v0);
                                                                        let a271 = 16u64;
                                                                        if nat_eq(a270, a271) {
                                                                            0u64
                                                                        } else {
                                                                            let a272 = f11(v0);
                                                                            let a273 = 17u64;
                                                                            if nat_eq(a272, a273) {
                                                                                0u64
                                                                            } else {
                                                                                let a274 = f11(v0);
                                                                                let a275 = 18u64;
                                                                                if nat_eq(a274, a275) {
                                                                                    0u64
                                                                                } else {
                                                                                    let a276 = f11(v0);
                                                                                    let a277 = 19u64;
                                                                                    if nat_eq(a276, a277) {
                                                                                        0u64
                                                                                    } else {
                                                                                        let a278 = f11(v0);
                                                                                        let a279 = 20u64;
                                                                                        let _ = nat_eq(a278, a279);
                                                                                        0u64
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f15(v0: u64) -> u64 {
    if f16(v0) {
        let a285 = {
            let a282 = {
                let a280 = v0;
                let a281 = 200u64;
                nat_sub(a280, a281)
            };
            let a283 = 3u64;
            let a284 = 0u64;
            nat_rem(a282, a283, a284)
        };
        let a286 = 0u64;
        if nat_eq(a285, a286) {
            0u64
        } else {
            let a292 = {
                let a289 = {
                    let a287 = v0;
                    let a288 = 200u64;
                    nat_sub(a287, a288)
                };
                let a290 = 3u64;
                let a291 = 0u64;
                nat_rem(a289, a290, a291)
            };
            let a293 = 1u64;
            if nat_eq(a292, a293) {
                4294967295u64
            } else {
                let a299 = {
                    let a296 = {
                        let a294 = v0;
                        let a295 = 200u64;
                        nat_sub(a294, a295)
                    };
                    let a297 = 3u64;
                    let a298 = 0u64;
                    nat_rem(a296, a297, a298)
                };
                let a300 = 2u64;
                let _ = nat_eq(a299, a300);
                8u64
            }
        }
    } else {
        let a301 = f11(v0);
        let a302 = 0u64;
        if nat_eq(a301, a302) {
            2u64
        } else {
            let a303 = f11(v0);
            let a304 = 1u64;
            if nat_eq(a303, a304) {
                4u64
            } else {
                let a305 = f11(v0);
                let a306 = 2u64;
                if nat_eq(a305, a306) {
                    8u64
                } else {
                    let a307 = f11(v0);
                    let a308 = 3u64;
                    if nat_eq(a307, a308) {
                        1u64
                    } else {
                        let a309 = f11(v0);
                        let a310 = 4u64;
                        if nat_eq(a309, a310) {
                            2u64
                        } else {
                            let a311 = f11(v0);
                            let a312 = 5u64;
                            if nat_eq(a311, a312) {
                                2u64
                            } else {
                                let a313 = f11(v0);
                                let a314 = 6u64;
                                if nat_eq(a313, a314) {
                                    3u64
                                } else {
                                    let a315 = f11(v0);
                                    let a316 = 7u64;
                                    if nat_eq(a315, a316) {
                                        2u64
                                    } else {
                                        let a317 = f11(v0);
                                        let a318 = 8u64;
                                        if nat_eq(a317, a318) {
                                            2u64
                                        } else {
                                            let a319 = f11(v0);
                                            let a320 = 9u64;
                                            if nat_eq(a319, a320) {
                                                2u64
                                            } else {
                                                let a321 = f11(v0);
                                                let a322 = 10u64;
                                                if nat_eq(a321, a322) {
                                                    2u64
                                                } else {
                                                    let a323 = f11(v0);
                                                    let a324 = 11u64;
                                                    if nat_eq(a323, a324) {
                                                        0u64
                                                    } else {
                                                        let a325 = f11(v0);
                                                        let a326 = 12u64;
                                                        if nat_eq(a325, a326) {
                                                            4294967295u64
                                                        } else {
                                                            let a327 = f11(v0);
                                                            let a328 = 13u64;
                                                            if nat_eq(a327, a328) {
                                                                2147483647u64
                                                            } else {
                                                                let a329 = f11(v0);
                                                                let a330 = 14u64;
                                                                if nat_eq(a329, a330) {
                                                                    2u64
                                                                } else {
                                                                    let a331 = f11(v0);
                                                                    let a332 = 15u64;
                                                                    if nat_eq(a331, a332) {
                                                                        2u64
                                                                    } else {
                                                                        let a333 = f11(v0);
                                                                        let a334 = 16u64;
                                                                        if nat_eq(a333, a334) {
                                                                            2u64
                                                                        } else {
                                                                            let a335 = f11(v0);
                                                                            let a336 = 17u64;
                                                                            if nat_eq(a335, a336) {
                                                                                2u64
                                                                            } else {
                                                                                let a337 = f11(v0);
                                                                                let a338 = 18u64;
                                                                                if nat_eq(a337, a338) {
                                                                                    2u64
                                                                                } else {
                                                                                    let a339 = f11(v0);
                                                                                    let a340 = 19u64;
                                                                                    if nat_eq(a339, a340) {
                                                                                        1u64
                                                                                    } else {
                                                                                        let a341 = f11(v0);
                                                                                        let a342 = 20u64;
                                                                                        let _ = nat_eq(a341, a342);
                                                                                        0u64
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f16(v0: u64) -> bool {
    let a343 = 200u64;
    let a344 = v0;
    nat_le(a343, a344)
}

pub fn f17(v0: u64) -> u64 {
    let a350 = {
        let a347 = {
            let a345 = v0;
            let a346 = 200u64;
            nat_sub(a345, a346)
        };
        let a348 = 12u64;
        let a349 = 0u64;
        nat_rem(a347, a348, a349)
    };
    let a351 = 3u64;
    let a352 = 0u64;
    nat_quot(a350, a351, a352)
}

pub fn f18(v0: u64) -> u64 {
    let a353 = f11(v0);
    let a354 = 9u64;
    if nat_eq(a353, a354) {
        1u64
    } else {
        let a355 = f11(v0);
        let a356 = 10u64;
        if nat_eq(a355, a356) {
            7u64
        } else {
            let a357 = f11(v0);
            let a358 = 2u64;
            if nat_eq(a357, a358) {
                5u64
            } else {
                3u64
            }
        }
    }
}

pub fn f19(v0: u64, v1: u64) -> R<u64> {
    let a369 = f24(v0);
    let a370 = if f26(v0, v1) {
        let a359 = f11(v0);
        let a360 = 15u64;
        if if if nat_eq(a359, a360) {
            true
        } else {
            let a361 = f12(v0);
            let a362 = 1u64;
            nat_eq(a361, a362)
        } {
            true
        } else {
            let a363 = f12(v0);
            let a364 = 2u64;
            if nat_eq(a363, a364) {
                let a365 = f10(v0);
                let a366 = 1u64;
                nat_eq(a365, a366)
            } else {
                false
            }
        } {
            2u64
        } else {
            1u64
        }
    } else {
        let a367 = f12(v0);
        let a368 = 2u64;
        if nat_eq(a367, a368) {
            2u64
        } else {
            1u64
        }
    };
    nat_mul(a369, a370)
}

pub fn f20(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a371 = f12(v0);
        let a372 = 4u64;
        if nat_eq(a371, a372) {
            100u64
        } else {
            let a373 = f12(v0);
            let a374 = 3u64;
            if nat_eq(a373, a374) {
                240u64
            } else {
                let a377 = {
                    let a375 = f19(v0, v1)?;
                    let a376 = f3(v0, 0u64);
                    nat_mul(a375, a376)?
                };
                let a378 = f24(v0);
                nat_add(a377, a378)?
            }
        }
    };
    let a379 = f11(v0);
    let a380 = 16u64;
    if if if nat_eq(a379, a380) {
        if !f26(v0, v1) {
            true
        } else {
            let a381 = f10(v0);
            let a382 = 1u64;
            nat_eq(a381, a382)
        }
    } else {
        false
    } {
        true
    } else {
        let a383 = f11(v0);
        let a384 = 17u64;
        if nat_eq(a383, a384) {
            f26(v0, v1)
        } else {
            false
        }
    } {
        Ok(0u64)
    } else {
        let a385 = f11(v0);
        let a386 = 18u64;
        if if nat_eq(a385, a386) {
            if !f26(v0, v1) {
                true
            } else {
                let a387 = f10(v0);
                let a388 = 1u64;
                nat_eq(a387, a388)
            }
        } else {
            false
        } {
            let a389 = v2;
            let a390 = 1u64;
            nat_add(a389, a390)
        } else {
            Ok(v2)
        }
    }
}

pub fn f21(v0: u64, v1: u64) -> R<u64> {
    let a391 = f12(v0);
    let a392 = 4u64;
    if nat_eq(a391, a392) {
        Ok(140u64)
    } else {
        let a393 = f12(v0);
        let a394 = 3u64;
        if nat_eq(a393, a394) {
            Ok(40u64)
        } else {
            let a397 = {
                let a395 = f20(v0, v1)?;
                let a396 = f3(v0, 1u64);
                nat_mul(a395, a396)?
            };
            let a398 = f24(v0);
            nat_add(a397, a398)
        }
    }
}

pub fn f22(v0: u64, v1: u64) -> R<u64> {
    let a399 = f12(v0);
    let a400 = 4u64;
    if nat_eq(a399, a400) {
        Ok(1000u64)
    } else {
        let a401 = f12(v0);
        let a402 = 3u64;
        if nat_eq(a401, a402) {
            Ok(600u64)
        } else {
            let a405 = {
                let a403 = f21(v0, v1)?;
                let a404 = f3(v0, 2u64);
                nat_mul(a403, a404)?
            };
            let a406 = f24(v0);
            nat_add(a405, a406)
        }
    }
}

pub fn f23(v0: Adt0) -> R<Adt1> {
    let a408 = match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h407, _) => {
            h407
        }
    };
    let a409 = 0u64;
    if nat_eq(a408, a409) {
        Ok(Adt1::C0)
    } else {
        let a411 = match v0.clone() {
            Adt0::C0(h410, _, _, _, _, _, _, _, _, _) => {
                h410
            }
        };
        let a412 = 0u64;
        if if if if nat_eq(a411, a412) {
            true
        } else {
            let a414 = match v0.clone() {
                Adt0::C0(_, h413, _, _, _, _, _, _, _, _) => {
                    h413
                }
            };
            let a415 = 0u64;
            nat_eq(a414, a415)
        } {
            true
        } else {
            let a417 = match v0.clone() {
                Adt0::C0(_, _, h416, _, _, _, _, _, _, _) => {
                    h416
                }
            };
            let a418 = 0u64;
            nat_eq(a417, a418)
        } {
            true
        } else {
            let a420 = match v0.clone() {
                Adt0::C0(_, _, _, h419, _, _, _, _, _, _) => {
                    h419
                }
            };
            let a421 = 0u64;
            nat_eq(a420, a421)
        } {
            Ok(Adt1::C1(0u64))
        } else {
            let a423 = match v0.clone() {
                Adt0::C0(_, _, _, _, _, _, _, _, h422, _) => {
                    h422
                }
            };
            let a424 = 1u64;
            if nat_eq(a423, a424) {
                Ok(Adt1::C1({
                    let a434 = {
                        let a432 = match v0.clone() {
                            Adt0::C0(_, _, _, _, _, _, _, _, _, h425) => {
                                h425
                            }
                        };
                        let a433 = {
                            let a430 = {
                                let a427 = match v0.clone() {
                                    Adt0::C0(h426, _, _, _, _, _, _, _, _, _) => {
                                        h426
                                    }
                                };
                                let a428 = 1u64;
                                nat_sub(a427, a428)
                            };
                            let a431 = match v0.clone() {
                                Adt0::C0(_, _, _, _, h429, _, _, _, _, _) => {
                                    h429
                                }
                            };
                            nat_mul(a430, a431)?
                        };
                        nat_add(a432, a433)?
                    };
                    let a435 = f27(v0)?;
                    nat_add(a434, a435)?
                }))
            } else {
                Ok(Adt1::C1({
                    let a444 = {
                        let a441 = {
                            let a438 = match v0.clone() {
                                Adt0::C0(h436, _, _, _, _, _, _, _, _, _) => {
                                    h436
                                }
                            };
                            let a439 = match v0.clone() {
                                Adt0::C0(_, _, _, _, h437, _, _, _, _, _) => {
                                    h437
                                }
                            };
                            nat_mul(a438, a439)?
                        };
                        let a442 = match v0.clone() {
                            Adt0::C0(_, _, _, _, _, _, _, _, h440, _) => {
                                h440
                            }
                        };
                        let a443 = 0u64;
                        nat_quot(a441, a442, a443)
                    };
                    let a445 = f27(v0)?;
                    nat_add(a444, a445)?
                }))
            }
        }
    }
}

pub fn f24(v0: u64) -> u64 {
    let a446 = f9(v0);
    let a447 = 1u64;
    if nat_eq(a446, a447) {
        2u64
    } else {
        4u64
    }
}

pub fn f25(v0: u64, v1: u64) -> R<u64> {
    let a448 = f9(v0);
    let a449 = 1u64;
    if nat_eq(a448, a449) {
        Ok(15360u64)
    } else {
        let a450 = f11(v0);
        let a451 = 0u64;
        if nat_eq(a450, a451) {
            let a452 = v1;
            let a453 = 0u64;
            if nat_eq(a452, a453) {
                Ok(1568669696u64)
            } else {
                let a457 = v1;
                let a458 = {
                    let a454 = f20(v0, 0u64)?;
                    let a455 = 4u64;
                    let a456 = 0u64;
                    nat_quot(a454, a455, a456)
                };
                if nat_eq(a457, a458) {
                    Ok(3716153344u64)
                } else {
                    let a464 = v1;
                    let a465 = {
                        let a462 = {
                            let a459 = f20(v0, 0u64)?;
                            let a460 = 4u64;
                            let a461 = 0u64;
                            nat_quot(a459, a460, a461)
                        };
                        let a463 = 1u64;
                        nat_add(a462, a463)?
                    };
                    if nat_eq(a464, a465) {
                        Ok(1065353216u64)
                    } else {
                        Ok(0u64)
                    }
                }
            }
        } else {
            let a466 = f11(v0);
            let a467 = 3u64;
            if nat_eq(a466, a467) {
                Ok(0u64)
            } else {
                let a468 = f11(v0);
                let a469 = 4u64;
                if nat_eq(a468, a469) {
                    let a473 = {
                        let a470 = v1;
                        let a471 = 2u64;
                        let a472 = 0u64;
                        nat_rem(a470, a471, a472)
                    };
                    let a474 = 0u64;
                    if nat_eq(a473, a474) {
                        Ok(0u64)
                    } else {
                        Ok(2147483648u64)
                    }
                } else {
                    let a475 = f11(v0);
                    let a476 = 5u64;
                    if nat_eq(a475, a476) {
                        Ok(f28(v1))
                    } else {
                        let a477 = f11(v0);
                        let a478 = 6u64;
                        if if nat_eq(a477, a478) {
                            let a479 = v1;
                            let a480 = 0u64;
                            nat_eq(a479, a480)
                        } else {
                            false
                        } {
                            Ok(2139095041u64)
                        } else {
                            let a481 = f11(v0);
                            let a482 = 10u64;
                            if nat_eq(a481, a482) {
                                Ok(f29(v1))
                            } else {
                                let a483 = f11(v0);
                                let a484 = 19u64;
                                if nat_eq(a483, a484) {
                                    Ok(1065353216u64)
                                } else {
                                    Ok(f30(v1))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f26(v0: u64, v1: u64) -> bool {
    let a485 = v1;
    let a486 = 0u64;
    if nat_eq(a485, a486) {
        true
    } else {
        let a487 = v1;
        let a488 = 3u64;
        if nat_eq(a487, a488) {
            let a489 = f10(v0);
            let a490 = 1u64;
            nat_eq(a489, a490)
        } else {
            false
        }
    }
}

pub fn f27(v0: Adt0) -> R<u64> {
    let a511 = {
        let a503 = {
            let a495 = {
                let a492 = match v0.clone() {
                    Adt0::C0(_, h491, _, _, _, _, _, _, _, _) => {
                        h491
                    }
                };
                let a493 = 1u64;
                nat_sub(a492, a493)
            };
            let a496 = match v0.clone() {
                Adt0::C0(_, _, _, _, _, h494, _, _, _, _) => {
                    h494
                }
            };
            nat_mul(a495, a496)?
        };
        let a504 = {
            let a501 = {
                let a498 = match v0.clone() {
                    Adt0::C0(_, _, h497, _, _, _, _, _, _, _) => {
                        h497
                    }
                };
                let a499 = 1u64;
                nat_sub(a498, a499)
            };
            let a502 = match v0.clone() {
                Adt0::C0(_, _, _, _, _, _, h500, _, _, _) => {
                    h500
                }
            };
            nat_mul(a501, a502)?
        };
        nat_add(a503, a504)?
    };
    let a512 = {
        let a509 = {
            let a506 = match v0.clone() {
                Adt0::C0(_, _, _, h505, _, _, _, _, _, _) => {
                    h505
                }
            };
            let a507 = 1u64;
            nat_sub(a506, a507)
        };
        let a510 = match v0 {
            Adt0::C0(_, _, _, _, _, _, _, h508, _, _) => {
                h508
            }
        };
        nat_mul(a509, a510)?
    };
    nat_add(a511, a512)
}

pub fn f28(v0: u64) -> u64 {
    let a516 = {
        let a513 = v0;
        let a514 = 5u64;
        let a515 = 0u64;
        nat_rem(a513, a514, a515)
    };
    let a517 = 2u64;
    if nat_lt(a516, a517) {
        let a521 = {
            let a518 = v0;
            let a519 = 5u64;
            let a520 = 0u64;
            nat_rem(a518, a519, a520)
        };
        let a522 = 1u64;
        if nat_lt(a521, a522) {
            2139095040u64
        } else {
            4286578688u64
        }
    } else {
        let a526 = {
            let a523 = v0;
            let a524 = 5u64;
            let a525 = 0u64;
            nat_rem(a523, a524, a525)
        };
        let a527 = 3u64;
        if nat_lt(a526, a527) {
            1065353216u64
        } else {
            let a531 = {
                let a528 = v0;
                let a529 = 5u64;
                let a530 = 0u64;
                nat_rem(a528, a529, a530)
            };
            let a532 = 4u64;
            if nat_lt(a531, a532) {
                3212836864u64
            } else {
                0u64
            }
        }
    }
}

pub fn f29(v0: u64) -> u64 {
    let a536 = {
        let a533 = v0;
        let a534 = 4u64;
        let a535 = 0u64;
        nat_rem(a533, a534, a535)
    };
    let a537 = 2u64;
    if nat_lt(a536, a537) {
        let a541 = {
            let a538 = v0;
            let a539 = 4u64;
            let a540 = 0u64;
            nat_rem(a538, a539, a540)
        };
        let a542 = 1u64;
        if nat_lt(a541, a542) {
            1u64
        } else {
            2147483649u64
        }
    } else {
        let a546 = {
            let a543 = v0;
            let a544 = 4u64;
            let a545 = 0u64;
            nat_rem(a543, a544, a545)
        };
        let a547 = 3u64;
        if nat_lt(a546, a547) {
            0u64
        } else {
            2147483648u64
        }
    }
}

pub fn f30(v0: u64) -> u64 {
    let a551 = {
        let a548 = v0;
        let a549 = 8u64;
        let a550 = 0u64;
        nat_rem(a548, a549, a550)
    };
    let a552 = 4u64;
    if nat_lt(a551, a552) {
        let a556 = {
            let a553 = v0;
            let a554 = 8u64;
            let a555 = 0u64;
            nat_rem(a553, a554, a555)
        };
        let a557 = 2u64;
        if nat_lt(a556, a557) {
            let a561 = {
                let a558 = v0;
                let a559 = 8u64;
                let a560 = 0u64;
                nat_rem(a558, a559, a560)
            };
            let a562 = 1u64;
            if nat_lt(a561, a562) {
                3229614080u64
            } else {
                3221225472u64
            }
        } else {
            let a566 = {
                let a563 = v0;
                let a564 = 8u64;
                let a565 = 0u64;
                nat_rem(a563, a564, a565)
            };
            let a567 = 3u64;
            if nat_lt(a566, a567) {
                3212836864u64
            } else {
                2147483648u64
            }
        }
    } else {
        let a571 = {
            let a568 = v0;
            let a569 = 8u64;
            let a570 = 0u64;
            nat_rem(a568, a569, a570)
        };
        let a572 = 6u64;
        if nat_lt(a571, a572) {
            let a576 = {
                let a573 = v0;
                let a574 = 8u64;
                let a575 = 0u64;
                nat_rem(a573, a574, a575)
            };
            let a577 = 5u64;
            if nat_lt(a576, a577) {
                0u64
            } else {
                1065353216u64
            }
        } else {
            let a581 = {
                let a578 = v0;
                let a579 = 8u64;
                let a580 = 0u64;
                nat_rem(a578, a579, a580)
            };
            let a582 = 7u64;
            if nat_lt(a581, a582) {
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
