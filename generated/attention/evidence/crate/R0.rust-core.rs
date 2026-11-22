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
            let m132 = match v0.clone() {
                Adt2::C0(_, _, h131, _, _, _) => {
                    h131
                }
            };
            match m132 {
                Adt1::C0(_, _, _, _, _, _, h130, _, _, _, _) => {
                    h130
                }
            }
        }, f9({
            let m135 = match v0 {
                Adt2::C0(_, h134, _, _, _, _) => {
                    h134
                }
            };
            match m135 {
                Adt1::C0(h133, _, _, _, _, _, _, _, _, _, _) => {
                    h133
                }
            }
        })))
    } else {
        Ok(Adt3::C0)
    }
}

pub fn f3(v0: Adt1) -> R<bool> {
    let a137 = match v0.clone() {
        Adt1::C0(h136, _, _, _, _, _, _, _, _, _, _) => {
            h136
        }
    };
    let a138 = 0u64;
    if if if if nat_eq(a137, a138) {
        let a140 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h139, _, _, _, _, _) => {
                h139
            }
        };
        let a141 = 4u64;
        nat_eq(a140, a141)
    } else {
        false
    } {
        let a147 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h142, _, _, _, _) => {
                h142
            }
        };
        let a148 = {
            let a145 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, h143, _, _, _, _, _) => {
                    h143
                }
            };
            let a146 = match v0.clone() {
                Adt1::C0(_, h144, _, _, _, _, _, _, _, _, _) => {
                    h144
                }
            };
            nat_mul(a145, a146)?
        };
        nat_eq(a147, a148)
    } else {
        false
    } {
        let a154 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h149, _, _, _) => {
                h149
            }
        };
        let a155 = {
            let a152 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h150, _, _, _, _) => {
                    h150
                }
            };
            let a153 = match v0.clone() {
                Adt1::C0(_, _, h151, _, _, _, _, _, _, _, _) => {
                    h151
                }
            };
            nat_mul(a152, a153)?
        };
        nat_eq(a154, a155)
    } else {
        false
    } {
        let a161 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h156, _, _) => {
                h156
            }
        };
        let a162 = {
            let a159 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h157, _, _, _) => {
                    h157
                }
            };
            let a160 = match v0 {
                Adt1::C0(_, _, _, h158, _, _, _, _, _, _, _) => {
                    h158
                }
            };
            nat_mul(a159, a160)?
        };
        Ok(nat_eq(a161, a162))
    } else {
        Ok(false)
    }
}

pub fn f4(v0: Adt1, v1: Adt1, v2: Adt1) -> R<bool> {
    if if if f10(match v0.clone() {
        Adt1::C0(h163, _, _, _, _, _, _, _, _, _, _) => {
            h163
        }
    }, match v0.clone() {
        Adt1::C0(_, h164, _, _, _, _, _, _, _, _, _) => {
            h164
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h165, _, _, _, _, _) => {
            h165
        }
    }) {
        f11(match v1.clone() {
            Adt1::C0(_, _, _, h166, _, _, _, _, _, _, _) => {
                h166
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h167, _, _, _, _, _, _, _) => {
                h167
            }
        })
    } else {
        false
    } {
        f11(match v1 {
            Adt1::C0(_, _, _, _, h168, _, _, _, _, _, _) => {
                h168
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h169, _, _, _, _, _, _) => {
                h169
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
    let a171 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h170, _) => {
            h170
        }
    };
    let a172 = 0u64;
    if nat_eq(a171, a172) {
        Ok(true)
    } else {
        let a174 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h173, _) => {
                h173
            }
        };
        let a175 = 0u64;
        if nat_eq(a174, a175) {
            Ok(true)
        } else {
            let a177 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h176) => {
                    h176
                }
            };
            let a178 = 0u64;
            if nat_eq(a177, a178) {
                Ok(true)
            } else {
                let a180 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h179) => {
                        h179
                    }
                };
                let a181 = 0u64;
                if nat_eq(a180, a181) {
                    Ok(true)
                } else {
                    let a187 = {
                        let a184 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h182) => {
                                h182
                            }
                        };
                        let a185 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h183, _) => {
                                h183
                            }
                        };
                        nat_add(a184, a185)?
                    };
                    let a188 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h186) => {
                            h186
                        }
                    };
                    if nat_le(a187, a188) {
                        Ok(true)
                    } else {
                        let a194 = {
                            let a191 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h189) => {
                                    h189
                                }
                            };
                            let a192 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h190, _) => {
                                    h190
                                }
                            };
                            nat_add(a191, a192)?
                        };
                        let a195 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h193) => {
                                h193
                            }
                        };
                        Ok(nat_le(a194, a195))
                    }
                }
            }
        }
    }
}

pub fn f6(v0: Option<Adt1>, v1: Adt1, v2: Adt1, v3: Adt1) -> R<bool> {
    let m196 = v0;
    match m196 {
        None => {
            Ok(true)
        }
        Some(v4) => {
            let a198 = match v4.clone() {
                Adt1::C0(h197, _, _, _, _, _, _, _, _, _, _) => {
                    h197
                }
            };
            let a199 = 1u64;
            if if if if if if nat_eq(a198, a199) {
                let a201 = match v4.clone() {
                    Adt1::C0(_, _, _, _, _, h200, _, _, _, _, _) => {
                        h200
                    }
                };
                let a202 = 2u64;
                nat_eq(a201, a202)
            } else {
                false
            } {
                let a205 = match v2 {
                    Adt1::C0(_, _, h203, _, _, _, _, _, _, _, _) => {
                        h203
                    }
                };
                let a206 = match v4.clone() {
                    Adt1::C0(_, h204, _, _, _, _, _, _, _, _, _) => {
                        h204
                    }
                };
                nat_le(a205, a206)
            } else {
                false
            } {
                let a209 = match v1.clone() {
                    Adt1::C0(_, _, h207, _, _, _, _, _, _, _, _) => {
                        h207
                    }
                };
                let a210 = match v4.clone() {
                    Adt1::C0(_, _, h208, _, _, _, _, _, _, _, _) => {
                        h208
                    }
                };
                nat_le(a209, a210)
            } else {
                false
            } {
                f11(match v1.clone() {
                    Adt1::C0(_, _, _, h211, _, _, _, _, _, _, _) => {
                        h211
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, h212, _, _, _, _, _, _, _) => {
                        h212
                    }
                })
            } else {
                false
            } {
                f11(match v1 {
                    Adt1::C0(_, _, _, _, h213, _, _, _, _, _, _) => {
                        h213
                    }
                }, match v4.clone() {
                    Adt1::C0(_, _, _, _, h214, _, _, _, _, _, _) => {
                        h214
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
    let m215 = v0;
    match m215 {
        None => {
            Ok(true)
        }
        Some(v3) => {
            let a217 = match v3.clone() {
                Adt1::C0(h216, _, _, _, _, _, _, _, _, _, _) => {
                    h216
                }
            };
            let a218 = 0u64;
            if if if if if if nat_eq(a217, a218) {
                let a220 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h219, _, _, _, _, _) => {
                        h219
                    }
                };
                let a221 = 4u64;
                nat_eq(a220, a221)
            } else {
                false
            } {
                let a224 = match v3.clone() {
                    Adt1::C0(_, h222, _, _, _, _, _, _, _, _, _) => {
                        h222
                    }
                };
                let a225 = match v1 {
                    Adt1::C0(_, _, _, h223, _, _, _, _, _, _, _) => {
                        h223
                    }
                };
                nat_eq(a224, a225)
            } else {
                false
            } {
                let a227 = match v3.clone() {
                    Adt1::C0(_, _, h226, _, _, _, _, _, _, _, _) => {
                        h226
                    }
                };
                let a228 = 1u64;
                nat_eq(a227, a228)
            } else {
                false
            } {
                let a230 = match v3.clone() {
                    Adt1::C0(_, _, _, h229, _, _, _, _, _, _, _) => {
                        h229
                    }
                };
                let a231 = 1u64;
                nat_eq(a230, a231)
            } else {
                false
            } {
                let a233 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h232, _, _, _, _, _, _) => {
                        h232
                    }
                };
                let a234 = 1u64;
                nat_eq(a233, a234)
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

pub fn f9(v0: u64) -> u64 {
    if f12(v0) {
        8u64
    } else {
        f13(v0)
    }
}

pub fn f10(v0: u64, v1: u64, v2: u64) -> bool {
    let m248 = f14(v0);
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
                f12(v0)
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
                f11(v1, match v3 {
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

pub fn f11(v0: u64, v1: u64) -> bool {
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

pub fn f12(v0: u64) -> bool {
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

pub fn f13(v0: u64) -> u64 {
    let a270 = v0;
    let a271 = 30u64;
    if nat_eq(a270, a271) {
        30u64
    } else {
        0u64
    }
}

pub fn f14(v0: u64) -> Adt4 {
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
