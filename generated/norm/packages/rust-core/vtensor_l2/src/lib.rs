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
    C1(u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64),
}

pub fn f0(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> R<Adt0> {
    f1(v0, Adt1::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt1::C0(v1, v2, v3, v4, v9, v10, v11, v12), v13, v14)
}

pub fn f1(v0: u64, v1: Adt1, v2: Adt1, v3: u64, v4: u64) -> R<Adt0> {
    let v5: u64 = f2(match v1.clone() {
        Adt1::C0(h1, _, _, _, _, _, _, _) => {
            h1
        }
    })?;
    let v6: u64 = {
        let a7 = match v1.clone() {
            Adt1::C0(_, h2, _, _, _, _, _, _) => {
                h2
            }
        };
        let a8 = {
            let a5 = match v1.clone() {
                Adt1::C0(_, _, h3, _, _, _, _, _) => {
                    h3
                }
            };
            let a6 = match v1.clone() {
                Adt1::C0(_, _, _, h4, _, _, _, _) => {
                    h4
                }
            };
            nat_mul(a5, a6)?
        };
        nat_mul(a7, a8)?
    };
    let a10 = v0;
    let a11 = f3(match v1.clone() {
        Adt1::C0(h9, _, _, _, _, _, _, _) => {
            h9
        }
    }, v6)?;
    if nat_lt(a10, a11) {
        f4({
            let a12 = v0;
            let a13 = v5;
            let a14 = 0u64;
            nat_rem(a12, a13, a14)
        }, {
            let a15 = v0;
            let a16 = v5;
            let a17 = 0u64;
            nat_quot(a15, a16, a17)
        }, v1, v2, v3, v4)
    } else {
        Ok(Adt0::C0)
    }
}

pub fn f2(v0: u64) -> R<u64> {
    let a20 = 5u64;
    let a21 = {
        let a18 = 4u64;
        let a19 = v0;
        nat_mul(a18, a19)?
    };
    nat_add(a20, a21)
}

pub fn f3(v0: u64, v1: u64) -> R<u64> {
    let a22 = v0;
    let a23 = 0u64;
    if nat_eq(a22, a23) {
        Ok(0u64)
    } else {
        let a24 = v1;
        let a25 = f2(v0)?;
        nat_mul(a24, a25)
    }
}

pub fn f4(v0: u64, v1: u64, v2: Adt1, v3: Adt1, v4: u64, v5: u64) -> R<Adt0> {
    let v6: u64 = match v2.clone() {
        Adt1::C0(h26, _, _, _, _, _, _, _) => {
            h26
        }
    };
    let v7: u64 = {
        let a29 = 1u64;
        let a30 = {
            let a27 = 3u64;
            let a28 = v6;
            nat_mul(a27, a28)?
        };
        nat_add(a29, a30)?
    };
    let a31 = v0;
    let a32 = 0u64;
    if nat_eq(a31, a32) {
        Ok(f5(0u64))
    } else {
        let a33 = v0;
        let a34 = v7;
        if nat_lt(a33, a34) {
            let v8: u64 = {
                let a35 = v0;
                let a36 = 1u64;
                nat_sub(a35, a36)
            };
            let v9: u64 = {
                let a42 = {
                    let a37 = v1;
                    let a38 = v6;
                    nat_mul(a37, a38)?
                };
                let a43 = {
                    let a39 = v8;
                    let a40 = 3u64;
                    let a41 = 0u64;
                    nat_quot(a39, a40, a41)
                };
                nat_add(a42, a43)?
            };
            let v10: u64 = f6(v9, v2)?;
            let v11: u64 = {
                let a44 = v8;
                let a45 = 3u64;
                let a46 = 0u64;
                nat_rem(a44, a45, a46)
            };
            let a47 = v11;
            let a48 = 0u64;
            if nat_eq(a47, a48) {
                Ok(f7(10u64, 0u64, v10, 0u64, v10, 1u64, 1u64))
            } else {
                let a49 = v11;
                let a50 = 1u64;
                if nat_eq(a49, a50) {
                    Ok(f7(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f7(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            }
        } else {
            let a51 = v0;
            let a52 = v7;
            if nat_eq(a51, a52) {
                Ok(f7(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a55 = v0;
                let a56 = {
                    let a53 = v7;
                    let a54 = 1u64;
                    nat_add(a53, a54)?
                };
                if nat_eq(a55, a56) {
                    Ok(f7(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a59 = v0;
                    let a60 = {
                        let a57 = v7;
                        let a58 = 2u64;
                        nat_add(a57, a58)?
                    };
                    if nat_eq(a59, a60) {
                        Ok(f7(16u64, 1u64, 1u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        let a63 = v0;
                        let a64 = {
                            let a61 = v7;
                            let a62 = 3u64;
                            nat_add(a61, a62)?
                        };
                        if nat_eq(a63, a64) {
                            let a65 = v5;
                            let a66 = 1u64;
                            if if nat_eq(a65, a66) {
                                true
                            } else {
                                let a67 = v5;
                                let a68 = 2u64;
                                nat_eq(a67, a68)
                            } {
                                Ok(f7(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                            } else {
                                Ok(f7(11u64, 2u64, 1065353216u64, 2u64, v4, 1u64, 2u64))
                            }
                        } else {
                            let v12: u64 = {
                                let a75 = {
                                    let a69 = v1;
                                    let a70 = v6;
                                    nat_mul(a69, a70)?
                                };
                                let a76 = {
                                    let a73 = v0;
                                    let a74 = {
                                        let a71 = v7;
                                        let a72 = 4u64;
                                        nat_add(a71, a72)?
                                    };
                                    nat_sub(a73, a74)
                                };
                                nat_add(a75, a76)?
                            };
                            Ok(f7(10u64, 0u64, f6(v12, v2)?, 1u64, 2u64, 3u64, f6(v12, v3)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f5(v0: u64) -> Adt0 {
    f7(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f6(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a78 = v0;
        let a79 = match v1.clone() {
            Adt1::C0(h77, _, _, _, _, _, _, _) => {
                h77
            }
        };
        let a80 = 0u64;
        nat_quot(a78, a79, a80)
    };
    let v3: u64 = {
        let a82 = v2;
        let a83 = match v1.clone() {
            Adt1::C0(_, h81, _, _, _, _, _, _) => {
                h81
            }
        };
        let a84 = 0u64;
        nat_quot(a82, a83, a84)
    };
    f8({
        let a86 = v0;
        let a87 = match v1.clone() {
            Adt1::C0(h85, _, _, _, _, _, _, _) => {
                h85
            }
        };
        let a88 = 0u64;
        nat_rem(a86, a87, a88)
    }, {
        let a90 = v2;
        let a91 = match v1.clone() {
            Adt1::C0(_, h89, _, _, _, _, _, _) => {
                h89
            }
        };
        let a92 = 0u64;
        nat_rem(a90, a91, a92)
    }, {
        let a94 = v3;
        let a95 = match v1.clone() {
            Adt1::C0(_, _, h93, _, _, _, _, _) => {
                h93
            }
        };
        let a96 = 0u64;
        nat_rem(a94, a95, a96)
    }, {
        let a98 = v3;
        let a99 = match v1.clone() {
            Adt1::C0(_, _, h97, _, _, _, _, _) => {
                h97
            }
        };
        let a100 = 0u64;
        nat_quot(a98, a99, a100)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, h101, _, _, _) => {
            h101
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h102, _, _) => {
            h102
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h103, _) => {
            h103
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, h104) => {
            h104
        }
    })
}

pub fn f7(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> Adt0 {
    let a105 = v0;
    let a106 = 4u64;
    if if if nat_le(a105, a106) {
        true
    } else {
        let a107 = 9u64;
        let a108 = v0;
        if nat_le(a107, a108) {
            let a109 = v0;
            let a110 = 12u64;
            nat_le(a109, a110)
        } else {
            false
        }
    } {
        true
    } else {
        let a111 = v0;
        let a112 = 16u64;
        if if nat_eq(a111, a112) {
            true
        } else {
            let a113 = v0;
            let a114 = 19u64;
            nat_eq(a113, a114)
        } {
            true
        } else {
            let a115 = v0;
            let a116 = 21u64;
            if nat_eq(a115, a116) {
                true
            } else {
                let a117 = v0;
                let a118 = 22u64;
                nat_eq(a117, a118)
            }
        }
    } {
        Adt0::C1({
            let a119 = v0;
            let a120 = 4u64;
            if if nat_lt(a119, a120) {
                true
            } else {
                let a121 = 21u64;
                let a122 = v0;
                nat_le(a121, a122)
            } {
                0u64
            } else {
                let a123 = v0;
                let a124 = 16u64;
                if if nat_eq(a123, a124) {
                    true
                } else {
                    let a125 = v0;
                    let a126 = 19u64;
                    nat_eq(a125, a126)
                } {
                    2u64
                } else {
                    1u64
                }
            }
        }, {
            let a127 = v0;
            let a128 = 4u64;
            if nat_lt(a127, a128) {
                v0
            } else {
                let a129 = v0;
                let a130 = 4u64;
                if if nat_eq(a129, a130) {
                    true
                } else {
                    let a131 = v0;
                    let a132 = 16u64;
                    nat_eq(a131, a132)
                } {
                    0u64
                } else {
                    let a133 = v0;
                    let a134 = 19u64;
                    if nat_eq(a133, a134) {
                        3u64
                    } else {
                        let a135 = 21u64;
                        let a136 = v0;
                        if nat_le(a135, a136) {
                            let a137 = v0;
                            let a138 = 17u64;
                            nat_sub(a137, a138)
                        } else {
                            let a139 = v0;
                            let a140 = 8u64;
                            nat_sub(a139, a140)
                        }
                    }
                }
            }
        }, v1, v2, v3, v4, v5, v6)
    } else {
        Adt0::C0
    }
}

pub fn f8(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a153 = {
        let a149 = {
            let a145 = {
                let a141 = v0;
                let a142 = v4;
                nat_mul(a141, a142)?
            };
            let a146 = {
                let a143 = v1;
                let a144 = v5;
                nat_mul(a143, a144)?
            };
            nat_add(a145, a146)?
        };
        let a150 = {
            let a147 = v2;
            let a148 = v6;
            nat_mul(a147, a148)?
        };
        nat_add(a149, a150)?
    };
    let a154 = {
        let a151 = v3;
        let a152 = v7;
        nat_mul(a151, a152)?
    };
    nat_add(a153, a154)
}

pub fn run(p0: u64, p1: u64, p2: u64, p3: u64, p4: u64, p5: u64, p6: u64, p7: u64, p8: u64, p9: u64, p10: u64, p11: u64, p12: u64, p13: u64, p14: u64) -> R<Adt0> {
    f0(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14)
}
