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
        let a67 = f16(match v3.clone() {
            Adt1::C0(h65, _, _, _, _, _, _, _, _, _, _) => {
                h65
            }
        });
        nat_lt(a66, a67)
    } else {
        false
    } {
        let a69 = 0u64;
        let a70 = f25(match v1.clone() {
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
        f26(v0.clone())
    } else {
        false
    } {
        f26(v3.clone())
    } else {
        false
    } {
        let v4: u64 = f22(v1.clone())?;
        let v5: u64 = {
            let a115 = match v0.clone() {
                Adt1::C0(_, _, h110, _, _, _, _, _, _, _, _) => {
                    h110
                }
            };
            let a116 = {
                let a113 = match v0.clone() {
                    Adt1::C0(_, _, _, h111, _, _, _, _, _, _, _) => {
                        h111
                    }
                };
                let a114 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h112, _, _, _, _, _, _) => {
                        h112
                    }
                };
                nat_mul(a113, a114)?
            };
            nat_mul(a115, a116)?
        };
        let v6: u64 = if f15(v0.clone()) {
            0u64
        } else {
            let a124 = {
                let a117 = 2u64;
                let a118 = v4;
                nat_mul(a117, a118)?
            };
            let a125 = {
                let a122 = v5;
                let a123 = {
                    let a120 = 1u64;
                    let a121 = match v0.clone() {
                        Adt1::C0(_, h119, _, _, _, _, _, _, _, _, _) => {
                            h119
                        }
                    };
                    nat_add(a120, a121)?
                };
                nat_mul(a122, a123)?
            };
            nat_add(a124, a125)?
        };
        Ok(Adt3::C2(Adt2::C0(v6, if f15(v0.clone()) {
            3u64
        } else {
            let a126 = 3u64;
            let a127 = v4;
            nat_add(a126, a127)?
        }, 41u64, 0u64, 0u64, v0, v1, v3)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f4(v0: u64, v1: u64, v2: Adt1, v3: Adt1) -> R<Adt3> {
    let a129 = match v2.clone() {
        Adt1::C0(h128, _, _, _, _, _, _, _, _, _, _) => {
            h128
        }
    };
    let a130 = 0u64;
    if if if if if if if if if if if nat_eq(a129, a130) {
        let a132 = match v3.clone() {
            Adt1::C0(h131, _, _, _, _, _, _, _, _, _, _) => {
                h131
            }
        };
        let a133 = 26u64;
        nat_eq(a132, a133)
    } else {
        false
    } {
        let a135 = match v2.clone() {
            Adt1::C0(_, h134, _, _, _, _, _, _, _, _, _) => {
                h134
            }
        };
        let a136 = 2147483647u64;
        nat_le(a135, a136)
    } else {
        false
    } {
        let a139 = match v3.clone() {
            Adt1::C0(_, h137, _, _, _, _, _, _, _, _, _) => {
                h137
            }
        };
        let a140 = match v2.clone() {
            Adt1::C0(_, h138, _, _, _, _, _, _, _, _, _) => {
                h138
            }
        };
        nat_le(a139, a140)
    } else {
        false
    } {
        let a143 = match v2.clone() {
            Adt1::C0(_, _, h141, _, _, _, _, _, _, _, _) => {
                h141
            }
        };
        let a144 = match v3.clone() {
            Adt1::C0(_, _, h142, _, _, _, _, _, _, _, _) => {
                h142
            }
        };
        nat_eq(a143, a144)
    } else {
        false
    } {
        let a147 = match v2.clone() {
            Adt1::C0(_, _, _, h145, _, _, _, _, _, _, _) => {
                h145
            }
        };
        let a148 = match v3.clone() {
            Adt1::C0(_, _, _, h146, _, _, _, _, _, _, _) => {
                h146
            }
        };
        nat_eq(a147, a148)
    } else {
        false
    } {
        let a151 = match v2.clone() {
            Adt1::C0(_, _, _, _, h149, _, _, _, _, _, _) => {
                h149
            }
        };
        let a152 = match v3.clone() {
            Adt1::C0(_, _, _, _, h150, _, _, _, _, _, _) => {
                h150
            }
        };
        nat_eq(a151, a152)
    } else {
        false
    } {
        let a153 = v0;
        let a154 = 67u64;
        if nat_eq(a153, a154) {
            true
        } else {
            let a155 = v1;
            let a156 = 1u64;
            if nat_le(a155, a156) {
                let a159 = match v3.clone() {
                    Adt1::C0(_, h157, _, _, _, _, _, _, _, _, _) => {
                        h157
                    }
                };
                let a160 = match v2.clone() {
                    Adt1::C0(_, h158, _, _, _, _, _, _, _, _, _) => {
                        h158
                    }
                };
                nat_eq(a159, a160)
            } else {
                false
            }
        }
    } else {
        false
    } {
        f27(v2.clone())?
    } else {
        false
    } {
        f27(v3.clone())?
    } else {
        false
    } {
        f28(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a166 = match v3.clone() {
                Adt1::C0(_, _, h161, _, _, _, _, _, _, _, _) => {
                    h161
                }
            };
            let a167 = {
                let a164 = match v3.clone() {
                    Adt1::C0(_, _, _, h162, _, _, _, _, _, _, _) => {
                        h162
                    }
                };
                let a165 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h163, _, _, _, _, _, _) => {
                        h163
                    }
                };
                nat_mul(a164, a165)?
            };
            nat_mul(a166, a167)?
        }, 3u64, v0, {
            let a168 = v0;
            let a169 = 67u64;
            if nat_eq(a168, a169) {
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
    let a170 = v0;
    let a171 = 2u64;
    if if nat_le(a170, a171) {
        true
    } else {
        let a172 = v0;
        let a173 = 5u64;
        nat_eq(a172, a173)
    } {
        true
    } else {
        let a174 = v0;
        let a175 = 6u64;
        nat_eq(a174, a175)
    }
}

pub fn f6(v0: u64, v1: bool, v2: u64, v3: u64, v4: Adt1, v5: Adt1, v6: Adt1) -> R<Adt3> {
    let a176 = v2;
    let a177 = 1u64;
    if if if if if if if if if if if if if if if nat_le(a176, a177) {
        let a179 = 0u64;
        let a180 = f16(match v4.clone() {
            Adt1::C0(h178, _, _, _, _, _, _, _, _, _, _) => {
                h178
            }
        });
        nat_lt(a179, a180)
    } else {
        false
    } {
        let a183 = match v4.clone() {
            Adt1::C0(h181, _, _, _, _, _, _, _, _, _, _) => {
                h181
            }
        };
        let a184 = match v5.clone() {
            Adt1::C0(h182, _, _, _, _, _, _, _, _, _, _) => {
                h182
            }
        };
        nat_eq(a183, a184)
    } else {
        false
    } {
        let a187 = match v4.clone() {
            Adt1::C0(h185, _, _, _, _, _, _, _, _, _, _) => {
                h185
            }
        };
        let a188 = match v6.clone() {
            Adt1::C0(h186, _, _, _, _, _, _, _, _, _, _) => {
                h186
            }
        };
        nat_eq(a187, a188)
    } else {
        false
    } {
        f12(v4.clone(), v5.clone())
    } else {
        false
    } {
        f29(v4.clone())?
    } else {
        false
    } {
        f29(v5.clone())?
    } else {
        false
    } {
        f29(v6.clone())?
    } else {
        false
    } {
        let a191 = match v4.clone() {
            Adt1::C0(_, _, h189, _, _, _, _, _, _, _, _) => {
                h189
            }
        };
        let a192 = match v6.clone() {
            Adt1::C0(_, _, h190, _, _, _, _, _, _, _, _) => {
                h190
            }
        };
        nat_eq(a191, a192)
    } else {
        false
    } {
        let a195 = match v4.clone() {
            Adt1::C0(_, _, _, h193, _, _, _, _, _, _, _) => {
                h193
            }
        };
        let a196 = match v6.clone() {
            Adt1::C0(_, _, _, h194, _, _, _, _, _, _, _) => {
                h194
            }
        };
        nat_eq(a195, a196)
    } else {
        false
    } {
        let a199 = match v4.clone() {
            Adt1::C0(_, _, _, _, h197, _, _, _, _, _, _) => {
                h197
            }
        };
        let a200 = match v6.clone() {
            Adt1::C0(_, _, _, _, h198, _, _, _, _, _, _) => {
                h198
            }
        };
        nat_eq(a199, a200)
    } else {
        false
    } {
        let a207 = match v6.clone() {
            Adt1::C0(_, h201, _, _, _, _, _, _, _, _, _) => {
                h201
            }
        };
        let a208 = if v1 {
            match v4.clone() {
                Adt1::C0(_, h202, _, _, _, _, _, _, _, _, _) => {
                    h202
                }
            }
        } else {
            let a204 = match v4.clone() {
                Adt1::C0(_, h203, _, _, _, _, _, _, _, _, _) => {
                    h203
                }
            };
            let a205 = 2u64;
            let a206 = 0u64;
            nat_quot(a204, a205, a206)
        };
        nat_eq(a207, a208)
    } else {
        false
    } {
        let a210 = match v6.clone() {
            Adt1::C0(_, h209, _, _, _, _, _, _, _, _, _) => {
                h209
            }
        };
        let a211 = 2147483647u64;
        nat_le(a210, a211)
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
            let a212 = v0;
            let a213 = 6u64;
            if nat_eq(a212, a213) {
                let a216 = 1u64;
                let a217 = {
                    let a214 = 10u64;
                    let a215 = f22(v6.clone())?;
                    nat_mul(a214, a215)?
                };
                nat_add(a216, a217)?
            } else {
                let a218 = 4u64;
                let a219 = f22(v6.clone())?;
                nat_mul(a218, a219)?
            }
        }, {
            let a220 = v0;
            let a221 = 6u64;
            if nat_eq(a220, a221) {
                4u64
            } else {
                3u64
            }
        }, if v1 {
            98u64
        } else {
            let a222 = 99u64;
            let a223 = v2;
            nat_add(a222, a223)?
        }, v0, v3, v4, v5, v6)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f7(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, h224, _, _) => {
            h224
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h225) => {
            h225
        }
    };
    let a227 = v0;
    let a228 = match v1.clone() {
        Adt2::C0(h226, _, _, _, _, _, _, _) => {
            h226
        }
    };
    if nat_lt(a227, a228) {
        Ok(Adt3::C5({
            let a230 = v0;
            let a231 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, _, h229, _, _, _, _) => {
                    h229
                }
            };
            nat_mul(a230, a231)?
        }, {
            let a233 = v0;
            let a234 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h232, _, _, _, _) => {
                    h232
                }
            };
            nat_mul(a233, a234)?
        }, match v2 {
            Adt1::C0(_, h235, _, _, _, _, _, _, _, _, _) => {
                h235
            }
        }, match v3 {
            Adt1::C0(_, h236, _, _, _, _, _, _, _, _, _) => {
                h236
            }
        }, {
            let a238 = match v1 {
                Adt2::C0(_, _, _, h237, _, _, _, _) => {
                    h237
                }
            };
            let a239 = 1u64;
            nat_eq(a238, a239)
        }))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f8(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let a241 = v0;
    let a242 = match v1.clone() {
        Adt2::C0(h240, _, _, _, _, _, _, _) => {
            h240
        }
    };
    if nat_lt(a241, a242) {
        let v3: u64 = {
            let a244 = 2u64;
            let a245 = f22(match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, h243, _) => {
                    h243
                }
            })?;
            nat_mul(a244, a245)?
        };
        let a246 = v0;
        let a247 = v3;
        if nat_lt(a246, a247) {
            f30(v0, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, h248, _) => {
                    h248
                }
            }, {
                let m251 = match v1 {
                    Adt2::C0(_, _, _, _, _, _, _, h250) => {
                        h250
                    }
                };
                match m251 {
                    Adt1::C0(_, _, h249, _, _, _, _, _, _, _, _) => {
                        h249
                    }
                }
            }, v2)
        } else {
            f31({
                let a252 = v0;
                let a253 = v3;
                nat_sub(a252, a253)
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, h254, _, _) => {
                    h254
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, h255, _) => {
                    h255
                }
            }, match v1 {
                Adt2::C0(_, _, _, _, _, _, _, h256) => {
                    h256
                }
            }, v2)
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f9(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(_, _, _, _, h257, _, _, _) => {
            h257
        }
    };
    let a259 = v0;
    let a260 = match v1.clone() {
        Adt2::C0(h258, _, _, _, _, _, _, _) => {
            h258
        }
    };
    if nat_lt(a259, a260) {
        let a261 = v0;
        let a262 = 0u64;
        if nat_eq(a261, a262) {
            Ok(Adt3::C3(1u64, 6u64, 0u64, 0u64, 2u64, v3, 2u64, 0u64, 0u64, 3u64, 4u64, 4u64))
        } else {
            let v4: u64 = {
                let a263 = v0;
                let a264 = 1u64;
                nat_sub(a263, a264)
            };
            let v5: u64 = {
                let a265 = v4;
                let a266 = 10u64;
                let a267 = 0u64;
                nat_rem(a265, a266, a267)
            };
            let v6: u64 = {
                let a271 = 4u64;
                let a272 = {
                    let a268 = v4;
                    let a269 = 10u64;
                    let a270 = 0u64;
                    nat_quot(a268, a269, a270)
                };
                nat_mul(a271, a272)?
            };
            let a273 = v5;
            let a274 = 2u64;
            if nat_lt(a273, a274) {
                f10({
                    let a275 = v6;
                    let a276 = v5;
                    nat_add(a275, a276)?
                }, v1)
            } else {
                let a277 = v5;
                let a278 = 2u64;
                if nat_eq(a277, a278) {
                    Ok(f32(0u64, 2u64, v3))
                } else {
                    let a279 = v5;
                    let a280 = 3u64;
                    if nat_eq(a279, a280) {
                        Ok(f33(0u64, 2u64, v3, {
                            let a281 = v2;
                            let a282 = 2u64;
                            nat_eq(a281, a282)
                        }))
                    } else {
                        let a283 = v5;
                        let a284 = 4u64;
                        if nat_eq(a283, a284) {
                            Ok(f32(1u64, 1u64, 3u64))
                        } else {
                            let a285 = v5;
                            let a286 = 5u64;
                            if nat_eq(a285, a286) {
                                Ok(f33(1u64, 1u64, 3u64, {
                                    let a287 = v2;
                                    let a288 = 0u64;
                                    nat_eq(a287, a288)
                                }))
                            } else {
                                let a289 = v5;
                                let a290 = 6u64;
                                if nat_eq(a289, a290) {
                                    Ok(f32(1u64, 2u64, v3))
                                } else {
                                    let a291 = v5;
                                    let a292 = 7u64;
                                    if nat_eq(a291, a292) {
                                        Ok(f33(1u64, 2u64, v3, {
                                            let a293 = v2;
                                            let a294 = 2u64;
                                            nat_eq(a293, a294)
                                        }))
                                    } else {
                                        let a295 = v5;
                                        let a296 = 8u64;
                                        if nat_eq(a295, a296) {
                                            Ok(Adt3::C4(2u64, {
                                                let m299 = match v1 {
                                                    Adt2::C0(_, _, _, _, _, _, _, h298) => {
                                                        h298
                                                    }
                                                };
                                                match m299 {
                                                    Adt1::C0(h297, _, _, _, _, _, _, _, _, _, _) => {
                                                        h297
                                                    }
                                                }
                                            }, 0u64, 1u64, 0u64))
                                        } else {
                                            f10({
                                                let a300 = v6;
                                                let a301 = 3u64;
                                                nat_add(a300, a301)?
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
        Adt2::C0(_, _, _, _, _, h302, _, _) => {
            h302
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h303, _) => {
            h303
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h304) => {
            h304
        }
    };
    let v5: u64 = match v1.clone() {
        Adt2::C0(_, _, h305, _, _, _, _, _) => {
            h305
        }
    };
    let v6: u64 = {
        let a306 = v0;
        let a307 = 4u64;
        let a308 = 0u64;
        nat_rem(a306, a307, a308)
    };
    let v7: u64 = {
        let a309 = v0;
        let a310 = 4u64;
        let a311 = 0u64;
        nat_quot(a309, a310, a311)
    };
    let v8: u64 = {
        let a313 = v7;
        let a314 = match v4.clone() {
            Adt1::C0(_, h312, _, _, _, _, _, _, _, _, _) => {
                h312
            }
        };
        let a315 = 0u64;
        nat_quot(a313, a314, a315)
    };
    let v9: u64 = f16(match v4.clone() {
        Adt1::C0(h316, _, _, _, _, _, _, _, _, _, _) => {
            h316
        }
    });
    let v10: u64 = {
        let a321 = {
            let a318 = v7;
            let a319 = match v4.clone() {
                Adt1::C0(_, h317, _, _, _, _, _, _, _, _, _) => {
                    h317
                }
            };
            let a320 = 0u64;
            nat_rem(a318, a319, a320)
        };
        let a322 = v9;
        nat_mul(a321, a322)?
    };
    let v11: u64 = {
        let a324 = match v4.clone() {
            Adt1::C0(_, h323, _, _, _, _, _, _, _, _, _) => {
                h323
            }
        };
        let a325 = v9;
        nat_mul(a324, a325)?
    };
    let a327 = v0;
    let a328 = match v1.clone() {
        Adt2::C0(h326, _, _, _, _, _, _, _) => {
            h326
        }
    };
    if nat_lt(a327, a328) {
        let a329 = v6;
        let a330 = 0u64;
        if nat_eq(a329, a330) {
            Ok(Adt3::C3(0u64, 0u64, match v2.clone() {
                Adt1::C0(h331, _, _, _, _, _, _, _, _, _, _) => {
                    h331
                }
            }, 0u64, 0u64, {
                let a339 = {
                    let a335 = {
                        let a333 = v8;
                        let a334 = match v2 {
                            Adt1::C0(_, _, _, _, _, _, h332, _, _, _, _) => {
                                h332
                            }
                        };
                        nat_mul(a333, a334)?
                    };
                    let a336 = v10;
                    nat_add(a335, a336)?
                };
                let a340 = {
                    let a337 = v5;
                    let a338 = 100u64;
                    if nat_eq(a337, a338) {
                        v11
                    } else {
                        0u64
                    }
                };
                nat_add(a339, a340)?
            }, 2u64, 0u64, 0u64, 0u64, v9, 4u64))
        } else {
            let a341 = v6;
            let a342 = 1u64;
            if nat_eq(a341, a342) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h343, _, _, _, _, _, _, _, _, _, _) => {
                        h343
                    }
                }, 0u64, {
                    let a344 = v5;
                    let a345 = 98u64;
                    if nat_eq(a344, a345) {
                        4u64
                    } else {
                        0u64
                    }
                }, {
                    let a353 = {
                        let a349 = {
                            let a347 = v8;
                            let a348 = match v3 {
                                Adt1::C0(_, _, _, _, _, _, h346, _, _, _, _) => {
                                    h346
                                }
                            };
                            nat_mul(a347, a348)?
                        };
                        let a350 = v10;
                        nat_add(a349, a350)?
                    };
                    let a354 = {
                        let a351 = v5;
                        let a352 = 99u64;
                        if nat_eq(a351, a352) {
                            v11
                        } else {
                            0u64
                        }
                    };
                    nat_add(a353, a354)?
                }, 2u64, 0u64, 0u64, 1u64, v9, 4u64))
            } else {
                let a355 = v6;
                let a356 = 2u64;
                if nat_eq(a355, a356) {
                    Ok(Adt3::C4(match v1 {
                        Adt2::C0(_, _, _, h357, _, _, _, _) => {
                            h357
                        }
                    }, match v4 {
                        Adt1::C0(h358, _, _, _, _, _, _, _, _, _, _) => {
                            h358
                        }
                    }, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt3::C3(0u64, 0u64, 0u64, match v4.clone() {
                        Adt1::C0(h359, _, _, _, _, _, _, _, _, _, _) => {
                            h359
                        }
                    }, 1u64, 0u64, 2u64, 0u64, 3u64, {
                        let a363 = {
                            let a361 = v8;
                            let a362 = match v4 {
                                Adt1::C0(_, _, _, _, _, _, h360, _, _, _, _) => {
                                    h360
                                }
                            };
                            nat_mul(a361, a362)?
                        };
                        let a364 = v10;
                        nat_add(a363, a364)?
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
        Adt2::C0(_, _, _, _, _, h365, _, _) => {
            h365
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h366, _) => {
            h366
        }
    };
    let v5: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h367) => {
            h367
        }
    };
    let v6: u64 = {
        let a369 = match v1.clone() {
            Adt2::C0(_, h368, _, _, _, _, _, _) => {
                h368
            }
        };
        let a370 = 3u64;
        if nat_eq(a369, a370) {
            0u64
        } else {
            let a371 = f22(v3.clone())?;
            let a372 = 2u64;
            nat_mul(a371, a372)?
        }
    };
    let a374 = match v1.clone() {
        Adt2::C0(h373, _, _, _, _, _, _, _) => {
            h373
        }
    };
    let a375 = v0;
    if nat_le(a374, a375) {
        Ok(Adt3::C1)
    } else {
        let a376 = v0;
        let a377 = v6;
        if nat_lt(a376, a377) {
            let v7: u64 = {
                let a378 = v0;
                let a379 = 2u64;
                let a380 = 0u64;
                nat_quot(a378, a379, a380)
            };
            let v8: u64 = {
                let a385 = 3u64;
                let a386 = {
                    let a382 = f34(v7, v5.clone())?;
                    let a383 = f16(match v5 {
                        Adt1::C0(h381, _, _, _, _, _, _, _, _, _, _) => {
                            h381
                        }
                    });
                    let a384 = 0u64;
                    nat_quot(a382, a383, a384)
                };
                nat_add(a385, a386)?
            };
            let a390 = {
                let a387 = v0;
                let a388 = 2u64;
                let a389 = 0u64;
                nat_rem(a387, a388, a389)
            };
            let a391 = 0u64;
            if nat_eq(a390, a391) {
                Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, v8, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
            } else {
                let a392 = v2;
                let a393 = 0u64;
                if nat_eq(a392, a393) {
                    Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 2u64, 1u64, 2u64, 0u64, 0u64, v8, 8u64, 8u64))
                } else {
                    Ok(Adt3::C1)
                }
            }
        } else {
            let v9: u64 = {
                let a394 = v0;
                let a395 = v6;
                nat_sub(a394, a395)
            };
            let v10: u64 = f23(match v3.clone() {
                Adt1::C0(h396, _, _, _, _, _, _, _, _, _, _) => {
                    h396
                }
            }, match v4.clone() {
                Adt1::C0(h397, _, _, _, _, _, _, _, _, _, _) => {
                    h397
                }
            }, match v5.clone() {
                Adt1::C0(h398, _, _, _, _, _, _, _, _, _, _) => {
                    h398
                }
            })?;
            let v11: u64 = {
                let a399 = v9;
                let a400 = v10;
                let a401 = 0u64;
                nat_quot(a399, a400, a401)
            };
            let v12: u64 = {
                let a402 = v9;
                let a403 = v10;
                let a404 = 0u64;
                nat_rem(a402, a403, a404)
            };
            let v13: u64 = {
                let a406 = match v3.clone() {
                    Adt1::C0(h405, _, _, _, _, _, _, _, _, _, _) => {
                        h405
                    }
                };
                let a407 = 0u64;
                if nat_eq(a406, a407) {
                    0u64
                } else {
                    1u64
                }
            };
            let v14: u64 = {
                let a409 = match v4.clone() {
                    Adt1::C0(h408, _, _, _, _, _, _, _, _, _, _) => {
                        h408
                    }
                };
                let a410 = 0u64;
                if nat_eq(a409, a410) {
                    0u64
                } else {
                    1u64
                }
            };
            let a411 = v12;
            let a412 = v13;
            if nat_lt(a411, a412) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h413, _, _, _, _, _, _, _, _, _, _) => {
                        h413
                    }
                }, 0u64, 0u64, f34(v11, v3.clone())?, 2u64, 0u64, 0u64, 0u64, f16(match v3 {
                    Adt1::C0(h414, _, _, _, _, _, _, _, _, _, _) => {
                        h414
                    }
                }), 4u64))
            } else {
                let a417 = v12;
                let a418 = {
                    let a415 = v13;
                    let a416 = v14;
                    nat_add(a415, a416)?
                };
                if nat_lt(a417, a418) {
                    Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                        Adt1::C0(h419, _, _, _, _, _, _, _, _, _, _) => {
                            h419
                        }
                    }, 0u64, 4u64, f35(v11, v3, v4.clone())?, 2u64, 0u64, 0u64, 1u64, f16(match v4 {
                        Adt1::C0(h420, _, _, _, _, _, _, _, _, _, _) => {
                            h420
                        }
                    }), 4u64))
                } else {
                    let a423 = v12;
                    let a424 = {
                        let a421 = v13;
                        let a422 = v14;
                        nat_add(a421, a422)?
                    };
                    if nat_eq(a423, a424) {
                        Ok(Adt3::C3(1u64, match v1 {
                            Adt2::C0(_, _, h425, _, _, _, _, _) => {
                                h425
                            }
                        }, 0u64, 0u64, {
                            let a426 = v13;
                            let a427 = 0u64;
                            if nat_eq(a426, a427) {
                                0u64
                            } else {
                                1u64
                            }
                        }, {
                            let a428 = v13;
                            let a429 = 0u64;
                            if nat_eq(a428, a429) {
                                f34(v11, v3.clone())?
                            } else {
                                0u64
                            }
                        }, {
                            let a430 = v14;
                            let a431 = 0u64;
                            if nat_eq(a430, a431) {
                                4u64
                            } else {
                                1u64
                            }
                        }, {
                            let a432 = v14;
                            let a433 = 0u64;
                            if nat_eq(a432, a433) {
                                f35(v11, v3, v4)?
                            } else {
                                1u64
                            }
                        }, {
                            let a435 = match v5.clone() {
                                Adt1::C0(h434, _, _, _, _, _, _, _, _, _, _) => {
                                    h434
                                }
                            };
                            let a436 = 0u64;
                            if nat_eq(a435, a436) {
                                3u64
                            } else {
                                0u64
                            }
                        }, {
                            let a438 = match v5.clone() {
                                Adt1::C0(h437, _, _, _, _, _, _, _, _, _, _) => {
                                    h437
                                }
                            };
                            let a439 = 0u64;
                            if nat_eq(a438, a439) {
                                f34(v11, v5)?
                            } else {
                                0u64
                            }
                        }, 4u64, 4u64))
                    } else {
                        Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                            Adt1::C0(h440, _, _, _, _, _, _, _, _, _, _) => {
                                h440
                            }
                        }, 1u64, 0u64, 2u64, 0u64, 3u64, f34(v11, v5.clone())?, 4u64, f16(match v5 {
                            Adt1::C0(h441, _, _, _, _, _, _, _, _, _, _) => {
                                h441
                            }
                        })))
                    }
                }
            }
        }
    }
}

pub fn f12(v0: Adt1, v1: Adt1) -> bool {
    let a444 = match v0.clone() {
        Adt1::C0(_, h442, _, _, _, _, _, _, _, _, _) => {
            h442
        }
    };
    let a445 = match v1.clone() {
        Adt1::C0(_, h443, _, _, _, _, _, _, _, _, _) => {
            h443
        }
    };
    if if if nat_eq(a444, a445) {
        let a448 = match v0.clone() {
            Adt1::C0(_, _, h446, _, _, _, _, _, _, _, _) => {
                h446
            }
        };
        let a449 = match v1.clone() {
            Adt1::C0(_, _, h447, _, _, _, _, _, _, _, _) => {
                h447
            }
        };
        nat_eq(a448, a449)
    } else {
        false
    } {
        let a452 = match v0.clone() {
            Adt1::C0(_, _, _, h450, _, _, _, _, _, _, _) => {
                h450
            }
        };
        let a453 = match v1.clone() {
            Adt1::C0(_, _, _, h451, _, _, _, _, _, _, _) => {
                h451
            }
        };
        nat_eq(a452, a453)
    } else {
        false
    } {
        let a456 = match v0 {
            Adt1::C0(_, _, _, _, h454, _, _, _, _, _, _) => {
                h454
            }
        };
        let a457 = match v1 {
            Adt1::C0(_, _, _, _, h455, _, _, _, _, _, _) => {
                h455
            }
        };
        nat_eq(a456, a457)
    } else {
        false
    }
}

pub fn f13(v0: Adt1, v1: Adt1) -> bool {
    if f15(v1.clone()) {
        f15(v0)
    } else {
        let a463 = {
            let a460 = match v0.clone() {
                Adt1::C0(_, h458, _, _, _, _, _, _, _, _, _) => {
                    h458
                }
            };
            let a461 = match v1.clone() {
                Adt1::C0(_, h459, _, _, _, _, _, _, _, _, _) => {
                    h459
                }
            };
            let a462 = 1u64;
            nat_rem(a460, a461, a462)
        };
        let a464 = 0u64;
        if if if nat_eq(a463, a464) {
            let a470 = {
                let a467 = match v0.clone() {
                    Adt1::C0(_, _, h465, _, _, _, _, _, _, _, _) => {
                        h465
                    }
                };
                let a468 = match v1.clone() {
                    Adt1::C0(_, _, h466, _, _, _, _, _, _, _, _) => {
                        h466
                    }
                };
                let a469 = 1u64;
                nat_rem(a467, a468, a469)
            };
            let a471 = 0u64;
            nat_eq(a470, a471)
        } else {
            false
        } {
            let a477 = {
                let a474 = match v0.clone() {
                    Adt1::C0(_, _, _, h472, _, _, _, _, _, _, _) => {
                        h472
                    }
                };
                let a475 = match v1.clone() {
                    Adt1::C0(_, _, _, h473, _, _, _, _, _, _, _) => {
                        h473
                    }
                };
                let a476 = 1u64;
                nat_rem(a474, a475, a476)
            };
            let a478 = 0u64;
            nat_eq(a477, a478)
        } else {
            false
        } {
            let a484 = {
                let a481 = match v0 {
                    Adt1::C0(_, _, _, _, h479, _, _, _, _, _, _) => {
                        h479
                    }
                };
                let a482 = match v1 {
                    Adt1::C0(_, _, _, _, h480, _, _, _, _, _, _) => {
                        h480
                    }
                };
                let a483 = 1u64;
                nat_rem(a481, a482, a483)
            };
            let a485 = 0u64;
            nat_eq(a484, a485)
        } else {
            false
        }
    }
}

pub fn f14(v0: u64, v1: u64, v2: u64) -> bool {
    let a486 = v0;
    let a487 = 0u64;
    if if if nat_eq(a486, a487) {
        let a488 = v1;
        let a489 = 0u64;
        nat_eq(a488, a489)
    } else {
        false
    } {
        let a490 = v2;
        let a491 = 0u64;
        nat_eq(a490, a491)
    } else {
        false
    } {
        true
    } else {
        let a492 = v0;
        let a493 = 1u64;
        if if nat_eq(a492, a493) {
            true
        } else {
            let a494 = v0;
            let a495 = 30u64;
            nat_eq(a494, a495)
        } {
            let a496 = v1;
            let a497 = v0;
            if if nat_eq(a496, a497) {
                let a498 = v2;
                let a499 = v0;
                nat_eq(a498, a499)
            } else {
                false
            } {
                true
            } else {
                let a500 = v1;
                let a501 = 0u64;
                if nat_eq(a500, a501) {
                    let a502 = v2;
                    let a503 = v0;
                    if nat_eq(a502, a503) {
                        true
                    } else {
                        let a504 = v2;
                        let a505 = 0u64;
                        nat_eq(a504, a505)
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
    f36(match v0.clone() {
        Adt1::C0(_, h506, _, _, _, _, _, _, _, _, _) => {
            h506
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h507, _, _, _, _, _, _, _, _) => {
            h507
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h508, _, _, _, _, _, _, _) => {
            h508
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, h509, _, _, _, _, _, _) => {
            h509
        }
    })
}

pub fn f16(v0: u64) -> u64 {
    let a510 = v0;
    let a511 = 0u64;
    if nat_eq(a510, a511) {
        4u64
    } else {
        let a512 = v0;
        let a513 = 1u64;
        if if nat_eq(a512, a513) {
            true
        } else {
            let a514 = v0;
            let a515 = 30u64;
            nat_eq(a514, a515)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f17(v0: Adt1) -> bool {
    let v1: u64 = f16(match v0.clone() {
        Adt1::C0(h516, _, _, _, _, _, _, _, _, _, _) => {
            h516
        }
    });
    let a521 = {
        let a518 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h517) => {
                h517
            }
        };
        let a519 = v1;
        let a520 = 1u64;
        nat_rem(a518, a519, a520)
    };
    let a522 = 0u64;
    if if nat_eq(a521, a522) {
        f37(match v0.clone() {
            Adt1::C0(_, h523, _, _, _, _, _, _, _, _, _) => {
                h523
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h524, _, _, _, _, _) => {
                h524
            }
        }, v1)
    } else {
        false
    } {
        f38(match v0.clone() {
            Adt1::C0(_, _, h525, _, _, _, _, _, _, _, _) => {
                h525
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h526, _, _, _, _, _, _, _) => {
                h526
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h527, _, _, _, _, _, _) => {
                h527
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h528, _, _, _, _) => {
                h528
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h529, _, _, _) => {
                h529
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h530, _, _) => {
                h530
            }
        }, v1)
    } else {
        false
    }
}

pub fn f18(v0: Adt1, v1: Adt1) -> R<bool> {
    let a538 = {
        let a536 = match v0.clone() {
            Adt1::C0(_, _, h531, _, _, _, _, _, _, _, _) => {
                h531
            }
        };
        let a537 = {
            let a534 = match v0.clone() {
                Adt1::C0(_, _, _, h532, _, _, _, _, _, _, _) => {
                    h532
                }
            };
            let a535 = match v0.clone() {
                Adt1::C0(_, _, _, _, h533, _, _, _, _, _, _) => {
                    h533
                }
            };
            nat_mul(a534, a535)?
        };
        nat_mul(a536, a537)?
    };
    let a539 = 9223372036854775807u64;
    if nat_le(a538, a539) {
        if f26(v1.clone()) {
            let a541 = match v1 {
                Adt1::C0(_, h540, _, _, _, _, _, _, _, _, _) => {
                    h540
                }
            };
            let a542 = 2147483647u64;
            Ok(nat_le(a541, a542))
        } else {
            let a544 = match v0 {
                Adt1::C0(_, h543, _, _, _, _, _, _, _, _, _) => {
                    h543
                }
            };
            let a545 = 2147483647u64;
            Ok(nat_le(a544, a545))
        }
    } else {
        Ok(false)
    }
}

pub fn f19(v0: Adt1, v1: Adt1) -> R<bool> {
    if f28(v0.clone(), v1.clone())? {
        Ok(true)
    } else {
        Ok(f39(v0, v1))
    }
}

pub fn f20(v0: Adt1) -> R<bool> {
    f40({
        let a548 = match v0.clone() {
            Adt1::C0(_, h546, _, _, _, _, _, _, _, _, _) => {
                h546
            }
        };
        let a549 = f16(match v0.clone() {
            Adt1::C0(h547, _, _, _, _, _, _, _, _, _, _) => {
                h547
            }
        });
        nat_mul(a548, a549)?
    }, match v0.clone() {
        Adt1::C0(_, _, h550, _, _, _, _, _, _, _, _) => {
            h550
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h551, _, _, _, _, _, _, _) => {
            h551
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h552, _, _, _, _, _, _) => {
            h552
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h553, _, _, _, _) => {
            h553
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h554, _, _, _) => {
            h554
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h555, _, _) => {
            h555
        }
    })
}

pub fn f21(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a562 = {
        let a556 = v0;
        let a557 = v2;
        nat_mul(a556, a557)?
    };
    let a563 = {
        let a558 = v1;
        let a559 = 0u64;
        if nat_eq(a558, a559) {
            0u64
        } else {
            let a560 = v0;
            let a561 = 2u64;
            nat_mul(a560, a561)?
        }
    };
    nat_add(a562, a563)
}

pub fn f22(v0: Adt1) -> R<u64> {
    let a572 = {
        let a566 = match v0.clone() {
            Adt1::C0(_, h564, _, _, _, _, _, _, _, _, _) => {
                h564
            }
        };
        let a567 = match v0.clone() {
            Adt1::C0(_, _, h565, _, _, _, _, _, _, _, _) => {
                h565
            }
        };
        nat_mul(a566, a567)?
    };
    let a573 = {
        let a570 = match v0.clone() {
            Adt1::C0(_, _, _, h568, _, _, _, _, _, _, _) => {
                h568
            }
        };
        let a571 = match v0 {
            Adt1::C0(_, _, _, _, h569, _, _, _, _, _, _) => {
                h569
            }
        };
        nat_mul(a570, a571)?
    };
    nat_mul(a572, a573)
}

pub fn f23(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a584 = {
        let a580 = {
            let a576 = 1u64;
            let a577 = {
                let a574 = v0;
                let a575 = 0u64;
                if nat_eq(a574, a575) {
                    0u64
                } else {
                    1u64
                }
            };
            nat_add(a576, a577)?
        };
        let a581 = {
            let a578 = v1;
            let a579 = 0u64;
            if nat_eq(a578, a579) {
                0u64
            } else {
                1u64
            }
        };
        nat_add(a580, a581)?
    };
    let a585 = {
        let a582 = v2;
        let a583 = 0u64;
        if nat_eq(a582, a583) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a584, a585)
}

pub fn f24(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f12(v0.clone(), v1.clone()) {
        let a588 = match v0.clone() {
            Adt1::C0(h586, _, _, _, _, _, _, _, _, _, _) => {
                h586
            }
        };
        let a589 = match v1.clone() {
            Adt1::C0(h587, _, _, _, _, _, _, _, _, _, _) => {
                h587
            }
        };
        nat_eq(a588, a589)
    } else {
        false
    } {
        let a592 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h590, _, _, _, _, _) => {
                h590
            }
        };
        let a593 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h591, _, _, _, _, _) => {
                h591
            }
        };
        nat_eq(a592, a593)
    } else {
        false
    } {
        let a596 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h594, _, _, _, _) => {
                h594
            }
        };
        let a597 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h595, _, _, _, _) => {
                h595
            }
        };
        nat_eq(a596, a597)
    } else {
        false
    } {
        let a600 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h598, _, _, _) => {
                h598
            }
        };
        let a601 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h599, _, _, _) => {
                h599
            }
        };
        nat_eq(a600, a601)
    } else {
        false
    } {
        let a604 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h602, _, _) => {
                h602
            }
        };
        let a605 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h603, _, _) => {
                h603
            }
        };
        nat_eq(a604, a605)
    } else {
        false
    } {
        let a607 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h606) => {
                h606
            }
        };
        let a608 = 0u64;
        if if nat_eq(a607, a608) {
            true
        } else {
            let a610 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h609) => {
                    h609
                }
            };
            let a611 = 0u64;
            nat_eq(a610, a611)
        } {
            true
        } else {
            let a614 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h612) => {
                    h612
                }
            };
            let a615 = match v1 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h613) => {
                    h613
                }
            };
            nat_eq(a614, a615)
        }
    } else {
        false
    }
}

pub fn f25(v0: u64) -> u64 {
    let a616 = v0;
    let a617 = 26u64;
    if nat_eq(a616, a617) {
        4u64
    } else {
        let a618 = v0;
        let a619 = 27u64;
        if nat_eq(a618, a619) {
            8u64
        } else {
            0u64
        }
    }
}

pub fn f26(v0: Adt1) -> bool {
    let a621 = match v0.clone() {
        Adt1::C0(_, h620, _, _, _, _, _, _, _, _, _) => {
            h620
        }
    };
    let a622 = 1u64;
    if nat_eq(a621, a622) {
        true
    } else {
        let a625 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h623, _, _, _, _, _) => {
                h623
            }
        };
        let a626 = f16(match v0 {
            Adt1::C0(h624, _, _, _, _, _, _, _, _, _, _) => {
                h624
            }
        });
        nat_eq(a625, a626)
    }
}

pub fn f27(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a628 = match v0.clone() {
            Adt1::C0(_, h627, _, _, _, _, _, _, _, _, _) => {
                h627
            }
        };
        let a629 = 4u64;
        nat_mul(a628, a629)?
    };
    let v2: u64 = {
        let a632 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h630, _, _, _, _) => {
                h630
            }
        };
        let a633 = match v0.clone() {
            Adt1::C0(_, _, h631, _, _, _, _, _, _, _, _) => {
                h631
            }
        };
        nat_mul(a632, a633)?
    };
    let v3: u64 = {
        let a635 = v2;
        let a636 = match v0.clone() {
            Adt1::C0(_, _, _, h634, _, _, _, _, _, _, _) => {
                h634
            }
        };
        nat_mul(a635, a636)?
    };
    let a638 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h637, _, _, _, _, _) => {
            h637
        }
    };
    let a639 = 4u64;
    if if if if nat_eq(a638, a639) {
        let a644 = {
            let a641 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h640) => {
                    h640
                }
            };
            let a642 = 4u64;
            let a643 = 1u64;
            nat_rem(a641, a642, a643)
        };
        let a645 = 0u64;
        nat_eq(a644, a645)
    } else {
        false
    } {
        let a653 = {
            let a651 = match v0.clone() {
                Adt1::C0(_, _, h646, _, _, _, _, _, _, _, _) => {
                    h646
                }
            };
            let a652 = {
                let a649 = match v0.clone() {
                    Adt1::C0(_, _, _, h647, _, _, _, _, _, _, _) => {
                        h647
                    }
                };
                let a650 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h648, _, _, _, _, _, _) => {
                        h648
                    }
                };
                nat_mul(a649, a650)?
            };
            nat_mul(a651, a652)?
        };
        let a654 = 1u64;
        if nat_le(a653, a654) {
            true
        } else {
            let a656 = v1;
            let a657 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h655, _, _, _, _) => {
                    h655
                }
            };
            if nat_le(a656, a657) {
                let a662 = {
                    let a659 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, h658, _, _, _, _) => {
                            h658
                        }
                    };
                    let a660 = 4u64;
                    let a661 = 1u64;
                    nat_rem(a659, a660, a661)
                };
                let a663 = 0u64;
                nat_eq(a662, a663)
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a665 = match v0.clone() {
            Adt1::C0(_, _, _, h664, _, _, _, _, _, _, _) => {
                h664
            }
        };
        let a666 = 1u64;
        if nat_le(a665, a666) {
            true
        } else {
            let a668 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h667, _, _, _) => {
                    h667
                }
            };
            let a669 = v2;
            nat_eq(a668, a669)
        }
    } else {
        false
    } {
        let a671 = match v0.clone() {
            Adt1::C0(_, _, _, _, h670, _, _, _, _, _, _) => {
                h670
            }
        };
        let a672 = 1u64;
        if nat_le(a671, a672) {
            Ok(true)
        } else {
            let a674 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, h673, _, _) => {
                    h673
                }
            };
            let a675 = v3;
            Ok(nat_eq(a674, a675))
        }
    } else {
        Ok(false)
    }
}

pub fn f28(v0: Adt1, v1: Adt1) -> R<bool> {
    let a677 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h676, _) => {
            h676
        }
    };
    let a678 = 0u64;
    if nat_eq(a677, a678) {
        Ok(true)
    } else {
        let a680 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h679, _) => {
                h679
            }
        };
        let a681 = 0u64;
        if nat_eq(a680, a681) {
            Ok(true)
        } else {
            let a683 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h682) => {
                    h682
                }
            };
            let a684 = 0u64;
            if nat_eq(a683, a684) {
                Ok(true)
            } else {
                let a686 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h685) => {
                        h685
                    }
                };
                let a687 = 0u64;
                if nat_eq(a686, a687) {
                    Ok(true)
                } else {
                    let a693 = {
                        let a690 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h688) => {
                                h688
                            }
                        };
                        let a691 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h689, _) => {
                                h689
                            }
                        };
                        nat_add(a690, a691)?
                    };
                    let a694 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h692) => {
                            h692
                        }
                    };
                    if nat_le(a693, a694) {
                        Ok(true)
                    } else {
                        let a700 = {
                            let a697 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h695) => {
                                    h695
                                }
                            };
                            let a698 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h696, _) => {
                                    h696
                                }
                            };
                            nat_add(a697, a698)?
                        };
                        let a701 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h699) => {
                                h699
                            }
                        };
                        Ok(nat_le(a700, a701))
                    }
                }
            }
        }
    }
}

pub fn f29(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a704 = match v0.clone() {
            Adt1::C0(_, _, h702, _, _, _, _, _, _, _, _) => {
                h702
            }
        };
        let a705 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h703, _, _, _, _) => {
                h703
            }
        };
        nat_mul(a704, a705)?
    };
    if if f26(v0.clone()) {
        let a707 = match v0.clone() {
            Adt1::C0(_, _, _, h706, _, _, _, _, _, _, _) => {
                h706
            }
        };
        let a708 = 1u64;
        if nat_eq(a707, a708) {
            true
        } else {
            let a710 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h709, _, _, _) => {
                    h709
                }
            };
            let a711 = v1;
            nat_eq(a710, a711)
        }
    } else {
        false
    } {
        let a713 = match v0.clone() {
            Adt1::C0(_, _, _, _, h712, _, _, _, _, _, _) => {
                h712
            }
        };
        let a714 = 1u64;
        if nat_eq(a713, a714) {
            Ok(true)
        } else {
            let a719 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h715, _, _) => {
                    h715
                }
            };
            let a720 = {
                let a717 = v1;
                let a718 = match v0 {
                    Adt1::C0(_, _, _, h716, _, _, _, _, _, _, _) => {
                        h716
                    }
                };
                nat_mul(a717, a718)?
            };
            Ok(nat_eq(a719, a720))
        }
    } else {
        Ok(false)
    }
}

pub fn f30(v0: u64, v1: Adt1, v2: u64, v3: u64) -> R<Adt3> {
    let v4: u64 = {
        let a721 = v0;
        let a722 = 2u64;
        let a723 = 0u64;
        nat_quot(a721, a722, a723)
    };
    let v5: u64 = f25(match v1.clone() {
        Adt1::C0(h724, _, _, _, _, _, _, _, _, _, _) => {
            h724
        }
    });
    let a728 = {
        let a725 = v0;
        let a726 = 2u64;
        let a727 = 0u64;
        nat_rem(a725, a726, a727)
    };
    let a729 = 0u64;
    if nat_eq(a728, a729) {
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 4u64, f34(v4, v1)?, 2u64, 0u64, 0u64, 2u64, v5, 8u64))
    } else {
        let a730 = v3;
        let a731 = v2;
        if if nat_lt(a730, a731) {
            let a734 = v3;
            let a735 = {
                let a732 = v5;
                let a733 = 4u64;
                if nat_eq(a732, a733) {
                    2147483648u64
                } else {
                    9223372036854775808u64
                }
            };
            nat_lt(a734, a735)
        } else {
            false
        } {
            Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, {
                let a736 = 3u64;
                let a737 = v4;
                nat_add(a736, a737)?
            }, 8u64, 8u64))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f31(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64) -> R<Adt3> {
    let v5: u64 = {
        let a739 = 1u64;
        let a740 = match v1.clone() {
            Adt1::C0(_, h738, _, _, _, _, _, _, _, _, _) => {
                h738
            }
        };
        nat_add(a739, a740)?
    };
    let v6: u64 = {
        let a741 = v0;
        let a742 = v5;
        let a743 = 0u64;
        nat_rem(a741, a742, a743)
    };
    let v7: u64 = {
        let a744 = v0;
        let a745 = v5;
        let a746 = 0u64;
        nat_quot(a744, a745, a746)
    };
    let v8: u64 = {
        let a748 = v7;
        let a749 = match v1.clone() {
            Adt1::C0(_, _, h747, _, _, _, _, _, _, _, _) => {
                h747
            }
        };
        let a750 = 0u64;
        nat_rem(a748, a749, a750)
    };
    let v9: u64 = {
        let a752 = v7;
        let a753 = match v1.clone() {
            Adt1::C0(_, _, h751, _, _, _, _, _, _, _, _) => {
                h751
            }
        };
        let a754 = 0u64;
        nat_quot(a752, a753, a754)
    };
    let v10: u64 = {
        let a756 = v9;
        let a757 = match v1.clone() {
            Adt1::C0(_, _, _, h755, _, _, _, _, _, _, _) => {
                h755
            }
        };
        let a758 = 0u64;
        nat_rem(a756, a757, a758)
    };
    let v11: u64 = {
        let a760 = v9;
        let a761 = match v1.clone() {
            Adt1::C0(_, _, _, h759, _, _, _, _, _, _, _) => {
                h759
            }
        };
        let a762 = 0u64;
        nat_quot(a760, a761, a762)
    };
    let a763 = v6;
    let a764 = 0u64;
    if nat_eq(a763, a764) {
        let v12: u64 = {
            let a781 = v8;
            let a782 = {
                let a779 = match v2.clone() {
                    Adt1::C0(_, h765, _, _, _, _, _, _, _, _, _) => {
                        h765
                    }
                };
                let a780 = {
                    let a777 = {
                        let a767 = v10;
                        let a768 = match v2.clone() {
                            Adt1::C0(_, _, h766, _, _, _, _, _, _, _, _) => {
                                h766
                            }
                        };
                        let a769 = 0u64;
                        nat_rem(a767, a768, a769)
                    };
                    let a778 = {
                        let a775 = match v2.clone() {
                            Adt1::C0(_, _, h770, _, _, _, _, _, _, _, _) => {
                                h770
                            }
                        };
                        let a776 = {
                            let a772 = v11;
                            let a773 = match v2 {
                                Adt1::C0(_, _, _, h771, _, _, _, _, _, _, _) => {
                                    h771
                                }
                            };
                            let a774 = 0u64;
                            nat_rem(a772, a773, a774)
                        };
                        nat_mul(a775, a776)?
                    };
                    nat_add(a777, a778)?
                };
                nat_mul(a779, a780)?
            };
            nat_add(a781, a782)?
        };
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, {
            let a783 = 3u64;
            let a784 = v12;
            nat_add(a783, a784)?
        }, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
    } else {
        let a786 = v4;
        let a787 = match v3.clone() {
            Adt1::C0(_, _, h785, _, _, _, _, _, _, _, _) => {
                h785
            }
        };
        if nat_lt(a786, a787) {
            let v13: u64 = {
                let a788 = v6;
                let a789 = 1u64;
                nat_sub(a788, a789)
            };
            let v14: u64 = f41(v13, v8, v10, v11, match v1.clone() {
                Adt1::C0(_, _, _, _, _, h790, _, _, _, _, _) => {
                    h790
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, h791, _, _, _, _) => {
                    h791
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h792, _, _, _) => {
                    h792
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h793, _, _) => {
                    h793
                }
            })?;
            let v15: u64 = f41(v13, v4, v10, v11, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h794, _, _, _, _, _) => {
                    h794
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h795, _, _, _, _) => {
                    h795
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h796, _, _, _) => {
                    h796
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h797, _, _) => {
                    h797
                }
            })?;
            Ok(Adt3::C3(0u64, 0u64, match v1.clone() {
                Adt1::C0(h798, _, _, _, _, _, _, _, _, _, _) => {
                    h798
                }
            }, match v3.clone() {
                Adt1::C0(h799, _, _, _, _, _, _, _, _, _, _) => {
                    h799
                }
            }, 0u64, v14, 2u64, 0u64, 3u64, v15, f16(match v1 {
                Adt1::C0(h800, _, _, _, _, _, _, _, _, _, _) => {
                    h800
                }
            }), f16(match v3 {
                Adt1::C0(h801, _, _, _, _, _, _, _, _, _, _) => {
                    h801
                }
            })))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f32(v0: u64, v1: u64, v2: u64) -> Adt3 {
    Adt3::C3(2u64, 0u64, 0u64, 0u64, 1u64, v0, v1, v2, 0u64, 2u64, 4u64, 4u64)
}

pub fn f33(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt3 {
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

pub fn f34(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a803 = v0;
        let a804 = match v1.clone() {
            Adt1::C0(_, h802, _, _, _, _, _, _, _, _, _) => {
                h802
            }
        };
        let a805 = 0u64;
        nat_quot(a803, a804, a805)
    };
    let v3: u64 = {
        let a807 = v2;
        let a808 = match v1.clone() {
            Adt1::C0(_, _, h806, _, _, _, _, _, _, _, _) => {
                h806
            }
        };
        let a809 = 0u64;
        nat_quot(a807, a808, a809)
    };
    f41({
        let a811 = v0;
        let a812 = match v1.clone() {
            Adt1::C0(_, h810, _, _, _, _, _, _, _, _, _) => {
                h810
            }
        };
        let a813 = 0u64;
        nat_rem(a811, a812, a813)
    }, {
        let a815 = v2;
        let a816 = match v1.clone() {
            Adt1::C0(_, _, h814, _, _, _, _, _, _, _, _) => {
                h814
            }
        };
        let a817 = 0u64;
        nat_rem(a815, a816, a817)
    }, {
        let a819 = v3;
        let a820 = match v1.clone() {
            Adt1::C0(_, _, _, h818, _, _, _, _, _, _, _) => {
                h818
            }
        };
        let a821 = 0u64;
        nat_rem(a819, a820, a821)
    }, {
        let a823 = v3;
        let a824 = match v1.clone() {
            Adt1::C0(_, _, _, h822, _, _, _, _, _, _, _) => {
                h822
            }
        };
        let a825 = 0u64;
        nat_quot(a823, a824, a825)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h826, _, _, _, _, _) => {
            h826
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h827, _, _, _, _) => {
            h827
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h828, _, _, _) => {
            h828
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h829, _, _) => {
            h829
        }
    })
}

pub fn f35(v0: u64, v1: Adt1, v2: Adt1) -> R<u64> {
    let v3: u64 = {
        let a831 = v0;
        let a832 = match v1.clone() {
            Adt1::C0(_, h830, _, _, _, _, _, _, _, _, _) => {
                h830
            }
        };
        let a833 = 0u64;
        nat_quot(a831, a832, a833)
    };
    let v4: u64 = {
        let a835 = v3;
        let a836 = match v1.clone() {
            Adt1::C0(_, _, h834, _, _, _, _, _, _, _, _) => {
                h834
            }
        };
        let a837 = 0u64;
        nat_quot(a835, a836, a837)
    };
    f41({
        let a843 = {
            let a839 = v0;
            let a840 = match v1.clone() {
                Adt1::C0(_, h838, _, _, _, _, _, _, _, _, _) => {
                    h838
                }
            };
            let a841 = 0u64;
            nat_rem(a839, a840, a841)
        };
        let a844 = match v2.clone() {
            Adt1::C0(_, h842, _, _, _, _, _, _, _, _, _) => {
                h842
            }
        };
        let a845 = 0u64;
        nat_rem(a843, a844, a845)
    }, {
        let a851 = {
            let a847 = v3;
            let a848 = match v1.clone() {
                Adt1::C0(_, _, h846, _, _, _, _, _, _, _, _) => {
                    h846
                }
            };
            let a849 = 0u64;
            nat_rem(a847, a848, a849)
        };
        let a852 = match v2.clone() {
            Adt1::C0(_, _, h850, _, _, _, _, _, _, _, _) => {
                h850
            }
        };
        let a853 = 0u64;
        nat_rem(a851, a852, a853)
    }, {
        let a859 = {
            let a855 = v4;
            let a856 = match v1.clone() {
                Adt1::C0(_, _, _, h854, _, _, _, _, _, _, _) => {
                    h854
                }
            };
            let a857 = 0u64;
            nat_rem(a855, a856, a857)
        };
        let a860 = match v2.clone() {
            Adt1::C0(_, _, _, h858, _, _, _, _, _, _, _) => {
                h858
            }
        };
        let a861 = 0u64;
        nat_rem(a859, a860, a861)
    }, {
        let a867 = {
            let a863 = v4;
            let a864 = match v1 {
                Adt1::C0(_, _, _, h862, _, _, _, _, _, _, _) => {
                    h862
                }
            };
            let a865 = 0u64;
            nat_quot(a863, a864, a865)
        };
        let a868 = match v2.clone() {
            Adt1::C0(_, _, _, _, h866, _, _, _, _, _, _) => {
                h866
            }
        };
        let a869 = 0u64;
        nat_rem(a867, a868, a869)
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h870, _, _, _, _, _) => {
            h870
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h871, _, _, _, _) => {
            h871
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h872, _, _, _) => {
            h872
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h873, _, _) => {
            h873
        }
    })
}

pub fn f36(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a874 = v0;
    let a875 = 0u64;
    if if if nat_eq(a874, a875) {
        true
    } else {
        let a876 = v1;
        let a877 = 0u64;
        nat_eq(a876, a877)
    } {
        true
    } else {
        let a878 = v2;
        let a879 = 0u64;
        nat_eq(a878, a879)
    } {
        true
    } else {
        let a880 = v3;
        let a881 = 0u64;
        nat_eq(a880, a881)
    }
}

pub fn f37(v0: u64, v1: u64, v2: u64) -> bool {
    let a882 = v0;
    let a883 = 1u64;
    if nat_le(a882, a883) {
        true
    } else {
        let a887 = {
            let a884 = v1;
            let a885 = v2;
            let a886 = 1u64;
            nat_rem(a884, a885, a886)
        };
        let a888 = 0u64;
        nat_eq(a887, a888)
    }
}

pub fn f38(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f37(v0, v3, v6) {
        f37(v1, v4, v6)
    } else {
        false
    } {
        f37(v2, v5, v6)
    } else {
        false
    }
}

pub fn f39(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f12(v0.clone(), v1.clone()) {
        let a891 = match v0.clone() {
            Adt1::C0(h889, _, _, _, _, _, _, _, _, _, _) => {
                h889
            }
        };
        let a892 = match v1.clone() {
            Adt1::C0(h890, _, _, _, _, _, _, _, _, _, _) => {
                h890
            }
        };
        nat_eq(a891, a892)
    } else {
        false
    } {
        let a895 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h893) => {
                h893
            }
        };
        let a896 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h894) => {
                h894
            }
        };
        nat_eq(a895, a896)
    } else {
        false
    } {
        let a899 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h897, _, _, _, _, _) => {
                h897
            }
        };
        let a900 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h898, _, _, _, _, _) => {
                h898
            }
        };
        nat_eq(a899, a900)
    } else {
        false
    } {
        let a903 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h901, _, _, _, _) => {
                h901
            }
        };
        let a904 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h902, _, _, _, _) => {
                h902
            }
        };
        nat_eq(a903, a904)
    } else {
        false
    } {
        let a907 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h905, _, _, _) => {
                h905
            }
        };
        let a908 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h906, _, _, _) => {
                h906
            }
        };
        nat_eq(a907, a908)
    } else {
        false
    } {
        let a911 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h909, _, _) => {
                h909
            }
        };
        let a912 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h910, _, _) => {
                h910
            }
        };
        nat_eq(a911, a912)
    } else {
        false
    }
}

pub fn f40(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f42(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f42(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f42(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f42(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f42(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f42(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f41(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a925 = {
        let a921 = {
            let a917 = {
                let a913 = v0;
                let a914 = v4;
                nat_mul(a913, a914)?
            };
            let a918 = {
                let a915 = v1;
                let a916 = v5;
                nat_mul(a915, a916)?
            };
            nat_add(a917, a918)?
        };
        let a922 = {
            let a919 = v2;
            let a920 = v6;
            nat_mul(a919, a920)?
        };
        nat_add(a921, a922)?
    };
    let a926 = {
        let a923 = v3;
        let a924 = v7;
        nat_mul(a923, a924)?
    };
    nat_add(a925, a926)
}

pub fn f42(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a927 = v1;
    let a928 = 1u64;
    if if if nat_le(a927, a928) {
        true
    } else {
        let a929 = v0;
        let a930 = v2;
        nat_le(a929, a930)
    } {
        let a931 = v3;
        let a932 = 1u64;
        if nat_le(a931, a932) {
            true
        } else {
            let a939 = {
                let a937 = v0;
                let a938 = {
                    let a935 = {
                        let a933 = v1;
                        let a934 = 1u64;
                        nat_sub(a933, a934)
                    };
                    let a936 = v2;
                    nat_mul(a935, a936)?
                };
                nat_add(a937, a938)?
            };
            let a940 = v4;
            nat_le(a939, a940)
        }
    } else {
        false
    } {
        let a941 = v5;
        let a942 = 1u64;
        if nat_le(a941, a942) {
            Ok(true)
        } else {
            let a955 = {
                let a953 = {
                    let a947 = v0;
                    let a948 = {
                        let a945 = {
                            let a943 = v1;
                            let a944 = 1u64;
                            nat_sub(a943, a944)
                        };
                        let a946 = v2;
                        nat_mul(a945, a946)?
                    };
                    nat_add(a947, a948)?
                };
                let a954 = {
                    let a951 = {
                        let a949 = v3;
                        let a950 = 1u64;
                        nat_sub(a949, a950)
                    };
                    let a952 = v4;
                    nat_mul(a951, a952)?
                };
                nat_add(a953, a954)?
            };
            let a956 = v6;
            Ok(nat_le(a955, a956))
        }
    } else {
        Ok(false)
    }
}

pub fn run(p0: Adt0) -> R<Adt3> {
    f0(p0)
}
