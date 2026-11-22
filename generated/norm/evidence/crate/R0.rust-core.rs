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

#[derive(Clone)]
pub enum Adt2 {
    C0(u64, u64, u64, u64),
}

pub fn f0(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt0> {
    let a1 = v0;
    let a2 = 64u64;
    if nat_eq(a1, a2) {
        f1(v1, Adt1::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt1::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
    } else {
        let a3 = v0;
        let a4 = 65u64;
        if nat_eq(a3, a4) {
            f2(v1, Adt1::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt1::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
        } else {
            Ok(Adt0::C0)
        }
    }
}

pub fn f1(v0: u64, v1: Adt1, v2: Adt1, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt0> {
    let v7: u64 = {
        let a5 = v5;
        let a6 = 6u64;
        nat_sub(a5, a6)
    };
    let v8: bool = f3(match v1.clone() {
        Adt1::C0(_, _, _, _, h7, _, _, _) => {
            h7
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, h8, _, _, _) => {
            h8
        }
    });
    let a10 = v0;
    let a11 = f4(v8, match v1.clone() {
        Adt1::C0(h9, _, _, _, _, _, _, _) => {
            h9
        }
    }, v4, v7)?;
    if nat_lt(a10, a11) {
        let v9: u64 = f5(match v1.clone() {
            Adt1::C0(h12, _, _, _, _, _, _, _) => {
                h12
            }
        }, v4, v7)?;
        let a13 = v0;
        let a14 = v9;
        if nat_lt(a13, a14) {
            f6(v0, v2, 4u64, 6u64, v6)
        } else {
            let v10: u64 = {
                let a15 = v0;
                let a16 = v9;
                nat_sub(a15, a16)
            };
            let v11: u64 = f7(v8, match v1.clone() {
                Adt1::C0(h17, _, _, _, _, _, _, _) => {
                    h17
                }
            })?;
            f8({
                let a18 = v10;
                let a19 = v11;
                let a20 = 0u64;
                nat_rem(a18, a19, a20)
            }, {
                let a21 = v10;
                let a22 = v11;
                let a23 = 0u64;
                nat_quot(a21, a22, a23)
            }, v1, v2, v3, v8)
        }
    } else {
        Ok(Adt0::C0)
    }
}

pub fn f2(v0: u64, v1: Adt1, v2: Adt1, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt0> {
    let v7: u64 = {
        let a24 = v5;
        let a25 = 6u64;
        nat_sub(a24, a25)
    };
    let v8: Adt1 = f9(v2);
    let v9: u64 = f10(match v1.clone() {
        Adt1::C0(h26, _, _, _, _, _, _, _) => {
            h26
        }
    }, match v1.clone() {
        Adt1::C0(_, h27, _, _, _, _, _, _) => {
            h27
        }
    }, match v1.clone() {
        Adt1::C0(_, _, h28, _, _, _, _, _) => {
            h28
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, h29, _, _, _, _) => {
            h29
        }
    }, v7)?;
    let a34 = v0;
    let a35 = f11(match v1.clone() {
        Adt1::C0(h30, _, _, _, _, _, _, _) => {
            h30
        }
    }, match v1.clone() {
        Adt1::C0(_, h31, _, _, _, _, _, _) => {
            h31
        }
    }, match v1.clone() {
        Adt1::C0(_, _, h32, _, _, _, _, _) => {
            h32
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, h33, _, _, _, _) => {
            h33
        }
    }, v4, v7)?;
    if nat_lt(a34, a35) {
        let a36 = v0;
        let a37 = v9;
        if nat_lt(a36, a37) {
            f6(v0, v8, 4u64, 6u64, v6)
        } else {
            let v10: Adt2 = f12({
                let a38 = v0;
                let a39 = v9;
                nat_sub(a38, a39)
            }, match v1.clone() {
                Adt1::C0(h40, _, _, _, _, _, _, _) => {
                    h40
                }
            }, match v1.clone() {
                Adt1::C0(_, h41, _, _, _, _, _, _) => {
                    h41
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h42, _, _, _, _, _) => {
                    h42
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, h43, _, _, _, _) => {
                    h43
                }
            }, v4)?;
            f13(v10, v1, v8, v3)
        }
    } else {
        Ok(Adt0::C0)
    }
}

pub fn f3(v0: u64, v1: u64) -> bool {
    let a44 = v0;
    let a45 = 4u64;
    if nat_eq(a44, a45) {
        let a46 = v1;
        let a47 = 4u64;
        nat_eq(a46, a47)
    } else {
        false
    }
}

pub fn f4(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a48 = v1;
    let a49 = 0u64;
    if if nat_eq(a48, a49) {
        true
    } else {
        let a50 = v2;
        let a51 = 0u64;
        nat_eq(a50, a51)
    } {
        Ok(0u64)
    } else {
        let a54 = f5(v1, v2, v3)?;
        let a55 = {
            let a52 = v2;
            let a53 = f7(v0, v1)?;
            nat_mul(a52, a53)?
        };
        nat_add(a54, a55)
    }
}

pub fn f5(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a56 = v2;
    let a57 = 0u64;
    if nat_eq(a56, a57) {
        Ok(0u64)
    } else {
        let a60 = 2u64;
        let a61 = {
            let a58 = v0;
            let a59 = v1;
            nat_mul(a58, a59)?
        };
        nat_mul(a60, a61)
    }
}

pub fn f6(v0: u64, v1: Adt1, v2: u64, v3: u64, v4: u64) -> R<Adt0> {
    let v5: u64 = {
        let a68 = v3;
        let a69 = {
            let a65 = f14({
                let a62 = v0;
                let a63 = 2u64;
                let a64 = 0u64;
                nat_quot(a62, a63, a64)
            }, v1)?;
            let a66 = v2;
            let a67 = 0u64;
            nat_quot(a65, a66, a67)
        };
        nat_add(a68, a69)?
    };
    let a73 = {
        let a70 = v0;
        let a71 = 2u64;
        let a72 = 0u64;
        nat_rem(a70, a71, a72)
    };
    let a74 = 0u64;
    if nat_eq(a73, a74) {
        Ok(f15(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a75 = v4;
        let a76 = 0u64;
        if nat_eq(a75, a76) {
            Ok(f15(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt0::C0)
        }
    }
}

pub fn f7(v0: bool, v1: u64) -> R<u64> {
    let a81 = {
        let a79 = {
            let a77 = f16(v0, v1)?;
            let a78 = f17(v0, v1)?;
            nat_add(a77, a78)?
        };
        let a80 = f18(v0);
        nat_add(a79, a80)?
    };
    let a82 = v1;
    nat_add(a81, a82)
}

pub fn f8(v0: u64, v1: u64, v2: Adt1, v3: Adt1, v4: u64, v5: bool) -> R<Adt0> {
    let v6: u64 = f16(v5, match v2.clone() {
        Adt1::C0(h83, _, _, _, _, _, _, _) => {
            h83
        }
    })?;
    let v7: u64 = {
        let a85 = v6;
        let a86 = f17(v5, match v2.clone() {
            Adt1::C0(h84, _, _, _, _, _, _, _) => {
                h84
            }
        })?;
        nat_add(a85, a86)?
    };
    let v8: u64 = {
        let a87 = v7;
        let a88 = f18(v5);
        nat_add(a87, a88)?
    };
    let a89 = v0;
    let a90 = v6;
    if nat_lt(a89, a90) {
        f19(v0, v1, v2, v5)
    } else {
        let a91 = v0;
        let a92 = v7;
        if nat_lt(a91, a92) {
            f20({
                let a93 = v0;
                let a94 = v6;
                nat_sub(a93, a94)
            }, v1, v2, v3, v5)
        } else {
            let a95 = v0;
            let a96 = v8;
            if nat_lt(a95, a96) {
                Ok(f21({
                    let a97 = v0;
                    let a98 = v7;
                    nat_sub(a97, a98)
                }, match v2 {
                    Adt1::C0(h99, _, _, _, _, _, _, _) => {
                        h99
                    }
                }, v4, v5))
            } else {
                let a103 = v0;
                let a104 = {
                    let a101 = v8;
                    let a102 = match v2.clone() {
                        Adt1::C0(h100, _, _, _, _, _, _, _) => {
                            h100
                        }
                    };
                    nat_add(a101, a102)?
                };
                if nat_lt(a103, a104) {
                    let v9: u64 = f14({
                        let a110 = {
                            let a106 = v1;
                            let a107 = match v2 {
                                Adt1::C0(h105, _, _, _, _, _, _, _) => {
                                    h105
                                }
                            };
                            nat_mul(a106, a107)?
                        };
                        let a111 = {
                            let a108 = v0;
                            let a109 = v8;
                            nat_sub(a108, a109)
                        };
                        nat_add(a110, a111)?
                    }, v3)?;
                    Ok(f15(10u64, 3u64, v9, 1u64, 1u64, 3u64, v9))
                } else {
                    Ok(Adt0::C0)
                }
            }
        }
    }
}

pub fn f9(v0: Adt1) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h112, _, _, _, _, _, _, _) => {
            h112
        }
    }, match v0.clone() {
        Adt1::C0(_, h113, _, _, _, _, _, _) => {
            h113
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h114, _, _, _, _, _) => {
            h114
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h115, _, _, _, _) => {
            h115
        }
    }, 4u64, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h116, _, _) => {
            h116
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h117, _) => {
            h117
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, h118) => {
            h118
        }
    })
}

pub fn f10(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a119 = v4;
    let a120 = 0u64;
    if nat_eq(a119, a120) {
        Ok(0u64)
    } else {
        let a127 = 2u64;
        let a128 = {
            let a125 = {
                let a121 = v0;
                let a122 = v1;
                nat_mul(a121, a122)?
            };
            let a126 = {
                let a123 = v2;
                let a124 = v3;
                nat_mul(a123, a124)?
            };
            nat_mul(a125, a126)?
        };
        nat_mul(a127, a128)
    }
}

pub fn f11(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a129 = v0;
    let a130 = 0u64;
    if if if if nat_eq(a129, a130) {
        true
    } else {
        let a131 = v1;
        let a132 = 0u64;
        nat_eq(a131, a132)
    } {
        true
    } else {
        let a133 = v2;
        let a134 = 0u64;
        nat_eq(a133, a134)
    } {
        true
    } else {
        let a135 = v3;
        let a136 = 0u64;
        nat_eq(a135, a136)
    } {
        Ok(0u64)
    } else {
        let a137 = f10(v0, v1, v2, v3, v5)?;
        let a138 = f22(v0, v1, v2, v3, v4)?;
        nat_add(a137, a138)
    }
}

pub fn f12(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt2> {
    let v6: u64 = f23(v3, v5)?;
    let v7: u64 = f24(v1, v2, v6)?;
    let v8: u64 = {
        let a141 = {
            let a139 = f25(v3, v5)?;
            let a140 = v4;
            nat_mul(a139, a140)?
        };
        let a142 = v7;
        nat_mul(a141, a142)?
    };
    let a143 = v0;
    let a144 = v8;
    if nat_lt(a143, a144) {
        let v9: u64 = {
            let a145 = v0;
            let a146 = v7;
            let a147 = 0u64;
            nat_quot(a145, a146, a147)
        };
        Ok(Adt2::C0({
            let a151 = {
                let a148 = v9;
                let a149 = v4;
                let a150 = 0u64;
                nat_quot(a148, a149, a150)
            };
            let a152 = v6;
            nat_mul(a151, a152)?
        }, v6, {
            let a153 = v9;
            let a154 = v4;
            let a155 = 0u64;
            nat_rem(a153, a154, a155)
        }, {
            let a156 = v0;
            let a157 = v7;
            let a158 = 0u64;
            nat_rem(a156, a157, a158)
        }))
    } else {
        let v10: u64 = {
            let a159 = v0;
            let a160 = v8;
            nat_sub(a159, a160)
        };
        let v11: u64 = f24(v1, v2, f26(v3, v5)?)?;
        Ok(Adt2::C0({
            let a161 = f25(v3, v5)?;
            let a162 = v6;
            nat_mul(a161, a162)?
        }, f26(v3, v5)?, {
            let a163 = v10;
            let a164 = v11;
            let a165 = 0u64;
            nat_quot(a163, a164, a165)
        }, {
            let a166 = v10;
            let a167 = v11;
            let a168 = 0u64;
            nat_rem(a166, a167, a168)
        }))
    }
}

pub fn f13(v0: Adt2, v1: Adt1, v2: Adt1, v3: u64) -> R<Adt0> {
    let v4: u64 = match v0.clone() {
        Adt2::C0(_, _, _, h169) => {
            h169
        }
    };
    let v5: u64 = match v1.clone() {
        Adt1::C0(h170, _, _, _, _, _, _, _) => {
            h170
        }
    };
    let v6: u64 = {
        let a173 = match v0.clone() {
            Adt2::C0(_, h171, _, _) => {
                h171
            }
        };
        let a174 = match v1.clone() {
            Adt1::C0(_, h172, _, _, _, _, _, _) => {
                h172
            }
        };
        nat_mul(a173, a174)?
    };
    let v7: u64 = {
        let a175 = v6;
        let a176 = v5;
        nat_mul(a175, a176)?
    };
    let v8: u64 = {
        let a179 = {
            let a177 = 2u64;
            let a178 = v5;
            nat_mul(a177, a178)?
        };
        let a180 = 2u64;
        nat_add(a179, a180)?
    };
    let v9: u64 = {
        let a183 = {
            let a181 = 4u64;
            let a182 = v5;
            nat_mul(a181, a182)?
        };
        let a184 = 2u64;
        nat_add(a183, a184)?
    };
    let v10: u64 = {
        let a187 = 1u64;
        let a188 = {
            let a185 = v6;
            let a186 = v8;
            nat_mul(a185, a186)?
        };
        nat_add(a187, a188)?
    };
    let v11: u64 = {
        let a189 = v10;
        let a190 = 4u64;
        nat_add(a189, a190)?
    };
    let v12: u64 = {
        let a193 = v11;
        let a194 = {
            let a191 = v6;
            let a192 = v9;
            nat_mul(a191, a192)?
        };
        nat_add(a193, a194)?
    };
    let v13: u64 = {
        let a195 = v12;
        let a196 = 6u64;
        nat_add(a195, a196)?
    };
    let a197 = v4;
    let a198 = 0u64;
    if nat_eq(a197, a198) {
        Ok(f27(0u64))
    } else {
        let a199 = v4;
        let a200 = v10;
        if nat_lt(a199, a200) {
            let v14: u64 = {
                let a201 = v4;
                let a202 = 1u64;
                nat_sub(a201, a202)
            };
            f28(false, {
                let a203 = v14;
                let a204 = v8;
                let a205 = 0u64;
                nat_rem(a203, a204, a205)
            }, {
                let a206 = v14;
                let a207 = v8;
                let a208 = 0u64;
                nat_quot(a206, a207, a208)
            }, v0, v1, v2)
        } else {
            let a209 = v4;
            let a210 = v11;
            if nat_lt(a209, a210) {
                Ok(f29({
                    let a211 = v4;
                    let a212 = v10;
                    nat_sub(a211, a212)
                }, v7))
            } else {
                let a213 = v4;
                let a214 = v12;
                if nat_lt(a213, a214) {
                    let v15: u64 = {
                        let a215 = v4;
                        let a216 = v11;
                        nat_sub(a215, a216)
                    };
                    f28(true, {
                        let a217 = v15;
                        let a218 = v9;
                        let a219 = 0u64;
                        nat_rem(a217, a218, a219)
                    }, {
                        let a220 = v15;
                        let a221 = v9;
                        let a222 = 0u64;
                        nat_quot(a220, a221, a222)
                    }, v0, v1, v2)
                } else {
                    let a223 = v4;
                    let a224 = v13;
                    if nat_lt(a223, a224) {
                        Ok(f30({
                            let a225 = v4;
                            let a226 = v12;
                            nat_sub(a225, a226)
                        }, v7, v3))
                    } else {
                        let a229 = v4;
                        let a230 = {
                            let a227 = v13;
                            let a228 = v7;
                            nat_add(a227, a228)?
                        };
                        if nat_lt(a229, a230) {
                            let v16: u64 = {
                                let a231 = v4;
                                let a232 = v13;
                                nat_sub(a231, a232)
                            };
                            let v17: u64 = {
                                let a233 = v16;
                                let a234 = v5;
                                let a235 = 0u64;
                                nat_quot(a233, a234, a235)
                            };
                            let v18: u64 = {
                                let a241 = {
                                    let a236 = f31(v17, v0, v1)?;
                                    let a237 = v5;
                                    nat_mul(a236, a237)?
                                };
                                let a242 = {
                                    let a238 = v16;
                                    let a239 = v5;
                                    let a240 = 0u64;
                                    nat_rem(a238, a239, a240)
                                };
                                nat_add(a241, a242)?
                            };
                            let v19: u64 = f14(v18, v2)?;
                            Ok(f15(10u64, 3u64, v19, 1u64, 1u64, 3u64, v19))
                        } else {
                            Ok(Adt0::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f14(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a244 = v0;
        let a245 = match v1.clone() {
            Adt1::C0(h243, _, _, _, _, _, _, _) => {
                h243
            }
        };
        let a246 = 0u64;
        nat_quot(a244, a245, a246)
    };
    let v3: u64 = {
        let a248 = v2;
        let a249 = match v1.clone() {
            Adt1::C0(_, h247, _, _, _, _, _, _) => {
                h247
            }
        };
        let a250 = 0u64;
        nat_quot(a248, a249, a250)
    };
    f32({
        let a252 = v0;
        let a253 = match v1.clone() {
            Adt1::C0(h251, _, _, _, _, _, _, _) => {
                h251
            }
        };
        let a254 = 0u64;
        nat_rem(a252, a253, a254)
    }, {
        let a256 = v2;
        let a257 = match v1.clone() {
            Adt1::C0(_, h255, _, _, _, _, _, _) => {
                h255
            }
        };
        let a258 = 0u64;
        nat_rem(a256, a257, a258)
    }, {
        let a260 = v3;
        let a261 = match v1.clone() {
            Adt1::C0(_, _, h259, _, _, _, _, _) => {
                h259
            }
        };
        let a262 = 0u64;
        nat_rem(a260, a261, a262)
    }, {
        let a264 = v3;
        let a265 = match v1.clone() {
            Adt1::C0(_, _, h263, _, _, _, _, _) => {
                h263
            }
        };
        let a266 = 0u64;
        nat_quot(a264, a265, a266)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, h267, _, _, _) => {
            h267
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h268, _, _) => {
            h268
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h269, _) => {
            h269
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, h270) => {
            h270
        }
    })
}

pub fn f15(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> Adt0 {
    let a271 = v0;
    let a272 = 4u64;
    if if if nat_le(a271, a272) {
        true
    } else {
        let a273 = 9u64;
        let a274 = v0;
        if nat_le(a273, a274) {
            let a275 = v0;
            let a276 = 12u64;
            nat_le(a275, a276)
        } else {
            false
        }
    } {
        true
    } else {
        let a277 = v0;
        let a278 = 16u64;
        if if nat_eq(a277, a278) {
            true
        } else {
            let a279 = v0;
            let a280 = 19u64;
            nat_eq(a279, a280)
        } {
            true
        } else {
            let a281 = v0;
            let a282 = 21u64;
            if nat_eq(a281, a282) {
                true
            } else {
                let a283 = v0;
                let a284 = 22u64;
                nat_eq(a283, a284)
            }
        }
    } {
        Adt0::C1({
            let a285 = v0;
            let a286 = 4u64;
            if if nat_lt(a285, a286) {
                true
            } else {
                let a287 = 21u64;
                let a288 = v0;
                nat_le(a287, a288)
            } {
                0u64
            } else {
                let a289 = v0;
                let a290 = 16u64;
                if if nat_eq(a289, a290) {
                    true
                } else {
                    let a291 = v0;
                    let a292 = 19u64;
                    nat_eq(a291, a292)
                } {
                    2u64
                } else {
                    1u64
                }
            }
        }, {
            let a293 = v0;
            let a294 = 4u64;
            if nat_lt(a293, a294) {
                v0
            } else {
                let a295 = v0;
                let a296 = 4u64;
                if if nat_eq(a295, a296) {
                    true
                } else {
                    let a297 = v0;
                    let a298 = 16u64;
                    nat_eq(a297, a298)
                } {
                    0u64
                } else {
                    let a299 = v0;
                    let a300 = 19u64;
                    if nat_eq(a299, a300) {
                        3u64
                    } else {
                        let a301 = 21u64;
                        let a302 = v0;
                        if nat_le(a301, a302) {
                            let a303 = v0;
                            let a304 = 17u64;
                            nat_sub(a303, a304)
                        } else {
                            let a305 = v0;
                            let a306 = 8u64;
                            nat_sub(a305, a306)
                        }
                    }
                }
            }
        }, v1, v2, v3, v4, v5, v6)
    } else {
        Adt0::C0
    }
}

pub fn f16(v0: bool, v1: u64) -> R<u64> {
    let a309 = if v0 {
        5u64
    } else {
        4u64
    };
    let a310 = {
        let a307 = f33(v0);
        let a308 = v1;
        nat_mul(a307, a308)?
    };
    nat_add(a309, a310)
}

pub fn f17(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a318 = {
            let a311 = 13u64;
            let a312 = f34(v1);
            nat_mul(a311, a312)?
        };
        let a319 = {
            let a316 = 4u64;
            let a317 = {
                let a313 = v1;
                let a314 = 4u64;
                let a315 = 0u64;
                nat_rem(a313, a314, a315)
            };
            nat_mul(a316, a317)?
        };
        nat_add(a318, a319)
    } else {
        let a320 = 3u64;
        let a321 = v1;
        nat_mul(a320, a321)
    }
}

pub fn f18(v0: bool) -> u64 {
    if v0 {
        6u64
    } else {
        5u64
    }
}

pub fn f19(v0: u64, v1: u64, v2: Adt1, v3: bool) -> R<Adt0> {
    let v4: u64 = match v2.clone() {
        Adt1::C0(h322, _, _, _, _, _, _, _) => {
            h322
        }
    };
    let v5: u64 = {
        let a325 = 1u64;
        let a326 = {
            let a323 = v4;
            let a324 = f33(v3);
            nat_mul(a323, a324)?
        };
        nat_add(a325, a326)?
    };
    let a327 = v0;
    let a328 = 0u64;
    if nat_eq(a327, a328) {
        Ok(f27(0u64))
    } else {
        let a329 = v0;
        let a330 = v5;
        if nat_lt(a329, a330) {
            let v6: u64 = {
                let a331 = v0;
                let a332 = 1u64;
                nat_sub(a331, a332)
            };
            let v7: u64 = f14({
                let a338 = {
                    let a333 = v1;
                    let a334 = v4;
                    nat_mul(a333, a334)?
                };
                let a339 = {
                    let a335 = v6;
                    let a336 = f33(v3);
                    let a337 = 0u64;
                    nat_quot(a335, a336, a337)
                };
                nat_add(a338, a339)?
            }, v2)?;
            if v3 {
                let a343 = {
                    let a340 = v6;
                    let a341 = 2u64;
                    let a342 = 0u64;
                    nat_rem(a340, a341, a342)
                };
                let a344 = 0u64;
                if nat_eq(a343, a344) {
                    Ok(f15(1u64, 0u64, v7, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f15(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            } else {
                Ok(f15(4u64, 1u64, 0u64, 0u64, v7, 1u64, 0u64))
            }
        } else {
            let v8: u64 = {
                let a345 = v0;
                let a346 = v5;
                nat_sub(a345, a346)
            };
            let a347 = v8;
            let a348 = 0u64;
            if nat_eq(a347, a348) {
                Ok(f15(3u64, 2u64, v4, 2u64, 0u64, 1u64, 1u64))
            } else {
                if if v3 {
                    let a349 = v8;
                    let a350 = 1u64;
                    nat_eq(a349, a350)
                } else {
                    false
                } {
                    Ok(f15(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a351 = v8;
                    let a352 = if v3 {
                        2u64
                    } else {
                        1u64
                    };
                    if nat_eq(a351, a352) {
                        Ok(f15(11u64, 1u64, 0u64, 1u64, 1u64, 1u64, 3u64))
                    } else {
                        let a353 = v8;
                        let a354 = if v3 {
                            3u64
                        } else {
                            2u64
                        };
                        if nat_eq(a353, a354) {
                            Ok(f27(0u64))
                        } else {
                            Ok(Adt0::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f20(v0: u64, v1: u64, v2: Adt1, v3: Adt1, v4: bool) -> R<Adt0> {
    let v5: u64 = if v4 {
        let a356 = 13u64;
        let a357 = f34(match v2.clone() {
            Adt1::C0(h355, _, _, _, _, _, _, _) => {
                h355
            }
        });
        nat_mul(a356, a357)?
    } else {
        0u64
    };
    let a358 = v0;
    let a359 = v5;
    if nat_lt(a358, a359) {
        f35({
            let a360 = v0;
            let a361 = 13u64;
            let a362 = 0u64;
            nat_rem(a360, a361, a362)
        }, v1, {
            let a363 = v0;
            let a364 = 13u64;
            let a365 = 0u64;
            nat_quot(a363, a364, a365)
        }, v2, v3)
    } else {
        let v6: u64 = {
            let a366 = v0;
            let a367 = v5;
            nat_sub(a366, a367)
        };
        let v7: u64 = if v4 {
            4u64
        } else {
            3u64
        };
        f36({
            let a368 = v6;
            let a369 = v7;
            let a370 = 0u64;
            nat_rem(a368, a369, a370)
        }, {
            let a380 = {
                let a375 = {
                    let a372 = v1;
                    let a373 = match v2.clone() {
                        Adt1::C0(h371, _, _, _, _, _, _, _) => {
                            h371
                        }
                    };
                    nat_mul(a372, a373)?
                };
                let a376 = if v4 {
                    f37(match v2.clone() {
                        Adt1::C0(h374, _, _, _, _, _, _, _) => {
                            h374
                        }
                    })?
                } else {
                    0u64
                };
                nat_add(a375, a376)?
            };
            let a381 = {
                let a377 = v6;
                let a378 = v7;
                let a379 = 0u64;
                nat_quot(a377, a378, a379)
            };
            nat_add(a380, a381)?
        }, v2, v3, v4)
    }
}

pub fn f21(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt0 {
    let a382 = v0;
    let a383 = 0u64;
    if nat_eq(a382, a383) {
        f15(if v3 {
            22u64
        } else {
            3u64
        }, 2u64, v1, 2u64, 0u64, 1u64, 1u64)
    } else {
        let a384 = v0;
        let a385 = 1u64;
        if nat_eq(a384, a385) {
            f15(if v3 {
                21u64
            } else {
                11u64
            }, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64)
        } else {
            if if v3 {
                let a386 = v0;
                let a387 = 2u64;
                nat_eq(a386, a387)
            } else {
                false
            } {
                f15(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
            } else {
                let a388 = v0;
                let a389 = if v3 {
                    3u64
                } else {
                    2u64
                };
                if nat_eq(a388, a389) {
                    f15(4u64, 1u64, 1u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a390 = v0;
                    let a391 = if v3 {
                        4u64
                    } else {
                        3u64
                    };
                    if nat_eq(a390, a391) {
                        f15(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a392 = v0;
                        let a393 = if v3 {
                            5u64
                        } else {
                            4u64
                        };
                        if nat_eq(a392, a393) {
                            f15(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt0::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f22(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a398 = v3;
    let a399 = {
        let a396 = {
            let a394 = f25(v2, v4)?;
            let a395 = f24(v0, v1, f23(v2, v4)?)?;
            nat_mul(a394, a395)?
        };
        let a397 = f24(v0, v1, f26(v2, v4)?)?;
        nat_add(a396, a397)?
    };
    nat_mul(a398, a399)
}

pub fn f23(v0: u64, v1: u64) -> R<u64> {
    let a404 = {
        let a402 = {
            let a400 = v0;
            let a401 = v1;
            nat_add(a400, a401)?
        };
        let a403 = 1u64;
        nat_sub(a402, a403)
    };
    let a405 = v1;
    let a406 = 0u64;
    Ok(nat_quot(a404, a405, a406))
}

pub fn f24(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a407 = v2;
    let a408 = 0u64;
    if nat_eq(a407, a408) {
        Ok(0u64)
    } else {
        let a413 = 11u64;
        let a414 = {
            let a411 = {
                let a409 = v1;
                let a410 = v2;
                nat_mul(a409, a410)?
            };
            let a412 = f38(v0)?;
            nat_mul(a411, a412)?
        };
        nat_add(a413, a414)
    }
}

pub fn f25(v0: u64, v1: u64) -> R<u64> {
    let a415 = v0;
    let a416 = f23(v0, v1)?;
    let a417 = 0u64;
    Ok(nat_quot(a415, a416, a417))
}

pub fn f26(v0: u64, v1: u64) -> R<u64> {
    let a418 = v0;
    let a419 = f23(v0, v1)?;
    let a420 = 0u64;
    Ok(nat_rem(a418, a419, a420))
}

pub fn f27(v0: u64) -> Adt0 {
    f15(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f28(v0: bool, v1: u64, v2: u64, v3: Adt2, v4: Adt1, v5: Adt1) -> R<Adt0> {
    let v6: u64 = if v0 {
        4u64
    } else {
        2u64
    };
    let v7: u64 = {
        let a424 = 1u64;
        let a425 = {
            let a422 = match v4.clone() {
                Adt1::C0(h421, _, _, _, _, _, _, _) => {
                    h421
                }
            };
            let a423 = v6;
            nat_mul(a422, a423)?
        };
        nat_add(a424, a425)?
    };
    let a426 = v1;
    let a427 = 0u64;
    if nat_eq(a426, a427) {
        Ok(f27(1u64))
    } else {
        let a428 = v1;
        let a429 = v7;
        if nat_lt(a428, a429) {
            let v8: u64 = {
                let a430 = v1;
                let a431 = 1u64;
                nat_sub(a430, a431)
            };
            let v9: u64 = {
                let a438 = {
                    let a433 = f31(v2, v3, v4.clone())?;
                    let a434 = match v4.clone() {
                        Adt1::C0(h432, _, _, _, _, _, _, _) => {
                            h432
                        }
                    };
                    nat_mul(a433, a434)?
                };
                let a439 = {
                    let a435 = v8;
                    let a436 = v6;
                    let a437 = 0u64;
                    nat_quot(a435, a436, a437)
                };
                nat_add(a438, a439)?
            };
            let v10: u64 = f14(v9, v4)?;
            let v11: u64 = f14(v9, v5)?;
            let v12: u64 = {
                let a440 = v8;
                let a441 = v6;
                let a442 = 0u64;
                nat_rem(a440, a441, a442)
            };
            if v0 {
                let a443 = v12;
                let a444 = 0u64;
                if nat_eq(a443, a444) {
                    Ok(f15(9u64, 0u64, v10, 1u64, 3u64, 3u64, v11))
                } else {
                    let a445 = v12;
                    let a446 = 1u64;
                    if nat_eq(a445, a446) {
                        Ok(f15(10u64, 3u64, v11, 3u64, v11, 1u64, 2u64))
                    } else {
                        let a447 = v12;
                        let a448 = 2u64;
                        if nat_eq(a447, a448) {
                            Ok(f15(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                        } else {
                            Ok(f15(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                        }
                    }
                }
            } else {
                let a449 = v12;
                let a450 = 0u64;
                if nat_eq(a449, a450) {
                    Ok(f15(1u64, 0u64, v10, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(f15(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                }
            }
        } else {
            let a451 = v1;
            let a452 = v7;
            if nat_eq(a451, a452) {
                Ok(f15(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                Ok(Adt0::C0)
            }
        }
    }
}

pub fn f29(v0: u64, v1: u64) -> Adt0 {
    let a453 = v0;
    let a454 = 0u64;
    if nat_eq(a453, a454) {
        f15(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a455 = v0;
        let a456 = 1u64;
        if nat_eq(a455, a456) {
            f15(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 3u64)
        } else {
            let a457 = v0;
            let a458 = 2u64;
            if nat_eq(a457, a458) {
                f15(2u64, 1u64, 3u64, 2u64, 0u64, 1u64, 3u64)
            } else {
                let a459 = v0;
                let a460 = 3u64;
                if nat_eq(a459, a460) {
                    f27(0u64)
                } else {
                    Adt0::C0
                }
            }
        }
    }
}

pub fn f30(v0: u64, v1: u64, v2: u64) -> Adt0 {
    let a461 = v0;
    let a462 = 0u64;
    if nat_eq(a461, a462) {
        f15(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a463 = v0;
        let a464 = 1u64;
        if nat_eq(a463, a464) {
            f15(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 2u64)
        } else {
            let a465 = v0;
            let a466 = 2u64;
            if nat_eq(a465, a466) {
                f15(2u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64)
            } else {
                let a467 = v0;
                let a468 = 3u64;
                if nat_eq(a467, a468) {
                    f15(4u64, 1u64, 2u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a469 = v0;
                    let a470 = 4u64;
                    if nat_eq(a469, a470) {
                        f15(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a471 = v0;
                        let a472 = 5u64;
                        if nat_eq(a471, a472) {
                            f15(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt0::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f31(v0: u64, v1: Adt2, v2: Adt1) -> R<u64> {
    let a483 = {
        let a481 = {
            let a478 = {
                let a475 = match v1.clone() {
                    Adt2::C0(_, _, h473, _) => {
                        h473
                    }
                };
                let a476 = match v2.clone() {
                    Adt1::C0(_, _, h474, _, _, _, _, _) => {
                        h474
                    }
                };
                nat_mul(a475, a476)?
            };
            let a479 = match v1 {
                Adt2::C0(h477, _, _, _) => {
                    h477
                }
            };
            nat_add(a478, a479)?
        };
        let a482 = match v2 {
            Adt1::C0(_, h480, _, _, _, _, _, _) => {
                h480
            }
        };
        nat_mul(a481, a482)?
    };
    let a484 = v0;
    nat_add(a483, a484)
}

pub fn f32(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a497 = {
        let a493 = {
            let a489 = {
                let a485 = v0;
                let a486 = v4;
                nat_mul(a485, a486)?
            };
            let a490 = {
                let a487 = v1;
                let a488 = v5;
                nat_mul(a487, a488)?
            };
            nat_add(a489, a490)?
        };
        let a494 = {
            let a491 = v2;
            let a492 = v6;
            nat_mul(a491, a492)?
        };
        nat_add(a493, a494)?
    };
    let a498 = {
        let a495 = v3;
        let a496 = v7;
        nat_mul(a495, a496)?
    };
    nat_add(a497, a498)
}

pub fn f33(v0: bool) -> u64 {
    if v0 {
        2u64
    } else {
        1u64
    }
}

pub fn f34(v0: u64) -> u64 {
    let a499 = v0;
    let a500 = 4u64;
    let a501 = 0u64;
    nat_quot(a499, a500, a501)
}

pub fn f35(v0: u64, v1: u64, v2: u64, v3: Adt1, v4: Adt1) -> R<Adt0> {
    let v5: u64 = {
        let a509 = {
            let a507 = {
                let a503 = v1;
                let a504 = match v3.clone() {
                    Adt1::C0(h502, _, _, _, _, _, _, _) => {
                        h502
                    }
                };
                nat_mul(a503, a504)?
            };
            let a508 = {
                let a505 = v2;
                let a506 = 4u64;
                nat_mul(a505, a506)?
            };
            nat_add(a507, a508)?
        };
        let a510 = f39(v0);
        nat_add(a509, a510)?
    };
    let v6: u64 = f14(v5, v3)?;
    let v7: u64 = f14(v5, v4)?;
    let a511 = v0;
    let a512 = 0u64;
    if if if if nat_eq(a511, a512) {
        true
    } else {
        let a513 = v0;
        let a514 = 2u64;
        nat_eq(a513, a514)
    } {
        true
    } else {
        let a515 = v0;
        let a516 = 5u64;
        nat_eq(a515, a516)
    } {
        true
    } else {
        let a517 = v0;
        let a518 = 7u64;
        nat_eq(a517, a518)
    } {
        Ok(f15(9u64, 0u64, v6, 1u64, 3u64, 3u64, v7))
    } else {
        let a519 = v0;
        let a520 = 1u64;
        if if if if nat_eq(a519, a520) {
            true
        } else {
            let a521 = v0;
            let a522 = 3u64;
            nat_eq(a521, a522)
        } {
            true
        } else {
            let a523 = v0;
            let a524 = 6u64;
            nat_eq(a523, a524)
        } {
            true
        } else {
            let a525 = v0;
            let a526 = 8u64;
            nat_eq(a525, a526)
        } {
            Ok(f15(10u64, 3u64, v7, 3u64, v7, 1u64, {
                let a527 = v0;
                let a528 = 1u64;
                if nat_eq(a527, a528) {
                    4u64
                } else {
                    let a529 = v0;
                    let a530 = 6u64;
                    if nat_eq(a529, a530) {
                        5u64
                    } else {
                        1u64
                    }
                }
            }))
        } else {
            let a531 = v0;
            let a532 = 4u64;
            if nat_eq(a531, a532) {
                Ok(f15(4u64, 1u64, 4u64, 1u64, 1u64, 1u64, 4u64))
            } else {
                let a533 = v0;
                let a534 = 9u64;
                if nat_eq(a533, a534) {
                    Ok(f15(4u64, 1u64, 5u64, 1u64, 1u64, 1u64, 5u64))
                } else {
                    let a535 = v0;
                    let a536 = 10u64;
                    if nat_eq(a535, a536) {
                        Ok(f15(4u64, 1u64, 4u64, 1u64, 5u64, 1u64, 1u64))
                    } else {
                        let a537 = v0;
                        let a538 = 11u64;
                        if nat_eq(a537, a538) {
                            Ok(f15(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            let a539 = v0;
                            let a540 = 12u64;
                            if nat_eq(a539, a540) {
                                Ok(f15(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
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

pub fn f36(v0: u64, v1: u64, v2: Adt1, v3: Adt1, v4: bool) -> R<Adt0> {
    let v5: u64 = f14(v1, v2)?;
    let v6: u64 = f14(v1, v3)?;
    let a541 = v0;
    let a542 = 0u64;
    if nat_eq(a541, a542) {
        Ok(f15(9u64, 0u64, v5, 1u64, 3u64, 3u64, v6))
    } else {
        let a543 = v0;
        let a544 = 1u64;
        if nat_eq(a543, a544) {
            Ok(f15(10u64, 3u64, v6, 3u64, v6, 1u64, 1u64))
        } else {
            if v4 {
                let a545 = v0;
                let a546 = 2u64;
                if nat_eq(a545, a546) {
                    Ok(f15(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a547 = v0;
                    let a548 = 3u64;
                    if nat_eq(a547, a548) {
                        Ok(f15(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    } else {
                        Ok(Adt0::C0)
                    }
                }
            } else {
                let a549 = v0;
                let a550 = 2u64;
                if nat_eq(a549, a550) {
                    Ok(f15(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    Ok(Adt0::C0)
                }
            }
        }
    }
}

pub fn f37(v0: u64) -> R<u64> {
    let a551 = 4u64;
    let a552 = f34(v0);
    nat_mul(a551, a552)
}

pub fn f38(v0: u64) -> R<u64> {
    let a555 = {
        let a553 = 7u64;
        let a554 = v0;
        nat_mul(a553, a554)?
    };
    let a556 = 4u64;
    nat_add(a555, a556)
}

pub fn f39(v0: u64) -> u64 {
    let a557 = v0;
    let a558 = 5u64;
    if nat_lt(a557, a558) {
        let a559 = v0;
        let a560 = 2u64;
        if nat_lt(a559, a560) {
            0u64
        } else {
            1u64
        }
    } else {
        let a561 = v0;
        let a562 = 7u64;
        if nat_lt(a561, a562) {
            2u64
        } else {
            3u64
        }
    }
}
