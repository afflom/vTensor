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
    C0(Adt1, Adt1, Adt1, Option<Adt1>, Option<Adt1>, Adt1),
}

#[derive(Clone)]
pub enum Adt3 {
    C0,
    C1(Adt2),
    C2(u64, u64, u64, u64, u64, u64, u64, u64),
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
    if if if if if if if if if if if if if if if if if if if nat_eq(a2, a3) {
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
        let a8 = v3;
        let a9 = 0u64;
        if nat_eq(a8, a9) {
            true
        } else {
            let a10 = v3;
            let a11 = 2147483648u64;
            nat_eq(a10, a11)
        }
    } else {
        false
    } {
        let a12 = v4;
        let a13 = 0u64;
        if if nat_eq(a12, a13) {
            true
        } else {
            let a14 = v4;
            let a15 = 10u64;
            nat_eq(a14, a15)
        } {
            true
        } else {
            let a16 = v4;
            let a17 = 15u64;
            nat_eq(a16, a17)
        }
    } else {
        false
    } {
        let a19 = match v5.clone() {
            Adt1::C0(h18, _, _, _, _, _, _, _, _, _, _) => {
                h18
            }
        };
        let a20 = 0u64;
        nat_eq(a19, a20)
    } else {
        false
    } {
        let a22 = match v5.clone() {
            Adt1::C0(_, _, _, _, _, h21, _, _, _, _, _) => {
                h21
            }
        };
        let a23 = 4u64;
        nat_eq(a22, a23)
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
        let a26 = match v5.clone() {
            Adt1::C0(_, h24, _, _, _, _, _, _, _, _, _) => {
                h24
            }
        };
        let a27 = match v6.clone() {
            Adt1::C0(_, h25, _, _, _, _, _, _, _, _, _) => {
                h25
            }
        };
        nat_eq(a26, a27)
    } else {
        false
    } {
        let a30 = match v6.clone() {
            Adt1::C0(_, _, h28, _, _, _, _, _, _, _, _) => {
                h28
            }
        };
        let a31 = match v7.clone() {
            Adt1::C0(_, _, h29, _, _, _, _, _, _, _, _) => {
                h29
            }
        };
        nat_eq(a30, a31)
    } else {
        false
    } {
        let a34 = match v10.clone() {
            Adt1::C0(_, h32, _, _, _, _, _, _, _, _, _) => {
                h32
            }
        };
        let a35 = match v7.clone() {
            Adt1::C0(_, h33, _, _, _, _, _, _, _, _, _) => {
                h33
            }
        };
        nat_eq(a34, a35)
    } else {
        false
    } {
        let a38 = match v10.clone() {
            Adt1::C0(_, _, h36, _, _, _, _, _, _, _, _) => {
                h36
            }
        };
        let a39 = match v5.clone() {
            Adt1::C0(_, _, _, h37, _, _, _, _, _, _, _) => {
                h37
            }
        };
        nat_eq(a38, a39)
    } else {
        false
    } {
        let a42 = match v10.clone() {
            Adt1::C0(_, _, _, h40, _, _, _, _, _, _, _) => {
                h40
            }
        };
        let a43 = match v5.clone() {
            Adt1::C0(_, _, h41, _, _, _, _, _, _, _, _) => {
                h41
            }
        };
        nat_eq(a42, a43)
    } else {
        false
    } {
        let a46 = match v10.clone() {
            Adt1::C0(_, _, _, _, h44, _, _, _, _, _, _) => {
                h44
            }
        };
        let a47 = match v5.clone() {
            Adt1::C0(_, _, _, _, h45, _, _, _, _, _, _) => {
                h45
            }
        };
        nat_eq(a46, a47)
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
        Ok(Adt3::C1(Adt2::C0(v5, v6, v7, v8, v9, v10)))
    } else {
        Ok(Adt3::C0)
    }
}

pub fn f2(v0: Adt2, v1: u64, v2: u64, v3: u64) -> R<Adt3> {
    let a51 = v1;
    let a52 = {
        let m50 = match v0.clone() {
            Adt2::C0(h49, _, _, _, _, _) => {
                h49
            }
        };
        match m50 {
            Adt1::C0(_, _, h48, _, _, _, _, _, _, _, _) => {
                h48
            }
        }
    };
    if if if nat_lt(a51, a52) {
        let a56 = v2;
        let a57 = {
            let m55 = match v0.clone() {
                Adt2::C0(h54, _, _, _, _, _) => {
                    h54
                }
            };
            match m55 {
                Adt1::C0(_, _, _, h53, _, _, _, _, _, _, _) => {
                    h53
                }
            }
        };
        nat_lt(a56, a57)
    } else {
        false
    } {
        let a61 = v3;
        let a62 = {
            let m60 = match v0.clone() {
                Adt2::C0(h59, _, _, _, _, _) => {
                    h59
                }
            };
            match m60 {
                Adt1::C0(_, _, _, _, h58, _, _, _, _, _, _) => {
                    h58
                }
            }
        };
        nat_lt(a61, a62)
    } else {
        false
    } {
        Ok(Adt3::C2(f8(match v0.clone() {
            Adt2::C0(h63, _, _, _, _, _) => {
                h63
            }
        }, v1, v2, v3)?, f8(match v0.clone() {
            Adt2::C0(_, h64, _, _, _, _) => {
                h64
            }
        }, 0u64, {
            let a74 = v2;
            let a75 = {
                let a71 = {
                    let m67 = match v0.clone() {
                        Adt2::C0(h66, _, _, _, _, _) => {
                            h66
                        }
                    };
                    match m67 {
                        Adt1::C0(_, _, _, h65, _, _, _, _, _, _, _) => {
                            h65
                        }
                    }
                };
                let a72 = {
                    let m70 = match v0.clone() {
                        Adt2::C0(_, h69, _, _, _, _) => {
                            h69
                        }
                    };
                    match m70 {
                        Adt1::C0(_, _, _, h68, _, _, _, _, _, _, _) => {
                            h68
                        }
                    }
                };
                let a73 = 0u64;
                nat_quot(a71, a72, a73)
            };
            let a76 = 0u64;
            nat_quot(a74, a75, a76)
        }, {
            let a86 = v3;
            let a87 = {
                let a83 = {
                    let m79 = match v0.clone() {
                        Adt2::C0(h78, _, _, _, _, _) => {
                            h78
                        }
                    };
                    match m79 {
                        Adt1::C0(_, _, _, _, h77, _, _, _, _, _, _) => {
                            h77
                        }
                    }
                };
                let a84 = {
                    let m82 = match v0.clone() {
                        Adt2::C0(_, h81, _, _, _, _) => {
                            h81
                        }
                    };
                    match m82 {
                        Adt1::C0(_, _, _, _, h80, _, _, _, _, _, _) => {
                            h80
                        }
                    }
                };
                let a85 = 0u64;
                nat_quot(a83, a84, a85)
            };
            let a88 = 0u64;
            nat_quot(a86, a87, a88)
        })?, f8(match v0.clone() {
            Adt2::C0(_, _, h89, _, _, _) => {
                h89
            }
        }, 0u64, {
            let a99 = v2;
            let a100 = {
                let a96 = {
                    let m92 = match v0.clone() {
                        Adt2::C0(h91, _, _, _, _, _) => {
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
                        Adt2::C0(_, _, h94, _, _, _) => {
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
                        Adt2::C0(h103, _, _, _, _, _) => {
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
                        Adt2::C0(_, _, h106, _, _, _) => {
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
        })?, {
            let m115 = match v0.clone() {
                Adt2::C0(_, _, _, h114, _, _) => {
                    h114
                }
            };
            match m115 {
                None => {
                    0u64
                }
                Some(v4) => {
                    f8(v4.clone(), v1, {
                        let a117 = v2;
                        let a118 = match v4.clone() {
                            Adt1::C0(_, _, _, h116, _, _, _, _, _, _, _) => {
                                h116
                            }
                        };
                        let a119 = 0u64;
                        nat_rem(a117, a118, a119)
                    }, {
                        let a121 = v3;
                        let a122 = match v4 {
                            Adt1::C0(_, _, _, _, h120, _, _, _, _, _, _) => {
                                h120
                            }
                        };
                        let a123 = 0u64;
                        nat_rem(a121, a122, a123)
                    })?
                }
            }
        }, {
            let a124 = v2;
            let a125 = 4u64;
            nat_mul(a124, a125)?
        }, f8(match v0.clone() {
            Adt2::C0(_, _, _, _, _, h126) => {
                h126
            }
        }, v2, v1, v3)?, {
            let m129 = match v0.clone() {
                Adt2::C0(_, h128, _, _, _, _) => {
                    h128
                }
            };
            match m129 {
                Adt1::C0(_, _, _, _, _, _, h127, _, _, _, _) => {
                    h127
                }
            }
        }, {
            let m132 = match v0 {
                Adt2::C0(_, _, h131, _, _, _) => {
                    h131
                }
            };
            match m132 {
                Adt1::C0(_, _, _, _, _, _, h130, _, _, _, _) => {
                    h130
                }
            }
        }))
    } else {
        Ok(Adt3::C0)
    }
}

pub fn f3(v0: Adt1) -> R<bool> {
    let a134 = match v0.clone() {
        Adt1::C0(h133, _, _, _, _, _, _, _, _, _, _) => {
            h133
        }
    };
    let a135 = 0u64;
    if if if if nat_eq(a134, a135) {
        let a137 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h136, _, _, _, _, _) => {
                h136
            }
        };
        let a138 = 4u64;
        nat_eq(a137, a138)
    } else {
        false
    } {
        let a144 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h139, _, _, _, _) => {
                h139
            }
        };
        let a145 = {
            let a142 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, h140, _, _, _, _, _) => {
                    h140
                }
            };
            let a143 = match v0.clone() {
                Adt1::C0(_, h141, _, _, _, _, _, _, _, _, _) => {
                    h141
                }
            };
            nat_mul(a142, a143)?
        };
        nat_eq(a144, a145)
    } else {
        false
    } {
        let a151 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h146, _, _, _) => {
                h146
            }
        };
        let a152 = {
            let a149 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h147, _, _, _, _) => {
                    h147
                }
            };
            let a150 = match v0.clone() {
                Adt1::C0(_, _, h148, _, _, _, _, _, _, _, _) => {
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
            Adt1::C0(_, _, _, _, _, _, _, _, h153, _, _) => {
                h153
            }
        };
        let a159 = {
            let a156 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h154, _, _, _) => {
                    h154
                }
            };
            let a157 = match v0 {
                Adt1::C0(_, _, _, h155, _, _, _, _, _, _, _) => {
                    h155
                }
            };
            nat_mul(a156, a157)?
        };
        Ok(nat_eq(a158, a159))
    } else {
        Ok(false)
    }
}

pub fn f4(v0: Adt1, v1: Adt1, v2: Adt1) -> R<bool> {
    let a161 = match v0.clone() {
        Adt1::C0(h160, _, _, _, _, _, _, _, _, _, _) => {
            h160
        }
    };
    let a162 = 1u64;
    if if if if if if nat_eq(a161, a162) {
        true
    } else {
        let a164 = match v0.clone() {
            Adt1::C0(h163, _, _, _, _, _, _, _, _, _, _) => {
                h163
            }
        };
        let a165 = 30u64;
        nat_eq(a164, a165)
    } {
        let a167 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h166, _, _, _, _, _) => {
                h166
            }
        };
        let a168 = 2u64;
        nat_eq(a167, a168)
    } else {
        false
    } {
        true
    } else {
        let a170 = match v0.clone() {
            Adt1::C0(h169, _, _, _, _, _, _, _, _, _, _) => {
                h169
            }
        };
        let a171 = 0u64;
        if nat_eq(a170, a171) {
            let a173 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, h172, _, _, _, _, _) => {
                    h172
                }
            };
            let a174 = 4u64;
            nat_eq(a173, a174)
        } else {
            false
        }
    } {
        f9(match v1.clone() {
            Adt1::C0(_, _, _, h175, _, _, _, _, _, _, _) => {
                h175
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h176, _, _, _, _, _, _, _) => {
                h176
            }
        })
    } else {
        false
    } {
        f9(match v1 {
            Adt1::C0(_, _, _, _, h177, _, _, _, _, _, _) => {
                h177
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h178, _, _, _, _, _, _) => {
                h178
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
    let a180 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h179, _) => {
            h179
        }
    };
    let a181 = 0u64;
    if nat_eq(a180, a181) {
        Ok(true)
    } else {
        let a183 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h182, _) => {
                h182
            }
        };
        let a184 = 0u64;
        if nat_eq(a183, a184) {
            Ok(true)
        } else {
            let a186 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h185) => {
                    h185
                }
            };
            let a187 = 0u64;
            if nat_eq(a186, a187) {
                Ok(true)
            } else {
                let a189 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h188) => {
                        h188
                    }
                };
                let a190 = 0u64;
                if nat_eq(a189, a190) {
                    Ok(true)
                } else {
                    let a196 = {
                        let a193 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h191) => {
                                h191
                            }
                        };
                        let a194 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h192, _) => {
                                h192
                            }
                        };
                        nat_add(a193, a194)?
                    };
                    let a197 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h195) => {
                            h195
                        }
                    };
                    if nat_le(a196, a197) {
                        Ok(true)
                    } else {
                        let a203 = {
                            let a200 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h198) => {
                                    h198
                                }
                            };
                            let a201 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h199, _) => {
                                    h199
                                }
                            };
                            nat_add(a200, a201)?
                        };
                        let a204 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h202) => {
                                h202
                            }
                        };
                        Ok(nat_le(a203, a204))
                    }
                }
            }
        }
    }
}

pub fn f6(v0: Option<Adt1>, v1: Adt1, v2: Adt1, v3: Adt1) -> R<bool> {
    let m205 = v0;
    match m205 {
        None => {
            Ok(true)
        }
        Some(v4) => {
            let a207 = match v4.clone() {
                Adt1::C0(h206, _, _, _, _, _, _, _, _, _, _) => {
                    h206
                }
            };
            let a208 = 1u64;
            if if if if if if nat_eq(a207, a208) {
                let a210 = match v4.clone() {
                    Adt1::C0(_, _, _, _, _, h209, _, _, _, _, _) => {
                        h209
                    }
                };
                let a211 = 2u64;
                nat_eq(a210, a211)
            } else {
                false
            } {
                let a214 = match v2 {
                    Adt1::C0(_, _, h212, _, _, _, _, _, _, _, _) => {
                        h212
                    }
                };
                let a215 = match v4.clone() {
                    Adt1::C0(_, h213, _, _, _, _, _, _, _, _, _) => {
                        h213
                    }
                };
                nat_le(a214, a215)
            } else {
                false
            } {
                let a218 = match v1.clone() {
                    Adt1::C0(_, _, h216, _, _, _, _, _, _, _, _) => {
                        h216
                    }
                };
                let a219 = match v4.clone() {
                    Adt1::C0(_, _, h217, _, _, _, _, _, _, _, _) => {
                        h217
                    }
                };
                nat_le(a218, a219)
            } else {
                false
            } {
                f9(match v1.clone() {
                    Adt1::C0(_, _, _, h220, _, _, _, _, _, _, _) => {
                        h220
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, h221, _, _, _, _, _, _, _) => {
                        h221
                    }
                })
            } else {
                false
            } {
                f9(match v1 {
                    Adt1::C0(_, _, _, _, h222, _, _, _, _, _, _) => {
                        h222
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, _, h223, _, _, _, _, _, _) => {
                        h223
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
    let m224 = v0;
    match m224 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a226 = match v3.clone() {
                Adt1::C0(h225, _, _, _, _, _, _, _, _, _, _) => {
                    h225
                }
            };
            let a227 = 0u64;
            if if if if if if nat_eq(a226, a227) {
                let a229 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h228, _, _, _, _, _) => {
                        h228
                    }
                };
                let a230 = 4u64;
                nat_eq(a229, a230)
            } else {
                false
            } {
                let a233 = match v3.clone() {
                    Adt1::C0(_, h231, _, _, _, _, _, _, _, _, _) => {
                        h231
                    }
                };
                let a234 = match v1 {
                    Adt1::C0(_, _, _, h232, _, _, _, _, _, _, _) => {
                        h232
                    }
                };
                nat_eq(a233, a234)
            } else {
                false
            } {
                let a236 = match v3.clone() {
                    Adt1::C0(_, _, h235, _, _, _, _, _, _, _, _) => {
                        h235
                    }
                };
                let a237 = 1u64;
                nat_eq(a236, a237)
            } else {
                false
            } {
                let a239 = match v3.clone() {
                    Adt1::C0(_, _, _, h238, _, _, _, _, _, _, _) => {
                        h238
                    }
                };
                let a240 = 1u64;
                nat_eq(a239, a240)
            } else {
                false
            } {
                let a242 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h241, _, _, _, _, _, _) => {
                        h241
                    }
                };
                let a243 = 1u64;
                nat_eq(a242, a243)
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

pub fn f8(v0: Adt1, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a255 = {
        let a250 = {
            let a245 = v1;
            let a246 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h244, _, _, _, _) => {
                    h244
                }
            };
            nat_mul(a245, a246)?
        };
        let a251 = {
            let a248 = v2;
            let a249 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h247, _, _, _) => {
                    h247
                }
            };
            nat_mul(a248, a249)?
        };
        nat_add(a250, a251)?
    };
    let a256 = {
        let a253 = v3;
        let a254 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h252, _, _) => {
                h252
            }
        };
        nat_mul(a253, a254)?
    };
    nat_add(a255, a256)
}

pub fn f9(v0: u64, v1: u64) -> bool {
    let a257 = 0u64;
    let a258 = v1;
    if nat_lt(a257, a258) {
        let a262 = {
            let a259 = v0;
            let a260 = v1;
            let a261 = 0u64;
            nat_rem(a259, a260, a261)
        };
        let a263 = 0u64;
        nat_eq(a262, a263)
    } else {
        false
    }
}

pub fn run(p0: Adt0) -> R<Adt3> {
    f0(p0)
}
