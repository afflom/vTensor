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
    C0(Adt1, Adt1, Adt1, Option<Adt1>, Option<Adt1>, Adt1, Option<u64>),
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
        let a4 = v2;
        let a5 = 0u64;
        if nat_eq(a4, a5) {
            true
        } else {
            let a6 = v2;
            let a7 = 2147483648u64;
            nat_eq(a6, a7)
        }
    } else {
        false
    } {
        let a8 = v4;
        let a9 = 0u64;
        if if nat_eq(a8, a9) {
            true
        } else {
            let a10 = v4;
            let a11 = 10u64;
            nat_eq(a10, a11)
        } {
            true
        } else {
            let a12 = v4;
            let a13 = 15u64;
            nat_eq(a12, a13)
        }
    } else {
        false
    } {
        let a15 = match v5.clone() {
            Adt1::C0(h14, _, _, _, _, _, _, _, _, _, _) => {
                h14
            }
        };
        let a16 = 0u64;
        nat_eq(a15, a16)
    } else {
        false
    } {
        let a18 = match v5.clone() {
            Adt1::C0(_, _, _, _, _, h17, _, _, _, _, _) => {
                h17
            }
        };
        let a19 = 4u64;
        nat_eq(a18, a19)
    } else {
        false
    } {
        f3(v10.clone())?
    } else {
        false
    } {
        f4(v6.clone(), v5.clone(), v10.clone())?
    } else {
        false
    } {
        f4(v7.clone(), v5.clone(), v10.clone())?
    } else {
        false
    } {
        let a22 = match v5.clone() {
            Adt1::C0(_, h20, _, _, _, _, _, _, _, _, _) => {
                h20
            }
        };
        let a23 = match v6.clone() {
            Adt1::C0(_, h21, _, _, _, _, _, _, _, _, _) => {
                h21
            }
        };
        nat_eq(a22, a23)
    } else {
        false
    } {
        let a26 = match v6.clone() {
            Adt1::C0(_, _, h24, _, _, _, _, _, _, _, _) => {
                h24
            }
        };
        let a27 = match v7.clone() {
            Adt1::C0(_, _, h25, _, _, _, _, _, _, _, _) => {
                h25
            }
        };
        nat_eq(a26, a27)
    } else {
        false
    } {
        let a30 = match v10.clone() {
            Adt1::C0(_, h28, _, _, _, _, _, _, _, _, _) => {
                h28
            }
        };
        let a31 = match v7.clone() {
            Adt1::C0(_, h29, _, _, _, _, _, _, _, _, _) => {
                h29
            }
        };
        nat_eq(a30, a31)
    } else {
        false
    } {
        let a34 = match v10.clone() {
            Adt1::C0(_, _, h32, _, _, _, _, _, _, _, _) => {
                h32
            }
        };
        let a35 = match v5.clone() {
            Adt1::C0(_, _, _, h33, _, _, _, _, _, _, _) => {
                h33
            }
        };
        nat_eq(a34, a35)
    } else {
        false
    } {
        let a38 = match v10.clone() {
            Adt1::C0(_, _, _, h36, _, _, _, _, _, _, _) => {
                h36
            }
        };
        let a39 = match v5.clone() {
            Adt1::C0(_, _, h37, _, _, _, _, _, _, _, _) => {
                h37
            }
        };
        nat_eq(a38, a39)
    } else {
        false
    } {
        let a42 = match v10.clone() {
            Adt1::C0(_, _, _, _, h40, _, _, _, _, _, _) => {
                h40
            }
        };
        let a43 = match v5.clone() {
            Adt1::C0(_, _, _, _, h41, _, _, _, _, _, _) => {
                h41
            }
        };
        nat_eq(a42, a43)
    } else {
        false
    } {
        f5(v5.clone(), v10.clone())?
    } else {
        false
    } {
        f6(v8.clone(), v5.clone(), v6.clone(), v10.clone())?
    } else {
        false
    } {
        f7(v9.clone(), v5.clone(), v10.clone())?
    } else {
        false
    } {
        Ok(Adt3::C1(Adt2::C0(v5, v6, v7, v8, v9, v10, f8(v3))))
    } else {
        Ok(Adt3::C0)
    }
}

pub fn f2(v0: Adt2, v1: u64, v2: u64, v3: u64) -> R<Adt3> {
    let a47 = v1;
    let a48 = {
        let m46 = match v0.clone() {
            Adt2::C0(h45, _, _, _, _, _, _) => {
                h45
            }
        };
        match m46 {
            Adt1::C0(_, _, h44, _, _, _, _, _, _, _, _) => {
                h44
            }
        }
    };
    if if if nat_lt(a47, a48) {
        let a52 = v2;
        let a53 = {
            let m51 = match v0.clone() {
                Adt2::C0(h50, _, _, _, _, _, _) => {
                    h50
                }
            };
            match m51 {
                Adt1::C0(_, _, _, h49, _, _, _, _, _, _, _) => {
                    h49
                }
            }
        };
        nat_lt(a52, a53)
    } else {
        false
    } {
        let a57 = v3;
        let a58 = {
            let m56 = match v0.clone() {
                Adt2::C0(h55, _, _, _, _, _, _) => {
                    h55
                }
            };
            match m56 {
                Adt1::C0(_, _, _, _, h54, _, _, _, _, _, _) => {
                    h54
                }
            }
        };
        nat_lt(a57, a58)
    } else {
        false
    } {
        Ok(Adt3::C2(f9(match v0.clone() {
            Adt2::C0(h59, _, _, _, _, _, _) => {
                h59
            }
        }, v1, v2, v3)?, f9(match v0.clone() {
            Adt2::C0(_, h60, _, _, _, _, _) => {
                h60
            }
        }, 0u64, {
            let a70 = v2;
            let a71 = {
                let a67 = {
                    let m63 = match v0.clone() {
                        Adt2::C0(h62, _, _, _, _, _, _) => {
                            h62
                        }
                    };
                    match m63 {
                        Adt1::C0(_, _, _, h61, _, _, _, _, _, _, _) => {
                            h61
                        }
                    }
                };
                let a68 = {
                    let m66 = match v0.clone() {
                        Adt2::C0(_, h65, _, _, _, _, _) => {
                            h65
                        }
                    };
                    match m66 {
                        Adt1::C0(_, _, _, h64, _, _, _, _, _, _, _) => {
                            h64
                        }
                    }
                };
                let a69 = 0u64;
                nat_quot(a67, a68, a69)
            };
            let a72 = 0u64;
            nat_quot(a70, a71, a72)
        }, {
            let a82 = v3;
            let a83 = {
                let a79 = {
                    let m75 = match v0.clone() {
                        Adt2::C0(h74, _, _, _, _, _, _) => {
                            h74
                        }
                    };
                    match m75 {
                        Adt1::C0(_, _, _, _, h73, _, _, _, _, _, _) => {
                            h73
                        }
                    }
                };
                let a80 = {
                    let m78 = match v0.clone() {
                        Adt2::C0(_, h77, _, _, _, _, _) => {
                            h77
                        }
                    };
                    match m78 {
                        Adt1::C0(_, _, _, _, h76, _, _, _, _, _, _) => {
                            h76
                        }
                    }
                };
                let a81 = 0u64;
                nat_quot(a79, a80, a81)
            };
            let a84 = 0u64;
            nat_quot(a82, a83, a84)
        })?, f9(match v0.clone() {
            Adt2::C0(_, _, h85, _, _, _, _) => {
                h85
            }
        }, 0u64, {
            let a95 = v2;
            let a96 = {
                let a92 = {
                    let m88 = match v0.clone() {
                        Adt2::C0(h87, _, _, _, _, _, _) => {
                            h87
                        }
                    };
                    match m88 {
                        Adt1::C0(_, _, _, h86, _, _, _, _, _, _, _) => {
                            h86
                        }
                    }
                };
                let a93 = {
                    let m91 = match v0.clone() {
                        Adt2::C0(_, _, h90, _, _, _, _) => {
                            h90
                        }
                    };
                    match m91 {
                        Adt1::C0(_, _, _, h89, _, _, _, _, _, _, _) => {
                            h89
                        }
                    }
                };
                let a94 = 0u64;
                nat_quot(a92, a93, a94)
            };
            let a97 = 0u64;
            nat_quot(a95, a96, a97)
        }, {
            let a107 = v3;
            let a108 = {
                let a104 = {
                    let m100 = match v0.clone() {
                        Adt2::C0(h99, _, _, _, _, _, _) => {
                            h99
                        }
                    };
                    match m100 {
                        Adt1::C0(_, _, _, _, h98, _, _, _, _, _, _) => {
                            h98
                        }
                    }
                };
                let a105 = {
                    let m103 = match v0.clone() {
                        Adt2::C0(_, _, h102, _, _, _, _) => {
                            h102
                        }
                    };
                    match m103 {
                        Adt1::C0(_, _, _, _, h101, _, _, _, _, _, _) => {
                            h101
                        }
                    }
                };
                let a106 = 0u64;
                nat_quot(a104, a105, a106)
            };
            let a109 = 0u64;
            nat_quot(a107, a108, a109)
        })?, {
            let m111 = match v0.clone() {
                Adt2::C0(_, _, _, h110, _, _, _) => {
                    h110
                }
            };
            match m111 {
                None => {
                    0u64
                }
                Some(v4) => {
                    f9(v4.clone(), v1, {
                        let a113 = v2;
                        let a114 = match v4.clone() {
                            Adt1::C0(_, _, _, h112, _, _, _, _, _, _, _) => {
                                h112
                            }
                        };
                        let a115 = 0u64;
                        nat_rem(a113, a114, a115)
                    }, {
                        let a117 = v3;
                        let a118 = match v4 {
                            Adt1::C0(_, _, _, _, h116, _, _, _, _, _, _) => {
                                h116
                            }
                        };
                        let a119 = 0u64;
                        nat_rem(a117, a118, a119)
                    })?
                }
            }
        }, {
            let a120 = v2;
            let a121 = 4u64;
            nat_mul(a120, a121)?
        }, f9(match v0.clone() {
            Adt2::C0(_, _, _, _, _, h122, _) => {
                h122
            }
        }, v2, v1, v3)?, {
            let m125 = match v0.clone() {
                Adt2::C0(_, h124, _, _, _, _, _) => {
                    h124
                }
            };
            match m125 {
                Adt1::C0(_, _, _, _, _, _, h123, _, _, _, _) => {
                    h123
                }
            }
        }, {
            let m128 = match v0.clone() {
                Adt2::C0(_, _, h127, _, _, _, _) => {
                    h127
                }
            };
            match m128 {
                Adt1::C0(_, _, _, _, _, _, h126, _, _, _, _) => {
                    h126
                }
            }
        }, f10({
            let m131 = match v0 {
                Adt2::C0(_, h130, _, _, _, _, _) => {
                    h130
                }
            };
            match m131 {
                Adt1::C0(h129, _, _, _, _, _, _, _, _, _, _) => {
                    h129
                }
            }
        })))
    } else {
        Ok(Adt3::C0)
    }
}

pub fn f3(v0: Adt1) -> R<bool> {
    let a133 = match v0.clone() {
        Adt1::C0(h132, _, _, _, _, _, _, _, _, _, _) => {
            h132
        }
    };
    let a134 = 0u64;
    if if if if nat_eq(a133, a134) {
        let a136 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h135, _, _, _, _, _) => {
                h135
            }
        };
        let a137 = 4u64;
        nat_eq(a136, a137)
    } else {
        false
    } {
        let a143 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h138, _, _, _, _) => {
                h138
            }
        };
        let a144 = {
            let a141 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, h139, _, _, _, _, _) => {
                    h139
                }
            };
            let a142 = match v0.clone() {
                Adt1::C0(_, h140, _, _, _, _, _, _, _, _, _) => {
                    h140
                }
            };
            nat_mul(a141, a142)?
        };
        nat_eq(a143, a144)
    } else {
        false
    } {
        let a150 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h145, _, _, _) => {
                h145
            }
        };
        let a151 = {
            let a148 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h146, _, _, _, _) => {
                    h146
                }
            };
            let a149 = match v0.clone() {
                Adt1::C0(_, _, h147, _, _, _, _, _, _, _, _) => {
                    h147
                }
            };
            nat_mul(a148, a149)?
        };
        nat_eq(a150, a151)
    } else {
        false
    } {
        let a157 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h152, _, _) => {
                h152
            }
        };
        let a158 = {
            let a155 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h153, _, _, _) => {
                    h153
                }
            };
            let a156 = match v0 {
                Adt1::C0(_, _, _, h154, _, _, _, _, _, _, _) => {
                    h154
                }
            };
            nat_mul(a155, a156)?
        };
        Ok(nat_eq(a157, a158))
    } else {
        Ok(false)
    }
}

pub fn f4(v0: Adt1, v1: Adt1, v2: Adt1) -> R<bool> {
    if if if f11(match v0.clone() {
        Adt1::C0(h159, _, _, _, _, _, _, _, _, _, _) => {
            h159
        }
    }, match v0.clone() {
        Adt1::C0(_, h160, _, _, _, _, _, _, _, _, _) => {
            h160
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h161, _, _, _, _, _) => {
            h161
        }
    }) {
        f12(match v1.clone() {
            Adt1::C0(_, _, _, h162, _, _, _, _, _, _, _) => {
                h162
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h163, _, _, _, _, _, _, _) => {
                h163
            }
        })
    } else {
        false
    } {
        f12(match v1 {
            Adt1::C0(_, _, _, _, h164, _, _, _, _, _, _) => {
                h164
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h165, _, _, _, _, _, _) => {
                h165
            }
        })
    } else {
        false
    } {
        f5(v0, v2)
    } else {
        Ok(false)
    }
}

pub fn f5(v0: Adt1, v1: Adt1) -> R<bool> {
    let a167 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h166, _) => {
            h166
        }
    };
    let a168 = 0u64;
    if nat_eq(a167, a168) {
        Ok(true)
    } else {
        let a170 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h169, _) => {
                h169
            }
        };
        let a171 = 0u64;
        if nat_eq(a170, a171) {
            Ok(true)
        } else {
            let a173 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h172) => {
                    h172
                }
            };
            let a174 = 0u64;
            if nat_eq(a173, a174) {
                Ok(true)
            } else {
                let a176 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h175) => {
                        h175
                    }
                };
                let a177 = 0u64;
                if nat_eq(a176, a177) {
                    Ok(true)
                } else {
                    let a183 = {
                        let a180 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h178) => {
                                h178
                            }
                        };
                        let a181 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h179, _) => {
                                h179
                            }
                        };
                        nat_add(a180, a181)?
                    };
                    let a184 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h182) => {
                            h182
                        }
                    };
                    if nat_le(a183, a184) {
                        Ok(true)
                    } else {
                        let a190 = {
                            let a187 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h185) => {
                                    h185
                                }
                            };
                            let a188 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h186, _) => {
                                    h186
                                }
                            };
                            nat_add(a187, a188)?
                        };
                        let a191 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h189) => {
                                h189
                            }
                        };
                        Ok(nat_le(a190, a191))
                    }
                }
            }
        }
    }
}

pub fn f6(v0: Option<Adt1>, v1: Adt1, v2: Adt1, v3: Adt1) -> R<bool> {
    let m192 = v0;
    match m192 {
        None => {
            Ok(true)
        }
        Some(v4) => {
            let a194 = match v4.clone() {
                Adt1::C0(h193, _, _, _, _, _, _, _, _, _, _) => {
                    h193
                }
            };
            let a195 = 1u64;
            if if if if if if nat_eq(a194, a195) {
                let a197 = match v4.clone() {
                    Adt1::C0(_, _, _, _, _, h196, _, _, _, _, _) => {
                        h196
                    }
                };
                let a198 = 2u64;
                nat_eq(a197, a198)
            } else {
                false
            } {
                let a201 = match v2 {
                    Adt1::C0(_, _, h199, _, _, _, _, _, _, _, _) => {
                        h199
                    }
                };
                let a202 = match v4.clone() {
                    Adt1::C0(_, h200, _, _, _, _, _, _, _, _, _) => {
                        h200
                    }
                };
                nat_le(a201, a202)
            } else {
                false
            } {
                let a205 = match v1.clone() {
                    Adt1::C0(_, _, h203, _, _, _, _, _, _, _, _) => {
                        h203
                    }
                };
                let a206 = match v4.clone() {
                    Adt1::C0(_, _, h204, _, _, _, _, _, _, _, _) => {
                        h204
                    }
                };
                nat_le(a205, a206)
            } else {
                false
            } {
                f12(match v1.clone() {
                    Adt1::C0(_, _, _, h207, _, _, _, _, _, _, _) => {
                        h207
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, h208, _, _, _, _, _, _, _) => {
                        h208
                    }
                })
            } else {
                false
            } {
                f12(match v1 {
                    Adt1::C0(_, _, _, _, h209, _, _, _, _, _, _) => {
                        h209
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, _, h210, _, _, _, _, _, _) => {
                        h210
                    }
                })
            } else {
                false
            } {
                f5(v4, v3)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f7(v0: Option<Adt1>, v1: Adt1, v2: Adt1) -> R<bool> {
    let m211 = v0;
    match m211 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a213 = match v3.clone() {
                Adt1::C0(h212, _, _, _, _, _, _, _, _, _, _) => {
                    h212
                }
            };
            let a214 = 0u64;
            if if if if if if nat_eq(a213, a214) {
                let a216 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h215, _, _, _, _, _) => {
                        h215
                    }
                };
                let a217 = 4u64;
                nat_eq(a216, a217)
            } else {
                false
            } {
                let a220 = match v3.clone() {
                    Adt1::C0(_, h218, _, _, _, _, _, _, _, _, _) => {
                        h218
                    }
                };
                let a221 = match v1 {
                    Adt1::C0(_, _, _, h219, _, _, _, _, _, _, _) => {
                        h219
                    }
                };
                nat_eq(a220, a221)
            } else {
                false
            } {
                let a223 = match v3.clone() {
                    Adt1::C0(_, _, h222, _, _, _, _, _, _, _, _) => {
                        h222
                    }
                };
                let a224 = 1u64;
                nat_eq(a223, a224)
            } else {
                false
            } {
                let a226 = match v3.clone() {
                    Adt1::C0(_, _, _, h225, _, _, _, _, _, _, _) => {
                        h225
                    }
                };
                let a227 = 1u64;
                nat_eq(a226, a227)
            } else {
                false
            } {
                let a229 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h228, _, _, _, _, _, _) => {
                        h228
                    }
                };
                let a230 = 1u64;
                nat_eq(a229, a230)
            } else {
                false
            } {
                f5(v3, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f8(v0: u64) -> Option<u64> {
    let a231 = v0;
    let a232 = 0u64;
    if if nat_eq(a231, a232) {
        true
    } else {
        let a233 = v0;
        let a234 = 2147483648u64;
        nat_eq(a233, a234)
    } {
        None::<u64>
    } else {
        Some(v0)
    }
}

pub fn f9(v0: Adt1, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a246 = {
        let a241 = {
            let a236 = v1;
            let a237 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h235, _, _, _, _) => {
                    h235
                }
            };
            nat_mul(a236, a237)?
        };
        let a242 = {
            let a239 = v2;
            let a240 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h238, _, _, _) => {
                    h238
                }
            };
            nat_mul(a239, a240)?
        };
        nat_add(a241, a242)?
    };
    let a247 = {
        let a244 = v3;
        let a245 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h243, _, _) => {
                h243
            }
        };
        nat_mul(a244, a245)?
    };
    nat_add(a246, a247)
}

pub fn f10(v0: u64) -> u64 {
    if f13(v0) {
        8u64
    } else {
        f14(v0)
    }
}

pub fn f11(v0: u64, v1: u64, v2: u64) -> bool {
    let m248 = f15(v0);
    match m248 {
        Adt4::C0 => {
            false
        }
        Adt4::C1(v3) => {
            let a249 = v0;
            let a250 = 0u64;
            if if if if if nat_eq(a249, a250) {
                true
            } else {
                let a251 = v0;
                let a252 = 1u64;
                nat_eq(a251, a252)
            } {
                true
            } else {
                let a253 = v0;
                let a254 = 30u64;
                nat_eq(a253, a254)
            } {
                true
            } else {
                f13(v0)
            } {
                let a256 = v2;
                let a257 = match v3.clone() {
                    Adt5::C0(_, _, h255) => {
                        h255
                    }
                };
                nat_eq(a256, a257)
            } else {
                false
            } {
                f12(v1, match v3 {
                    Adt5::C0(_, h258, _) => {
                        h258
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f12(v0: u64, v1: u64) -> bool {
    let a259 = 0u64;
    let a260 = v1;
    if nat_lt(a259, a260) {
        let a264 = {
            let a261 = v0;
            let a262 = v1;
            let a263 = 0u64;
            nat_rem(a261, a262, a263)
        };
        let a265 = 0u64;
        nat_eq(a264, a265)
    } else {
        false
    }
}

pub fn f13(v0: u64) -> bool {
    let a266 = v0;
    let a267 = 2u64;
    if nat_eq(a266, a267) {
        true
    } else {
        let a268 = v0;
        let a269 = 8u64;
        nat_eq(a268, a269)
    }
}

pub fn f14(v0: u64) -> u64 {
    let a270 = v0;
    let a271 = 30u64;
    if nat_eq(a270, a271) {
        30u64
    } else {
        0u64
    }
}

pub fn f15(v0: u64) -> Adt4 {
    let a272 = v0;
    let a273 = 0u64;
    if nat_eq(a272, a273) {
        Adt4::C1(Adt5::C0(0u64, 1u64, 4u64))
    } else {
        let a274 = v0;
        let a275 = 1u64;
        if nat_eq(a274, a275) {
            Adt4::C1(Adt5::C0(1u64, 1u64, 2u64))
        } else {
            let a276 = v0;
            let a277 = 2u64;
            if nat_eq(a276, a277) {
                Adt4::C1(Adt5::C0(2u64, 32u64, 18u64))
            } else {
                let a278 = v0;
                let a279 = 3u64;
                if nat_eq(a278, a279) {
                    Adt4::C1(Adt5::C0(3u64, 32u64, 20u64))
                } else {
                    let a280 = v0;
                    let a281 = 6u64;
                    if nat_eq(a280, a281) {
                        Adt4::C1(Adt5::C0(6u64, 32u64, 22u64))
                    } else {
                        let a282 = v0;
                        let a283 = 7u64;
                        if nat_eq(a282, a283) {
                            Adt4::C1(Adt5::C0(7u64, 32u64, 24u64))
                        } else {
                            let a284 = v0;
                            let a285 = 8u64;
                            if nat_eq(a284, a285) {
                                Adt4::C1(Adt5::C0(8u64, 32u64, 34u64))
                            } else {
                                let a286 = v0;
                                let a287 = 9u64;
                                if nat_eq(a286, a287) {
                                    Adt4::C1(Adt5::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a288 = v0;
                                    let a289 = 10u64;
                                    if nat_eq(a288, a289) {
                                        Adt4::C1(Adt5::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a290 = v0;
                                        let a291 = 11u64;
                                        if nat_eq(a290, a291) {
                                            Adt4::C1(Adt5::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a292 = v0;
                                            let a293 = 12u64;
                                            if nat_eq(a292, a293) {
                                                Adt4::C1(Adt5::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a294 = v0;
                                                let a295 = 13u64;
                                                if nat_eq(a294, a295) {
                                                    Adt4::C1(Adt5::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a296 = v0;
                                                    let a297 = 14u64;
                                                    if nat_eq(a296, a297) {
                                                        Adt4::C1(Adt5::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a298 = v0;
                                                        let a299 = 15u64;
                                                        if nat_eq(a298, a299) {
                                                            Adt4::C1(Adt5::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a300 = v0;
                                                            let a301 = 16u64;
                                                            if nat_eq(a300, a301) {
                                                                Adt4::C1(Adt5::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a302 = v0;
                                                                let a303 = 17u64;
                                                                if nat_eq(a302, a303) {
                                                                    Adt4::C1(Adt5::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a304 = v0;
                                                                    let a305 = 18u64;
                                                                    if nat_eq(a304, a305) {
                                                                        Adt4::C1(Adt5::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a306 = v0;
                                                                        let a307 = 19u64;
                                                                        if nat_eq(a306, a307) {
                                                                            Adt4::C1(Adt5::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a308 = v0;
                                                                            let a309 = 20u64;
                                                                            if nat_eq(a308, a309) {
                                                                                Adt4::C1(Adt5::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a310 = v0;
                                                                                let a311 = 21u64;
                                                                                if nat_eq(a310, a311) {
                                                                                    Adt4::C1(Adt5::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a312 = v0;
                                                                                    let a313 = 22u64;
                                                                                    if nat_eq(a312, a313) {
                                                                                        Adt4::C1(Adt5::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a314 = v0;
                                                                                        let a315 = 23u64;
                                                                                        if nat_eq(a314, a315) {
                                                                                            Adt4::C1(Adt5::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a316 = v0;
                                                                                            let a317 = 24u64;
                                                                                            if nat_eq(a316, a317) {
                                                                                                Adt4::C1(Adt5::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a318 = v0;
                                                                                                let a319 = 25u64;
                                                                                                if nat_eq(a318, a319) {
                                                                                                    Adt4::C1(Adt5::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a320 = v0;
                                                                                                    let a321 = 26u64;
                                                                                                    if nat_eq(a320, a321) {
                                                                                                        Adt4::C1(Adt5::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a322 = v0;
                                                                                                        let a323 = 27u64;
                                                                                                        if nat_eq(a322, a323) {
                                                                                                            Adt4::C1(Adt5::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a324 = v0;
                                                                                                            let a325 = 28u64;
                                                                                                            if nat_eq(a324, a325) {
                                                                                                                Adt4::C1(Adt5::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a326 = v0;
                                                                                                                let a327 = 29u64;
                                                                                                                if nat_eq(a326, a327) {
                                                                                                                    Adt4::C1(Adt5::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a328 = v0;
                                                                                                                    let a329 = 30u64;
                                                                                                                    if nat_eq(a328, a329) {
                                                                                                                        Adt4::C1(Adt5::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a330 = v0;
                                                                                                                        let a331 = 34u64;
                                                                                                                        if nat_eq(a330, a331) {
                                                                                                                            Adt4::C1(Adt5::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a332 = v0;
                                                                                                                            let a333 = 35u64;
                                                                                                                            if nat_eq(a332, a333) {
                                                                                                                                Adt4::C1(Adt5::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a334 = v0;
                                                                                                                                let a335 = 39u64;
                                                                                                                                if nat_eq(a334, a335) {
                                                                                                                                    Adt4::C1(Adt5::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a336 = v0;
                                                                                                                                    let a337 = 40u64;
                                                                                                                                    if nat_eq(a336, a337) {
                                                                                                                                        Adt4::C1(Adt5::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a338 = v0;
                                                                                                                                        let a339 = 41u64;
                                                                                                                                        if nat_eq(a338, a339) {
                                                                                                                                            Adt4::C1(Adt5::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a340 = v0;
                                                                                                                                            let a341 = 42u64;
                                                                                                                                            if nat_eq(a340, a341) {
                                                                                                                                                Adt4::C1(Adt5::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a342 = v0;
                                                                                                                                                let a343 = 43u64;
                                                                                                                                                if nat_eq(a342, a343) {
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
