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
    C0,
    C1(u64, u64, u64, u64, Option<u64>),
    C2(u64, u64, u64, u64),
    C3(u64, u64, u64, u64, u64, u64),
    C4(u64, u64, u64, u64, u64, u64, u64, u64),
    C5(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
    C6(bool),
    C7(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64),
    C1(u64, u64),
    C2(u64, u64, u64),
    C3(u64, u64, u64),
}

pub fn f0(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt0> {
    let a1 = v0;
    let a2 = 30u64;
    if nat_eq(a1, a2) {
        let a3 = v1;
        let a4 = 0u64;
        if nat_eq(a3, a4) {
            Ok(Adt0::C7(13u64, 0u64, 0u64, 1099511627776u64))
        } else {
            let a5 = v1;
            let a6 = 1u64;
            if nat_eq(a5, a6) {
                Ok(Adt0::C7(18u64, 26u64, 27u64, 1099511627776u64))
            } else {
                Ok(Adt0::C0)
            }
        }
    } else {
        let a7 = v0;
        let a8 = 35u64;
        if nat_eq(a7, a8) {
            f1(v1)
        } else {
            let a9 = v0;
            let a10 = 106u64;
            if nat_eq(a9, a10) {
                Ok(f2(v1))
            } else {
                let a11 = v0;
                let a12 = 6u64;
                if nat_eq(a11, a12) {
                    f3(v1, v2, v3)
                } else {
                    let a13 = v0;
                    let a14 = 10u64;
                    if nat_eq(a13, a14) {
                        f4(v1)
                    } else {
                        let a15 = v0;
                        let a16 = 9u64;
                        if nat_eq(a15, a16) {
                            f5(v1)
                        } else {
                            let a17 = v0;
                            let a18 = 47u64;
                            if nat_eq(a17, a18) {
                                Ok(f6(v1))
                            } else {
                                let a19 = v0;
                                let a20 = 44u64;
                                if nat_eq(a19, a20) {
                                    Ok(f7(v1))
                                } else {
                                    let a21 = v0;
                                    let a22 = 43u64;
                                    if nat_eq(a21, a22) {
                                        Ok(f8(v1))
                                    } else {
                                        let a23 = v0;
                                        let a24 = 42u64;
                                        if nat_eq(a23, a24) {
                                            Ok(f9(v1))
                                        } else {
                                            let a25 = v0;
                                            let a26 = 41u64;
                                            if nat_eq(a25, a26) {
                                                Ok(f10(v1))
                                            } else {
                                                let a27 = v0;
                                                let a28 = 36u64;
                                                if nat_eq(a27, a28) {
                                                    Ok(f11(v1))
                                                } else {
                                                    Ok(Adt0::C0)
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f1(v0: u64) -> R<Adt0> {
    let a29 = v0;
    let a30 = 0u64;
    if nat_eq(a29, a30) {
        f12(1u64, Adt1::C0(0u64))
    } else {
        let a31 = v0;
        let a32 = 1u64;
        if nat_eq(a31, a32) {
            f12(0u64, Adt1::C0(4u64))
        } else {
            let a33 = v0;
            let a34 = 2u64;
            if nat_eq(a33, a34) {
                f12(0u64, Adt1::C1(1280u64, 0u64))
            } else {
                let a35 = v0;
                let a36 = 3u64;
                if nat_eq(a35, a36) {
                    f12(0u64, Adt1::C1(0u64, 24u64))
                } else {
                    let a37 = v0;
                    let a38 = 4u64;
                    if nat_eq(a37, a38) {
                        f12(0u64, Adt1::C2(0u64, 2u64, 0u64))
                    } else {
                        let a39 = v0;
                        let a40 = 5u64;
                        if nat_eq(a39, a40) {
                            f12(0u64, Adt1::C3(0u64, 0u64, 0u64))
                        } else {
                            Ok(Adt0::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f2(v0: u64) -> Adt0 {
    let a41 = v0;
    let a42 = 0u64;
    if nat_eq(a41, a42) {
        Adt0::C5(1u64, 1u64, 1u64, 1u64, 4u64, 4u64, 4u64, 4u64, 0u64, 4u64, 0u64)
    } else {
        let a43 = v0;
        let a44 = 1u64;
        if nat_eq(a43, a44) {
            Adt0::C5(18446744073709551615u64, 1u64, 1u64, 1u64, 8u64, 8u64, 8u64, 8u64, 1u64, 8u64, 1u64)
        } else {
            let a45 = v0;
            let a46 = 2u64;
            if nat_eq(a45, a46) {
                Adt0::C5(18446744073709551615u64, 1u64, 1u64, 1u64, 18u64, 18u64, 18u64, 18u64, 32u64, 18u64, 1u64)
            } else {
                let a47 = v0;
                let a48 = 3u64;
                if nat_eq(a47, a48) {
                    Adt0::C5(2u64, 2u64, 1u64, 1u64, 1u64, 18446744073709551615u64, 1u64, 1u64, 1u64, 1u64, 1u64)
                } else {
                    let a49 = v0;
                    let a50 = 4u64;
                    if nat_eq(a49, a50) {
                        Adt0::C5(18446744073709551615u64, 0u64, 1u64, 1u64, 18446744073709551615u64, 18446744073709551615u64, 18446744073709551615u64, 18446744073709551615u64, 1u64, 8u64, 2u64)
                    } else {
                        Adt0::C0
                    }
                }
            }
        }
    }
}

pub fn f3(v0: u64, v1: u64, v2: u64) -> R<Adt0> {
    let a51 = v0;
    let a52 = 48u64;
    if nat_lt(a51, a52) {
        let v3: u64 = {
            let a53 = v0;
            let a54 = 16u64;
            if nat_lt(a53, a54) {
                1u64
            } else {
                let a55 = v0;
                let a56 = 32u64;
                if nat_lt(a55, a56) {
                    2u64
                } else {
                    4u64
                }
            }
        };
        let v4: u64 = {
            let a63 = {
                let a60 = {
                    let a57 = v0;
                    let a58 = 8u64;
                    let a59 = 0u64;
                    nat_quot(a57, a58, a59)
                };
                let a61 = 2u64;
                let a62 = 0u64;
                nat_rem(a60, a61, a62)
            };
            let a64 = 0u64;
            if nat_eq(a63, a64) {
                1u64
            } else {
                3u64
            }
        };
        let v5: u64 = {
            let a71 = {
                let a68 = {
                    let a65 = v0;
                    let a66 = 4u64;
                    let a67 = 0u64;
                    nat_quot(a65, a66, a67)
                };
                let a69 = 2u64;
                let a70 = 0u64;
                nat_rem(a68, a69, a70)
            };
            let a72 = 0u64;
            if nat_eq(a71, a72) {
                1u64
            } else {
                2u64
            }
        };
        let v6: u64 = {
            let a79 = {
                let a76 = {
                    let a73 = v0;
                    let a74 = 2u64;
                    let a75 = 0u64;
                    nat_quot(a73, a74, a75)
                };
                let a77 = 2u64;
                let a78 = 0u64;
                nat_rem(a76, a77, a78)
            };
            let a80 = 0u64;
            if nat_eq(a79, a80) {
                1u64
            } else {
                2u64
            }
        };
        let v7: u64 = {
            let a84 = {
                let a81 = v0;
                let a82 = 2u64;
                let a83 = 0u64;
                nat_rem(a81, a82, a83)
            };
            let a85 = 0u64;
            if nat_eq(a84, a85) {
                0u64
            } else {
                7u64
            }
        };
        let v8: u64 = {
            let a86 = v2;
            let a87 = v7;
            nat_add(a86, a87)?
        };
        let v9: u64 = {
            let a90 = {
                let a88 = v3;
                let a89 = v8;
                nat_mul(a88, a89)?
            };
            let a91 = 11u64;
            nat_add(a90, a91)?
        };
        let v10: u64 = {
            let a94 = {
                let a92 = v4;
                let a93 = v9;
                nat_mul(a92, a93)?
            };
            let a95 = 13u64;
            nat_add(a94, a95)?
        };
        Ok(Adt0::C4({
            let a96 = v1;
            let a97 = v3;
            nat_mul(a96, a97)?
        }, v4, v5, v6, v8, v9, v10, {
            let a100 = {
                let a98 = v5;
                let a99 = v10;
                nat_mul(a98, a99)?
            };
            let a101 = 17u64;
            nat_add(a100, a101)?
        }))
    } else {
        let a102 = v0;
        let a103 = 52u64;
        if nat_lt(a102, a103) {
            Ok(Adt0::C4({
                let a104 = v0;
                let a105 = 48u64;
                if nat_eq(a104, a105) {
                    0u64
                } else {
                    v1
                }
            }, {
                let a106 = v0;
                let a107 = 49u64;
                if nat_eq(a106, a107) {
                    0u64
                } else {
                    2u64
                }
            }, {
                let a108 = v0;
                let a109 = 50u64;
                if nat_eq(a108, a109) {
                    0u64
                } else {
                    2u64
                }
            }, {
                let a110 = v0;
                let a111 = 51u64;
                if nat_eq(a110, a111) {
                    0u64
                } else {
                    2u64
                }
            }, v2, {
                let a112 = v2;
                let a113 = 2u64;
                nat_mul(a112, a113)?
            }, {
                let a114 = v2;
                let a115 = 4u64;
                nat_mul(a114, a115)?
            }, {
                let a116 = v2;
                let a117 = 8u64;
                nat_mul(a116, a117)?
            }))
        } else {
            Ok(Adt0::C0)
        }
    }
}

pub fn f4(v0: u64) -> R<Adt0> {
    let a118 = v0;
    let a119 = 864u64;
    if nat_lt(a118, a119) {
        let v1: u64 = {
            let a123 = {
                let a120 = v0;
                let a121 = 108u64;
                let a122 = 0u64;
                nat_quot(a120, a121, a122)
            };
            let a124 = 0u64;
            if nat_eq(a123, a124) {
                1u64
            } else {
                let a128 = {
                    let a125 = v0;
                    let a126 = 108u64;
                    let a127 = 0u64;
                    nat_quot(a125, a126, a127)
                };
                let a129 = 1u64;
                if nat_eq(a128, a129) {
                    2u64
                } else {
                    let a133 = {
                        let a130 = v0;
                        let a131 = 108u64;
                        let a132 = 0u64;
                        nat_quot(a130, a131, a132)
                    };
                    let a134 = 2u64;
                    if nat_eq(a133, a134) {
                        4u64
                    } else {
                        let a138 = {
                            let a135 = v0;
                            let a136 = 108u64;
                            let a137 = 0u64;
                            nat_quot(a135, a136, a137)
                        };
                        let a139 = 3u64;
                        if nat_eq(a138, a139) {
                            8u64
                        } else {
                            let a143 = {
                                let a140 = v0;
                                let a141 = 108u64;
                                let a142 = 0u64;
                                nat_quot(a140, a141, a142)
                            };
                            let a144 = 4u64;
                            if nat_eq(a143, a144) {
                                16u64
                            } else {
                                let a148 = {
                                    let a145 = v0;
                                    let a146 = 108u64;
                                    let a147 = 0u64;
                                    nat_quot(a145, a146, a147)
                                };
                                let a149 = 5u64;
                                if nat_eq(a148, a149) {
                                    64u64
                                } else {
                                    let a153 = {
                                        let a150 = v0;
                                        let a151 = 108u64;
                                        let a152 = 0u64;
                                        nat_quot(a150, a151, a152)
                                    };
                                    let a154 = 6u64;
                                    if nat_eq(a153, a154) {
                                        256u64
                                    } else {
                                        let a158 = {
                                            let a155 = v0;
                                            let a156 = 108u64;
                                            let a157 = 0u64;
                                            nat_quot(a155, a156, a157)
                                        };
                                        let a159 = 7u64;
                                        if nat_eq(a158, a159) {
                                            1024u64
                                        } else {
                                            1u64
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };
        let v2: u64 = {
            let a166 = {
                let a163 = {
                    let a160 = v0;
                    let a161 = 36u64;
                    let a162 = 0u64;
                    nat_quot(a160, a161, a162)
                };
                let a164 = 3u64;
                let a165 = 0u64;
                nat_rem(a163, a164, a165)
            };
            let a167 = 0u64;
            if nat_eq(a166, a167) {
                0u64
            } else {
                let a174 = {
                    let a171 = {
                        let a168 = v0;
                        let a169 = 36u64;
                        let a170 = 0u64;
                        nat_quot(a168, a169, a170)
                    };
                    let a172 = 3u64;
                    let a173 = 0u64;
                    nat_rem(a171, a172, a173)
                };
                let a175 = 1u64;
                if nat_eq(a174, a175) {
                    v1
                } else {
                    let a182 = {
                        let a179 = {
                            let a176 = v0;
                            let a177 = 36u64;
                            let a178 = 0u64;
                            nat_quot(a176, a177, a178)
                        };
                        let a180 = 3u64;
                        let a181 = 0u64;
                        nat_rem(a179, a180, a181)
                    };
                    let a183 = 2u64;
                    if nat_eq(a182, a183) {
                        let a184 = v1;
                        let a185 = 3u64;
                        nat_mul(a184, a185)?
                    } else {
                        0u64
                    }
                }
            }
        };
        let v3: u64 = {
            let a192 = {
                let a189 = {
                    let a186 = v0;
                    let a187 = 6u64;
                    let a188 = 0u64;
                    nat_quot(a186, a187, a188)
                };
                let a190 = 6u64;
                let a191 = 0u64;
                nat_rem(a189, a190, a191)
            };
            let a193 = 0u64;
            if nat_eq(a192, a193) {
                0u64
            } else {
                let a200 = {
                    let a197 = {
                        let a194 = v0;
                        let a195 = 6u64;
                        let a196 = 0u64;
                        nat_quot(a194, a195, a196)
                    };
                    let a198 = 6u64;
                    let a199 = 0u64;
                    nat_rem(a197, a198, a199)
                };
                let a201 = 1u64;
                if nat_eq(a200, a201) {
                    1u64
                } else {
                    let a208 = {
                        let a205 = {
                            let a202 = v0;
                            let a203 = 6u64;
                            let a204 = 0u64;
                            nat_quot(a202, a203, a204)
                        };
                        let a206 = 6u64;
                        let a207 = 0u64;
                        nat_rem(a205, a206, a207)
                    };
                    let a209 = 2u64;
                    if nat_eq(a208, a209) {
                        let a210 = v1;
                        let a211 = 1u64;
                        nat_sub(a210, a211)
                    } else {
                        let a218 = {
                            let a215 = {
                                let a212 = v0;
                                let a213 = 6u64;
                                let a214 = 0u64;
                                nat_quot(a212, a213, a214)
                            };
                            let a216 = 6u64;
                            let a217 = 0u64;
                            nat_rem(a215, a216, a217)
                        };
                        let a219 = 3u64;
                        if nat_eq(a218, a219) {
                            v1
                        } else {
                            let a226 = {
                                let a223 = {
                                    let a220 = v0;
                                    let a221 = 6u64;
                                    let a222 = 0u64;
                                    nat_quot(a220, a221, a222)
                                };
                                let a224 = 6u64;
                                let a225 = 0u64;
                                nat_rem(a223, a224, a225)
                            };
                            let a227 = 4u64;
                            if nat_eq(a226, a227) {
                                let a228 = v1;
                                let a229 = 1u64;
                                nat_add(a228, a229)?
                            } else {
                                let a236 = {
                                    let a233 = {
                                        let a230 = v0;
                                        let a231 = 6u64;
                                        let a232 = 0u64;
                                        nat_quot(a230, a231, a232)
                                    };
                                    let a234 = 6u64;
                                    let a235 = 0u64;
                                    nat_rem(a233, a234, a235)
                                };
                                let a237 = 5u64;
                                if nat_eq(a236, a237) {
                                    let a238 = v1;
                                    let a239 = 4u64;
                                    nat_mul(a238, a239)?
                                } else {
                                    0u64
                                }
                            }
                        }
                    }
                }
            }
        };
        let v4: u64 = {
            let a243 = {
                let a240 = v0;
                let a241 = 6u64;
                let a242 = 0u64;
                nat_rem(a240, a241, a242)
            };
            let a244 = 0u64;
            if nat_eq(a243, a244) {
                0u64
            } else {
                let a248 = {
                    let a245 = v0;
                    let a246 = 6u64;
                    let a247 = 0u64;
                    nat_rem(a245, a246, a247)
                };
                let a249 = 1u64;
                if nat_eq(a248, a249) {
                    1u64
                } else {
                    let a253 = {
                        let a250 = v0;
                        let a251 = 6u64;
                        let a252 = 0u64;
                        nat_rem(a250, a251, a252)
                    };
                    let a254 = 2u64;
                    if nat_eq(a253, a254) {
                        let a255 = v1;
                        let a256 = 1u64;
                        nat_sub(a255, a256)
                    } else {
                        let a260 = {
                            let a257 = v0;
                            let a258 = 6u64;
                            let a259 = 0u64;
                            nat_rem(a257, a258, a259)
                        };
                        let a261 = 3u64;
                        if nat_eq(a260, a261) {
                            v1
                        } else {
                            let a265 = {
                                let a262 = v0;
                                let a263 = 6u64;
                                let a264 = 0u64;
                                nat_rem(a262, a263, a264)
                            };
                            let a266 = 4u64;
                            if nat_eq(a265, a266) {
                                let a267 = v1;
                                let a268 = 1u64;
                                nat_add(a267, a268)?
                            } else {
                                let a272 = {
                                    let a269 = v0;
                                    let a270 = 6u64;
                                    let a271 = 0u64;
                                    nat_rem(a269, a270, a271)
                                };
                                let a273 = 5u64;
                                if nat_eq(a272, a273) {
                                    let a276 = {
                                        let a274 = v1;
                                        let a275 = 2u64;
                                        nat_mul(a274, a275)?
                                    };
                                    let a277 = 1u64;
                                    nat_add(a276, a277)?
                                } else {
                                    0u64
                                }
                            }
                        }
                    }
                }
            }
        };
        Ok(Adt0::C2(v3, v2, v4, v1))
    } else {
        Ok(Adt0::C0)
    }
}

pub fn f5(v0: u64) -> R<Adt0> {
    let a278 = v0;
    let a279 = 1024u64;
    if nat_lt(a278, a279) {
        let v1: u64 = {
            let a286 = {
                let a283 = {
                    let a280 = v0;
                    let a281 = 16u64;
                    let a282 = 0u64;
                    nat_quot(a280, a281, a282)
                };
                let a284 = 4u64;
                let a285 = 0u64;
                nat_rem(a283, a284, a285)
            };
            let a287 = 0u64;
            if nat_eq(a286, a287) {
                0u64
            } else {
                let a294 = {
                    let a291 = {
                        let a288 = v0;
                        let a289 = 16u64;
                        let a290 = 0u64;
                        nat_quot(a288, a289, a290)
                    };
                    let a292 = 4u64;
                    let a293 = 0u64;
                    nat_rem(a291, a292, a293)
                };
                let a295 = 1u64;
                if nat_eq(a294, a295) {
                    1u64
                } else {
                    let a302 = {
                        let a299 = {
                            let a296 = v0;
                            let a297 = 16u64;
                            let a298 = 0u64;
                            nat_quot(a296, a297, a298)
                        };
                        let a300 = 4u64;
                        let a301 = 0u64;
                        nat_rem(a299, a300, a301)
                    };
                    let a303 = 2u64;
                    if nat_eq(a302, a303) {
                        8u64
                    } else {
                        let a310 = {
                            let a307 = {
                                let a304 = v0;
                                let a305 = 16u64;
                                let a306 = 0u64;
                                nat_quot(a304, a305, a306)
                            };
                            let a308 = 4u64;
                            let a309 = 0u64;
                            nat_rem(a307, a308, a309)
                        };
                        let a311 = 3u64;
                        if nat_eq(a310, a311) {
                            64u64
                        } else {
                            0u64
                        }
                    }
                }
            }
        };
        let v2: u64 = {
            let a318 = {
                let a315 = {
                    let a312 = v0;
                    let a313 = 4u64;
                    let a314 = 0u64;
                    nat_quot(a312, a313, a314)
                };
                let a316 = 4u64;
                let a317 = 0u64;
                nat_rem(a315, a316, a317)
            };
            let a319 = 0u64;
            if nat_eq(a318, a319) {
                0u64
            } else {
                let a326 = {
                    let a323 = {
                        let a320 = v0;
                        let a321 = 4u64;
                        let a322 = 0u64;
                        nat_quot(a320, a321, a322)
                    };
                    let a324 = 4u64;
                    let a325 = 0u64;
                    nat_rem(a323, a324, a325)
                };
                let a327 = 1u64;
                if nat_eq(a326, a327) {
                    1u64
                } else {
                    let a334 = {
                        let a331 = {
                            let a328 = v0;
                            let a329 = 4u64;
                            let a330 = 0u64;
                            nat_quot(a328, a329, a330)
                        };
                        let a332 = 4u64;
                        let a333 = 0u64;
                        nat_rem(a331, a332, a333)
                    };
                    let a335 = 2u64;
                    if nat_eq(a334, a335) {
                        v1
                    } else {
                        let a342 = {
                            let a339 = {
                                let a336 = v0;
                                let a337 = 4u64;
                                let a338 = 0u64;
                                nat_quot(a336, a337, a338)
                            };
                            let a340 = 4u64;
                            let a341 = 0u64;
                            nat_rem(a339, a340, a341)
                        };
                        let a343 = 3u64;
                        if nat_eq(a342, a343) {
                            let a344 = v1;
                            let a345 = 1u64;
                            nat_add(a344, a345)?
                        } else {
                            0u64
                        }
                    }
                }
            }
        };
        let v3: u64 = {
            let a349 = {
                let a346 = v0;
                let a347 = 4u64;
                let a348 = 0u64;
                nat_rem(a346, a347, a348)
            };
            let a350 = 0u64;
            if nat_eq(a349, a350) {
                0u64
            } else {
                let a354 = {
                    let a351 = v0;
                    let a352 = 4u64;
                    let a353 = 0u64;
                    nat_rem(a351, a352, a353)
                };
                let a355 = 1u64;
                if nat_eq(a354, a355) {
                    1u64
                } else {
                    let a359 = {
                        let a356 = v0;
                        let a357 = 4u64;
                        let a358 = 0u64;
                        nat_rem(a356, a357, a358)
                    };
                    let a360 = 2u64;
                    if nat_eq(a359, a360) {
                        v1
                    } else {
                        let a364 = {
                            let a361 = v0;
                            let a362 = 4u64;
                            let a363 = 0u64;
                            nat_rem(a361, a362, a363)
                        };
                        let a365 = 3u64;
                        if nat_eq(a364, a365) {
                            let a366 = v1;
                            let a367 = 1u64;
                            nat_add(a366, a367)?
                        } else {
                            0u64
                        }
                    }
                }
            }
        };
        Ok(Adt0::C3({
            let a368 = v0;
            let a369 = 256u64;
            let a370 = 0u64;
            nat_quot(a368, a369, a370)
        }, {
            let a374 = {
                let a371 = v0;
                let a372 = 128u64;
                let a373 = 0u64;
                nat_quot(a371, a372, a373)
            };
            let a375 = 2u64;
            let a376 = 0u64;
            nat_rem(a374, a375, a376)
        }, {
            let a380 = {
                let a377 = v0;
                let a378 = 64u64;
                let a379 = 0u64;
                nat_quot(a377, a378, a379)
            };
            let a381 = 2u64;
            let a382 = 0u64;
            nat_rem(a380, a381, a382)
        }, v1, v2, v3))
    } else {
        Ok(Adt0::C0)
    }
}

pub fn f6(v0: u64) -> Adt0 {
    let a383 = v0;
    let a384 = 0u64;
    if nat_eq(a383, a384) {
        Adt0::C1(3u64, 21u64, 3u64, 0u64, Some(4294967294u64))
    } else {
        let a385 = v0;
        let a386 = 1u64;
        if nat_eq(a385, a386) {
            Adt0::C1(0u64, 22u64, 3u64, 0u64, None::<u64>)
        } else {
            let a387 = v0;
            let a388 = 2u64;
            if nat_eq(a387, a388) {
                Adt0::C1(0u64, 0u64, 2147483648u64, 0u64, None::<u64>)
            } else {
                let a389 = v0;
                let a390 = 3u64;
                if nat_eq(a389, a390) {
                    Adt0::C1(4u64, 0u64, 0u64, 0u64, None::<u64>)
                } else {
                    Adt0::C0
                }
            }
        }
    }
}

pub fn f7(v0: u64) -> Adt0 {
    let a391 = v0;
    let a392 = 0u64;
    if nat_eq(a391, a392) {
        Adt0::C1(24u64, 0u64, 0u64, 0u64, Some(15360u64))
    } else {
        let a393 = v0;
        let a394 = 1u64;
        if nat_eq(a393, a394) {
            Adt0::C1(31u64, 0u64, 0u64, 0u64, Some(1920u64))
        } else {
            let a395 = v0;
            let a396 = 2u64;
            if nat_eq(a395, a396) {
                Adt0::C1(25u64, 15360u64, 0u64, 0u64, None::<u64>)
            } else {
                let a397 = v0;
                let a398 = 3u64;
                if nat_eq(a397, a398) {
                    Adt0::C1(32u64, 1920u64, 0u64, 0u64, None::<u64>)
                } else {
                    let a399 = v0;
                    let a400 = 4u64;
                    if nat_eq(a399, a400) {
                        Adt0::C1(26u64, 0u64, 2u64, 0u64, None::<u64>)
                    } else {
                        let a401 = v0;
                        let a402 = 5u64;
                        if nat_eq(a401, a402) {
                            Adt0::C1(33u64, 0u64, 2u64, 0u64, None::<u64>)
                        } else {
                            let a403 = v0;
                            let a404 = 6u64;
                            if nat_eq(a403, a404) {
                                Adt0::C1(27u64, 0u64, 1u64, 0u64, None::<u64>)
                            } else {
                                let a405 = v0;
                                let a406 = 7u64;
                                if nat_eq(a405, a406) {
                                    Adt0::C1(34u64, 0u64, 0u64, 192u64, None::<u64>)
                                } else {
                                    let a407 = v0;
                                    let a408 = 8u64;
                                    if nat_eq(a407, a408) {
                                        Adt0::C1(30u64, 0u64, 0u64, 0u64, None::<u64>)
                                    } else {
                                        let a409 = v0;
                                        let a410 = 9u64;
                                        if nat_eq(a409, a410) {
                                            Adt0::C1(37u64, 0u64, 0u64, 0u64, None::<u64>)
                                        } else {
                                            Adt0::C0
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f8(v0: u64) -> Adt0 {
    let a411 = v0;
    let a412 = 0u64;
    if nat_eq(a411, a412) {
        Adt0::C1(17u64, 0u64, 0u64, 0u64, Some(10368u64))
    } else {
        let a413 = v0;
        let a414 = 1u64;
        if nat_eq(a413, a414) {
            Adt0::C1(18u64, 10368u64, 0u64, 0u64, None::<u64>)
        } else {
            let a415 = v0;
            let a416 = 2u64;
            if nat_eq(a415, a416) {
                Adt0::C1(19u64, 0u64, 4u64, 0u64, None::<u64>)
            } else {
                let a417 = v0;
                let a418 = 3u64;
                if nat_eq(a417, a418) {
                    Adt0::C1(20u64, 0u64, 2u64, 0u64, None::<u64>)
                } else {
                    let a419 = v0;
                    let a420 = 4u64;
                    if nat_eq(a419, a420) {
                        Adt0::C1(23u64, 0u64, 0u64, 0u64, None::<u64>)
                    } else {
                        Adt0::C0
                    }
                }
            }
        }
    }
}

pub fn f9(v0: u64) -> Adt0 {
    let a421 = v0;
    let a422 = 0u64;
    if nat_eq(a421, a422) {
        Adt0::C1(11u64, 0u64, 0u64, 0u64, Some(160u64))
    } else {
        let a423 = v0;
        let a424 = 1u64;
        if nat_eq(a423, a424) {
            Adt0::C1(12u64, 160u64, 0u64, 0u64, None::<u64>)
        } else {
            let a425 = v0;
            let a426 = 2u64;
            if nat_eq(a425, a426) {
                Adt0::C1(13u64, 0u64, 4u64, 0u64, None::<u64>)
            } else {
                let a427 = v0;
                let a428 = 3u64;
                if nat_eq(a427, a428) {
                    Adt0::C1(14u64, 0u64, 0u64, 256u64, None::<u64>)
                } else {
                    let a429 = v0;
                    let a430 = 4u64;
                    if nat_eq(a429, a430) {
                        Adt0::C1(16u64, 0u64, 0u64, 0u64, None::<u64>)
                    } else {
                        Adt0::C0
                    }
                }
            }
        }
    }
}

pub fn f10(v0: u64) -> Adt0 {
    let a431 = v0;
    let a432 = 0u64;
    if nat_eq(a431, a432) {
        Adt0::C1(5u64, 0u64, 0u64, 0u64, Some(3456u64))
    } else {
        let a433 = v0;
        let a434 = 1u64;
        if nat_eq(a433, a434) {
            Adt0::C1(5u64, 0u64, 0u64, 5u64, None::<u64>)
        } else {
            let a435 = v0;
            let a436 = 2u64;
            if nat_eq(a435, a436) {
                Adt0::C1(5u64, 1u64, 0u64, 0u64, None::<u64>)
            } else {
                let a437 = v0;
                let a438 = 3u64;
                if nat_eq(a437, a438) {
                    Adt0::C1(6u64, 3456u64, 0u64, 0u64, None::<u64>)
                } else {
                    let a439 = v0;
                    let a440 = 4u64;
                    if nat_eq(a439, a440) {
                        Adt0::C1(6u64, 0u64, 0u64, 9u64, None::<u64>)
                    } else {
                        let a441 = v0;
                        let a442 = 5u64;
                        if nat_eq(a441, a442) {
                            Adt0::C1(7u64, 0u64, 2u64, 0u64, None::<u64>)
                        } else {
                            let a443 = v0;
                            let a444 = 6u64;
                            if nat_eq(a443, a444) {
                                Adt0::C1(7u64, 0u64, 0u64, 10u64, None::<u64>)
                            } else {
                                let a445 = v0;
                                let a446 = 7u64;
                                if nat_eq(a445, a446) {
                                    Adt0::C1(8u64, 0u64, 0u64, 256u64, None::<u64>)
                                } else {
                                    let a447 = v0;
                                    let a448 = 8u64;
                                    if nat_eq(a447, a448) {
                                        Adt0::C1(9u64, 215u64, 8u64, 0u64, None::<u64>)
                                    } else {
                                        let a449 = v0;
                                        let a450 = 9u64;
                                        if nat_eq(a449, a450) {
                                            Adt0::C1(10u64, 0u64, 0u64, 0u64, None::<u64>)
                                        } else {
                                            let a451 = v0;
                                            let a452 = 10u64;
                                            if nat_eq(a451, a452) {
                                                Adt0::C1(18446744073709551615u64, 18446744073709551615u64, 18446744073709551615u64, 18446744073709551615u64, None::<u64>)
                                            } else {
                                                Adt0::C0
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f11(v0: u64) -> Adt0 {
    let a453 = v0;
    let a454 = 0u64;
    if nat_eq(a453, a454) {
        Adt0::C1(1u64, 120u64, 0u64, 0u64, None::<u64>)
    } else {
        let a455 = v0;
        let a456 = 1u64;
        if nat_eq(a455, a456) {
            Adt0::C1(1u64, 0u64, 0u64, 24u64, None::<u64>)
        } else {
            let a457 = v0;
            let a458 = 2u64;
            if nat_eq(a457, a458) {
                Adt0::C1(2u64, 0u64, 2u64, 0u64, None::<u64>)
            } else {
                let a459 = v0;
                let a460 = 3u64;
                if nat_eq(a459, a460) {
                    Adt0::C1(3u64, 96u64, 1u64, 0u64, None::<u64>)
                } else {
                    let a461 = v0;
                    let a462 = 4u64;
                    if nat_eq(a461, a462) {
                        Adt0::C1(4u64, 0u64, 0u64, 0u64, None::<u64>)
                    } else {
                        Adt0::C0
                    }
                }
            }
        }
    }
}

pub fn f12(v0: u64, v1: Adt1) -> R<Adt0> {
    Ok(Adt0::C6({
        let m463 = f13(v0, v1)?;
        match m463 {
            None => {
                true
            }
            Some(_) => {
                false
            }
        }
    }))
}

pub fn f13(v0: u64, v1: Adt1) -> R<Option<u64>> {
    let a464 = v0;
    let a465 = 0u64;
    if nat_eq(a464, a465) {
        let m466 = v1;
        match m466 {
            Adt1::C0(v2) => {
                let a467 = v2;
                let a468 = 4u64;
                if nat_lt(a467, a468) {
                    Ok(Some({
                        let a469 = v2;
                        let a470 = 0u64;
                        if nat_eq(a469, a470) {
                            1280u64
                        } else {
                            let a471 = v2;
                            let a472 = 1u64;
                            if nat_eq(a471, a472) {
                                24u64
                            } else {
                                let a473 = v2;
                                let a474 = 2u64;
                                if nat_eq(a473, a474) {
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
            Adt1::C1(v3, v4) => {
                let a475 = v3;
                let a476 = 1280u64;
                if if nat_lt(a475, a476) {
                    let a477 = v4;
                    let a478 = 24u64;
                    nat_lt(a477, a478)
                } else {
                    false
                } {
                    Ok(Some(f14(v3, v4)?))
                } else {
                    Ok(None::<u64>)
                }
            }
            Adt1::C2(v5, v6, v7) => {
                let a479 = v5;
                let a480 = 1280u64;
                if if if nat_lt(a479, a480) {
                    let a481 = v6;
                    let a482 = 2u64;
                    nat_lt(a481, a482)
                } else {
                    false
                } {
                    let a483 = v7;
                    let a484 = 2u64;
                    nat_lt(a483, a484)
                } else {
                    false
                } {
                    Ok(Some({
                        let a485 = v7;
                        let a486 = 0u64;
                        if nat_eq(a485, a486) {
                            f15(v5, v6)?
                        } else {
                            4u64
                        }
                    }))
                } else {
                    Ok(None::<u64>)
                }
            }
            Adt1::C3(v8, v9, v10) => {
                let a487 = v8;
                let a488 = 1280u64;
                if if if nat_lt(a487, a488) {
                    let a489 = v9;
                    let a490 = 2u64;
                    nat_lt(a489, a490)
                } else {
                    false
                } {
                    let a491 = v10;
                    let a492 = f15(v8, v9)?;
                    nat_lt(a491, a492)
                } else {
                    false
                } {
                    Ok(Some(f16(v8, v9, v10)?))
                } else {
                    Ok(None::<u64>)
                }
            }
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f14(v0: u64, v1: u64) -> R<u64> {
    let a493 = v1;
    let a494 = 0u64;
    if nat_eq(a493, a494) {
        Ok(f17(v0))
    } else {
        let a495 = v1;
        let a496 = 1u64;
        if nat_eq(a495, a496) {
            Ok(f18(v0))
        } else {
            let a497 = v1;
            let a498 = 2u64;
            if nat_eq(a497, a498) {
                Ok(f19(v0))
            } else {
                let a499 = v1;
                let a500 = 3u64;
                if nat_eq(a499, a500) {
                    Ok(f20(v0))
                } else {
                    let a501 = v1;
                    let a502 = 4u64;
                    if nat_eq(a501, a502) {
                        Ok(f21(v0, 0u64))
                    } else {
                        let a503 = v1;
                        let a504 = 5u64;
                        if nat_eq(a503, a504) {
                            Ok(f22(v0, 0u64))
                        } else {
                            let a505 = v1;
                            let a506 = 6u64;
                            if nat_eq(a505, a506) {
                                Ok(f21(v0, 1u64))
                            } else {
                                let a507 = v1;
                                let a508 = 7u64;
                                if nat_eq(a507, a508) {
                                    Ok(f22(v0, 1u64))
                                } else {
                                    let a509 = v1;
                                    let a510 = 16u64;
                                    if nat_lt(a509, a510) {
                                        f23(v0, {
                                            let a513 = {
                                                let a511 = v1;
                                                let a512 = 8u64;
                                                nat_sub(a511, a512)
                                            };
                                            let a514 = 4u64;
                                            let a515 = 0u64;
                                            nat_quot(a513, a514, a515)
                                        }, {
                                            let a518 = {
                                                let a516 = v1;
                                                let a517 = 8u64;
                                                nat_sub(a516, a517)
                                            };
                                            let a519 = 4u64;
                                            let a520 = 0u64;
                                            nat_rem(a518, a519, a520)
                                        })
                                    } else {
                                        let a521 = v1;
                                        let a522 = 16u64;
                                        if nat_eq(a521, a522) {
                                            Ok(f24(v0))
                                        } else {
                                            let a523 = v1;
                                            let a524 = 17u64;
                                            if nat_eq(a523, a524) {
                                                f25(v0)
                                            } else {
                                                let a525 = v1;
                                                let a526 = 18u64;
                                                if nat_eq(a525, a526) {
                                                    f26(v0)
                                                } else {
                                                    let a527 = v1;
                                                    let a528 = 19u64;
                                                    if nat_eq(a527, a528) {
                                                        f27(v0)
                                                    } else {
                                                        let a529 = v1;
                                                        let a530 = 20u64;
                                                        if nat_eq(a529, a530) {
                                                            f15(v0, 0u64)
                                                        } else {
                                                            let a531 = v1;
                                                            let a532 = 21u64;
                                                            if nat_eq(a531, a532) {
                                                                f15(v0, 1u64)
                                                            } else {
                                                                let a533 = v1;
                                                                let a534 = 22u64;
                                                                if nat_eq(a533, a534) {
                                                                    let a535 = v0;
                                                                    let a536 = 416u64;
                                                                    if nat_eq(a535, a536) {
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

pub fn f15(v0: u64, v1: u64) -> R<u64> {
    let a541 = {
        let a539 = {
            let a537 = f19(v0);
            let a538 = f28(v0, v1);
            nat_mul(a537, a538)?
        };
        let a540 = f21(v0, v1);
        nat_mul(a539, a540)?
    };
    let a542 = f22(v0, v1);
    nat_mul(a541, a542)
}

pub fn f16(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a543 = v2;
        let a544 = f19(v0);
        let a545 = 0u64;
        nat_rem(a543, a544, a545)
    };
    let v4: u64 = {
        let a546 = v2;
        let a547 = f19(v0);
        let a548 = 0u64;
        nat_quot(a546, a547, a548)
    };
    let v5: u64 = {
        let a549 = v4;
        let a550 = f28(v0, v1);
        let a551 = 0u64;
        nat_rem(a549, a550, a551)
    };
    let v6: u64 = {
        let a552 = v4;
        let a553 = f28(v0, v1);
        let a554 = 0u64;
        nat_quot(a552, a553, a554)
    };
    let v7: u64 = {
        let a555 = v6;
        let a556 = f21(v0, v1);
        let a557 = 0u64;
        nat_rem(a555, a556, a557)
    };
    let v8: u64 = {
        let a558 = v6;
        let a559 = f21(v0, v1);
        let a560 = 0u64;
        nat_quot(a558, a559, a560)
    };
    let a561 = f17(v0);
    let a562 = 1u64;
    if if if nat_eq(a561, a562) {
        let a563 = f18(v0);
        let a564 = 1u64;
        nat_eq(a563, a564)
    } else {
        false
    } {
        let a565 = f19(v0);
        let a566 = 1u64;
        nat_eq(a565, a566)
    } else {
        false
    } {
        f29(v0, v1)
    } else {
        Ok(f30({
            let a585 = {
                let a583 = {
                    let a579 = {
                        let a575 = {
                            let a571 = {
                                let a567 = v3;
                                let a568 = 7u64;
                                nat_mul(a567, a568)?
                            };
                            let a572 = {
                                let a569 = v5;
                                let a570 = 11u64;
                                nat_mul(a569, a570)?
                            };
                            nat_add(a571, a572)?
                        };
                        let a576 = {
                            let a573 = v7;
                            let a574 = 3u64;
                            nat_mul(a573, a574)?
                        };
                        nat_add(a575, a576)?
                    };
                    let a580 = {
                        let a577 = v8;
                        let a578 = 5u64;
                        nat_mul(a577, a578)?
                    };
                    nat_add(a579, a580)?
                };
                let a584 = {
                    let a581 = v1;
                    let a582 = 0u64;
                    if nat_eq(a581, a582) {
                        1u64
                    } else {
                        2u64
                    }
                };
                nat_add(a583, a584)?
            };
            let a586 = 19u64;
            let a587 = 0u64;
            nat_rem(a585, a586, a587)
        }))
    }
}

pub fn f17(v0: u64) -> u64 {
    let a594 = {
        let a591 = {
            let a588 = v0;
            let a589 = 320u64;
            let a590 = 0u64;
            nat_quot(a588, a589, a590)
        };
        let a592 = 4u64;
        let a593 = 0u64;
        nat_rem(a591, a592, a593)
    };
    let a595 = 0u64;
    if nat_eq(a594, a595) {
        0u64
    } else {
        let a602 = {
            let a599 = {
                let a596 = v0;
                let a597 = 320u64;
                let a598 = 0u64;
                nat_quot(a596, a597, a598)
            };
            let a600 = 4u64;
            let a601 = 0u64;
            nat_rem(a599, a600, a601)
        };
        let a603 = 1u64;
        if nat_eq(a602, a603) {
            1u64
        } else {
            let a610 = {
                let a607 = {
                    let a604 = v0;
                    let a605 = 320u64;
                    let a606 = 0u64;
                    nat_quot(a604, a605, a606)
                };
                let a608 = 4u64;
                let a609 = 0u64;
                nat_rem(a607, a608, a609)
            };
            let a611 = 2u64;
            if nat_eq(a610, a611) {
                2u64
            } else {
                5u64
            }
        }
    }
}

pub fn f18(v0: u64) -> u64 {
    let a618 = {
        let a615 = {
            let a612 = v0;
            let a613 = 80u64;
            let a614 = 0u64;
            nat_quot(a612, a613, a614)
        };
        let a616 = 4u64;
        let a617 = 0u64;
        nat_rem(a615, a616, a617)
    };
    let a619 = 0u64;
    if nat_eq(a618, a619) {
        0u64
    } else {
        let a626 = {
            let a623 = {
                let a620 = v0;
                let a621 = 80u64;
                let a622 = 0u64;
                nat_quot(a620, a621, a622)
            };
            let a624 = 4u64;
            let a625 = 0u64;
            nat_rem(a623, a624, a625)
        };
        let a627 = 1u64;
        if nat_eq(a626, a627) {
            1u64
        } else {
            let a634 = {
                let a631 = {
                    let a628 = v0;
                    let a629 = 80u64;
                    let a630 = 0u64;
                    nat_quot(a628, a629, a630)
                };
                let a632 = 4u64;
                let a633 = 0u64;
                nat_rem(a631, a632, a633)
            };
            let a635 = 2u64;
            if nat_eq(a634, a635) {
                3u64
            } else {
                7u64
            }
        }
    }
}

pub fn f19(v0: u64) -> u64 {
    let a642 = {
        let a639 = {
            let a636 = v0;
            let a637 = 16u64;
            let a638 = 0u64;
            nat_quot(a636, a637, a638)
        };
        let a640 = 5u64;
        let a641 = 0u64;
        nat_rem(a639, a640, a641)
    };
    let a643 = 0u64;
    if nat_eq(a642, a643) {
        0u64
    } else {
        let a650 = {
            let a647 = {
                let a644 = v0;
                let a645 = 16u64;
                let a646 = 0u64;
                nat_quot(a644, a645, a646)
            };
            let a648 = 5u64;
            let a649 = 0u64;
            nat_rem(a647, a648, a649)
        };
        let a651 = 1u64;
        if nat_eq(a650, a651) {
            1u64
        } else {
            let a658 = {
                let a655 = {
                    let a652 = v0;
                    let a653 = 16u64;
                    let a654 = 0u64;
                    nat_quot(a652, a653, a654)
                };
                let a656 = 5u64;
                let a657 = 0u64;
                nat_rem(a655, a656, a657)
            };
            let a659 = 2u64;
            if nat_eq(a658, a659) {
                2u64
            } else {
                let a666 = {
                    let a663 = {
                        let a660 = v0;
                        let a661 = 16u64;
                        let a662 = 0u64;
                        nat_quot(a660, a661, a662)
                    };
                    let a664 = 5u64;
                    let a665 = 0u64;
                    nat_rem(a663, a664, a665)
                };
                let a667 = 3u64;
                if nat_eq(a666, a667) {
                    7u64
                } else {
                    33u64
                }
            }
        }
    }
}

pub fn f20(v0: u64) -> u64 {
    let a671 = {
        let a668 = v0;
        let a669 = 4u64;
        let a670 = 0u64;
        nat_quot(a668, a669, a670)
    };
    let a672 = 4u64;
    let a673 = 0u64;
    nat_rem(a671, a672, a673)
}

pub fn f21(v0: u64, v1: u64) -> u64 {
    let a674 = v1;
    let a675 = 0u64;
    if nat_eq(a674, a675) {
        let a676 = 2u64;
        let a677 = f31(v0);
        if nat_le(a676, a677) {
            2u64
        } else {
            1u64
        }
    } else {
        let a678 = f31(v0);
        let a679 = 0u64;
        if nat_eq(a678, a679) {
            1u64
        } else {
            let a680 = f31(v0);
            let a681 = 1u64;
            if nat_eq(a680, a681) {
                3u64
            } else {
                4u64
            }
        }
    }
}

pub fn f22(v0: u64, v1: u64) -> u64 {
    let a682 = v1;
    let a683 = 0u64;
    if nat_eq(a682, a683) {
        let a684 = f31(v0);
        let a685 = 3u64;
        if nat_eq(a684, a685) {
            2u64
        } else {
            1u64
        }
    } else {
        let a686 = f31(v0);
        let a687 = 0u64;
        if nat_eq(a686, a687) {
            1u64
        } else {
            let a688 = f31(v0);
            let a689 = 3u64;
            if nat_eq(a688, a689) {
                4u64
            } else {
                2u64
            }
        }
    }
}

pub fn f23(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a690 = v2;
    let a691 = 0u64;
    if nat_eq(a690, a691) {
        Ok(4u64)
    } else {
        let a692 = v2;
        let a693 = 1u64;
        if nat_eq(a692, a693) {
            f32(v0)
        } else {
            let a694 = v2;
            let a695 = 2u64;
            if nat_eq(a694, a695) {
                f33(v0, v1)
            } else {
                f34(v0, v1)
            }
        }
    }
}

pub fn f24(v0: u64) -> u64 {
    let a696 = 2u64;
    let a697 = f20(v0);
    if nat_le(a696, a697) {
        8u64
    } else {
        4u64
    }
}

pub fn f25(v0: u64) -> R<u64> {
    let a700 = {
        let a698 = f24(v0);
        let a699 = f17(v0);
        nat_mul(a698, a699)?
    };
    let a701 = f35(v0);
    nat_add(a700, a701)
}

pub fn f26(v0: u64) -> R<u64> {
    let a704 = {
        let a702 = f25(v0)?;
        let a703 = f18(v0);
        nat_mul(a702, a703)?
    };
    let a705 = f35(v0);
    nat_add(a704, a705)
}

pub fn f27(v0: u64) -> R<u64> {
    let a708 = {
        let a706 = f26(v0)?;
        let a707 = f21(v0, 1u64);
        nat_mul(a706, a707)?
    };
    let a709 = f35(v0);
    nat_add(a708, a709)
}

pub fn f28(v0: u64, v1: u64) -> u64 {
    let a710 = v1;
    let a711 = 0u64;
    if nat_eq(a710, a711) {
        f17(v0)
    } else {
        f18(v0)
    }
}

pub fn f29(v0: u64, v1: u64) -> R<u64> {
    let a719 = {
        let a717 = {
            let a715 = {
                let a712 = f20(v0);
                let a713 = 2u64;
                let a714 = 0u64;
                nat_rem(a712, a713, a714)
            };
            let a716 = 4u64;
            nat_mul(a715, a716)?
        };
        let a718 = f31(v0);
        nat_add(a717, a718)?
    };
    let a720 = 0u64;
    if nat_eq(a719, a720) {
        let a721 = v1;
        let a722 = 0u64;
        if nat_eq(a721, a722) {
            Ok(2139095040u64)
        } else {
            Ok(0u64)
        }
    } else {
        let a730 = {
            let a728 = {
                let a726 = {
                    let a723 = f20(v0);
                    let a724 = 2u64;
                    let a725 = 0u64;
                    nat_rem(a723, a724, a725)
                };
                let a727 = 4u64;
                nat_mul(a726, a727)?
            };
            let a729 = f31(v0);
            nat_add(a728, a729)?
        };
        let a731 = 1u64;
        if nat_eq(a730, a731) {
            let a732 = v1;
            let a733 = 0u64;
            if nat_eq(a732, a733) {
                Ok(2139095040u64)
            } else {
                Ok(3212836864u64)
            }
        } else {
            let a741 = {
                let a739 = {
                    let a737 = {
                        let a734 = f20(v0);
                        let a735 = 2u64;
                        let a736 = 0u64;
                        nat_rem(a734, a735, a736)
                    };
                    let a738 = 4u64;
                    nat_mul(a737, a738)?
                };
                let a740 = f31(v0);
                nat_add(a739, a740)?
            };
            let a742 = 2u64;
            if nat_eq(a741, a742) {
                let a743 = v1;
                let a744 = 0u64;
                if nat_eq(a743, a744) {
                    Ok(2143289344u64)
                } else {
                    Ok(1065353216u64)
                }
            } else {
                let a752 = {
                    let a750 = {
                        let a748 = {
                            let a745 = f20(v0);
                            let a746 = 2u64;
                            let a747 = 0u64;
                            nat_rem(a745, a746, a747)
                        };
                        let a749 = 4u64;
                        nat_mul(a748, a749)?
                    };
                    let a751 = f31(v0);
                    nat_add(a750, a751)?
                };
                let a753 = 3u64;
                if nat_eq(a752, a753) {
                    let a754 = v1;
                    let a755 = 0u64;
                    if nat_eq(a754, a755) {
                        Ok(2147483648u64)
                    } else {
                        Ok(1065353216u64)
                    }
                } else {
                    let a763 = {
                        let a761 = {
                            let a759 = {
                                let a756 = f20(v0);
                                let a757 = 2u64;
                                let a758 = 0u64;
                                nat_rem(a756, a757, a758)
                            };
                            let a760 = 4u64;
                            nat_mul(a759, a760)?
                        };
                        let a762 = f31(v0);
                        nat_add(a761, a762)?
                    };
                    let a764 = 4u64;
                    if nat_eq(a763, a764) {
                        let a765 = v1;
                        let a766 = 0u64;
                        if nat_eq(a765, a766) {
                            Ok(1u64)
                        } else {
                            Ok(1065353216u64)
                        }
                    } else {
                        let a774 = {
                            let a772 = {
                                let a770 = {
                                    let a767 = f20(v0);
                                    let a768 = 2u64;
                                    let a769 = 0u64;
                                    nat_rem(a767, a768, a769)
                                };
                                let a771 = 4u64;
                                nat_mul(a770, a771)?
                            };
                            let a773 = f31(v0);
                            nat_add(a772, a773)?
                        };
                        let a775 = 5u64;
                        if nat_eq(a774, a775) {
                            let a776 = v1;
                            let a777 = 0u64;
                            if nat_eq(a776, a777) {
                                Ok(8388608u64)
                            } else {
                                Ok(1056964608u64)
                            }
                        } else {
                            let a785 = {
                                let a783 = {
                                    let a781 = {
                                        let a778 = f20(v0);
                                        let a779 = 2u64;
                                        let a780 = 0u64;
                                        nat_rem(a778, a779, a780)
                                    };
                                    let a782 = 4u64;
                                    nat_mul(a781, a782)?
                                };
                                let a784 = f31(v0);
                                nat_add(a783, a784)?
                            };
                            let a786 = 6u64;
                            if nat_eq(a785, a786) {
                                let a787 = v1;
                                let a788 = 0u64;
                                if nat_eq(a787, a788) {
                                    Ok(2139095039u64)
                                } else {
                                    Ok(1073741824u64)
                                }
                            } else {
                                let a789 = v1;
                                let a790 = 0u64;
                                let _ = nat_eq(a789, a790);
                                Ok(4286578688u64)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f30(v0: u64) -> u64 {
    let a791 = v0;
    let a792 = 0u64;
    if nat_eq(a791, a792) {
        3222274048u64
    } else {
        let a793 = v0;
        let a794 = 1u64;
        if nat_eq(a793, a794) {
            3221225472u64
        } else {
            let a795 = v0;
            let a796 = 2u64;
            if nat_eq(a795, a796) {
                3219128320u64
            } else {
                let a797 = v0;
                let a798 = 3u64;
                if nat_eq(a797, a798) {
                    3217031168u64
                } else {
                    let a799 = v0;
                    let a800 = 4u64;
                    if nat_eq(a799, a800) {
                        3214934016u64
                    } else {
                        let a801 = v0;
                        let a802 = 5u64;
                        if nat_eq(a801, a802) {
                            3212836864u64
                        } else {
                            let a803 = v0;
                            let a804 = 6u64;
                            if nat_eq(a803, a804) {
                                3208642560u64
                            } else {
                                let a805 = v0;
                                let a806 = 7u64;
                                if nat_eq(a805, a806) {
                                    3204448256u64
                                } else {
                                    let a807 = v0;
                                    let a808 = 8u64;
                                    if nat_eq(a807, a808) {
                                        3196059648u64
                                    } else {
                                        let a809 = v0;
                                        let a810 = 9u64;
                                        if nat_eq(a809, a810) {
                                            0u64
                                        } else {
                                            let a811 = v0;
                                            let a812 = 10u64;
                                            if nat_eq(a811, a812) {
                                                1048576000u64
                                            } else {
                                                let a813 = v0;
                                                let a814 = 11u64;
                                                if nat_eq(a813, a814) {
                                                    1056964608u64
                                                } else {
                                                    let a815 = v0;
                                                    let a816 = 12u64;
                                                    if nat_eq(a815, a816) {
                                                        1061158912u64
                                                    } else {
                                                        let a817 = v0;
                                                        let a818 = 13u64;
                                                        if nat_eq(a817, a818) {
                                                            1065353216u64
                                                        } else {
                                                            let a819 = v0;
                                                            let a820 = 14u64;
                                                            if nat_eq(a819, a820) {
                                                                1067450368u64
                                                            } else {
                                                                let a821 = v0;
                                                                let a822 = 15u64;
                                                                if nat_eq(a821, a822) {
                                                                    1069547520u64
                                                                } else {
                                                                    let a823 = v0;
                                                                    let a824 = 16u64;
                                                                    if nat_eq(a823, a824) {
                                                                        1071644672u64
                                                                    } else {
                                                                        let a825 = v0;
                                                                        let a826 = 17u64;
                                                                        if nat_eq(a825, a826) {
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

pub fn f31(v0: u64) -> u64 {
    let a827 = v0;
    let a828 = 4u64;
    let a829 = 0u64;
    nat_rem(a827, a828, a829)
}

pub fn f32(v0: u64) -> R<u64> {
    let a837 = {
        let a835 = {
            let a830 = f19(v0);
            let a831 = 0u64;
            if nat_eq(a830, a831) {
                1u64
            } else {
                f19(v0)
            }
        };
        let a836 = {
            let a832 = f20(v0);
            let a833 = 2u64;
            let a834 = 0u64;
            nat_rem(a832, a833, a834)
        };
        nat_add(a835, a836)?
    };
    let a838 = 4u64;
    nat_mul(a837, a838)
}

pub fn f33(v0: u64, v1: u64) -> R<u64> {
    let a846 = {
        let a839 = f32(v0)?;
        let a840 = f28(v0, v1);
        nat_mul(a839, a840)?
    };
    let a847 = {
        let a844 = {
            let a841 = f20(v0);
            let a842 = 2u64;
            let a843 = 0u64;
            nat_rem(a841, a842, a843)
        };
        let a845 = 4u64;
        nat_mul(a844, a845)?
    };
    nat_add(a846, a847)
}

pub fn f34(v0: u64, v1: u64) -> R<u64> {
    let a855 = {
        let a848 = f33(v0, v1)?;
        let a849 = f21(v0, v1);
        nat_mul(a848, a849)?
    };
    let a856 = {
        let a853 = {
            let a850 = f20(v0);
            let a851 = 2u64;
            let a852 = 0u64;
            nat_rem(a850, a851, a852)
        };
        let a854 = 4u64;
        nat_mul(a853, a854)?
    };
    nat_add(a855, a856)
}

pub fn f35(v0: u64) -> u64 {
    let a857 = 2u64;
    let a858 = f20(v0);
    if nat_le(a857, a858) {
        4u64
    } else {
        0u64
    }
}

pub fn run(p0: u64, p1: u64, p2: u64, p3: u64) -> R<Adt0> {
    f0(p0, p1, p2, p3)
}
