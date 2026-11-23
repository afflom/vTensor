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
    C0(u64, bool, u64, u64, u64, Adt1, Adt1, Adt1, Option<Adt1>, Option<Adt1>, Adt1),
    C1(Adt2, u64, u64, u64),
    C2(u64, bool, u64, u64, Adt1, Option<Adt1>, Option<Adt1>, Adt1),
    C3(Adt3, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Adt1, Adt1, Adt1, Option<Adt1>, Option<Adt1>, Adt1, Option<u64>, Option<u64>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(Adt1, Option<Adt1>, Adt1, u64, Option<u64>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0,
    C1(Adt2),
    C2(u64, u64, u64, u64, u64, u64, u64, u64, u64),
    C3(Adt3),
    C4(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt5 {
    C0,
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(u64, u64, u64),
}

pub fn f0(v0: Adt0) -> R<Adt4> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11) => {
            f1(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11)
        }
        Adt0::C1(v24, v25, v26, v27) => {
            f4(v24, v25, v26, v27)
        }
        Adt0::C2(v12, v13, v14, v15, v16, v17, v18, v19) => {
            f2(v12, v13, v14, v15, v16, v17, v18, v19)
        }
        Adt0::C3(v20, v21, v22, v23) => {
            f3(v20, v21, v22, v23)
        }
    }
}

pub fn f1(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1, v8: Option<Adt1>, v9: Option<Adt1>, v10: Adt1) -> R<Adt4> {
    let a2 = v0;
    let a3 = 71u64;
    if if if if if if if if if if if if if if if if if if nat_eq(a2, a3) {
        !v1
    } else {
        false
    } {
        f5(v2, match v5.clone() {
            Adt1::C0(_, _, _, h4, _, _, _, _, _, _, _) => {
                h4
            }
        }, v8.clone())
    } else {
        false
    } {
        let a5 = v4;
        let a6 = 0u64;
        if if nat_eq(a5, a6) {
            true
        } else {
            let a7 = v4;
            let a8 = 10u64;
            nat_eq(a7, a8)
        } {
            true
        } else {
            let a9 = v4;
            let a10 = 15u64;
            nat_eq(a9, a10)
        }
    } else {
        false
    } {
        let a12 = match v5.clone() {
            Adt1::C0(h11, _, _, _, _, _, _, _, _, _, _) => {
                h11
            }
        };
        let a13 = 0u64;
        nat_eq(a12, a13)
    } else {
        false
    } {
        let a15 = match v5.clone() {
            Adt1::C0(_, _, _, _, _, h14, _, _, _, _, _) => {
                h14
            }
        };
        let a16 = 4u64;
        nat_eq(a15, a16)
    } else {
        false
    } {
        f6(v10.clone())?
    } else {
        false
    } {
        f7(v6.clone(), v5.clone(), v10.clone())?
    } else {
        false
    } {
        f7(v7.clone(), v5.clone(), v10.clone())?
    } else {
        false
    } {
        let a19 = match v5.clone() {
            Adt1::C0(_, h17, _, _, _, _, _, _, _, _, _) => {
                h17
            }
        };
        let a20 = match v6.clone() {
            Adt1::C0(_, h18, _, _, _, _, _, _, _, _, _) => {
                h18
            }
        };
        nat_eq(a19, a20)
    } else {
        false
    } {
        let a23 = match v6.clone() {
            Adt1::C0(_, _, h21, _, _, _, _, _, _, _, _) => {
                h21
            }
        };
        let a24 = match v7.clone() {
            Adt1::C0(_, _, h22, _, _, _, _, _, _, _, _) => {
                h22
            }
        };
        nat_eq(a23, a24)
    } else {
        false
    } {
        let a27 = match v10.clone() {
            Adt1::C0(_, h25, _, _, _, _, _, _, _, _, _) => {
                h25
            }
        };
        let a28 = match v7.clone() {
            Adt1::C0(_, h26, _, _, _, _, _, _, _, _, _) => {
                h26
            }
        };
        nat_eq(a27, a28)
    } else {
        false
    } {
        let a31 = match v10.clone() {
            Adt1::C0(_, _, h29, _, _, _, _, _, _, _, _) => {
                h29
            }
        };
        let a32 = match v5.clone() {
            Adt1::C0(_, _, _, h30, _, _, _, _, _, _, _) => {
                h30
            }
        };
        nat_eq(a31, a32)
    } else {
        false
    } {
        let a35 = match v10.clone() {
            Adt1::C0(_, _, _, h33, _, _, _, _, _, _, _) => {
                h33
            }
        };
        let a36 = match v5.clone() {
            Adt1::C0(_, _, h34, _, _, _, _, _, _, _, _) => {
                h34
            }
        };
        nat_eq(a35, a36)
    } else {
        false
    } {
        let a39 = match v10.clone() {
            Adt1::C0(_, _, _, _, h37, _, _, _, _, _, _) => {
                h37
            }
        };
        let a40 = match v5.clone() {
            Adt1::C0(_, _, _, _, h38, _, _, _, _, _, _) => {
                h38
            }
        };
        nat_eq(a39, a40)
    } else {
        false
    } {
        f8(v5.clone(), v10.clone())?
    } else {
        false
    } {
        f9(v8.clone(), v5.clone(), v6.clone(), v10.clone())?
    } else {
        false
    } {
        f10(v9.clone(), v5.clone(), v10.clone())?
    } else {
        false
    } {
        Ok(Adt4::C1(Adt2::C0(v5, v6, v7, v8, v9, v10, f11(v2), f12(v3))))
    } else {
        Ok(Adt4::C0)
    }
}

pub fn f2(v0: u64, v1: bool, v2: u64, v3: u64, v4: Adt1, v5: Option<Adt1>, v6: Option<Adt1>, v7: Adt1) -> R<Adt4> {
    if v1 {
        Ok(Adt4::C0)
    } else {
        let a41 = v0;
        let a42 = 43u64;
        if if if if if if if if nat_eq(a41, a42) {
            let a44 = match v4.clone() {
                Adt1::C0(h43, _, _, _, _, _, _, _, _, _, _) => {
                    h43
                }
            };
            let a45 = 0u64;
            if nat_eq(a44, a45) {
                f13(v4.clone(), 4u64)?
            } else {
                false
            }
        } else {
            false
        } {
            let a47 = match v7.clone() {
                Adt1::C0(h46, _, _, _, _, _, _, _, _, _, _) => {
                    h46
                }
            };
            let a48 = 0u64;
            if nat_eq(a47, a48) {
                f13(v7.clone(), 4u64)?
            } else {
                false
            }
        } else {
            false
        } {
            f14(v4.clone(), v7.clone())
        } else {
            false
        } {
            if f8(v4.clone(), v7.clone())? {
                true
            } else {
                f15(v4.clone(), v7.clone())
            }
        } else {
            false
        } {
            f16(v5.clone(), v4.clone(), v7.clone())?
        } else {
            false
        } {
            f17(v6, v4.clone(), v7.clone())?
        } else {
            false
        } {
            f5(v3, match v4.clone() {
                Adt1::C0(_, _, _, h49, _, _, _, _, _, _, _) => {
                    h49
                }
            }, v5.clone())
        } else {
            false
        } {
            Ok(Adt4::C3(Adt3::C0(v4, v5, v7, v2, f11(v3))))
        } else {
            Ok(Adt4::C0)
        }
    }
}

pub fn f3(v0: Adt3, v1: u64, v2: u64, v3: u64) -> R<Adt4> {
    let v4: Adt1 = match v0.clone() {
        Adt3::C0(h50, _, _, _, _) => {
            h50
        }
    };
    let a52 = v1;
    let a53 = match v4.clone() {
        Adt1::C0(_, _, h51, _, _, _, _, _, _, _, _) => {
            h51
        }
    };
    if if if nat_lt(a52, a53) {
        let a55 = v2;
        let a56 = match v4.clone() {
            Adt1::C0(_, _, _, h54, _, _, _, _, _, _, _) => {
                h54
            }
        };
        nat_lt(a55, a56)
    } else {
        false
    } {
        let a58 = v3;
        let a59 = match v4.clone() {
            Adt1::C0(_, _, _, _, h57, _, _, _, _, _, _) => {
                h57
            }
        };
        nat_lt(a58, a59)
    } else {
        false
    } {
        let v6: u64 = {
            let m61 = match v0.clone() {
                Adt3::C0(_, h60, _, _, _) => {
                    h60
                }
            };
            match m61 {
                None => {
                    0u64
                }
                Some(v5) => {
                    f18(v5.clone(), v1, {
                        let a63 = v2;
                        let a64 = match v5.clone() {
                            Adt1::C0(_, _, _, h62, _, _, _, _, _, _, _) => {
                                h62
                            }
                        };
                        let a65 = 0u64;
                        nat_rem(a63, a64, a65)
                    }, {
                        let a67 = v3;
                        let a68 = match v5 {
                            Adt1::C0(_, _, _, _, h66, _, _, _, _, _, _) => {
                                h66
                            }
                        };
                        let a69 = 0u64;
                        nat_rem(a67, a68, a69)
                    })?
                }
            }
        };
        Ok(Adt4::C4(f18(v4, v1, v2, v3)?, v6, {
            let a70 = v2;
            let a71 = 4u64;
            nat_mul(a70, a71)?
        }, f18(match v0 {
            Adt3::C0(_, _, h72, _, _) => {
                h72
            }
        }, v1, v2, v3)?))
    } else {
        Ok(Adt4::C0)
    }
}

pub fn f4(v0: Adt2, v1: u64, v2: u64, v3: u64) -> R<Adt4> {
    let a76 = v1;
    let a77 = {
        let m75 = match v0.clone() {
            Adt2::C0(h74, _, _, _, _, _, _, _) => {
                h74
            }
        };
        match m75 {
            Adt1::C0(_, _, h73, _, _, _, _, _, _, _, _) => {
                h73
            }
        }
    };
    if if if nat_lt(a76, a77) {
        let a81 = v2;
        let a82 = {
            let m80 = match v0.clone() {
                Adt2::C0(h79, _, _, _, _, _, _, _) => {
                    h79
                }
            };
            match m80 {
                Adt1::C0(_, _, _, h78, _, _, _, _, _, _, _) => {
                    h78
                }
            }
        };
        nat_lt(a81, a82)
    } else {
        false
    } {
        let a86 = v3;
        let a87 = {
            let m85 = match v0.clone() {
                Adt2::C0(h84, _, _, _, _, _, _, _) => {
                    h84
                }
            };
            match m85 {
                Adt1::C0(_, _, _, _, h83, _, _, _, _, _, _) => {
                    h83
                }
            }
        };
        nat_lt(a86, a87)
    } else {
        false
    } {
        Ok(Adt4::C2(f18(match v0.clone() {
            Adt2::C0(h88, _, _, _, _, _, _, _) => {
                h88
            }
        }, v1, v2, v3)?, f18(match v0.clone() {
            Adt2::C0(_, h89, _, _, _, _, _, _) => {
                h89
            }
        }, 0u64, {
            let a99 = v2;
            let a100 = {
                let a96 = {
                    let m92 = match v0.clone() {
                        Adt2::C0(h91, _, _, _, _, _, _, _) => {
                            h91
                        }
                    };
                    match m92 {
                        Adt1::C0(_, _, _, h90, _, _, _, _, _, _, _) => {
                            h90
                        }
                    }
                };
                let a97 = {
                    let m95 = match v0.clone() {
                        Adt2::C0(_, h94, _, _, _, _, _, _) => {
                            h94
                        }
                    };
                    match m95 {
                        Adt1::C0(_, _, _, h93, _, _, _, _, _, _, _) => {
                            h93
                        }
                    }
                };
                let a98 = 0u64;
                nat_quot(a96, a97, a98)
            };
            let a101 = 0u64;
            nat_quot(a99, a100, a101)
        }, {
            let a111 = v3;
            let a112 = {
                let a108 = {
                    let m104 = match v0.clone() {
                        Adt2::C0(h103, _, _, _, _, _, _, _) => {
                            h103
                        }
                    };
                    match m104 {
                        Adt1::C0(_, _, _, _, h102, _, _, _, _, _, _) => {
                            h102
                        }
                    }
                };
                let a109 = {
                    let m107 = match v0.clone() {
                        Adt2::C0(_, h106, _, _, _, _, _, _) => {
                            h106
                        }
                    };
                    match m107 {
                        Adt1::C0(_, _, _, _, h105, _, _, _, _, _, _) => {
                            h105
                        }
                    }
                };
                let a110 = 0u64;
                nat_quot(a108, a109, a110)
            };
            let a113 = 0u64;
            nat_quot(a111, a112, a113)
        })?, f18(match v0.clone() {
            Adt2::C0(_, _, h114, _, _, _, _, _) => {
                h114
            }
        }, 0u64, {
            let a124 = v2;
            let a125 = {
                let a121 = {
                    let m117 = match v0.clone() {
                        Adt2::C0(h116, _, _, _, _, _, _, _) => {
                            h116
                        }
                    };
                    match m117 {
                        Adt1::C0(_, _, _, h115, _, _, _, _, _, _, _) => {
                            h115
                        }
                    }
                };
                let a122 = {
                    let m120 = match v0.clone() {
                        Adt2::C0(_, _, h119, _, _, _, _, _) => {
                            h119
                        }
                    };
                    match m120 {
                        Adt1::C0(_, _, _, h118, _, _, _, _, _, _, _) => {
                            h118
                        }
                    }
                };
                let a123 = 0u64;
                nat_quot(a121, a122, a123)
            };
            let a126 = 0u64;
            nat_quot(a124, a125, a126)
        }, {
            let a136 = v3;
            let a137 = {
                let a133 = {
                    let m129 = match v0.clone() {
                        Adt2::C0(h128, _, _, _, _, _, _, _) => {
                            h128
                        }
                    };
                    match m129 {
                        Adt1::C0(_, _, _, _, h127, _, _, _, _, _, _) => {
                            h127
                        }
                    }
                };
                let a134 = {
                    let m132 = match v0.clone() {
                        Adt2::C0(_, _, h131, _, _, _, _, _) => {
                            h131
                        }
                    };
                    match m132 {
                        Adt1::C0(_, _, _, _, h130, _, _, _, _, _, _) => {
                            h130
                        }
                    }
                };
                let a135 = 0u64;
                nat_quot(a133, a134, a135)
            };
            let a138 = 0u64;
            nat_quot(a136, a137, a138)
        })?, {
            let m140 = match v0.clone() {
                Adt2::C0(_, _, _, h139, _, _, _, _) => {
                    h139
                }
            };
            match m140 {
                None => {
                    0u64
                }
                Some(v4) => {
                    f18(v4.clone(), v1, {
                        let a142 = v2;
                        let a143 = match v4.clone() {
                            Adt1::C0(_, _, _, h141, _, _, _, _, _, _, _) => {
                                h141
                            }
                        };
                        let a144 = 0u64;
                        nat_rem(a142, a143, a144)
                    }, {
                        let a146 = v3;
                        let a147 = match v4 {
                            Adt1::C0(_, _, _, _, h145, _, _, _, _, _, _) => {
                                h145
                            }
                        };
                        let a148 = 0u64;
                        nat_rem(a146, a147, a148)
                    })?
                }
            }
        }, {
            let a149 = v2;
            let a150 = 4u64;
            nat_mul(a149, a150)?
        }, f18(match v0.clone() {
            Adt2::C0(_, _, _, _, _, h151, _, _) => {
                h151
            }
        }, v2, v1, v3)?, {
            let m154 = match v0.clone() {
                Adt2::C0(_, h153, _, _, _, _, _, _) => {
                    h153
                }
            };
            match m154 {
                Adt1::C0(_, _, _, _, _, _, h152, _, _, _, _) => {
                    h152
                }
            }
        }, {
            let m157 = match v0.clone() {
                Adt2::C0(_, _, h156, _, _, _, _, _) => {
                    h156
                }
            };
            match m157 {
                Adt1::C0(_, _, _, _, _, _, h155, _, _, _, _) => {
                    h155
                }
            }
        }, f19({
            let m160 = match v0 {
                Adt2::C0(_, h159, _, _, _, _, _, _) => {
                    h159
                }
            };
            match m160 {
                Adt1::C0(h158, _, _, _, _, _, _, _, _, _, _) => {
                    h158
                }
            }
        })))
    } else {
        Ok(Adt4::C0)
    }
}

pub fn f5(v0: u64, v1: u64, v2: Option<Adt1>) -> bool {
    let a161 = v0;
    let a162 = 0u64;
    if if nat_eq(a161, a162) {
        true
    } else {
        let a163 = 2147483648u64;
        let a164 = v0;
        if nat_le(a163, a164) {
            let a165 = v0;
            let a166 = 4286578688u64;
            nat_lt(a165, a166)
        } else {
            false
        }
    } {
        true
    } else {
        let a167 = v0;
        let a168 = 2139095040u64;
        if if nat_lt(a167, a168) {
            let a169 = v1;
            let a170 = 4294967295u64;
            nat_le(a169, a170)
        } else {
            false
        } {
            let m171 = v2;
            match m171 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            }
        } else {
            false
        }
    }
}

pub fn f6(v0: Adt1) -> R<bool> {
    let a173 = match v0.clone() {
        Adt1::C0(h172, _, _, _, _, _, _, _, _, _, _) => {
            h172
        }
    };
    let a174 = 0u64;
    if if if if nat_eq(a173, a174) {
        let a176 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h175, _, _, _, _, _) => {
                h175
            }
        };
        let a177 = 4u64;
        nat_eq(a176, a177)
    } else {
        false
    } {
        let a183 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h178, _, _, _, _) => {
                h178
            }
        };
        let a184 = {
            let a181 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, h179, _, _, _, _, _) => {
                    h179
                }
            };
            let a182 = match v0.clone() {
                Adt1::C0(_, h180, _, _, _, _, _, _, _, _, _) => {
                    h180
                }
            };
            nat_mul(a181, a182)?
        };
        nat_eq(a183, a184)
    } else {
        false
    } {
        let a190 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h185, _, _, _) => {
                h185
            }
        };
        let a191 = {
            let a188 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h186, _, _, _, _) => {
                    h186
                }
            };
            let a189 = match v0.clone() {
                Adt1::C0(_, _, h187, _, _, _, _, _, _, _, _) => {
                    h187
                }
            };
            nat_mul(a188, a189)?
        };
        nat_eq(a190, a191)
    } else {
        false
    } {
        let a197 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h192, _, _) => {
                h192
            }
        };
        let a198 = {
            let a195 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h193, _, _, _) => {
                    h193
                }
            };
            let a196 = match v0 {
                Adt1::C0(_, _, _, h194, _, _, _, _, _, _, _) => {
                    h194
                }
            };
            nat_mul(a195, a196)?
        };
        Ok(nat_eq(a197, a198))
    } else {
        Ok(false)
    }
}

pub fn f7(v0: Adt1, v1: Adt1, v2: Adt1) -> R<bool> {
    if if if f20(match v0.clone() {
        Adt1::C0(h199, _, _, _, _, _, _, _, _, _, _) => {
            h199
        }
    }, match v0.clone() {
        Adt1::C0(_, h200, _, _, _, _, _, _, _, _, _) => {
            h200
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h201, _, _, _, _, _) => {
            h201
        }
    }) {
        f21(match v1.clone() {
            Adt1::C0(_, _, _, h202, _, _, _, _, _, _, _) => {
                h202
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h203, _, _, _, _, _, _, _) => {
                h203
            }
        })
    } else {
        false
    } {
        f21(match v1 {
            Adt1::C0(_, _, _, _, h204, _, _, _, _, _, _) => {
                h204
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h205, _, _, _, _, _, _) => {
                h205
            }
        })
    } else {
        false
    } {
        f8(v0, v2)
    } else {
        Ok(false)
    }
}

pub fn f8(v0: Adt1, v1: Adt1) -> R<bool> {
    let a207 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h206, _) => {
            h206
        }
    };
    let a208 = 0u64;
    if nat_eq(a207, a208) {
        Ok(true)
    } else {
        let a210 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h209, _) => {
                h209
            }
        };
        let a211 = 0u64;
        if nat_eq(a210, a211) {
            Ok(true)
        } else {
            let a213 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h212) => {
                    h212
                }
            };
            let a214 = 0u64;
            if nat_eq(a213, a214) {
                Ok(true)
            } else {
                let a216 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h215) => {
                        h215
                    }
                };
                let a217 = 0u64;
                if nat_eq(a216, a217) {
                    Ok(true)
                } else {
                    let a223 = {
                        let a220 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h218) => {
                                h218
                            }
                        };
                        let a221 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h219, _) => {
                                h219
                            }
                        };
                        nat_add(a220, a221)?
                    };
                    let a224 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h222) => {
                            h222
                        }
                    };
                    if nat_le(a223, a224) {
                        Ok(true)
                    } else {
                        let a230 = {
                            let a227 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h225) => {
                                    h225
                                }
                            };
                            let a228 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h226, _) => {
                                    h226
                                }
                            };
                            nat_add(a227, a228)?
                        };
                        let a231 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h229) => {
                                h229
                            }
                        };
                        Ok(nat_le(a230, a231))
                    }
                }
            }
        }
    }
}

pub fn f9(v0: Option<Adt1>, v1: Adt1, v2: Adt1, v3: Adt1) -> R<bool> {
    let m232 = v0;
    match m232 {
        None => {
            Ok(true)
        }
        Some(v4) => {
            let a234 = match v4.clone() {
                Adt1::C0(h233, _, _, _, _, _, _, _, _, _, _) => {
                    h233
                }
            };
            let a235 = 1u64;
            if if if if if if nat_eq(a234, a235) {
                let a237 = match v4.clone() {
                    Adt1::C0(_, _, _, _, _, h236, _, _, _, _, _) => {
                        h236
                    }
                };
                let a238 = 2u64;
                nat_eq(a237, a238)
            } else {
                false
            } {
                let a241 = match v2 {
                    Adt1::C0(_, _, h239, _, _, _, _, _, _, _, _) => {
                        h239
                    }
                };
                let a242 = match v4.clone() {
                    Adt1::C0(_, h240, _, _, _, _, _, _, _, _, _) => {
                        h240
                    }
                };
                nat_le(a241, a242)
            } else {
                false
            } {
                let a245 = match v1.clone() {
                    Adt1::C0(_, _, h243, _, _, _, _, _, _, _, _) => {
                        h243
                    }
                };
                let a246 = match v4.clone() {
                    Adt1::C0(_, _, h244, _, _, _, _, _, _, _, _) => {
                        h244
                    }
                };
                nat_le(a245, a246)
            } else {
                false
            } {
                f21(match v1.clone() {
                    Adt1::C0(_, _, _, h247, _, _, _, _, _, _, _) => {
                        h247
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, h248, _, _, _, _, _, _, _) => {
                        h248
                    }
                })
            } else {
                false
            } {
                f21(match v1 {
                    Adt1::C0(_, _, _, _, h249, _, _, _, _, _, _) => {
                        h249
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, _, h250, _, _, _, _, _, _) => {
                        h250
                    }
                })
            } else {
                false
            } {
                f8(v4, v3)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f10(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m251 = v0;
    match m251 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a253 = match v3.clone() {
                Adt1::C0(h252, _, _, _, _, _, _, _, _, _, _) => {
                    h252
                }
            };
            let a254 = 0u64;
            if if if if if if nat_eq(a253, a254) {
                let a256 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h255, _, _, _, _, _) => {
                        h255
                    }
                };
                let a257 = 4u64;
                nat_eq(a256, a257)
            } else {
                false
            } {
                let a260 = match v3.clone() {
                    Adt1::C0(_, h258, _, _, _, _, _, _, _, _, _) => {
                        h258
                    }
                };
                let a261 = match v1 {
                    Adt1::C0(_, _, _, h259, _, _, _, _, _, _, _) => {
                        h259
                    }
                };
                nat_eq(a260, a261)
            } else {
                false
            } {
                let a263 = match v3.clone() {
                    Adt1::C0(_, _, h262, _, _, _, _, _, _, _, _) => {
                        h262
                    }
                };
                let a264 = 1u64;
                nat_eq(a263, a264)
            } else {
                false
            } {
                let a266 = match v3.clone() {
                    Adt1::C0(_, _, _, h265, _, _, _, _, _, _, _) => {
                        h265
                    }
                };
                let a267 = 1u64;
                nat_eq(a266, a267)
            } else {
                false
            } {
                let a269 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h268, _, _, _, _, _, _) => {
                        h268
                    }
                };
                let a270 = 1u64;
                nat_eq(a269, a270)
            } else {
                false
            } {
                f8(v3, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f11(v0: u64) -> Option<u64> {
    let a271 = 0u64;
    let a272 = v0;
    if if nat_lt(a271, a272) {
        let a273 = v0;
        let a274 = 2139095040u64;
        nat_lt(a273, a274)
    } else {
        false
    } {
        Some(v0)
    } else {
        None::<u64>
    }
}

pub fn f12(v0: u64) -> Option<u64> {
    let a275 = v0;
    let a276 = 0u64;
    if if nat_eq(a275, a276) {
        true
    } else {
        let a277 = v0;
        let a278 = 2147483648u64;
        nat_eq(a277, a278)
    } {
        None::<u64>
    } else {
        Some(v0)
    }
}

pub fn f13(v0: Adt1, v1: u64) -> R<bool> {
    let a280 = match v0.clone() {
        Adt1::C0(_, h279, _, _, _, _, _, _, _, _, _) => {
            h279
        }
    };
    let a281 = 1u64;
    if if if if nat_eq(a280, a281) {
        true
    } else {
        let a283 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h282, _, _, _, _, _) => {
                h282
            }
        };
        let a284 = v1;
        nat_eq(a283, a284)
    } {
        let a286 = match v0.clone() {
            Adt1::C0(_, _, h285, _, _, _, _, _, _, _, _) => {
                h285
            }
        };
        let a287 = 1u64;
        if nat_eq(a286, a287) {
            true
        } else {
            let a292 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h288, _, _, _, _) => {
                    h288
                }
            };
            let a293 = {
                let a290 = v1;
                let a291 = match v0.clone() {
                    Adt1::C0(_, h289, _, _, _, _, _, _, _, _, _) => {
                        h289
                    }
                };
                nat_mul(a290, a291)?
            };
            nat_eq(a292, a293)
        }
    } else {
        false
    } {
        let a295 = match v0.clone() {
            Adt1::C0(_, _, _, h294, _, _, _, _, _, _, _) => {
                h294
            }
        };
        let a296 = 1u64;
        if nat_eq(a295, a296) {
            true
        } else {
            let a304 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h297, _, _, _) => {
                    h297
                }
            };
            let a305 = {
                let a302 = {
                    let a299 = v1;
                    let a300 = match v0.clone() {
                        Adt1::C0(_, h298, _, _, _, _, _, _, _, _, _) => {
                            h298
                        }
                    };
                    nat_mul(a299, a300)?
                };
                let a303 = match v0.clone() {
                    Adt1::C0(_, _, h301, _, _, _, _, _, _, _, _) => {
                        h301
                    }
                };
                nat_mul(a302, a303)?
            };
            nat_eq(a304, a305)
        }
    } else {
        false
    } {
        let a307 = match v0.clone() {
            Adt1::C0(_, _, _, _, h306, _, _, _, _, _, _) => {
                h306
            }
        };
        let a308 = 1u64;
        if nat_eq(a307, a308) {
            Ok(true)
        } else {
            let a319 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h309, _, _) => {
                    h309
                }
            };
            let a320 = {
                let a317 = {
                    let a314 = {
                        let a311 = v1;
                        let a312 = match v0.clone() {
                            Adt1::C0(_, h310, _, _, _, _, _, _, _, _, _) => {
                                h310
                            }
                        };
                        nat_mul(a311, a312)?
                    };
                    let a315 = match v0.clone() {
                        Adt1::C0(_, _, h313, _, _, _, _, _, _, _, _) => {
                            h313
                        }
                    };
                    nat_mul(a314, a315)?
                };
                let a318 = match v0 {
                    Adt1::C0(_, _, _, h316, _, _, _, _, _, _, _) => {
                        h316
                    }
                };
                nat_mul(a317, a318)?
            };
            Ok(nat_eq(a319, a320))
        }
    } else {
        Ok(false)
    }
}

pub fn f14(v0: Adt1, v1: Adt1) -> bool {
    let a323 = match v0.clone() {
        Adt1::C0(_, h321, _, _, _, _, _, _, _, _, _) => {
            h321
        }
    };
    let a324 = match v1.clone() {
        Adt1::C0(_, h322, _, _, _, _, _, _, _, _, _) => {
            h322
        }
    };
    if if if nat_eq(a323, a324) {
        let a327 = match v0.clone() {
            Adt1::C0(_, _, h325, _, _, _, _, _, _, _, _) => {
                h325
            }
        };
        let a328 = match v1.clone() {
            Adt1::C0(_, _, h326, _, _, _, _, _, _, _, _) => {
                h326
            }
        };
        nat_eq(a327, a328)
    } else {
        false
    } {
        let a331 = match v0.clone() {
            Adt1::C0(_, _, _, h329, _, _, _, _, _, _, _) => {
                h329
            }
        };
        let a332 = match v1.clone() {
            Adt1::C0(_, _, _, h330, _, _, _, _, _, _, _) => {
                h330
            }
        };
        nat_eq(a331, a332)
    } else {
        false
    } {
        let a335 = match v0 {
            Adt1::C0(_, _, _, _, h333, _, _, _, _, _, _) => {
                h333
            }
        };
        let a336 = match v1 {
            Adt1::C0(_, _, _, _, h334, _, _, _, _, _, _) => {
                h334
            }
        };
        nat_eq(a335, a336)
    } else {
        false
    }
}

pub fn f15(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f14(v0.clone(), v1.clone()) {
        let a339 = match v0.clone() {
            Adt1::C0(h337, _, _, _, _, _, _, _, _, _, _) => {
                h337
            }
        };
        let a340 = match v1.clone() {
            Adt1::C0(h338, _, _, _, _, _, _, _, _, _, _) => {
                h338
            }
        };
        nat_eq(a339, a340)
    } else {
        false
    } {
        let a343 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h341) => {
                h341
            }
        };
        let a344 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h342) => {
                h342
            }
        };
        nat_eq(a343, a344)
    } else {
        false
    } {
        let a347 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h345, _, _, _, _, _) => {
                h345
            }
        };
        let a348 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h346, _, _, _, _, _) => {
                h346
            }
        };
        nat_eq(a347, a348)
    } else {
        false
    } {
        let a351 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h349, _, _, _, _) => {
                h349
            }
        };
        let a352 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h350, _, _, _, _) => {
                h350
            }
        };
        nat_eq(a351, a352)
    } else {
        false
    } {
        let a355 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h353, _, _, _) => {
                h353
            }
        };
        let a356 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h354, _, _, _) => {
                h354
            }
        };
        nat_eq(a355, a356)
    } else {
        false
    } {
        let a359 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h357, _, _) => {
                h357
            }
        };
        let a360 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h358, _, _) => {
                h358
            }
        };
        nat_eq(a359, a360)
    } else {
        false
    }
}

pub fn f16(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m361 = v0;
    match m361 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a363 = match v3.clone() {
                Adt1::C0(h362, _, _, _, _, _, _, _, _, _, _) => {
                    h362
                }
            };
            let a364 = 0u64;
            if if if if if if if nat_eq(a363, a364) {
                true
            } else {
                let a366 = match v3.clone() {
                    Adt1::C0(h365, _, _, _, _, _, _, _, _, _, _) => {
                        h365
                    }
                };
                let a367 = 1u64;
                nat_eq(a366, a367)
            } {
                f13(v3.clone(), {
                    let a369 = match v3.clone() {
                        Adt1::C0(h368, _, _, _, _, _, _, _, _, _, _) => {
                            h368
                        }
                    };
                    let a370 = 0u64;
                    if nat_eq(a369, a370) {
                        4u64
                    } else {
                        2u64
                    }
                })?
            } else {
                false
            } {
                let a373 = match v3.clone() {
                    Adt1::C0(_, h371, _, _, _, _, _, _, _, _, _) => {
                        h371
                    }
                };
                let a374 = match v1.clone() {
                    Adt1::C0(_, h372, _, _, _, _, _, _, _, _, _) => {
                        h372
                    }
                };
                nat_eq(a373, a374)
            } else {
                false
            } {
                let a377 = match v1.clone() {
                    Adt1::C0(_, _, h375, _, _, _, _, _, _, _, _) => {
                        h375
                    }
                };
                let a378 = match v3.clone() {
                    Adt1::C0(_, _, h376, _, _, _, _, _, _, _, _) => {
                        h376
                    }
                };
                nat_le(a377, a378)
            } else {
                false
            } {
                f21(match v1.clone() {
                    Adt1::C0(_, _, _, h379, _, _, _, _, _, _, _) => {
                        h379
                    }
                }, match v3.clone() {
                    Adt1::C0(_, _, _, h380, _, _, _, _, _, _, _) => {
                        h380
                    }
                })
            } else {
                false
            } {
                f21(match v1 {
                    Adt1::C0(_, _, _, _, h381, _, _, _, _, _, _) => {
                        h381
                    }
                }, match v3.clone() {
                    Adt1::C0(_, _, _, _, h382, _, _, _, _, _, _) => {
                        h382
                    }
                })
            } else {
                false
            } {
                f8(v3, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f17(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m383 = v0;
    match m383 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a385 = match v3.clone() {
                Adt1::C0(h384, _, _, _, _, _, _, _, _, _, _) => {
                    h384
                }
            };
            let a386 = 0u64;
            if if if if if if nat_eq(a385, a386) {
                let a388 = match v3.clone() {
                    Adt1::C0(_, h387, _, _, _, _, _, _, _, _, _) => {
                        h387
                    }
                };
                let a389 = 1u64;
                if nat_eq(a388, a389) {
                    true
                } else {
                    let a391 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, h390, _, _, _, _, _) => {
                            h390
                        }
                    };
                    let a392 = 4u64;
                    nat_eq(a391, a392)
                }
            } else {
                false
            } {
                let a395 = match v3.clone() {
                    Adt1::C0(_, h393, _, _, _, _, _, _, _, _, _) => {
                        h393
                    }
                };
                let a396 = match v1 {
                    Adt1::C0(_, _, _, h394, _, _, _, _, _, _, _) => {
                        h394
                    }
                };
                nat_eq(a395, a396)
            } else {
                false
            } {
                let a398 = match v3.clone() {
                    Adt1::C0(_, _, h397, _, _, _, _, _, _, _, _) => {
                        h397
                    }
                };
                let a399 = 1u64;
                nat_eq(a398, a399)
            } else {
                false
            } {
                let a401 = match v3.clone() {
                    Adt1::C0(_, _, _, h400, _, _, _, _, _, _, _) => {
                        h400
                    }
                };
                let a402 = 1u64;
                nat_eq(a401, a402)
            } else {
                false
            } {
                let a404 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h403, _, _, _, _, _, _) => {
                        h403
                    }
                };
                let a405 = 1u64;
                nat_eq(a404, a405)
            } else {
                false
            } {
                f8(v3, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f18(v0: Adt1, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a417 = {
        let a412 = {
            let a407 = v1;
            let a408 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h406, _, _, _, _) => {
                    h406
                }
            };
            nat_mul(a407, a408)?
        };
        let a413 = {
            let a410 = v2;
            let a411 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h409, _, _, _) => {
                    h409
                }
            };
            nat_mul(a410, a411)?
        };
        nat_add(a412, a413)?
    };
    let a418 = {
        let a415 = v3;
        let a416 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h414, _, _) => {
                h414
            }
        };
        nat_mul(a415, a416)?
    };
    nat_add(a417, a418)
}

pub fn f19(v0: u64) -> u64 {
    if f22(v0) {
        8u64
    } else {
        f23(v0)
    }
}

pub fn f20(v0: u64, v1: u64, v2: u64) -> bool {
    let m419 = f24(v0);
    match m419 {
        Adt5::C0 => {
            false
        }
        Adt5::C1(v3) => {
            let a420 = v0;
            let a421 = 0u64;
            if if if if if nat_eq(a420, a421) {
                true
            } else {
                let a422 = v0;
                let a423 = 1u64;
                nat_eq(a422, a423)
            } {
                true
            } else {
                let a424 = v0;
                let a425 = 30u64;
                nat_eq(a424, a425)
            } {
                true
            } else {
                f22(v0)
            } {
                let a427 = v2;
                let a428 = match v3.clone() {
                    Adt6::C0(_, _, h426) => {
                        h426
                    }
                };
                nat_eq(a427, a428)
            } else {
                false
            } {
                f21(v1, match v3 {
                    Adt6::C0(_, h429, _) => {
                        h429
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f21(v0: u64, v1: u64) -> bool {
    let a430 = 0u64;
    let a431 = v1;
    if nat_lt(a430, a431) {
        let a435 = {
            let a432 = v0;
            let a433 = v1;
            let a434 = 0u64;
            nat_rem(a432, a433, a434)
        };
        let a436 = 0u64;
        nat_eq(a435, a436)
    } else {
        false
    }
}

pub fn f22(v0: u64) -> bool {
    let a437 = v0;
    let a438 = 2u64;
    if nat_eq(a437, a438) {
        true
    } else {
        let a439 = v0;
        let a440 = 8u64;
        nat_eq(a439, a440)
    }
}

pub fn f23(v0: u64) -> u64 {
    let a441 = v0;
    let a442 = 30u64;
    if nat_eq(a441, a442) {
        30u64
    } else {
        0u64
    }
}

pub fn f24(v0: u64) -> Adt5 {
    let a443 = v0;
    let a444 = 0u64;
    if nat_eq(a443, a444) {
        Adt5::C1(Adt6::C0(0u64, 1u64, 4u64))
    } else {
        let a445 = v0;
        let a446 = 1u64;
        if nat_eq(a445, a446) {
            Adt5::C1(Adt6::C0(1u64, 1u64, 2u64))
        } else {
            let a447 = v0;
            let a448 = 2u64;
            if nat_eq(a447, a448) {
                Adt5::C1(Adt6::C0(2u64, 32u64, 18u64))
            } else {
                let a449 = v0;
                let a450 = 3u64;
                if nat_eq(a449, a450) {
                    Adt5::C1(Adt6::C0(3u64, 32u64, 20u64))
                } else {
                    let a451 = v0;
                    let a452 = 6u64;
                    if nat_eq(a451, a452) {
                        Adt5::C1(Adt6::C0(6u64, 32u64, 22u64))
                    } else {
                        let a453 = v0;
                        let a454 = 7u64;
                        if nat_eq(a453, a454) {
                            Adt5::C1(Adt6::C0(7u64, 32u64, 24u64))
                        } else {
                            let a455 = v0;
                            let a456 = 8u64;
                            if nat_eq(a455, a456) {
                                Adt5::C1(Adt6::C0(8u64, 32u64, 34u64))
                            } else {
                                let a457 = v0;
                                let a458 = 9u64;
                                if nat_eq(a457, a458) {
                                    Adt5::C1(Adt6::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a459 = v0;
                                    let a460 = 10u64;
                                    if nat_eq(a459, a460) {
                                        Adt5::C1(Adt6::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a461 = v0;
                                        let a462 = 11u64;
                                        if nat_eq(a461, a462) {
                                            Adt5::C1(Adt6::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a463 = v0;
                                            let a464 = 12u64;
                                            if nat_eq(a463, a464) {
                                                Adt5::C1(Adt6::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a465 = v0;
                                                let a466 = 13u64;
                                                if nat_eq(a465, a466) {
                                                    Adt5::C1(Adt6::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a467 = v0;
                                                    let a468 = 14u64;
                                                    if nat_eq(a467, a468) {
                                                        Adt5::C1(Adt6::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a469 = v0;
                                                        let a470 = 15u64;
                                                        if nat_eq(a469, a470) {
                                                            Adt5::C1(Adt6::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a471 = v0;
                                                            let a472 = 16u64;
                                                            if nat_eq(a471, a472) {
                                                                Adt5::C1(Adt6::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a473 = v0;
                                                                let a474 = 17u64;
                                                                if nat_eq(a473, a474) {
                                                                    Adt5::C1(Adt6::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a475 = v0;
                                                                    let a476 = 18u64;
                                                                    if nat_eq(a475, a476) {
                                                                        Adt5::C1(Adt6::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a477 = v0;
                                                                        let a478 = 19u64;
                                                                        if nat_eq(a477, a478) {
                                                                            Adt5::C1(Adt6::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a479 = v0;
                                                                            let a480 = 20u64;
                                                                            if nat_eq(a479, a480) {
                                                                                Adt5::C1(Adt6::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a481 = v0;
                                                                                let a482 = 21u64;
                                                                                if nat_eq(a481, a482) {
                                                                                    Adt5::C1(Adt6::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a483 = v0;
                                                                                    let a484 = 22u64;
                                                                                    if nat_eq(a483, a484) {
                                                                                        Adt5::C1(Adt6::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a485 = v0;
                                                                                        let a486 = 23u64;
                                                                                        if nat_eq(a485, a486) {
                                                                                            Adt5::C1(Adt6::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a487 = v0;
                                                                                            let a488 = 24u64;
                                                                                            if nat_eq(a487, a488) {
                                                                                                Adt5::C1(Adt6::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a489 = v0;
                                                                                                let a490 = 25u64;
                                                                                                if nat_eq(a489, a490) {
                                                                                                    Adt5::C1(Adt6::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a491 = v0;
                                                                                                    let a492 = 26u64;
                                                                                                    if nat_eq(a491, a492) {
                                                                                                        Adt5::C1(Adt6::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a493 = v0;
                                                                                                        let a494 = 27u64;
                                                                                                        if nat_eq(a493, a494) {
                                                                                                            Adt5::C1(Adt6::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a495 = v0;
                                                                                                            let a496 = 28u64;
                                                                                                            if nat_eq(a495, a496) {
                                                                                                                Adt5::C1(Adt6::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a497 = v0;
                                                                                                                let a498 = 29u64;
                                                                                                                if nat_eq(a497, a498) {
                                                                                                                    Adt5::C1(Adt6::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a499 = v0;
                                                                                                                    let a500 = 30u64;
                                                                                                                    if nat_eq(a499, a500) {
                                                                                                                        Adt5::C1(Adt6::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a501 = v0;
                                                                                                                        let a502 = 34u64;
                                                                                                                        if nat_eq(a501, a502) {
                                                                                                                            Adt5::C1(Adt6::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a503 = v0;
                                                                                                                            let a504 = 35u64;
                                                                                                                            if nat_eq(a503, a504) {
                                                                                                                                Adt5::C1(Adt6::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a505 = v0;
                                                                                                                                let a506 = 39u64;
                                                                                                                                if nat_eq(a505, a506) {
                                                                                                                                    Adt5::C1(Adt6::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a507 = v0;
                                                                                                                                    let a508 = 40u64;
                                                                                                                                    if nat_eq(a507, a508) {
                                                                                                                                        Adt5::C1(Adt6::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a509 = v0;
                                                                                                                                        let a510 = 41u64;
                                                                                                                                        if nat_eq(a509, a510) {
                                                                                                                                            Adt5::C1(Adt6::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a511 = v0;
                                                                                                                                            let a512 = 42u64;
                                                                                                                                            if nat_eq(a511, a512) {
                                                                                                                                                Adt5::C1(Adt6::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a513 = v0;
                                                                                                                                                let a514 = 43u64;
                                                                                                                                                if nat_eq(a513, a514) {
                                                                                                                                                    Adt5::C1(Adt6::C0(43u64, 1u64, 1u64))
                                                                                                                                                } else {
                                                                                                                                                    Adt5::C0
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
