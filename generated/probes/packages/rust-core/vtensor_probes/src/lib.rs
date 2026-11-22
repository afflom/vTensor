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
}

pub fn f0(v0: u64, v1: u64) -> R<Adt0> {
    let a1 = v0;
    let a2 = 10u64;
    if nat_eq(a1, a2) {
        f1(v1)
    } else {
        let a3 = v0;
        let a4 = 9u64;
        if nat_eq(a3, a4) {
            f2(v1)
        } else {
            let a5 = v0;
            let a6 = 47u64;
            if nat_eq(a5, a6) {
                Ok(f3(v1))
            } else {
                let a7 = v0;
                let a8 = 44u64;
                if nat_eq(a7, a8) {
                    Ok(f4(v1))
                } else {
                    let a9 = v0;
                    let a10 = 43u64;
                    if nat_eq(a9, a10) {
                        Ok(f5(v1))
                    } else {
                        let a11 = v0;
                        let a12 = 42u64;
                        if nat_eq(a11, a12) {
                            Ok(f6(v1))
                        } else {
                            let a13 = v0;
                            let a14 = 41u64;
                            if nat_eq(a13, a14) {
                                Ok(f7(v1))
                            } else {
                                let a15 = v0;
                                let a16 = 36u64;
                                if nat_eq(a15, a16) {
                                    Ok(f8(v1))
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

pub fn f1(v0: u64) -> R<Adt0> {
    let a17 = v0;
    let a18 = 864u64;
    if nat_lt(a17, a18) {
        let v1: u64 = {
            let a22 = {
                let a19 = v0;
                let a20 = 108u64;
                let a21 = 0u64;
                nat_quot(a19, a20, a21)
            };
            let a23 = 0u64;
            if nat_eq(a22, a23) {
                1u64
            } else {
                let a27 = {
                    let a24 = v0;
                    let a25 = 108u64;
                    let a26 = 0u64;
                    nat_quot(a24, a25, a26)
                };
                let a28 = 1u64;
                if nat_eq(a27, a28) {
                    2u64
                } else {
                    let a32 = {
                        let a29 = v0;
                        let a30 = 108u64;
                        let a31 = 0u64;
                        nat_quot(a29, a30, a31)
                    };
                    let a33 = 2u64;
                    if nat_eq(a32, a33) {
                        4u64
                    } else {
                        let a37 = {
                            let a34 = v0;
                            let a35 = 108u64;
                            let a36 = 0u64;
                            nat_quot(a34, a35, a36)
                        };
                        let a38 = 3u64;
                        if nat_eq(a37, a38) {
                            8u64
                        } else {
                            let a42 = {
                                let a39 = v0;
                                let a40 = 108u64;
                                let a41 = 0u64;
                                nat_quot(a39, a40, a41)
                            };
                            let a43 = 4u64;
                            if nat_eq(a42, a43) {
                                16u64
                            } else {
                                let a47 = {
                                    let a44 = v0;
                                    let a45 = 108u64;
                                    let a46 = 0u64;
                                    nat_quot(a44, a45, a46)
                                };
                                let a48 = 5u64;
                                if nat_eq(a47, a48) {
                                    64u64
                                } else {
                                    let a52 = {
                                        let a49 = v0;
                                        let a50 = 108u64;
                                        let a51 = 0u64;
                                        nat_quot(a49, a50, a51)
                                    };
                                    let a53 = 6u64;
                                    if nat_eq(a52, a53) {
                                        256u64
                                    } else {
                                        let a57 = {
                                            let a54 = v0;
                                            let a55 = 108u64;
                                            let a56 = 0u64;
                                            nat_quot(a54, a55, a56)
                                        };
                                        let a58 = 7u64;
                                        if nat_eq(a57, a58) {
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
            let a65 = {
                let a62 = {
                    let a59 = v0;
                    let a60 = 36u64;
                    let a61 = 0u64;
                    nat_quot(a59, a60, a61)
                };
                let a63 = 3u64;
                let a64 = 0u64;
                nat_rem(a62, a63, a64)
            };
            let a66 = 0u64;
            if nat_eq(a65, a66) {
                0u64
            } else {
                let a73 = {
                    let a70 = {
                        let a67 = v0;
                        let a68 = 36u64;
                        let a69 = 0u64;
                        nat_quot(a67, a68, a69)
                    };
                    let a71 = 3u64;
                    let a72 = 0u64;
                    nat_rem(a70, a71, a72)
                };
                let a74 = 1u64;
                if nat_eq(a73, a74) {
                    v1
                } else {
                    let a81 = {
                        let a78 = {
                            let a75 = v0;
                            let a76 = 36u64;
                            let a77 = 0u64;
                            nat_quot(a75, a76, a77)
                        };
                        let a79 = 3u64;
                        let a80 = 0u64;
                        nat_rem(a78, a79, a80)
                    };
                    let a82 = 2u64;
                    if nat_eq(a81, a82) {
                        let a83 = v1;
                        let a84 = 3u64;
                        nat_mul(a83, a84)?
                    } else {
                        0u64
                    }
                }
            }
        };
        let v3: u64 = {
            let a91 = {
                let a88 = {
                    let a85 = v0;
                    let a86 = 6u64;
                    let a87 = 0u64;
                    nat_quot(a85, a86, a87)
                };
                let a89 = 6u64;
                let a90 = 0u64;
                nat_rem(a88, a89, a90)
            };
            let a92 = 0u64;
            if nat_eq(a91, a92) {
                0u64
            } else {
                let a99 = {
                    let a96 = {
                        let a93 = v0;
                        let a94 = 6u64;
                        let a95 = 0u64;
                        nat_quot(a93, a94, a95)
                    };
                    let a97 = 6u64;
                    let a98 = 0u64;
                    nat_rem(a96, a97, a98)
                };
                let a100 = 1u64;
                if nat_eq(a99, a100) {
                    1u64
                } else {
                    let a107 = {
                        let a104 = {
                            let a101 = v0;
                            let a102 = 6u64;
                            let a103 = 0u64;
                            nat_quot(a101, a102, a103)
                        };
                        let a105 = 6u64;
                        let a106 = 0u64;
                        nat_rem(a104, a105, a106)
                    };
                    let a108 = 2u64;
                    if nat_eq(a107, a108) {
                        let a109 = v1;
                        let a110 = 1u64;
                        nat_sub(a109, a110)
                    } else {
                        let a117 = {
                            let a114 = {
                                let a111 = v0;
                                let a112 = 6u64;
                                let a113 = 0u64;
                                nat_quot(a111, a112, a113)
                            };
                            let a115 = 6u64;
                            let a116 = 0u64;
                            nat_rem(a114, a115, a116)
                        };
                        let a118 = 3u64;
                        if nat_eq(a117, a118) {
                            v1
                        } else {
                            let a125 = {
                                let a122 = {
                                    let a119 = v0;
                                    let a120 = 6u64;
                                    let a121 = 0u64;
                                    nat_quot(a119, a120, a121)
                                };
                                let a123 = 6u64;
                                let a124 = 0u64;
                                nat_rem(a122, a123, a124)
                            };
                            let a126 = 4u64;
                            if nat_eq(a125, a126) {
                                let a127 = v1;
                                let a128 = 1u64;
                                nat_add(a127, a128)?
                            } else {
                                let a135 = {
                                    let a132 = {
                                        let a129 = v0;
                                        let a130 = 6u64;
                                        let a131 = 0u64;
                                        nat_quot(a129, a130, a131)
                                    };
                                    let a133 = 6u64;
                                    let a134 = 0u64;
                                    nat_rem(a132, a133, a134)
                                };
                                let a136 = 5u64;
                                if nat_eq(a135, a136) {
                                    let a137 = v1;
                                    let a138 = 4u64;
                                    nat_mul(a137, a138)?
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
            let a142 = {
                let a139 = v0;
                let a140 = 6u64;
                let a141 = 0u64;
                nat_rem(a139, a140, a141)
            };
            let a143 = 0u64;
            if nat_eq(a142, a143) {
                0u64
            } else {
                let a147 = {
                    let a144 = v0;
                    let a145 = 6u64;
                    let a146 = 0u64;
                    nat_rem(a144, a145, a146)
                };
                let a148 = 1u64;
                if nat_eq(a147, a148) {
                    1u64
                } else {
                    let a152 = {
                        let a149 = v0;
                        let a150 = 6u64;
                        let a151 = 0u64;
                        nat_rem(a149, a150, a151)
                    };
                    let a153 = 2u64;
                    if nat_eq(a152, a153) {
                        let a154 = v1;
                        let a155 = 1u64;
                        nat_sub(a154, a155)
                    } else {
                        let a159 = {
                            let a156 = v0;
                            let a157 = 6u64;
                            let a158 = 0u64;
                            nat_rem(a156, a157, a158)
                        };
                        let a160 = 3u64;
                        if nat_eq(a159, a160) {
                            v1
                        } else {
                            let a164 = {
                                let a161 = v0;
                                let a162 = 6u64;
                                let a163 = 0u64;
                                nat_rem(a161, a162, a163)
                            };
                            let a165 = 4u64;
                            if nat_eq(a164, a165) {
                                let a166 = v1;
                                let a167 = 1u64;
                                nat_add(a166, a167)?
                            } else {
                                let a171 = {
                                    let a168 = v0;
                                    let a169 = 6u64;
                                    let a170 = 0u64;
                                    nat_rem(a168, a169, a170)
                                };
                                let a172 = 5u64;
                                if nat_eq(a171, a172) {
                                    let a175 = {
                                        let a173 = v1;
                                        let a174 = 2u64;
                                        nat_mul(a173, a174)?
                                    };
                                    let a176 = 1u64;
                                    nat_add(a175, a176)?
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

pub fn f2(v0: u64) -> R<Adt0> {
    let a177 = v0;
    let a178 = 1024u64;
    if nat_lt(a177, a178) {
        let v1: u64 = {
            let a185 = {
                let a182 = {
                    let a179 = v0;
                    let a180 = 16u64;
                    let a181 = 0u64;
                    nat_quot(a179, a180, a181)
                };
                let a183 = 4u64;
                let a184 = 0u64;
                nat_rem(a182, a183, a184)
            };
            let a186 = 0u64;
            if nat_eq(a185, a186) {
                0u64
            } else {
                let a193 = {
                    let a190 = {
                        let a187 = v0;
                        let a188 = 16u64;
                        let a189 = 0u64;
                        nat_quot(a187, a188, a189)
                    };
                    let a191 = 4u64;
                    let a192 = 0u64;
                    nat_rem(a190, a191, a192)
                };
                let a194 = 1u64;
                if nat_eq(a193, a194) {
                    1u64
                } else {
                    let a201 = {
                        let a198 = {
                            let a195 = v0;
                            let a196 = 16u64;
                            let a197 = 0u64;
                            nat_quot(a195, a196, a197)
                        };
                        let a199 = 4u64;
                        let a200 = 0u64;
                        nat_rem(a198, a199, a200)
                    };
                    let a202 = 2u64;
                    if nat_eq(a201, a202) {
                        8u64
                    } else {
                        let a209 = {
                            let a206 = {
                                let a203 = v0;
                                let a204 = 16u64;
                                let a205 = 0u64;
                                nat_quot(a203, a204, a205)
                            };
                            let a207 = 4u64;
                            let a208 = 0u64;
                            nat_rem(a206, a207, a208)
                        };
                        let a210 = 3u64;
                        if nat_eq(a209, a210) {
                            64u64
                        } else {
                            0u64
                        }
                    }
                }
            }
        };
        let v2: u64 = {
            let a217 = {
                let a214 = {
                    let a211 = v0;
                    let a212 = 4u64;
                    let a213 = 0u64;
                    nat_quot(a211, a212, a213)
                };
                let a215 = 4u64;
                let a216 = 0u64;
                nat_rem(a214, a215, a216)
            };
            let a218 = 0u64;
            if nat_eq(a217, a218) {
                0u64
            } else {
                let a225 = {
                    let a222 = {
                        let a219 = v0;
                        let a220 = 4u64;
                        let a221 = 0u64;
                        nat_quot(a219, a220, a221)
                    };
                    let a223 = 4u64;
                    let a224 = 0u64;
                    nat_rem(a222, a223, a224)
                };
                let a226 = 1u64;
                if nat_eq(a225, a226) {
                    1u64
                } else {
                    let a233 = {
                        let a230 = {
                            let a227 = v0;
                            let a228 = 4u64;
                            let a229 = 0u64;
                            nat_quot(a227, a228, a229)
                        };
                        let a231 = 4u64;
                        let a232 = 0u64;
                        nat_rem(a230, a231, a232)
                    };
                    let a234 = 2u64;
                    if nat_eq(a233, a234) {
                        v1
                    } else {
                        let a241 = {
                            let a238 = {
                                let a235 = v0;
                                let a236 = 4u64;
                                let a237 = 0u64;
                                nat_quot(a235, a236, a237)
                            };
                            let a239 = 4u64;
                            let a240 = 0u64;
                            nat_rem(a238, a239, a240)
                        };
                        let a242 = 3u64;
                        if nat_eq(a241, a242) {
                            let a243 = v1;
                            let a244 = 1u64;
                            nat_add(a243, a244)?
                        } else {
                            0u64
                        }
                    }
                }
            }
        };
        let v3: u64 = {
            let a248 = {
                let a245 = v0;
                let a246 = 4u64;
                let a247 = 0u64;
                nat_rem(a245, a246, a247)
            };
            let a249 = 0u64;
            if nat_eq(a248, a249) {
                0u64
            } else {
                let a253 = {
                    let a250 = v0;
                    let a251 = 4u64;
                    let a252 = 0u64;
                    nat_rem(a250, a251, a252)
                };
                let a254 = 1u64;
                if nat_eq(a253, a254) {
                    1u64
                } else {
                    let a258 = {
                        let a255 = v0;
                        let a256 = 4u64;
                        let a257 = 0u64;
                        nat_rem(a255, a256, a257)
                    };
                    let a259 = 2u64;
                    if nat_eq(a258, a259) {
                        v1
                    } else {
                        let a263 = {
                            let a260 = v0;
                            let a261 = 4u64;
                            let a262 = 0u64;
                            nat_rem(a260, a261, a262)
                        };
                        let a264 = 3u64;
                        if nat_eq(a263, a264) {
                            let a265 = v1;
                            let a266 = 1u64;
                            nat_add(a265, a266)?
                        } else {
                            0u64
                        }
                    }
                }
            }
        };
        Ok(Adt0::C3({
            let a267 = v0;
            let a268 = 256u64;
            let a269 = 0u64;
            nat_quot(a267, a268, a269)
        }, {
            let a273 = {
                let a270 = v0;
                let a271 = 128u64;
                let a272 = 0u64;
                nat_quot(a270, a271, a272)
            };
            let a274 = 2u64;
            let a275 = 0u64;
            nat_rem(a273, a274, a275)
        }, {
            let a279 = {
                let a276 = v0;
                let a277 = 64u64;
                let a278 = 0u64;
                nat_quot(a276, a277, a278)
            };
            let a280 = 2u64;
            let a281 = 0u64;
            nat_rem(a279, a280, a281)
        }, v1, v2, v3))
    } else {
        Ok(Adt0::C0)
    }
}

pub fn f3(v0: u64) -> Adt0 {
    let a282 = v0;
    let a283 = 0u64;
    if nat_eq(a282, a283) {
        Adt0::C1(3u64, 21u64, 3u64, 0u64, Some(4294967294u64))
    } else {
        let a284 = v0;
        let a285 = 1u64;
        if nat_eq(a284, a285) {
            Adt0::C1(0u64, 22u64, 3u64, 0u64, None::<u64>)
        } else {
            let a286 = v0;
            let a287 = 2u64;
            if nat_eq(a286, a287) {
                Adt0::C1(0u64, 0u64, 2147483648u64, 0u64, None::<u64>)
            } else {
                let a288 = v0;
                let a289 = 3u64;
                if nat_eq(a288, a289) {
                    Adt0::C1(4u64, 0u64, 0u64, 0u64, None::<u64>)
                } else {
                    Adt0::C0
                }
            }
        }
    }
}

pub fn f4(v0: u64) -> Adt0 {
    let a290 = v0;
    let a291 = 0u64;
    if nat_eq(a290, a291) {
        Adt0::C1(24u64, 0u64, 0u64, 0u64, Some(15360u64))
    } else {
        let a292 = v0;
        let a293 = 1u64;
        if nat_eq(a292, a293) {
            Adt0::C1(31u64, 0u64, 0u64, 0u64, Some(1920u64))
        } else {
            let a294 = v0;
            let a295 = 2u64;
            if nat_eq(a294, a295) {
                Adt0::C1(25u64, 15360u64, 0u64, 0u64, None::<u64>)
            } else {
                let a296 = v0;
                let a297 = 3u64;
                if nat_eq(a296, a297) {
                    Adt0::C1(32u64, 1920u64, 0u64, 0u64, None::<u64>)
                } else {
                    let a298 = v0;
                    let a299 = 4u64;
                    if nat_eq(a298, a299) {
                        Adt0::C1(26u64, 0u64, 2u64, 0u64, None::<u64>)
                    } else {
                        let a300 = v0;
                        let a301 = 5u64;
                        if nat_eq(a300, a301) {
                            Adt0::C1(33u64, 0u64, 2u64, 0u64, None::<u64>)
                        } else {
                            let a302 = v0;
                            let a303 = 6u64;
                            if nat_eq(a302, a303) {
                                Adt0::C1(27u64, 0u64, 1u64, 0u64, None::<u64>)
                            } else {
                                let a304 = v0;
                                let a305 = 7u64;
                                if nat_eq(a304, a305) {
                                    Adt0::C1(34u64, 0u64, 0u64, 192u64, None::<u64>)
                                } else {
                                    let a306 = v0;
                                    let a307 = 8u64;
                                    if nat_eq(a306, a307) {
                                        Adt0::C1(30u64, 0u64, 0u64, 0u64, None::<u64>)
                                    } else {
                                        let a308 = v0;
                                        let a309 = 9u64;
                                        if nat_eq(a308, a309) {
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

pub fn f5(v0: u64) -> Adt0 {
    let a310 = v0;
    let a311 = 0u64;
    if nat_eq(a310, a311) {
        Adt0::C1(17u64, 0u64, 0u64, 0u64, Some(10368u64))
    } else {
        let a312 = v0;
        let a313 = 1u64;
        if nat_eq(a312, a313) {
            Adt0::C1(18u64, 10368u64, 0u64, 0u64, None::<u64>)
        } else {
            let a314 = v0;
            let a315 = 2u64;
            if nat_eq(a314, a315) {
                Adt0::C1(19u64, 0u64, 4u64, 0u64, None::<u64>)
            } else {
                let a316 = v0;
                let a317 = 3u64;
                if nat_eq(a316, a317) {
                    Adt0::C1(20u64, 0u64, 2u64, 0u64, None::<u64>)
                } else {
                    let a318 = v0;
                    let a319 = 4u64;
                    if nat_eq(a318, a319) {
                        Adt0::C1(23u64, 0u64, 0u64, 0u64, None::<u64>)
                    } else {
                        Adt0::C0
                    }
                }
            }
        }
    }
}

pub fn f6(v0: u64) -> Adt0 {
    let a320 = v0;
    let a321 = 0u64;
    if nat_eq(a320, a321) {
        Adt0::C1(11u64, 0u64, 0u64, 0u64, Some(160u64))
    } else {
        let a322 = v0;
        let a323 = 1u64;
        if nat_eq(a322, a323) {
            Adt0::C1(12u64, 160u64, 0u64, 0u64, None::<u64>)
        } else {
            let a324 = v0;
            let a325 = 2u64;
            if nat_eq(a324, a325) {
                Adt0::C1(13u64, 0u64, 4u64, 0u64, None::<u64>)
            } else {
                let a326 = v0;
                let a327 = 3u64;
                if nat_eq(a326, a327) {
                    Adt0::C1(14u64, 0u64, 0u64, 256u64, None::<u64>)
                } else {
                    let a328 = v0;
                    let a329 = 4u64;
                    if nat_eq(a328, a329) {
                        Adt0::C1(16u64, 0u64, 0u64, 0u64, None::<u64>)
                    } else {
                        Adt0::C0
                    }
                }
            }
        }
    }
}

pub fn f7(v0: u64) -> Adt0 {
    let a330 = v0;
    let a331 = 0u64;
    if nat_eq(a330, a331) {
        Adt0::C1(5u64, 0u64, 0u64, 0u64, Some(3456u64))
    } else {
        let a332 = v0;
        let a333 = 1u64;
        if nat_eq(a332, a333) {
            Adt0::C1(5u64, 0u64, 0u64, 5u64, None::<u64>)
        } else {
            let a334 = v0;
            let a335 = 2u64;
            if nat_eq(a334, a335) {
                Adt0::C1(5u64, 1u64, 0u64, 0u64, None::<u64>)
            } else {
                let a336 = v0;
                let a337 = 3u64;
                if nat_eq(a336, a337) {
                    Adt0::C1(6u64, 3456u64, 0u64, 0u64, None::<u64>)
                } else {
                    let a338 = v0;
                    let a339 = 4u64;
                    if nat_eq(a338, a339) {
                        Adt0::C1(6u64, 0u64, 0u64, 9u64, None::<u64>)
                    } else {
                        let a340 = v0;
                        let a341 = 5u64;
                        if nat_eq(a340, a341) {
                            Adt0::C1(7u64, 0u64, 2u64, 0u64, None::<u64>)
                        } else {
                            let a342 = v0;
                            let a343 = 6u64;
                            if nat_eq(a342, a343) {
                                Adt0::C1(7u64, 0u64, 0u64, 10u64, None::<u64>)
                            } else {
                                let a344 = v0;
                                let a345 = 7u64;
                                if nat_eq(a344, a345) {
                                    Adt0::C1(8u64, 0u64, 0u64, 256u64, None::<u64>)
                                } else {
                                    let a346 = v0;
                                    let a347 = 8u64;
                                    if nat_eq(a346, a347) {
                                        Adt0::C1(9u64, 215u64, 8u64, 0u64, None::<u64>)
                                    } else {
                                        let a348 = v0;
                                        let a349 = 9u64;
                                        if nat_eq(a348, a349) {
                                            Adt0::C1(10u64, 0u64, 0u64, 0u64, None::<u64>)
                                        } else {
                                            let a350 = v0;
                                            let a351 = 10u64;
                                            if nat_eq(a350, a351) {
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

pub fn f8(v0: u64) -> Adt0 {
    let a352 = v0;
    let a353 = 0u64;
    if nat_eq(a352, a353) {
        Adt0::C1(1u64, 120u64, 0u64, 0u64, None::<u64>)
    } else {
        let a354 = v0;
        let a355 = 1u64;
        if nat_eq(a354, a355) {
            Adt0::C1(1u64, 0u64, 0u64, 24u64, None::<u64>)
        } else {
            let a356 = v0;
            let a357 = 2u64;
            if nat_eq(a356, a357) {
                Adt0::C1(2u64, 0u64, 2u64, 0u64, None::<u64>)
            } else {
                let a358 = v0;
                let a359 = 3u64;
                if nat_eq(a358, a359) {
                    Adt0::C1(3u64, 96u64, 1u64, 0u64, None::<u64>)
                } else {
                    let a360 = v0;
                    let a361 = 4u64;
                    if nat_eq(a360, a361) {
                        Adt0::C1(4u64, 0u64, 0u64, 0u64, None::<u64>)
                    } else {
                        Adt0::C0
                    }
                }
            }
        }
    }
}

pub fn run(p0: u64, p1: u64) -> R<Adt0> {
    f0(p0, p1)
}
