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
    C0(u64, Option<Adt1>, Option<Adt1>, Adt1),
    C1(u64, Adt2, u64),
    C2(u64, bool, Adt1, Adt1, Adt1, Adt1),
    C3(u64, u64, u64, u64, bool, Adt1, Option<Adt1>, Adt1),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64, u64, u64, u64, u64, Adt1, Adt1, Adt1),
}

#[derive(Clone)]
pub enum Adt3 {
    C0,
    C1,
    C2(Adt2),
    C3(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
    C4(u64, u64, u64, u64, u64),
    C5(u64, u64, u64, u64, bool),
}

pub fn f0(v0: Adt0) -> R<Adt3> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1, v2, v3, v4) => {
            let m2 = f1(v1);
            match m2 {
                None => {
                    Ok(Adt3::C0)
                }
                Some(v5) => {
                    let m3 = v2;
                    match m3 {
                        None => {
                            Ok(Adt3::C1)
                        }
                        Some(v6) => {
                            let m4 = v3;
                            match m4 {
                                None => {
                                    Ok(Adt3::C1)
                                }
                                Some(v7) => {
                                    f2(v5, v6, v7, v4)
                                }
                            }
                        }
                    }
                }
            }
        }
        Adt0::C1(v24, v25, v26) => {
            let a16 = match v25.clone() {
                Adt2::C0(_, _, h15, _, _, _, _, _) => {
                    h15
                }
            };
            let a17 = 66u64;
            if if nat_eq(a16, a17) {
                true
            } else {
                let a19 = match v25.clone() {
                    Adt2::C0(_, _, h18, _, _, _, _, _) => {
                        h18
                    }
                };
                let a20 = 67u64;
                nat_eq(a19, a20)
            } {
                f7(v24, v25)
            } else {
                let a22 = match v25.clone() {
                    Adt2::C0(_, _, h21, _, _, _, _, _) => {
                        h21
                    }
                };
                let a23 = 41u64;
                if nat_eq(a22, a23) {
                    f8(v24, v25, v26)
                } else {
                    let a25 = 98u64;
                    let a26 = match v25.clone() {
                        Adt2::C0(_, _, h24, _, _, _, _, _) => {
                            h24
                        }
                    };
                    if nat_le(a25, a26) {
                        let a28 = match v25.clone() {
                            Adt2::C0(_, _, _, h27, _, _, _, _) => {
                                h27
                            }
                        };
                        let a29 = 6u64;
                        if nat_eq(a28, a29) {
                            f9(v24, v25, v26)
                        } else {
                            f10(v24, v25)
                        }
                    } else {
                        f11(v24, v25, v26)
                    }
                }
            }
        }
        Adt0::C2(v8, v9, v10, v11, v12, v13) => {
            let a5 = v8;
            let a6 = 41u64;
            if nat_eq(a5, a6) {
                if v9 {
                    Ok(Adt3::C1)
                } else {
                    f3(v10, v11, v12, v13)
                }
            } else {
                Ok(Adt3::C0)
            }
        }
        Adt0::C3(v14, v15, v16, v17, v18, v19, v20, v21) => {
            let a7 = v14;
            let a8 = 66u64;
            if if nat_eq(a7, a8) {
                true
            } else {
                let a9 = v14;
                let a10 = 67u64;
                nat_eq(a9, a10)
            } {
                if v18 {
                    Ok(Adt3::C1)
                } else {
                    let m11 = v20;
                    match m11 {
                        None => {
                            f4(v14, v15, v19, v21)
                        }
                        Some(_) => {
                            Ok(Adt3::C1)
                        }
                    }
                }
            } else {
                let a12 = v14;
                let a13 = 98u64;
                if nat_eq(a12, a13) {
                    if if f5(v15) {
                        !v18
                    } else {
                        false
                    } {
                        let m14 = v20;
                        match m14 {
                            None => {
                                f6(v15, false, v16, v17, v19.clone(), v19, v21)
                            }
                            Some(v23) => {
                                f6(v15, true, v16, v17, v19, v23, v21)
                            }
                        }
                    } else {
                        Ok(Adt3::C1)
                    }
                } else {
                    Ok(Adt3::C0)
                }
            }
        }
    }
}

pub fn f1(v0: u64) -> Option<u64> {
    let a30 = v0;
    let a31 = 2u64;
    if nat_eq(a30, a31) {
        Some(0u64)
    } else {
        let a32 = v0;
        let a33 = 5u64;
        if nat_eq(a32, a33) {
            Some(1u64)
        } else {
            let a34 = v0;
            let a35 = 6u64;
            if nat_eq(a34, a35) {
                Some(2u64)
            } else {
                let a36 = v0;
                let a37 = 7u64;
                if nat_eq(a36, a37) {
                    Some(3u64)
                } else {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f2(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if if f12(v1.clone(), v3.clone()) {
        f13(v1.clone(), v2.clone())
    } else {
        false
    } {
        f14(match v1.clone() {
            Adt1::C0(h38, _, _, _, _, _, _, _, _, _, _) => {
                h38
            }
        }, match v2.clone() {
            Adt1::C0(h39, _, _, _, _, _, _, _, _, _, _) => {
                h39
            }
        }, match v3.clone() {
            Adt1::C0(h40, _, _, _, _, _, _, _, _, _, _) => {
                h40
            }
        })
    } else {
        false
    } {
        if f15(v1.clone()) {
            true
        } else {
            let a43 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, h41, _, _, _, _, _) => {
                    h41
                }
            };
            let a44 = f16(match v1.clone() {
                Adt1::C0(h42, _, _, _, _, _, _, _, _, _, _) => {
                    h42
                }
            });
            if if if if if if if nat_eq(a43, a44) {
                let a47 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h45, _, _, _, _, _) => {
                        h45
                    }
                };
                let a48 = f16(match v3.clone() {
                    Adt1::C0(h46, _, _, _, _, _, _, _, _, _, _) => {
                        h46
                    }
                });
                nat_eq(a47, a48)
            } else {
                false
            } {
                f17(v1.clone())
            } else {
                false
            } {
                f17(v2.clone())
            } else {
                false
            } {
                f17(v3.clone())
            } else {
                false
            } {
                f18(v1.clone(), v2.clone())?
            } else {
                false
            } {
                f19(v1.clone(), v3.clone())?
            } else {
                false
            } {
                f19(v2.clone(), v3.clone())?
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v4: u64 = if if f15(v1.clone()) {
            true
        } else {
            f20(v3.clone())?
        } {
            0u64
        } else {
            let a51 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h49, _) => {
                    h49
                }
            };
            let a52 = f16(match v3.clone() {
                Adt1::C0(h50, _, _, _, _, _, _, _, _, _, _) => {
                    h50
                }
            });
            let a53 = 0u64;
            nat_quot(a51, a52, a53)
        };
        Ok(Adt3::C2(Adt2::C0(f21(f22(v1.clone())?, v4, f23(match v1.clone() {
            Adt1::C0(h54, _, _, _, _, _, _, _, _, _, _) => {
                h54
            }
        }, match v2.clone() {
            Adt1::C0(h55, _, _, _, _, _, _, _, _, _, _) => {
                h55
            }
        }, match v3.clone() {
            Adt1::C0(h56, _, _, _, _, _, _, _, _, _, _) => {
                h56
            }
        })?)?, {
            let a57 = 3u64;
            let a58 = v4;
            nat_add(a57, a58)?
        }, v0, 0u64, 0u64, v1, v2, v3)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f3(v0: Adt1, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if if if if if if if if if if if if if f24(v2, v3.clone()) {
        let a60 = match v0.clone() {
            Adt1::C0(h59, _, _, _, _, _, _, _, _, _, _) => {
                h59
            }
        };
        let a61 = 0u64;
        if nat_eq(a60, a61) {
            true
        } else {
            let a63 = match v0.clone() {
                Adt1::C0(h62, _, _, _, _, _, _, _, _, _, _) => {
                    h62
                }
            };
            let a64 = 1u64;
            nat_eq(a63, a64)
        }
    } else {
        false
    } {
        let a66 = 0u64;
        let a67 = f25(match v3.clone() {
            Adt1::C0(h65, _, _, _, _, _, _, _, _, _, _) => {
                h65
            }
        });
        nat_lt(a66, a67)
    } else {
        false
    } {
        let a69 = 0u64;
        let a70 = f26(match v1.clone() {
            Adt1::C0(h68, _, _, _, _, _, _, _, _, _, _) => {
                h68
            }
        });
        nat_lt(a69, a70)
    } else {
        false
    } {
        let a73 = match v0.clone() {
            Adt1::C0(_, h71, _, _, _, _, _, _, _, _, _) => {
                h71
            }
        };
        let a74 = match v3.clone() {
            Adt1::C0(_, h72, _, _, _, _, _, _, _, _, _) => {
                h72
            }
        };
        nat_eq(a73, a74)
    } else {
        false
    } {
        let a77 = match v0.clone() {
            Adt1::C0(_, _, _, h75, _, _, _, _, _, _, _) => {
                h75
            }
        };
        let a78 = match v3.clone() {
            Adt1::C0(_, _, _, h76, _, _, _, _, _, _, _) => {
                h76
            }
        };
        nat_eq(a77, a78)
    } else {
        false
    } {
        let a81 = match v0.clone() {
            Adt1::C0(_, _, _, _, h79, _, _, _, _, _, _) => {
                h79
            }
        };
        let a82 = match v3.clone() {
            Adt1::C0(_, _, _, _, h80, _, _, _, _, _, _) => {
                h80
            }
        };
        nat_eq(a81, a82)
    } else {
        false
    } {
        let a85 = match v0.clone() {
            Adt1::C0(_, _, h83, _, _, _, _, _, _, _, _) => {
                h83
            }
        };
        let a86 = match v1.clone() {
            Adt1::C0(_, h84, _, _, _, _, _, _, _, _, _) => {
                h84
            }
        };
        nat_eq(a85, a86)
    } else {
        false
    } {
        let a88 = match v1.clone() {
            Adt1::C0(_, _, _, _, h87, _, _, _, _, _, _) => {
                h87
            }
        };
        let a89 = 1u64;
        nat_eq(a88, a89)
    } else {
        false
    } {
        let a91 = 0u64;
        let a92 = match v1.clone() {
            Adt1::C0(_, _, h90, _, _, _, _, _, _, _, _) => {
                h90
            }
        };
        nat_lt(a91, a92)
    } else {
        false
    } {
        let a94 = 0u64;
        let a95 = match v1.clone() {
            Adt1::C0(_, _, _, h93, _, _, _, _, _, _, _) => {
                h93
            }
        };
        nat_lt(a94, a95)
    } else {
        false
    } {
        let a101 = {
            let a98 = match v0.clone() {
                Adt1::C0(_, _, _, h96, _, _, _, _, _, _, _) => {
                    h96
                }
            };
            let a99 = match v1.clone() {
                Adt1::C0(_, _, h97, _, _, _, _, _, _, _, _) => {
                    h97
                }
            };
            let a100 = 0u64;
            nat_rem(a98, a99, a100)
        };
        let a102 = 0u64;
        nat_eq(a101, a102)
    } else {
        false
    } {
        let a108 = {
            let a105 = match v0.clone() {
                Adt1::C0(_, _, _, _, h103, _, _, _, _, _, _) => {
                    h103
                }
            };
            let a106 = match v1.clone() {
                Adt1::C0(_, _, _, h104, _, _, _, _, _, _, _) => {
                    h104
                }
            };
            let a107 = 0u64;
            nat_rem(a105, a106, a107)
        };
        let a109 = 0u64;
        nat_eq(a108, a109)
    } else {
        false
    } {
        f27(v0.clone())
    } else {
        false
    } {
        let a111 = match v3.clone() {
            Adt1::C0(_, h110, _, _, _, _, _, _, _, _, _) => {
                h110
            }
        };
        let a112 = 1u64;
        if nat_eq(a111, a112) {
            true
        } else {
            let a115 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h113, _, _, _, _, _) => {
                    h113
                }
            };
            let a116 = f25(match v3.clone() {
                Adt1::C0(h114, _, _, _, _, _, _, _, _, _, _) => {
                    h114
                }
            });
            nat_eq(a115, a116)
        }
    } else {
        false
    } {
        let v4: u64 = f22(v1.clone())?;
        let v5: u64 = {
            let a122 = match v0.clone() {
                Adt1::C0(_, _, h117, _, _, _, _, _, _, _, _) => {
                    h117
                }
            };
            let a123 = {
                let a120 = match v0.clone() {
                    Adt1::C0(_, _, _, h118, _, _, _, _, _, _, _) => {
                        h118
                    }
                };
                let a121 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h119, _, _, _, _, _, _) => {
                        h119
                    }
                };
                nat_mul(a120, a121)?
            };
            nat_mul(a122, a123)?
        };
        let v6: u64 = if f15(v0.clone()) {
            0u64
        } else {
            let a135 = {
                let a124 = 2u64;
                let a125 = v4;
                nat_mul(a124, a125)?
            };
            let a136 = {
                let a133 = v5;
                let a134 = {
                    let a131 = 1u64;
                    let a132 = {
                        let a129 = match v0.clone() {
                            Adt1::C0(_, h126, _, _, _, _, _, _, _, _, _) => {
                                h126
                            }
                        };
                        let a130 = f28(match v0.clone() {
                            Adt1::C0(h127, _, _, _, _, _, _, _, _, _, _) => {
                                h127
                            }
                        }, match v3.clone() {
                            Adt1::C0(h128, _, _, _, _, _, _, _, _, _, _) => {
                                h128
                            }
                        });
                        nat_mul(a129, a130)?
                    };
                    nat_add(a131, a132)?
                };
                nat_mul(a133, a134)?
            };
            nat_add(a135, a136)?
        };
        Ok(Adt3::C2(Adt2::C0(v6, if f15(v0.clone()) {
            3u64
        } else {
            let a137 = 3u64;
            let a138 = v4;
            nat_add(a137, a138)?
        }, 41u64, 0u64, 0u64, v0, v1, v3)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f4(v0: u64, v1: u64, v2: Adt1, v3: Adt1) -> R<Adt3> {
    let a140 = match v2.clone() {
        Adt1::C0(h139, _, _, _, _, _, _, _, _, _, _) => {
            h139
        }
    };
    let a141 = 0u64;
    if if if if if if if if if if if nat_eq(a140, a141) {
        let a143 = match v3.clone() {
            Adt1::C0(h142, _, _, _, _, _, _, _, _, _, _) => {
                h142
            }
        };
        let a144 = 26u64;
        nat_eq(a143, a144)
    } else {
        false
    } {
        let a146 = match v2.clone() {
            Adt1::C0(_, h145, _, _, _, _, _, _, _, _, _) => {
                h145
            }
        };
        let a147 = 2147483647u64;
        nat_le(a146, a147)
    } else {
        false
    } {
        let a150 = match v3.clone() {
            Adt1::C0(_, h148, _, _, _, _, _, _, _, _, _) => {
                h148
            }
        };
        let a151 = match v2.clone() {
            Adt1::C0(_, h149, _, _, _, _, _, _, _, _, _) => {
                h149
            }
        };
        nat_le(a150, a151)
    } else {
        false
    } {
        let a154 = match v2.clone() {
            Adt1::C0(_, _, h152, _, _, _, _, _, _, _, _) => {
                h152
            }
        };
        let a155 = match v3.clone() {
            Adt1::C0(_, _, h153, _, _, _, _, _, _, _, _) => {
                h153
            }
        };
        nat_eq(a154, a155)
    } else {
        false
    } {
        let a158 = match v2.clone() {
            Adt1::C0(_, _, _, h156, _, _, _, _, _, _, _) => {
                h156
            }
        };
        let a159 = match v3.clone() {
            Adt1::C0(_, _, _, h157, _, _, _, _, _, _, _) => {
                h157
            }
        };
        nat_eq(a158, a159)
    } else {
        false
    } {
        let a162 = match v2.clone() {
            Adt1::C0(_, _, _, _, h160, _, _, _, _, _, _) => {
                h160
            }
        };
        let a163 = match v3.clone() {
            Adt1::C0(_, _, _, _, h161, _, _, _, _, _, _) => {
                h161
            }
        };
        nat_eq(a162, a163)
    } else {
        false
    } {
        let a164 = v0;
        let a165 = 67u64;
        if nat_eq(a164, a165) {
            true
        } else {
            let a166 = v1;
            let a167 = 1u64;
            if nat_le(a166, a167) {
                let a170 = match v3.clone() {
                    Adt1::C0(_, h168, _, _, _, _, _, _, _, _, _) => {
                        h168
                    }
                };
                let a171 = match v2.clone() {
                    Adt1::C0(_, h169, _, _, _, _, _, _, _, _, _) => {
                        h169
                    }
                };
                nat_eq(a170, a171)
            } else {
                false
            }
        }
    } else {
        false
    } {
        f29(v2.clone())?
    } else {
        false
    } {
        f29(v3.clone())?
    } else {
        false
    } {
        f30(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a177 = match v3.clone() {
                Adt1::C0(_, _, h172, _, _, _, _, _, _, _, _) => {
                    h172
                }
            };
            let a178 = {
                let a175 = match v3.clone() {
                    Adt1::C0(_, _, _, h173, _, _, _, _, _, _, _) => {
                        h173
                    }
                };
                let a176 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h174, _, _, _, _, _, _) => {
                        h174
                    }
                };
                nat_mul(a175, a176)?
            };
            nat_mul(a177, a178)?
        }, 3u64, v0, {
            let a179 = v0;
            let a180 = 67u64;
            if nat_eq(a179, a180) {
                1u64
            } else {
                v1
            }
        }, 0u64, v2.clone(), v2, v3)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f5(v0: u64) -> bool {
    let a181 = v0;
    let a182 = 2u64;
    if if nat_le(a181, a182) {
        true
    } else {
        let a183 = v0;
        let a184 = 5u64;
        nat_eq(a183, a184)
    } {
        true
    } else {
        let a185 = v0;
        let a186 = 6u64;
        nat_eq(a185, a186)
    }
}

pub fn f6(v0: u64, v1: bool, v2: u64, v3: u64, v4: Adt1, v5: Adt1, v6: Adt1) -> R<Adt3> {
    let a187 = v2;
    let a188 = 1u64;
    if if if if if if if if if if if if if if if nat_le(a187, a188) {
        let a190 = 0u64;
        let a191 = f16(match v4.clone() {
            Adt1::C0(h189, _, _, _, _, _, _, _, _, _, _) => {
                h189
            }
        });
        nat_lt(a190, a191)
    } else {
        false
    } {
        let a194 = match v4.clone() {
            Adt1::C0(h192, _, _, _, _, _, _, _, _, _, _) => {
                h192
            }
        };
        let a195 = match v5.clone() {
            Adt1::C0(h193, _, _, _, _, _, _, _, _, _, _) => {
                h193
            }
        };
        nat_eq(a194, a195)
    } else {
        false
    } {
        let a198 = match v4.clone() {
            Adt1::C0(h196, _, _, _, _, _, _, _, _, _, _) => {
                h196
            }
        };
        let a199 = match v6.clone() {
            Adt1::C0(h197, _, _, _, _, _, _, _, _, _, _) => {
                h197
            }
        };
        nat_eq(a198, a199)
    } else {
        false
    } {
        f12(v4.clone(), v5.clone())
    } else {
        false
    } {
        f31(v4.clone())?
    } else {
        false
    } {
        f31(v5.clone())?
    } else {
        false
    } {
        f31(v6.clone())?
    } else {
        false
    } {
        let a202 = match v4.clone() {
            Adt1::C0(_, _, h200, _, _, _, _, _, _, _, _) => {
                h200
            }
        };
        let a203 = match v6.clone() {
            Adt1::C0(_, _, h201, _, _, _, _, _, _, _, _) => {
                h201
            }
        };
        nat_eq(a202, a203)
    } else {
        false
    } {
        let a206 = match v4.clone() {
            Adt1::C0(_, _, _, h204, _, _, _, _, _, _, _) => {
                h204
            }
        };
        let a207 = match v6.clone() {
            Adt1::C0(_, _, _, h205, _, _, _, _, _, _, _) => {
                h205
            }
        };
        nat_eq(a206, a207)
    } else {
        false
    } {
        let a210 = match v4.clone() {
            Adt1::C0(_, _, _, _, h208, _, _, _, _, _, _) => {
                h208
            }
        };
        let a211 = match v6.clone() {
            Adt1::C0(_, _, _, _, h209, _, _, _, _, _, _) => {
                h209
            }
        };
        nat_eq(a210, a211)
    } else {
        false
    } {
        let a218 = match v6.clone() {
            Adt1::C0(_, h212, _, _, _, _, _, _, _, _, _) => {
                h212
            }
        };
        let a219 = if v1 {
            match v4.clone() {
                Adt1::C0(_, h213, _, _, _, _, _, _, _, _, _) => {
                    h213
                }
            }
        } else {
            let a215 = match v4.clone() {
                Adt1::C0(_, h214, _, _, _, _, _, _, _, _, _) => {
                    h214
                }
            };
            let a216 = 2u64;
            let a217 = 0u64;
            nat_quot(a215, a216, a217)
        };
        nat_eq(a218, a219)
    } else {
        false
    } {
        let a221 = match v6.clone() {
            Adt1::C0(_, h220, _, _, _, _, _, _, _, _, _) => {
                h220
            }
        };
        let a222 = 2147483647u64;
        nat_le(a221, a222)
    } else {
        false
    } {
        f19(v4.clone(), v6.clone())?
    } else {
        false
    } {
        f19(v5.clone(), v6.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a223 = v0;
            let a224 = 6u64;
            if nat_eq(a223, a224) {
                let a227 = 1u64;
                let a228 = {
                    let a225 = 10u64;
                    let a226 = f22(v6.clone())?;
                    nat_mul(a225, a226)?
                };
                nat_add(a227, a228)?
            } else {
                let a229 = 4u64;
                let a230 = f22(v6.clone())?;
                nat_mul(a229, a230)?
            }
        }, {
            let a231 = v0;
            let a232 = 6u64;
            if nat_eq(a231, a232) {
                4u64
            } else {
                3u64
            }
        }, if v1 {
            98u64
        } else {
            let a233 = 99u64;
            let a234 = v2;
            nat_add(a233, a234)?
        }, v0, v3, v4, v5, v6)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f7(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, h235, _, _) => {
            h235
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h236) => {
            h236
        }
    };
    let a238 = v0;
    let a239 = match v1.clone() {
        Adt2::C0(h237, _, _, _, _, _, _, _) => {
            h237
        }
    };
    if nat_lt(a238, a239) {
        Ok(Adt3::C5({
            let a241 = v0;
            let a242 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, _, h240, _, _, _, _) => {
                    h240
                }
            };
            nat_mul(a241, a242)?
        }, {
            let a244 = v0;
            let a245 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h243, _, _, _, _) => {
                    h243
                }
            };
            nat_mul(a244, a245)?
        }, match v2 {
            Adt1::C0(_, h246, _, _, _, _, _, _, _, _, _) => {
                h246
            }
        }, match v3 {
            Adt1::C0(_, h247, _, _, _, _, _, _, _, _, _) => {
                h247
            }
        }, {
            let a249 = match v1 {
                Adt2::C0(_, _, _, h248, _, _, _, _) => {
                    h248
                }
            };
            let a250 = 1u64;
            nat_eq(a249, a250)
        }))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f8(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let a252 = v0;
    let a253 = match v1.clone() {
        Adt2::C0(h251, _, _, _, _, _, _, _) => {
            h251
        }
    };
    if nat_lt(a252, a253) {
        let v3: u64 = {
            let a255 = 2u64;
            let a256 = f22(match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, h254, _) => {
                    h254
                }
            })?;
            nat_mul(a255, a256)?
        };
        let a257 = v0;
        let a258 = v3;
        if nat_lt(a257, a258) {
            f32(v0, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, h259, _) => {
                    h259
                }
            }, {
                let m262 = match v1 {
                    Adt2::C0(_, _, _, _, _, _, _, h261) => {
                        h261
                    }
                };
                match m262 {
                    Adt1::C0(_, _, h260, _, _, _, _, _, _, _, _) => {
                        h260
                    }
                }
            }, v2)
        } else {
            f33({
                let a263 = v0;
                let a264 = v3;
                nat_sub(a263, a264)
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, h265, _, _) => {
                    h265
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, h266, _) => {
                    h266
                }
            }, match v1 {
                Adt2::C0(_, _, _, _, _, _, _, h267) => {
                    h267
                }
            }, v2)
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f9(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(_, _, _, _, h268, _, _, _) => {
            h268
        }
    };
    let a270 = v0;
    let a271 = match v1.clone() {
        Adt2::C0(h269, _, _, _, _, _, _, _) => {
            h269
        }
    };
    if nat_lt(a270, a271) {
        let a272 = v0;
        let a273 = 0u64;
        if nat_eq(a272, a273) {
            Ok(Adt3::C3(1u64, 6u64, 0u64, 0u64, 2u64, v3, 2u64, 0u64, 0u64, 3u64, 4u64, 4u64))
        } else {
            let v4: u64 = {
                let a274 = v0;
                let a275 = 1u64;
                nat_sub(a274, a275)
            };
            let v5: u64 = {
                let a276 = v4;
                let a277 = 10u64;
                let a278 = 0u64;
                nat_rem(a276, a277, a278)
            };
            let v6: u64 = {
                let a282 = 4u64;
                let a283 = {
                    let a279 = v4;
                    let a280 = 10u64;
                    let a281 = 0u64;
                    nat_quot(a279, a280, a281)
                };
                nat_mul(a282, a283)?
            };
            let a284 = v5;
            let a285 = 2u64;
            if nat_lt(a284, a285) {
                f10({
                    let a286 = v6;
                    let a287 = v5;
                    nat_add(a286, a287)?
                }, v1)
            } else {
                let a288 = v5;
                let a289 = 2u64;
                if nat_eq(a288, a289) {
                    Ok(f34(0u64, 2u64, v3))
                } else {
                    let a290 = v5;
                    let a291 = 3u64;
                    if nat_eq(a290, a291) {
                        Ok(f35(0u64, 2u64, v3, {
                            let a292 = v2;
                            let a293 = 2u64;
                            nat_eq(a292, a293)
                        }))
                    } else {
                        let a294 = v5;
                        let a295 = 4u64;
                        if nat_eq(a294, a295) {
                            Ok(f34(1u64, 1u64, 3u64))
                        } else {
                            let a296 = v5;
                            let a297 = 5u64;
                            if nat_eq(a296, a297) {
                                Ok(f35(1u64, 1u64, 3u64, {
                                    let a298 = v2;
                                    let a299 = 0u64;
                                    nat_eq(a298, a299)
                                }))
                            } else {
                                let a300 = v5;
                                let a301 = 6u64;
                                if nat_eq(a300, a301) {
                                    Ok(f34(1u64, 2u64, v3))
                                } else {
                                    let a302 = v5;
                                    let a303 = 7u64;
                                    if nat_eq(a302, a303) {
                                        Ok(f35(1u64, 2u64, v3, {
                                            let a304 = v2;
                                            let a305 = 2u64;
                                            nat_eq(a304, a305)
                                        }))
                                    } else {
                                        let a306 = v5;
                                        let a307 = 8u64;
                                        if nat_eq(a306, a307) {
                                            Ok(Adt3::C4(2u64, {
                                                let m310 = match v1 {
                                                    Adt2::C0(_, _, _, _, _, _, _, h309) => {
                                                        h309
                                                    }
                                                };
                                                match m310 {
                                                    Adt1::C0(h308, _, _, _, _, _, _, _, _, _, _) => {
                                                        h308
                                                    }
                                                }
                                            }, 0u64, 1u64, 0u64))
                                        } else {
                                            f10({
                                                let a311 = v6;
                                                let a312 = 3u64;
                                                nat_add(a311, a312)?
                                            }, v1)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f10(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, h313, _, _) => {
            h313
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h314, _) => {
            h314
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h315) => {
            h315
        }
    };
    let v5: u64 = match v1.clone() {
        Adt2::C0(_, _, h316, _, _, _, _, _) => {
            h316
        }
    };
    let v6: u64 = {
        let a317 = v0;
        let a318 = 4u64;
        let a319 = 0u64;
        nat_rem(a317, a318, a319)
    };
    let v7: u64 = {
        let a320 = v0;
        let a321 = 4u64;
        let a322 = 0u64;
        nat_quot(a320, a321, a322)
    };
    let v8: u64 = {
        let a324 = v7;
        let a325 = match v4.clone() {
            Adt1::C0(_, h323, _, _, _, _, _, _, _, _, _) => {
                h323
            }
        };
        let a326 = 0u64;
        nat_quot(a324, a325, a326)
    };
    let v9: u64 = f16(match v4.clone() {
        Adt1::C0(h327, _, _, _, _, _, _, _, _, _, _) => {
            h327
        }
    });
    let v10: u64 = {
        let a332 = {
            let a329 = v7;
            let a330 = match v4.clone() {
                Adt1::C0(_, h328, _, _, _, _, _, _, _, _, _) => {
                    h328
                }
            };
            let a331 = 0u64;
            nat_rem(a329, a330, a331)
        };
        let a333 = v9;
        nat_mul(a332, a333)?
    };
    let v11: u64 = {
        let a335 = match v4.clone() {
            Adt1::C0(_, h334, _, _, _, _, _, _, _, _, _) => {
                h334
            }
        };
        let a336 = v9;
        nat_mul(a335, a336)?
    };
    let a338 = v0;
    let a339 = match v1.clone() {
        Adt2::C0(h337, _, _, _, _, _, _, _) => {
            h337
        }
    };
    if nat_lt(a338, a339) {
        let a340 = v6;
        let a341 = 0u64;
        if nat_eq(a340, a341) {
            Ok(Adt3::C3(0u64, 0u64, match v2.clone() {
                Adt1::C0(h342, _, _, _, _, _, _, _, _, _, _) => {
                    h342
                }
            }, 0u64, 0u64, {
                let a350 = {
                    let a346 = {
                        let a344 = v8;
                        let a345 = match v2 {
                            Adt1::C0(_, _, _, _, _, _, h343, _, _, _, _) => {
                                h343
                            }
                        };
                        nat_mul(a344, a345)?
                    };
                    let a347 = v10;
                    nat_add(a346, a347)?
                };
                let a351 = {
                    let a348 = v5;
                    let a349 = 100u64;
                    if nat_eq(a348, a349) {
                        v11
                    } else {
                        0u64
                    }
                };
                nat_add(a350, a351)?
            }, 2u64, 0u64, 0u64, 0u64, v9, 4u64))
        } else {
            let a352 = v6;
            let a353 = 1u64;
            if nat_eq(a352, a353) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h354, _, _, _, _, _, _, _, _, _, _) => {
                        h354
                    }
                }, 0u64, {
                    let a355 = v5;
                    let a356 = 98u64;
                    if nat_eq(a355, a356) {
                        4u64
                    } else {
                        0u64
                    }
                }, {
                    let a364 = {
                        let a360 = {
                            let a358 = v8;
                            let a359 = match v3 {
                                Adt1::C0(_, _, _, _, _, _, h357, _, _, _, _) => {
                                    h357
                                }
                            };
                            nat_mul(a358, a359)?
                        };
                        let a361 = v10;
                        nat_add(a360, a361)?
                    };
                    let a365 = {
                        let a362 = v5;
                        let a363 = 99u64;
                        if nat_eq(a362, a363) {
                            v11
                        } else {
                            0u64
                        }
                    };
                    nat_add(a364, a365)?
                }, 2u64, 0u64, 0u64, 1u64, v9, 4u64))
            } else {
                let a366 = v6;
                let a367 = 2u64;
                if nat_eq(a366, a367) {
                    Ok(Adt3::C4(match v1 {
                        Adt2::C0(_, _, _, h368, _, _, _, _) => {
                            h368
                        }
                    }, match v4 {
                        Adt1::C0(h369, _, _, _, _, _, _, _, _, _, _) => {
                            h369
                        }
                    }, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt3::C3(0u64, 0u64, 0u64, match v4.clone() {
                        Adt1::C0(h370, _, _, _, _, _, _, _, _, _, _) => {
                            h370
                        }
                    }, 1u64, 0u64, 2u64, 0u64, 3u64, {
                        let a374 = {
                            let a372 = v8;
                            let a373 = match v4 {
                                Adt1::C0(_, _, _, _, _, _, h371, _, _, _, _) => {
                                    h371
                                }
                            };
                            nat_mul(a372, a373)?
                        };
                        let a375 = v10;
                        nat_add(a374, a375)?
                    }, 4u64, v9))
                }
            }
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f11(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, h376, _, _) => {
            h376
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h377, _) => {
            h377
        }
    };
    let v5: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h378) => {
            h378
        }
    };
    let v6: u64 = {
        let a380 = match v1.clone() {
            Adt2::C0(_, h379, _, _, _, _, _, _) => {
                h379
            }
        };
        let a381 = 3u64;
        if nat_eq(a380, a381) {
            0u64
        } else {
            let a382 = f22(v3.clone())?;
            let a383 = 2u64;
            nat_mul(a382, a383)?
        }
    };
    let a385 = match v1.clone() {
        Adt2::C0(h384, _, _, _, _, _, _, _) => {
            h384
        }
    };
    let a386 = v0;
    if nat_le(a385, a386) {
        Ok(Adt3::C1)
    } else {
        let a387 = v0;
        let a388 = v6;
        if nat_lt(a387, a388) {
            let v7: u64 = {
                let a389 = v0;
                let a390 = 2u64;
                let a391 = 0u64;
                nat_quot(a389, a390, a391)
            };
            let v8: u64 = {
                let a396 = 3u64;
                let a397 = {
                    let a393 = f36(v7, v5.clone())?;
                    let a394 = f16(match v5 {
                        Adt1::C0(h392, _, _, _, _, _, _, _, _, _, _) => {
                            h392
                        }
                    });
                    let a395 = 0u64;
                    nat_quot(a393, a394, a395)
                };
                nat_add(a396, a397)?
            };
            let a401 = {
                let a398 = v0;
                let a399 = 2u64;
                let a400 = 0u64;
                nat_rem(a398, a399, a400)
            };
            let a402 = 0u64;
            if nat_eq(a401, a402) {
                Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, v8, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
            } else {
                let a403 = v2;
                let a404 = 0u64;
                if nat_eq(a403, a404) {
                    Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 2u64, 1u64, 2u64, 0u64, 0u64, v8, 8u64, 8u64))
                } else {
                    Ok(Adt3::C1)
                }
            }
        } else {
            let v9: u64 = {
                let a405 = v0;
                let a406 = v6;
                nat_sub(a405, a406)
            };
            let v10: u64 = f23(match v3.clone() {
                Adt1::C0(h407, _, _, _, _, _, _, _, _, _, _) => {
                    h407
                }
            }, match v4.clone() {
                Adt1::C0(h408, _, _, _, _, _, _, _, _, _, _) => {
                    h408
                }
            }, match v5.clone() {
                Adt1::C0(h409, _, _, _, _, _, _, _, _, _, _) => {
                    h409
                }
            })?;
            let v11: u64 = {
                let a410 = v9;
                let a411 = v10;
                let a412 = 0u64;
                nat_quot(a410, a411, a412)
            };
            let v12: u64 = {
                let a413 = v9;
                let a414 = v10;
                let a415 = 0u64;
                nat_rem(a413, a414, a415)
            };
            let v13: u64 = {
                let a417 = match v3.clone() {
                    Adt1::C0(h416, _, _, _, _, _, _, _, _, _, _) => {
                        h416
                    }
                };
                let a418 = 0u64;
                if nat_eq(a417, a418) {
                    0u64
                } else {
                    1u64
                }
            };
            let v14: u64 = {
                let a420 = match v4.clone() {
                    Adt1::C0(h419, _, _, _, _, _, _, _, _, _, _) => {
                        h419
                    }
                };
                let a421 = 0u64;
                if nat_eq(a420, a421) {
                    0u64
                } else {
                    1u64
                }
            };
            let a422 = v12;
            let a423 = v13;
            if nat_lt(a422, a423) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h424, _, _, _, _, _, _, _, _, _, _) => {
                        h424
                    }
                }, 0u64, 0u64, f36(v11, v3.clone())?, 2u64, 0u64, 0u64, 0u64, f16(match v3 {
                    Adt1::C0(h425, _, _, _, _, _, _, _, _, _, _) => {
                        h425
                    }
                }), 4u64))
            } else {
                let a428 = v12;
                let a429 = {
                    let a426 = v13;
                    let a427 = v14;
                    nat_add(a426, a427)?
                };
                if nat_lt(a428, a429) {
                    Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                        Adt1::C0(h430, _, _, _, _, _, _, _, _, _, _) => {
                            h430
                        }
                    }, 0u64, 4u64, f37(v11, v3, v4.clone())?, 2u64, 0u64, 0u64, 1u64, f16(match v4 {
                        Adt1::C0(h431, _, _, _, _, _, _, _, _, _, _) => {
                            h431
                        }
                    }), 4u64))
                } else {
                    let a434 = v12;
                    let a435 = {
                        let a432 = v13;
                        let a433 = v14;
                        nat_add(a432, a433)?
                    };
                    if nat_eq(a434, a435) {
                        Ok(Adt3::C3(1u64, match v1 {
                            Adt2::C0(_, _, h436, _, _, _, _, _) => {
                                h436
                            }
                        }, 0u64, 0u64, {
                            let a437 = v13;
                            let a438 = 0u64;
                            if nat_eq(a437, a438) {
                                0u64
                            } else {
                                1u64
                            }
                        }, {
                            let a439 = v13;
                            let a440 = 0u64;
                            if nat_eq(a439, a440) {
                                f36(v11, v3.clone())?
                            } else {
                                0u64
                            }
                        }, {
                            let a441 = v14;
                            let a442 = 0u64;
                            if nat_eq(a441, a442) {
                                4u64
                            } else {
                                1u64
                            }
                        }, {
                            let a443 = v14;
                            let a444 = 0u64;
                            if nat_eq(a443, a444) {
                                f37(v11, v3, v4)?
                            } else {
                                1u64
                            }
                        }, {
                            let a446 = match v5.clone() {
                                Adt1::C0(h445, _, _, _, _, _, _, _, _, _, _) => {
                                    h445
                                }
                            };
                            let a447 = 0u64;
                            if nat_eq(a446, a447) {
                                3u64
                            } else {
                                0u64
                            }
                        }, {
                            let a449 = match v5.clone() {
                                Adt1::C0(h448, _, _, _, _, _, _, _, _, _, _) => {
                                    h448
                                }
                            };
                            let a450 = 0u64;
                            if nat_eq(a449, a450) {
                                f36(v11, v5)?
                            } else {
                                0u64
                            }
                        }, 4u64, 4u64))
                    } else {
                        Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                            Adt1::C0(h451, _, _, _, _, _, _, _, _, _, _) => {
                                h451
                            }
                        }, 1u64, 0u64, 2u64, 0u64, 3u64, f36(v11, v5.clone())?, 4u64, f16(match v5 {
                            Adt1::C0(h452, _, _, _, _, _, _, _, _, _, _) => {
                                h452
                            }
                        })))
                    }
                }
            }
        }
    }
}

pub fn f12(v0: Adt1, v1: Adt1) -> bool {
    let a455 = match v0.clone() {
        Adt1::C0(_, h453, _, _, _, _, _, _, _, _, _) => {
            h453
        }
    };
    let a456 = match v1.clone() {
        Adt1::C0(_, h454, _, _, _, _, _, _, _, _, _) => {
            h454
        }
    };
    if if if nat_eq(a455, a456) {
        let a459 = match v0.clone() {
            Adt1::C0(_, _, h457, _, _, _, _, _, _, _, _) => {
                h457
            }
        };
        let a460 = match v1.clone() {
            Adt1::C0(_, _, h458, _, _, _, _, _, _, _, _) => {
                h458
            }
        };
        nat_eq(a459, a460)
    } else {
        false
    } {
        let a463 = match v0.clone() {
            Adt1::C0(_, _, _, h461, _, _, _, _, _, _, _) => {
                h461
            }
        };
        let a464 = match v1.clone() {
            Adt1::C0(_, _, _, h462, _, _, _, _, _, _, _) => {
                h462
            }
        };
        nat_eq(a463, a464)
    } else {
        false
    } {
        let a467 = match v0 {
            Adt1::C0(_, _, _, _, h465, _, _, _, _, _, _) => {
                h465
            }
        };
        let a468 = match v1 {
            Adt1::C0(_, _, _, _, h466, _, _, _, _, _, _) => {
                h466
            }
        };
        nat_eq(a467, a468)
    } else {
        false
    }
}

pub fn f13(v0: Adt1, v1: Adt1) -> bool {
    if f15(v1.clone()) {
        f15(v0)
    } else {
        let a474 = {
            let a471 = match v0.clone() {
                Adt1::C0(_, h469, _, _, _, _, _, _, _, _, _) => {
                    h469
                }
            };
            let a472 = match v1.clone() {
                Adt1::C0(_, h470, _, _, _, _, _, _, _, _, _) => {
                    h470
                }
            };
            let a473 = 1u64;
            nat_rem(a471, a472, a473)
        };
        let a475 = 0u64;
        if if if nat_eq(a474, a475) {
            let a481 = {
                let a478 = match v0.clone() {
                    Adt1::C0(_, _, h476, _, _, _, _, _, _, _, _) => {
                        h476
                    }
                };
                let a479 = match v1.clone() {
                    Adt1::C0(_, _, h477, _, _, _, _, _, _, _, _) => {
                        h477
                    }
                };
                let a480 = 1u64;
                nat_rem(a478, a479, a480)
            };
            let a482 = 0u64;
            nat_eq(a481, a482)
        } else {
            false
        } {
            let a488 = {
                let a485 = match v0.clone() {
                    Adt1::C0(_, _, _, h483, _, _, _, _, _, _, _) => {
                        h483
                    }
                };
                let a486 = match v1.clone() {
                    Adt1::C0(_, _, _, h484, _, _, _, _, _, _, _) => {
                        h484
                    }
                };
                let a487 = 1u64;
                nat_rem(a485, a486, a487)
            };
            let a489 = 0u64;
            nat_eq(a488, a489)
        } else {
            false
        } {
            let a495 = {
                let a492 = match v0 {
                    Adt1::C0(_, _, _, _, h490, _, _, _, _, _, _) => {
                        h490
                    }
                };
                let a493 = match v1 {
                    Adt1::C0(_, _, _, _, h491, _, _, _, _, _, _) => {
                        h491
                    }
                };
                let a494 = 1u64;
                nat_rem(a492, a493, a494)
            };
            let a496 = 0u64;
            nat_eq(a495, a496)
        } else {
            false
        }
    }
}

pub fn f14(v0: u64, v1: u64, v2: u64) -> bool {
    let a497 = v0;
    let a498 = 0u64;
    if if if nat_eq(a497, a498) {
        let a499 = v1;
        let a500 = 0u64;
        nat_eq(a499, a500)
    } else {
        false
    } {
        let a501 = v2;
        let a502 = 0u64;
        nat_eq(a501, a502)
    } else {
        false
    } {
        true
    } else {
        let a503 = v0;
        let a504 = 1u64;
        if if nat_eq(a503, a504) {
            true
        } else {
            let a505 = v0;
            let a506 = 30u64;
            nat_eq(a505, a506)
        } {
            let a507 = v1;
            let a508 = v0;
            if if nat_eq(a507, a508) {
                let a509 = v2;
                let a510 = v0;
                nat_eq(a509, a510)
            } else {
                false
            } {
                true
            } else {
                let a511 = v1;
                let a512 = 0u64;
                if nat_eq(a511, a512) {
                    let a513 = v2;
                    let a514 = v0;
                    if nat_eq(a513, a514) {
                        true
                    } else {
                        let a515 = v2;
                        let a516 = 0u64;
                        nat_eq(a515, a516)
                    }
                } else {
                    false
                }
            }
        } else {
            false
        }
    }
}

pub fn f15(v0: Adt1) -> bool {
    f38(match v0.clone() {
        Adt1::C0(_, h517, _, _, _, _, _, _, _, _, _) => {
            h517
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h518, _, _, _, _, _, _, _, _) => {
            h518
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h519, _, _, _, _, _, _, _) => {
            h519
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, h520, _, _, _, _, _, _) => {
            h520
        }
    })
}

pub fn f16(v0: u64) -> u64 {
    let a521 = v0;
    let a522 = 0u64;
    if nat_eq(a521, a522) {
        4u64
    } else {
        let a523 = v0;
        let a524 = 1u64;
        if if nat_eq(a523, a524) {
            true
        } else {
            let a525 = v0;
            let a526 = 30u64;
            nat_eq(a525, a526)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f17(v0: Adt1) -> bool {
    let v1: u64 = f16(match v0.clone() {
        Adt1::C0(h527, _, _, _, _, _, _, _, _, _, _) => {
            h527
        }
    });
    let a532 = {
        let a529 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h528) => {
                h528
            }
        };
        let a530 = v1;
        let a531 = 1u64;
        nat_rem(a529, a530, a531)
    };
    let a533 = 0u64;
    if if nat_eq(a532, a533) {
        f39(match v0.clone() {
            Adt1::C0(_, h534, _, _, _, _, _, _, _, _, _) => {
                h534
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h535, _, _, _, _, _) => {
                h535
            }
        }, v1)
    } else {
        false
    } {
        f40(match v0.clone() {
            Adt1::C0(_, _, h536, _, _, _, _, _, _, _, _) => {
                h536
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h537, _, _, _, _, _, _, _) => {
                h537
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h538, _, _, _, _, _, _) => {
                h538
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h539, _, _, _, _) => {
                h539
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h540, _, _, _) => {
                h540
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h541, _, _) => {
                h541
            }
        }, v1)
    } else {
        false
    }
}

pub fn f18(v0: Adt1, v1: Adt1) -> R<bool> {
    let a549 = {
        let a547 = match v0.clone() {
            Adt1::C0(_, _, h542, _, _, _, _, _, _, _, _) => {
                h542
            }
        };
        let a548 = {
            let a545 = match v0.clone() {
                Adt1::C0(_, _, _, h543, _, _, _, _, _, _, _) => {
                    h543
                }
            };
            let a546 = match v0.clone() {
                Adt1::C0(_, _, _, _, h544, _, _, _, _, _, _) => {
                    h544
                }
            };
            nat_mul(a545, a546)?
        };
        nat_mul(a547, a548)?
    };
    let a550 = 9223372036854775807u64;
    if nat_le(a549, a550) {
        if f27(v1.clone()) {
            let a552 = match v1 {
                Adt1::C0(_, h551, _, _, _, _, _, _, _, _, _) => {
                    h551
                }
            };
            let a553 = 2147483647u64;
            Ok(nat_le(a552, a553))
        } else {
            let a555 = match v0 {
                Adt1::C0(_, h554, _, _, _, _, _, _, _, _, _) => {
                    h554
                }
            };
            let a556 = 2147483647u64;
            Ok(nat_le(a555, a556))
        }
    } else {
        Ok(false)
    }
}

pub fn f19(v0: Adt1, v1: Adt1) -> R<bool> {
    if f30(v0.clone(), v1.clone())? {
        Ok(true)
    } else {
        Ok(f41(v0, v1))
    }
}

pub fn f20(v0: Adt1) -> R<bool> {
    f42({
        let a559 = match v0.clone() {
            Adt1::C0(_, h557, _, _, _, _, _, _, _, _, _) => {
                h557
            }
        };
        let a560 = f16(match v0.clone() {
            Adt1::C0(h558, _, _, _, _, _, _, _, _, _, _) => {
                h558
            }
        });
        nat_mul(a559, a560)?
    }, match v0.clone() {
        Adt1::C0(_, _, h561, _, _, _, _, _, _, _, _) => {
            h561
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h562, _, _, _, _, _, _, _) => {
            h562
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h563, _, _, _, _, _, _) => {
            h563
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h564, _, _, _, _) => {
            h564
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h565, _, _, _) => {
            h565
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h566, _, _) => {
            h566
        }
    })
}

pub fn f21(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a573 = {
        let a567 = v0;
        let a568 = v2;
        nat_mul(a567, a568)?
    };
    let a574 = {
        let a569 = v1;
        let a570 = 0u64;
        if nat_eq(a569, a570) {
            0u64
        } else {
            let a571 = v0;
            let a572 = 2u64;
            nat_mul(a571, a572)?
        }
    };
    nat_add(a573, a574)
}

pub fn f22(v0: Adt1) -> R<u64> {
    let a583 = {
        let a577 = match v0.clone() {
            Adt1::C0(_, h575, _, _, _, _, _, _, _, _, _) => {
                h575
            }
        };
        let a578 = match v0.clone() {
            Adt1::C0(_, _, h576, _, _, _, _, _, _, _, _) => {
                h576
            }
        };
        nat_mul(a577, a578)?
    };
    let a584 = {
        let a581 = match v0.clone() {
            Adt1::C0(_, _, _, h579, _, _, _, _, _, _, _) => {
                h579
            }
        };
        let a582 = match v0 {
            Adt1::C0(_, _, _, _, h580, _, _, _, _, _, _) => {
                h580
            }
        };
        nat_mul(a581, a582)?
    };
    nat_mul(a583, a584)
}

pub fn f23(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a595 = {
        let a591 = {
            let a587 = 1u64;
            let a588 = {
                let a585 = v0;
                let a586 = 0u64;
                if nat_eq(a585, a586) {
                    0u64
                } else {
                    1u64
                }
            };
            nat_add(a587, a588)?
        };
        let a592 = {
            let a589 = v1;
            let a590 = 0u64;
            if nat_eq(a589, a590) {
                0u64
            } else {
                1u64
            }
        };
        nat_add(a591, a592)?
    };
    let a596 = {
        let a593 = v2;
        let a594 = 0u64;
        if nat_eq(a593, a594) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a595, a596)
}

pub fn f24(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f12(v0.clone(), v1.clone()) {
        let a599 = match v0.clone() {
            Adt1::C0(h597, _, _, _, _, _, _, _, _, _, _) => {
                h597
            }
        };
        let a600 = match v1.clone() {
            Adt1::C0(h598, _, _, _, _, _, _, _, _, _, _) => {
                h598
            }
        };
        nat_eq(a599, a600)
    } else {
        false
    } {
        let a603 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h601, _, _, _, _, _) => {
                h601
            }
        };
        let a604 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h602, _, _, _, _, _) => {
                h602
            }
        };
        nat_eq(a603, a604)
    } else {
        false
    } {
        let a607 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h605, _, _, _, _) => {
                h605
            }
        };
        let a608 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h606, _, _, _, _) => {
                h606
            }
        };
        nat_eq(a607, a608)
    } else {
        false
    } {
        let a611 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h609, _, _, _) => {
                h609
            }
        };
        let a612 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h610, _, _, _) => {
                h610
            }
        };
        nat_eq(a611, a612)
    } else {
        false
    } {
        let a615 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h613, _, _) => {
                h613
            }
        };
        let a616 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h614, _, _) => {
                h614
            }
        };
        nat_eq(a615, a616)
    } else {
        false
    } {
        let a618 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h617) => {
                h617
            }
        };
        let a619 = 0u64;
        if if nat_eq(a618, a619) {
            true
        } else {
            let a621 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h620) => {
                    h620
                }
            };
            let a622 = 0u64;
            nat_eq(a621, a622)
        } {
            true
        } else {
            let a625 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h623) => {
                    h623
                }
            };
            let a626 = match v1 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h624) => {
                    h624
                }
            };
            nat_eq(a625, a626)
        }
    } else {
        false
    }
}

pub fn f25(v0: u64) -> u64 {
    let a627 = v0;
    let a628 = 43u64;
    if nat_eq(a627, a628) {
        1u64
    } else {
        f16(v0)
    }
}

pub fn f26(v0: u64) -> u64 {
    let a629 = v0;
    let a630 = 26u64;
    if nat_eq(a629, a630) {
        4u64
    } else {
        let a631 = v0;
        let a632 = 27u64;
        if nat_eq(a631, a632) {
            8u64
        } else {
            0u64
        }
    }
}

pub fn f27(v0: Adt1) -> bool {
    let a634 = match v0.clone() {
        Adt1::C0(_, h633, _, _, _, _, _, _, _, _, _) => {
            h633
        }
    };
    let a635 = 1u64;
    if nat_eq(a634, a635) {
        true
    } else {
        let a638 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h636, _, _, _, _, _) => {
                h636
            }
        };
        let a639 = f16(match v0 {
            Adt1::C0(h637, _, _, _, _, _, _, _, _, _, _) => {
                h637
            }
        });
        nat_eq(a638, a639)
    }
}

pub fn f28(v0: u64, v1: u64) -> u64 {
    let a640 = v0;
    let a641 = 1u64;
    if if nat_eq(a640, a641) {
        let a642 = v1;
        let a643 = 43u64;
        nat_eq(a642, a643)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f29(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a645 = match v0.clone() {
            Adt1::C0(_, h644, _, _, _, _, _, _, _, _, _) => {
                h644
            }
        };
        let a646 = 4u64;
        nat_mul(a645, a646)?
    };
    let v2: u64 = {
        let a649 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h647, _, _, _, _) => {
                h647
            }
        };
        let a650 = match v0.clone() {
            Adt1::C0(_, _, h648, _, _, _, _, _, _, _, _) => {
                h648
            }
        };
        nat_mul(a649, a650)?
    };
    let v3: u64 = {
        let a652 = v2;
        let a653 = match v0.clone() {
            Adt1::C0(_, _, _, h651, _, _, _, _, _, _, _) => {
                h651
            }
        };
        nat_mul(a652, a653)?
    };
    let a655 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h654, _, _, _, _, _) => {
            h654
        }
    };
    let a656 = 4u64;
    if if if if nat_eq(a655, a656) {
        let a661 = {
            let a658 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h657) => {
                    h657
                }
            };
            let a659 = 4u64;
            let a660 = 1u64;
            nat_rem(a658, a659, a660)
        };
        let a662 = 0u64;
        nat_eq(a661, a662)
    } else {
        false
    } {
        let a670 = {
            let a668 = match v0.clone() {
                Adt1::C0(_, _, h663, _, _, _, _, _, _, _, _) => {
                    h663
                }
            };
            let a669 = {
                let a666 = match v0.clone() {
                    Adt1::C0(_, _, _, h664, _, _, _, _, _, _, _) => {
                        h664
                    }
                };
                let a667 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h665, _, _, _, _, _, _) => {
                        h665
                    }
                };
                nat_mul(a666, a667)?
            };
            nat_mul(a668, a669)?
        };
        let a671 = 1u64;
        if nat_le(a670, a671) {
            true
        } else {
            let a673 = v1;
            let a674 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h672, _, _, _, _) => {
                    h672
                }
            };
            if nat_le(a673, a674) {
                let a679 = {
                    let a676 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, h675, _, _, _, _) => {
                            h675
                        }
                    };
                    let a677 = 4u64;
                    let a678 = 1u64;
                    nat_rem(a676, a677, a678)
                };
                let a680 = 0u64;
                nat_eq(a679, a680)
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a682 = match v0.clone() {
            Adt1::C0(_, _, _, h681, _, _, _, _, _, _, _) => {
                h681
            }
        };
        let a683 = 1u64;
        if nat_le(a682, a683) {
            true
        } else {
            let a685 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h684, _, _, _) => {
                    h684
                }
            };
            let a686 = v2;
            nat_eq(a685, a686)
        }
    } else {
        false
    } {
        let a688 = match v0.clone() {
            Adt1::C0(_, _, _, _, h687, _, _, _, _, _, _) => {
                h687
            }
        };
        let a689 = 1u64;
        if nat_le(a688, a689) {
            Ok(true)
        } else {
            let a691 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, h690, _, _) => {
                    h690
                }
            };
            let a692 = v3;
            Ok(nat_eq(a691, a692))
        }
    } else {
        Ok(false)
    }
}

pub fn f30(v0: Adt1, v1: Adt1) -> R<bool> {
    let a694 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h693, _) => {
            h693
        }
    };
    let a695 = 0u64;
    if nat_eq(a694, a695) {
        Ok(true)
    } else {
        let a697 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h696, _) => {
                h696
            }
        };
        let a698 = 0u64;
        if nat_eq(a697, a698) {
            Ok(true)
        } else {
            let a700 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h699) => {
                    h699
                }
            };
            let a701 = 0u64;
            if nat_eq(a700, a701) {
                Ok(true)
            } else {
                let a703 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h702) => {
                        h702
                    }
                };
                let a704 = 0u64;
                if nat_eq(a703, a704) {
                    Ok(true)
                } else {
                    let a710 = {
                        let a707 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h705) => {
                                h705
                            }
                        };
                        let a708 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h706, _) => {
                                h706
                            }
                        };
                        nat_add(a707, a708)?
                    };
                    let a711 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h709) => {
                            h709
                        }
                    };
                    if nat_le(a710, a711) {
                        Ok(true)
                    } else {
                        let a717 = {
                            let a714 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h712) => {
                                    h712
                                }
                            };
                            let a715 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h713, _) => {
                                    h713
                                }
                            };
                            nat_add(a714, a715)?
                        };
                        let a718 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h716) => {
                                h716
                            }
                        };
                        Ok(nat_le(a717, a718))
                    }
                }
            }
        }
    }
}

pub fn f31(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a721 = match v0.clone() {
            Adt1::C0(_, _, h719, _, _, _, _, _, _, _, _) => {
                h719
            }
        };
        let a722 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h720, _, _, _, _) => {
                h720
            }
        };
        nat_mul(a721, a722)?
    };
    if if f27(v0.clone()) {
        let a724 = match v0.clone() {
            Adt1::C0(_, _, _, h723, _, _, _, _, _, _, _) => {
                h723
            }
        };
        let a725 = 1u64;
        if nat_eq(a724, a725) {
            true
        } else {
            let a727 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h726, _, _, _) => {
                    h726
                }
            };
            let a728 = v1;
            nat_eq(a727, a728)
        }
    } else {
        false
    } {
        let a730 = match v0.clone() {
            Adt1::C0(_, _, _, _, h729, _, _, _, _, _, _) => {
                h729
            }
        };
        let a731 = 1u64;
        if nat_eq(a730, a731) {
            Ok(true)
        } else {
            let a736 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h732, _, _) => {
                    h732
                }
            };
            let a737 = {
                let a734 = v1;
                let a735 = match v0 {
                    Adt1::C0(_, _, _, h733, _, _, _, _, _, _, _) => {
                        h733
                    }
                };
                nat_mul(a734, a735)?
            };
            Ok(nat_eq(a736, a737))
        }
    } else {
        Ok(false)
    }
}

pub fn f32(v0: u64, v1: Adt1, v2: u64, v3: u64) -> R<Adt3> {
    let v4: u64 = {
        let a738 = v0;
        let a739 = 2u64;
        let a740 = 0u64;
        nat_quot(a738, a739, a740)
    };
    let v5: u64 = f26(match v1.clone() {
        Adt1::C0(h741, _, _, _, _, _, _, _, _, _, _) => {
            h741
        }
    });
    let a745 = {
        let a742 = v0;
        let a743 = 2u64;
        let a744 = 0u64;
        nat_rem(a742, a743, a744)
    };
    let a746 = 0u64;
    if nat_eq(a745, a746) {
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 4u64, f36(v4, v1)?, 2u64, 0u64, 0u64, 2u64, v5, 8u64))
    } else {
        let a747 = v3;
        let a748 = v2;
        if if nat_lt(a747, a748) {
            let a751 = v3;
            let a752 = {
                let a749 = v5;
                let a750 = 4u64;
                if nat_eq(a749, a750) {
                    2147483648u64
                } else {
                    9223372036854775808u64
                }
            };
            nat_lt(a751, a752)
        } else {
            false
        } {
            Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, {
                let a753 = 3u64;
                let a754 = v4;
                nat_add(a753, a754)?
            }, 8u64, 8u64))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f33(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64) -> R<Adt3> {
    let v5: u64 = {
        let a760 = 1u64;
        let a761 = {
            let a758 = match v1.clone() {
                Adt1::C0(_, h755, _, _, _, _, _, _, _, _, _) => {
                    h755
                }
            };
            let a759 = f28(match v1.clone() {
                Adt1::C0(h756, _, _, _, _, _, _, _, _, _, _) => {
                    h756
                }
            }, match v3.clone() {
                Adt1::C0(h757, _, _, _, _, _, _, _, _, _, _) => {
                    h757
                }
            });
            nat_mul(a758, a759)?
        };
        nat_add(a760, a761)?
    };
    let v6: u64 = {
        let a762 = v0;
        let a763 = v5;
        let a764 = 0u64;
        nat_rem(a762, a763, a764)
    };
    let v7: u64 = {
        let a765 = v0;
        let a766 = v5;
        let a767 = 0u64;
        nat_quot(a765, a766, a767)
    };
    let v8: u64 = {
        let a769 = v7;
        let a770 = match v1.clone() {
            Adt1::C0(_, _, h768, _, _, _, _, _, _, _, _) => {
                h768
            }
        };
        let a771 = 0u64;
        nat_rem(a769, a770, a771)
    };
    let v9: u64 = {
        let a773 = v7;
        let a774 = match v1.clone() {
            Adt1::C0(_, _, h772, _, _, _, _, _, _, _, _) => {
                h772
            }
        };
        let a775 = 0u64;
        nat_quot(a773, a774, a775)
    };
    let v10: u64 = {
        let a777 = v9;
        let a778 = match v1.clone() {
            Adt1::C0(_, _, _, h776, _, _, _, _, _, _, _) => {
                h776
            }
        };
        let a779 = 0u64;
        nat_rem(a777, a778, a779)
    };
    let v11: u64 = {
        let a781 = v9;
        let a782 = match v1.clone() {
            Adt1::C0(_, _, _, h780, _, _, _, _, _, _, _) => {
                h780
            }
        };
        let a783 = 0u64;
        nat_quot(a781, a782, a783)
    };
    let a784 = v6;
    let a785 = 0u64;
    if nat_eq(a784, a785) {
        let v12: u64 = {
            let a802 = v8;
            let a803 = {
                let a800 = match v2.clone() {
                    Adt1::C0(_, h786, _, _, _, _, _, _, _, _, _) => {
                        h786
                    }
                };
                let a801 = {
                    let a798 = {
                        let a788 = v10;
                        let a789 = match v2.clone() {
                            Adt1::C0(_, _, h787, _, _, _, _, _, _, _, _) => {
                                h787
                            }
                        };
                        let a790 = 0u64;
                        nat_rem(a788, a789, a790)
                    };
                    let a799 = {
                        let a796 = match v2.clone() {
                            Adt1::C0(_, _, h791, _, _, _, _, _, _, _, _) => {
                                h791
                            }
                        };
                        let a797 = {
                            let a793 = v11;
                            let a794 = match v2 {
                                Adt1::C0(_, _, _, h792, _, _, _, _, _, _, _) => {
                                    h792
                                }
                            };
                            let a795 = 0u64;
                            nat_rem(a793, a794, a795)
                        };
                        nat_mul(a796, a797)?
                    };
                    nat_add(a798, a799)?
                };
                nat_mul(a800, a801)?
            };
            nat_add(a802, a803)?
        };
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, {
            let a804 = 3u64;
            let a805 = v12;
            nat_add(a804, a805)?
        }, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
    } else {
        let a807 = v4;
        let a808 = match v3.clone() {
            Adt1::C0(_, _, h806, _, _, _, _, _, _, _, _) => {
                h806
            }
        };
        if nat_lt(a807, a808) {
            let v13: u64 = {
                let a813 = {
                    let a809 = v6;
                    let a810 = 1u64;
                    nat_sub(a809, a810)
                };
                let a814 = f28(match v1.clone() {
                    Adt1::C0(h811, _, _, _, _, _, _, _, _, _, _) => {
                        h811
                    }
                }, match v3.clone() {
                    Adt1::C0(h812, _, _, _, _, _, _, _, _, _, _) => {
                        h812
                    }
                });
                let a815 = 0u64;
                nat_quot(a813, a814, a815)
            };
            let v14: u64 = f43(v13, v8, v10, v11, match v1.clone() {
                Adt1::C0(_, _, _, _, _, h816, _, _, _, _, _) => {
                    h816
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, h817, _, _, _, _) => {
                    h817
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h818, _, _, _) => {
                    h818
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h819, _, _) => {
                    h819
                }
            })?;
            let v15: u64 = f43(v13, v4, v10, v11, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h820, _, _, _, _, _) => {
                    h820
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h821, _, _, _, _) => {
                    h821
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h822, _, _, _) => {
                    h822
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h823, _, _) => {
                    h823
                }
            })?;
            Ok(f44(match v1 {
                Adt1::C0(h824, _, _, _, _, _, _, _, _, _, _) => {
                    h824
                }
            }, match v3 {
                Adt1::C0(h825, _, _, _, _, _, _, _, _, _, _) => {
                    h825
                }
            }, {
                let a826 = v6;
                let a827 = 1u64;
                nat_sub(a826, a827)
            }, v14, v15))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f34(v0: u64, v1: u64, v2: u64) -> Adt3 {
    Adt3::C3(2u64, 0u64, 0u64, 0u64, 1u64, v0, v1, v2, 0u64, 2u64, 4u64, 4u64)
}

pub fn f35(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt3 {
    Adt3::C3(2u64, 3u64, 0u64, 0u64, if v3 {
        v1
    } else {
        1u64
    }, if v3 {
        v2
    } else {
        v0
    }, 2u64, 0u64, 0u64, v0, 4u64, 4u64)
}

pub fn f36(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a829 = v0;
        let a830 = match v1.clone() {
            Adt1::C0(_, h828, _, _, _, _, _, _, _, _, _) => {
                h828
            }
        };
        let a831 = 0u64;
        nat_quot(a829, a830, a831)
    };
    let v3: u64 = {
        let a833 = v2;
        let a834 = match v1.clone() {
            Adt1::C0(_, _, h832, _, _, _, _, _, _, _, _) => {
                h832
            }
        };
        let a835 = 0u64;
        nat_quot(a833, a834, a835)
    };
    f43({
        let a837 = v0;
        let a838 = match v1.clone() {
            Adt1::C0(_, h836, _, _, _, _, _, _, _, _, _) => {
                h836
            }
        };
        let a839 = 0u64;
        nat_rem(a837, a838, a839)
    }, {
        let a841 = v2;
        let a842 = match v1.clone() {
            Adt1::C0(_, _, h840, _, _, _, _, _, _, _, _) => {
                h840
            }
        };
        let a843 = 0u64;
        nat_rem(a841, a842, a843)
    }, {
        let a845 = v3;
        let a846 = match v1.clone() {
            Adt1::C0(_, _, _, h844, _, _, _, _, _, _, _) => {
                h844
            }
        };
        let a847 = 0u64;
        nat_rem(a845, a846, a847)
    }, {
        let a849 = v3;
        let a850 = match v1.clone() {
            Adt1::C0(_, _, _, h848, _, _, _, _, _, _, _) => {
                h848
            }
        };
        let a851 = 0u64;
        nat_quot(a849, a850, a851)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h852, _, _, _, _, _) => {
            h852
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h853, _, _, _, _) => {
            h853
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h854, _, _, _) => {
            h854
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h855, _, _) => {
            h855
        }
    })
}

pub fn f37(v0: u64, v1: Adt1, v2: Adt1) -> R<u64> {
    let v3: u64 = {
        let a857 = v0;
        let a858 = match v1.clone() {
            Adt1::C0(_, h856, _, _, _, _, _, _, _, _, _) => {
                h856
            }
        };
        let a859 = 0u64;
        nat_quot(a857, a858, a859)
    };
    let v4: u64 = {
        let a861 = v3;
        let a862 = match v1.clone() {
            Adt1::C0(_, _, h860, _, _, _, _, _, _, _, _) => {
                h860
            }
        };
        let a863 = 0u64;
        nat_quot(a861, a862, a863)
    };
    f43({
        let a869 = {
            let a865 = v0;
            let a866 = match v1.clone() {
                Adt1::C0(_, h864, _, _, _, _, _, _, _, _, _) => {
                    h864
                }
            };
            let a867 = 0u64;
            nat_rem(a865, a866, a867)
        };
        let a870 = match v2.clone() {
            Adt1::C0(_, h868, _, _, _, _, _, _, _, _, _) => {
                h868
            }
        };
        let a871 = 0u64;
        nat_rem(a869, a870, a871)
    }, {
        let a877 = {
            let a873 = v3;
            let a874 = match v1.clone() {
                Adt1::C0(_, _, h872, _, _, _, _, _, _, _, _) => {
                    h872
                }
            };
            let a875 = 0u64;
            nat_rem(a873, a874, a875)
        };
        let a878 = match v2.clone() {
            Adt1::C0(_, _, h876, _, _, _, _, _, _, _, _) => {
                h876
            }
        };
        let a879 = 0u64;
        nat_rem(a877, a878, a879)
    }, {
        let a885 = {
            let a881 = v4;
            let a882 = match v1.clone() {
                Adt1::C0(_, _, _, h880, _, _, _, _, _, _, _) => {
                    h880
                }
            };
            let a883 = 0u64;
            nat_rem(a881, a882, a883)
        };
        let a886 = match v2.clone() {
            Adt1::C0(_, _, _, h884, _, _, _, _, _, _, _) => {
                h884
            }
        };
        let a887 = 0u64;
        nat_rem(a885, a886, a887)
    }, {
        let a893 = {
            let a889 = v4;
            let a890 = match v1 {
                Adt1::C0(_, _, _, h888, _, _, _, _, _, _, _) => {
                    h888
                }
            };
            let a891 = 0u64;
            nat_quot(a889, a890, a891)
        };
        let a894 = match v2.clone() {
            Adt1::C0(_, _, _, _, h892, _, _, _, _, _, _) => {
                h892
            }
        };
        let a895 = 0u64;
        nat_rem(a893, a894, a895)
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h896, _, _, _, _, _) => {
            h896
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h897, _, _, _, _) => {
            h897
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h898, _, _, _) => {
            h898
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h899, _, _) => {
            h899
        }
    })
}

pub fn f38(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a900 = v0;
    let a901 = 0u64;
    if if if nat_eq(a900, a901) {
        true
    } else {
        let a902 = v1;
        let a903 = 0u64;
        nat_eq(a902, a903)
    } {
        true
    } else {
        let a904 = v2;
        let a905 = 0u64;
        nat_eq(a904, a905)
    } {
        true
    } else {
        let a906 = v3;
        let a907 = 0u64;
        nat_eq(a906, a907)
    }
}

pub fn f39(v0: u64, v1: u64, v2: u64) -> bool {
    let a908 = v0;
    let a909 = 1u64;
    if nat_le(a908, a909) {
        true
    } else {
        let a913 = {
            let a910 = v1;
            let a911 = v2;
            let a912 = 1u64;
            nat_rem(a910, a911, a912)
        };
        let a914 = 0u64;
        nat_eq(a913, a914)
    }
}

pub fn f40(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f39(v0, v3, v6) {
        f39(v1, v4, v6)
    } else {
        false
    } {
        f39(v2, v5, v6)
    } else {
        false
    }
}

pub fn f41(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f12(v0.clone(), v1.clone()) {
        let a917 = match v0.clone() {
            Adt1::C0(h915, _, _, _, _, _, _, _, _, _, _) => {
                h915
            }
        };
        let a918 = match v1.clone() {
            Adt1::C0(h916, _, _, _, _, _, _, _, _, _, _) => {
                h916
            }
        };
        nat_eq(a917, a918)
    } else {
        false
    } {
        let a921 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h919) => {
                h919
            }
        };
        let a922 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h920) => {
                h920
            }
        };
        nat_eq(a921, a922)
    } else {
        false
    } {
        let a925 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h923, _, _, _, _, _) => {
                h923
            }
        };
        let a926 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h924, _, _, _, _, _) => {
                h924
            }
        };
        nat_eq(a925, a926)
    } else {
        false
    } {
        let a929 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h927, _, _, _, _) => {
                h927
            }
        };
        let a930 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h928, _, _, _, _) => {
                h928
            }
        };
        nat_eq(a929, a930)
    } else {
        false
    } {
        let a933 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h931, _, _, _) => {
                h931
            }
        };
        let a934 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h932, _, _, _) => {
                h932
            }
        };
        nat_eq(a933, a934)
    } else {
        false
    } {
        let a937 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h935, _, _) => {
                h935
            }
        };
        let a938 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h936, _, _) => {
                h936
            }
        };
        nat_eq(a937, a938)
    } else {
        false
    }
}

pub fn f42(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f45(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f45(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f45(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f45(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f45(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f45(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f43(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a951 = {
        let a947 = {
            let a943 = {
                let a939 = v0;
                let a940 = v4;
                nat_mul(a939, a940)?
            };
            let a944 = {
                let a941 = v1;
                let a942 = v5;
                nat_mul(a941, a942)?
            };
            nat_add(a943, a944)?
        };
        let a948 = {
            let a945 = v2;
            let a946 = v6;
            nat_mul(a945, a946)?
        };
        nat_add(a947, a948)?
    };
    let a952 = {
        let a949 = v3;
        let a950 = v7;
        nat_mul(a949, a950)?
    };
    nat_add(a951, a952)
}

pub fn f44(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt3 {
    let a953 = f28(v0, v1);
    let a954 = 2u64;
    if nat_eq(a953, a954) {
        let a958 = {
            let a955 = v2;
            let a956 = 2u64;
            let a957 = 0u64;
            nat_rem(a955, a956, a957)
        };
        let a959 = 0u64;
        if nat_eq(a958, a959) {
            Adt3::C3(0u64, 0u64, v0, 0u64, 0u64, v3, 2u64, 0u64, 1u64, 0u64, f16(v0), 8u64)
        } else {
            Adt3::C3(0u64, 0u64, 0u64, v1, 1u64, 0u64, 2u64, 0u64, 3u64, v4, 8u64, f25(v1))
        }
    } else {
        Adt3::C3(0u64, 0u64, v0, v1, 0u64, v3, 2u64, 0u64, 3u64, v4, f16(v0), f25(v1))
    }
}

pub fn f45(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a960 = v1;
    let a961 = 1u64;
    if if if nat_le(a960, a961) {
        true
    } else {
        let a962 = v0;
        let a963 = v2;
        nat_le(a962, a963)
    } {
        let a964 = v3;
        let a965 = 1u64;
        if nat_le(a964, a965) {
            true
        } else {
            let a972 = {
                let a970 = v0;
                let a971 = {
                    let a968 = {
                        let a966 = v1;
                        let a967 = 1u64;
                        nat_sub(a966, a967)
                    };
                    let a969 = v2;
                    nat_mul(a968, a969)?
                };
                nat_add(a970, a971)?
            };
            let a973 = v4;
            nat_le(a972, a973)
        }
    } else {
        false
    } {
        let a974 = v5;
        let a975 = 1u64;
        if nat_le(a974, a975) {
            Ok(true)
        } else {
            let a988 = {
                let a986 = {
                    let a980 = v0;
                    let a981 = {
                        let a978 = {
                            let a976 = v1;
                            let a977 = 1u64;
                            nat_sub(a976, a977)
                        };
                        let a979 = v2;
                        nat_mul(a978, a979)?
                    };
                    nat_add(a980, a981)?
                };
                let a987 = {
                    let a984 = {
                        let a982 = v3;
                        let a983 = 1u64;
                        nat_sub(a982, a983)
                    };
                    let a985 = v4;
                    nat_mul(a984, a985)?
                };
                nat_add(a986, a987)?
            };
            let a989 = v6;
            Ok(nat_le(a988, a989))
        }
    } else {
        Ok(false)
    }
}
