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
    C6(u64, u64, u64, u64, u64),
}

pub fn f0(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt0> {
    let a1 = v0;
    let a2 = 35u64;
    if nat_eq(a1, a2) {
        Ok(f1(v1))
    } else {
        let a3 = v0;
        let a4 = 106u64;
        if nat_eq(a3, a4) {
            Ok(f2(v1))
        } else {
            let a5 = v0;
            let a6 = 6u64;
            if nat_eq(a5, a6) {
                f3(v1, v2, v3)
            } else {
                let a7 = v0;
                let a8 = 10u64;
                if nat_eq(a7, a8) {
                    f4(v1)
                } else {
                    let a9 = v0;
                    let a10 = 9u64;
                    if nat_eq(a9, a10) {
                        f5(v1)
                    } else {
                        let a11 = v0;
                        let a12 = 47u64;
                        if nat_eq(a11, a12) {
                            Ok(f6(v1))
                        } else {
                            let a13 = v0;
                            let a14 = 44u64;
                            if nat_eq(a13, a14) {
                                Ok(f7(v1))
                            } else {
                                let a15 = v0;
                                let a16 = 43u64;
                                if nat_eq(a15, a16) {
                                    Ok(f8(v1))
                                } else {
                                    let a17 = v0;
                                    let a18 = 42u64;
                                    if nat_eq(a17, a18) {
                                        Ok(f9(v1))
                                    } else {
                                        let a19 = v0;
                                        let a20 = 41u64;
                                        if nat_eq(a19, a20) {
                                            Ok(f10(v1))
                                        } else {
                                            let a21 = v0;
                                            let a22 = 36u64;
                                            if nat_eq(a21, a22) {
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

pub fn f1(v0: u64) -> Adt0 {
    let a23 = v0;
    let a24 = 0u64;
    if nat_eq(a23, a24) {
        Adt0::C6(1u64, 0u64, 0u64, 0u64, 0u64)
    } else {
        let a25 = v0;
        let a26 = 1u64;
        if nat_eq(a25, a26) {
            Adt0::C6(0u64, 0u64, 4u64, 0u64, 0u64)
        } else {
            let a27 = v0;
            let a28 = 2u64;
            if nat_eq(a27, a28) {
                Adt0::C6(0u64, 1u64, 1280u64, 0u64, 0u64)
            } else {
                let a29 = v0;
                let a30 = 3u64;
                if nat_eq(a29, a30) {
                    Adt0::C6(0u64, 1u64, 0u64, 24u64, 0u64)
                } else {
                    let a31 = v0;
                    let a32 = 4u64;
                    if nat_eq(a31, a32) {
                        Adt0::C6(0u64, 2u64, 0u64, 2u64, 0u64)
                    } else {
                        let a33 = v0;
                        let a34 = 5u64;
                        if nat_eq(a33, a34) {
                            Adt0::C6(0u64, 3u64, 0u64, 0u64, 0u64)
                        } else {
                            Adt0::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f2(v0: u64) -> Adt0 {
    let a35 = v0;
    let a36 = 0u64;
    if nat_eq(a35, a36) {
        Adt0::C5(1u64, 1u64, 1u64, 1u64, 4u64, 4u64, 4u64, 4u64, 0u64, 4u64, 0u64)
    } else {
        let a37 = v0;
        let a38 = 1u64;
        if nat_eq(a37, a38) {
            Adt0::C5(18446744073709551615u64, 1u64, 1u64, 1u64, 8u64, 8u64, 8u64, 8u64, 1u64, 8u64, 1u64)
        } else {
            let a39 = v0;
            let a40 = 2u64;
            if nat_eq(a39, a40) {
                Adt0::C5(18446744073709551615u64, 1u64, 1u64, 1u64, 18u64, 18u64, 18u64, 18u64, 32u64, 18u64, 1u64)
            } else {
                let a41 = v0;
                let a42 = 3u64;
                if nat_eq(a41, a42) {
                    Adt0::C5(2u64, 2u64, 1u64, 1u64, 1u64, 18446744073709551615u64, 1u64, 1u64, 1u64, 1u64, 1u64)
                } else {
                    let a43 = v0;
                    let a44 = 4u64;
                    if nat_eq(a43, a44) {
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
    let a45 = v0;
    let a46 = 48u64;
    if nat_lt(a45, a46) {
        let v3: u64 = {
            let a47 = v0;
            let a48 = 16u64;
            if nat_lt(a47, a48) {
                1u64
            } else {
                let a49 = v0;
                let a50 = 32u64;
                if nat_lt(a49, a50) {
                    2u64
                } else {
                    4u64
                }
            }
        };
        let v4: u64 = {
            let a57 = {
                let a54 = {
                    let a51 = v0;
                    let a52 = 8u64;
                    let a53 = 0u64;
                    nat_quot(a51, a52, a53)
                };
                let a55 = 2u64;
                let a56 = 0u64;
                nat_rem(a54, a55, a56)
            };
            let a58 = 0u64;
            if nat_eq(a57, a58) {
                1u64
            } else {
                3u64
            }
        };
        let v5: u64 = {
            let a65 = {
                let a62 = {
                    let a59 = v0;
                    let a60 = 4u64;
                    let a61 = 0u64;
                    nat_quot(a59, a60, a61)
                };
                let a63 = 2u64;
                let a64 = 0u64;
                nat_rem(a62, a63, a64)
            };
            let a66 = 0u64;
            if nat_eq(a65, a66) {
                1u64
            } else {
                2u64
            }
        };
        let v6: u64 = {
            let a73 = {
                let a70 = {
                    let a67 = v0;
                    let a68 = 2u64;
                    let a69 = 0u64;
                    nat_quot(a67, a68, a69)
                };
                let a71 = 2u64;
                let a72 = 0u64;
                nat_rem(a70, a71, a72)
            };
            let a74 = 0u64;
            if nat_eq(a73, a74) {
                1u64
            } else {
                2u64
            }
        };
        let v7: u64 = {
            let a78 = {
                let a75 = v0;
                let a76 = 2u64;
                let a77 = 0u64;
                nat_rem(a75, a76, a77)
            };
            let a79 = 0u64;
            if nat_eq(a78, a79) {
                0u64
            } else {
                7u64
            }
        };
        let v8: u64 = {
            let a80 = v2;
            let a81 = v7;
            nat_add(a80, a81)?
        };
        let v9: u64 = {
            let a84 = {
                let a82 = v3;
                let a83 = v8;
                nat_mul(a82, a83)?
            };
            let a85 = 11u64;
            nat_add(a84, a85)?
        };
        let v10: u64 = {
            let a88 = {
                let a86 = v4;
                let a87 = v9;
                nat_mul(a86, a87)?
            };
            let a89 = 13u64;
            nat_add(a88, a89)?
        };
        Ok(Adt0::C4({
            let a90 = v1;
            let a91 = v3;
            nat_mul(a90, a91)?
        }, v4, v5, v6, v8, v9, v10, {
            let a94 = {
                let a92 = v5;
                let a93 = v10;
                nat_mul(a92, a93)?
            };
            let a95 = 17u64;
            nat_add(a94, a95)?
        }))
    } else {
        let a96 = v0;
        let a97 = 52u64;
        if nat_lt(a96, a97) {
            Ok(Adt0::C4({
                let a98 = v0;
                let a99 = 48u64;
                if nat_eq(a98, a99) {
                    0u64
                } else {
                    v1
                }
            }, {
                let a100 = v0;
                let a101 = 49u64;
                if nat_eq(a100, a101) {
                    0u64
                } else {
                    2u64
                }
            }, {
                let a102 = v0;
                let a103 = 50u64;
                if nat_eq(a102, a103) {
                    0u64
                } else {
                    2u64
                }
            }, {
                let a104 = v0;
                let a105 = 51u64;
                if nat_eq(a104, a105) {
                    0u64
                } else {
                    2u64
                }
            }, v2, {
                let a106 = v2;
                let a107 = 2u64;
                nat_mul(a106, a107)?
            }, {
                let a108 = v2;
                let a109 = 4u64;
                nat_mul(a108, a109)?
            }, {
                let a110 = v2;
                let a111 = 8u64;
                nat_mul(a110, a111)?
            }))
        } else {
            Ok(Adt0::C0)
        }
    }
}

pub fn f4(v0: u64) -> R<Adt0> {
    let a112 = v0;
    let a113 = 864u64;
    if nat_lt(a112, a113) {
        let v1: u64 = {
            let a117 = {
                let a114 = v0;
                let a115 = 108u64;
                let a116 = 0u64;
                nat_quot(a114, a115, a116)
            };
            let a118 = 0u64;
            if nat_eq(a117, a118) {
                1u64
            } else {
                let a122 = {
                    let a119 = v0;
                    let a120 = 108u64;
                    let a121 = 0u64;
                    nat_quot(a119, a120, a121)
                };
                let a123 = 1u64;
                if nat_eq(a122, a123) {
                    2u64
                } else {
                    let a127 = {
                        let a124 = v0;
                        let a125 = 108u64;
                        let a126 = 0u64;
                        nat_quot(a124, a125, a126)
                    };
                    let a128 = 2u64;
                    if nat_eq(a127, a128) {
                        4u64
                    } else {
                        let a132 = {
                            let a129 = v0;
                            let a130 = 108u64;
                            let a131 = 0u64;
                            nat_quot(a129, a130, a131)
                        };
                        let a133 = 3u64;
                        if nat_eq(a132, a133) {
                            8u64
                        } else {
                            let a137 = {
                                let a134 = v0;
                                let a135 = 108u64;
                                let a136 = 0u64;
                                nat_quot(a134, a135, a136)
                            };
                            let a138 = 4u64;
                            if nat_eq(a137, a138) {
                                16u64
                            } else {
                                let a142 = {
                                    let a139 = v0;
                                    let a140 = 108u64;
                                    let a141 = 0u64;
                                    nat_quot(a139, a140, a141)
                                };
                                let a143 = 5u64;
                                if nat_eq(a142, a143) {
                                    64u64
                                } else {
                                    let a147 = {
                                        let a144 = v0;
                                        let a145 = 108u64;
                                        let a146 = 0u64;
                                        nat_quot(a144, a145, a146)
                                    };
                                    let a148 = 6u64;
                                    if nat_eq(a147, a148) {
                                        256u64
                                    } else {
                                        let a152 = {
                                            let a149 = v0;
                                            let a150 = 108u64;
                                            let a151 = 0u64;
                                            nat_quot(a149, a150, a151)
                                        };
                                        let a153 = 7u64;
                                        if nat_eq(a152, a153) {
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
            let a160 = {
                let a157 = {
                    let a154 = v0;
                    let a155 = 36u64;
                    let a156 = 0u64;
                    nat_quot(a154, a155, a156)
                };
                let a158 = 3u64;
                let a159 = 0u64;
                nat_rem(a157, a158, a159)
            };
            let a161 = 0u64;
            if nat_eq(a160, a161) {
                0u64
            } else {
                let a168 = {
                    let a165 = {
                        let a162 = v0;
                        let a163 = 36u64;
                        let a164 = 0u64;
                        nat_quot(a162, a163, a164)
                    };
                    let a166 = 3u64;
                    let a167 = 0u64;
                    nat_rem(a165, a166, a167)
                };
                let a169 = 1u64;
                if nat_eq(a168, a169) {
                    v1
                } else {
                    let a176 = {
                        let a173 = {
                            let a170 = v0;
                            let a171 = 36u64;
                            let a172 = 0u64;
                            nat_quot(a170, a171, a172)
                        };
                        let a174 = 3u64;
                        let a175 = 0u64;
                        nat_rem(a173, a174, a175)
                    };
                    let a177 = 2u64;
                    if nat_eq(a176, a177) {
                        let a178 = v1;
                        let a179 = 3u64;
                        nat_mul(a178, a179)?
                    } else {
                        0u64
                    }
                }
            }
        };
        let v3: u64 = {
            let a186 = {
                let a183 = {
                    let a180 = v0;
                    let a181 = 6u64;
                    let a182 = 0u64;
                    nat_quot(a180, a181, a182)
                };
                let a184 = 6u64;
                let a185 = 0u64;
                nat_rem(a183, a184, a185)
            };
            let a187 = 0u64;
            if nat_eq(a186, a187) {
                0u64
            } else {
                let a194 = {
                    let a191 = {
                        let a188 = v0;
                        let a189 = 6u64;
                        let a190 = 0u64;
                        nat_quot(a188, a189, a190)
                    };
                    let a192 = 6u64;
                    let a193 = 0u64;
                    nat_rem(a191, a192, a193)
                };
                let a195 = 1u64;
                if nat_eq(a194, a195) {
                    1u64
                } else {
                    let a202 = {
                        let a199 = {
                            let a196 = v0;
                            let a197 = 6u64;
                            let a198 = 0u64;
                            nat_quot(a196, a197, a198)
                        };
                        let a200 = 6u64;
                        let a201 = 0u64;
                        nat_rem(a199, a200, a201)
                    };
                    let a203 = 2u64;
                    if nat_eq(a202, a203) {
                        let a204 = v1;
                        let a205 = 1u64;
                        nat_sub(a204, a205)
                    } else {
                        let a212 = {
                            let a209 = {
                                let a206 = v0;
                                let a207 = 6u64;
                                let a208 = 0u64;
                                nat_quot(a206, a207, a208)
                            };
                            let a210 = 6u64;
                            let a211 = 0u64;
                            nat_rem(a209, a210, a211)
                        };
                        let a213 = 3u64;
                        if nat_eq(a212, a213) {
                            v1
                        } else {
                            let a220 = {
                                let a217 = {
                                    let a214 = v0;
                                    let a215 = 6u64;
                                    let a216 = 0u64;
                                    nat_quot(a214, a215, a216)
                                };
                                let a218 = 6u64;
                                let a219 = 0u64;
                                nat_rem(a217, a218, a219)
                            };
                            let a221 = 4u64;
                            if nat_eq(a220, a221) {
                                let a222 = v1;
                                let a223 = 1u64;
                                nat_add(a222, a223)?
                            } else {
                                let a230 = {
                                    let a227 = {
                                        let a224 = v0;
                                        let a225 = 6u64;
                                        let a226 = 0u64;
                                        nat_quot(a224, a225, a226)
                                    };
                                    let a228 = 6u64;
                                    let a229 = 0u64;
                                    nat_rem(a227, a228, a229)
                                };
                                let a231 = 5u64;
                                if nat_eq(a230, a231) {
                                    let a232 = v1;
                                    let a233 = 4u64;
                                    nat_mul(a232, a233)?
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
            let a237 = {
                let a234 = v0;
                let a235 = 6u64;
                let a236 = 0u64;
                nat_rem(a234, a235, a236)
            };
            let a238 = 0u64;
            if nat_eq(a237, a238) {
                0u64
            } else {
                let a242 = {
                    let a239 = v0;
                    let a240 = 6u64;
                    let a241 = 0u64;
                    nat_rem(a239, a240, a241)
                };
                let a243 = 1u64;
                if nat_eq(a242, a243) {
                    1u64
                } else {
                    let a247 = {
                        let a244 = v0;
                        let a245 = 6u64;
                        let a246 = 0u64;
                        nat_rem(a244, a245, a246)
                    };
                    let a248 = 2u64;
                    if nat_eq(a247, a248) {
                        let a249 = v1;
                        let a250 = 1u64;
                        nat_sub(a249, a250)
                    } else {
                        let a254 = {
                            let a251 = v0;
                            let a252 = 6u64;
                            let a253 = 0u64;
                            nat_rem(a251, a252, a253)
                        };
                        let a255 = 3u64;
                        if nat_eq(a254, a255) {
                            v1
                        } else {
                            let a259 = {
                                let a256 = v0;
                                let a257 = 6u64;
                                let a258 = 0u64;
                                nat_rem(a256, a257, a258)
                            };
                            let a260 = 4u64;
                            if nat_eq(a259, a260) {
                                let a261 = v1;
                                let a262 = 1u64;
                                nat_add(a261, a262)?
                            } else {
                                let a266 = {
                                    let a263 = v0;
                                    let a264 = 6u64;
                                    let a265 = 0u64;
                                    nat_rem(a263, a264, a265)
                                };
                                let a267 = 5u64;
                                if nat_eq(a266, a267) {
                                    let a270 = {
                                        let a268 = v1;
                                        let a269 = 2u64;
                                        nat_mul(a268, a269)?
                                    };
                                    let a271 = 1u64;
                                    nat_add(a270, a271)?
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
    let a272 = v0;
    let a273 = 1024u64;
    if nat_lt(a272, a273) {
        let v1: u64 = {
            let a280 = {
                let a277 = {
                    let a274 = v0;
                    let a275 = 16u64;
                    let a276 = 0u64;
                    nat_quot(a274, a275, a276)
                };
                let a278 = 4u64;
                let a279 = 0u64;
                nat_rem(a277, a278, a279)
            };
            let a281 = 0u64;
            if nat_eq(a280, a281) {
                0u64
            } else {
                let a288 = {
                    let a285 = {
                        let a282 = v0;
                        let a283 = 16u64;
                        let a284 = 0u64;
                        nat_quot(a282, a283, a284)
                    };
                    let a286 = 4u64;
                    let a287 = 0u64;
                    nat_rem(a285, a286, a287)
                };
                let a289 = 1u64;
                if nat_eq(a288, a289) {
                    1u64
                } else {
                    let a296 = {
                        let a293 = {
                            let a290 = v0;
                            let a291 = 16u64;
                            let a292 = 0u64;
                            nat_quot(a290, a291, a292)
                        };
                        let a294 = 4u64;
                        let a295 = 0u64;
                        nat_rem(a293, a294, a295)
                    };
                    let a297 = 2u64;
                    if nat_eq(a296, a297) {
                        8u64
                    } else {
                        let a304 = {
                            let a301 = {
                                let a298 = v0;
                                let a299 = 16u64;
                                let a300 = 0u64;
                                nat_quot(a298, a299, a300)
                            };
                            let a302 = 4u64;
                            let a303 = 0u64;
                            nat_rem(a301, a302, a303)
                        };
                        let a305 = 3u64;
                        if nat_eq(a304, a305) {
                            64u64
                        } else {
                            0u64
                        }
                    }
                }
            }
        };
        let v2: u64 = {
            let a312 = {
                let a309 = {
                    let a306 = v0;
                    let a307 = 4u64;
                    let a308 = 0u64;
                    nat_quot(a306, a307, a308)
                };
                let a310 = 4u64;
                let a311 = 0u64;
                nat_rem(a309, a310, a311)
            };
            let a313 = 0u64;
            if nat_eq(a312, a313) {
                0u64
            } else {
                let a320 = {
                    let a317 = {
                        let a314 = v0;
                        let a315 = 4u64;
                        let a316 = 0u64;
                        nat_quot(a314, a315, a316)
                    };
                    let a318 = 4u64;
                    let a319 = 0u64;
                    nat_rem(a317, a318, a319)
                };
                let a321 = 1u64;
                if nat_eq(a320, a321) {
                    1u64
                } else {
                    let a328 = {
                        let a325 = {
                            let a322 = v0;
                            let a323 = 4u64;
                            let a324 = 0u64;
                            nat_quot(a322, a323, a324)
                        };
                        let a326 = 4u64;
                        let a327 = 0u64;
                        nat_rem(a325, a326, a327)
                    };
                    let a329 = 2u64;
                    if nat_eq(a328, a329) {
                        v1
                    } else {
                        let a336 = {
                            let a333 = {
                                let a330 = v0;
                                let a331 = 4u64;
                                let a332 = 0u64;
                                nat_quot(a330, a331, a332)
                            };
                            let a334 = 4u64;
                            let a335 = 0u64;
                            nat_rem(a333, a334, a335)
                        };
                        let a337 = 3u64;
                        if nat_eq(a336, a337) {
                            let a338 = v1;
                            let a339 = 1u64;
                            nat_add(a338, a339)?
                        } else {
                            0u64
                        }
                    }
                }
            }
        };
        let v3: u64 = {
            let a343 = {
                let a340 = v0;
                let a341 = 4u64;
                let a342 = 0u64;
                nat_rem(a340, a341, a342)
            };
            let a344 = 0u64;
            if nat_eq(a343, a344) {
                0u64
            } else {
                let a348 = {
                    let a345 = v0;
                    let a346 = 4u64;
                    let a347 = 0u64;
                    nat_rem(a345, a346, a347)
                };
                let a349 = 1u64;
                if nat_eq(a348, a349) {
                    1u64
                } else {
                    let a353 = {
                        let a350 = v0;
                        let a351 = 4u64;
                        let a352 = 0u64;
                        nat_rem(a350, a351, a352)
                    };
                    let a354 = 2u64;
                    if nat_eq(a353, a354) {
                        v1
                    } else {
                        let a358 = {
                            let a355 = v0;
                            let a356 = 4u64;
                            let a357 = 0u64;
                            nat_rem(a355, a356, a357)
                        };
                        let a359 = 3u64;
                        if nat_eq(a358, a359) {
                            let a360 = v1;
                            let a361 = 1u64;
                            nat_add(a360, a361)?
                        } else {
                            0u64
                        }
                    }
                }
            }
        };
        Ok(Adt0::C3({
            let a362 = v0;
            let a363 = 256u64;
            let a364 = 0u64;
            nat_quot(a362, a363, a364)
        }, {
            let a368 = {
                let a365 = v0;
                let a366 = 128u64;
                let a367 = 0u64;
                nat_quot(a365, a366, a367)
            };
            let a369 = 2u64;
            let a370 = 0u64;
            nat_rem(a368, a369, a370)
        }, {
            let a374 = {
                let a371 = v0;
                let a372 = 64u64;
                let a373 = 0u64;
                nat_quot(a371, a372, a373)
            };
            let a375 = 2u64;
            let a376 = 0u64;
            nat_rem(a374, a375, a376)
        }, v1, v2, v3))
    } else {
        Ok(Adt0::C0)
    }
}

pub fn f6(v0: u64) -> Adt0 {
    let a377 = v0;
    let a378 = 0u64;
    if nat_eq(a377, a378) {
        Adt0::C1(3u64, 21u64, 3u64, 0u64, Some(4294967294u64))
    } else {
        let a379 = v0;
        let a380 = 1u64;
        if nat_eq(a379, a380) {
            Adt0::C1(0u64, 22u64, 3u64, 0u64, None::<u64>)
        } else {
            let a381 = v0;
            let a382 = 2u64;
            if nat_eq(a381, a382) {
                Adt0::C1(0u64, 0u64, 2147483648u64, 0u64, None::<u64>)
            } else {
                let a383 = v0;
                let a384 = 3u64;
                if nat_eq(a383, a384) {
                    Adt0::C1(4u64, 0u64, 0u64, 0u64, None::<u64>)
                } else {
                    Adt0::C0
                }
            }
        }
    }
}

pub fn f7(v0: u64) -> Adt0 {
    let a385 = v0;
    let a386 = 0u64;
    if nat_eq(a385, a386) {
        Adt0::C1(24u64, 0u64, 0u64, 0u64, Some(15360u64))
    } else {
        let a387 = v0;
        let a388 = 1u64;
        if nat_eq(a387, a388) {
            Adt0::C1(31u64, 0u64, 0u64, 0u64, Some(1920u64))
        } else {
            let a389 = v0;
            let a390 = 2u64;
            if nat_eq(a389, a390) {
                Adt0::C1(25u64, 15360u64, 0u64, 0u64, None::<u64>)
            } else {
                let a391 = v0;
                let a392 = 3u64;
                if nat_eq(a391, a392) {
                    Adt0::C1(32u64, 1920u64, 0u64, 0u64, None::<u64>)
                } else {
                    let a393 = v0;
                    let a394 = 4u64;
                    if nat_eq(a393, a394) {
                        Adt0::C1(26u64, 0u64, 2u64, 0u64, None::<u64>)
                    } else {
                        let a395 = v0;
                        let a396 = 5u64;
                        if nat_eq(a395, a396) {
                            Adt0::C1(33u64, 0u64, 2u64, 0u64, None::<u64>)
                        } else {
                            let a397 = v0;
                            let a398 = 6u64;
                            if nat_eq(a397, a398) {
                                Adt0::C1(27u64, 0u64, 1u64, 0u64, None::<u64>)
                            } else {
                                let a399 = v0;
                                let a400 = 7u64;
                                if nat_eq(a399, a400) {
                                    Adt0::C1(34u64, 0u64, 0u64, 192u64, None::<u64>)
                                } else {
                                    let a401 = v0;
                                    let a402 = 8u64;
                                    if nat_eq(a401, a402) {
                                        Adt0::C1(30u64, 0u64, 0u64, 0u64, None::<u64>)
                                    } else {
                                        let a403 = v0;
                                        let a404 = 9u64;
                                        if nat_eq(a403, a404) {
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
    let a405 = v0;
    let a406 = 0u64;
    if nat_eq(a405, a406) {
        Adt0::C1(17u64, 0u64, 0u64, 0u64, Some(10368u64))
    } else {
        let a407 = v0;
        let a408 = 1u64;
        if nat_eq(a407, a408) {
            Adt0::C1(18u64, 10368u64, 0u64, 0u64, None::<u64>)
        } else {
            let a409 = v0;
            let a410 = 2u64;
            if nat_eq(a409, a410) {
                Adt0::C1(19u64, 0u64, 4u64, 0u64, None::<u64>)
            } else {
                let a411 = v0;
                let a412 = 3u64;
                if nat_eq(a411, a412) {
                    Adt0::C1(20u64, 0u64, 2u64, 0u64, None::<u64>)
                } else {
                    let a413 = v0;
                    let a414 = 4u64;
                    if nat_eq(a413, a414) {
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
    let a415 = v0;
    let a416 = 0u64;
    if nat_eq(a415, a416) {
        Adt0::C1(11u64, 0u64, 0u64, 0u64, Some(160u64))
    } else {
        let a417 = v0;
        let a418 = 1u64;
        if nat_eq(a417, a418) {
            Adt0::C1(12u64, 160u64, 0u64, 0u64, None::<u64>)
        } else {
            let a419 = v0;
            let a420 = 2u64;
            if nat_eq(a419, a420) {
                Adt0::C1(13u64, 0u64, 4u64, 0u64, None::<u64>)
            } else {
                let a421 = v0;
                let a422 = 3u64;
                if nat_eq(a421, a422) {
                    Adt0::C1(14u64, 0u64, 0u64, 256u64, None::<u64>)
                } else {
                    let a423 = v0;
                    let a424 = 4u64;
                    if nat_eq(a423, a424) {
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
    let a425 = v0;
    let a426 = 0u64;
    if nat_eq(a425, a426) {
        Adt0::C1(5u64, 0u64, 0u64, 0u64, Some(3456u64))
    } else {
        let a427 = v0;
        let a428 = 1u64;
        if nat_eq(a427, a428) {
            Adt0::C1(5u64, 0u64, 0u64, 5u64, None::<u64>)
        } else {
            let a429 = v0;
            let a430 = 2u64;
            if nat_eq(a429, a430) {
                Adt0::C1(5u64, 1u64, 0u64, 0u64, None::<u64>)
            } else {
                let a431 = v0;
                let a432 = 3u64;
                if nat_eq(a431, a432) {
                    Adt0::C1(6u64, 3456u64, 0u64, 0u64, None::<u64>)
                } else {
                    let a433 = v0;
                    let a434 = 4u64;
                    if nat_eq(a433, a434) {
                        Adt0::C1(6u64, 0u64, 0u64, 9u64, None::<u64>)
                    } else {
                        let a435 = v0;
                        let a436 = 5u64;
                        if nat_eq(a435, a436) {
                            Adt0::C1(7u64, 0u64, 2u64, 0u64, None::<u64>)
                        } else {
                            let a437 = v0;
                            let a438 = 6u64;
                            if nat_eq(a437, a438) {
                                Adt0::C1(7u64, 0u64, 0u64, 10u64, None::<u64>)
                            } else {
                                let a439 = v0;
                                let a440 = 7u64;
                                if nat_eq(a439, a440) {
                                    Adt0::C1(8u64, 0u64, 0u64, 256u64, None::<u64>)
                                } else {
                                    let a441 = v0;
                                    let a442 = 8u64;
                                    if nat_eq(a441, a442) {
                                        Adt0::C1(9u64, 215u64, 8u64, 0u64, None::<u64>)
                                    } else {
                                        let a443 = v0;
                                        let a444 = 9u64;
                                        if nat_eq(a443, a444) {
                                            Adt0::C1(10u64, 0u64, 0u64, 0u64, None::<u64>)
                                        } else {
                                            let a445 = v0;
                                            let a446 = 10u64;
                                            if nat_eq(a445, a446) {
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
    let a447 = v0;
    let a448 = 0u64;
    if nat_eq(a447, a448) {
        Adt0::C1(1u64, 120u64, 0u64, 0u64, None::<u64>)
    } else {
        let a449 = v0;
        let a450 = 1u64;
        if nat_eq(a449, a450) {
            Adt0::C1(1u64, 0u64, 0u64, 24u64, None::<u64>)
        } else {
            let a451 = v0;
            let a452 = 2u64;
            if nat_eq(a451, a452) {
                Adt0::C1(2u64, 0u64, 2u64, 0u64, None::<u64>)
            } else {
                let a453 = v0;
                let a454 = 3u64;
                if nat_eq(a453, a454) {
                    Adt0::C1(3u64, 96u64, 1u64, 0u64, None::<u64>)
                } else {
                    let a455 = v0;
                    let a456 = 4u64;
                    if nat_eq(a455, a456) {
                        Adt0::C1(4u64, 0u64, 0u64, 0u64, None::<u64>)
                    } else {
                        Adt0::C0
                    }
                }
            }
        }
    }
}

pub fn run(p0: u64, p1: u64, p2: u64, p3: u64) -> R<Adt0> {
    f0(p0, p1, p2, p3)
}
