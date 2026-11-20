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
    C1,
    C2,
    C3(u64, u64, u64),
}

pub fn f0(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt0> {
    if f1(v3) {
        f2(v0, v1, v2, {
            let v4: u64 = v3;
            let a381 = {
                let a189 = {
                    let a93 = {
                        let a45 = {
                            let a21 = {
                                let a9 = {
                                    let a3 = {
                                        let a1 = v4;
                                        let a2 = 9223372036854775808u64;
                                        fixed_u64::bit_and(a1, a2)
                                    };
                                    let a4 = 9223372036854775808u64;
                                    if equal(a3, a4) {
                                        9223372036854775808u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a10 = {
                                    let a7 = {
                                        let a5 = v4;
                                        let a6 = 4611686018427387904u64;
                                        fixed_u64::bit_and(a5, a6)
                                    };
                                    let a8 = 4611686018427387904u64;
                                    if equal(a7, a8) {
                                        4611686018427387904u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a9, a10)?
                            };
                            let a22 = {
                                let a19 = {
                                    let a13 = {
                                        let a11 = v4;
                                        let a12 = 2305843009213693952u64;
                                        fixed_u64::bit_and(a11, a12)
                                    };
                                    let a14 = 2305843009213693952u64;
                                    if equal(a13, a14) {
                                        2305843009213693952u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a20 = {
                                    let a17 = {
                                        let a15 = v4;
                                        let a16 = 1152921504606846976u64;
                                        fixed_u64::bit_and(a15, a16)
                                    };
                                    let a18 = 1152921504606846976u64;
                                    if equal(a17, a18) {
                                        1152921504606846976u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a19, a20)?
                            };
                            nat_add(a21, a22)?
                        };
                        let a46 = {
                            let a43 = {
                                let a31 = {
                                    let a25 = {
                                        let a23 = v4;
                                        let a24 = 576460752303423488u64;
                                        fixed_u64::bit_and(a23, a24)
                                    };
                                    let a26 = 576460752303423488u64;
                                    if equal(a25, a26) {
                                        576460752303423488u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a32 = {
                                    let a29 = {
                                        let a27 = v4;
                                        let a28 = 288230376151711744u64;
                                        fixed_u64::bit_and(a27, a28)
                                    };
                                    let a30 = 288230376151711744u64;
                                    if equal(a29, a30) {
                                        288230376151711744u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a31, a32)?
                            };
                            let a44 = {
                                let a41 = {
                                    let a35 = {
                                        let a33 = v4;
                                        let a34 = 144115188075855872u64;
                                        fixed_u64::bit_and(a33, a34)
                                    };
                                    let a36 = 144115188075855872u64;
                                    if equal(a35, a36) {
                                        144115188075855872u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a42 = {
                                    let a39 = {
                                        let a37 = v4;
                                        let a38 = 72057594037927936u64;
                                        fixed_u64::bit_and(a37, a38)
                                    };
                                    let a40 = 72057594037927936u64;
                                    if equal(a39, a40) {
                                        72057594037927936u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a41, a42)?
                            };
                            nat_add(a43, a44)?
                        };
                        nat_add(a45, a46)?
                    };
                    let a94 = {
                        let a91 = {
                            let a67 = {
                                let a55 = {
                                    let a49 = {
                                        let a47 = v4;
                                        let a48 = 36028797018963968u64;
                                        fixed_u64::bit_and(a47, a48)
                                    };
                                    let a50 = 36028797018963968u64;
                                    if equal(a49, a50) {
                                        36028797018963968u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a56 = {
                                    let a53 = {
                                        let a51 = v4;
                                        let a52 = 18014398509481984u64;
                                        fixed_u64::bit_and(a51, a52)
                                    };
                                    let a54 = 18014398509481984u64;
                                    if equal(a53, a54) {
                                        18014398509481984u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a55, a56)?
                            };
                            let a68 = {
                                let a65 = {
                                    let a59 = {
                                        let a57 = v4;
                                        let a58 = 9007199254740992u64;
                                        fixed_u64::bit_and(a57, a58)
                                    };
                                    let a60 = 9007199254740992u64;
                                    if equal(a59, a60) {
                                        9007199254740992u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a66 = {
                                    let a63 = {
                                        let a61 = v4;
                                        let a62 = 4503599627370496u64;
                                        fixed_u64::bit_and(a61, a62)
                                    };
                                    let a64 = 4503599627370496u64;
                                    if equal(a63, a64) {
                                        4503599627370496u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a65, a66)?
                            };
                            nat_add(a67, a68)?
                        };
                        let a92 = {
                            let a89 = {
                                let a77 = {
                                    let a71 = {
                                        let a69 = v4;
                                        let a70 = 2251799813685248u64;
                                        fixed_u64::bit_and(a69, a70)
                                    };
                                    let a72 = 2251799813685248u64;
                                    if equal(a71, a72) {
                                        2251799813685248u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a78 = {
                                    let a75 = {
                                        let a73 = v4;
                                        let a74 = 1125899906842624u64;
                                        fixed_u64::bit_and(a73, a74)
                                    };
                                    let a76 = 1125899906842624u64;
                                    if equal(a75, a76) {
                                        1125899906842624u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a77, a78)?
                            };
                            let a90 = {
                                let a87 = {
                                    let a81 = {
                                        let a79 = v4;
                                        let a80 = 562949953421312u64;
                                        fixed_u64::bit_and(a79, a80)
                                    };
                                    let a82 = 562949953421312u64;
                                    if equal(a81, a82) {
                                        562949953421312u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a88 = {
                                    let a85 = {
                                        let a83 = v4;
                                        let a84 = 281474976710656u64;
                                        fixed_u64::bit_and(a83, a84)
                                    };
                                    let a86 = 281474976710656u64;
                                    if equal(a85, a86) {
                                        281474976710656u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a87, a88)?
                            };
                            nat_add(a89, a90)?
                        };
                        nat_add(a91, a92)?
                    };
                    nat_add(a93, a94)?
                };
                let a190 = {
                    let a187 = {
                        let a139 = {
                            let a115 = {
                                let a103 = {
                                    let a97 = {
                                        let a95 = v4;
                                        let a96 = 140737488355328u64;
                                        fixed_u64::bit_and(a95, a96)
                                    };
                                    let a98 = 140737488355328u64;
                                    if equal(a97, a98) {
                                        140737488355328u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a104 = {
                                    let a101 = {
                                        let a99 = v4;
                                        let a100 = 70368744177664u64;
                                        fixed_u64::bit_and(a99, a100)
                                    };
                                    let a102 = 70368744177664u64;
                                    if equal(a101, a102) {
                                        70368744177664u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a103, a104)?
                            };
                            let a116 = {
                                let a113 = {
                                    let a107 = {
                                        let a105 = v4;
                                        let a106 = 35184372088832u64;
                                        fixed_u64::bit_and(a105, a106)
                                    };
                                    let a108 = 35184372088832u64;
                                    if equal(a107, a108) {
                                        35184372088832u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a114 = {
                                    let a111 = {
                                        let a109 = v4;
                                        let a110 = 17592186044416u64;
                                        fixed_u64::bit_and(a109, a110)
                                    };
                                    let a112 = 17592186044416u64;
                                    if equal(a111, a112) {
                                        17592186044416u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a113, a114)?
                            };
                            nat_add(a115, a116)?
                        };
                        let a140 = {
                            let a137 = {
                                let a125 = {
                                    let a119 = {
                                        let a117 = v4;
                                        let a118 = 8796093022208u64;
                                        fixed_u64::bit_and(a117, a118)
                                    };
                                    let a120 = 8796093022208u64;
                                    if equal(a119, a120) {
                                        8796093022208u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a126 = {
                                    let a123 = {
                                        let a121 = v4;
                                        let a122 = 4398046511104u64;
                                        fixed_u64::bit_and(a121, a122)
                                    };
                                    let a124 = 4398046511104u64;
                                    if equal(a123, a124) {
                                        4398046511104u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a125, a126)?
                            };
                            let a138 = {
                                let a135 = {
                                    let a129 = {
                                        let a127 = v4;
                                        let a128 = 2199023255552u64;
                                        fixed_u64::bit_and(a127, a128)
                                    };
                                    let a130 = 2199023255552u64;
                                    if equal(a129, a130) {
                                        2199023255552u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a136 = {
                                    let a133 = {
                                        let a131 = v4;
                                        let a132 = 1099511627776u64;
                                        fixed_u64::bit_and(a131, a132)
                                    };
                                    let a134 = 1099511627776u64;
                                    if equal(a133, a134) {
                                        1099511627776u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a135, a136)?
                            };
                            nat_add(a137, a138)?
                        };
                        nat_add(a139, a140)?
                    };
                    let a188 = {
                        let a185 = {
                            let a161 = {
                                let a149 = {
                                    let a143 = {
                                        let a141 = v4;
                                        let a142 = 549755813888u64;
                                        fixed_u64::bit_and(a141, a142)
                                    };
                                    let a144 = 549755813888u64;
                                    if equal(a143, a144) {
                                        549755813888u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a150 = {
                                    let a147 = {
                                        let a145 = v4;
                                        let a146 = 274877906944u64;
                                        fixed_u64::bit_and(a145, a146)
                                    };
                                    let a148 = 274877906944u64;
                                    if equal(a147, a148) {
                                        274877906944u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a149, a150)?
                            };
                            let a162 = {
                                let a159 = {
                                    let a153 = {
                                        let a151 = v4;
                                        let a152 = 137438953472u64;
                                        fixed_u64::bit_and(a151, a152)
                                    };
                                    let a154 = 137438953472u64;
                                    if equal(a153, a154) {
                                        137438953472u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a160 = {
                                    let a157 = {
                                        let a155 = v4;
                                        let a156 = 68719476736u64;
                                        fixed_u64::bit_and(a155, a156)
                                    };
                                    let a158 = 68719476736u64;
                                    if equal(a157, a158) {
                                        68719476736u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a159, a160)?
                            };
                            nat_add(a161, a162)?
                        };
                        let a186 = {
                            let a183 = {
                                let a171 = {
                                    let a165 = {
                                        let a163 = v4;
                                        let a164 = 34359738368u64;
                                        fixed_u64::bit_and(a163, a164)
                                    };
                                    let a166 = 34359738368u64;
                                    if equal(a165, a166) {
                                        34359738368u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a172 = {
                                    let a169 = {
                                        let a167 = v4;
                                        let a168 = 17179869184u64;
                                        fixed_u64::bit_and(a167, a168)
                                    };
                                    let a170 = 17179869184u64;
                                    if equal(a169, a170) {
                                        17179869184u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a171, a172)?
                            };
                            let a184 = {
                                let a181 = {
                                    let a175 = {
                                        let a173 = v4;
                                        let a174 = 8589934592u64;
                                        fixed_u64::bit_and(a173, a174)
                                    };
                                    let a176 = 8589934592u64;
                                    if equal(a175, a176) {
                                        8589934592u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a182 = {
                                    let a179 = {
                                        let a177 = v4;
                                        let a178 = 4294967296u64;
                                        fixed_u64::bit_and(a177, a178)
                                    };
                                    let a180 = 4294967296u64;
                                    if equal(a179, a180) {
                                        4294967296u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a181, a182)?
                            };
                            nat_add(a183, a184)?
                        };
                        nat_add(a185, a186)?
                    };
                    nat_add(a187, a188)?
                };
                nat_add(a189, a190)?
            };
            let a382 = {
                let a379 = {
                    let a283 = {
                        let a235 = {
                            let a211 = {
                                let a199 = {
                                    let a193 = {
                                        let a191 = v4;
                                        let a192 = 2147483648u64;
                                        fixed_u64::bit_and(a191, a192)
                                    };
                                    let a194 = 2147483648u64;
                                    if equal(a193, a194) {
                                        2147483648u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a200 = {
                                    let a197 = {
                                        let a195 = v4;
                                        let a196 = 1073741824u64;
                                        fixed_u64::bit_and(a195, a196)
                                    };
                                    let a198 = 1073741824u64;
                                    if equal(a197, a198) {
                                        1073741824u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a199, a200)?
                            };
                            let a212 = {
                                let a209 = {
                                    let a203 = {
                                        let a201 = v4;
                                        let a202 = 536870912u64;
                                        fixed_u64::bit_and(a201, a202)
                                    };
                                    let a204 = 536870912u64;
                                    if equal(a203, a204) {
                                        536870912u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a210 = {
                                    let a207 = {
                                        let a205 = v4;
                                        let a206 = 268435456u64;
                                        fixed_u64::bit_and(a205, a206)
                                    };
                                    let a208 = 268435456u64;
                                    if equal(a207, a208) {
                                        268435456u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a209, a210)?
                            };
                            nat_add(a211, a212)?
                        };
                        let a236 = {
                            let a233 = {
                                let a221 = {
                                    let a215 = {
                                        let a213 = v4;
                                        let a214 = 134217728u64;
                                        fixed_u64::bit_and(a213, a214)
                                    };
                                    let a216 = 134217728u64;
                                    if equal(a215, a216) {
                                        134217728u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a222 = {
                                    let a219 = {
                                        let a217 = v4;
                                        let a218 = 67108864u64;
                                        fixed_u64::bit_and(a217, a218)
                                    };
                                    let a220 = 67108864u64;
                                    if equal(a219, a220) {
                                        67108864u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a221, a222)?
                            };
                            let a234 = {
                                let a231 = {
                                    let a225 = {
                                        let a223 = v4;
                                        let a224 = 33554432u64;
                                        fixed_u64::bit_and(a223, a224)
                                    };
                                    let a226 = 33554432u64;
                                    if equal(a225, a226) {
                                        33554432u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a232 = {
                                    let a229 = {
                                        let a227 = v4;
                                        let a228 = 16777216u64;
                                        fixed_u64::bit_and(a227, a228)
                                    };
                                    let a230 = 16777216u64;
                                    if equal(a229, a230) {
                                        16777216u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a231, a232)?
                            };
                            nat_add(a233, a234)?
                        };
                        nat_add(a235, a236)?
                    };
                    let a284 = {
                        let a281 = {
                            let a257 = {
                                let a245 = {
                                    let a239 = {
                                        let a237 = v4;
                                        let a238 = 8388608u64;
                                        fixed_u64::bit_and(a237, a238)
                                    };
                                    let a240 = 8388608u64;
                                    if equal(a239, a240) {
                                        8388608u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a246 = {
                                    let a243 = {
                                        let a241 = v4;
                                        let a242 = 4194304u64;
                                        fixed_u64::bit_and(a241, a242)
                                    };
                                    let a244 = 4194304u64;
                                    if equal(a243, a244) {
                                        4194304u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a245, a246)?
                            };
                            let a258 = {
                                let a255 = {
                                    let a249 = {
                                        let a247 = v4;
                                        let a248 = 2097152u64;
                                        fixed_u64::bit_and(a247, a248)
                                    };
                                    let a250 = 2097152u64;
                                    if equal(a249, a250) {
                                        2097152u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a256 = {
                                    let a253 = {
                                        let a251 = v4;
                                        let a252 = 1048576u64;
                                        fixed_u64::bit_and(a251, a252)
                                    };
                                    let a254 = 1048576u64;
                                    if equal(a253, a254) {
                                        1048576u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a255, a256)?
                            };
                            nat_add(a257, a258)?
                        };
                        let a282 = {
                            let a279 = {
                                let a267 = {
                                    let a261 = {
                                        let a259 = v4;
                                        let a260 = 524288u64;
                                        fixed_u64::bit_and(a259, a260)
                                    };
                                    let a262 = 524288u64;
                                    if equal(a261, a262) {
                                        524288u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a268 = {
                                    let a265 = {
                                        let a263 = v4;
                                        let a264 = 262144u64;
                                        fixed_u64::bit_and(a263, a264)
                                    };
                                    let a266 = 262144u64;
                                    if equal(a265, a266) {
                                        262144u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a267, a268)?
                            };
                            let a280 = {
                                let a277 = {
                                    let a271 = {
                                        let a269 = v4;
                                        let a270 = 131072u64;
                                        fixed_u64::bit_and(a269, a270)
                                    };
                                    let a272 = 131072u64;
                                    if equal(a271, a272) {
                                        131072u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a278 = {
                                    let a275 = {
                                        let a273 = v4;
                                        let a274 = 65536u64;
                                        fixed_u64::bit_and(a273, a274)
                                    };
                                    let a276 = 65536u64;
                                    if equal(a275, a276) {
                                        65536u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a277, a278)?
                            };
                            nat_add(a279, a280)?
                        };
                        nat_add(a281, a282)?
                    };
                    nat_add(a283, a284)?
                };
                let a380 = {
                    let a377 = {
                        let a329 = {
                            let a305 = {
                                let a293 = {
                                    let a287 = {
                                        let a285 = v4;
                                        let a286 = 32768u64;
                                        fixed_u64::bit_and(a285, a286)
                                    };
                                    let a288 = 32768u64;
                                    if equal(a287, a288) {
                                        32768u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a294 = {
                                    let a291 = {
                                        let a289 = v4;
                                        let a290 = 16384u64;
                                        fixed_u64::bit_and(a289, a290)
                                    };
                                    let a292 = 16384u64;
                                    if equal(a291, a292) {
                                        16384u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a293, a294)?
                            };
                            let a306 = {
                                let a303 = {
                                    let a297 = {
                                        let a295 = v4;
                                        let a296 = 8192u64;
                                        fixed_u64::bit_and(a295, a296)
                                    };
                                    let a298 = 8192u64;
                                    if equal(a297, a298) {
                                        8192u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a304 = {
                                    let a301 = {
                                        let a299 = v4;
                                        let a300 = 4096u64;
                                        fixed_u64::bit_and(a299, a300)
                                    };
                                    let a302 = 4096u64;
                                    if equal(a301, a302) {
                                        4096u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a303, a304)?
                            };
                            nat_add(a305, a306)?
                        };
                        let a330 = {
                            let a327 = {
                                let a315 = {
                                    let a309 = {
                                        let a307 = v4;
                                        let a308 = 2048u64;
                                        fixed_u64::bit_and(a307, a308)
                                    };
                                    let a310 = 2048u64;
                                    if equal(a309, a310) {
                                        2048u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a316 = {
                                    let a313 = {
                                        let a311 = v4;
                                        let a312 = 1024u64;
                                        fixed_u64::bit_and(a311, a312)
                                    };
                                    let a314 = 1024u64;
                                    if equal(a313, a314) {
                                        1024u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a315, a316)?
                            };
                            let a328 = {
                                let a325 = {
                                    let a319 = {
                                        let a317 = v4;
                                        let a318 = 512u64;
                                        fixed_u64::bit_and(a317, a318)
                                    };
                                    let a320 = 512u64;
                                    if equal(a319, a320) {
                                        512u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a326 = {
                                    let a323 = {
                                        let a321 = v4;
                                        let a322 = 256u64;
                                        fixed_u64::bit_and(a321, a322)
                                    };
                                    let a324 = 256u64;
                                    if equal(a323, a324) {
                                        256u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a325, a326)?
                            };
                            nat_add(a327, a328)?
                        };
                        nat_add(a329, a330)?
                    };
                    let a378 = {
                        let a375 = {
                            let a351 = {
                                let a339 = {
                                    let a333 = {
                                        let a331 = v4;
                                        let a332 = 128u64;
                                        fixed_u64::bit_and(a331, a332)
                                    };
                                    let a334 = 128u64;
                                    if equal(a333, a334) {
                                        128u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a340 = {
                                    let a337 = {
                                        let a335 = v4;
                                        let a336 = 64u64;
                                        fixed_u64::bit_and(a335, a336)
                                    };
                                    let a338 = 64u64;
                                    if equal(a337, a338) {
                                        64u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a339, a340)?
                            };
                            let a352 = {
                                let a349 = {
                                    let a343 = {
                                        let a341 = v4;
                                        let a342 = 32u64;
                                        fixed_u64::bit_and(a341, a342)
                                    };
                                    let a344 = 32u64;
                                    if equal(a343, a344) {
                                        32u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a350 = {
                                    let a347 = {
                                        let a345 = v4;
                                        let a346 = 16u64;
                                        fixed_u64::bit_and(a345, a346)
                                    };
                                    let a348 = 16u64;
                                    if equal(a347, a348) {
                                        16u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a349, a350)?
                            };
                            nat_add(a351, a352)?
                        };
                        let a376 = {
                            let a373 = {
                                let a361 = {
                                    let a355 = {
                                        let a353 = v4;
                                        let a354 = 8u64;
                                        fixed_u64::bit_and(a353, a354)
                                    };
                                    let a356 = 8u64;
                                    if equal(a355, a356) {
                                        8u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a362 = {
                                    let a359 = {
                                        let a357 = v4;
                                        let a358 = 4u64;
                                        fixed_u64::bit_and(a357, a358)
                                    };
                                    let a360 = 4u64;
                                    if equal(a359, a360) {
                                        4u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a361, a362)?
                            };
                            let a374 = {
                                let a371 = {
                                    let a365 = {
                                        let a363 = v4;
                                        let a364 = 2u64;
                                        fixed_u64::bit_and(a363, a364)
                                    };
                                    let a366 = 2u64;
                                    if equal(a365, a366) {
                                        2u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a372 = {
                                    let a369 = {
                                        let a367 = v4;
                                        let a368 = 1u64;
                                        fixed_u64::bit_and(a367, a368)
                                    };
                                    let a370 = 1u64;
                                    if equal(a369, a370) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a371, a372)?
                            };
                            nat_add(a373, a374)?
                        };
                        nat_add(a375, a376)?
                    };
                    nat_add(a377, a378)?
                };
                nat_add(a379, a380)?
            };
            nat_add(a381, a382)?
        })
    } else {
        Ok(Adt0::C0)
    }
}

pub fn f1(v0: u64) -> bool {
    let m385 = {
        let a383 = v0;
        let a384 = 1u64;
        fixed_u64::checked_sub(a383, a384)
    };
    match m385 {
        None => {
            false
        }
        Some(v1) => {
            let a388 = {
                let a386 = v0;
                let a387 = v1;
                fixed_u64::bit_and(a386, a387)
            };
            let a389 = 0u64;
            equal(a388, a389)
        }
    }
}

pub fn f2(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt0> {
    let a393 = {
        let a390 = v1;
        let a391 = v3;
        let a392 = 0u64;
        nat_rem(a390, a391, a392)
    };
    let a394 = 0u64;
    if nat_eq(a393, a394) {
        f3(v0, v1, {
            let a403 = v2;
            let a404 = {
                let a400 = {
                    let a398 = v3;
                    let a399 = {
                        let a395 = v2;
                        let a396 = v3;
                        let a397 = 0u64;
                        nat_rem(a395, a396, a397)
                    };
                    nat_sub(a398, a399)
                };
                let a401 = v3;
                let a402 = 0u64;
                nat_rem(a400, a401, a402)
            };
            nat_add(a403, a404)?
        })
    } else {
        Ok(Adt0::C1)
    }
}

pub fn f3(v0: u64, v1: u64, v2: u64) -> R<Adt0> {
    let a405 = v1;
    let a406 = v0;
    if if nat_le(a405, a406) {
        let a409 = v2;
        let a410 = {
            let a407 = v0;
            let a408 = v1;
            nat_sub(a407, a408)
        };
        nat_le(a409, a410)
    } else {
        false
    } {
        Ok(Adt0::C3(v1, v2, {
            let a411 = v1;
            let a412 = v2;
            nat_add(a411, a412)?
        }))
    } else {
        Ok(Adt0::C2)
    }
}

pub fn run(p0: u64, p1: u64, p2: u64, p3: u64) -> R<Adt0> {
    f0(p0, p1, p2, p3)
}
