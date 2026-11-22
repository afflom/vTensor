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

pub fn f0(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64, v19: u64, v20: u64) -> R<Adt0> {
    f1(v0, Adt1::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt1::C0(v1, v2, v3, v4, v9, v10, v11, v12), Adt1::C0(v1, v2, v3, v4, v13, v14, v15, v16), v17, v18, v19, v20)
}

pub fn f1(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt0> {
    let v8: u64 = {
        let a1 = v6;
        let a2 = 6u64;
        nat_sub(a1, a2)
    };
    let v9: Adt1 = f2(v3);
    let a4 = v0;
    let a5 = f3(match v1.clone() {
        Adt1::C0(h3, _, _, _, _, _, _, _) => {
            h3
        }
    }, v5, v8)?;
    if nat_lt(a4, a5) {
        let v10: u64 = f4(match v1.clone() {
            Adt1::C0(h6, _, _, _, _, _, _, _) => {
                h6
            }
        }, v5, v8)?;
        let a7 = v0;
        let a8 = v10;
        if nat_lt(a7, a8) {
            f5(v0, v9, 4u64, 6u64, v7)
        } else {
            let v11: u64 = {
                let a9 = v0;
                let a10 = v10;
                nat_sub(a9, a10)
            };
            let v12: u64 = f6(match v1.clone() {
                Adt1::C0(h11, _, _, _, _, _, _, _) => {
                    h11
                }
            })?;
            f7({
                let a12 = v11;
                let a13 = v12;
                let a14 = 0u64;
                nat_rem(a12, a13, a14)
            }, {
                let a15 = v11;
                let a16 = v12;
                let a17 = 0u64;
                nat_quot(a15, a16, a17)
            }, v1, v2, v9, v4)
        }
    } else {
        Ok(Adt0::C0)
    }
}

pub fn f2(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h18, _, _, _, _, _, _, _) => {
            h18
        }
    }, match v0.clone() {
        Adt1::C0(_, h19, _, _, _, _, _, _) => {
            h19
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h20, _, _, _, _, _) => {
            h20
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h21, _, _, _, _) => {
            h21
        }
    }, 4u64, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h22, _, _) => {
            h22
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h23, _) => {
            h23
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, h24) => {
            h24
        }
    })
}

pub fn f3(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a25 = v0;
    let a26 = 0u64;
    if if nat_eq(a25, a26) {
        true
    } else {
        let a27 = v1;
        let a28 = 0u64;
        nat_eq(a27, a28)
    } {
        Ok(0u64)
    } else {
        let a31 = f4(v0, v1, v2)?;
        let a32 = {
            let a29 = v1;
            let a30 = f6(v0)?;
            nat_mul(a29, a30)?
        };
        nat_add(a31, a32)
    }
}

pub fn f4(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a33 = v2;
    let a34 = 0u64;
    if nat_eq(a33, a34) {
        Ok(0u64)
    } else {
        let a37 = 2u64;
        let a38 = {
            let a35 = v0;
            let a36 = v1;
            nat_mul(a35, a36)?
        };
        nat_mul(a37, a38)
    }
}

pub fn f5(v0: u64, v1: Adt1, v2: u64, v3: u64, v4: u64) -> R<Adt0> {
    let v5: u64 = {
        let a45 = v3;
        let a46 = {
            let a42 = f8({
                let a39 = v0;
                let a40 = 2u64;
                let a41 = 0u64;
                nat_quot(a39, a40, a41)
            }, v1)?;
            let a43 = v2;
            let a44 = 0u64;
            nat_quot(a42, a43, a44)
        };
        nat_add(a45, a46)?
    };
    let a50 = {
        let a47 = v0;
        let a48 = 2u64;
        let a49 = 0u64;
        nat_rem(a47, a48, a49)
    };
    let a51 = 0u64;
    if nat_eq(a50, a51) {
        Ok(f9(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a52 = v4;
        let a53 = 0u64;
        if nat_eq(a52, a53) {
            Ok(f9(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt0::C0)
        }
    }
}

pub fn f6(v0: u64) -> R<u64> {
    let a56 = 13u64;
    let a57 = {
        let a54 = 9u64;
        let a55 = v0;
        nat_mul(a54, a55)?
    };
    nat_add(a56, a57)
}

pub fn f7(v0: u64, v1: u64, v2: Adt1, v3: Adt1, v4: Adt1, v5: u64) -> R<Adt0> {
    let v6: u64 = f10(match v2.clone() {
        Adt1::C0(h58, _, _, _, _, _, _, _) => {
            h58
        }
    })?;
    let v7: u64 = f11(match v2.clone() {
        Adt1::C0(h59, _, _, _, _, _, _, _) => {
            h59
        }
    })?;
    let a60 = v0;
    let a61 = 2u64;
    if nat_lt(a60, a61) {
        Ok(f12(v0))
    } else {
        let a62 = v0;
        let a63 = v6;
        if nat_lt(a62, a63) {
            f13({
                let a64 = v0;
                let a65 = 2u64;
                nat_sub(a64, a65)
            }, v1, v2, v3)
        } else {
            let a66 = v0;
            let a67 = v7;
            if nat_lt(a66, a67) {
                Ok(f14({
                    let a68 = v0;
                    let a69 = v6;
                    nat_sub(a68, a69)
                }, match v2 {
                    Adt1::C0(h70, _, _, _, _, _, _, _) => {
                        h70
                    }
                }, v5))
            } else {
                let a72 = v0;
                let a73 = f6(match v2.clone() {
                    Adt1::C0(h71, _, _, _, _, _, _, _) => {
                        h71
                    }
                })?;
                if nat_lt(a72, a73) {
                    f15({
                        let a74 = v0;
                        let a75 = v7;
                        nat_sub(a74, a75)
                    }, v1, v2, v3, v4)
                } else {
                    Ok(Adt0::C0)
                }
            }
        }
    }
}

pub fn f8(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a77 = v0;
        let a78 = match v1.clone() {
            Adt1::C0(h76, _, _, _, _, _, _, _) => {
                h76
            }
        };
        let a79 = 0u64;
        nat_quot(a77, a78, a79)
    };
    let v3: u64 = {
        let a81 = v2;
        let a82 = match v1.clone() {
            Adt1::C0(_, h80, _, _, _, _, _, _) => {
                h80
            }
        };
        let a83 = 0u64;
        nat_quot(a81, a82, a83)
    };
    f16({
        let a85 = v0;
        let a86 = match v1.clone() {
            Adt1::C0(h84, _, _, _, _, _, _, _) => {
                h84
            }
        };
        let a87 = 0u64;
        nat_rem(a85, a86, a87)
    }, {
        let a89 = v2;
        let a90 = match v1.clone() {
            Adt1::C0(_, h88, _, _, _, _, _, _) => {
                h88
            }
        };
        let a91 = 0u64;
        nat_rem(a89, a90, a91)
    }, {
        let a93 = v3;
        let a94 = match v1.clone() {
            Adt1::C0(_, _, h92, _, _, _, _, _) => {
                h92
            }
        };
        let a95 = 0u64;
        nat_rem(a93, a94, a95)
    }, {
        let a97 = v3;
        let a98 = match v1.clone() {
            Adt1::C0(_, _, h96, _, _, _, _, _) => {
                h96
            }
        };
        let a99 = 0u64;
        nat_quot(a97, a98, a99)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, h100, _, _, _) => {
            h100
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h101, _, _) => {
            h101
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h102, _) => {
            h102
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, h103) => {
            h103
        }
    })
}

pub fn f9(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> Adt0 {
    let a104 = v0;
    let a105 = 4u64;
    if if if nat_le(a104, a105) {
        true
    } else {
        let a106 = 9u64;
        let a107 = v0;
        if nat_le(a106, a107) {
            let a108 = v0;
            let a109 = 12u64;
            nat_le(a108, a109)
        } else {
            false
        }
    } {
        true
    } else {
        let a110 = v0;
        let a111 = 16u64;
        if if nat_eq(a110, a111) {
            true
        } else {
            let a112 = v0;
            let a113 = 19u64;
            nat_eq(a112, a113)
        } {
            true
        } else {
            let a114 = v0;
            let a115 = 21u64;
            if nat_eq(a114, a115) {
                true
            } else {
                let a116 = v0;
                let a117 = 22u64;
                nat_eq(a116, a117)
            }
        }
    } {
        Adt0::C1({
            let a118 = v0;
            let a119 = 4u64;
            if if nat_lt(a118, a119) {
                true
            } else {
                let a120 = 21u64;
                let a121 = v0;
                nat_le(a120, a121)
            } {
                0u64
            } else {
                let a122 = v0;
                let a123 = 16u64;
                if if nat_eq(a122, a123) {
                    true
                } else {
                    let a124 = v0;
                    let a125 = 19u64;
                    nat_eq(a124, a125)
                } {
                    2u64
                } else {
                    1u64
                }
            }
        }, {
            let a126 = v0;
            let a127 = 4u64;
            if nat_lt(a126, a127) {
                v0
            } else {
                let a128 = v0;
                let a129 = 4u64;
                if if nat_eq(a128, a129) {
                    true
                } else {
                    let a130 = v0;
                    let a131 = 16u64;
                    nat_eq(a130, a131)
                } {
                    0u64
                } else {
                    let a132 = v0;
                    let a133 = 19u64;
                    if nat_eq(a132, a133) {
                        3u64
                    } else {
                        let a134 = 21u64;
                        let a135 = v0;
                        if nat_le(a134, a135) {
                            let a136 = v0;
                            let a137 = 17u64;
                            nat_sub(a136, a137)
                        } else {
                            let a138 = v0;
                            let a139 = 8u64;
                            nat_sub(a138, a139)
                        }
                    }
                }
            }
        }, v1, v2, v3, v4, v5, v6)
    } else {
        Adt0::C0
    }
}

pub fn f10(v0: u64) -> R<u64> {
    let a142 = 2u64;
    let a143 = {
        let a140 = 6u64;
        let a141 = v0;
        nat_mul(a140, a141)?
    };
    nat_add(a142, a143)
}

pub fn f11(v0: u64) -> R<u64> {
    let a144 = f10(v0)?;
    let a145 = 11u64;
    nat_add(a144, a145)
}

pub fn f12(v0: u64) -> Adt0 {
    f9(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f13(v0: u64, v1: u64, v2: Adt1, v3: Adt1) -> R<Adt0> {
    let v4: u64 = {
        let a146 = v0;
        let a147 = 6u64;
        let a148 = 0u64;
        nat_rem(a146, a147, a148)
    };
    let v5: u64 = {
        let a155 = {
            let a150 = v1;
            let a151 = match v2.clone() {
                Adt1::C0(h149, _, _, _, _, _, _, _) => {
                    h149
                }
            };
            nat_mul(a150, a151)?
        };
        let a156 = {
            let a152 = v0;
            let a153 = 6u64;
            let a154 = 0u64;
            nat_quot(a152, a153, a154)
        };
        nat_add(a155, a156)?
    };
    let v6: u64 = f8(v5, v3)?;
    let v7: u64 = f8(v5, v2)?;
    let a157 = v4;
    let a158 = 0u64;
    if nat_eq(a157, a158) {
        Ok(f9(10u64, 4u64, v6, 4u64, v6, 1u64, 2u64))
    } else {
        let a159 = v4;
        let a160 = 1u64;
        if nat_eq(a159, a160) {
            Ok(f9(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a161 = v4;
            let a162 = 2u64;
            if nat_eq(a161, a162) {
                Ok(f9(0u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            } else {
                let a163 = v4;
                let a164 = 3u64;
                if nat_eq(a163, a164) {
                    Ok(f9(10u64, 4u64, v6, 0u64, v7, 1u64, 2u64))
                } else {
                    let a165 = v4;
                    let a166 = 4u64;
                    if nat_eq(a165, a166) {
                        Ok(f9(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(f9(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                    }
                }
            }
        }
    }
}

pub fn f14(v0: u64, v1: u64, v2: u64) -> Adt0 {
    let a167 = v0;
    let a168 = 0u64;
    if nat_eq(a167, a168) {
        f9(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 3u64)
    } else {
        let a169 = v0;
        let a170 = 1u64;
        if nat_eq(a169, a170) {
            f9(3u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
        } else {
            let a171 = v0;
            let a172 = 2u64;
            if nat_eq(a171, a172) {
                f9(11u64, 1u64, 3u64, 1u64, 2u64, 1u64, 4u64)
            } else {
                let a173 = v0;
                let a174 = 3u64;
                if nat_eq(a173, a174) {
                    f9(4u64, 1u64, 4u64, 2u64, v2, 1u64, 4u64)
                } else {
                    let a175 = v0;
                    let a176 = 4u64;
                    if nat_eq(a175, a176) {
                        f9(12u64, 1u64, 4u64, 2u64, 0u64, 1u64, 4u64)
                    } else {
                        let a177 = v0;
                        let a178 = 5u64;
                        if nat_eq(a177, a178) {
                            f9(11u64, 2u64, 1065353216u64, 1u64, 4u64, 1u64, 4u64)
                        } else {
                            let a179 = v0;
                            let a180 = 6u64;
                            if nat_eq(a179, a180) {
                                f9(10u64, 2u64, v2, 1u64, 2u64, 1u64, 5u64)
                            } else {
                                let a181 = v0;
                                let a182 = 7u64;
                                if nat_eq(a181, a182) {
                                    f9(4u64, 1u64, 3u64, 1u64, 5u64, 1u64, 5u64)
                                } else {
                                    let a183 = v0;
                                    let a184 = 8u64;
                                    if nat_eq(a183, a184) {
                                        f9(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 2u64)
                                    } else {
                                        let a185 = v0;
                                        let a186 = 9u64;
                                        if nat_eq(a185, a186) {
                                            Adt0::C1(3u64, 0u64, 1u64, 2u64, 2u64, 2u64, 1u64, 2u64)
                                        } else {
                                            let a187 = v0;
                                            let a188 = 10u64;
                                            if nat_eq(a187, a188) {
                                                f9(11u64, 1u64, 2u64, 1u64, 5u64, 1u64, 5u64)
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

pub fn f15(v0: u64, v1: u64, v2: Adt1, v3: Adt1, v4: Adt1) -> R<Adt0> {
    let v5: u64 = {
        let a189 = v0;
        let a190 = 3u64;
        let a191 = 0u64;
        nat_rem(a189, a190, a191)
    };
    let v6: u64 = {
        let a198 = {
            let a193 = v1;
            let a194 = match v2.clone() {
                Adt1::C0(h192, _, _, _, _, _, _, _) => {
                    h192
                }
            };
            nat_mul(a193, a194)?
        };
        let a199 = {
            let a195 = v0;
            let a196 = 3u64;
            let a197 = 0u64;
            nat_quot(a195, a196, a197)
        };
        nat_add(a198, a199)?
    };
    let v7: u64 = f8(v6, v3)?;
    let v8: u64 = f8(v6, v2)?;
    let v9: u64 = f8(v6, v4)?;
    let a200 = v5;
    let a201 = 0u64;
    if nat_eq(a200, a201) {
        Ok(f9(10u64, 4u64, v7, 1u64, 5u64, 1u64, 2u64))
    } else {
        let a202 = v5;
        let a203 = 1u64;
        if nat_eq(a202, a203) {
            Ok(f9(4u64, 0u64, v8, 1u64, 2u64, 1u64, 2u64))
        } else {
            Ok(f9(10u64, 1u64, 2u64, 1u64, 4u64, 3u64, v9))
        }
    }
}

pub fn f16(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a216 = {
        let a212 = {
            let a208 = {
                let a204 = v0;
                let a205 = v4;
                nat_mul(a204, a205)?
            };
            let a209 = {
                let a206 = v1;
                let a207 = v5;
                nat_mul(a206, a207)?
            };
            nat_add(a208, a209)?
        };
        let a213 = {
            let a210 = v2;
            let a211 = v6;
            nat_mul(a210, a211)?
        };
        nat_add(a212, a213)?
    };
    let a217 = {
        let a214 = v3;
        let a215 = v7;
        nat_mul(a214, a215)?
    };
    nat_add(a216, a217)
}

pub fn run(p0: u64, p1: u64, p2: u64, p3: u64, p4: u64, p5: u64, p6: u64, p7: u64, p8: u64, p9: u64, p10: u64, p11: u64, p12: u64, p13: u64, p14: u64, p15: u64, p16: u64, p17: u64, p18: u64, p19: u64, p20: u64) -> R<Adt0> {
    f0(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15, p16, p17, p18, p19, p20)
}
