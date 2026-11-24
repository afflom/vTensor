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
    C0(u64),
    C1(u64),
    C2(u64, u64),
    C3(u64),
    C4(u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, bool, bool, bool, bool, bool),
}

pub fn f0(v0: Adt0) -> R<u64> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1) => {
            let a2 = v1;
            let a3 = 0u64;
            if nat_eq(a2, a3) {
                Ok(0u64)
            } else {
                let a8 = v1;
                let a9 = {
                    let a6 = {
                        let m5 = f1();
                        match m5 {
                            Adt1::C0(_, h4, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h4
                            }
                        }
                    };
                    let a7 = 1u64;
                    nat_sub(a6, a7)
                };
                nat_add(a8, a9)
            }
        }
        Adt0::C1(v2) => {
            let a21 = {
                let a17 = {
                    let m11 = f1();
                    match m11 {
                        Adt1::C0(_, h10, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h10
                        }
                    }
                };
                let a18 = {
                    let a14 = v2;
                    let a15 = {
                        let m13 = f1();
                        match m13 {
                            Adt1::C0(_, h12, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h12
                            }
                        }
                    };
                    let a16 = 0u64;
                    nat_rem(a14, a15, a16)
                };
                nat_sub(a17, a18)
            };
            let a22 = {
                let m20 = f1();
                match m20 {
                    Adt1::C0(_, h19, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h19
                    }
                }
            };
            let a23 = 0u64;
            Ok(nat_rem(a21, a22, a23))
        }
        Adt0::C2(v3, v4) => {
            let a24 = v3;
            let a25 = 0u64;
            if if nat_eq(a24, a25) {
                true
            } else {
                let a26 = v4;
                let a27 = 0u64;
                nat_eq(a26, a27)
            } {
                Ok(0u64)
            } else {
                let a33 = {
                    let a30 = {
                        let a28 = v4;
                        let a29 = 1u64;
                        nat_sub(a28, a29)
                    };
                    let a31 = v3;
                    let a32 = 0u64;
                    nat_quot(a30, a31, a32)
                };
                let a34 = 1u64;
                nat_add(a33, a34)
            }
        }
        Adt0::C3(v12) => {
            let a35 = v12;
            let a36 = 2u64;
            if nat_eq(a35, a36) {
                Ok(40u64)
            } else {
                let a37 = v12;
                let a38 = 3u64;
                if nat_eq(a37, a38) {
                    Ok(80u64)
                } else {
                    let a39 = v12;
                    let a40 = 4u64;
                    if nat_eq(a39, a40) {
                        Ok(90u64)
                    } else {
                        Ok(0u64)
                    }
                }
            }
        }
        Adt0::C4(v5, v6, v7, v8, v9, v10, v11) => {
            Ok(f2(v5, v6, v7, v8, v9, v10, v11))
        }
    }
}

pub fn f1() -> Adt1 {
    Adt1::C0(3u64, 64u64, 1u64, 0u64, 0u64, 32210688909071478u64, 2319107987578909264u64, 2338615471088231766u64, 125823003551060u64, 6229163776170873974u64, 2392383844493062757u64, false, false, true, false, true)
}

pub fn f2(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> u64 {
    let a41 = v2;
    let a42 = 0u64;
    if nat_eq(a41, a42) {
        1u64
    } else {
        let a43 = 90u64;
        let a44 = v0;
        if if nat_le(a43, a44) {
            let a45 = v0;
            let a46 = 93u64;
            nat_le(a45, a46)
        } else {
            false
        } {
            let a47 = v5;
            let a48 = 4294967295u64;
            if nat_eq(a47, a48) {
                v6
            } else {
                let a49 = v5;
                let a50 = v6;
                if nat_lt(a49, a50) {
                    v5
                } else {
                    v6
                }
            }
        } else {
            let a51 = v0;
            let a52 = 0u64;
            if if if if nat_eq(a51, a52) {
                true
            } else {
                let a53 = v0;
                let a54 = 5u64;
                if nat_eq(a53, a54) {
                    true
                } else {
                    let a55 = 8u64;
                    let a56 = v0;
                    if nat_le(a55, a56) {
                        let a57 = v0;
                        let a58 = 14u64;
                        nat_le(a57, a58)
                    } else {
                        false
                    }
                }
            } {
                true
            } else {
                let a59 = 16u64;
                let a60 = v0;
                if if if nat_le(a59, a60) {
                    let a61 = v0;
                    let a62 = 17u64;
                    nat_le(a61, a62)
                } else {
                    false
                } {
                    true
                } else {
                    let a63 = 19u64;
                    let a64 = v0;
                    if nat_le(a63, a64) {
                        let a65 = v0;
                        let a66 = 20u64;
                        nat_le(a65, a66)
                    } else {
                        false
                    }
                } {
                    true
                } else {
                    let a67 = 31u64;
                    let a68 = v0;
                    if if nat_le(a67, a68) {
                        let a69 = v0;
                        let a70 = 32u64;
                        nat_le(a69, a70)
                    } else {
                        false
                    } {
                        true
                    } else {
                        let a71 = 35u64;
                        let a72 = v0;
                        if nat_le(a71, a72) {
                            let a73 = v0;
                            let a74 = 38u64;
                            nat_le(a73, a74)
                        } else {
                            false
                        }
                    }
                }
            } {
                true
            } else {
                let a75 = v0;
                let a76 = 40u64;
                if if nat_eq(a75, a76) {
                    true
                } else {
                    let a77 = v0;
                    let a78 = 42u64;
                    if nat_eq(a77, a78) {
                        true
                    } else {
                        let a79 = v0;
                        let a80 = 47u64;
                        nat_eq(a79, a80)
                    }
                } {
                    true
                } else {
                    let a81 = 57u64;
                    let a82 = v0;
                    if if if nat_le(a81, a82) {
                        let a83 = v0;
                        let a84 = 59u64;
                        nat_le(a83, a84)
                    } else {
                        false
                    } {
                        true
                    } else {
                        let a85 = v0;
                        let a86 = 68u64;
                        nat_eq(a85, a86)
                    } {
                        true
                    } else {
                        let a87 = 75u64;
                        let a88 = v0;
                        if if nat_le(a87, a88) {
                            let a89 = v0;
                            let a90 = 77u64;
                            nat_le(a89, a90)
                        } else {
                            false
                        } {
                            true
                        } else {
                            let a91 = v0;
                            let a92 = 99u64;
                            nat_eq(a91, a92)
                        }
                    }
                }
            } {
                1u64
            } else {
                let a93 = v0;
                let a94 = 89u64;
                if if nat_eq(a93, a94) {
                    let a95 = 0u64;
                    let a96 = v1;
                    if if nat_le(a95, a96) {
                        let a97 = v1;
                        let a98 = 7u64;
                        nat_le(a97, a98)
                    } else {
                        false
                    } {
                        true
                    } else {
                        let a99 = 11u64;
                        let a100 = v1;
                        if if nat_le(a99, a100) {
                            let a101 = v1;
                            let a102 = 15u64;
                            nat_le(a101, a102)
                        } else {
                            false
                        } {
                            true
                        } else {
                            let a103 = 18u64;
                            let a104 = v1;
                            if nat_le(a103, a104) {
                                let a105 = v1;
                                let a106 = 21u64;
                                nat_le(a105, a106)
                            } else {
                                false
                            }
                        }
                    }
                } else {
                    false
                } {
                    1u64
                } else {
                    let a107 = v0;
                    let a108 = 39u64;
                    if if nat_eq(a107, a108) {
                        let a109 = v2;
                        let a110 = 262144u64;
                        nat_lt(a109, a110)
                    } else {
                        false
                    } {
                        1u64
                    } else {
                        let a111 = v0;
                        let a112 = 43u64;
                        if nat_eq(a111, a112) {
                            let a113 = v3;
                            let a114 = v6;
                            if nat_lt(a113, a114) {
                                v3
                            } else {
                                v6
                            }
                        } else {
                            let a115 = 79u64;
                            let a116 = v0;
                            if if nat_le(a115, a116) {
                                let a117 = v0;
                                let a118 = 81u64;
                                nat_le(a117, a118)
                            } else {
                                false
                            } {
                                let a119 = v4;
                                let a120 = v6;
                                if nat_lt(a119, a120) {
                                    v4
                                } else {
                                    v6
                                }
                            } else {
                                v6
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn run(p0: Adt0) -> R<u64> {
    f0(p0)
}
