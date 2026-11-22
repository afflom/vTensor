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

pub fn f0(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64) -> R<Adt0> {
    f1(v0, Adt1::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt1::C0(v1, v2, v3, v4, v9, v10, v11, v12), v13, v14, v15, v16)
}

pub fn f1(v0: u64, v1: Adt1, v2: Adt1, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt0> {
    let v7: u64 = {
        let a1 = v5;
        let a2 = 6u64;
        nat_sub(a1, a2)
    };
    let v8: bool = f2(match v1.clone() {
        Adt1::C0(_, _, _, _, h3, _, _, _) => {
            h3
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, h4, _, _, _) => {
            h4
        }
    });
    let a6 = v0;
    let a7 = f3(v8, match v1.clone() {
        Adt1::C0(h5, _, _, _, _, _, _, _) => {
            h5
        }
    }, v4, v7)?;
    if nat_lt(a6, a7) {
        let v9: u64 = f4(match v1.clone() {
            Adt1::C0(h8, _, _, _, _, _, _, _) => {
                h8
            }
        }, v4, v7)?;
        let a9 = v0;
        let a10 = v9;
        if nat_lt(a9, a10) {
            f5(v0, v2, 4u64, 6u64, v6)
        } else {
            let v10: u64 = {
                let a11 = v0;
                let a12 = v9;
                nat_sub(a11, a12)
            };
            let v11: u64 = f6(v8, match v1.clone() {
                Adt1::C0(h13, _, _, _, _, _, _, _) => {
                    h13
                }
            })?;
            f7({
                let a14 = v10;
                let a15 = v11;
                let a16 = 0u64;
                nat_rem(a14, a15, a16)
            }, {
                let a17 = v10;
                let a18 = v11;
                let a19 = 0u64;
                nat_quot(a17, a18, a19)
            }, v1, v2, v3, v8)
        }
    } else {
        Ok(Adt0::C0)
    }
}

pub fn f2(v0: u64, v1: u64) -> bool {
    let a20 = v0;
    let a21 = 4u64;
    if nat_eq(a20, a21) {
        let a22 = v1;
        let a23 = 4u64;
        nat_eq(a22, a23)
    } else {
        false
    }
}

pub fn f3(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a24 = v1;
    let a25 = 0u64;
    if if nat_eq(a24, a25) {
        true
    } else {
        let a26 = v2;
        let a27 = 0u64;
        nat_eq(a26, a27)
    } {
        Ok(0u64)
    } else {
        let a30 = f4(v1, v2, v3)?;
        let a31 = {
            let a28 = v2;
            let a29 = f6(v0, v1)?;
            nat_mul(a28, a29)?
        };
        nat_add(a30, a31)
    }
}

pub fn f4(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a32 = v2;
    let a33 = 0u64;
    if nat_eq(a32, a33) {
        Ok(0u64)
    } else {
        let a36 = 2u64;
        let a37 = {
            let a34 = v0;
            let a35 = v1;
            nat_mul(a34, a35)?
        };
        nat_mul(a36, a37)
    }
}

pub fn f5(v0: u64, v1: Adt1, v2: u64, v3: u64, v4: u64) -> R<Adt0> {
    let v5: u64 = {
        let a44 = v3;
        let a45 = {
            let a41 = f8({
                let a38 = v0;
                let a39 = 2u64;
                let a40 = 0u64;
                nat_quot(a38, a39, a40)
            }, v1)?;
            let a42 = v2;
            let a43 = 0u64;
            nat_quot(a41, a42, a43)
        };
        nat_add(a44, a45)?
    };
    let a49 = {
        let a46 = v0;
        let a47 = 2u64;
        let a48 = 0u64;
        nat_rem(a46, a47, a48)
    };
    let a50 = 0u64;
    if nat_eq(a49, a50) {
        Ok(f9(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a51 = v4;
        let a52 = 0u64;
        if nat_eq(a51, a52) {
            Ok(f9(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt0::C0)
        }
    }
}

pub fn f6(v0: bool, v1: u64) -> R<u64> {
    let a57 = {
        let a55 = {
            let a53 = f10(v0, v1)?;
            let a54 = f11(v0, v1)?;
            nat_add(a53, a54)?
        };
        let a56 = f12(v0);
        nat_add(a55, a56)?
    };
    let a58 = v1;
    nat_add(a57, a58)
}

pub fn f7(v0: u64, v1: u64, v2: Adt1, v3: Adt1, v4: u64, v5: bool) -> R<Adt0> {
    let v6: u64 = f10(v5, match v2.clone() {
        Adt1::C0(h59, _, _, _, _, _, _, _) => {
            h59
        }
    })?;
    let v7: u64 = {
        let a61 = v6;
        let a62 = f11(v5, match v2.clone() {
            Adt1::C0(h60, _, _, _, _, _, _, _) => {
                h60
            }
        })?;
        nat_add(a61, a62)?
    };
    let v8: u64 = {
        let a63 = v7;
        let a64 = f12(v5);
        nat_add(a63, a64)?
    };
    let a65 = v0;
    let a66 = v6;
    if nat_lt(a65, a66) {
        f13(v0, v1, v2, v5)
    } else {
        let a67 = v0;
        let a68 = v7;
        if nat_lt(a67, a68) {
            f14({
                let a69 = v0;
                let a70 = v6;
                nat_sub(a69, a70)
            }, v1, v2, v3, v5)
        } else {
            let a71 = v0;
            let a72 = v8;
            if nat_lt(a71, a72) {
                Ok(f15({
                    let a73 = v0;
                    let a74 = v7;
                    nat_sub(a73, a74)
                }, match v2 {
                    Adt1::C0(h75, _, _, _, _, _, _, _) => {
                        h75
                    }
                }, v4, v5))
            } else {
                let a79 = v0;
                let a80 = {
                    let a77 = v8;
                    let a78 = match v2.clone() {
                        Adt1::C0(h76, _, _, _, _, _, _, _) => {
                            h76
                        }
                    };
                    nat_add(a77, a78)?
                };
                if nat_lt(a79, a80) {
                    let v9: u64 = f8({
                        let a86 = {
                            let a82 = v1;
                            let a83 = match v2 {
                                Adt1::C0(h81, _, _, _, _, _, _, _) => {
                                    h81
                                }
                            };
                            nat_mul(a82, a83)?
                        };
                        let a87 = {
                            let a84 = v0;
                            let a85 = v8;
                            nat_sub(a84, a85)
                        };
                        nat_add(a86, a87)?
                    }, v3)?;
                    Ok(f9(10u64, 3u64, v9, 1u64, 1u64, 3u64, v9))
                } else {
                    Ok(Adt0::C0)
                }
            }
        }
    }
}

pub fn f8(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a89 = v0;
        let a90 = match v1.clone() {
            Adt1::C0(h88, _, _, _, _, _, _, _) => {
                h88
            }
        };
        let a91 = 0u64;
        nat_quot(a89, a90, a91)
    };
    let v3: u64 = {
        let a93 = v2;
        let a94 = match v1.clone() {
            Adt1::C0(_, h92, _, _, _, _, _, _) => {
                h92
            }
        };
        let a95 = 0u64;
        nat_quot(a93, a94, a95)
    };
    f16({
        let a97 = v0;
        let a98 = match v1.clone() {
            Adt1::C0(h96, _, _, _, _, _, _, _) => {
                h96
            }
        };
        let a99 = 0u64;
        nat_rem(a97, a98, a99)
    }, {
        let a101 = v2;
        let a102 = match v1.clone() {
            Adt1::C0(_, h100, _, _, _, _, _, _) => {
                h100
            }
        };
        let a103 = 0u64;
        nat_rem(a101, a102, a103)
    }, {
        let a105 = v3;
        let a106 = match v1.clone() {
            Adt1::C0(_, _, h104, _, _, _, _, _) => {
                h104
            }
        };
        let a107 = 0u64;
        nat_rem(a105, a106, a107)
    }, {
        let a109 = v3;
        let a110 = match v1.clone() {
            Adt1::C0(_, _, h108, _, _, _, _, _) => {
                h108
            }
        };
        let a111 = 0u64;
        nat_quot(a109, a110, a111)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, h112, _, _, _) => {
            h112
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h113, _, _) => {
            h113
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h114, _) => {
            h114
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, h115) => {
            h115
        }
    })
}

pub fn f9(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> Adt0 {
    let a116 = v0;
    let a117 = 4u64;
    if if if nat_le(a116, a117) {
        true
    } else {
        let a118 = 9u64;
        let a119 = v0;
        if nat_le(a118, a119) {
            let a120 = v0;
            let a121 = 12u64;
            nat_le(a120, a121)
        } else {
            false
        }
    } {
        true
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
            true
        } else {
            let a126 = v0;
            let a127 = 21u64;
            if nat_eq(a126, a127) {
                true
            } else {
                let a128 = v0;
                let a129 = 22u64;
                nat_eq(a128, a129)
            }
        }
    } {
        Adt0::C1({
            let a130 = v0;
            let a131 = 4u64;
            if if nat_lt(a130, a131) {
                true
            } else {
                let a132 = 21u64;
                let a133 = v0;
                nat_le(a132, a133)
            } {
                0u64
            } else {
                let a134 = v0;
                let a135 = 16u64;
                if if nat_eq(a134, a135) {
                    true
                } else {
                    let a136 = v0;
                    let a137 = 19u64;
                    nat_eq(a136, a137)
                } {
                    2u64
                } else {
                    1u64
                }
            }
        }, {
            let a138 = v0;
            let a139 = 4u64;
            if nat_lt(a138, a139) {
                v0
            } else {
                let a140 = v0;
                let a141 = 4u64;
                if if nat_eq(a140, a141) {
                    true
                } else {
                    let a142 = v0;
                    let a143 = 16u64;
                    nat_eq(a142, a143)
                } {
                    0u64
                } else {
                    let a144 = v0;
                    let a145 = 19u64;
                    if nat_eq(a144, a145) {
                        3u64
                    } else {
                        let a146 = 21u64;
                        let a147 = v0;
                        if nat_le(a146, a147) {
                            let a148 = v0;
                            let a149 = 17u64;
                            nat_sub(a148, a149)
                        } else {
                            let a150 = v0;
                            let a151 = 8u64;
                            nat_sub(a150, a151)
                        }
                    }
                }
            }
        }, v1, v2, v3, v4, v5, v6)
    } else {
        Adt0::C0
    }
}

pub fn f10(v0: bool, v1: u64) -> R<u64> {
    let a154 = if v0 {
        5u64
    } else {
        4u64
    };
    let a155 = {
        let a152 = f17(v0);
        let a153 = v1;
        nat_mul(a152, a153)?
    };
    nat_add(a154, a155)
}

pub fn f11(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a163 = {
            let a156 = 13u64;
            let a157 = f18(v1);
            nat_mul(a156, a157)?
        };
        let a164 = {
            let a161 = 4u64;
            let a162 = {
                let a158 = v1;
                let a159 = 4u64;
                let a160 = 0u64;
                nat_rem(a158, a159, a160)
            };
            nat_mul(a161, a162)?
        };
        nat_add(a163, a164)
    } else {
        let a165 = 3u64;
        let a166 = v1;
        nat_mul(a165, a166)
    }
}

pub fn f12(v0: bool) -> u64 {
    if v0 {
        6u64
    } else {
        5u64
    }
}

pub fn f13(v0: u64, v1: u64, v2: Adt1, v3: bool) -> R<Adt0> {
    let v4: u64 = match v2.clone() {
        Adt1::C0(h167, _, _, _, _, _, _, _) => {
            h167
        }
    };
    let v5: u64 = {
        let a170 = 1u64;
        let a171 = {
            let a168 = v4;
            let a169 = f17(v3);
            nat_mul(a168, a169)?
        };
        nat_add(a170, a171)?
    };
    let a172 = v0;
    let a173 = 0u64;
    if nat_eq(a172, a173) {
        Ok(f19(0u64))
    } else {
        let a174 = v0;
        let a175 = v5;
        if nat_lt(a174, a175) {
            let v6: u64 = {
                let a176 = v0;
                let a177 = 1u64;
                nat_sub(a176, a177)
            };
            let v7: u64 = f8({
                let a183 = {
                    let a178 = v1;
                    let a179 = v4;
                    nat_mul(a178, a179)?
                };
                let a184 = {
                    let a180 = v6;
                    let a181 = f17(v3);
                    let a182 = 0u64;
                    nat_quot(a180, a181, a182)
                };
                nat_add(a183, a184)?
            }, v2)?;
            if v3 {
                let a188 = {
                    let a185 = v6;
                    let a186 = 2u64;
                    let a187 = 0u64;
                    nat_rem(a185, a186, a187)
                };
                let a189 = 0u64;
                if nat_eq(a188, a189) {
                    Ok(f9(1u64, 0u64, v7, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f9(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            } else {
                Ok(f9(4u64, 1u64, 0u64, 0u64, v7, 1u64, 0u64))
            }
        } else {
            let v8: u64 = {
                let a190 = v0;
                let a191 = v5;
                nat_sub(a190, a191)
            };
            let a192 = v8;
            let a193 = 0u64;
            if nat_eq(a192, a193) {
                Ok(f9(3u64, 2u64, v4, 2u64, 0u64, 1u64, 1u64))
            } else {
                if if v3 {
                    let a194 = v8;
                    let a195 = 1u64;
                    nat_eq(a194, a195)
                } else {
                    false
                } {
                    Ok(f9(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a196 = v8;
                    let a197 = if v3 {
                        2u64
                    } else {
                        1u64
                    };
                    if nat_eq(a196, a197) {
                        Ok(f9(11u64, 1u64, 0u64, 1u64, 1u64, 1u64, 3u64))
                    } else {
                        let a198 = v8;
                        let a199 = if v3 {
                            3u64
                        } else {
                            2u64
                        };
                        if nat_eq(a198, a199) {
                            Ok(f19(0u64))
                        } else {
                            Ok(Adt0::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f14(v0: u64, v1: u64, v2: Adt1, v3: Adt1, v4: bool) -> R<Adt0> {
    let v5: u64 = if v4 {
        let a201 = 13u64;
        let a202 = f18(match v2.clone() {
            Adt1::C0(h200, _, _, _, _, _, _, _) => {
                h200
            }
        });
        nat_mul(a201, a202)?
    } else {
        0u64
    };
    let a203 = v0;
    let a204 = v5;
    if nat_lt(a203, a204) {
        f20({
            let a205 = v0;
            let a206 = 13u64;
            let a207 = 0u64;
            nat_rem(a205, a206, a207)
        }, v1, {
            let a208 = v0;
            let a209 = 13u64;
            let a210 = 0u64;
            nat_quot(a208, a209, a210)
        }, v2, v3)
    } else {
        let v6: u64 = {
            let a211 = v0;
            let a212 = v5;
            nat_sub(a211, a212)
        };
        let v7: u64 = if v4 {
            4u64
        } else {
            3u64
        };
        f21({
            let a213 = v6;
            let a214 = v7;
            let a215 = 0u64;
            nat_rem(a213, a214, a215)
        }, {
            let a225 = {
                let a220 = {
                    let a217 = v1;
                    let a218 = match v2.clone() {
                        Adt1::C0(h216, _, _, _, _, _, _, _) => {
                            h216
                        }
                    };
                    nat_mul(a217, a218)?
                };
                let a221 = if v4 {
                    f22(match v2.clone() {
                        Adt1::C0(h219, _, _, _, _, _, _, _) => {
                            h219
                        }
                    })?
                } else {
                    0u64
                };
                nat_add(a220, a221)?
            };
            let a226 = {
                let a222 = v6;
                let a223 = v7;
                let a224 = 0u64;
                nat_quot(a222, a223, a224)
            };
            nat_add(a225, a226)?
        }, v2, v3, v4)
    }
}

pub fn f15(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt0 {
    let a227 = v0;
    let a228 = 0u64;
    if nat_eq(a227, a228) {
        f9(if v3 {
            22u64
        } else {
            3u64
        }, 2u64, v1, 2u64, 0u64, 1u64, 1u64)
    } else {
        let a229 = v0;
        let a230 = 1u64;
        if nat_eq(a229, a230) {
            f9(if v3 {
                21u64
            } else {
                11u64
            }, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64)
        } else {
            if if v3 {
                let a231 = v0;
                let a232 = 2u64;
                nat_eq(a231, a232)
            } else {
                false
            } {
                f9(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
            } else {
                let a233 = v0;
                let a234 = if v3 {
                    3u64
                } else {
                    2u64
                };
                if nat_eq(a233, a234) {
                    f9(4u64, 1u64, 1u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a235 = v0;
                    let a236 = if v3 {
                        4u64
                    } else {
                        3u64
                    };
                    if nat_eq(a235, a236) {
                        f9(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a237 = v0;
                        let a238 = if v3 {
                            5u64
                        } else {
                            4u64
                        };
                        if nat_eq(a237, a238) {
                            f9(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt0::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f16(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a251 = {
        let a247 = {
            let a243 = {
                let a239 = v0;
                let a240 = v4;
                nat_mul(a239, a240)?
            };
            let a244 = {
                let a241 = v1;
                let a242 = v5;
                nat_mul(a241, a242)?
            };
            nat_add(a243, a244)?
        };
        let a248 = {
            let a245 = v2;
            let a246 = v6;
            nat_mul(a245, a246)?
        };
        nat_add(a247, a248)?
    };
    let a252 = {
        let a249 = v3;
        let a250 = v7;
        nat_mul(a249, a250)?
    };
    nat_add(a251, a252)
}

pub fn f17(v0: bool) -> u64 {
    if v0 {
        2u64
    } else {
        1u64
    }
}

pub fn f18(v0: u64) -> u64 {
    let a253 = v0;
    let a254 = 4u64;
    let a255 = 0u64;
    nat_quot(a253, a254, a255)
}

pub fn f19(v0: u64) -> Adt0 {
    f9(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f20(v0: u64, v1: u64, v2: u64, v3: Adt1, v4: Adt1) -> R<Adt0> {
    let v5: u64 = {
        let a263 = {
            let a261 = {
                let a257 = v1;
                let a258 = match v3.clone() {
                    Adt1::C0(h256, _, _, _, _, _, _, _) => {
                        h256
                    }
                };
                nat_mul(a257, a258)?
            };
            let a262 = {
                let a259 = v2;
                let a260 = 4u64;
                nat_mul(a259, a260)?
            };
            nat_add(a261, a262)?
        };
        let a264 = f23(v0);
        nat_add(a263, a264)?
    };
    let v6: u64 = f8(v5, v3)?;
    let v7: u64 = f8(v5, v4)?;
    let a265 = v0;
    let a266 = 0u64;
    if if if if nat_eq(a265, a266) {
        true
    } else {
        let a267 = v0;
        let a268 = 2u64;
        nat_eq(a267, a268)
    } {
        true
    } else {
        let a269 = v0;
        let a270 = 5u64;
        nat_eq(a269, a270)
    } {
        true
    } else {
        let a271 = v0;
        let a272 = 7u64;
        nat_eq(a271, a272)
    } {
        Ok(f9(9u64, 0u64, v6, 1u64, 3u64, 3u64, v7))
    } else {
        let a273 = v0;
        let a274 = 1u64;
        if if if if nat_eq(a273, a274) {
            true
        } else {
            let a275 = v0;
            let a276 = 3u64;
            nat_eq(a275, a276)
        } {
            true
        } else {
            let a277 = v0;
            let a278 = 6u64;
            nat_eq(a277, a278)
        } {
            true
        } else {
            let a279 = v0;
            let a280 = 8u64;
            nat_eq(a279, a280)
        } {
            Ok(f9(10u64, 3u64, v7, 3u64, v7, 1u64, {
                let a281 = v0;
                let a282 = 1u64;
                if nat_eq(a281, a282) {
                    4u64
                } else {
                    let a283 = v0;
                    let a284 = 6u64;
                    if nat_eq(a283, a284) {
                        5u64
                    } else {
                        1u64
                    }
                }
            }))
        } else {
            let a285 = v0;
            let a286 = 4u64;
            if nat_eq(a285, a286) {
                Ok(f9(4u64, 1u64, 4u64, 1u64, 1u64, 1u64, 4u64))
            } else {
                let a287 = v0;
                let a288 = 9u64;
                if nat_eq(a287, a288) {
                    Ok(f9(4u64, 1u64, 5u64, 1u64, 1u64, 1u64, 5u64))
                } else {
                    let a289 = v0;
                    let a290 = 10u64;
                    if nat_eq(a289, a290) {
                        Ok(f9(4u64, 1u64, 4u64, 1u64, 5u64, 1u64, 1u64))
                    } else {
                        let a291 = v0;
                        let a292 = 11u64;
                        if nat_eq(a291, a292) {
                            Ok(f9(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            let a293 = v0;
                            let a294 = 12u64;
                            if nat_eq(a293, a294) {
                                Ok(f9(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
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

pub fn f21(v0: u64, v1: u64, v2: Adt1, v3: Adt1, v4: bool) -> R<Adt0> {
    let v5: u64 = f8(v1, v2)?;
    let v6: u64 = f8(v1, v3)?;
    let a295 = v0;
    let a296 = 0u64;
    if nat_eq(a295, a296) {
        Ok(f9(9u64, 0u64, v5, 1u64, 3u64, 3u64, v6))
    } else {
        let a297 = v0;
        let a298 = 1u64;
        if nat_eq(a297, a298) {
            Ok(f9(10u64, 3u64, v6, 3u64, v6, 1u64, 1u64))
        } else {
            if v4 {
                let a299 = v0;
                let a300 = 2u64;
                if nat_eq(a299, a300) {
                    Ok(f9(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a301 = v0;
                    let a302 = 3u64;
                    if nat_eq(a301, a302) {
                        Ok(f9(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    } else {
                        Ok(Adt0::C0)
                    }
                }
            } else {
                let a303 = v0;
                let a304 = 2u64;
                if nat_eq(a303, a304) {
                    Ok(f9(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    Ok(Adt0::C0)
                }
            }
        }
    }
}

pub fn f22(v0: u64) -> R<u64> {
    let a305 = 4u64;
    let a306 = f18(v0);
    nat_mul(a305, a306)
}

pub fn f23(v0: u64) -> u64 {
    let a307 = v0;
    let a308 = 5u64;
    if nat_lt(a307, a308) {
        let a309 = v0;
        let a310 = 2u64;
        if nat_lt(a309, a310) {
            0u64
        } else {
            1u64
        }
    } else {
        let a311 = v0;
        let a312 = 7u64;
        if nat_lt(a311, a312) {
            2u64
        } else {
            3u64
        }
    }
}

pub fn run(p0: u64, p1: u64, p2: u64, p3: u64, p4: u64, p5: u64, p6: u64, p7: u64, p8: u64, p9: u64, p10: u64, p11: u64, p12: u64, p13: u64, p14: u64, p15: u64, p16: u64) -> R<Adt0> {
    f0(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15, p16)
}
