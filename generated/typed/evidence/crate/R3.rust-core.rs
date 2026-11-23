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
        if nat_eq(a79, a80) {
            let a86 = {
                let a83 = match v0.clone() {
                    Adt1::C0(_, h81, _, _, _, _, _, _, _, _, _) => {
                        h81
                    }
                };
                let a84 = f29(match v3.clone() {
                    Adt1::C0(h82, _, _, _, _, _, _, _, _, _, _) => {
                        h82
                    }
                });
                let a85 = 1u64;
                nat_rem(a83, a84, a85)
            };
            let a87 = 0u64;
            nat_eq(a86, a87)
        } else {
            false
        }
    } else {
        false
    } {
        let a90 = match v0.clone() {
            Adt1::C0(_, _, _, h88, _, _, _, _, _, _, _) => {
                h88
            }
        };
        let a91 = match v3.clone() {
            Adt1::C0(_, _, _, h89, _, _, _, _, _, _, _) => {
                h89
            }
        };
        nat_eq(a90, a91)
    } else {
        false
    } {
        let a94 = match v0.clone() {
            Adt1::C0(_, _, _, _, h92, _, _, _, _, _, _) => {
                h92
            }
        };
        let a95 = match v3.clone() {
            Adt1::C0(_, _, _, _, h93, _, _, _, _, _, _) => {
                h93
            }
        };
        nat_eq(a94, a95)
    } else {
        false
    } {
        let a98 = match v0.clone() {
            Adt1::C0(_, _, h96, _, _, _, _, _, _, _, _) => {
                h96
            }
        };
        let a99 = match v1.clone() {
            Adt1::C0(_, h97, _, _, _, _, _, _, _, _, _) => {
                h97
            }
        };
        nat_eq(a98, a99)
    } else {
        false
    } {
        let a101 = match v1.clone() {
            Adt1::C0(_, _, _, _, h100, _, _, _, _, _, _) => {
                h100
            }
        };
        let a102 = 1u64;
        nat_eq(a101, a102)
    } else {
        false
    } {
        let a104 = 0u64;
        let a105 = match v1.clone() {
            Adt1::C0(_, _, h103, _, _, _, _, _, _, _, _) => {
                h103
            }
        };
        nat_lt(a104, a105)
    } else {
        false
    } {
        let a107 = 0u64;
        let a108 = match v1.clone() {
            Adt1::C0(_, _, _, h106, _, _, _, _, _, _, _) => {
                h106
            }
        };
        nat_lt(a107, a108)
    } else {
        false
    } {
        let a114 = {
            let a111 = match v0.clone() {
                Adt1::C0(_, _, _, h109, _, _, _, _, _, _, _) => {
                    h109
                }
            };
            let a112 = match v1.clone() {
                Adt1::C0(_, _, h110, _, _, _, _, _, _, _, _) => {
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
        let a121 = {
            let a118 = match v0.clone() {
                Adt1::C0(_, _, _, _, h116, _, _, _, _, _, _) => {
                    h116
                }
            };
            let a119 = match v1.clone() {
                Adt1::C0(_, _, _, h117, _, _, _, _, _, _, _) => {
                    h117
                }
            };
            let a120 = 0u64;
            nat_rem(a118, a119, a120)
        };
        let a122 = 0u64;
        nat_eq(a121, a122)
    } else {
        false
    } {
        f30(v0.clone())
    } else {
        false
    } {
        let a124 = match v3.clone() {
            Adt1::C0(_, h123, _, _, _, _, _, _, _, _, _) => {
                h123
            }
        };
        let a125 = 1u64;
        if nat_eq(a124, a125) {
            true
        } else {
            let a128 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h126, _, _, _, _, _) => {
                    h126
                }
            };
            let a129 = f27(match v3.clone() {
                Adt1::C0(h127, _, _, _, _, _, _, _, _, _, _) => {
                    h127
                }
            });
            nat_eq(a128, a129)
        }
    } else {
        false
    } {
        let v4: u64 = f24(v1.clone())?;
        let v5: u64 = {
            let a135 = match v0.clone() {
                Adt1::C0(_, _, h130, _, _, _, _, _, _, _, _) => {
                    h130
                }
            };
            let a136 = {
                let a133 = match v0.clone() {
                    Adt1::C0(_, _, _, h131, _, _, _, _, _, _, _) => {
                        h131
                    }
                };
                let a134 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h132, _, _, _, _, _, _) => {
                        h132
                    }
                };
                nat_mul(a133, a134)?
            };
            nat_mul(a135, a136)?
        };
        let v6: u64 = if f17(v0.clone()) {
            0u64
        } else {
            let a144 = {
                let a137 = 2u64;
                let a138 = v4;
                nat_mul(a137, a138)?
            };
            let a145 = {
                let a142 = v5;
                let a143 = {
                    let a140 = 1u64;
                    let a141 = f31(v0.clone(), match v3.clone() {
                        Adt1::C0(h139, _, _, _, _, _, _, _, _, _, _) => {
                            h139
                        }
                    })?;
                    nat_add(a140, a141)?
                };
                nat_mul(a142, a143)?
            };
            nat_add(a144, a145)?
        };
        Ok(Adt3::C2(Adt2::C0(v6, if f17(v0.clone()) {
            3u64
        } else {
            let a146 = 3u64;
            let a147 = v4;
            nat_add(a146, a147)?
        }, 41u64, 0u64, 0u64, v0, v1, v3)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f4(v0: Adt1, v1: Adt1, v2: Adt1) -> R<Adt3> {
    let v3: u64 = f29(match v2.clone() {
        Adt1::C0(h148, _, _, _, _, _, _, _, _, _, _) => {
            h148
        }
    });
    if if if if if if if f32(match v2.clone() {
        Adt1::C0(h149, _, _, _, _, _, _, _, _, _, _) => {
            h149
        }
    }) {
        f26(v1.clone(), v2.clone())
    } else {
        false
    } {
        let a151 = 0u64;
        let a152 = f18(match v0.clone() {
            Adt1::C0(h150, _, _, _, _, _, _, _, _, _, _) => {
                h150
            }
        });
        nat_lt(a151, a152)
    } else {
        false
    } {
        let a155 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h153, _, _, _, _, _) => {
                h153
            }
        };
        let a156 = f18(match v0.clone() {
            Adt1::C0(h154, _, _, _, _, _, _, _, _, _, _) => {
                h154
            }
        });
        nat_eq(a155, a156)
    } else {
        false
    } {
        let a161 = {
            let a158 = match v0.clone() {
                Adt1::C0(_, h157, _, _, _, _, _, _, _, _, _) => {
                    h157
                }
            };
            let a159 = v3;
            let a160 = 1u64;
            nat_rem(a158, a159, a160)
        };
        let a162 = 0u64;
        if nat_eq(a161, a162) {
            let a167 = {
                let a164 = match v2.clone() {
                    Adt1::C0(_, h163, _, _, _, _, _, _, _, _, _) => {
                        h163
                    }
                };
                let a165 = v3;
                let a166 = 1u64;
                nat_rem(a164, a165, a166)
            };
            let a168 = 0u64;
            nat_eq(a167, a168)
        } else {
            false
        }
    } else {
        false
    } {
        let a171 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h169, _, _, _, _, _) => {
                h169
            }
        };
        let a172 = f27(match v2.clone() {
            Adt1::C0(h170, _, _, _, _, _, _, _, _, _, _) => {
                h170
            }
        });
        nat_eq(a171, a172)
    } else {
        false
    } {
        let a173 = f24(v0.clone())?;
        let a174 = f24(v2.clone())?;
        nat_eq(a173, a174)
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a175 = f24(v0.clone())?;
            let a176 = v3;
            let a177 = 0u64;
            nat_quot(a175, a176, a177)
        }, 3u64, 33u64, 0u64, 0u64, v0, v1, v2)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f5(v0: u64, v1: u64, v2: Adt1, v3: Adt1) -> R<Adt3> {
    let a179 = match v2.clone() {
        Adt1::C0(h178, _, _, _, _, _, _, _, _, _, _) => {
            h178
        }
    };
    let a180 = 0u64;
    if if if if if if if if if if if nat_eq(a179, a180) {
        let a182 = match v3.clone() {
            Adt1::C0(h181, _, _, _, _, _, _, _, _, _, _) => {
                h181
            }
        };
        let a183 = 26u64;
        nat_eq(a182, a183)
    } else {
        false
    } {
        let a185 = match v2.clone() {
            Adt1::C0(_, h184, _, _, _, _, _, _, _, _, _) => {
                h184
            }
        };
        let a186 = 2147483647u64;
        nat_le(a185, a186)
    } else {
        false
    } {
        let a189 = match v3.clone() {
            Adt1::C0(_, h187, _, _, _, _, _, _, _, _, _) => {
                h187
            }
        };
        let a190 = match v2.clone() {
            Adt1::C0(_, h188, _, _, _, _, _, _, _, _, _) => {
                h188
            }
        };
        nat_le(a189, a190)
    } else {
        false
    } {
        let a193 = match v2.clone() {
            Adt1::C0(_, _, h191, _, _, _, _, _, _, _, _) => {
                h191
            }
        };
        let a194 = match v3.clone() {
            Adt1::C0(_, _, h192, _, _, _, _, _, _, _, _) => {
                h192
            }
        };
        nat_eq(a193, a194)
    } else {
        false
    } {
        let a197 = match v2.clone() {
            Adt1::C0(_, _, _, h195, _, _, _, _, _, _, _) => {
                h195
            }
        };
        let a198 = match v3.clone() {
            Adt1::C0(_, _, _, h196, _, _, _, _, _, _, _) => {
                h196
            }
        };
        nat_eq(a197, a198)
    } else {
        false
    } {
        let a201 = match v2.clone() {
            Adt1::C0(_, _, _, _, h199, _, _, _, _, _, _) => {
                h199
            }
        };
        let a202 = match v3.clone() {
            Adt1::C0(_, _, _, _, h200, _, _, _, _, _, _) => {
                h200
            }
        };
        nat_eq(a201, a202)
    } else {
        false
    } {
        let a203 = v0;
        let a204 = 67u64;
        if nat_eq(a203, a204) {
            true
        } else {
            let a205 = v1;
            let a206 = 1u64;
            if nat_le(a205, a206) {
                let a209 = match v3.clone() {
                    Adt1::C0(_, h207, _, _, _, _, _, _, _, _, _) => {
                        h207
                    }
                };
                let a210 = match v2.clone() {
                    Adt1::C0(_, h208, _, _, _, _, _, _, _, _, _) => {
                        h208
                    }
                };
                nat_eq(a209, a210)
            } else {
                false
            }
        }
    } else {
        false
    } {
        f33(v2.clone())?
    } else {
        false
    } {
        f33(v3.clone())?
    } else {
        false
    } {
        f34(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a216 = match v3.clone() {
                Adt1::C0(_, _, h211, _, _, _, _, _, _, _, _) => {
                    h211
                }
            };
            let a217 = {
                let a214 = match v3.clone() {
                    Adt1::C0(_, _, _, h212, _, _, _, _, _, _, _) => {
                        h212
                    }
                };
                let a215 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h213, _, _, _, _, _, _) => {
                        h213
                    }
                };
                nat_mul(a214, a215)?
            };
            nat_mul(a216, a217)?
        }, 3u64, v0, {
            let a218 = v0;
            let a219 = 67u64;
            if nat_eq(a218, a219) {
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
    let a220 = v0;
    let a221 = 2u64;
    if if nat_le(a220, a221) {
        true
    } else {
        let a222 = v0;
        let a223 = 5u64;
        nat_eq(a222, a223)
    } {
        true
    } else {
        let a224 = v0;
        let a225 = 6u64;
        nat_eq(a224, a225)
    }
}

pub fn f7(v0: u64, v1: bool, v2: u64, v3: u64, v4: Adt1, v5: Adt1, v6: Adt1) -> R<Adt3> {
    let a226 = v2;
    let a227 = 1u64;
    if if if if if if if if if if if if if if if nat_le(a226, a227) {
        let a229 = 0u64;
        let a230 = f18(match v4.clone() {
            Adt1::C0(h228, _, _, _, _, _, _, _, _, _, _) => {
                h228
            }
        });
        nat_lt(a229, a230)
    } else {
        false
    } {
        let a233 = match v4.clone() {
            Adt1::C0(h231, _, _, _, _, _, _, _, _, _, _) => {
                h231
            }
        };
        let a234 = match v5.clone() {
            Adt1::C0(h232, _, _, _, _, _, _, _, _, _, _) => {
                h232
            }
        };
        nat_eq(a233, a234)
    } else {
        false
    } {
        let a237 = match v4.clone() {
            Adt1::C0(h235, _, _, _, _, _, _, _, _, _, _) => {
                h235
            }
        };
        let a238 = match v6.clone() {
            Adt1::C0(h236, _, _, _, _, _, _, _, _, _, _) => {
                h236
            }
        };
        nat_eq(a237, a238)
    } else {
        false
    } {
        f14(v4.clone(), v5.clone())
    } else {
        false
    } {
        f35(v4.clone())?
    } else {
        false
    } {
        f35(v5.clone())?
    } else {
        false
    } {
        f35(v6.clone())?
    } else {
        false
    } {
        let a241 = match v4.clone() {
            Adt1::C0(_, _, h239, _, _, _, _, _, _, _, _) => {
                h239
            }
        };
        let a242 = match v6.clone() {
            Adt1::C0(_, _, h240, _, _, _, _, _, _, _, _) => {
                h240
            }
        };
        nat_eq(a241, a242)
    } else {
        false
    } {
        let a245 = match v4.clone() {
            Adt1::C0(_, _, _, h243, _, _, _, _, _, _, _) => {
                h243
            }
        };
        let a246 = match v6.clone() {
            Adt1::C0(_, _, _, h244, _, _, _, _, _, _, _) => {
                h244
            }
        };
        nat_eq(a245, a246)
    } else {
        false
    } {
        let a249 = match v4.clone() {
            Adt1::C0(_, _, _, _, h247, _, _, _, _, _, _) => {
                h247
            }
        };
        let a250 = match v6.clone() {
            Adt1::C0(_, _, _, _, h248, _, _, _, _, _, _) => {
                h248
            }
        };
        nat_eq(a249, a250)
    } else {
        false
    } {
        let a257 = match v6.clone() {
            Adt1::C0(_, h251, _, _, _, _, _, _, _, _, _) => {
                h251
            }
        };
        let a258 = if v1 {
            match v4.clone() {
                Adt1::C0(_, h252, _, _, _, _, _, _, _, _, _) => {
                    h252
                }
            }
        } else {
            let a254 = match v4.clone() {
                Adt1::C0(_, h253, _, _, _, _, _, _, _, _, _) => {
                    h253
                }
            };
            let a255 = 2u64;
            let a256 = 0u64;
            nat_quot(a254, a255, a256)
        };
        nat_eq(a257, a258)
    } else {
        false
    } {
        let a260 = match v6.clone() {
            Adt1::C0(_, h259, _, _, _, _, _, _, _, _, _) => {
                h259
            }
        };
        let a261 = 2147483647u64;
        nat_le(a260, a261)
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
            let a262 = v0;
            let a263 = 6u64;
            if nat_eq(a262, a263) {
                let a266 = 1u64;
                let a267 = {
                    let a264 = 10u64;
                    let a265 = f24(v6.clone())?;
                    nat_mul(a264, a265)?
                };
                nat_add(a266, a267)?
            } else {
                let a268 = 4u64;
                let a269 = f24(v6.clone())?;
                nat_mul(a268, a269)?
            }
        }, {
            let a270 = v0;
            let a271 = 6u64;
            if nat_eq(a270, a271) {
                4u64
            } else {
                3u64
            }
        }, if v1 {
            98u64
        } else {
            let a272 = 99u64;
            let a273 = v2;
            nat_add(a272, a273)?
        }, v0, v3, v4, v5, v6)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f8(v0: u64, v1: Adt2) -> R<Adt3> {
    let a275 = v0;
    let a276 = match v1.clone() {
        Adt2::C0(h274, _, _, _, _, _, _, _) => {
            h274
        }
    };
    if nat_lt(a275, a276) {
        let v2: Adt1 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h277) => {
                h277
            }
        };
        let v3: u64 = f29(match v2.clone() {
            Adt1::C0(h278, _, _, _, _, _, _, _, _, _, _) => {
                h278
            }
        });
        let v4: u64 = {
            let a280 = match v2.clone() {
                Adt1::C0(_, h279, _, _, _, _, _, _, _, _, _) => {
                    h279
                }
            };
            let a281 = v3;
            let a282 = 0u64;
            nat_quot(a280, a281, a282)
        };
        let v5: u64 = {
            let a283 = v0;
            let a284 = v4;
            let a285 = 0u64;
            nat_quot(a283, a284, a285)
        };
        let v6: u64 = {
            let a287 = v5;
            let a288 = match v2.clone() {
                Adt1::C0(_, _, h286, _, _, _, _, _, _, _, _) => {
                    h286
                }
            };
            let a289 = 0u64;
            nat_quot(a287, a288, a289)
        };
        Ok(Adt3::C6(f36({
            let a290 = v0;
            let a291 = v3;
            nat_mul(a290, a291)?
        }, match v1 {
            Adt2::C0(_, _, _, _, _, h292, _, _) => {
                h292
            }
        })?, f37({
            let a293 = v0;
            let a294 = v4;
            let a295 = 0u64;
            nat_rem(a293, a294, a295)
        }, {
            let a297 = v5;
            let a298 = match v2.clone() {
                Adt1::C0(_, _, h296, _, _, _, _, _, _, _, _) => {
                    h296
                }
            };
            let a299 = 0u64;
            nat_rem(a297, a298, a299)
        }, {
            let a301 = v6;
            let a302 = match v2.clone() {
                Adt1::C0(_, _, _, h300, _, _, _, _, _, _, _) => {
                    h300
                }
            };
            let a303 = 0u64;
            nat_rem(a301, a302, a303)
        }, {
            let a305 = v6;
            let a306 = match v2.clone() {
                Adt1::C0(_, _, _, h304, _, _, _, _, _, _, _) => {
                    h304
                }
            };
            let a307 = 0u64;
            nat_quot(a305, a306, a307)
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, h308, _, _, _, _, _) => {
                h308
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h309, _, _, _, _) => {
                h309
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h310, _, _, _) => {
                h310
            }
        }, match v2 {
            Adt1::C0(_, _, _, _, _, _, _, _, h311, _, _) => {
                h311
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f9(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, h312, _, _) => {
            h312
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h313) => {
            h313
        }
    };
    let a315 = v0;
    let a316 = match v1.clone() {
        Adt2::C0(h314, _, _, _, _, _, _, _) => {
            h314
        }
    };
    if nat_lt(a315, a316) {
        Ok(Adt3::C5({
            let a318 = v0;
            let a319 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, _, h317, _, _, _, _) => {
                    h317
                }
            };
            nat_mul(a318, a319)?
        }, {
            let a321 = v0;
            let a322 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h320, _, _, _, _) => {
                    h320
                }
            };
            nat_mul(a321, a322)?
        }, match v2 {
            Adt1::C0(_, h323, _, _, _, _, _, _, _, _, _) => {
                h323
            }
        }, match v3 {
            Adt1::C0(_, h324, _, _, _, _, _, _, _, _, _) => {
                h324
            }
        }, {
            let a326 = match v1 {
                Adt2::C0(_, _, _, h325, _, _, _, _) => {
                    h325
                }
            };
            let a327 = 1u64;
            nat_eq(a326, a327)
        }))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f10(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let a329 = v0;
    let a330 = match v1.clone() {
        Adt2::C0(h328, _, _, _, _, _, _, _) => {
            h328
        }
    };
    if nat_lt(a329, a330) {
        let v3: u64 = {
            let a332 = 2u64;
            let a333 = f24(match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, h331, _) => {
                    h331
                }
            })?;
            nat_mul(a332, a333)?
        };
        let a334 = v0;
        let a335 = v3;
        if nat_lt(a334, a335) {
            f38(v0, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, h336, _) => {
                    h336
                }
            }, {
                let m339 = match v1 {
                    Adt2::C0(_, _, _, _, _, _, _, h338) => {
                        h338
                    }
                };
                match m339 {
                    Adt1::C0(_, _, h337, _, _, _, _, _, _, _, _) => {
                        h337
                    }
                }
            }, v2)
        } else {
            f39({
                let a340 = v0;
                let a341 = v3;
                nat_sub(a340, a341)
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, h342, _, _) => {
                    h342
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, h343, _) => {
                    h343
                }
            }, match v1 {
                Adt2::C0(_, _, _, _, _, _, _, h344) => {
                    h344
                }
            }, v2)
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f11(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(_, _, _, _, h345, _, _, _) => {
            h345
        }
    };
    let a347 = v0;
    let a348 = match v1.clone() {
        Adt2::C0(h346, _, _, _, _, _, _, _) => {
            h346
        }
    };
    if nat_lt(a347, a348) {
        let a349 = v0;
        let a350 = 0u64;
        if nat_eq(a349, a350) {
            Ok(Adt3::C3(1u64, 6u64, 0u64, 0u64, 2u64, v3, 2u64, 0u64, 0u64, 3u64, 4u64, 4u64))
        } else {
            let v4: u64 = {
                let a351 = v0;
                let a352 = 1u64;
                nat_sub(a351, a352)
            };
            let v5: u64 = {
                let a353 = v4;
                let a354 = 10u64;
                let a355 = 0u64;
                nat_rem(a353, a354, a355)
            };
            let v6: u64 = {
                let a359 = 4u64;
                let a360 = {
                    let a356 = v4;
                    let a357 = 10u64;
                    let a358 = 0u64;
                    nat_quot(a356, a357, a358)
                };
                nat_mul(a359, a360)?
            };
            let a361 = v5;
            let a362 = 2u64;
            if nat_lt(a361, a362) {
                f12({
                    let a363 = v6;
                    let a364 = v5;
                    nat_add(a363, a364)?
                }, v1)
            } else {
                let a365 = v5;
                let a366 = 2u64;
                if nat_eq(a365, a366) {
                    Ok(f40(0u64, 2u64, v3))
                } else {
                    let a367 = v5;
                    let a368 = 3u64;
                    if nat_eq(a367, a368) {
                        Ok(f41(0u64, 2u64, v3, {
                            let a369 = v2;
                            let a370 = 2u64;
                            nat_eq(a369, a370)
                        }))
                    } else {
                        let a371 = v5;
                        let a372 = 4u64;
                        if nat_eq(a371, a372) {
                            Ok(f40(1u64, 1u64, 3u64))
                        } else {
                            let a373 = v5;
                            let a374 = 5u64;
                            if nat_eq(a373, a374) {
                                Ok(f41(1u64, 1u64, 3u64, {
                                    let a375 = v2;
                                    let a376 = 0u64;
                                    nat_eq(a375, a376)
                                }))
                            } else {
                                let a377 = v5;
                                let a378 = 6u64;
                                if nat_eq(a377, a378) {
                                    Ok(f40(1u64, 2u64, v3))
                                } else {
                                    let a379 = v5;
                                    let a380 = 7u64;
                                    if nat_eq(a379, a380) {
                                        Ok(f41(1u64, 2u64, v3, {
                                            let a381 = v2;
                                            let a382 = 2u64;
                                            nat_eq(a381, a382)
                                        }))
                                    } else {
                                        let a383 = v5;
                                        let a384 = 8u64;
                                        if nat_eq(a383, a384) {
                                            Ok(Adt3::C4(2u64, {
                                                let m387 = match v1 {
                                                    Adt2::C0(_, _, _, _, _, _, _, h386) => {
                                                        h386
                                                    }
                                                };
                                                match m387 {
                                                    Adt1::C0(h385, _, _, _, _, _, _, _, _, _, _) => {
                                                        h385
                                                    }
                                                }
                                            }, 0u64, 1u64, 0u64))
                                        } else {
                                            f12({
                                                let a388 = v6;
                                                let a389 = 3u64;
                                                nat_add(a388, a389)?
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
        Adt2::C0(_, _, _, _, _, h390, _, _) => {
            h390
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h391, _) => {
            h391
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h392) => {
            h392
        }
    };
    let v5: u64 = match v1.clone() {
        Adt2::C0(_, _, h393, _, _, _, _, _) => {
            h393
        }
    };
    let v6: u64 = {
        let a394 = v0;
        let a395 = 4u64;
        let a396 = 0u64;
        nat_rem(a394, a395, a396)
    };
    let v7: u64 = {
        let a397 = v0;
        let a398 = 4u64;
        let a399 = 0u64;
        nat_quot(a397, a398, a399)
    };
    let v8: u64 = {
        let a401 = v7;
        let a402 = match v4.clone() {
            Adt1::C0(_, h400, _, _, _, _, _, _, _, _, _) => {
                h400
            }
        };
        let a403 = 0u64;
        nat_quot(a401, a402, a403)
    };
    let v9: u64 = f18(match v4.clone() {
        Adt1::C0(h404, _, _, _, _, _, _, _, _, _, _) => {
            h404
        }
    });
    let v10: u64 = {
        let a409 = {
            let a406 = v7;
            let a407 = match v4.clone() {
                Adt1::C0(_, h405, _, _, _, _, _, _, _, _, _) => {
                    h405
                }
            };
            let a408 = 0u64;
            nat_rem(a406, a407, a408)
        };
        let a410 = v9;
        nat_mul(a409, a410)?
    };
    let v11: u64 = {
        let a412 = match v4.clone() {
            Adt1::C0(_, h411, _, _, _, _, _, _, _, _, _) => {
                h411
            }
        };
        let a413 = v9;
        nat_mul(a412, a413)?
    };
    let a415 = v0;
    let a416 = match v1.clone() {
        Adt2::C0(h414, _, _, _, _, _, _, _) => {
            h414
        }
    };
    if nat_lt(a415, a416) {
        let a417 = v6;
        let a418 = 0u64;
        if nat_eq(a417, a418) {
            Ok(Adt3::C3(0u64, 0u64, match v2.clone() {
                Adt1::C0(h419, _, _, _, _, _, _, _, _, _, _) => {
                    h419
                }
            }, 0u64, 0u64, {
                let a427 = {
                    let a423 = {
                        let a421 = v8;
                        let a422 = match v2 {
                            Adt1::C0(_, _, _, _, _, _, h420, _, _, _, _) => {
                                h420
                            }
                        };
                        nat_mul(a421, a422)?
                    };
                    let a424 = v10;
                    nat_add(a423, a424)?
                };
                let a428 = {
                    let a425 = v5;
                    let a426 = 100u64;
                    if nat_eq(a425, a426) {
                        v11
                    } else {
                        0u64
                    }
                };
                nat_add(a427, a428)?
            }, 2u64, 0u64, 0u64, 0u64, v9, 4u64))
        } else {
            let a429 = v6;
            let a430 = 1u64;
            if nat_eq(a429, a430) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h431, _, _, _, _, _, _, _, _, _, _) => {
                        h431
                    }
                }, 0u64, {
                    let a432 = v5;
                    let a433 = 98u64;
                    if nat_eq(a432, a433) {
                        4u64
                    } else {
                        0u64
                    }
                }, {
                    let a441 = {
                        let a437 = {
                            let a435 = v8;
                            let a436 = match v3 {
                                Adt1::C0(_, _, _, _, _, _, h434, _, _, _, _) => {
                                    h434
                                }
                            };
                            nat_mul(a435, a436)?
                        };
                        let a438 = v10;
                        nat_add(a437, a438)?
                    };
                    let a442 = {
                        let a439 = v5;
                        let a440 = 99u64;
                        if nat_eq(a439, a440) {
                            v11
                        } else {
                            0u64
                        }
                    };
                    nat_add(a441, a442)?
                }, 2u64, 0u64, 0u64, 1u64, v9, 4u64))
            } else {
                let a443 = v6;
                let a444 = 2u64;
                if nat_eq(a443, a444) {
                    Ok(Adt3::C4(match v1 {
                        Adt2::C0(_, _, _, h445, _, _, _, _) => {
                            h445
                        }
                    }, match v4 {
                        Adt1::C0(h446, _, _, _, _, _, _, _, _, _, _) => {
                            h446
                        }
                    }, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt3::C3(0u64, 0u64, 0u64, match v4.clone() {
                        Adt1::C0(h447, _, _, _, _, _, _, _, _, _, _) => {
                            h447
                        }
                    }, 1u64, 0u64, 2u64, 0u64, 3u64, {
                        let a451 = {
                            let a449 = v8;
                            let a450 = match v4 {
                                Adt1::C0(_, _, _, _, _, _, h448, _, _, _, _) => {
                                    h448
                                }
                            };
                            nat_mul(a449, a450)?
                        };
                        let a452 = v10;
                        nat_add(a451, a452)?
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
        Adt2::C0(_, _, _, _, _, h453, _, _) => {
            h453
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h454, _) => {
            h454
        }
    };
    let v5: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h455) => {
            h455
        }
    };
    let v6: u64 = {
        let a457 = match v1.clone() {
            Adt2::C0(_, h456, _, _, _, _, _, _) => {
                h456
            }
        };
        let a458 = 3u64;
        if nat_eq(a457, a458) {
            0u64
        } else {
            let a459 = f24(v3.clone())?;
            let a460 = 2u64;
            nat_mul(a459, a460)?
        }
    };
    let a462 = match v1.clone() {
        Adt2::C0(h461, _, _, _, _, _, _, _) => {
            h461
        }
    };
    let a463 = v0;
    if nat_le(a462, a463) {
        Ok(Adt3::C1)
    } else {
        let a464 = v0;
        let a465 = v6;
        if nat_lt(a464, a465) {
            let v7: u64 = {
                let a466 = v0;
                let a467 = 2u64;
                let a468 = 0u64;
                nat_quot(a466, a467, a468)
            };
            let v8: u64 = {
                let a473 = 3u64;
                let a474 = {
                    let a470 = f36(v7, v5.clone())?;
                    let a471 = f18(match v5 {
                        Adt1::C0(h469, _, _, _, _, _, _, _, _, _, _) => {
                            h469
                        }
                    });
                    let a472 = 0u64;
                    nat_quot(a470, a471, a472)
                };
                nat_add(a473, a474)?
            };
            let a478 = {
                let a475 = v0;
                let a476 = 2u64;
                let a477 = 0u64;
                nat_rem(a475, a476, a477)
            };
            let a479 = 0u64;
            if nat_eq(a478, a479) {
                Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, v8, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
            } else {
                let a480 = v2;
                let a481 = 0u64;
                if nat_eq(a480, a481) {
                    Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 2u64, 1u64, 2u64, 0u64, 0u64, v8, 8u64, 8u64))
                } else {
                    Ok(Adt3::C1)
                }
            }
        } else {
            let v9: u64 = {
                let a482 = v0;
                let a483 = v6;
                nat_sub(a482, a483)
            };
            let v10: u64 = f25(match v3.clone() {
                Adt1::C0(h484, _, _, _, _, _, _, _, _, _, _) => {
                    h484
                }
            }, match v4.clone() {
                Adt1::C0(h485, _, _, _, _, _, _, _, _, _, _) => {
                    h485
                }
            }, match v5.clone() {
                Adt1::C0(h486, _, _, _, _, _, _, _, _, _, _) => {
                    h486
                }
            })?;
            let v11: u64 = {
                let a487 = v9;
                let a488 = v10;
                let a489 = 0u64;
                nat_quot(a487, a488, a489)
            };
            let v12: u64 = {
                let a490 = v9;
                let a491 = v10;
                let a492 = 0u64;
                nat_rem(a490, a491, a492)
            };
            let v13: u64 = {
                let a494 = match v3.clone() {
                    Adt1::C0(h493, _, _, _, _, _, _, _, _, _, _) => {
                        h493
                    }
                };
                let a495 = 0u64;
                if nat_eq(a494, a495) {
                    0u64
                } else {
                    1u64
                }
            };
            let v14: u64 = {
                let a497 = match v4.clone() {
                    Adt1::C0(h496, _, _, _, _, _, _, _, _, _, _) => {
                        h496
                    }
                };
                let a498 = 0u64;
                if nat_eq(a497, a498) {
                    0u64
                } else {
                    1u64
                }
            };
            let a499 = v12;
            let a500 = v13;
            if nat_lt(a499, a500) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h501, _, _, _, _, _, _, _, _, _, _) => {
                        h501
                    }
                }, 0u64, 0u64, f36(v11, v3.clone())?, 2u64, 0u64, 0u64, 0u64, f18(match v3 {
                    Adt1::C0(h502, _, _, _, _, _, _, _, _, _, _) => {
                        h502
                    }
                }), 4u64))
            } else {
                let a505 = v12;
                let a506 = {
                    let a503 = v13;
                    let a504 = v14;
                    nat_add(a503, a504)?
                };
                if nat_lt(a505, a506) {
                    Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                        Adt1::C0(h507, _, _, _, _, _, _, _, _, _, _) => {
                            h507
                        }
                    }, 0u64, 4u64, f42(v11, v3, v4.clone())?, 2u64, 0u64, 0u64, 1u64, f18(match v4 {
                        Adt1::C0(h508, _, _, _, _, _, _, _, _, _, _) => {
                            h508
                        }
                    }), 4u64))
                } else {
                    let a511 = v12;
                    let a512 = {
                        let a509 = v13;
                        let a510 = v14;
                        nat_add(a509, a510)?
                    };
                    if nat_eq(a511, a512) {
                        Ok(Adt3::C3(1u64, match v1 {
                            Adt2::C0(_, _, h513, _, _, _, _, _) => {
                                h513
                            }
                        }, 0u64, 0u64, {
                            let a514 = v13;
                            let a515 = 0u64;
                            if nat_eq(a514, a515) {
                                0u64
                            } else {
                                1u64
                            }
                        }, {
                            let a516 = v13;
                            let a517 = 0u64;
                            if nat_eq(a516, a517) {
                                f36(v11, v3.clone())?
                            } else {
                                0u64
                            }
                        }, {
                            let a518 = v14;
                            let a519 = 0u64;
                            if nat_eq(a518, a519) {
                                4u64
                            } else {
                                1u64
                            }
                        }, {
                            let a520 = v14;
                            let a521 = 0u64;
                            if nat_eq(a520, a521) {
                                f42(v11, v3, v4)?
                            } else {
                                1u64
                            }
                        }, {
                            let a523 = match v5.clone() {
                                Adt1::C0(h522, _, _, _, _, _, _, _, _, _, _) => {
                                    h522
                                }
                            };
                            let a524 = 0u64;
                            if nat_eq(a523, a524) {
                                3u64
                            } else {
                                0u64
                            }
                        }, {
                            let a526 = match v5.clone() {
                                Adt1::C0(h525, _, _, _, _, _, _, _, _, _, _) => {
                                    h525
                                }
                            };
                            let a527 = 0u64;
                            if nat_eq(a526, a527) {
                                f36(v11, v5)?
                            } else {
                                0u64
                            }
                        }, 4u64, 4u64))
                    } else {
                        Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                            Adt1::C0(h528, _, _, _, _, _, _, _, _, _, _) => {
                                h528
                            }
                        }, 1u64, 0u64, 2u64, 0u64, 3u64, f36(v11, v5.clone())?, 4u64, f18(match v5 {
                            Adt1::C0(h529, _, _, _, _, _, _, _, _, _, _) => {
                                h529
                            }
                        })))
                    }
                }
            }
        }
    }
}

pub fn f14(v0: Adt1, v1: Adt1) -> bool {
    let a532 = match v0.clone() {
        Adt1::C0(_, h530, _, _, _, _, _, _, _, _, _) => {
            h530
        }
    };
    let a533 = match v1.clone() {
        Adt1::C0(_, h531, _, _, _, _, _, _, _, _, _) => {
            h531
        }
    };
    if if if nat_eq(a532, a533) {
        let a536 = match v0.clone() {
            Adt1::C0(_, _, h534, _, _, _, _, _, _, _, _) => {
                h534
            }
        };
        let a537 = match v1.clone() {
            Adt1::C0(_, _, h535, _, _, _, _, _, _, _, _) => {
                h535
            }
        };
        nat_eq(a536, a537)
    } else {
        false
    } {
        let a540 = match v0.clone() {
            Adt1::C0(_, _, _, h538, _, _, _, _, _, _, _) => {
                h538
            }
        };
        let a541 = match v1.clone() {
            Adt1::C0(_, _, _, h539, _, _, _, _, _, _, _) => {
                h539
            }
        };
        nat_eq(a540, a541)
    } else {
        false
    } {
        let a544 = match v0 {
            Adt1::C0(_, _, _, _, h542, _, _, _, _, _, _) => {
                h542
            }
        };
        let a545 = match v1 {
            Adt1::C0(_, _, _, _, h543, _, _, _, _, _, _) => {
                h543
            }
        };
        nat_eq(a544, a545)
    } else {
        false
    }
}

pub fn f15(v0: Adt1, v1: Adt1) -> bool {
    if f17(v1.clone()) {
        f17(v0)
    } else {
        let a551 = {
            let a548 = match v0.clone() {
                Adt1::C0(_, h546, _, _, _, _, _, _, _, _, _) => {
                    h546
                }
            };
            let a549 = match v1.clone() {
                Adt1::C0(_, h547, _, _, _, _, _, _, _, _, _) => {
                    h547
                }
            };
            let a550 = 1u64;
            nat_rem(a548, a549, a550)
        };
        let a552 = 0u64;
        if if if nat_eq(a551, a552) {
            let a558 = {
                let a555 = match v0.clone() {
                    Adt1::C0(_, _, h553, _, _, _, _, _, _, _, _) => {
                        h553
                    }
                };
                let a556 = match v1.clone() {
                    Adt1::C0(_, _, h554, _, _, _, _, _, _, _, _) => {
                        h554
                    }
                };
                let a557 = 1u64;
                nat_rem(a555, a556, a557)
            };
            let a559 = 0u64;
            nat_eq(a558, a559)
        } else {
            false
        } {
            let a565 = {
                let a562 = match v0.clone() {
                    Adt1::C0(_, _, _, h560, _, _, _, _, _, _, _) => {
                        h560
                    }
                };
                let a563 = match v1.clone() {
                    Adt1::C0(_, _, _, h561, _, _, _, _, _, _, _) => {
                        h561
                    }
                };
                let a564 = 1u64;
                nat_rem(a562, a563, a564)
            };
            let a566 = 0u64;
            nat_eq(a565, a566)
        } else {
            false
        } {
            let a572 = {
                let a569 = match v0 {
                    Adt1::C0(_, _, _, _, h567, _, _, _, _, _, _) => {
                        h567
                    }
                };
                let a570 = match v1 {
                    Adt1::C0(_, _, _, _, h568, _, _, _, _, _, _) => {
                        h568
                    }
                };
                let a571 = 1u64;
                nat_rem(a569, a570, a571)
            };
            let a573 = 0u64;
            nat_eq(a572, a573)
        } else {
            false
        }
    }
}

pub fn f16(v0: u64, v1: u64, v2: u64) -> bool {
    let a574 = v0;
    let a575 = 0u64;
    if if if nat_eq(a574, a575) {
        let a576 = v1;
        let a577 = 0u64;
        nat_eq(a576, a577)
    } else {
        false
    } {
        let a578 = v2;
        let a579 = 0u64;
        nat_eq(a578, a579)
    } else {
        false
    } {
        true
    } else {
        let a580 = v0;
        let a581 = 1u64;
        if if nat_eq(a580, a581) {
            true
        } else {
            let a582 = v0;
            let a583 = 30u64;
            nat_eq(a582, a583)
        } {
            let a584 = v1;
            let a585 = v0;
            if if nat_eq(a584, a585) {
                let a586 = v2;
                let a587 = v0;
                nat_eq(a586, a587)
            } else {
                false
            } {
                true
            } else {
                let a588 = v1;
                let a589 = 0u64;
                if nat_eq(a588, a589) {
                    let a590 = v2;
                    let a591 = v0;
                    if nat_eq(a590, a591) {
                        true
                    } else {
                        let a592 = v2;
                        let a593 = 0u64;
                        nat_eq(a592, a593)
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
    f43(match v0.clone() {
        Adt1::C0(_, h594, _, _, _, _, _, _, _, _, _) => {
            h594
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h595, _, _, _, _, _, _, _, _) => {
            h595
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h596, _, _, _, _, _, _, _) => {
            h596
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, h597, _, _, _, _, _, _) => {
            h597
        }
    })
}

pub fn f18(v0: u64) -> u64 {
    let a598 = v0;
    let a599 = 0u64;
    if nat_eq(a598, a599) {
        4u64
    } else {
        let a600 = v0;
        let a601 = 1u64;
        if if nat_eq(a600, a601) {
            true
        } else {
            let a602 = v0;
            let a603 = 30u64;
            nat_eq(a602, a603)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f19(v0: Adt1) -> bool {
    let v1: u64 = f18(match v0.clone() {
        Adt1::C0(h604, _, _, _, _, _, _, _, _, _, _) => {
            h604
        }
    });
    let a609 = {
        let a606 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h605) => {
                h605
            }
        };
        let a607 = v1;
        let a608 = 1u64;
        nat_rem(a606, a607, a608)
    };
    let a610 = 0u64;
    if if nat_eq(a609, a610) {
        f44(match v0.clone() {
            Adt1::C0(_, h611, _, _, _, _, _, _, _, _, _) => {
                h611
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h612, _, _, _, _, _) => {
                h612
            }
        }, v1)
    } else {
        false
    } {
        f45(match v0.clone() {
            Adt1::C0(_, _, h613, _, _, _, _, _, _, _, _) => {
                h613
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h614, _, _, _, _, _, _, _) => {
                h614
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h615, _, _, _, _, _, _) => {
                h615
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h616, _, _, _, _) => {
                h616
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h617, _, _, _) => {
                h617
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h618, _, _) => {
                h618
            }
        }, v1)
    } else {
        false
    }
}

pub fn f20(v0: Adt1, v1: Adt1) -> R<bool> {
    let a626 = {
        let a624 = match v0.clone() {
            Adt1::C0(_, _, h619, _, _, _, _, _, _, _, _) => {
                h619
            }
        };
        let a625 = {
            let a622 = match v0.clone() {
                Adt1::C0(_, _, _, h620, _, _, _, _, _, _, _) => {
                    h620
                }
            };
            let a623 = match v0.clone() {
                Adt1::C0(_, _, _, _, h621, _, _, _, _, _, _) => {
                    h621
                }
            };
            nat_mul(a622, a623)?
        };
        nat_mul(a624, a625)?
    };
    let a627 = 9223372036854775807u64;
    if nat_le(a626, a627) {
        if f30(v1.clone()) {
            let a629 = match v1 {
                Adt1::C0(_, h628, _, _, _, _, _, _, _, _, _) => {
                    h628
                }
            };
            let a630 = 2147483647u64;
            Ok(nat_le(a629, a630))
        } else {
            let a632 = match v0 {
                Adt1::C0(_, h631, _, _, _, _, _, _, _, _, _) => {
                    h631
                }
            };
            let a633 = 2147483647u64;
            Ok(nat_le(a632, a633))
        }
    } else {
        Ok(false)
    }
}

pub fn f21(v0: Adt1, v1: Adt1) -> R<bool> {
    if f34(v0.clone(), v1.clone())? {
        Ok(true)
    } else {
        Ok(f46(v0, v1))
    }
}

pub fn f22(v0: Adt1) -> R<bool> {
    f47({
        let a636 = match v0.clone() {
            Adt1::C0(_, h634, _, _, _, _, _, _, _, _, _) => {
                h634
            }
        };
        let a637 = f18(match v0.clone() {
            Adt1::C0(h635, _, _, _, _, _, _, _, _, _, _) => {
                h635
            }
        });
        nat_mul(a636, a637)?
    }, match v0.clone() {
        Adt1::C0(_, _, h638, _, _, _, _, _, _, _, _) => {
            h638
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h639, _, _, _, _, _, _, _) => {
            h639
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h640, _, _, _, _, _, _) => {
            h640
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h641, _, _, _, _) => {
            h641
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h642, _, _, _) => {
            h642
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h643, _, _) => {
            h643
        }
    })
}

pub fn f23(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a650 = {
        let a644 = v0;
        let a645 = v2;
        nat_mul(a644, a645)?
    };
    let a651 = {
        let a646 = v1;
        let a647 = 0u64;
        if nat_eq(a646, a647) {
            0u64
        } else {
            let a648 = v0;
            let a649 = 2u64;
            nat_mul(a648, a649)?
        }
    };
    nat_add(a650, a651)
}

pub fn f24(v0: Adt1) -> R<u64> {
    let a660 = {
        let a654 = match v0.clone() {
            Adt1::C0(_, h652, _, _, _, _, _, _, _, _, _) => {
                h652
            }
        };
        let a655 = match v0.clone() {
            Adt1::C0(_, _, h653, _, _, _, _, _, _, _, _) => {
                h653
            }
        };
        nat_mul(a654, a655)?
    };
    let a661 = {
        let a658 = match v0.clone() {
            Adt1::C0(_, _, _, h656, _, _, _, _, _, _, _) => {
                h656
            }
        };
        let a659 = match v0 {
            Adt1::C0(_, _, _, _, h657, _, _, _, _, _, _) => {
                h657
            }
        };
        nat_mul(a658, a659)?
    };
    nat_mul(a660, a661)
}

pub fn f25(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a672 = {
        let a668 = {
            let a664 = 1u64;
            let a665 = {
                let a662 = v0;
                let a663 = 0u64;
                if nat_eq(a662, a663) {
                    0u64
                } else {
                    1u64
                }
            };
            nat_add(a664, a665)?
        };
        let a669 = {
            let a666 = v1;
            let a667 = 0u64;
            if nat_eq(a666, a667) {
                0u64
            } else {
                1u64
            }
        };
        nat_add(a668, a669)?
    };
    let a673 = {
        let a670 = v2;
        let a671 = 0u64;
        if nat_eq(a670, a671) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a672, a673)
}

pub fn f26(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f14(v0.clone(), v1.clone()) {
        let a676 = match v0.clone() {
            Adt1::C0(h674, _, _, _, _, _, _, _, _, _, _) => {
                h674
            }
        };
        let a677 = match v1.clone() {
            Adt1::C0(h675, _, _, _, _, _, _, _, _, _, _) => {
                h675
            }
        };
        nat_eq(a676, a677)
    } else {
        false
    } {
        let a680 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h678, _, _, _, _, _) => {
                h678
            }
        };
        let a681 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h679, _, _, _, _, _) => {
                h679
            }
        };
        nat_eq(a680, a681)
    } else {
        false
    } {
        let a684 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h682, _, _, _, _) => {
                h682
            }
        };
        let a685 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h683, _, _, _, _) => {
                h683
            }
        };
        nat_eq(a684, a685)
    } else {
        false
    } {
        let a688 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h686, _, _, _) => {
                h686
            }
        };
        let a689 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h687, _, _, _) => {
                h687
            }
        };
        nat_eq(a688, a689)
    } else {
        false
    } {
        let a692 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h690, _, _) => {
                h690
            }
        };
        let a693 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h691, _, _) => {
                h691
            }
        };
        nat_eq(a692, a693)
    } else {
        false
    } {
        let a695 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h694) => {
                h694
            }
        };
        let a696 = 0u64;
        if if nat_eq(a695, a696) {
            true
        } else {
            let a698 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h697) => {
                    h697
                }
            };
            let a699 = 0u64;
            nat_eq(a698, a699)
        } {
            true
        } else {
            let a702 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h700) => {
                    h700
                }
            };
            let a703 = match v1 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h701) => {
                    h701
                }
            };
            nat_eq(a702, a703)
        }
    } else {
        false
    }
}

pub fn f27(v0: u64) -> u64 {
    if f32(v0) {
        let m704 = f48(v0);
        match m704 {
            Adt4::C0 => {
                0u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, _, h705) => {
                        h705
                    }
                }
            }
        }
    } else {
        let a706 = v0;
        let a707 = 43u64;
        if nat_eq(a706, a707) {
            1u64
        } else {
            f18(v0)
        }
    }
}

pub fn f28(v0: u64) -> u64 {
    let a708 = v0;
    let a709 = 26u64;
    if nat_eq(a708, a709) {
        4u64
    } else {
        let a710 = v0;
        let a711 = 27u64;
        if nat_eq(a710, a711) {
            8u64
        } else {
            0u64
        }
    }
}

pub fn f29(v0: u64) -> u64 {
    if f32(v0) {
        let m712 = f48(v0);
        match m712 {
            Adt4::C0 => {
                1u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, h713, _) => {
                        h713
                    }
                }
            }
        }
    } else {
        1u64
    }
}

pub fn f30(v0: Adt1) -> bool {
    let a715 = match v0.clone() {
        Adt1::C0(_, h714, _, _, _, _, _, _, _, _, _) => {
            h714
        }
    };
    let a716 = 1u64;
    if nat_eq(a715, a716) {
        true
    } else {
        let a719 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h717, _, _, _, _, _) => {
                h717
            }
        };
        let a720 = f18(match v0 {
            Adt1::C0(h718, _, _, _, _, _, _, _, _, _, _) => {
                h718
            }
        });
        nat_eq(a719, a720)
    }
}

pub fn f31(v0: Adt1, v1: u64) -> R<u64> {
    let a726 = {
        let a722 = match v0.clone() {
            Adt1::C0(_, h721, _, _, _, _, _, _, _, _, _) => {
                h721
            }
        };
        let a723 = f29(v1);
        let a724 = 0u64;
        nat_quot(a722, a723, a724)
    };
    let a727 = f49(match v0 {
        Adt1::C0(h725, _, _, _, _, _, _, _, _, _, _) => {
            h725
        }
    }, v1);
    nat_mul(a726, a727)
}

pub fn f32(v0: u64) -> bool {
    let a728 = v0;
    let a729 = 2u64;
    if if if if nat_eq(a728, a729) {
        true
    } else {
        let a730 = v0;
        let a731 = 3u64;
        nat_eq(a730, a731)
    } {
        true
    } else {
        let a732 = v0;
        let a733 = 6u64;
        nat_eq(a732, a733)
    } {
        true
    } else {
        let a734 = v0;
        let a735 = 7u64;
        if nat_eq(a734, a735) {
            true
        } else {
            let a736 = v0;
            let a737 = 8u64;
            nat_eq(a736, a737)
        }
    } {
        true
    } else {
        let a738 = v0;
        let a739 = 41u64;
        if nat_eq(a738, a739) {
            true
        } else {
            let a740 = v0;
            let a741 = 42u64;
            nat_eq(a740, a741)
        }
    }
}

pub fn f33(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a743 = match v0.clone() {
            Adt1::C0(_, h742, _, _, _, _, _, _, _, _, _) => {
                h742
            }
        };
        let a744 = 4u64;
        nat_mul(a743, a744)?
    };
    let v2: u64 = {
        let a747 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h745, _, _, _, _) => {
                h745
            }
        };
        let a748 = match v0.clone() {
            Adt1::C0(_, _, h746, _, _, _, _, _, _, _, _) => {
                h746
            }
        };
        nat_mul(a747, a748)?
    };
    let v3: u64 = {
        let a750 = v2;
        let a751 = match v0.clone() {
            Adt1::C0(_, _, _, h749, _, _, _, _, _, _, _) => {
                h749
            }
        };
        nat_mul(a750, a751)?
    };
    let a753 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h752, _, _, _, _, _) => {
            h752
        }
    };
    let a754 = 4u64;
    if if if if nat_eq(a753, a754) {
        let a759 = {
            let a756 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h755) => {
                    h755
                }
            };
            let a757 = 4u64;
            let a758 = 1u64;
            nat_rem(a756, a757, a758)
        };
        let a760 = 0u64;
        nat_eq(a759, a760)
    } else {
        false
    } {
        let a768 = {
            let a766 = match v0.clone() {
                Adt1::C0(_, _, h761, _, _, _, _, _, _, _, _) => {
                    h761
                }
            };
            let a767 = {
                let a764 = match v0.clone() {
                    Adt1::C0(_, _, _, h762, _, _, _, _, _, _, _) => {
                        h762
                    }
                };
                let a765 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h763, _, _, _, _, _, _) => {
                        h763
                    }
                };
                nat_mul(a764, a765)?
            };
            nat_mul(a766, a767)?
        };
        let a769 = 1u64;
        if nat_le(a768, a769) {
            true
        } else {
            let a771 = v1;
            let a772 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h770, _, _, _, _) => {
                    h770
                }
            };
            if nat_le(a771, a772) {
                let a777 = {
                    let a774 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, h773, _, _, _, _) => {
                            h773
                        }
                    };
                    let a775 = 4u64;
                    let a776 = 1u64;
                    nat_rem(a774, a775, a776)
                };
                let a778 = 0u64;
                nat_eq(a777, a778)
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a780 = match v0.clone() {
            Adt1::C0(_, _, _, h779, _, _, _, _, _, _, _) => {
                h779
            }
        };
        let a781 = 1u64;
        if nat_le(a780, a781) {
            true
        } else {
            let a783 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h782, _, _, _) => {
                    h782
                }
            };
            let a784 = v2;
            nat_eq(a783, a784)
        }
    } else {
        false
    } {
        let a786 = match v0.clone() {
            Adt1::C0(_, _, _, _, h785, _, _, _, _, _, _) => {
                h785
            }
        };
        let a787 = 1u64;
        if nat_le(a786, a787) {
            Ok(true)
        } else {
            let a789 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, h788, _, _) => {
                    h788
                }
            };
            let a790 = v3;
            Ok(nat_eq(a789, a790))
        }
    } else {
        Ok(false)
    }
}

pub fn f34(v0: Adt1, v1: Adt1) -> R<bool> {
    let a792 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h791, _) => {
            h791
        }
    };
    let a793 = 0u64;
    if nat_eq(a792, a793) {
        Ok(true)
    } else {
        let a795 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h794, _) => {
                h794
            }
        };
        let a796 = 0u64;
        if nat_eq(a795, a796) {
            Ok(true)
        } else {
            let a798 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h797) => {
                    h797
                }
            };
            let a799 = 0u64;
            if nat_eq(a798, a799) {
                Ok(true)
            } else {
                let a801 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h800) => {
                        h800
                    }
                };
                let a802 = 0u64;
                if nat_eq(a801, a802) {
                    Ok(true)
                } else {
                    let a808 = {
                        let a805 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h803) => {
                                h803
                            }
                        };
                        let a806 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h804, _) => {
                                h804
                            }
                        };
                        nat_add(a805, a806)?
                    };
                    let a809 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h807) => {
                            h807
                        }
                    };
                    if nat_le(a808, a809) {
                        Ok(true)
                    } else {
                        let a815 = {
                            let a812 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h810) => {
                                    h810
                                }
                            };
                            let a813 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h811, _) => {
                                    h811
                                }
                            };
                            nat_add(a812, a813)?
                        };
                        let a816 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h814) => {
                                h814
                            }
                        };
                        Ok(nat_le(a815, a816))
                    }
                }
            }
        }
    }
}

pub fn f35(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a819 = match v0.clone() {
            Adt1::C0(_, _, h817, _, _, _, _, _, _, _, _) => {
                h817
            }
        };
        let a820 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h818, _, _, _, _) => {
                h818
            }
        };
        nat_mul(a819, a820)?
    };
    if if f30(v0.clone()) {
        let a822 = match v0.clone() {
            Adt1::C0(_, _, _, h821, _, _, _, _, _, _, _) => {
                h821
            }
        };
        let a823 = 1u64;
        if nat_eq(a822, a823) {
            true
        } else {
            let a825 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h824, _, _, _) => {
                    h824
                }
            };
            let a826 = v1;
            nat_eq(a825, a826)
        }
    } else {
        false
    } {
        let a828 = match v0.clone() {
            Adt1::C0(_, _, _, _, h827, _, _, _, _, _, _) => {
                h827
            }
        };
        let a829 = 1u64;
        if nat_eq(a828, a829) {
            Ok(true)
        } else {
            let a834 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h830, _, _) => {
                    h830
                }
            };
            let a835 = {
                let a832 = v1;
                let a833 = match v0 {
                    Adt1::C0(_, _, _, h831, _, _, _, _, _, _, _) => {
                        h831
                    }
                };
                nat_mul(a832, a833)?
            };
            Ok(nat_eq(a834, a835))
        }
    } else {
        Ok(false)
    }
}

pub fn f36(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a837 = v0;
        let a838 = match v1.clone() {
            Adt1::C0(_, h836, _, _, _, _, _, _, _, _, _) => {
                h836
            }
        };
        let a839 = 0u64;
        nat_quot(a837, a838, a839)
    };
    let v3: u64 = {
        let a841 = v2;
        let a842 = match v1.clone() {
            Adt1::C0(_, _, h840, _, _, _, _, _, _, _, _) => {
                h840
            }
        };
        let a843 = 0u64;
        nat_quot(a841, a842, a843)
    };
    f37({
        let a845 = v0;
        let a846 = match v1.clone() {
            Adt1::C0(_, h844, _, _, _, _, _, _, _, _, _) => {
                h844
            }
        };
        let a847 = 0u64;
        nat_rem(a845, a846, a847)
    }, {
        let a849 = v2;
        let a850 = match v1.clone() {
            Adt1::C0(_, _, h848, _, _, _, _, _, _, _, _) => {
                h848
            }
        };
        let a851 = 0u64;
        nat_rem(a849, a850, a851)
    }, {
        let a853 = v3;
        let a854 = match v1.clone() {
            Adt1::C0(_, _, _, h852, _, _, _, _, _, _, _) => {
                h852
            }
        };
        let a855 = 0u64;
        nat_rem(a853, a854, a855)
    }, {
        let a857 = v3;
        let a858 = match v1.clone() {
            Adt1::C0(_, _, _, h856, _, _, _, _, _, _, _) => {
                h856
            }
        };
        let a859 = 0u64;
        nat_quot(a857, a858, a859)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h860, _, _, _, _, _) => {
            h860
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h861, _, _, _, _) => {
            h861
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h862, _, _, _) => {
            h862
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h863, _, _) => {
            h863
        }
    })
}

pub fn f37(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a876 = {
        let a872 = {
            let a868 = {
                let a864 = v0;
                let a865 = v4;
                nat_mul(a864, a865)?
            };
            let a869 = {
                let a866 = v1;
                let a867 = v5;
                nat_mul(a866, a867)?
            };
            nat_add(a868, a869)?
        };
        let a873 = {
            let a870 = v2;
            let a871 = v6;
            nat_mul(a870, a871)?
        };
        nat_add(a872, a873)?
    };
    let a877 = {
        let a874 = v3;
        let a875 = v7;
        nat_mul(a874, a875)?
    };
    nat_add(a876, a877)
}

pub fn f38(v0: u64, v1: Adt1, v2: u64, v3: u64) -> R<Adt3> {
    let v4: u64 = {
        let a878 = v0;
        let a879 = 2u64;
        let a880 = 0u64;
        nat_quot(a878, a879, a880)
    };
    let v5: u64 = f28(match v1.clone() {
        Adt1::C0(h881, _, _, _, _, _, _, _, _, _, _) => {
            h881
        }
    });
    let a885 = {
        let a882 = v0;
        let a883 = 2u64;
        let a884 = 0u64;
        nat_rem(a882, a883, a884)
    };
    let a886 = 0u64;
    if nat_eq(a885, a886) {
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 4u64, f36(v4, v1)?, 2u64, 0u64, 0u64, 2u64, v5, 8u64))
    } else {
        let a887 = v3;
        let a888 = v2;
        if if nat_lt(a887, a888) {
            let a891 = v3;
            let a892 = {
                let a889 = v5;
                let a890 = 4u64;
                if nat_eq(a889, a890) {
                    2147483648u64
                } else {
                    9223372036854775808u64
                }
            };
            nat_lt(a891, a892)
        } else {
            false
        } {
            Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, {
                let a893 = 3u64;
                let a894 = v4;
                nat_add(a893, a894)?
            }, 8u64, 8u64))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f39(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64) -> R<Adt3> {
    let v5: u64 = {
        let a896 = 1u64;
        let a897 = f31(v1.clone(), match v3.clone() {
            Adt1::C0(h895, _, _, _, _, _, _, _, _, _, _) => {
                h895
            }
        })?;
        nat_add(a896, a897)?
    };
    let v6: u64 = {
        let a898 = v0;
        let a899 = v5;
        let a900 = 0u64;
        nat_rem(a898, a899, a900)
    };
    let v7: u64 = {
        let a901 = v0;
        let a902 = v5;
        let a903 = 0u64;
        nat_quot(a901, a902, a903)
    };
    let v8: u64 = {
        let a905 = v7;
        let a906 = match v1.clone() {
            Adt1::C0(_, _, h904, _, _, _, _, _, _, _, _) => {
                h904
            }
        };
        let a907 = 0u64;
        nat_rem(a905, a906, a907)
    };
    let v9: u64 = {
        let a909 = v7;
        let a910 = match v1.clone() {
            Adt1::C0(_, _, h908, _, _, _, _, _, _, _, _) => {
                h908
            }
        };
        let a911 = 0u64;
        nat_quot(a909, a910, a911)
    };
    let v10: u64 = {
        let a913 = v9;
        let a914 = match v1.clone() {
            Adt1::C0(_, _, _, h912, _, _, _, _, _, _, _) => {
                h912
            }
        };
        let a915 = 0u64;
        nat_rem(a913, a914, a915)
    };
    let v11: u64 = {
        let a917 = v9;
        let a918 = match v1.clone() {
            Adt1::C0(_, _, _, h916, _, _, _, _, _, _, _) => {
                h916
            }
        };
        let a919 = 0u64;
        nat_quot(a917, a918, a919)
    };
    let a920 = v6;
    let a921 = 0u64;
    if nat_eq(a920, a921) {
        let v12: u64 = {
            let a938 = v8;
            let a939 = {
                let a936 = match v2.clone() {
                    Adt1::C0(_, h922, _, _, _, _, _, _, _, _, _) => {
                        h922
                    }
                };
                let a937 = {
                    let a934 = {
                        let a924 = v10;
                        let a925 = match v2.clone() {
                            Adt1::C0(_, _, h923, _, _, _, _, _, _, _, _) => {
                                h923
                            }
                        };
                        let a926 = 0u64;
                        nat_rem(a924, a925, a926)
                    };
                    let a935 = {
                        let a932 = match v2.clone() {
                            Adt1::C0(_, _, h927, _, _, _, _, _, _, _, _) => {
                                h927
                            }
                        };
                        let a933 = {
                            let a929 = v11;
                            let a930 = match v2 {
                                Adt1::C0(_, _, _, h928, _, _, _, _, _, _, _) => {
                                    h928
                                }
                            };
                            let a931 = 0u64;
                            nat_rem(a929, a930, a931)
                        };
                        nat_mul(a932, a933)?
                    };
                    nat_add(a934, a935)?
                };
                nat_mul(a936, a937)?
            };
            nat_add(a938, a939)?
        };
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, {
            let a940 = 3u64;
            let a941 = v12;
            nat_add(a940, a941)?
        }, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
    } else {
        let a943 = v4;
        let a944 = match v3.clone() {
            Adt1::C0(_, _, h942, _, _, _, _, _, _, _, _) => {
                h942
            }
        };
        if nat_lt(a943, a944) {
            let v13: u64 = {
                let a949 = {
                    let a945 = v6;
                    let a946 = 1u64;
                    nat_sub(a945, a946)
                };
                let a950 = f49(match v1.clone() {
                    Adt1::C0(h947, _, _, _, _, _, _, _, _, _, _) => {
                        h947
                    }
                }, match v3.clone() {
                    Adt1::C0(h948, _, _, _, _, _, _, _, _, _, _) => {
                        h948
                    }
                });
                let a951 = 0u64;
                nat_quot(a949, a950, a951)
            };
            let v14: u64 = f37({
                let a953 = v13;
                let a954 = f29(match v3.clone() {
                    Adt1::C0(h952, _, _, _, _, _, _, _, _, _, _) => {
                        h952
                    }
                });
                nat_mul(a953, a954)?
            }, v8, v10, v11, match v1.clone() {
                Adt1::C0(_, _, _, _, _, h955, _, _, _, _, _) => {
                    h955
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, h956, _, _, _, _) => {
                    h956
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h957, _, _, _) => {
                    h957
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h958, _, _) => {
                    h958
                }
            })?;
            let v15: u64 = f37(v13, v4, v10, v11, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h959, _, _, _, _, _) => {
                    h959
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h960, _, _, _, _) => {
                    h960
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h961, _, _, _) => {
                    h961
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h962, _, _) => {
                    h962
                }
            })?;
            if f32(match v3.clone() {
                Adt1::C0(h963, _, _, _, _, _, _, _, _, _, _) => {
                    h963
                }
            }) {
                Ok(Adt3::C6(v14, v15))
            } else {
                Ok(f50(match v1 {
                    Adt1::C0(h964, _, _, _, _, _, _, _, _, _, _) => {
                        h964
                    }
                }, match v3 {
                    Adt1::C0(h965, _, _, _, _, _, _, _, _, _, _) => {
                        h965
                    }
                }, {
                    let a966 = v6;
                    let a967 = 1u64;
                    nat_sub(a966, a967)
                }, v14, v15))
            }
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f40(v0: u64, v1: u64, v2: u64) -> Adt3 {
    Adt3::C3(2u64, 0u64, 0u64, 0u64, 1u64, v0, v1, v2, 0u64, 2u64, 4u64, 4u64)
}

pub fn f41(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt3 {
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

pub fn f42(v0: u64, v1: Adt1, v2: Adt1) -> R<u64> {
    let v3: u64 = {
        let a969 = v0;
        let a970 = match v1.clone() {
            Adt1::C0(_, h968, _, _, _, _, _, _, _, _, _) => {
                h968
            }
        };
        let a971 = 0u64;
        nat_quot(a969, a970, a971)
    };
    let v4: u64 = {
        let a973 = v3;
        let a974 = match v1.clone() {
            Adt1::C0(_, _, h972, _, _, _, _, _, _, _, _) => {
                h972
            }
        };
        let a975 = 0u64;
        nat_quot(a973, a974, a975)
    };
    f37({
        let a981 = {
            let a977 = v0;
            let a978 = match v1.clone() {
                Adt1::C0(_, h976, _, _, _, _, _, _, _, _, _) => {
                    h976
                }
            };
            let a979 = 0u64;
            nat_rem(a977, a978, a979)
        };
        let a982 = match v2.clone() {
            Adt1::C0(_, h980, _, _, _, _, _, _, _, _, _) => {
                h980
            }
        };
        let a983 = 0u64;
        nat_rem(a981, a982, a983)
    }, {
        let a989 = {
            let a985 = v3;
            let a986 = match v1.clone() {
                Adt1::C0(_, _, h984, _, _, _, _, _, _, _, _) => {
                    h984
                }
            };
            let a987 = 0u64;
            nat_rem(a985, a986, a987)
        };
        let a990 = match v2.clone() {
            Adt1::C0(_, _, h988, _, _, _, _, _, _, _, _) => {
                h988
            }
        };
        let a991 = 0u64;
        nat_rem(a989, a990, a991)
    }, {
        let a997 = {
            let a993 = v4;
            let a994 = match v1.clone() {
                Adt1::C0(_, _, _, h992, _, _, _, _, _, _, _) => {
                    h992
                }
            };
            let a995 = 0u64;
            nat_rem(a993, a994, a995)
        };
        let a998 = match v2.clone() {
            Adt1::C0(_, _, _, h996, _, _, _, _, _, _, _) => {
                h996
            }
        };
        let a999 = 0u64;
        nat_rem(a997, a998, a999)
    }, {
        let a1005 = {
            let a1001 = v4;
            let a1002 = match v1 {
                Adt1::C0(_, _, _, h1000, _, _, _, _, _, _, _) => {
                    h1000
                }
            };
            let a1003 = 0u64;
            nat_quot(a1001, a1002, a1003)
        };
        let a1006 = match v2.clone() {
            Adt1::C0(_, _, _, _, h1004, _, _, _, _, _, _) => {
                h1004
            }
        };
        let a1007 = 0u64;
        nat_rem(a1005, a1006, a1007)
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h1008, _, _, _, _, _) => {
            h1008
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h1009, _, _, _, _) => {
            h1009
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1010, _, _, _) => {
            h1010
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1011, _, _) => {
            h1011
        }
    })
}

pub fn f43(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1012 = v0;
    let a1013 = 0u64;
    if if if nat_eq(a1012, a1013) {
        true
    } else {
        let a1014 = v1;
        let a1015 = 0u64;
        nat_eq(a1014, a1015)
    } {
        true
    } else {
        let a1016 = v2;
        let a1017 = 0u64;
        nat_eq(a1016, a1017)
    } {
        true
    } else {
        let a1018 = v3;
        let a1019 = 0u64;
        nat_eq(a1018, a1019)
    }
}

pub fn f44(v0: u64, v1: u64, v2: u64) -> bool {
    let a1020 = v0;
    let a1021 = 1u64;
    if nat_le(a1020, a1021) {
        true
    } else {
        let a1025 = {
            let a1022 = v1;
            let a1023 = v2;
            let a1024 = 1u64;
            nat_rem(a1022, a1023, a1024)
        };
        let a1026 = 0u64;
        nat_eq(a1025, a1026)
    }
}

pub fn f45(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f44(v0, v3, v6) {
        f44(v1, v4, v6)
    } else {
        false
    } {
        f44(v2, v5, v6)
    } else {
        false
    }
}

pub fn f46(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f14(v0.clone(), v1.clone()) {
        let a1029 = match v0.clone() {
            Adt1::C0(h1027, _, _, _, _, _, _, _, _, _, _) => {
                h1027
            }
        };
        let a1030 = match v1.clone() {
            Adt1::C0(h1028, _, _, _, _, _, _, _, _, _, _) => {
                h1028
            }
        };
        nat_eq(a1029, a1030)
    } else {
        false
    } {
        let a1033 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1031) => {
                h1031
            }
        };
        let a1034 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1032) => {
                h1032
            }
        };
        nat_eq(a1033, a1034)
    } else {
        false
    } {
        let a1037 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1035, _, _, _, _, _) => {
                h1035
            }
        };
        let a1038 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1036, _, _, _, _, _) => {
                h1036
            }
        };
        nat_eq(a1037, a1038)
    } else {
        false
    } {
        let a1041 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1039, _, _, _, _) => {
                h1039
            }
        };
        let a1042 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1040, _, _, _, _) => {
                h1040
            }
        };
        nat_eq(a1041, a1042)
    } else {
        false
    } {
        let a1045 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1043, _, _, _) => {
                h1043
            }
        };
        let a1046 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1044, _, _, _) => {
                h1044
            }
        };
        nat_eq(a1045, a1046)
    } else {
        false
    } {
        let a1049 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1047, _, _) => {
                h1047
            }
        };
        let a1050 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1048, _, _) => {
                h1048
            }
        };
        nat_eq(a1049, a1050)
    } else {
        false
    }
}

pub fn f47(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f51(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f51(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f51(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f51(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f51(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f51(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f48(v0: u64) -> Adt4 {
    let a1051 = v0;
    let a1052 = 0u64;
    if nat_eq(a1051, a1052) {
        Adt4::C1(Adt5::C0(0u64, 1u64, 4u64))
    } else {
        let a1053 = v0;
        let a1054 = 1u64;
        if nat_eq(a1053, a1054) {
            Adt4::C1(Adt5::C0(1u64, 1u64, 2u64))
        } else {
            let a1055 = v0;
            let a1056 = 2u64;
            if nat_eq(a1055, a1056) {
                Adt4::C1(Adt5::C0(2u64, 32u64, 18u64))
            } else {
                let a1057 = v0;
                let a1058 = 3u64;
                if nat_eq(a1057, a1058) {
                    Adt4::C1(Adt5::C0(3u64, 32u64, 20u64))
                } else {
                    let a1059 = v0;
                    let a1060 = 6u64;
                    if nat_eq(a1059, a1060) {
                        Adt4::C1(Adt5::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1061 = v0;
                        let a1062 = 7u64;
                        if nat_eq(a1061, a1062) {
                            Adt4::C1(Adt5::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1063 = v0;
                            let a1064 = 8u64;
                            if nat_eq(a1063, a1064) {
                                Adt4::C1(Adt5::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1065 = v0;
                                let a1066 = 9u64;
                                if nat_eq(a1065, a1066) {
                                    Adt4::C1(Adt5::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1067 = v0;
                                    let a1068 = 10u64;
                                    if nat_eq(a1067, a1068) {
                                        Adt4::C1(Adt5::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1069 = v0;
                                        let a1070 = 11u64;
                                        if nat_eq(a1069, a1070) {
                                            Adt4::C1(Adt5::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1071 = v0;
                                            let a1072 = 12u64;
                                            if nat_eq(a1071, a1072) {
                                                Adt4::C1(Adt5::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1073 = v0;
                                                let a1074 = 13u64;
                                                if nat_eq(a1073, a1074) {
                                                    Adt4::C1(Adt5::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1075 = v0;
                                                    let a1076 = 14u64;
                                                    if nat_eq(a1075, a1076) {
                                                        Adt4::C1(Adt5::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1077 = v0;
                                                        let a1078 = 15u64;
                                                        if nat_eq(a1077, a1078) {
                                                            Adt4::C1(Adt5::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1079 = v0;
                                                            let a1080 = 16u64;
                                                            if nat_eq(a1079, a1080) {
                                                                Adt4::C1(Adt5::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1081 = v0;
                                                                let a1082 = 17u64;
                                                                if nat_eq(a1081, a1082) {
                                                                    Adt4::C1(Adt5::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1083 = v0;
                                                                    let a1084 = 18u64;
                                                                    if nat_eq(a1083, a1084) {
                                                                        Adt4::C1(Adt5::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1085 = v0;
                                                                        let a1086 = 19u64;
                                                                        if nat_eq(a1085, a1086) {
                                                                            Adt4::C1(Adt5::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1087 = v0;
                                                                            let a1088 = 20u64;
                                                                            if nat_eq(a1087, a1088) {
                                                                                Adt4::C1(Adt5::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1089 = v0;
                                                                                let a1090 = 21u64;
                                                                                if nat_eq(a1089, a1090) {
                                                                                    Adt4::C1(Adt5::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1091 = v0;
                                                                                    let a1092 = 22u64;
                                                                                    if nat_eq(a1091, a1092) {
                                                                                        Adt4::C1(Adt5::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1093 = v0;
                                                                                        let a1094 = 23u64;
                                                                                        if nat_eq(a1093, a1094) {
                                                                                            Adt4::C1(Adt5::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1095 = v0;
                                                                                            let a1096 = 24u64;
                                                                                            if nat_eq(a1095, a1096) {
                                                                                                Adt4::C1(Adt5::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1097 = v0;
                                                                                                let a1098 = 25u64;
                                                                                                if nat_eq(a1097, a1098) {
                                                                                                    Adt4::C1(Adt5::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1099 = v0;
                                                                                                    let a1100 = 26u64;
                                                                                                    if nat_eq(a1099, a1100) {
                                                                                                        Adt4::C1(Adt5::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1101 = v0;
                                                                                                        let a1102 = 27u64;
                                                                                                        if nat_eq(a1101, a1102) {
                                                                                                            Adt4::C1(Adt5::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1103 = v0;
                                                                                                            let a1104 = 28u64;
                                                                                                            if nat_eq(a1103, a1104) {
                                                                                                                Adt4::C1(Adt5::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1105 = v0;
                                                                                                                let a1106 = 29u64;
                                                                                                                if nat_eq(a1105, a1106) {
                                                                                                                    Adt4::C1(Adt5::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1107 = v0;
                                                                                                                    let a1108 = 30u64;
                                                                                                                    if nat_eq(a1107, a1108) {
                                                                                                                        Adt4::C1(Adt5::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1109 = v0;
                                                                                                                        let a1110 = 34u64;
                                                                                                                        if nat_eq(a1109, a1110) {
                                                                                                                            Adt4::C1(Adt5::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1111 = v0;
                                                                                                                            let a1112 = 35u64;
                                                                                                                            if nat_eq(a1111, a1112) {
                                                                                                                                Adt4::C1(Adt5::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1113 = v0;
                                                                                                                                let a1114 = 39u64;
                                                                                                                                if nat_eq(a1113, a1114) {
                                                                                                                                    Adt4::C1(Adt5::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1115 = v0;
                                                                                                                                    let a1116 = 40u64;
                                                                                                                                    if nat_eq(a1115, a1116) {
                                                                                                                                        Adt4::C1(Adt5::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1117 = v0;
                                                                                                                                        let a1118 = 41u64;
                                                                                                                                        if nat_eq(a1117, a1118) {
                                                                                                                                            Adt4::C1(Adt5::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1119 = v0;
                                                                                                                                            let a1120 = 42u64;
                                                                                                                                            if nat_eq(a1119, a1120) {
                                                                                                                                                Adt4::C1(Adt5::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1121 = v0;
                                                                                                                                                let a1122 = 43u64;
                                                                                                                                                if nat_eq(a1121, a1122) {
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

pub fn f49(v0: u64, v1: u64) -> u64 {
    let a1123 = v0;
    let a1124 = 1u64;
    if if nat_eq(a1123, a1124) {
        let a1125 = v1;
        let a1126 = 43u64;
        nat_eq(a1125, a1126)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f50(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt3 {
    let a1127 = f49(v0, v1);
    let a1128 = 2u64;
    if nat_eq(a1127, a1128) {
        let a1132 = {
            let a1129 = v2;
            let a1130 = 2u64;
            let a1131 = 0u64;
            nat_rem(a1129, a1130, a1131)
        };
        let a1133 = 0u64;
        if nat_eq(a1132, a1133) {
            Adt3::C3(0u64, 0u64, v0, 0u64, 0u64, v3, 2u64, 0u64, 1u64, 0u64, f18(v0), 8u64)
        } else {
            Adt3::C3(0u64, 0u64, 0u64, v1, 1u64, 0u64, 2u64, 0u64, 3u64, v4, 8u64, f27(v1))
        }
    } else {
        Adt3::C3(0u64, 0u64, v0, v1, 0u64, v3, 2u64, 0u64, 3u64, v4, f18(v0), f27(v1))
    }
}

pub fn f51(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a1134 = v1;
    let a1135 = 1u64;
    if if if nat_le(a1134, a1135) {
        true
    } else {
        let a1136 = v0;
        let a1137 = v2;
        nat_le(a1136, a1137)
    } {
        let a1138 = v3;
        let a1139 = 1u64;
        if nat_le(a1138, a1139) {
            true
        } else {
            let a1146 = {
                let a1144 = v0;
                let a1145 = {
                    let a1142 = {
                        let a1140 = v1;
                        let a1141 = 1u64;
                        nat_sub(a1140, a1141)
                    };
                    let a1143 = v2;
                    nat_mul(a1142, a1143)?
                };
                nat_add(a1144, a1145)?
            };
            let a1147 = v4;
            nat_le(a1146, a1147)
        }
    } else {
        false
    } {
        let a1148 = v5;
        let a1149 = 1u64;
        if nat_le(a1148, a1149) {
            Ok(true)
        } else {
            let a1162 = {
                let a1160 = {
                    let a1154 = v0;
                    let a1155 = {
                        let a1152 = {
                            let a1150 = v1;
                            let a1151 = 1u64;
                            nat_sub(a1150, a1151)
                        };
                        let a1153 = v2;
                        nat_mul(a1152, a1153)?
                    };
                    nat_add(a1154, a1155)?
                };
                let a1161 = {
                    let a1158 = {
                        let a1156 = v3;
                        let a1157 = 1u64;
                        nat_sub(a1156, a1157)
                    };
                    let a1159 = v4;
                    nat_mul(a1158, a1159)?
                };
                nat_add(a1160, a1161)?
            };
            let a1163 = v6;
            Ok(nat_le(a1162, a1163))
        }
    } else {
        Ok(false)
    }
}
