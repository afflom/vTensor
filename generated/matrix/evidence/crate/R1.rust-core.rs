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
    C0(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64),
    C1(bool, u64, u64, u64),
    C2,
    C3,
    C4(u64, u64, u64, u64, u64, u64),
}

pub fn f0(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: Adt0, v9: Adt0, v10: Adt0, v11: u64, v12: u64, v13: u64) -> R<Adt1> {
    let m1 = f1(v0, v1, v2, v3, v4, v5, v6, v7, f2(v13, v8), f2(v13, v9), v10)?;
    match m1 {
        Adt1::C0(v18, v19) => {
            Ok(Adt1::C0(v18, v19))
        }
        Adt1::C1(v14, v15, v16, v17) => {
            if v14 {
                Ok(Adt1::C1(true, v15, v16, v17))
            } else {
                Ok(f3(v15, v16, v17, {
                    let a2 = v15;
                    let a3 = 0u64;
                    if nat_eq(a2, a3) {
                        v11
                    } else {
                        v12
                    }
                }, f4(v11)))
            }
        }
        Adt1::C2 => {
            Ok(Adt1::C2)
        }
        Adt1::C3 => {
            Ok(Adt1::C3)
        }
        Adt1::C4(v20, v21, v22, v23, v24, v25) => {
            Ok(Adt1::C4(v20, v21, v22, v23, v24, v25))
        }
    }
}

pub fn f1(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: Adt0, v9: Adt0, v10: Adt0) -> R<Adt1> {
    if f5(v1, v3, v4, v5) {
        let a4 = v0;
        let a5 = 0u64;
        if nat_eq(a4, a5) {
            Ok(Adt1::C0(1u64, 0u64))
        } else {
            Ok(Adt1::C3)
        }
    } else {
        let a6 = 0u64;
        let a7 = v6;
        if if nat_lt(a6, a7) {
            let a8 = 0u64;
            let a9 = v7;
            nat_lt(a8, a9)
        } else {
            false
        } {
            let v11: u64 = {
                let a10 = v1;
                let a11 = v2;
                nat_mul(a10, a11)?
            };
            let v12: u64 = {
                let a12 = v2;
                let a13 = v3;
                nat_mul(a12, a13)?
            };
            let v13: u64 = {
                let a14 = v1;
                let a15 = v3;
                nat_mul(a14, a15)?
            };
            let v14: u64 = {
                let a20 = {
                    let a18 = {
                        let a16 = v11;
                        let a17 = v12;
                        nat_add(a16, a17)?
                    };
                    let a19 = 1u64;
                    nat_add(a18, a19)?
                };
                let a21 = v13;
                nat_add(a20, a21)?
            };
            let v15: u64 = {
                let a26 = 1u64;
                let a27 = {
                    let a24 = {
                        let a22 = v4;
                        let a23 = v5;
                        nat_mul(a22, a23)?
                    };
                    let a25 = v14;
                    nat_mul(a24, a25)?
                };
                nat_add(a26, a27)?
            };
            let a28 = v0;
            let a29 = 0u64;
            if nat_eq(a28, a29) {
                Ok(Adt1::C0(v15, {
                    let a32 = {
                        let a30 = v11;
                        let a31 = v12;
                        nat_add(a30, a31)?
                    };
                    let a33 = v13;
                    nat_add(a32, a33)?
                }))
            } else {
                let a34 = v0;
                let a35 = v15;
                if nat_lt(a34, a35) {
                    let v16: u64 = {
                        let a36 = v0;
                        let a37 = 1u64;
                        nat_sub(a36, a37)
                    };
                    let v17: u64 = {
                        let a38 = v16;
                        let a39 = v14;
                        let a40 = 0u64;
                        nat_quot(a38, a39, a40)
                    };
                    let v18: u64 = {
                        let a41 = v16;
                        let a42 = v14;
                        let a43 = 0u64;
                        nat_rem(a41, a42, a43)
                    };
                    let v19: u64 = {
                        let a44 = v17;
                        let a45 = v4;
                        let a46 = 0u64;
                        nat_rem(a44, a45, a46)
                    };
                    let v20: u64 = {
                        let a47 = v17;
                        let a48 = v4;
                        let a49 = 0u64;
                        nat_quot(a47, a48, a49)
                    };
                    let a50 = v18;
                    let a51 = v11;
                    if nat_lt(a50, a51) {
                        Ok(Adt1::C1(false, 0u64, v18, f6({
                            let a52 = v18;
                            let a53 = v2;
                            let a54 = 0u64;
                            nat_rem(a52, a53, a54)
                        }, {
                            let a55 = v18;
                            let a56 = v2;
                            let a57 = 0u64;
                            nat_quot(a55, a56, a57)
                        }, {
                            let a58 = v19;
                            let a59 = v6;
                            let a60 = 0u64;
                            nat_quot(a58, a59, a60)
                        }, {
                            let a61 = v20;
                            let a62 = v7;
                            let a63 = 0u64;
                            nat_quot(a61, a62, a63)
                        }, v8)?))
                    } else {
                        let a66 = v18;
                        let a67 = {
                            let a64 = v11;
                            let a65 = v12;
                            nat_add(a64, a65)?
                        };
                        if nat_lt(a66, a67) {
                            let v21: u64 = {
                                let a68 = v18;
                                let a69 = v11;
                                nat_sub(a68, a69)
                            };
                            Ok(Adt1::C1(false, 1u64, v18, f6({
                                let a70 = v21;
                                let a71 = v3;
                                let a72 = 0u64;
                                nat_quot(a70, a71, a72)
                            }, {
                                let a73 = v21;
                                let a74 = v3;
                                let a75 = 0u64;
                                nat_rem(a73, a74, a75)
                            }, v19, v20, v9)?))
                        } else {
                            let a78 = v18;
                            let a79 = {
                                let a76 = v11;
                                let a77 = v12;
                                nat_add(a76, a77)?
                            };
                            if nat_eq(a78, a79) {
                                Ok(Adt1::C2)
                            } else {
                                let v22: u64 = {
                                    let a84 = {
                                        let a82 = v18;
                                        let a83 = {
                                            let a80 = v11;
                                            let a81 = v12;
                                            nat_add(a80, a81)?
                                        };
                                        nat_sub(a82, a83)
                                    };
                                    let a85 = 1u64;
                                    nat_sub(a84, a85)
                                };
                                Ok(Adt1::C1(true, 2u64, {
                                    let a88 = {
                                        let a86 = v11;
                                        let a87 = v12;
                                        nat_add(a86, a87)?
                                    };
                                    let a89 = v22;
                                    nat_add(a88, a89)?
                                }, f6({
                                    let a90 = v22;
                                    let a91 = v3;
                                    let a92 = 0u64;
                                    nat_quot(a90, a91, a92)
                                }, {
                                    let a93 = v22;
                                    let a94 = v3;
                                    let a95 = 0u64;
                                    nat_rem(a93, a94, a95)
                                }, v19, v20, v10)?))
                            }
                        }
                    }
                } else {
                    Ok(Adt1::C3)
                }
            }
        } else {
            Ok(Adt1::C3)
        }
    }
}

pub fn f2(v0: u64, v1: Adt0) -> Adt0 {
    let a96 = v0;
    let a97 = 30u64;
    if nat_eq(a96, a97) {
        Adt0::C0(match v1.clone() {
            Adt0::C0(_, h98, _, _) => {
                h98
            }
        }, match v1.clone() {
            Adt0::C0(h99, _, _, _) => {
                h99
            }
        }, match v1.clone() {
            Adt0::C0(_, _, h100, _) => {
                h100
            }
        }, match v1 {
            Adt0::C0(_, _, _, h101) => {
                h101
            }
        })
    } else {
        v1
    }
}

pub fn f3(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt1 {
    let a102 = v3;
    let a103 = 0u64;
    if if nat_eq(a102, a103) {
        let a104 = v4;
        let a105 = 0u64;
        nat_eq(a104, a105)
    } else {
        false
    } {
        Adt1::C1(false, v0, v1, v2)
    } else {
        Adt1::C4(v0, v1, v2, v3, v4, {
            let a106 = v3;
            let a107 = 0u64;
            if nat_eq(a106, a107) {
                4u64
            } else {
                2u64
            }
        })
    }
}

pub fn f4(v0: u64) -> u64 {
    let a108 = v0;
    let a109 = 30u64;
    if nat_eq(a108, a109) {
        30u64
    } else {
        0u64
    }
}

pub fn f5(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a110 = v0;
    let a111 = 0u64;
    if if if nat_eq(a110, a111) {
        true
    } else {
        let a112 = v1;
        let a113 = 0u64;
        nat_eq(a112, a113)
    } {
        true
    } else {
        let a114 = v2;
        let a115 = 0u64;
        nat_eq(a114, a115)
    } {
        true
    } else {
        let a116 = v3;
        let a117 = 0u64;
        nat_eq(a116, a117)
    }
}

pub fn f6(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt0) -> R<u64> {
    let a134 = {
        let a129 = {
            let a124 = {
                let a119 = v0;
                let a120 = match v4.clone() {
                    Adt0::C0(h118, _, _, _) => {
                        h118
                    }
                };
                nat_mul(a119, a120)?
            };
            let a125 = {
                let a122 = v1;
                let a123 = match v4.clone() {
                    Adt0::C0(_, h121, _, _) => {
                        h121
                    }
                };
                nat_mul(a122, a123)?
            };
            nat_add(a124, a125)?
        };
        let a130 = {
            let a127 = v2;
            let a128 = match v4.clone() {
                Adt0::C0(_, _, h126, _) => {
                    h126
                }
            };
            nat_mul(a127, a128)?
        };
        nat_add(a129, a130)?
    };
    let a135 = {
        let a132 = v3;
        let a133 = match v4 {
            Adt0::C0(_, _, _, h131) => {
                h131
            }
        };
        nat_mul(a132, a133)?
    };
    nat_add(a134, a135)
}
