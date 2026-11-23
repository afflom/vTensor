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
    C6(u64, u64),
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
        Adt0::C1(v25, v26, v27) => {
            let a19 = match v26.clone() {
                Adt2::C0(_, _, h18, _, _, _, _, _) => {
                    h18
                }
            };
            let a20 = 33u64;
            if nat_eq(a19, a20) {
                f8(v25, v26)
            } else {
                let a22 = match v26.clone() {
                    Adt2::C0(_, _, h21, _, _, _, _, _) => {
                        h21
                    }
                };
                let a23 = 66u64;
                if if nat_eq(a22, a23) {
                    true
                } else {
                    let a25 = match v26.clone() {
                        Adt2::C0(_, _, h24, _, _, _, _, _) => {
                            h24
                        }
                    };
                    let a26 = 67u64;
                    nat_eq(a25, a26)
                } {
                    f9(v25, v26)
                } else {
                    let a28 = match v26.clone() {
                        Adt2::C0(_, _, h27, _, _, _, _, _) => {
                            h27
                        }
                    };
                    let a29 = 41u64;
                    if nat_eq(a28, a29) {
                        f10(v25, v26, v27)
                    } else {
                        let a31 = 98u64;
                        let a32 = match v26.clone() {
                            Adt2::C0(_, _, h30, _, _, _, _, _) => {
                                h30
                            }
                        };
                        if nat_le(a31, a32) {
                            let a34 = match v26.clone() {
                                Adt2::C0(_, _, _, h33, _, _, _, _) => {
                                    h33
                                }
                            };
                            let a35 = 6u64;
                            if nat_eq(a34, a35) {
                                f11(v25, v26, v27)
                            } else {
                                f12(v25, v26)
                            }
                        } else {
                            f13(v25, v26, v27)
                        }
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
            let a8 = 33u64;
            if nat_eq(a7, a8) {
                if v18 {
                    Ok(Adt3::C1)
                } else {
                    let m9 = v20;
                    match m9 {
                        None => {
                            Ok(Adt3::C1)
                        }
                        Some(v22) => {
                            f4(v19, v22, v21)
                        }
                    }
                }
            } else {
                let a10 = v14;
                let a11 = 66u64;
                if if nat_eq(a10, a11) {
                    true
                } else {
                    let a12 = v14;
                    let a13 = 67u64;
                    nat_eq(a12, a13)
                } {
                    if v18 {
                        Ok(Adt3::C1)
                    } else {
                        let m14 = v20;
                        match m14 {
                            None => {
                                f5(v14, v15, v19, v21)
                            }
                            Some(_) => {
                                Ok(Adt3::C1)
                            }
                        }
                    }
                } else {
                    let a15 = v14;
                    let a16 = 98u64;
                    if nat_eq(a15, a16) {
                        if if f6(v15) {
                            !v18
                        } else {
                            false
                        } {
                            let m17 = v20;
                            match m17 {
                                None => {
                                    f7(v15, false, v16, v17, v19.clone(), v19, v21)
                                }
                                Some(v24) => {
                                    f7(v15, true, v16, v17, v19, v24, v21)
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
}

pub fn f1(v0: u64) -> Option<u64> {
    let a36 = v0;
    let a37 = 2u64;
    if nat_eq(a36, a37) {
        Some(0u64)
    } else {
        let a38 = v0;
        let a39 = 5u64;
        if nat_eq(a38, a39) {
            Some(1u64)
        } else {
            let a40 = v0;
            let a41 = 6u64;
            if nat_eq(a40, a41) {
                Some(2u64)
            } else {
                let a42 = v0;
                let a43 = 7u64;
                if nat_eq(a42, a43) {
                    Some(3u64)
                } else {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f2(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if if f14(v1.clone(), v3.clone()) {
        f15(v1.clone(), v2.clone())
    } else {
        false
    } {
        f16(match v1.clone() {
            Adt1::C0(h44, _, _, _, _, _, _, _, _, _, _) => {
                h44
            }
        }, match v2.clone() {
            Adt1::C0(h45, _, _, _, _, _, _, _, _, _, _) => {
                h45
            }
        }, match v3.clone() {
            Adt1::C0(h46, _, _, _, _, _, _, _, _, _, _) => {
                h46
            }
        })
    } else {
        false
    } {
        if f17(v1.clone()) {
            true
        } else {
            let a49 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, h47, _, _, _, _, _) => {
                    h47
                }
            };
            let a50 = f18(match v1.clone() {
                Adt1::C0(h48, _, _, _, _, _, _, _, _, _, _) => {
                    h48
                }
            });
            if if if if if if if nat_eq(a49, a50) {
                let a53 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h51, _, _, _, _, _) => {
                        h51
                    }
                };
                let a54 = f18(match v3.clone() {
                    Adt1::C0(h52, _, _, _, _, _, _, _, _, _, _) => {
                        h52
                    }
                });
                nat_eq(a53, a54)
            } else {
                false
            } {
                f19(v1.clone())
            } else {
                false
            } {
                f19(v2.clone())
            } else {
                false
            } {
                f19(v3.clone())
            } else {
                false
            } {
                f20(v1.clone(), v2.clone())?
            } else {
                false
            } {
                f21(v1.clone(), v3.clone())?
            } else {
                false
            } {
                f21(v2.clone(), v3.clone())?
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v4: u64 = if if f17(v1.clone()) {
            true
        } else {
            f22(v3.clone())?
        } {
            0u64
        } else {
            let a57 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h55, _) => {
                    h55
                }
            };
            let a58 = f18(match v3.clone() {
                Adt1::C0(h56, _, _, _, _, _, _, _, _, _, _) => {
                    h56
                }
            });
            let a59 = 0u64;
            nat_quot(a57, a58, a59)
        };
        Ok(Adt3::C2(Adt2::C0(f23(f24(v1.clone())?, v4, f25(match v1.clone() {
            Adt1::C0(h60, _, _, _, _, _, _, _, _, _, _) => {
                h60
            }
        }, match v2.clone() {
            Adt1::C0(h61, _, _, _, _, _, _, _, _, _, _) => {
                h61
            }
        }, match v3.clone() {
            Adt1::C0(h62, _, _, _, _, _, _, _, _, _, _) => {
                h62
            }
        })?)?, {
            let a63 = 3u64;
            let a64 = v4;
            nat_add(a63, a64)?
        }, v0, 0u64, 0u64, v1, v2, v3)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f3(v0: Adt1, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if if if if if if if if if if if if if f26(v2, v3.clone()) {
        let a66 = match v0.clone() {
            Adt1::C0(h65, _, _, _, _, _, _, _, _, _, _) => {
                h65
            }
        };
        let a67 = 0u64;
        if nat_eq(a66, a67) {
            true
        } else {
            let a69 = match v0.clone() {
                Adt1::C0(h68, _, _, _, _, _, _, _, _, _, _) => {
                    h68
                }
            };
            let a70 = 1u64;
            nat_eq(a69, a70)
        }
    } else {
        false
    } {
        let a72 = 0u64;
        let a73 = f27(match v3.clone() {
            Adt1::C0(h71, _, _, _, _, _, _, _, _, _, _) => {
                h71
            }
        });
        nat_lt(a72, a73)
    } else {
        false
    } {
        let a75 = 0u64;
        let a76 = f28(match v1.clone() {
            Adt1::C0(h74, _, _, _, _, _, _, _, _, _, _) => {
                h74
            }
        });
        nat_lt(a75, a76)
    } else {
        false
    } {
        let a79 = match v0.clone() {
            Adt1::C0(_, h77, _, _, _, _, _, _, _, _, _) => {
                h77
            }
        };
        let a80 = match v3.clone() {
            Adt1::C0(_, h78, _, _, _, _, _, _, _, _, _) => {
                h78
            }
        };
        nat_eq(a79, a80)
    } else {
        false
    } {
        let a83 = match v0.clone() {
            Adt1::C0(_, _, _, h81, _, _, _, _, _, _, _) => {
                h81
            }
        };
        let a84 = match v3.clone() {
            Adt1::C0(_, _, _, h82, _, _, _, _, _, _, _) => {
                h82
            }
        };
        nat_eq(a83, a84)
    } else {
        false
    } {
        let a87 = match v0.clone() {
            Adt1::C0(_, _, _, _, h85, _, _, _, _, _, _) => {
                h85
            }
        };
        let a88 = match v3.clone() {
            Adt1::C0(_, _, _, _, h86, _, _, _, _, _, _) => {
                h86
            }
        };
        nat_eq(a87, a88)
    } else {
        false
    } {
        let a91 = match v0.clone() {
            Adt1::C0(_, _, h89, _, _, _, _, _, _, _, _) => {
                h89
            }
        };
        let a92 = match v1.clone() {
            Adt1::C0(_, h90, _, _, _, _, _, _, _, _, _) => {
                h90
            }
        };
        nat_eq(a91, a92)
    } else {
        false
    } {
        let a94 = match v1.clone() {
            Adt1::C0(_, _, _, _, h93, _, _, _, _, _, _) => {
                h93
            }
        };
        let a95 = 1u64;
        nat_eq(a94, a95)
    } else {
        false
    } {
        let a97 = 0u64;
        let a98 = match v1.clone() {
            Adt1::C0(_, _, h96, _, _, _, _, _, _, _, _) => {
                h96
            }
        };
        nat_lt(a97, a98)
    } else {
        false
    } {
        let a100 = 0u64;
        let a101 = match v1.clone() {
            Adt1::C0(_, _, _, h99, _, _, _, _, _, _, _) => {
                h99
            }
        };
        nat_lt(a100, a101)
    } else {
        false
    } {
        let a107 = {
            let a104 = match v0.clone() {
                Adt1::C0(_, _, _, h102, _, _, _, _, _, _, _) => {
                    h102
                }
            };
            let a105 = match v1.clone() {
                Adt1::C0(_, _, h103, _, _, _, _, _, _, _, _) => {
                    h103
                }
            };
            let a106 = 0u64;
            nat_rem(a104, a105, a106)
        };
        let a108 = 0u64;
        nat_eq(a107, a108)
    } else {
        false
    } {
        let a114 = {
            let a111 = match v0.clone() {
                Adt1::C0(_, _, _, _, h109, _, _, _, _, _, _) => {
                    h109
                }
            };
            let a112 = match v1.clone() {
                Adt1::C0(_, _, _, h110, _, _, _, _, _, _, _) => {
                    h110
                }
            };
            let a113 = 0u64;
            nat_rem(a111, a112, a113)
        };
        let a115 = 0u64;
        nat_eq(a114, a115)
    } else {
        false
    } {
        f29(v0.clone())
    } else {
        false
    } {
        let a117 = match v3.clone() {
            Adt1::C0(_, h116, _, _, _, _, _, _, _, _, _) => {
                h116
            }
        };
        let a118 = 1u64;
        if nat_eq(a117, a118) {
            true
        } else {
            let a121 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h119, _, _, _, _, _) => {
                    h119
                }
            };
            let a122 = f27(match v3.clone() {
                Adt1::C0(h120, _, _, _, _, _, _, _, _, _, _) => {
                    h120
                }
            });
            nat_eq(a121, a122)
        }
    } else {
        false
    } {
        let v4: u64 = f24(v1.clone())?;
        let v5: u64 = {
            let a128 = match v0.clone() {
                Adt1::C0(_, _, h123, _, _, _, _, _, _, _, _) => {
                    h123
                }
            };
            let a129 = {
                let a126 = match v0.clone() {
                    Adt1::C0(_, _, _, h124, _, _, _, _, _, _, _) => {
                        h124
                    }
                };
                let a127 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h125, _, _, _, _, _, _) => {
                        h125
                    }
                };
                nat_mul(a126, a127)?
            };
            nat_mul(a128, a129)?
        };
        let v6: u64 = if f17(v0.clone()) {
            0u64
        } else {
            let a137 = {
                let a130 = 2u64;
                let a131 = v4;
                nat_mul(a130, a131)?
            };
            let a138 = {
                let a135 = v5;
                let a136 = {
                    let a133 = 1u64;
                    let a134 = f30(v0.clone(), match v3.clone() {
                        Adt1::C0(h132, _, _, _, _, _, _, _, _, _, _) => {
                            h132
                        }
                    })?;
                    nat_add(a133, a134)?
                };
                nat_mul(a135, a136)?
            };
            nat_add(a137, a138)?
        };
        Ok(Adt3::C2(Adt2::C0(v6, if f17(v0.clone()) {
            3u64
        } else {
            let a139 = 3u64;
            let a140 = v4;
            nat_add(a139, a140)?
        }, 41u64, 0u64, 0u64, v0, v1, v3)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f4(v0: Adt1, v1: Adt1, v2: Adt1) -> R<Adt3> {
    let v3: u64 = f31(match v2.clone() {
        Adt1::C0(h141, _, _, _, _, _, _, _, _, _, _) => {
            h141
        }
    });
    let a143 = match v2.clone() {
        Adt1::C0(h142, _, _, _, _, _, _, _, _, _, _) => {
            h142
        }
    };
    let a144 = 8u64;
    if if if if if if if nat_eq(a143, a144) {
        f26(v1.clone(), v2.clone())
    } else {
        false
    } {
        let a146 = 0u64;
        let a147 = f18(match v0.clone() {
            Adt1::C0(h145, _, _, _, _, _, _, _, _, _, _) => {
                h145
            }
        });
        nat_lt(a146, a147)
    } else {
        false
    } {
        let a150 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h148, _, _, _, _, _) => {
                h148
            }
        };
        let a151 = f18(match v0.clone() {
            Adt1::C0(h149, _, _, _, _, _, _, _, _, _, _) => {
                h149
            }
        });
        nat_eq(a150, a151)
    } else {
        false
    } {
        let a156 = {
            let a153 = match v0.clone() {
                Adt1::C0(_, h152, _, _, _, _, _, _, _, _, _) => {
                    h152
                }
            };
            let a154 = v3;
            let a155 = 1u64;
            nat_rem(a153, a154, a155)
        };
        let a157 = 0u64;
        if nat_eq(a156, a157) {
            let a162 = {
                let a159 = match v2.clone() {
                    Adt1::C0(_, h158, _, _, _, _, _, _, _, _, _) => {
                        h158
                    }
                };
                let a160 = v3;
                let a161 = 1u64;
                nat_rem(a159, a160, a161)
            };
            let a163 = 0u64;
            nat_eq(a162, a163)
        } else {
            false
        }
    } else {
        false
    } {
        let a166 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h164, _, _, _, _, _) => {
                h164
            }
        };
        let a167 = f27(match v2.clone() {
            Adt1::C0(h165, _, _, _, _, _, _, _, _, _, _) => {
                h165
            }
        });
        nat_eq(a166, a167)
    } else {
        false
    } {
        let a168 = f24(v0.clone())?;
        let a169 = f24(v2.clone())?;
        nat_eq(a168, a169)
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a170 = f24(v0.clone())?;
            let a171 = v3;
            let a172 = 0u64;
            nat_quot(a170, a171, a172)
        }, 3u64, 33u64, 0u64, 0u64, v0, v1, v2)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f5(v0: u64, v1: u64, v2: Adt1, v3: Adt1) -> R<Adt3> {
    let a174 = match v2.clone() {
        Adt1::C0(h173, _, _, _, _, _, _, _, _, _, _) => {
            h173
        }
    };
    let a175 = 0u64;
    if if if if if if if if if if if nat_eq(a174, a175) {
        let a177 = match v3.clone() {
            Adt1::C0(h176, _, _, _, _, _, _, _, _, _, _) => {
                h176
            }
        };
        let a178 = 26u64;
        nat_eq(a177, a178)
    } else {
        false
    } {
        let a180 = match v2.clone() {
            Adt1::C0(_, h179, _, _, _, _, _, _, _, _, _) => {
                h179
            }
        };
        let a181 = 2147483647u64;
        nat_le(a180, a181)
    } else {
        false
    } {
        let a184 = match v3.clone() {
            Adt1::C0(_, h182, _, _, _, _, _, _, _, _, _) => {
                h182
            }
        };
        let a185 = match v2.clone() {
            Adt1::C0(_, h183, _, _, _, _, _, _, _, _, _) => {
                h183
            }
        };
        nat_le(a184, a185)
    } else {
        false
    } {
        let a188 = match v2.clone() {
            Adt1::C0(_, _, h186, _, _, _, _, _, _, _, _) => {
                h186
            }
        };
        let a189 = match v3.clone() {
            Adt1::C0(_, _, h187, _, _, _, _, _, _, _, _) => {
                h187
            }
        };
        nat_eq(a188, a189)
    } else {
        false
    } {
        let a192 = match v2.clone() {
            Adt1::C0(_, _, _, h190, _, _, _, _, _, _, _) => {
                h190
            }
        };
        let a193 = match v3.clone() {
            Adt1::C0(_, _, _, h191, _, _, _, _, _, _, _) => {
                h191
            }
        };
        nat_eq(a192, a193)
    } else {
        false
    } {
        let a196 = match v2.clone() {
            Adt1::C0(_, _, _, _, h194, _, _, _, _, _, _) => {
                h194
            }
        };
        let a197 = match v3.clone() {
            Adt1::C0(_, _, _, _, h195, _, _, _, _, _, _) => {
                h195
            }
        };
        nat_eq(a196, a197)
    } else {
        false
    } {
        let a198 = v0;
        let a199 = 67u64;
        if nat_eq(a198, a199) {
            true
        } else {
            let a200 = v1;
            let a201 = 1u64;
            if nat_le(a200, a201) {
                let a204 = match v3.clone() {
                    Adt1::C0(_, h202, _, _, _, _, _, _, _, _, _) => {
                        h202
                    }
                };
                let a205 = match v2.clone() {
                    Adt1::C0(_, h203, _, _, _, _, _, _, _, _, _) => {
                        h203
                    }
                };
                nat_eq(a204, a205)
            } else {
                false
            }
        }
    } else {
        false
    } {
        f32(v2.clone())?
    } else {
        false
    } {
        f32(v3.clone())?
    } else {
        false
    } {
        f33(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a211 = match v3.clone() {
                Adt1::C0(_, _, h206, _, _, _, _, _, _, _, _) => {
                    h206
                }
            };
            let a212 = {
                let a209 = match v3.clone() {
                    Adt1::C0(_, _, _, h207, _, _, _, _, _, _, _) => {
                        h207
                    }
                };
                let a210 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h208, _, _, _, _, _, _) => {
                        h208
                    }
                };
                nat_mul(a209, a210)?
            };
            nat_mul(a211, a212)?
        }, 3u64, v0, {
            let a213 = v0;
            let a214 = 67u64;
            if nat_eq(a213, a214) {
                1u64
            } else {
                v1
            }
        }, 0u64, v2.clone(), v2, v3)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f6(v0: u64) -> bool {
    let a215 = v0;
    let a216 = 2u64;
    if if nat_le(a215, a216) {
        true
    } else {
        let a217 = v0;
        let a218 = 5u64;
        nat_eq(a217, a218)
    } {
        true
    } else {
        let a219 = v0;
        let a220 = 6u64;
        nat_eq(a219, a220)
    }
}

pub fn f7(v0: u64, v1: bool, v2: u64, v3: u64, v4: Adt1, v5: Adt1, v6: Adt1) -> R<Adt3> {
    let a221 = v2;
    let a222 = 1u64;
    if if if if if if if if if if if if if if if nat_le(a221, a222) {
        let a224 = 0u64;
        let a225 = f18(match v4.clone() {
            Adt1::C0(h223, _, _, _, _, _, _, _, _, _, _) => {
                h223
            }
        });
        nat_lt(a224, a225)
    } else {
        false
    } {
        let a228 = match v4.clone() {
            Adt1::C0(h226, _, _, _, _, _, _, _, _, _, _) => {
                h226
            }
        };
        let a229 = match v5.clone() {
            Adt1::C0(h227, _, _, _, _, _, _, _, _, _, _) => {
                h227
            }
        };
        nat_eq(a228, a229)
    } else {
        false
    } {
        let a232 = match v4.clone() {
            Adt1::C0(h230, _, _, _, _, _, _, _, _, _, _) => {
                h230
            }
        };
        let a233 = match v6.clone() {
            Adt1::C0(h231, _, _, _, _, _, _, _, _, _, _) => {
                h231
            }
        };
        nat_eq(a232, a233)
    } else {
        false
    } {
        f14(v4.clone(), v5.clone())
    } else {
        false
    } {
        f34(v4.clone())?
    } else {
        false
    } {
        f34(v5.clone())?
    } else {
        false
    } {
        f34(v6.clone())?
    } else {
        false
    } {
        let a236 = match v4.clone() {
            Adt1::C0(_, _, h234, _, _, _, _, _, _, _, _) => {
                h234
            }
        };
        let a237 = match v6.clone() {
            Adt1::C0(_, _, h235, _, _, _, _, _, _, _, _) => {
                h235
            }
        };
        nat_eq(a236, a237)
    } else {
        false
    } {
        let a240 = match v4.clone() {
            Adt1::C0(_, _, _, h238, _, _, _, _, _, _, _) => {
                h238
            }
        };
        let a241 = match v6.clone() {
            Adt1::C0(_, _, _, h239, _, _, _, _, _, _, _) => {
                h239
            }
        };
        nat_eq(a240, a241)
    } else {
        false
    } {
        let a244 = match v4.clone() {
            Adt1::C0(_, _, _, _, h242, _, _, _, _, _, _) => {
                h242
            }
        };
        let a245 = match v6.clone() {
            Adt1::C0(_, _, _, _, h243, _, _, _, _, _, _) => {
                h243
            }
        };
        nat_eq(a244, a245)
    } else {
        false
    } {
        let a252 = match v6.clone() {
            Adt1::C0(_, h246, _, _, _, _, _, _, _, _, _) => {
                h246
            }
        };
        let a253 = if v1 {
            match v4.clone() {
                Adt1::C0(_, h247, _, _, _, _, _, _, _, _, _) => {
                    h247
                }
            }
        } else {
            let a249 = match v4.clone() {
                Adt1::C0(_, h248, _, _, _, _, _, _, _, _, _) => {
                    h248
                }
            };
            let a250 = 2u64;
            let a251 = 0u64;
            nat_quot(a249, a250, a251)
        };
        nat_eq(a252, a253)
    } else {
        false
    } {
        let a255 = match v6.clone() {
            Adt1::C0(_, h254, _, _, _, _, _, _, _, _, _) => {
                h254
            }
        };
        let a256 = 2147483647u64;
        nat_le(a255, a256)
    } else {
        false
    } {
        f21(v4.clone(), v6.clone())?
    } else {
        false
    } {
        f21(v5.clone(), v6.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a257 = v0;
            let a258 = 6u64;
            if nat_eq(a257, a258) {
                let a261 = 1u64;
                let a262 = {
                    let a259 = 10u64;
                    let a260 = f24(v6.clone())?;
                    nat_mul(a259, a260)?
                };
                nat_add(a261, a262)?
            } else {
                let a263 = 4u64;
                let a264 = f24(v6.clone())?;
                nat_mul(a263, a264)?
            }
        }, {
            let a265 = v0;
            let a266 = 6u64;
            if nat_eq(a265, a266) {
                4u64
            } else {
                3u64
            }
        }, if v1 {
            98u64
        } else {
            let a267 = 99u64;
            let a268 = v2;
            nat_add(a267, a268)?
        }, v0, v3, v4, v5, v6)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f8(v0: u64, v1: Adt2) -> R<Adt3> {
    let a270 = v0;
    let a271 = match v1.clone() {
        Adt2::C0(h269, _, _, _, _, _, _, _) => {
            h269
        }
    };
    if nat_lt(a270, a271) {
        let v2: Adt1 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h272) => {
                h272
            }
        };
        let v3: u64 = f31(match v2.clone() {
            Adt1::C0(h273, _, _, _, _, _, _, _, _, _, _) => {
                h273
            }
        });
        let v4: u64 = {
            let a275 = match v2.clone() {
                Adt1::C0(_, h274, _, _, _, _, _, _, _, _, _) => {
                    h274
                }
            };
            let a276 = v3;
            let a277 = 0u64;
            nat_quot(a275, a276, a277)
        };
        let v5: u64 = {
            let a278 = v0;
            let a279 = v4;
            let a280 = 0u64;
            nat_quot(a278, a279, a280)
        };
        let v6: u64 = {
            let a282 = v5;
            let a283 = match v2.clone() {
                Adt1::C0(_, _, h281, _, _, _, _, _, _, _, _) => {
                    h281
                }
            };
            let a284 = 0u64;
            nat_quot(a282, a283, a284)
        };
        Ok(Adt3::C6(f35({
            let a285 = v0;
            let a286 = v3;
            nat_mul(a285, a286)?
        }, match v1 {
            Adt2::C0(_, _, _, _, _, h287, _, _) => {
                h287
            }
        })?, f36({
            let a288 = v0;
            let a289 = v4;
            let a290 = 0u64;
            nat_rem(a288, a289, a290)
        }, {
            let a292 = v5;
            let a293 = match v2.clone() {
                Adt1::C0(_, _, h291, _, _, _, _, _, _, _, _) => {
                    h291
                }
            };
            let a294 = 0u64;
            nat_rem(a292, a293, a294)
        }, {
            let a296 = v6;
            let a297 = match v2.clone() {
                Adt1::C0(_, _, _, h295, _, _, _, _, _, _, _) => {
                    h295
                }
            };
            let a298 = 0u64;
            nat_rem(a296, a297, a298)
        }, {
            let a300 = v6;
            let a301 = match v2.clone() {
                Adt1::C0(_, _, _, h299, _, _, _, _, _, _, _) => {
                    h299
                }
            };
            let a302 = 0u64;
            nat_quot(a300, a301, a302)
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, h303, _, _, _, _, _) => {
                h303
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h304, _, _, _, _) => {
                h304
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h305, _, _, _) => {
                h305
            }
        }, match v2 {
            Adt1::C0(_, _, _, _, _, _, _, _, h306, _, _) => {
                h306
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f9(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, h307, _, _) => {
            h307
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h308) => {
            h308
        }
    };
    let a310 = v0;
    let a311 = match v1.clone() {
        Adt2::C0(h309, _, _, _, _, _, _, _) => {
            h309
        }
    };
    if nat_lt(a310, a311) {
        Ok(Adt3::C5({
            let a313 = v0;
            let a314 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, _, h312, _, _, _, _) => {
                    h312
                }
            };
            nat_mul(a313, a314)?
        }, {
            let a316 = v0;
            let a317 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h315, _, _, _, _) => {
                    h315
                }
            };
            nat_mul(a316, a317)?
        }, match v2 {
            Adt1::C0(_, h318, _, _, _, _, _, _, _, _, _) => {
                h318
            }
        }, match v3 {
            Adt1::C0(_, h319, _, _, _, _, _, _, _, _, _) => {
                h319
            }
        }, {
            let a321 = match v1 {
                Adt2::C0(_, _, _, h320, _, _, _, _) => {
                    h320
                }
            };
            let a322 = 1u64;
            nat_eq(a321, a322)
        }))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f10(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let a324 = v0;
    let a325 = match v1.clone() {
        Adt2::C0(h323, _, _, _, _, _, _, _) => {
            h323
        }
    };
    if nat_lt(a324, a325) {
        let v3: u64 = {
            let a327 = 2u64;
            let a328 = f24(match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, h326, _) => {
                    h326
                }
            })?;
            nat_mul(a327, a328)?
        };
        let a329 = v0;
        let a330 = v3;
        if nat_lt(a329, a330) {
            f37(v0, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, h331, _) => {
                    h331
                }
            }, {
                let m334 = match v1 {
                    Adt2::C0(_, _, _, _, _, _, _, h333) => {
                        h333
                    }
                };
                match m334 {
                    Adt1::C0(_, _, h332, _, _, _, _, _, _, _, _) => {
                        h332
                    }
                }
            }, v2)
        } else {
            f38({
                let a335 = v0;
                let a336 = v3;
                nat_sub(a335, a336)
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, h337, _, _) => {
                    h337
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, h338, _) => {
                    h338
                }
            }, match v1 {
                Adt2::C0(_, _, _, _, _, _, _, h339) => {
                    h339
                }
            }, v2)
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f11(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(_, _, _, _, h340, _, _, _) => {
            h340
        }
    };
    let a342 = v0;
    let a343 = match v1.clone() {
        Adt2::C0(h341, _, _, _, _, _, _, _) => {
            h341
        }
    };
    if nat_lt(a342, a343) {
        let a344 = v0;
        let a345 = 0u64;
        if nat_eq(a344, a345) {
            Ok(Adt3::C3(1u64, 6u64, 0u64, 0u64, 2u64, v3, 2u64, 0u64, 0u64, 3u64, 4u64, 4u64))
        } else {
            let v4: u64 = {
                let a346 = v0;
                let a347 = 1u64;
                nat_sub(a346, a347)
            };
            let v5: u64 = {
                let a348 = v4;
                let a349 = 10u64;
                let a350 = 0u64;
                nat_rem(a348, a349, a350)
            };
            let v6: u64 = {
                let a354 = 4u64;
                let a355 = {
                    let a351 = v4;
                    let a352 = 10u64;
                    let a353 = 0u64;
                    nat_quot(a351, a352, a353)
                };
                nat_mul(a354, a355)?
            };
            let a356 = v5;
            let a357 = 2u64;
            if nat_lt(a356, a357) {
                f12({
                    let a358 = v6;
                    let a359 = v5;
                    nat_add(a358, a359)?
                }, v1)
            } else {
                let a360 = v5;
                let a361 = 2u64;
                if nat_eq(a360, a361) {
                    Ok(f39(0u64, 2u64, v3))
                } else {
                    let a362 = v5;
                    let a363 = 3u64;
                    if nat_eq(a362, a363) {
                        Ok(f40(0u64, 2u64, v3, {
                            let a364 = v2;
                            let a365 = 2u64;
                            nat_eq(a364, a365)
                        }))
                    } else {
                        let a366 = v5;
                        let a367 = 4u64;
                        if nat_eq(a366, a367) {
                            Ok(f39(1u64, 1u64, 3u64))
                        } else {
                            let a368 = v5;
                            let a369 = 5u64;
                            if nat_eq(a368, a369) {
                                Ok(f40(1u64, 1u64, 3u64, {
                                    let a370 = v2;
                                    let a371 = 0u64;
                                    nat_eq(a370, a371)
                                }))
                            } else {
                                let a372 = v5;
                                let a373 = 6u64;
                                if nat_eq(a372, a373) {
                                    Ok(f39(1u64, 2u64, v3))
                                } else {
                                    let a374 = v5;
                                    let a375 = 7u64;
                                    if nat_eq(a374, a375) {
                                        Ok(f40(1u64, 2u64, v3, {
                                            let a376 = v2;
                                            let a377 = 2u64;
                                            nat_eq(a376, a377)
                                        }))
                                    } else {
                                        let a378 = v5;
                                        let a379 = 8u64;
                                        if nat_eq(a378, a379) {
                                            Ok(Adt3::C4(2u64, {
                                                let m382 = match v1 {
                                                    Adt2::C0(_, _, _, _, _, _, _, h381) => {
                                                        h381
                                                    }
                                                };
                                                match m382 {
                                                    Adt1::C0(h380, _, _, _, _, _, _, _, _, _, _) => {
                                                        h380
                                                    }
                                                }
                                            }, 0u64, 1u64, 0u64))
                                        } else {
                                            f12({
                                                let a383 = v6;
                                                let a384 = 3u64;
                                                nat_add(a383, a384)?
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

pub fn f12(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, h385, _, _) => {
            h385
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h386, _) => {
            h386
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h387) => {
            h387
        }
    };
    let v5: u64 = match v1.clone() {
        Adt2::C0(_, _, h388, _, _, _, _, _) => {
            h388
        }
    };
    let v6: u64 = {
        let a389 = v0;
        let a390 = 4u64;
        let a391 = 0u64;
        nat_rem(a389, a390, a391)
    };
    let v7: u64 = {
        let a392 = v0;
        let a393 = 4u64;
        let a394 = 0u64;
        nat_quot(a392, a393, a394)
    };
    let v8: u64 = {
        let a396 = v7;
        let a397 = match v4.clone() {
            Adt1::C0(_, h395, _, _, _, _, _, _, _, _, _) => {
                h395
            }
        };
        let a398 = 0u64;
        nat_quot(a396, a397, a398)
    };
    let v9: u64 = f18(match v4.clone() {
        Adt1::C0(h399, _, _, _, _, _, _, _, _, _, _) => {
            h399
        }
    });
    let v10: u64 = {
        let a404 = {
            let a401 = v7;
            let a402 = match v4.clone() {
                Adt1::C0(_, h400, _, _, _, _, _, _, _, _, _) => {
                    h400
                }
            };
            let a403 = 0u64;
            nat_rem(a401, a402, a403)
        };
        let a405 = v9;
        nat_mul(a404, a405)?
    };
    let v11: u64 = {
        let a407 = match v4.clone() {
            Adt1::C0(_, h406, _, _, _, _, _, _, _, _, _) => {
                h406
            }
        };
        let a408 = v9;
        nat_mul(a407, a408)?
    };
    let a410 = v0;
    let a411 = match v1.clone() {
        Adt2::C0(h409, _, _, _, _, _, _, _) => {
            h409
        }
    };
    if nat_lt(a410, a411) {
        let a412 = v6;
        let a413 = 0u64;
        if nat_eq(a412, a413) {
            Ok(Adt3::C3(0u64, 0u64, match v2.clone() {
                Adt1::C0(h414, _, _, _, _, _, _, _, _, _, _) => {
                    h414
                }
            }, 0u64, 0u64, {
                let a422 = {
                    let a418 = {
                        let a416 = v8;
                        let a417 = match v2 {
                            Adt1::C0(_, _, _, _, _, _, h415, _, _, _, _) => {
                                h415
                            }
                        };
                        nat_mul(a416, a417)?
                    };
                    let a419 = v10;
                    nat_add(a418, a419)?
                };
                let a423 = {
                    let a420 = v5;
                    let a421 = 100u64;
                    if nat_eq(a420, a421) {
                        v11
                    } else {
                        0u64
                    }
                };
                nat_add(a422, a423)?
            }, 2u64, 0u64, 0u64, 0u64, v9, 4u64))
        } else {
            let a424 = v6;
            let a425 = 1u64;
            if nat_eq(a424, a425) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h426, _, _, _, _, _, _, _, _, _, _) => {
                        h426
                    }
                }, 0u64, {
                    let a427 = v5;
                    let a428 = 98u64;
                    if nat_eq(a427, a428) {
                        4u64
                    } else {
                        0u64
                    }
                }, {
                    let a436 = {
                        let a432 = {
                            let a430 = v8;
                            let a431 = match v3 {
                                Adt1::C0(_, _, _, _, _, _, h429, _, _, _, _) => {
                                    h429
                                }
                            };
                            nat_mul(a430, a431)?
                        };
                        let a433 = v10;
                        nat_add(a432, a433)?
                    };
                    let a437 = {
                        let a434 = v5;
                        let a435 = 99u64;
                        if nat_eq(a434, a435) {
                            v11
                        } else {
                            0u64
                        }
                    };
                    nat_add(a436, a437)?
                }, 2u64, 0u64, 0u64, 1u64, v9, 4u64))
            } else {
                let a438 = v6;
                let a439 = 2u64;
                if nat_eq(a438, a439) {
                    Ok(Adt3::C4(match v1 {
                        Adt2::C0(_, _, _, h440, _, _, _, _) => {
                            h440
                        }
                    }, match v4 {
                        Adt1::C0(h441, _, _, _, _, _, _, _, _, _, _) => {
                            h441
                        }
                    }, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt3::C3(0u64, 0u64, 0u64, match v4.clone() {
                        Adt1::C0(h442, _, _, _, _, _, _, _, _, _, _) => {
                            h442
                        }
                    }, 1u64, 0u64, 2u64, 0u64, 3u64, {
                        let a446 = {
                            let a444 = v8;
                            let a445 = match v4 {
                                Adt1::C0(_, _, _, _, _, _, h443, _, _, _, _) => {
                                    h443
                                }
                            };
                            nat_mul(a444, a445)?
                        };
                        let a447 = v10;
                        nat_add(a446, a447)?
                    }, 4u64, v9))
                }
            }
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f13(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, h448, _, _) => {
            h448
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h449, _) => {
            h449
        }
    };
    let v5: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h450) => {
            h450
        }
    };
    let v6: u64 = {
        let a452 = match v1.clone() {
            Adt2::C0(_, h451, _, _, _, _, _, _) => {
                h451
            }
        };
        let a453 = 3u64;
        if nat_eq(a452, a453) {
            0u64
        } else {
            let a454 = f24(v3.clone())?;
            let a455 = 2u64;
            nat_mul(a454, a455)?
        }
    };
    let a457 = match v1.clone() {
        Adt2::C0(h456, _, _, _, _, _, _, _) => {
            h456
        }
    };
    let a458 = v0;
    if nat_le(a457, a458) {
        Ok(Adt3::C1)
    } else {
        let a459 = v0;
        let a460 = v6;
        if nat_lt(a459, a460) {
            let v7: u64 = {
                let a461 = v0;
                let a462 = 2u64;
                let a463 = 0u64;
                nat_quot(a461, a462, a463)
            };
            let v8: u64 = {
                let a468 = 3u64;
                let a469 = {
                    let a465 = f35(v7, v5.clone())?;
                    let a466 = f18(match v5 {
                        Adt1::C0(h464, _, _, _, _, _, _, _, _, _, _) => {
                            h464
                        }
                    });
                    let a467 = 0u64;
                    nat_quot(a465, a466, a467)
                };
                nat_add(a468, a469)?
            };
            let a473 = {
                let a470 = v0;
                let a471 = 2u64;
                let a472 = 0u64;
                nat_rem(a470, a471, a472)
            };
            let a474 = 0u64;
            if nat_eq(a473, a474) {
                Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, v8, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
            } else {
                let a475 = v2;
                let a476 = 0u64;
                if nat_eq(a475, a476) {
                    Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 2u64, 1u64, 2u64, 0u64, 0u64, v8, 8u64, 8u64))
                } else {
                    Ok(Adt3::C1)
                }
            }
        } else {
            let v9: u64 = {
                let a477 = v0;
                let a478 = v6;
                nat_sub(a477, a478)
            };
            let v10: u64 = f25(match v3.clone() {
                Adt1::C0(h479, _, _, _, _, _, _, _, _, _, _) => {
                    h479
                }
            }, match v4.clone() {
                Adt1::C0(h480, _, _, _, _, _, _, _, _, _, _) => {
                    h480
                }
            }, match v5.clone() {
                Adt1::C0(h481, _, _, _, _, _, _, _, _, _, _) => {
                    h481
                }
            })?;
            let v11: u64 = {
                let a482 = v9;
                let a483 = v10;
                let a484 = 0u64;
                nat_quot(a482, a483, a484)
            };
            let v12: u64 = {
                let a485 = v9;
                let a486 = v10;
                let a487 = 0u64;
                nat_rem(a485, a486, a487)
            };
            let v13: u64 = {
                let a489 = match v3.clone() {
                    Adt1::C0(h488, _, _, _, _, _, _, _, _, _, _) => {
                        h488
                    }
                };
                let a490 = 0u64;
                if nat_eq(a489, a490) {
                    0u64
                } else {
                    1u64
                }
            };
            let v14: u64 = {
                let a492 = match v4.clone() {
                    Adt1::C0(h491, _, _, _, _, _, _, _, _, _, _) => {
                        h491
                    }
                };
                let a493 = 0u64;
                if nat_eq(a492, a493) {
                    0u64
                } else {
                    1u64
                }
            };
            let a494 = v12;
            let a495 = v13;
            if nat_lt(a494, a495) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h496, _, _, _, _, _, _, _, _, _, _) => {
                        h496
                    }
                }, 0u64, 0u64, f35(v11, v3.clone())?, 2u64, 0u64, 0u64, 0u64, f18(match v3 {
                    Adt1::C0(h497, _, _, _, _, _, _, _, _, _, _) => {
                        h497
                    }
                }), 4u64))
            } else {
                let a500 = v12;
                let a501 = {
                    let a498 = v13;
                    let a499 = v14;
                    nat_add(a498, a499)?
                };
                if nat_lt(a500, a501) {
                    Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                        Adt1::C0(h502, _, _, _, _, _, _, _, _, _, _) => {
                            h502
                        }
                    }, 0u64, 4u64, f41(v11, v3, v4.clone())?, 2u64, 0u64, 0u64, 1u64, f18(match v4 {
                        Adt1::C0(h503, _, _, _, _, _, _, _, _, _, _) => {
                            h503
                        }
                    }), 4u64))
                } else {
                    let a506 = v12;
                    let a507 = {
                        let a504 = v13;
                        let a505 = v14;
                        nat_add(a504, a505)?
                    };
                    if nat_eq(a506, a507) {
                        Ok(Adt3::C3(1u64, match v1 {
                            Adt2::C0(_, _, h508, _, _, _, _, _) => {
                                h508
                            }
                        }, 0u64, 0u64, {
                            let a509 = v13;
                            let a510 = 0u64;
                            if nat_eq(a509, a510) {
                                0u64
                            } else {
                                1u64
                            }
                        }, {
                            let a511 = v13;
                            let a512 = 0u64;
                            if nat_eq(a511, a512) {
                                f35(v11, v3.clone())?
                            } else {
                                0u64
                            }
                        }, {
                            let a513 = v14;
                            let a514 = 0u64;
                            if nat_eq(a513, a514) {
                                4u64
                            } else {
                                1u64
                            }
                        }, {
                            let a515 = v14;
                            let a516 = 0u64;
                            if nat_eq(a515, a516) {
                                f41(v11, v3, v4)?
                            } else {
                                1u64
                            }
                        }, {
                            let a518 = match v5.clone() {
                                Adt1::C0(h517, _, _, _, _, _, _, _, _, _, _) => {
                                    h517
                                }
                            };
                            let a519 = 0u64;
                            if nat_eq(a518, a519) {
                                3u64
                            } else {
                                0u64
                            }
                        }, {
                            let a521 = match v5.clone() {
                                Adt1::C0(h520, _, _, _, _, _, _, _, _, _, _) => {
                                    h520
                                }
                            };
                            let a522 = 0u64;
                            if nat_eq(a521, a522) {
                                f35(v11, v5)?
                            } else {
                                0u64
                            }
                        }, 4u64, 4u64))
                    } else {
                        Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                            Adt1::C0(h523, _, _, _, _, _, _, _, _, _, _) => {
                                h523
                            }
                        }, 1u64, 0u64, 2u64, 0u64, 3u64, f35(v11, v5.clone())?, 4u64, f18(match v5 {
                            Adt1::C0(h524, _, _, _, _, _, _, _, _, _, _) => {
                                h524
                            }
                        })))
                    }
                }
            }
        }
    }
}

pub fn f14(v0: Adt1, v1: Adt1) -> bool {
    let a527 = match v0.clone() {
        Adt1::C0(_, h525, _, _, _, _, _, _, _, _, _) => {
            h525
        }
    };
    let a528 = match v1.clone() {
        Adt1::C0(_, h526, _, _, _, _, _, _, _, _, _) => {
            h526
        }
    };
    if if if nat_eq(a527, a528) {
        let a531 = match v0.clone() {
            Adt1::C0(_, _, h529, _, _, _, _, _, _, _, _) => {
                h529
            }
        };
        let a532 = match v1.clone() {
            Adt1::C0(_, _, h530, _, _, _, _, _, _, _, _) => {
                h530
            }
        };
        nat_eq(a531, a532)
    } else {
        false
    } {
        let a535 = match v0.clone() {
            Adt1::C0(_, _, _, h533, _, _, _, _, _, _, _) => {
                h533
            }
        };
        let a536 = match v1.clone() {
            Adt1::C0(_, _, _, h534, _, _, _, _, _, _, _) => {
                h534
            }
        };
        nat_eq(a535, a536)
    } else {
        false
    } {
        let a539 = match v0 {
            Adt1::C0(_, _, _, _, h537, _, _, _, _, _, _) => {
                h537
            }
        };
        let a540 = match v1 {
            Adt1::C0(_, _, _, _, h538, _, _, _, _, _, _) => {
                h538
            }
        };
        nat_eq(a539, a540)
    } else {
        false
    }
}

pub fn f15(v0: Adt1, v1: Adt1) -> bool {
    if f17(v1.clone()) {
        f17(v0)
    } else {
        let a546 = {
            let a543 = match v0.clone() {
                Adt1::C0(_, h541, _, _, _, _, _, _, _, _, _) => {
                    h541
                }
            };
            let a544 = match v1.clone() {
                Adt1::C0(_, h542, _, _, _, _, _, _, _, _, _) => {
                    h542
                }
            };
            let a545 = 1u64;
            nat_rem(a543, a544, a545)
        };
        let a547 = 0u64;
        if if if nat_eq(a546, a547) {
            let a553 = {
                let a550 = match v0.clone() {
                    Adt1::C0(_, _, h548, _, _, _, _, _, _, _, _) => {
                        h548
                    }
                };
                let a551 = match v1.clone() {
                    Adt1::C0(_, _, h549, _, _, _, _, _, _, _, _) => {
                        h549
                    }
                };
                let a552 = 1u64;
                nat_rem(a550, a551, a552)
            };
            let a554 = 0u64;
            nat_eq(a553, a554)
        } else {
            false
        } {
            let a560 = {
                let a557 = match v0.clone() {
                    Adt1::C0(_, _, _, h555, _, _, _, _, _, _, _) => {
                        h555
                    }
                };
                let a558 = match v1.clone() {
                    Adt1::C0(_, _, _, h556, _, _, _, _, _, _, _) => {
                        h556
                    }
                };
                let a559 = 1u64;
                nat_rem(a557, a558, a559)
            };
            let a561 = 0u64;
            nat_eq(a560, a561)
        } else {
            false
        } {
            let a567 = {
                let a564 = match v0 {
                    Adt1::C0(_, _, _, _, h562, _, _, _, _, _, _) => {
                        h562
                    }
                };
                let a565 = match v1 {
                    Adt1::C0(_, _, _, _, h563, _, _, _, _, _, _) => {
                        h563
                    }
                };
                let a566 = 1u64;
                nat_rem(a564, a565, a566)
            };
            let a568 = 0u64;
            nat_eq(a567, a568)
        } else {
            false
        }
    }
}

pub fn f16(v0: u64, v1: u64, v2: u64) -> bool {
    let a569 = v0;
    let a570 = 0u64;
    if if if nat_eq(a569, a570) {
        let a571 = v1;
        let a572 = 0u64;
        nat_eq(a571, a572)
    } else {
        false
    } {
        let a573 = v2;
        let a574 = 0u64;
        nat_eq(a573, a574)
    } else {
        false
    } {
        true
    } else {
        let a575 = v0;
        let a576 = 1u64;
        if if nat_eq(a575, a576) {
            true
        } else {
            let a577 = v0;
            let a578 = 30u64;
            nat_eq(a577, a578)
        } {
            let a579 = v1;
            let a580 = v0;
            if if nat_eq(a579, a580) {
                let a581 = v2;
                let a582 = v0;
                nat_eq(a581, a582)
            } else {
                false
            } {
                true
            } else {
                let a583 = v1;
                let a584 = 0u64;
                if nat_eq(a583, a584) {
                    let a585 = v2;
                    let a586 = v0;
                    if nat_eq(a585, a586) {
                        true
                    } else {
                        let a587 = v2;
                        let a588 = 0u64;
                        nat_eq(a587, a588)
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

pub fn f17(v0: Adt1) -> bool {
    f42(match v0.clone() {
        Adt1::C0(_, h589, _, _, _, _, _, _, _, _, _) => {
            h589
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h590, _, _, _, _, _, _, _, _) => {
            h590
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h591, _, _, _, _, _, _, _) => {
            h591
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, h592, _, _, _, _, _, _) => {
            h592
        }
    })
}

pub fn f18(v0: u64) -> u64 {
    let a593 = v0;
    let a594 = 0u64;
    if nat_eq(a593, a594) {
        4u64
    } else {
        let a595 = v0;
        let a596 = 1u64;
        if if nat_eq(a595, a596) {
            true
        } else {
            let a597 = v0;
            let a598 = 30u64;
            nat_eq(a597, a598)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f19(v0: Adt1) -> bool {
    let v1: u64 = f18(match v0.clone() {
        Adt1::C0(h599, _, _, _, _, _, _, _, _, _, _) => {
            h599
        }
    });
    let a604 = {
        let a601 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h600) => {
                h600
            }
        };
        let a602 = v1;
        let a603 = 1u64;
        nat_rem(a601, a602, a603)
    };
    let a605 = 0u64;
    if if nat_eq(a604, a605) {
        f43(match v0.clone() {
            Adt1::C0(_, h606, _, _, _, _, _, _, _, _, _) => {
                h606
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h607, _, _, _, _, _) => {
                h607
            }
        }, v1)
    } else {
        false
    } {
        f44(match v0.clone() {
            Adt1::C0(_, _, h608, _, _, _, _, _, _, _, _) => {
                h608
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h609, _, _, _, _, _, _, _) => {
                h609
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h610, _, _, _, _, _, _) => {
                h610
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h611, _, _, _, _) => {
                h611
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h612, _, _, _) => {
                h612
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h613, _, _) => {
                h613
            }
        }, v1)
    } else {
        false
    }
}

pub fn f20(v0: Adt1, v1: Adt1) -> R<bool> {
    let a621 = {
        let a619 = match v0.clone() {
            Adt1::C0(_, _, h614, _, _, _, _, _, _, _, _) => {
                h614
            }
        };
        let a620 = {
            let a617 = match v0.clone() {
                Adt1::C0(_, _, _, h615, _, _, _, _, _, _, _) => {
                    h615
                }
            };
            let a618 = match v0.clone() {
                Adt1::C0(_, _, _, _, h616, _, _, _, _, _, _) => {
                    h616
                }
            };
            nat_mul(a617, a618)?
        };
        nat_mul(a619, a620)?
    };
    let a622 = 9223372036854775807u64;
    if nat_le(a621, a622) {
        if f29(v1.clone()) {
            let a624 = match v1 {
                Adt1::C0(_, h623, _, _, _, _, _, _, _, _, _) => {
                    h623
                }
            };
            let a625 = 2147483647u64;
            Ok(nat_le(a624, a625))
        } else {
            let a627 = match v0 {
                Adt1::C0(_, h626, _, _, _, _, _, _, _, _, _) => {
                    h626
                }
            };
            let a628 = 2147483647u64;
            Ok(nat_le(a627, a628))
        }
    } else {
        Ok(false)
    }
}

pub fn f21(v0: Adt1, v1: Adt1) -> R<bool> {
    if f33(v0.clone(), v1.clone())? {
        Ok(true)
    } else {
        Ok(f45(v0, v1))
    }
}

pub fn f22(v0: Adt1) -> R<bool> {
    f46({
        let a631 = match v0.clone() {
            Adt1::C0(_, h629, _, _, _, _, _, _, _, _, _) => {
                h629
            }
        };
        let a632 = f18(match v0.clone() {
            Adt1::C0(h630, _, _, _, _, _, _, _, _, _, _) => {
                h630
            }
        });
        nat_mul(a631, a632)?
    }, match v0.clone() {
        Adt1::C0(_, _, h633, _, _, _, _, _, _, _, _) => {
            h633
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h634, _, _, _, _, _, _, _) => {
            h634
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h635, _, _, _, _, _, _) => {
            h635
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h636, _, _, _, _) => {
            h636
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h637, _, _, _) => {
            h637
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h638, _, _) => {
            h638
        }
    })
}

pub fn f23(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a645 = {
        let a639 = v0;
        let a640 = v2;
        nat_mul(a639, a640)?
    };
    let a646 = {
        let a641 = v1;
        let a642 = 0u64;
        if nat_eq(a641, a642) {
            0u64
        } else {
            let a643 = v0;
            let a644 = 2u64;
            nat_mul(a643, a644)?
        }
    };
    nat_add(a645, a646)
}

pub fn f24(v0: Adt1) -> R<u64> {
    let a655 = {
        let a649 = match v0.clone() {
            Adt1::C0(_, h647, _, _, _, _, _, _, _, _, _) => {
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
    let a656 = {
        let a653 = match v0.clone() {
            Adt1::C0(_, _, _, h651, _, _, _, _, _, _, _) => {
                h651
            }
        };
        let a654 = match v0 {
            Adt1::C0(_, _, _, _, h652, _, _, _, _, _, _) => {
                h652
            }
        };
        nat_mul(a653, a654)?
    };
    nat_mul(a655, a656)
}

pub fn f25(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a667 = {
        let a663 = {
            let a659 = 1u64;
            let a660 = {
                let a657 = v0;
                let a658 = 0u64;
                if nat_eq(a657, a658) {
                    0u64
                } else {
                    1u64
                }
            };
            nat_add(a659, a660)?
        };
        let a664 = {
            let a661 = v1;
            let a662 = 0u64;
            if nat_eq(a661, a662) {
                0u64
            } else {
                1u64
            }
        };
        nat_add(a663, a664)?
    };
    let a668 = {
        let a665 = v2;
        let a666 = 0u64;
        if nat_eq(a665, a666) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a667, a668)
}

pub fn f26(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f14(v0.clone(), v1.clone()) {
        let a671 = match v0.clone() {
            Adt1::C0(h669, _, _, _, _, _, _, _, _, _, _) => {
                h669
            }
        };
        let a672 = match v1.clone() {
            Adt1::C0(h670, _, _, _, _, _, _, _, _, _, _) => {
                h670
            }
        };
        nat_eq(a671, a672)
    } else {
        false
    } {
        let a675 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h673, _, _, _, _, _) => {
                h673
            }
        };
        let a676 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h674, _, _, _, _, _) => {
                h674
            }
        };
        nat_eq(a675, a676)
    } else {
        false
    } {
        let a679 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h677, _, _, _, _) => {
                h677
            }
        };
        let a680 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h678, _, _, _, _) => {
                h678
            }
        };
        nat_eq(a679, a680)
    } else {
        false
    } {
        let a683 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h681, _, _, _) => {
                h681
            }
        };
        let a684 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h682, _, _, _) => {
                h682
            }
        };
        nat_eq(a683, a684)
    } else {
        false
    } {
        let a687 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h685, _, _) => {
                h685
            }
        };
        let a688 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h686, _, _) => {
                h686
            }
        };
        nat_eq(a687, a688)
    } else {
        false
    } {
        let a690 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h689) => {
                h689
            }
        };
        let a691 = 0u64;
        if if nat_eq(a690, a691) {
            true
        } else {
            let a693 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h692) => {
                    h692
                }
            };
            let a694 = 0u64;
            nat_eq(a693, a694)
        } {
            true
        } else {
            let a697 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h695) => {
                    h695
                }
            };
            let a698 = match v1 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h696) => {
                    h696
                }
            };
            nat_eq(a697, a698)
        }
    } else {
        false
    }
}

pub fn f27(v0: u64) -> u64 {
    let a699 = v0;
    let a700 = 8u64;
    if nat_eq(a699, a700) {
        let m701 = f47(v0);
        match m701 {
            Adt4::C0 => {
                0u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, _, h702) => {
                        h702
                    }
                }
            }
        }
    } else {
        let a703 = v0;
        let a704 = 43u64;
        if nat_eq(a703, a704) {
            1u64
        } else {
            f18(v0)
        }
    }
}

pub fn f28(v0: u64) -> u64 {
    let a705 = v0;
    let a706 = 26u64;
    if nat_eq(a705, a706) {
        4u64
    } else {
        let a707 = v0;
        let a708 = 27u64;
        if nat_eq(a707, a708) {
            8u64
        } else {
            0u64
        }
    }
}

pub fn f29(v0: Adt1) -> bool {
    let a710 = match v0.clone() {
        Adt1::C0(_, h709, _, _, _, _, _, _, _, _, _) => {
            h709
        }
    };
    let a711 = 1u64;
    if nat_eq(a710, a711) {
        true
    } else {
        let a714 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h712, _, _, _, _, _) => {
                h712
            }
        };
        let a715 = f18(match v0 {
            Adt1::C0(h713, _, _, _, _, _, _, _, _, _, _) => {
                h713
            }
        });
        nat_eq(a714, a715)
    }
}

pub fn f30(v0: Adt1, v1: u64) -> R<u64> {
    let a721 = {
        let a717 = match v0.clone() {
            Adt1::C0(_, h716, _, _, _, _, _, _, _, _, _) => {
                h716
            }
        };
        let a718 = f31(v1);
        let a719 = 0u64;
        nat_quot(a717, a718, a719)
    };
    let a722 = f48(match v0 {
        Adt1::C0(h720, _, _, _, _, _, _, _, _, _, _) => {
            h720
        }
    }, v1);
    nat_mul(a721, a722)
}

pub fn f31(v0: u64) -> u64 {
    let a723 = v0;
    let a724 = 8u64;
    if nat_eq(a723, a724) {
        let m725 = f47(v0);
        match m725 {
            Adt4::C0 => {
                1u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, h726, _) => {
                        h726
                    }
                }
            }
        }
    } else {
        1u64
    }
}

pub fn f32(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a728 = match v0.clone() {
            Adt1::C0(_, h727, _, _, _, _, _, _, _, _, _) => {
                h727
            }
        };
        let a729 = 4u64;
        nat_mul(a728, a729)?
    };
    let v2: u64 = {
        let a732 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h730, _, _, _, _) => {
                h730
            }
        };
        let a733 = match v0.clone() {
            Adt1::C0(_, _, h731, _, _, _, _, _, _, _, _) => {
                h731
            }
        };
        nat_mul(a732, a733)?
    };
    let v3: u64 = {
        let a735 = v2;
        let a736 = match v0.clone() {
            Adt1::C0(_, _, _, h734, _, _, _, _, _, _, _) => {
                h734
            }
        };
        nat_mul(a735, a736)?
    };
    let a738 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h737, _, _, _, _, _) => {
            h737
        }
    };
    let a739 = 4u64;
    if if if if nat_eq(a738, a739) {
        let a744 = {
            let a741 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h740) => {
                    h740
                }
            };
            let a742 = 4u64;
            let a743 = 1u64;
            nat_rem(a741, a742, a743)
        };
        let a745 = 0u64;
        nat_eq(a744, a745)
    } else {
        false
    } {
        let a753 = {
            let a751 = match v0.clone() {
                Adt1::C0(_, _, h746, _, _, _, _, _, _, _, _) => {
                    h746
                }
            };
            let a752 = {
                let a749 = match v0.clone() {
                    Adt1::C0(_, _, _, h747, _, _, _, _, _, _, _) => {
                        h747
                    }
                };
                let a750 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h748, _, _, _, _, _, _) => {
                        h748
                    }
                };
                nat_mul(a749, a750)?
            };
            nat_mul(a751, a752)?
        };
        let a754 = 1u64;
        if nat_le(a753, a754) {
            true
        } else {
            let a756 = v1;
            let a757 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h755, _, _, _, _) => {
                    h755
                }
            };
            if nat_le(a756, a757) {
                let a762 = {
                    let a759 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, h758, _, _, _, _) => {
                            h758
                        }
                    };
                    let a760 = 4u64;
                    let a761 = 1u64;
                    nat_rem(a759, a760, a761)
                };
                let a763 = 0u64;
                nat_eq(a762, a763)
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a765 = match v0.clone() {
            Adt1::C0(_, _, _, h764, _, _, _, _, _, _, _) => {
                h764
            }
        };
        let a766 = 1u64;
        if nat_le(a765, a766) {
            true
        } else {
            let a768 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h767, _, _, _) => {
                    h767
                }
            };
            let a769 = v2;
            nat_eq(a768, a769)
        }
    } else {
        false
    } {
        let a771 = match v0.clone() {
            Adt1::C0(_, _, _, _, h770, _, _, _, _, _, _) => {
                h770
            }
        };
        let a772 = 1u64;
        if nat_le(a771, a772) {
            Ok(true)
        } else {
            let a774 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, h773, _, _) => {
                    h773
                }
            };
            let a775 = v3;
            Ok(nat_eq(a774, a775))
        }
    } else {
        Ok(false)
    }
}

pub fn f33(v0: Adt1, v1: Adt1) -> R<bool> {
    let a777 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h776, _) => {
            h776
        }
    };
    let a778 = 0u64;
    if nat_eq(a777, a778) {
        Ok(true)
    } else {
        let a780 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h779, _) => {
                h779
            }
        };
        let a781 = 0u64;
        if nat_eq(a780, a781) {
            Ok(true)
        } else {
            let a783 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h782) => {
                    h782
                }
            };
            let a784 = 0u64;
            if nat_eq(a783, a784) {
                Ok(true)
            } else {
                let a786 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h785) => {
                        h785
                    }
                };
                let a787 = 0u64;
                if nat_eq(a786, a787) {
                    Ok(true)
                } else {
                    let a793 = {
                        let a790 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h788) => {
                                h788
                            }
                        };
                        let a791 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h789, _) => {
                                h789
                            }
                        };
                        nat_add(a790, a791)?
                    };
                    let a794 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h792) => {
                            h792
                        }
                    };
                    if nat_le(a793, a794) {
                        Ok(true)
                    } else {
                        let a800 = {
                            let a797 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h795) => {
                                    h795
                                }
                            };
                            let a798 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h796, _) => {
                                    h796
                                }
                            };
                            nat_add(a797, a798)?
                        };
                        let a801 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h799) => {
                                h799
                            }
                        };
                        Ok(nat_le(a800, a801))
                    }
                }
            }
        }
    }
}

pub fn f34(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a804 = match v0.clone() {
            Adt1::C0(_, _, h802, _, _, _, _, _, _, _, _) => {
                h802
            }
        };
        let a805 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h803, _, _, _, _) => {
                h803
            }
        };
        nat_mul(a804, a805)?
    };
    if if f29(v0.clone()) {
        let a807 = match v0.clone() {
            Adt1::C0(_, _, _, h806, _, _, _, _, _, _, _) => {
                h806
            }
        };
        let a808 = 1u64;
        if nat_eq(a807, a808) {
            true
        } else {
            let a810 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h809, _, _, _) => {
                    h809
                }
            };
            let a811 = v1;
            nat_eq(a810, a811)
        }
    } else {
        false
    } {
        let a813 = match v0.clone() {
            Adt1::C0(_, _, _, _, h812, _, _, _, _, _, _) => {
                h812
            }
        };
        let a814 = 1u64;
        if nat_eq(a813, a814) {
            Ok(true)
        } else {
            let a819 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h815, _, _) => {
                    h815
                }
            };
            let a820 = {
                let a817 = v1;
                let a818 = match v0 {
                    Adt1::C0(_, _, _, h816, _, _, _, _, _, _, _) => {
                        h816
                    }
                };
                nat_mul(a817, a818)?
            };
            Ok(nat_eq(a819, a820))
        }
    } else {
        Ok(false)
    }
}

pub fn f35(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a822 = v0;
        let a823 = match v1.clone() {
            Adt1::C0(_, h821, _, _, _, _, _, _, _, _, _) => {
                h821
            }
        };
        let a824 = 0u64;
        nat_quot(a822, a823, a824)
    };
    let v3: u64 = {
        let a826 = v2;
        let a827 = match v1.clone() {
            Adt1::C0(_, _, h825, _, _, _, _, _, _, _, _) => {
                h825
            }
        };
        let a828 = 0u64;
        nat_quot(a826, a827, a828)
    };
    f36({
        let a830 = v0;
        let a831 = match v1.clone() {
            Adt1::C0(_, h829, _, _, _, _, _, _, _, _, _) => {
                h829
            }
        };
        let a832 = 0u64;
        nat_rem(a830, a831, a832)
    }, {
        let a834 = v2;
        let a835 = match v1.clone() {
            Adt1::C0(_, _, h833, _, _, _, _, _, _, _, _) => {
                h833
            }
        };
        let a836 = 0u64;
        nat_rem(a834, a835, a836)
    }, {
        let a838 = v3;
        let a839 = match v1.clone() {
            Adt1::C0(_, _, _, h837, _, _, _, _, _, _, _) => {
                h837
            }
        };
        let a840 = 0u64;
        nat_rem(a838, a839, a840)
    }, {
        let a842 = v3;
        let a843 = match v1.clone() {
            Adt1::C0(_, _, _, h841, _, _, _, _, _, _, _) => {
                h841
            }
        };
        let a844 = 0u64;
        nat_quot(a842, a843, a844)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h845, _, _, _, _, _) => {
            h845
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h846, _, _, _, _) => {
            h846
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h847, _, _, _) => {
            h847
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h848, _, _) => {
            h848
        }
    })
}

pub fn f36(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a861 = {
        let a857 = {
            let a853 = {
                let a849 = v0;
                let a850 = v4;
                nat_mul(a849, a850)?
            };
            let a854 = {
                let a851 = v1;
                let a852 = v5;
                nat_mul(a851, a852)?
            };
            nat_add(a853, a854)?
        };
        let a858 = {
            let a855 = v2;
            let a856 = v6;
            nat_mul(a855, a856)?
        };
        nat_add(a857, a858)?
    };
    let a862 = {
        let a859 = v3;
        let a860 = v7;
        nat_mul(a859, a860)?
    };
    nat_add(a861, a862)
}

pub fn f37(v0: u64, v1: Adt1, v2: u64, v3: u64) -> R<Adt3> {
    let v4: u64 = {
        let a863 = v0;
        let a864 = 2u64;
        let a865 = 0u64;
        nat_quot(a863, a864, a865)
    };
    let v5: u64 = f28(match v1.clone() {
        Adt1::C0(h866, _, _, _, _, _, _, _, _, _, _) => {
            h866
        }
    });
    let a870 = {
        let a867 = v0;
        let a868 = 2u64;
        let a869 = 0u64;
        nat_rem(a867, a868, a869)
    };
    let a871 = 0u64;
    if nat_eq(a870, a871) {
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 4u64, f35(v4, v1)?, 2u64, 0u64, 0u64, 2u64, v5, 8u64))
    } else {
        let a872 = v3;
        let a873 = v2;
        if if nat_lt(a872, a873) {
            let a876 = v3;
            let a877 = {
                let a874 = v5;
                let a875 = 4u64;
                if nat_eq(a874, a875) {
                    2147483648u64
                } else {
                    9223372036854775808u64
                }
            };
            nat_lt(a876, a877)
        } else {
            false
        } {
            Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, {
                let a878 = 3u64;
                let a879 = v4;
                nat_add(a878, a879)?
            }, 8u64, 8u64))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f38(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64) -> R<Adt3> {
    let v5: u64 = {
        let a881 = 1u64;
        let a882 = f30(v1.clone(), match v3.clone() {
            Adt1::C0(h880, _, _, _, _, _, _, _, _, _, _) => {
                h880
            }
        })?;
        nat_add(a881, a882)?
    };
    let v6: u64 = {
        let a883 = v0;
        let a884 = v5;
        let a885 = 0u64;
        nat_rem(a883, a884, a885)
    };
    let v7: u64 = {
        let a886 = v0;
        let a887 = v5;
        let a888 = 0u64;
        nat_quot(a886, a887, a888)
    };
    let v8: u64 = {
        let a890 = v7;
        let a891 = match v1.clone() {
            Adt1::C0(_, _, h889, _, _, _, _, _, _, _, _) => {
                h889
            }
        };
        let a892 = 0u64;
        nat_rem(a890, a891, a892)
    };
    let v9: u64 = {
        let a894 = v7;
        let a895 = match v1.clone() {
            Adt1::C0(_, _, h893, _, _, _, _, _, _, _, _) => {
                h893
            }
        };
        let a896 = 0u64;
        nat_quot(a894, a895, a896)
    };
    let v10: u64 = {
        let a898 = v9;
        let a899 = match v1.clone() {
            Adt1::C0(_, _, _, h897, _, _, _, _, _, _, _) => {
                h897
            }
        };
        let a900 = 0u64;
        nat_rem(a898, a899, a900)
    };
    let v11: u64 = {
        let a902 = v9;
        let a903 = match v1.clone() {
            Adt1::C0(_, _, _, h901, _, _, _, _, _, _, _) => {
                h901
            }
        };
        let a904 = 0u64;
        nat_quot(a902, a903, a904)
    };
    let a905 = v6;
    let a906 = 0u64;
    if nat_eq(a905, a906) {
        let v12: u64 = {
            let a923 = v8;
            let a924 = {
                let a921 = match v2.clone() {
                    Adt1::C0(_, h907, _, _, _, _, _, _, _, _, _) => {
                        h907
                    }
                };
                let a922 = {
                    let a919 = {
                        let a909 = v10;
                        let a910 = match v2.clone() {
                            Adt1::C0(_, _, h908, _, _, _, _, _, _, _, _) => {
                                h908
                            }
                        };
                        let a911 = 0u64;
                        nat_rem(a909, a910, a911)
                    };
                    let a920 = {
                        let a917 = match v2.clone() {
                            Adt1::C0(_, _, h912, _, _, _, _, _, _, _, _) => {
                                h912
                            }
                        };
                        let a918 = {
                            let a914 = v11;
                            let a915 = match v2 {
                                Adt1::C0(_, _, _, h913, _, _, _, _, _, _, _) => {
                                    h913
                                }
                            };
                            let a916 = 0u64;
                            nat_rem(a914, a915, a916)
                        };
                        nat_mul(a917, a918)?
                    };
                    nat_add(a919, a920)?
                };
                nat_mul(a921, a922)?
            };
            nat_add(a923, a924)?
        };
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, {
            let a925 = 3u64;
            let a926 = v12;
            nat_add(a925, a926)?
        }, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
    } else {
        let a928 = v4;
        let a929 = match v3.clone() {
            Adt1::C0(_, _, h927, _, _, _, _, _, _, _, _) => {
                h927
            }
        };
        if nat_lt(a928, a929) {
            let v13: u64 = {
                let a934 = {
                    let a930 = v6;
                    let a931 = 1u64;
                    nat_sub(a930, a931)
                };
                let a935 = f48(match v1.clone() {
                    Adt1::C0(h932, _, _, _, _, _, _, _, _, _, _) => {
                        h932
                    }
                }, match v3.clone() {
                    Adt1::C0(h933, _, _, _, _, _, _, _, _, _, _) => {
                        h933
                    }
                });
                let a936 = 0u64;
                nat_quot(a934, a935, a936)
            };
            let v14: u64 = f36({
                let a938 = v13;
                let a939 = f31(match v3.clone() {
                    Adt1::C0(h937, _, _, _, _, _, _, _, _, _, _) => {
                        h937
                    }
                });
                nat_mul(a938, a939)?
            }, v8, v10, v11, match v1.clone() {
                Adt1::C0(_, _, _, _, _, h940, _, _, _, _, _) => {
                    h940
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, h941, _, _, _, _) => {
                    h941
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h942, _, _, _) => {
                    h942
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h943, _, _) => {
                    h943
                }
            })?;
            let v15: u64 = f36(v13, v4, v10, v11, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h944, _, _, _, _, _) => {
                    h944
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h945, _, _, _, _) => {
                    h945
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h946, _, _, _) => {
                    h946
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h947, _, _) => {
                    h947
                }
            })?;
            let a949 = match v3.clone() {
                Adt1::C0(h948, _, _, _, _, _, _, _, _, _, _) => {
                    h948
                }
            };
            let a950 = 8u64;
            if nat_eq(a949, a950) {
                Ok(Adt3::C6(v14, v15))
            } else {
                Ok(f49(match v1 {
                    Adt1::C0(h951, _, _, _, _, _, _, _, _, _, _) => {
                        h951
                    }
                }, match v3 {
                    Adt1::C0(h952, _, _, _, _, _, _, _, _, _, _) => {
                        h952
                    }
                }, {
                    let a953 = v6;
                    let a954 = 1u64;
                    nat_sub(a953, a954)
                }, v14, v15))
            }
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f39(v0: u64, v1: u64, v2: u64) -> Adt3 {
    Adt3::C3(2u64, 0u64, 0u64, 0u64, 1u64, v0, v1, v2, 0u64, 2u64, 4u64, 4u64)
}

pub fn f40(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt3 {
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

pub fn f41(v0: u64, v1: Adt1, v2: Adt1) -> R<u64> {
    let v3: u64 = {
        let a956 = v0;
        let a957 = match v1.clone() {
            Adt1::C0(_, h955, _, _, _, _, _, _, _, _, _) => {
                h955
            }
        };
        let a958 = 0u64;
        nat_quot(a956, a957, a958)
    };
    let v4: u64 = {
        let a960 = v3;
        let a961 = match v1.clone() {
            Adt1::C0(_, _, h959, _, _, _, _, _, _, _, _) => {
                h959
            }
        };
        let a962 = 0u64;
        nat_quot(a960, a961, a962)
    };
    f36({
        let a968 = {
            let a964 = v0;
            let a965 = match v1.clone() {
                Adt1::C0(_, h963, _, _, _, _, _, _, _, _, _) => {
                    h963
                }
            };
            let a966 = 0u64;
            nat_rem(a964, a965, a966)
        };
        let a969 = match v2.clone() {
            Adt1::C0(_, h967, _, _, _, _, _, _, _, _, _) => {
                h967
            }
        };
        let a970 = 0u64;
        nat_rem(a968, a969, a970)
    }, {
        let a976 = {
            let a972 = v3;
            let a973 = match v1.clone() {
                Adt1::C0(_, _, h971, _, _, _, _, _, _, _, _) => {
                    h971
                }
            };
            let a974 = 0u64;
            nat_rem(a972, a973, a974)
        };
        let a977 = match v2.clone() {
            Adt1::C0(_, _, h975, _, _, _, _, _, _, _, _) => {
                h975
            }
        };
        let a978 = 0u64;
        nat_rem(a976, a977, a978)
    }, {
        let a984 = {
            let a980 = v4;
            let a981 = match v1.clone() {
                Adt1::C0(_, _, _, h979, _, _, _, _, _, _, _) => {
                    h979
                }
            };
            let a982 = 0u64;
            nat_rem(a980, a981, a982)
        };
        let a985 = match v2.clone() {
            Adt1::C0(_, _, _, h983, _, _, _, _, _, _, _) => {
                h983
            }
        };
        let a986 = 0u64;
        nat_rem(a984, a985, a986)
    }, {
        let a992 = {
            let a988 = v4;
            let a989 = match v1 {
                Adt1::C0(_, _, _, h987, _, _, _, _, _, _, _) => {
                    h987
                }
            };
            let a990 = 0u64;
            nat_quot(a988, a989, a990)
        };
        let a993 = match v2.clone() {
            Adt1::C0(_, _, _, _, h991, _, _, _, _, _, _) => {
                h991
            }
        };
        let a994 = 0u64;
        nat_rem(a992, a993, a994)
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h995, _, _, _, _, _) => {
            h995
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h996, _, _, _, _) => {
            h996
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h997, _, _, _) => {
            h997
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h998, _, _) => {
            h998
        }
    })
}

pub fn f42(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a999 = v0;
    let a1000 = 0u64;
    if if if nat_eq(a999, a1000) {
        true
    } else {
        let a1001 = v1;
        let a1002 = 0u64;
        nat_eq(a1001, a1002)
    } {
        true
    } else {
        let a1003 = v2;
        let a1004 = 0u64;
        nat_eq(a1003, a1004)
    } {
        true
    } else {
        let a1005 = v3;
        let a1006 = 0u64;
        nat_eq(a1005, a1006)
    }
}

pub fn f43(v0: u64, v1: u64, v2: u64) -> bool {
    let a1007 = v0;
    let a1008 = 1u64;
    if nat_le(a1007, a1008) {
        true
    } else {
        let a1012 = {
            let a1009 = v1;
            let a1010 = v2;
            let a1011 = 1u64;
            nat_rem(a1009, a1010, a1011)
        };
        let a1013 = 0u64;
        nat_eq(a1012, a1013)
    }
}

pub fn f44(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f43(v0, v3, v6) {
        f43(v1, v4, v6)
    } else {
        false
    } {
        f43(v2, v5, v6)
    } else {
        false
    }
}

pub fn f45(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f14(v0.clone(), v1.clone()) {
        let a1016 = match v0.clone() {
            Adt1::C0(h1014, _, _, _, _, _, _, _, _, _, _) => {
                h1014
            }
        };
        let a1017 = match v1.clone() {
            Adt1::C0(h1015, _, _, _, _, _, _, _, _, _, _) => {
                h1015
            }
        };
        nat_eq(a1016, a1017)
    } else {
        false
    } {
        let a1020 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1018) => {
                h1018
            }
        };
        let a1021 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1019) => {
                h1019
            }
        };
        nat_eq(a1020, a1021)
    } else {
        false
    } {
        let a1024 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1022, _, _, _, _, _) => {
                h1022
            }
        };
        let a1025 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1023, _, _, _, _, _) => {
                h1023
            }
        };
        nat_eq(a1024, a1025)
    } else {
        false
    } {
        let a1028 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1026, _, _, _, _) => {
                h1026
            }
        };
        let a1029 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1027, _, _, _, _) => {
                h1027
            }
        };
        nat_eq(a1028, a1029)
    } else {
        false
    } {
        let a1032 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1030, _, _, _) => {
                h1030
            }
        };
        let a1033 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1031, _, _, _) => {
                h1031
            }
        };
        nat_eq(a1032, a1033)
    } else {
        false
    } {
        let a1036 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1034, _, _) => {
                h1034
            }
        };
        let a1037 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1035, _, _) => {
                h1035
            }
        };
        nat_eq(a1036, a1037)
    } else {
        false
    }
}

pub fn f46(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f50(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f50(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f50(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f50(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f50(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f50(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f47(v0: u64) -> Adt4 {
    let a1038 = v0;
    let a1039 = 0u64;
    if nat_eq(a1038, a1039) {
        Adt4::C1(Adt5::C0(0u64, 1u64, 4u64))
    } else {
        let a1040 = v0;
        let a1041 = 1u64;
        if nat_eq(a1040, a1041) {
            Adt4::C1(Adt5::C0(1u64, 1u64, 2u64))
        } else {
            let a1042 = v0;
            let a1043 = 2u64;
            if nat_eq(a1042, a1043) {
                Adt4::C1(Adt5::C0(2u64, 32u64, 18u64))
            } else {
                let a1044 = v0;
                let a1045 = 3u64;
                if nat_eq(a1044, a1045) {
                    Adt4::C1(Adt5::C0(3u64, 32u64, 20u64))
                } else {
                    let a1046 = v0;
                    let a1047 = 6u64;
                    if nat_eq(a1046, a1047) {
                        Adt4::C1(Adt5::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1048 = v0;
                        let a1049 = 7u64;
                        if nat_eq(a1048, a1049) {
                            Adt4::C1(Adt5::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1050 = v0;
                            let a1051 = 8u64;
                            if nat_eq(a1050, a1051) {
                                Adt4::C1(Adt5::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1052 = v0;
                                let a1053 = 9u64;
                                if nat_eq(a1052, a1053) {
                                    Adt4::C1(Adt5::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1054 = v0;
                                    let a1055 = 10u64;
                                    if nat_eq(a1054, a1055) {
                                        Adt4::C1(Adt5::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1056 = v0;
                                        let a1057 = 11u64;
                                        if nat_eq(a1056, a1057) {
                                            Adt4::C1(Adt5::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1058 = v0;
                                            let a1059 = 12u64;
                                            if nat_eq(a1058, a1059) {
                                                Adt4::C1(Adt5::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1060 = v0;
                                                let a1061 = 13u64;
                                                if nat_eq(a1060, a1061) {
                                                    Adt4::C1(Adt5::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1062 = v0;
                                                    let a1063 = 14u64;
                                                    if nat_eq(a1062, a1063) {
                                                        Adt4::C1(Adt5::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1064 = v0;
                                                        let a1065 = 15u64;
                                                        if nat_eq(a1064, a1065) {
                                                            Adt4::C1(Adt5::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1066 = v0;
                                                            let a1067 = 16u64;
                                                            if nat_eq(a1066, a1067) {
                                                                Adt4::C1(Adt5::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1068 = v0;
                                                                let a1069 = 17u64;
                                                                if nat_eq(a1068, a1069) {
                                                                    Adt4::C1(Adt5::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1070 = v0;
                                                                    let a1071 = 18u64;
                                                                    if nat_eq(a1070, a1071) {
                                                                        Adt4::C1(Adt5::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1072 = v0;
                                                                        let a1073 = 19u64;
                                                                        if nat_eq(a1072, a1073) {
                                                                            Adt4::C1(Adt5::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1074 = v0;
                                                                            let a1075 = 20u64;
                                                                            if nat_eq(a1074, a1075) {
                                                                                Adt4::C1(Adt5::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1076 = v0;
                                                                                let a1077 = 21u64;
                                                                                if nat_eq(a1076, a1077) {
                                                                                    Adt4::C1(Adt5::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1078 = v0;
                                                                                    let a1079 = 22u64;
                                                                                    if nat_eq(a1078, a1079) {
                                                                                        Adt4::C1(Adt5::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1080 = v0;
                                                                                        let a1081 = 23u64;
                                                                                        if nat_eq(a1080, a1081) {
                                                                                            Adt4::C1(Adt5::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1082 = v0;
                                                                                            let a1083 = 24u64;
                                                                                            if nat_eq(a1082, a1083) {
                                                                                                Adt4::C1(Adt5::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1084 = v0;
                                                                                                let a1085 = 25u64;
                                                                                                if nat_eq(a1084, a1085) {
                                                                                                    Adt4::C1(Adt5::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1086 = v0;
                                                                                                    let a1087 = 26u64;
                                                                                                    if nat_eq(a1086, a1087) {
                                                                                                        Adt4::C1(Adt5::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1088 = v0;
                                                                                                        let a1089 = 27u64;
                                                                                                        if nat_eq(a1088, a1089) {
                                                                                                            Adt4::C1(Adt5::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1090 = v0;
                                                                                                            let a1091 = 28u64;
                                                                                                            if nat_eq(a1090, a1091) {
                                                                                                                Adt4::C1(Adt5::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1092 = v0;
                                                                                                                let a1093 = 29u64;
                                                                                                                if nat_eq(a1092, a1093) {
                                                                                                                    Adt4::C1(Adt5::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1094 = v0;
                                                                                                                    let a1095 = 30u64;
                                                                                                                    if nat_eq(a1094, a1095) {
                                                                                                                        Adt4::C1(Adt5::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1096 = v0;
                                                                                                                        let a1097 = 34u64;
                                                                                                                        if nat_eq(a1096, a1097) {
                                                                                                                            Adt4::C1(Adt5::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1098 = v0;
                                                                                                                            let a1099 = 35u64;
                                                                                                                            if nat_eq(a1098, a1099) {
                                                                                                                                Adt4::C1(Adt5::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1100 = v0;
                                                                                                                                let a1101 = 39u64;
                                                                                                                                if nat_eq(a1100, a1101) {
                                                                                                                                    Adt4::C1(Adt5::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1102 = v0;
                                                                                                                                    let a1103 = 40u64;
                                                                                                                                    if nat_eq(a1102, a1103) {
                                                                                                                                        Adt4::C1(Adt5::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1104 = v0;
                                                                                                                                        let a1105 = 41u64;
                                                                                                                                        if nat_eq(a1104, a1105) {
                                                                                                                                            Adt4::C1(Adt5::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1106 = v0;
                                                                                                                                            let a1107 = 42u64;
                                                                                                                                            if nat_eq(a1106, a1107) {
                                                                                                                                                Adt4::C1(Adt5::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1108 = v0;
                                                                                                                                                let a1109 = 43u64;
                                                                                                                                                if nat_eq(a1108, a1109) {
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

pub fn f48(v0: u64, v1: u64) -> u64 {
    let a1110 = v0;
    let a1111 = 1u64;
    if if nat_eq(a1110, a1111) {
        let a1112 = v1;
        let a1113 = 43u64;
        nat_eq(a1112, a1113)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f49(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt3 {
    let a1114 = f48(v0, v1);
    let a1115 = 2u64;
    if nat_eq(a1114, a1115) {
        let a1119 = {
            let a1116 = v2;
            let a1117 = 2u64;
            let a1118 = 0u64;
            nat_rem(a1116, a1117, a1118)
        };
        let a1120 = 0u64;
        if nat_eq(a1119, a1120) {
            Adt3::C3(0u64, 0u64, v0, 0u64, 0u64, v3, 2u64, 0u64, 1u64, 0u64, f18(v0), 8u64)
        } else {
            Adt3::C3(0u64, 0u64, 0u64, v1, 1u64, 0u64, 2u64, 0u64, 3u64, v4, 8u64, f27(v1))
        }
    } else {
        Adt3::C3(0u64, 0u64, v0, v1, 0u64, v3, 2u64, 0u64, 3u64, v4, f18(v0), f27(v1))
    }
}

pub fn f50(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a1121 = v1;
    let a1122 = 1u64;
    if if if nat_le(a1121, a1122) {
        true
    } else {
        let a1123 = v0;
        let a1124 = v2;
        nat_le(a1123, a1124)
    } {
        let a1125 = v3;
        let a1126 = 1u64;
        if nat_le(a1125, a1126) {
            true
        } else {
            let a1133 = {
                let a1131 = v0;
                let a1132 = {
                    let a1129 = {
                        let a1127 = v1;
                        let a1128 = 1u64;
                        nat_sub(a1127, a1128)
                    };
                    let a1130 = v2;
                    nat_mul(a1129, a1130)?
                };
                nat_add(a1131, a1132)?
            };
            let a1134 = v4;
            nat_le(a1133, a1134)
        }
    } else {
        false
    } {
        let a1135 = v5;
        let a1136 = 1u64;
        if nat_le(a1135, a1136) {
            Ok(true)
        } else {
            let a1149 = {
                let a1147 = {
                    let a1141 = v0;
                    let a1142 = {
                        let a1139 = {
                            let a1137 = v1;
                            let a1138 = 1u64;
                            nat_sub(a1137, a1138)
                        };
                        let a1140 = v2;
                        nat_mul(a1139, a1140)?
                    };
                    nat_add(a1141, a1142)?
                };
                let a1148 = {
                    let a1145 = {
                        let a1143 = v3;
                        let a1144 = 1u64;
                        nat_sub(a1143, a1144)
                    };
                    let a1146 = v4;
                    nat_mul(a1145, a1146)?
                };
                nat_add(a1147, a1148)?
            };
            let a1150 = v6;
            Ok(nat_le(a1149, a1150))
        }
    } else {
        Ok(false)
    }
}
