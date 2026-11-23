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
    C0,
    C1(Adt2),
    C2(u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt4 {
    C0,
    C1(Adt5),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(u64, u64, u64),
}

pub fn f0(v0: Adt0) -> R<Adt3> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11) => {
            f1(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11)
        }
        Adt0::C1(v12, v13, v14, v15) => {
            f2(v12, v13, v14, v15)
        }
    }
}

pub fn f1(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1, v8: Option<Adt1>, v9: Option<Adt1>, v10: Adt1) -> R<Adt3> {
    let a2 = v0;
    let a3 = 71u64;
    if if if if if if if if if if if if if if if if if if nat_eq(a2, a3) {
        !v1
    } else {
        false
    } {
        f3(v2, match v5.clone() {
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
        f4(v10.clone())?
    } else {
        false
    } {
        f5(v6.clone(), v5.clone(), v10.clone())?
    } else {
        false
    } {
        f5(v7.clone(), v5.clone(), v10.clone())?
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
        f6(v5.clone(), v10.clone())?
    } else {
        false
    } {
        f7(v8.clone(), v5.clone(), v6.clone(), v10.clone())?
    } else {
        false
    } {
        f8(v9.clone(), v5.clone(), v10.clone())?
    } else {
        false
    } {
        Ok(Adt3::C1(Adt2::C0(v5, v6, v7, v8, v9, v10, f9(v2), f10(v3))))
    } else {
        Ok(Adt3::C0)
    }
}

pub fn f2(v0: Adt2, v1: u64, v2: u64, v3: u64) -> R<Adt3> {
    let a44 = v1;
    let a45 = {
        let m43 = match v0.clone() {
            Adt2::C0(h42, _, _, _, _, _, _, _) => {
                h42
            }
        };
        match m43 {
            Adt1::C0(_, _, h41, _, _, _, _, _, _, _, _) => {
                h41
            }
        }
    };
    if if if nat_lt(a44, a45) {
        let a49 = v2;
        let a50 = {
            let m48 = match v0.clone() {
                Adt2::C0(h47, _, _, _, _, _, _, _) => {
                    h47
                }
            };
            match m48 {
                Adt1::C0(_, _, _, h46, _, _, _, _, _, _, _) => {
                    h46
                }
            }
        };
        nat_lt(a49, a50)
    } else {
        false
    } {
        let a54 = v3;
        let a55 = {
            let m53 = match v0.clone() {
                Adt2::C0(h52, _, _, _, _, _, _, _) => {
                    h52
                }
            };
            match m53 {
                Adt1::C0(_, _, _, _, h51, _, _, _, _, _, _) => {
                    h51
                }
            }
        };
        nat_lt(a54, a55)
    } else {
        false
    } {
        Ok(Adt3::C2(f11(match v0.clone() {
            Adt2::C0(h56, _, _, _, _, _, _, _) => {
                h56
            }
        }, v1, v2, v3)?, f11(match v0.clone() {
            Adt2::C0(_, h57, _, _, _, _, _, _) => {
                h57
            }
        }, 0u64, {
            let a67 = v2;
            let a68 = {
                let a64 = {
                    let m60 = match v0.clone() {
                        Adt2::C0(h59, _, _, _, _, _, _, _) => {
                            h59
                        }
                    };
                    match m60 {
                        Adt1::C0(_, _, _, h58, _, _, _, _, _, _, _) => {
                            h58
                        }
                    }
                };
                let a65 = {
                    let m63 = match v0.clone() {
                        Adt2::C0(_, h62, _, _, _, _, _, _) => {
                            h62
                        }
                    };
                    match m63 {
                        Adt1::C0(_, _, _, h61, _, _, _, _, _, _, _) => {
                            h61
                        }
                    }
                };
                let a66 = 0u64;
                nat_quot(a64, a65, a66)
            };
            let a69 = 0u64;
            nat_quot(a67, a68, a69)
        }, {
            let a79 = v3;
            let a80 = {
                let a76 = {
                    let m72 = match v0.clone() {
                        Adt2::C0(h71, _, _, _, _, _, _, _) => {
                            h71
                        }
                    };
                    match m72 {
                        Adt1::C0(_, _, _, _, h70, _, _, _, _, _, _) => {
                            h70
                        }
                    }
                };
                let a77 = {
                    let m75 = match v0.clone() {
                        Adt2::C0(_, h74, _, _, _, _, _, _) => {
                            h74
                        }
                    };
                    match m75 {
                        Adt1::C0(_, _, _, _, h73, _, _, _, _, _, _) => {
                            h73
                        }
                    }
                };
                let a78 = 0u64;
                nat_quot(a76, a77, a78)
            };
            let a81 = 0u64;
            nat_quot(a79, a80, a81)
        })?, f11(match v0.clone() {
            Adt2::C0(_, _, h82, _, _, _, _, _) => {
                h82
            }
        }, 0u64, {
            let a92 = v2;
            let a93 = {
                let a89 = {
                    let m85 = match v0.clone() {
                        Adt2::C0(h84, _, _, _, _, _, _, _) => {
                            h84
                        }
                    };
                    match m85 {
                        Adt1::C0(_, _, _, h83, _, _, _, _, _, _, _) => {
                            h83
                        }
                    }
                };
                let a90 = {
                    let m88 = match v0.clone() {
                        Adt2::C0(_, _, h87, _, _, _, _, _) => {
                            h87
                        }
                    };
                    match m88 {
                        Adt1::C0(_, _, _, h86, _, _, _, _, _, _, _) => {
                            h86
                        }
                    }
                };
                let a91 = 0u64;
                nat_quot(a89, a90, a91)
            };
            let a94 = 0u64;
            nat_quot(a92, a93, a94)
        }, {
            let a104 = v3;
            let a105 = {
                let a101 = {
                    let m97 = match v0.clone() {
                        Adt2::C0(h96, _, _, _, _, _, _, _) => {
                            h96
                        }
                    };
                    match m97 {
                        Adt1::C0(_, _, _, _, h95, _, _, _, _, _, _) => {
                            h95
                        }
                    }
                };
                let a102 = {
                    let m100 = match v0.clone() {
                        Adt2::C0(_, _, h99, _, _, _, _, _) => {
                            h99
                        }
                    };
                    match m100 {
                        Adt1::C0(_, _, _, _, h98, _, _, _, _, _, _) => {
                            h98
                        }
                    }
                };
                let a103 = 0u64;
                nat_quot(a101, a102, a103)
            };
            let a106 = 0u64;
            nat_quot(a104, a105, a106)
        })?, {
            let m108 = match v0.clone() {
                Adt2::C0(_, _, _, h107, _, _, _, _) => {
                    h107
                }
            };
            match m108 {
                None => {
                    0u64
                }
                Some(v4) => {
                    f11(v4.clone(), v1, {
                        let a110 = v2;
                        let a111 = match v4.clone() {
                            Adt1::C0(_, _, _, h109, _, _, _, _, _, _, _) => {
                                h109
                            }
                        };
                        let a112 = 0u64;
                        nat_rem(a110, a111, a112)
                    }, {
                        let a114 = v3;
                        let a115 = match v4 {
                            Adt1::C0(_, _, _, _, h113, _, _, _, _, _, _) => {
                                h113
                            }
                        };
                        let a116 = 0u64;
                        nat_rem(a114, a115, a116)
                    })?
                }
            }
        }, {
            let a117 = v2;
            let a118 = 4u64;
            nat_mul(a117, a118)?
        }, f11(match v0.clone() {
            Adt2::C0(_, _, _, _, _, h119, _, _) => {
                h119
            }
        }, v2, v1, v3)?, {
            let m122 = match v0.clone() {
                Adt2::C0(_, h121, _, _, _, _, _, _) => {
                    h121
                }
            };
            match m122 {
                Adt1::C0(_, _, _, _, _, _, h120, _, _, _, _) => {
                    h120
                }
            }
        }, {
            let m125 = match v0.clone() {
                Adt2::C0(_, _, h124, _, _, _, _, _) => {
                    h124
                }
            };
            match m125 {
                Adt1::C0(_, _, _, _, _, _, h123, _, _, _, _) => {
                    h123
                }
            }
        }, f12({
            let m128 = match v0 {
                Adt2::C0(_, h127, _, _, _, _, _, _) => {
                    h127
                }
            };
            match m128 {
                Adt1::C0(h126, _, _, _, _, _, _, _, _, _, _) => {
                    h126
                }
            }
        })))
    } else {
        Ok(Adt3::C0)
    }
}

pub fn f3(v0: u64, v1: u64, v2: Option<Adt1>) -> bool {
    let a129 = v0;
    let a130 = 0u64;
    if if nat_eq(a129, a130) {
        true
    } else {
        let a131 = 2147483648u64;
        let a132 = v0;
        if nat_le(a131, a132) {
            let a133 = v0;
            let a134 = 4286578688u64;
            nat_lt(a133, a134)
        } else {
            false
        }
    } {
        true
    } else {
        let a135 = v0;
        let a136 = 2139095040u64;
        if if nat_lt(a135, a136) {
            let a137 = v1;
            let a138 = 4294967295u64;
            nat_le(a137, a138)
        } else {
            false
        } {
            let m139 = v2;
            match m139 {
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

pub fn f4(v0: Adt1) -> R<bool> {
    let a141 = match v0.clone() {
        Adt1::C0(h140, _, _, _, _, _, _, _, _, _, _) => {
            h140
        }
    };
    let a142 = 0u64;
    if if if if nat_eq(a141, a142) {
        let a144 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h143, _, _, _, _, _) => {
                h143
            }
        };
        let a145 = 4u64;
        nat_eq(a144, a145)
    } else {
        false
    } {
        let a151 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h146, _, _, _, _) => {
                h146
            }
        };
        let a152 = {
            let a149 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, h147, _, _, _, _, _) => {
                    h147
                }
            };
            let a150 = match v0.clone() {
                Adt1::C0(_, h148, _, _, _, _, _, _, _, _, _) => {
                    h148
                }
            };
            nat_mul(a149, a150)?
        };
        nat_eq(a151, a152)
    } else {
        false
    } {
        let a158 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h153, _, _, _) => {
                h153
            }
        };
        let a159 = {
            let a156 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h154, _, _, _, _) => {
                    h154
                }
            };
            let a157 = match v0.clone() {
                Adt1::C0(_, _, h155, _, _, _, _, _, _, _, _) => {
                    h155
                }
            };
            nat_mul(a156, a157)?
        };
        nat_eq(a158, a159)
    } else {
        false
    } {
        let a165 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h160, _, _) => {
                h160
            }
        };
        let a166 = {
            let a163 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h161, _, _, _) => {
                    h161
                }
            };
            let a164 = match v0 {
                Adt1::C0(_, _, _, h162, _, _, _, _, _, _, _) => {
                    h162
                }
            };
            nat_mul(a163, a164)?
        };
        Ok(nat_eq(a165, a166))
    } else {
        Ok(false)
    }
}

pub fn f5(v0: Adt1, v1: Adt1, v2: Adt1) -> R<bool> {
    if if if f13(match v0.clone() {
        Adt1::C0(h167, _, _, _, _, _, _, _, _, _, _) => {
            h167
        }
    }, match v0.clone() {
        Adt1::C0(_, h168, _, _, _, _, _, _, _, _, _) => {
            h168
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h169, _, _, _, _, _) => {
            h169
        }
    }) {
        f14(match v1.clone() {
            Adt1::C0(_, _, _, h170, _, _, _, _, _, _, _) => {
                h170
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h171, _, _, _, _, _, _, _) => {
                h171
            }
        })
    } else {
        false
    } {
        f14(match v1 {
            Adt1::C0(_, _, _, _, h172, _, _, _, _, _, _) => {
                h172
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h173, _, _, _, _, _, _) => {
                h173
            }
        })
    } else {
        false
    } {
        f6(v0, v2)
    } else {
        Ok(false)
    }
}

pub fn f6(v0: Adt1, v1: Adt1) -> R<bool> {
    let a175 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h174, _) => {
            h174
        }
    };
    let a176 = 0u64;
    if nat_eq(a175, a176) {
        Ok(true)
    } else {
        let a178 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h177, _) => {
                h177
            }
        };
        let a179 = 0u64;
        if nat_eq(a178, a179) {
            Ok(true)
        } else {
            let a181 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h180) => {
                    h180
                }
            };
            let a182 = 0u64;
            if nat_eq(a181, a182) {
                Ok(true)
            } else {
                let a184 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h183) => {
                        h183
                    }
                };
                let a185 = 0u64;
                if nat_eq(a184, a185) {
                    Ok(true)
                } else {
                    let a191 = {
                        let a188 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h186) => {
                                h186
                            }
                        };
                        let a189 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h187, _) => {
                                h187
                            }
                        };
                        nat_add(a188, a189)?
                    };
                    let a192 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h190) => {
                            h190
                        }
                    };
                    if nat_le(a191, a192) {
                        Ok(true)
                    } else {
                        let a198 = {
                            let a195 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h193) => {
                                    h193
                                }
                            };
                            let a196 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h194, _) => {
                                    h194
                                }
                            };
                            nat_add(a195, a196)?
                        };
                        let a199 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h197) => {
                                h197
                            }
                        };
                        Ok(nat_le(a198, a199))
                    }
                }
            }
        }
    }
}

pub fn f7(v0: Option<Adt1>, v1: Adt1, v2: Adt1, v3: Adt1) -> R<bool> {
    let m200 = v0;
    match m200 {
        None => {
            Ok(true)
        }
        Some(v4) => {
            let a202 = match v4.clone() {
                Adt1::C0(h201, _, _, _, _, _, _, _, _, _, _) => {
                    h201
                }
            };
            let a203 = 1u64;
            if if if if if if nat_eq(a202, a203) {
                let a205 = match v4.clone() {
                    Adt1::C0(_, _, _, _, _, h204, _, _, _, _, _) => {
                        h204
                    }
                };
                let a206 = 2u64;
                nat_eq(a205, a206)
            } else {
                false
            } {
                let a209 = match v2 {
                    Adt1::C0(_, _, h207, _, _, _, _, _, _, _, _) => {
                        h207
                    }
                };
                let a210 = match v4.clone() {
                    Adt1::C0(_, h208, _, _, _, _, _, _, _, _, _) => {
                        h208
                    }
                };
                nat_le(a209, a210)
            } else {
                false
            } {
                let a213 = match v1.clone() {
                    Adt1::C0(_, _, h211, _, _, _, _, _, _, _, _) => {
                        h211
                    }
                };
                let a214 = match v4.clone() {
                    Adt1::C0(_, _, h212, _, _, _, _, _, _, _, _) => {
                        h212
                    }
                };
                nat_le(a213, a214)
            } else {
                false
            } {
                f14(match v1.clone() {
                    Adt1::C0(_, _, _, h215, _, _, _, _, _, _, _) => {
                        h215
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, h216, _, _, _, _, _, _, _) => {
                        h216
                    }
                })
            } else {
                false
            } {
                f14(match v1 {
                    Adt1::C0(_, _, _, _, h217, _, _, _, _, _, _) => {
                        h217
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, _, h218, _, _, _, _, _, _) => {
                        h218
                    }
                })
            } else {
                false
            } {
                f6(v4, v3)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f8(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m219 = v0;
    match m219 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a221 = match v3.clone() {
                Adt1::C0(h220, _, _, _, _, _, _, _, _, _, _) => {
                    h220
                }
            };
            let a222 = 0u64;
            if if if if if if nat_eq(a221, a222) {
                let a224 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h223, _, _, _, _, _) => {
                        h223
                    }
                };
                let a225 = 4u64;
                nat_eq(a224, a225)
            } else {
                false
            } {
                let a228 = match v3.clone() {
                    Adt1::C0(_, h226, _, _, _, _, _, _, _, _, _) => {
                        h226
                    }
                };
                let a229 = match v1 {
                    Adt1::C0(_, _, _, h227, _, _, _, _, _, _, _) => {
                        h227
                    }
                };
                nat_eq(a228, a229)
            } else {
                false
            } {
                let a231 = match v3.clone() {
                    Adt1::C0(_, _, h230, _, _, _, _, _, _, _, _) => {
                        h230
                    }
                };
                let a232 = 1u64;
                nat_eq(a231, a232)
            } else {
                false
            } {
                let a234 = match v3.clone() {
                    Adt1::C0(_, _, _, h233, _, _, _, _, _, _, _) => {
                        h233
                    }
                };
                let a235 = 1u64;
                nat_eq(a234, a235)
            } else {
                false
            } {
                let a237 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h236, _, _, _, _, _, _) => {
                        h236
                    }
                };
                let a238 = 1u64;
                nat_eq(a237, a238)
            } else {
                false
            } {
                f6(v3, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f9(v0: u64) -> Option<u64> {
    let a239 = 0u64;
    let a240 = v0;
    if if nat_lt(a239, a240) {
        let a241 = v0;
        let a242 = 2139095040u64;
        nat_lt(a241, a242)
    } else {
        false
    } {
        Some(v0)
    } else {
        None::<u64>
    }
}

pub fn f10(v0: u64) -> Option<u64> {
    let a243 = v0;
    let a244 = 0u64;
    if if nat_eq(a243, a244) {
        true
    } else {
        let a245 = v0;
        let a246 = 2147483648u64;
        nat_eq(a245, a246)
    } {
        None::<u64>
    } else {
        Some(v0)
    }
}

pub fn f11(v0: Adt1, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a258 = {
        let a253 = {
            let a248 = v1;
            let a249 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h247, _, _, _, _) => {
                    h247
                }
            };
            nat_mul(a248, a249)?
        };
        let a254 = {
            let a251 = v2;
            let a252 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h250, _, _, _) => {
                    h250
                }
            };
            nat_mul(a251, a252)?
        };
        nat_add(a253, a254)?
    };
    let a259 = {
        let a256 = v3;
        let a257 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h255, _, _) => {
                h255
            }
        };
        nat_mul(a256, a257)?
    };
    nat_add(a258, a259)
}

pub fn f12(v0: u64) -> u64 {
    if f15(v0) {
        8u64
    } else {
        f16(v0)
    }
}

pub fn f13(v0: u64, v1: u64, v2: u64) -> bool {
    let m260 = f17(v0);
    match m260 {
        Adt4::C0 => {
            false
        }
        Adt4::C1(v3) => {
            let a261 = v0;
            let a262 = 0u64;
            if if if if if nat_eq(a261, a262) {
                true
            } else {
                let a263 = v0;
                let a264 = 1u64;
                nat_eq(a263, a264)
            } {
                true
            } else {
                let a265 = v0;
                let a266 = 30u64;
                nat_eq(a265, a266)
            } {
                true
            } else {
                f15(v0)
            } {
                let a268 = v2;
                let a269 = match v3.clone() {
                    Adt5::C0(_, _, h267) => {
                        h267
                    }
                };
                nat_eq(a268, a269)
            } else {
                false
            } {
                f14(v1, match v3 {
                    Adt5::C0(_, h270, _) => {
                        h270
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f14(v0: u64, v1: u64) -> bool {
    let a271 = 0u64;
    let a272 = v1;
    if nat_lt(a271, a272) {
        let a276 = {
            let a273 = v0;
            let a274 = v1;
            let a275 = 0u64;
            nat_rem(a273, a274, a275)
        };
        let a277 = 0u64;
        nat_eq(a276, a277)
    } else {
        false
    }
}

pub fn f15(v0: u64) -> bool {
    let a278 = v0;
    let a279 = 2u64;
    if nat_eq(a278, a279) {
        true
    } else {
        let a280 = v0;
        let a281 = 8u64;
        nat_eq(a280, a281)
    }
}

pub fn f16(v0: u64) -> u64 {
    let a282 = v0;
    let a283 = 30u64;
    if nat_eq(a282, a283) {
        30u64
    } else {
        0u64
    }
}

pub fn f17(v0: u64) -> Adt4 {
    let a284 = v0;
    let a285 = 0u64;
    if nat_eq(a284, a285) {
        Adt4::C1(Adt5::C0(0u64, 1u64, 4u64))
    } else {
        let a286 = v0;
        let a287 = 1u64;
        if nat_eq(a286, a287) {
            Adt4::C1(Adt5::C0(1u64, 1u64, 2u64))
        } else {
            let a288 = v0;
            let a289 = 2u64;
            if nat_eq(a288, a289) {
                Adt4::C1(Adt5::C0(2u64, 32u64, 18u64))
            } else {
                let a290 = v0;
                let a291 = 3u64;
                if nat_eq(a290, a291) {
                    Adt4::C1(Adt5::C0(3u64, 32u64, 20u64))
                } else {
                    let a292 = v0;
                    let a293 = 6u64;
                    if nat_eq(a292, a293) {
                        Adt4::C1(Adt5::C0(6u64, 32u64, 22u64))
                    } else {
                        let a294 = v0;
                        let a295 = 7u64;
                        if nat_eq(a294, a295) {
                            Adt4::C1(Adt5::C0(7u64, 32u64, 24u64))
                        } else {
                            let a296 = v0;
                            let a297 = 8u64;
                            if nat_eq(a296, a297) {
                                Adt4::C1(Adt5::C0(8u64, 32u64, 34u64))
                            } else {
                                let a298 = v0;
                                let a299 = 9u64;
                                if nat_eq(a298, a299) {
                                    Adt4::C1(Adt5::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a300 = v0;
                                    let a301 = 10u64;
                                    if nat_eq(a300, a301) {
                                        Adt4::C1(Adt5::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a302 = v0;
                                        let a303 = 11u64;
                                        if nat_eq(a302, a303) {
                                            Adt4::C1(Adt5::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a304 = v0;
                                            let a305 = 12u64;
                                            if nat_eq(a304, a305) {
                                                Adt4::C1(Adt5::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a306 = v0;
                                                let a307 = 13u64;
                                                if nat_eq(a306, a307) {
                                                    Adt4::C1(Adt5::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a308 = v0;
                                                    let a309 = 14u64;
                                                    if nat_eq(a308, a309) {
                                                        Adt4::C1(Adt5::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a310 = v0;
                                                        let a311 = 15u64;
                                                        if nat_eq(a310, a311) {
                                                            Adt4::C1(Adt5::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a312 = v0;
                                                            let a313 = 16u64;
                                                            if nat_eq(a312, a313) {
                                                                Adt4::C1(Adt5::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a314 = v0;
                                                                let a315 = 17u64;
                                                                if nat_eq(a314, a315) {
                                                                    Adt4::C1(Adt5::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a316 = v0;
                                                                    let a317 = 18u64;
                                                                    if nat_eq(a316, a317) {
                                                                        Adt4::C1(Adt5::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a318 = v0;
                                                                        let a319 = 19u64;
                                                                        if nat_eq(a318, a319) {
                                                                            Adt4::C1(Adt5::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a320 = v0;
                                                                            let a321 = 20u64;
                                                                            if nat_eq(a320, a321) {
                                                                                Adt4::C1(Adt5::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a322 = v0;
                                                                                let a323 = 21u64;
                                                                                if nat_eq(a322, a323) {
                                                                                    Adt4::C1(Adt5::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a324 = v0;
                                                                                    let a325 = 22u64;
                                                                                    if nat_eq(a324, a325) {
                                                                                        Adt4::C1(Adt5::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a326 = v0;
                                                                                        let a327 = 23u64;
                                                                                        if nat_eq(a326, a327) {
                                                                                            Adt4::C1(Adt5::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a328 = v0;
                                                                                            let a329 = 24u64;
                                                                                            if nat_eq(a328, a329) {
                                                                                                Adt4::C1(Adt5::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a330 = v0;
                                                                                                let a331 = 25u64;
                                                                                                if nat_eq(a330, a331) {
                                                                                                    Adt4::C1(Adt5::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a332 = v0;
                                                                                                    let a333 = 26u64;
                                                                                                    if nat_eq(a332, a333) {
                                                                                                        Adt4::C1(Adt5::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a334 = v0;
                                                                                                        let a335 = 27u64;
                                                                                                        if nat_eq(a334, a335) {
                                                                                                            Adt4::C1(Adt5::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a336 = v0;
                                                                                                            let a337 = 28u64;
                                                                                                            if nat_eq(a336, a337) {
                                                                                                                Adt4::C1(Adt5::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a338 = v0;
                                                                                                                let a339 = 29u64;
                                                                                                                if nat_eq(a338, a339) {
                                                                                                                    Adt4::C1(Adt5::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a340 = v0;
                                                                                                                    let a341 = 30u64;
                                                                                                                    if nat_eq(a340, a341) {
                                                                                                                        Adt4::C1(Adt5::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a342 = v0;
                                                                                                                        let a343 = 34u64;
                                                                                                                        if nat_eq(a342, a343) {
                                                                                                                            Adt4::C1(Adt5::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a344 = v0;
                                                                                                                            let a345 = 35u64;
                                                                                                                            if nat_eq(a344, a345) {
                                                                                                                                Adt4::C1(Adt5::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a346 = v0;
                                                                                                                                let a347 = 39u64;
                                                                                                                                if nat_eq(a346, a347) {
                                                                                                                                    Adt4::C1(Adt5::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a348 = v0;
                                                                                                                                    let a349 = 40u64;
                                                                                                                                    if nat_eq(a348, a349) {
                                                                                                                                        Adt4::C1(Adt5::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a350 = v0;
                                                                                                                                        let a351 = 41u64;
                                                                                                                                        if nat_eq(a350, a351) {
                                                                                                                                            Adt4::C1(Adt5::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a352 = v0;
                                                                                                                                            let a353 = 42u64;
                                                                                                                                            if nat_eq(a352, a353) {
                                                                                                                                                Adt4::C1(Adt5::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a354 = v0;
                                                                                                                                                let a355 = 43u64;
                                                                                                                                                if nat_eq(a354, a355) {
                                                                                                                                                    Adt4::C1(Adt5::C0(43u64, 1u64, 1u64))
                                                                                                                                                } else {
                                                                                                                                                    Adt4::C0
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

pub fn run(p0: Adt0) -> R<Adt3> {
    f0(p0)
}
