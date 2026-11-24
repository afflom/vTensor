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
    C3(u64, u64, u64, u64, u64, bool, Adt1, Option<Adt1>, Adt1),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64, u64, u64, u64, u64, u64, Adt1, Adt1, Adt1),
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
        Adt0::C1(v26, v27, v28) => {
            let a19 = match v27.clone() {
                Adt2::C0(_, _, h18, _, _, _, _, _, _) => {
                    h18
                }
            };
            let a20 = 33u64;
            if nat_eq(a19, a20) {
                f8(v26, v27)
            } else {
                let a22 = match v27.clone() {
                    Adt2::C0(_, _, h21, _, _, _, _, _, _) => {
                        h21
                    }
                };
                let a23 = 66u64;
                if if nat_eq(a22, a23) {
                    true
                } else {
                    let a25 = match v27.clone() {
                        Adt2::C0(_, _, h24, _, _, _, _, _, _) => {
                            h24
                        }
                    };
                    let a26 = 67u64;
                    nat_eq(a25, a26)
                } {
                    f9(v26, v27)
                } else {
                    let a28 = match v27.clone() {
                        Adt2::C0(_, _, h27, _, _, _, _, _, _) => {
                            h27
                        }
                    };
                    let a29 = 41u64;
                    if nat_eq(a28, a29) {
                        f10(v26, v27, v28)
                    } else {
                        let a31 = 98u64;
                        let a32 = match v27.clone() {
                            Adt2::C0(_, _, h30, _, _, _, _, _, _) => {
                                h30
                            }
                        };
                        if nat_le(a31, a32) {
                            if f11(match v27.clone() {
                                Adt2::C0(_, _, _, h33, _, _, _, _, _) => {
                                    h33
                                }
                            }) {
                                f12(v26, v27, v28)
                            } else {
                                f13(v26, v27)
                            }
                        } else {
                            f14(v26, v27, v28)
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
        Adt0::C3(v14, v15, v16, v17, v18, v19, v20, v21, v22) => {
            let a7 = v14;
            let a8 = 33u64;
            if nat_eq(a7, a8) {
                if v19 {
                    Ok(Adt3::C1)
                } else {
                    let m9 = v21;
                    match m9 {
                        None => {
                            Ok(Adt3::C1)
                        }
                        Some(v23) => {
                            f4(v20, v23, v22)
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
                    if v19 {
                        Ok(Adt3::C1)
                    } else {
                        let m14 = v21;
                        match m14 {
                            None => {
                                f5(v14, v15, v20, v22)
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
                            !v19
                        } else {
                            false
                        } {
                            let m17 = v21;
                            match m17 {
                                None => {
                                    f7(v15, false, v16, v17, v18, v20.clone(), v20, v22)
                                }
                                Some(v25) => {
                                    f7(v15, true, v16, v17, v18, v20, v25, v22)
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
    let a34 = v0;
    let a35 = 2u64;
    if nat_eq(a34, a35) {
        Some(0u64)
    } else {
        let a36 = v0;
        let a37 = 5u64;
        if nat_eq(a36, a37) {
            Some(1u64)
        } else {
            let a38 = v0;
            let a39 = 6u64;
            if nat_eq(a38, a39) {
                Some(2u64)
            } else {
                let a40 = v0;
                let a41 = 7u64;
                if nat_eq(a40, a41) {
                    Some(3u64)
                } else {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f2(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if f15(v1.clone(), v3.clone()) {
        f16(v0, v1.clone(), v2.clone(), v3.clone())?
    } else {
        false
    } {
        if f17(v1.clone()) {
            true
        } else {
            let a44 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, h42, _, _, _, _, _) => {
                    h42
                }
            };
            let a45 = f18(match v1.clone() {
                Adt1::C0(h43, _, _, _, _, _, _, _, _, _, _) => {
                    h43
                }
            });
            if if if if if if if nat_eq(a44, a45) {
                let a48 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h46, _, _, _, _, _) => {
                        h46
                    }
                };
                let a49 = f18(match v3.clone() {
                    Adt1::C0(h47, _, _, _, _, _, _, _, _, _, _) => {
                        h47
                    }
                });
                nat_eq(a48, a49)
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
            let a52 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h50, _) => {
                    h50
                }
            };
            let a53 = f18(match v3.clone() {
                Adt1::C0(h51, _, _, _, _, _, _, _, _, _, _) => {
                    h51
                }
            });
            let a54 = 0u64;
            nat_quot(a52, a53, a54)
        };
        Ok(Adt3::C2(Adt2::C0(f23(f24(v1.clone())?, v4, f25(match v1.clone() {
            Adt1::C0(h55, _, _, _, _, _, _, _, _, _, _) => {
                h55
            }
        }, match v2.clone() {
            Adt1::C0(h56, _, _, _, _, _, _, _, _, _, _) => {
                h56
            }
        }, match v3.clone() {
            Adt1::C0(h57, _, _, _, _, _, _, _, _, _, _) => {
                h57
            }
        })?)?, {
            let a58 = 3u64;
            let a59 = v4;
            nat_add(a58, a59)?
        }, v0, 0u64, 0u64, 0u64, v1, v2, v3)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f3(v0: Adt1, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if if if if if if if if if if if if if f26(v2, v3.clone()) {
        let a61 = match v0.clone() {
            Adt1::C0(h60, _, _, _, _, _, _, _, _, _, _) => {
                h60
            }
        };
        let a62 = 0u64;
        if nat_eq(a61, a62) {
            true
        } else {
            let a64 = match v0.clone() {
                Adt1::C0(h63, _, _, _, _, _, _, _, _, _, _) => {
                    h63
                }
            };
            let a65 = 1u64;
            nat_eq(a64, a65)
        }
    } else {
        false
    } {
        let a67 = 0u64;
        let a68 = f27(match v3.clone() {
            Adt1::C0(h66, _, _, _, _, _, _, _, _, _, _) => {
                h66
            }
        });
        nat_lt(a67, a68)
    } else {
        false
    } {
        let a70 = 0u64;
        let a71 = f28(match v1.clone() {
            Adt1::C0(h69, _, _, _, _, _, _, _, _, _, _) => {
                h69
            }
        });
        nat_lt(a70, a71)
    } else {
        false
    } {
        let a74 = match v0.clone() {
            Adt1::C0(_, h72, _, _, _, _, _, _, _, _, _) => {
                h72
            }
        };
        let a75 = match v3.clone() {
            Adt1::C0(_, h73, _, _, _, _, _, _, _, _, _) => {
                h73
            }
        };
        if nat_eq(a74, a75) {
            let a81 = {
                let a78 = match v0.clone() {
                    Adt1::C0(_, h76, _, _, _, _, _, _, _, _, _) => {
                        h76
                    }
                };
                let a79 = f29(match v3.clone() {
                    Adt1::C0(h77, _, _, _, _, _, _, _, _, _, _) => {
                        h77
                    }
                });
                let a80 = 1u64;
                nat_rem(a78, a79, a80)
            };
            let a82 = 0u64;
            nat_eq(a81, a82)
        } else {
            false
        }
    } else {
        false
    } {
        let a85 = match v0.clone() {
            Adt1::C0(_, _, _, h83, _, _, _, _, _, _, _) => {
                h83
            }
        };
        let a86 = match v3.clone() {
            Adt1::C0(_, _, _, h84, _, _, _, _, _, _, _) => {
                h84
            }
        };
        nat_eq(a85, a86)
    } else {
        false
    } {
        let a89 = match v0.clone() {
            Adt1::C0(_, _, _, _, h87, _, _, _, _, _, _) => {
                h87
            }
        };
        let a90 = match v3.clone() {
            Adt1::C0(_, _, _, _, h88, _, _, _, _, _, _) => {
                h88
            }
        };
        nat_eq(a89, a90)
    } else {
        false
    } {
        let a93 = match v0.clone() {
            Adt1::C0(_, _, h91, _, _, _, _, _, _, _, _) => {
                h91
            }
        };
        let a94 = match v1.clone() {
            Adt1::C0(_, h92, _, _, _, _, _, _, _, _, _) => {
                h92
            }
        };
        nat_eq(a93, a94)
    } else {
        false
    } {
        let a96 = match v1.clone() {
            Adt1::C0(_, _, _, _, h95, _, _, _, _, _, _) => {
                h95
            }
        };
        let a97 = 1u64;
        nat_eq(a96, a97)
    } else {
        false
    } {
        let a99 = 0u64;
        let a100 = match v1.clone() {
            Adt1::C0(_, _, h98, _, _, _, _, _, _, _, _) => {
                h98
            }
        };
        nat_lt(a99, a100)
    } else {
        false
    } {
        let a102 = 0u64;
        let a103 = match v1.clone() {
            Adt1::C0(_, _, _, h101, _, _, _, _, _, _, _) => {
                h101
            }
        };
        nat_lt(a102, a103)
    } else {
        false
    } {
        let a109 = {
            let a106 = match v0.clone() {
                Adt1::C0(_, _, _, h104, _, _, _, _, _, _, _) => {
                    h104
                }
            };
            let a107 = match v1.clone() {
                Adt1::C0(_, _, h105, _, _, _, _, _, _, _, _) => {
                    h105
                }
            };
            let a108 = 0u64;
            nat_rem(a106, a107, a108)
        };
        let a110 = 0u64;
        nat_eq(a109, a110)
    } else {
        false
    } {
        let a116 = {
            let a113 = match v0.clone() {
                Adt1::C0(_, _, _, _, h111, _, _, _, _, _, _) => {
                    h111
                }
            };
            let a114 = match v1.clone() {
                Adt1::C0(_, _, _, h112, _, _, _, _, _, _, _) => {
                    h112
                }
            };
            let a115 = 0u64;
            nat_rem(a113, a114, a115)
        };
        let a117 = 0u64;
        nat_eq(a116, a117)
    } else {
        false
    } {
        f30(v0.clone())
    } else {
        false
    } {
        let a119 = match v3.clone() {
            Adt1::C0(_, h118, _, _, _, _, _, _, _, _, _) => {
                h118
            }
        };
        let a120 = 1u64;
        if nat_eq(a119, a120) {
            true
        } else {
            let a123 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h121, _, _, _, _, _) => {
                    h121
                }
            };
            let a124 = f27(match v3.clone() {
                Adt1::C0(h122, _, _, _, _, _, _, _, _, _, _) => {
                    h122
                }
            });
            nat_eq(a123, a124)
        }
    } else {
        false
    } {
        let v4: u64 = f24(v1.clone())?;
        let v5: u64 = {
            let a130 = match v0.clone() {
                Adt1::C0(_, _, h125, _, _, _, _, _, _, _, _) => {
                    h125
                }
            };
            let a131 = {
                let a128 = match v0.clone() {
                    Adt1::C0(_, _, _, h126, _, _, _, _, _, _, _) => {
                        h126
                    }
                };
                let a129 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h127, _, _, _, _, _, _) => {
                        h127
                    }
                };
                nat_mul(a128, a129)?
            };
            nat_mul(a130, a131)?
        };
        let v6: u64 = if f17(v0.clone()) {
            0u64
        } else {
            let a139 = {
                let a132 = 2u64;
                let a133 = v4;
                nat_mul(a132, a133)?
            };
            let a140 = {
                let a137 = v5;
                let a138 = {
                    let a135 = 1u64;
                    let a136 = f31(v0.clone(), match v3.clone() {
                        Adt1::C0(h134, _, _, _, _, _, _, _, _, _, _) => {
                            h134
                        }
                    })?;
                    nat_add(a135, a136)?
                };
                nat_mul(a137, a138)?
            };
            nat_add(a139, a140)?
        };
        Ok(Adt3::C2(Adt2::C0(v6, if f17(v0.clone()) {
            3u64
        } else {
            let a141 = 3u64;
            let a142 = v4;
            nat_add(a141, a142)?
        }, 41u64, 0u64, 0u64, 0u64, v0, v1, v3)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f4(v0: Adt1, v1: Adt1, v2: Adt1) -> R<Adt3> {
    let v3: u64 = f29(match v2.clone() {
        Adt1::C0(h143, _, _, _, _, _, _, _, _, _, _) => {
            h143
        }
    });
    if if if if if if if f32(match v2.clone() {
        Adt1::C0(h144, _, _, _, _, _, _, _, _, _, _) => {
            h144
        }
    }) {
        f26(v1.clone(), v2.clone())
    } else {
        false
    } {
        let a146 = 0u64;
        let a147 = f33(match v0.clone() {
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
        let a151 = f33(match v0.clone() {
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
        }, 3u64, 33u64, 0u64, 0u64, 0u64, v0, v1, v2)))
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
        f34(v2.clone())?
    } else {
        false
    } {
        f34(v3.clone())?
    } else {
        false
    } {
        f35(v2.clone(), v3.clone())?
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
        }, 0u64, 0u64, v2.clone(), v2, v3)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f6(v0: u64) -> bool {
    let a215 = v0;
    let a216 = 6u64;
    nat_le(a215, a216)
}

pub fn f7(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1) -> R<Adt3> {
    let a217 = v2;
    let a218 = 1u64;
    if if if if if if if if if if if if if if if nat_le(a217, a218) {
        let a220 = 0u64;
        let a221 = f33(match v5.clone() {
            Adt1::C0(h219, _, _, _, _, _, _, _, _, _, _) => {
                h219
            }
        });
        nat_lt(a220, a221)
    } else {
        false
    } {
        let a224 = match v5.clone() {
            Adt1::C0(h222, _, _, _, _, _, _, _, _, _, _) => {
                h222
            }
        };
        let a225 = match v6.clone() {
            Adt1::C0(h223, _, _, _, _, _, _, _, _, _, _) => {
                h223
            }
        };
        nat_eq(a224, a225)
    } else {
        false
    } {
        let a228 = match v5.clone() {
            Adt1::C0(h226, _, _, _, _, _, _, _, _, _, _) => {
                h226
            }
        };
        let a229 = match v7.clone() {
            Adt1::C0(h227, _, _, _, _, _, _, _, _, _, _) => {
                h227
            }
        };
        nat_eq(a228, a229)
    } else {
        false
    } {
        f15(v5.clone(), v6.clone())
    } else {
        false
    } {
        f36(v5.clone())?
    } else {
        false
    } {
        f36(v6.clone())?
    } else {
        false
    } {
        f36(v7.clone())?
    } else {
        false
    } {
        let a232 = match v5.clone() {
            Adt1::C0(_, _, h230, _, _, _, _, _, _, _, _) => {
                h230
            }
        };
        let a233 = match v7.clone() {
            Adt1::C0(_, _, h231, _, _, _, _, _, _, _, _) => {
                h231
            }
        };
        nat_eq(a232, a233)
    } else {
        false
    } {
        let a236 = match v5.clone() {
            Adt1::C0(_, _, _, h234, _, _, _, _, _, _, _) => {
                h234
            }
        };
        let a237 = match v7.clone() {
            Adt1::C0(_, _, _, h235, _, _, _, _, _, _, _) => {
                h235
            }
        };
        nat_eq(a236, a237)
    } else {
        false
    } {
        let a240 = match v5.clone() {
            Adt1::C0(_, _, _, _, h238, _, _, _, _, _, _) => {
                h238
            }
        };
        let a241 = match v7.clone() {
            Adt1::C0(_, _, _, _, h239, _, _, _, _, _, _) => {
                h239
            }
        };
        nat_eq(a240, a241)
    } else {
        false
    } {
        let a248 = match v7.clone() {
            Adt1::C0(_, h242, _, _, _, _, _, _, _, _, _) => {
                h242
            }
        };
        let a249 = if v1 {
            match v5.clone() {
                Adt1::C0(_, h243, _, _, _, _, _, _, _, _, _) => {
                    h243
                }
            }
        } else {
            let a245 = match v5.clone() {
                Adt1::C0(_, h244, _, _, _, _, _, _, _, _, _) => {
                    h244
                }
            };
            let a246 = 2u64;
            let a247 = 0u64;
            nat_quot(a245, a246, a247)
        };
        nat_eq(a248, a249)
    } else {
        false
    } {
        let a251 = match v7.clone() {
            Adt1::C0(_, h250, _, _, _, _, _, _, _, _, _) => {
                h250
            }
        };
        let a252 = 2147483647u64;
        nat_le(a251, a252)
    } else {
        false
    } {
        f21(v5.clone(), v7.clone())?
    } else {
        false
    } {
        f21(v6.clone(), v7.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0(if f11(v0) {
            let a255 = 1u64;
            let a256 = {
                let a253 = f37(v0);
                let a254 = f24(v7.clone())?;
                nat_mul(a253, a254)?
            };
            nat_add(a255, a256)?
        } else {
            let a257 = 4u64;
            let a258 = f24(v7.clone())?;
            nat_mul(a257, a258)?
        }, if f11(v0) {
            4u64
        } else {
            3u64
        }, if v1 {
            98u64
        } else {
            let a259 = 99u64;
            let a260 = v2;
            nat_add(a259, a260)?
        }, v0, v3, v4, v5, v6, v7)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f8(v0: u64, v1: Adt2) -> R<Adt3> {
    let a262 = v0;
    let a263 = match v1.clone() {
        Adt2::C0(h261, _, _, _, _, _, _, _, _) => {
            h261
        }
    };
    if nat_lt(a262, a263) {
        let v2: Adt1 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h264) => {
                h264
            }
        };
        let v3: u64 = f29(match v2.clone() {
            Adt1::C0(h265, _, _, _, _, _, _, _, _, _, _) => {
                h265
            }
        });
        let v4: u64 = {
            let a267 = match v2.clone() {
                Adt1::C0(_, h266, _, _, _, _, _, _, _, _, _) => {
                    h266
                }
            };
            let a268 = v3;
            let a269 = 0u64;
            nat_quot(a267, a268, a269)
        };
        let v5: u64 = {
            let a270 = v0;
            let a271 = v4;
            let a272 = 0u64;
            nat_quot(a270, a271, a272)
        };
        let v6: u64 = {
            let a274 = v5;
            let a275 = match v2.clone() {
                Adt1::C0(_, _, h273, _, _, _, _, _, _, _, _) => {
                    h273
                }
            };
            let a276 = 0u64;
            nat_quot(a274, a275, a276)
        };
        Ok(Adt3::C6(f38({
            let a277 = v0;
            let a278 = v3;
            nat_mul(a277, a278)?
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, h279, _, _) => {
                h279
            }
        })?, f39({
            let a280 = v0;
            let a281 = v4;
            let a282 = 0u64;
            nat_rem(a280, a281, a282)
        }, {
            let a284 = v5;
            let a285 = match v2.clone() {
                Adt1::C0(_, _, h283, _, _, _, _, _, _, _, _) => {
                    h283
                }
            };
            let a286 = 0u64;
            nat_rem(a284, a285, a286)
        }, {
            let a288 = v6;
            let a289 = match v2.clone() {
                Adt1::C0(_, _, _, h287, _, _, _, _, _, _, _) => {
                    h287
                }
            };
            let a290 = 0u64;
            nat_rem(a288, a289, a290)
        }, {
            let a292 = v6;
            let a293 = match v2.clone() {
                Adt1::C0(_, _, _, h291, _, _, _, _, _, _, _) => {
                    h291
                }
            };
            let a294 = 0u64;
            nat_quot(a292, a293, a294)
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, h295, _, _, _, _, _) => {
                h295
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h296, _, _, _, _) => {
                h296
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h297, _, _, _) => {
                h297
            }
        }, match v2 {
            Adt1::C0(_, _, _, _, _, _, _, _, h298, _, _) => {
                h298
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f9(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h299, _, _) => {
            h299
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h300) => {
            h300
        }
    };
    let a302 = v0;
    let a303 = match v1.clone() {
        Adt2::C0(h301, _, _, _, _, _, _, _, _) => {
            h301
        }
    };
    if nat_lt(a302, a303) {
        Ok(Adt3::C5({
            let a305 = v0;
            let a306 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, _, h304, _, _, _, _) => {
                    h304
                }
            };
            nat_mul(a305, a306)?
        }, {
            let a308 = v0;
            let a309 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h307, _, _, _, _) => {
                    h307
                }
            };
            nat_mul(a308, a309)?
        }, match v2 {
            Adt1::C0(_, h310, _, _, _, _, _, _, _, _, _) => {
                h310
            }
        }, match v3 {
            Adt1::C0(_, h311, _, _, _, _, _, _, _, _, _) => {
                h311
            }
        }, {
            let a313 = match v1 {
                Adt2::C0(_, _, _, h312, _, _, _, _, _) => {
                    h312
                }
            };
            let a314 = 1u64;
            nat_eq(a313, a314)
        }))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f10(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let a316 = v0;
    let a317 = match v1.clone() {
        Adt2::C0(h315, _, _, _, _, _, _, _, _) => {
            h315
        }
    };
    if nat_lt(a316, a317) {
        let v3: u64 = {
            let a319 = 2u64;
            let a320 = f24(match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h318, _) => {
                    h318
                }
            })?;
            nat_mul(a319, a320)?
        };
        let a321 = v0;
        let a322 = v3;
        if nat_lt(a321, a322) {
            f40(v0, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h323, _) => {
                    h323
                }
            }, {
                let m326 = match v1 {
                    Adt2::C0(_, _, _, _, _, _, _, _, h325) => {
                        h325
                    }
                };
                match m326 {
                    Adt1::C0(_, _, h324, _, _, _, _, _, _, _, _) => {
                        h324
                    }
                }
            }, v2)
        } else {
            f41({
                let a327 = v0;
                let a328 = v3;
                nat_sub(a327, a328)
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, h329, _, _) => {
                    h329
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h330, _) => {
                    h330
                }
            }, match v1 {
                Adt2::C0(_, _, _, _, _, _, _, _, h331) => {
                    h331
                }
            }, v2)
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f11(v0: u64) -> bool {
    let a332 = v0;
    let a333 = 3u64;
    if nat_eq(a332, a333) {
        true
    } else {
        let a334 = v0;
        let a335 = 6u64;
        nat_eq(a334, a335)
    }
}

pub fn f12(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, h336, _, _, _) => {
            h336
        }
    };
    let a338 = v0;
    let a339 = match v1.clone() {
        Adt2::C0(h337, _, _, _, _, _, _, _, _) => {
            h337
        }
    };
    if nat_lt(a338, a339) {
        let a340 = v0;
        let a341 = 0u64;
        if nat_eq(a340, a341) {
            Ok(Adt3::C3(1u64, 6u64, 0u64, 0u64, 2u64, v3, 2u64, 0u64, 0u64, 3u64, 4u64, 4u64))
        } else {
            let v4: u64 = {
                let a342 = v0;
                let a343 = 1u64;
                nat_sub(a342, a343)
            };
            let v5: u64 = {
                let a345 = v4;
                let a346 = f37(match v1.clone() {
                    Adt2::C0(_, _, _, h344, _, _, _, _, _) => {
                        h344
                    }
                });
                let a347 = 0u64;
                nat_rem(a345, a346, a347)
            };
            let v6: u64 = {
                let a352 = 4u64;
                let a353 = {
                    let a349 = v4;
                    let a350 = f37(match v1.clone() {
                        Adt2::C0(_, _, _, h348, _, _, _, _, _) => {
                            h348
                        }
                    });
                    let a351 = 0u64;
                    nat_quot(a349, a350, a351)
                };
                nat_mul(a352, a353)?
            };
            let a354 = v5;
            let a355 = 2u64;
            if nat_lt(a354, a355) {
                f13({
                    let a356 = v6;
                    let a357 = v5;
                    nat_add(a356, a357)?
                }, v1)
            } else {
                let a358 = v5;
                let a359 = 2u64;
                if nat_eq(a358, a359) {
                    Ok(f42(0u64, 2u64, v3))
                } else {
                    let a360 = v5;
                    let a361 = 3u64;
                    if nat_eq(a360, a361) {
                        Ok(f43(0u64, 2u64, v3, {
                            let a362 = v2;
                            let a363 = 2u64;
                            nat_eq(a362, a363)
                        }))
                    } else {
                        let a364 = v5;
                        let a365 = 4u64;
                        if nat_eq(a364, a365) {
                            Ok(f42(1u64, 1u64, 3u64))
                        } else {
                            let a366 = v5;
                            let a367 = 5u64;
                            if nat_eq(a366, a367) {
                                Ok(f43(1u64, 1u64, 3u64, {
                                    let a368 = v2;
                                    let a369 = 0u64;
                                    nat_eq(a368, a369)
                                }))
                            } else {
                                let a370 = v5;
                                let a371 = 6u64;
                                if nat_eq(a370, a371) {
                                    Ok(f42(1u64, 2u64, v3))
                                } else {
                                    let a372 = v5;
                                    let a373 = 7u64;
                                    if nat_eq(a372, a373) {
                                        Ok(f43(1u64, 2u64, v3, {
                                            let a374 = v2;
                                            let a375 = 2u64;
                                            nat_eq(a374, a375)
                                        }))
                                    } else {
                                        let a377 = match v1.clone() {
                                            Adt2::C0(_, _, _, h376, _, _, _, _, _) => {
                                                h376
                                            }
                                        };
                                        let a378 = 3u64;
                                        if nat_eq(a377, a378) {
                                            let a379 = v5;
                                            let a380 = 15u64;
                                            if nat_eq(a379, a380) {
                                                f13({
                                                    let a381 = v6;
                                                    let a382 = 3u64;
                                                    nat_add(a381, a382)?
                                                }, v1)
                                            } else {
                                                Ok(f44(v5, match v1 {
                                                    Adt2::C0(_, _, _, _, h383, _, _, _, _) => {
                                                        h383
                                                    }
                                                }))
                                            }
                                        } else {
                                            let a384 = v5;
                                            let a385 = 8u64;
                                            if nat_eq(a384, a385) {
                                                Ok(Adt3::C4(2u64, {
                                                    let m388 = match v1 {
                                                        Adt2::C0(_, _, _, _, _, _, _, _, h387) => {
                                                            h387
                                                        }
                                                    };
                                                    match m388 {
                                                        Adt1::C0(h386, _, _, _, _, _, _, _, _, _, _) => {
                                                            h386
                                                        }
                                                    }
                                                }, 0u64, 1u64, 0u64))
                                            } else {
                                                f13({
                                                    let a389 = v6;
                                                    let a390 = 3u64;
                                                    nat_add(a389, a390)?
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
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f13(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h391, _, _) => {
            h391
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h392, _) => {
            h392
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h393) => {
            h393
        }
    };
    let v5: u64 = match v1.clone() {
        Adt2::C0(_, _, h394, _, _, _, _, _, _) => {
            h394
        }
    };
    let v6: u64 = {
        let a395 = v0;
        let a396 = 4u64;
        let a397 = 0u64;
        nat_rem(a395, a396, a397)
    };
    let v7: u64 = {
        let a398 = v0;
        let a399 = 4u64;
        let a400 = 0u64;
        nat_quot(a398, a399, a400)
    };
    let v8: u64 = {
        let a402 = v7;
        let a403 = match v4.clone() {
            Adt1::C0(_, h401, _, _, _, _, _, _, _, _, _) => {
                h401
            }
        };
        let a404 = 0u64;
        nat_quot(a402, a403, a404)
    };
    let v9: u64 = f33(match v4.clone() {
        Adt1::C0(h405, _, _, _, _, _, _, _, _, _, _) => {
            h405
        }
    });
    let v10: u64 = {
        let a410 = {
            let a407 = v7;
            let a408 = match v4.clone() {
                Adt1::C0(_, h406, _, _, _, _, _, _, _, _, _) => {
                    h406
                }
            };
            let a409 = 0u64;
            nat_rem(a407, a408, a409)
        };
        let a411 = v9;
        nat_mul(a410, a411)?
    };
    let v11: u64 = {
        let a413 = match v4.clone() {
            Adt1::C0(_, h412, _, _, _, _, _, _, _, _, _) => {
                h412
            }
        };
        let a414 = v9;
        nat_mul(a413, a414)?
    };
    let a416 = v0;
    let a417 = match v1.clone() {
        Adt2::C0(h415, _, _, _, _, _, _, _, _) => {
            h415
        }
    };
    if nat_lt(a416, a417) {
        let a418 = v6;
        let a419 = 0u64;
        if nat_eq(a418, a419) {
            Ok(Adt3::C3(0u64, 0u64, match v2.clone() {
                Adt1::C0(h420, _, _, _, _, _, _, _, _, _, _) => {
                    h420
                }
            }, 0u64, 0u64, {
                let a428 = {
                    let a424 = {
                        let a422 = v8;
                        let a423 = match v2 {
                            Adt1::C0(_, _, _, _, _, _, h421, _, _, _, _) => {
                                h421
                            }
                        };
                        nat_mul(a422, a423)?
                    };
                    let a425 = v10;
                    nat_add(a424, a425)?
                };
                let a429 = {
                    let a426 = v5;
                    let a427 = 100u64;
                    if nat_eq(a426, a427) {
                        v11
                    } else {
                        0u64
                    }
                };
                nat_add(a428, a429)?
            }, 2u64, 0u64, 0u64, 0u64, v9, 4u64))
        } else {
            let a430 = v6;
            let a431 = 1u64;
            if nat_eq(a430, a431) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h432, _, _, _, _, _, _, _, _, _, _) => {
                        h432
                    }
                }, 0u64, {
                    let a433 = v5;
                    let a434 = 98u64;
                    if nat_eq(a433, a434) {
                        4u64
                    } else {
                        0u64
                    }
                }, {
                    let a442 = {
                        let a438 = {
                            let a436 = v8;
                            let a437 = match v3 {
                                Adt1::C0(_, _, _, _, _, _, h435, _, _, _, _) => {
                                    h435
                                }
                            };
                            nat_mul(a436, a437)?
                        };
                        let a439 = v10;
                        nat_add(a438, a439)?
                    };
                    let a443 = {
                        let a440 = v5;
                        let a441 = 99u64;
                        if nat_eq(a440, a441) {
                            v11
                        } else {
                            0u64
                        }
                    };
                    nat_add(a442, a443)?
                }, 2u64, 0u64, 0u64, 1u64, v9, 4u64))
            } else {
                let a444 = v6;
                let a445 = 2u64;
                if nat_eq(a444, a445) {
                    Ok(Adt3::C4(match v1 {
                        Adt2::C0(_, _, _, h446, _, _, _, _, _) => {
                            h446
                        }
                    }, match v4 {
                        Adt1::C0(h447, _, _, _, _, _, _, _, _, _, _) => {
                            h447
                        }
                    }, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt3::C3(0u64, 0u64, 0u64, match v4.clone() {
                        Adt1::C0(h448, _, _, _, _, _, _, _, _, _, _) => {
                            h448
                        }
                    }, 1u64, 0u64, 2u64, 0u64, 3u64, {
                        let a452 = {
                            let a450 = v8;
                            let a451 = match v4 {
                                Adt1::C0(_, _, _, _, _, _, h449, _, _, _, _) => {
                                    h449
                                }
                            };
                            nat_mul(a450, a451)?
                        };
                        let a453 = v10;
                        nat_add(a452, a453)?
                    }, 4u64, v9))
                }
            }
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f14(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h454, _, _) => {
            h454
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h455, _) => {
            h455
        }
    };
    let v5: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h456) => {
            h456
        }
    };
    let v6: u64 = {
        let a458 = match v1.clone() {
            Adt2::C0(_, h457, _, _, _, _, _, _, _) => {
                h457
            }
        };
        let a459 = 3u64;
        if nat_eq(a458, a459) {
            0u64
        } else {
            let a460 = f24(v3.clone())?;
            let a461 = 2u64;
            nat_mul(a460, a461)?
        }
    };
    let a463 = match v1.clone() {
        Adt2::C0(h462, _, _, _, _, _, _, _, _) => {
            h462
        }
    };
    let a464 = v0;
    if nat_le(a463, a464) {
        Ok(Adt3::C1)
    } else {
        let a465 = v0;
        let a466 = v6;
        if nat_lt(a465, a466) {
            let v7: u64 = {
                let a467 = v0;
                let a468 = 2u64;
                let a469 = 0u64;
                nat_quot(a467, a468, a469)
            };
            let v8: u64 = {
                let a474 = 3u64;
                let a475 = {
                    let a471 = f38(v7, v5.clone())?;
                    let a472 = f18(match v5 {
                        Adt1::C0(h470, _, _, _, _, _, _, _, _, _, _) => {
                            h470
                        }
                    });
                    let a473 = 0u64;
                    nat_quot(a471, a472, a473)
                };
                nat_add(a474, a475)?
            };
            let a479 = {
                let a476 = v0;
                let a477 = 2u64;
                let a478 = 0u64;
                nat_rem(a476, a477, a478)
            };
            let a480 = 0u64;
            if nat_eq(a479, a480) {
                Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, v8, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
            } else {
                let a481 = v2;
                let a482 = 0u64;
                if nat_eq(a481, a482) {
                    Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 2u64, 1u64, 2u64, 0u64, 0u64, v8, 8u64, 8u64))
                } else {
                    Ok(Adt3::C1)
                }
            }
        } else {
            let v9: u64 = {
                let a483 = v0;
                let a484 = v6;
                nat_sub(a483, a484)
            };
            let v10: u64 = f25(match v3.clone() {
                Adt1::C0(h485, _, _, _, _, _, _, _, _, _, _) => {
                    h485
                }
            }, match v4.clone() {
                Adt1::C0(h486, _, _, _, _, _, _, _, _, _, _) => {
                    h486
                }
            }, match v5.clone() {
                Adt1::C0(h487, _, _, _, _, _, _, _, _, _, _) => {
                    h487
                }
            })?;
            let v11: u64 = {
                let a488 = v9;
                let a489 = v10;
                let a490 = 0u64;
                nat_quot(a488, a489, a490)
            };
            let v12: u64 = {
                let a491 = v9;
                let a492 = v10;
                let a493 = 0u64;
                nat_rem(a491, a492, a493)
            };
            let v13: u64 = {
                let a495 = match v3.clone() {
                    Adt1::C0(h494, _, _, _, _, _, _, _, _, _, _) => {
                        h494
                    }
                };
                let a496 = 0u64;
                if nat_eq(a495, a496) {
                    0u64
                } else {
                    1u64
                }
            };
            let v14: u64 = {
                let a498 = match v4.clone() {
                    Adt1::C0(h497, _, _, _, _, _, _, _, _, _, _) => {
                        h497
                    }
                };
                let a499 = 0u64;
                if nat_eq(a498, a499) {
                    0u64
                } else {
                    1u64
                }
            };
            let a500 = v12;
            let a501 = v13;
            if nat_lt(a500, a501) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h502, _, _, _, _, _, _, _, _, _, _) => {
                        h502
                    }
                }, 0u64, 0u64, f38(v11, v3.clone())?, 2u64, 0u64, 0u64, 0u64, f18(match v3 {
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
                if nat_lt(a506, a507) {
                    Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                        Adt1::C0(h508, _, _, _, _, _, _, _, _, _, _) => {
                            h508
                        }
                    }, 0u64, 4u64, f45(v11, v3, v4.clone())?, 2u64, 0u64, 0u64, 1u64, f18(match v4 {
                        Adt1::C0(h509, _, _, _, _, _, _, _, _, _, _) => {
                            h509
                        }
                    }), 4u64))
                } else {
                    let a512 = v12;
                    let a513 = {
                        let a510 = v13;
                        let a511 = v14;
                        nat_add(a510, a511)?
                    };
                    if nat_eq(a512, a513) {
                        Ok(Adt3::C3(1u64, match v1 {
                            Adt2::C0(_, _, h514, _, _, _, _, _, _) => {
                                h514
                            }
                        }, 0u64, 0u64, {
                            let a515 = v13;
                            let a516 = 0u64;
                            if nat_eq(a515, a516) {
                                0u64
                            } else {
                                1u64
                            }
                        }, {
                            let a517 = v13;
                            let a518 = 0u64;
                            if nat_eq(a517, a518) {
                                f38(v11, v3.clone())?
                            } else {
                                0u64
                            }
                        }, {
                            let a519 = v14;
                            let a520 = 0u64;
                            if nat_eq(a519, a520) {
                                4u64
                            } else {
                                1u64
                            }
                        }, {
                            let a521 = v14;
                            let a522 = 0u64;
                            if nat_eq(a521, a522) {
                                f45(v11, v3, v4)?
                            } else {
                                1u64
                            }
                        }, {
                            let a524 = match v5.clone() {
                                Adt1::C0(h523, _, _, _, _, _, _, _, _, _, _) => {
                                    h523
                                }
                            };
                            let a525 = 0u64;
                            if nat_eq(a524, a525) {
                                3u64
                            } else {
                                0u64
                            }
                        }, {
                            let a527 = match v5.clone() {
                                Adt1::C0(h526, _, _, _, _, _, _, _, _, _, _) => {
                                    h526
                                }
                            };
                            let a528 = 0u64;
                            if nat_eq(a527, a528) {
                                f38(v11, v5)?
                            } else {
                                0u64
                            }
                        }, 4u64, 4u64))
                    } else {
                        Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                            Adt1::C0(h529, _, _, _, _, _, _, _, _, _, _) => {
                                h529
                            }
                        }, 1u64, 0u64, 2u64, 0u64, 3u64, f38(v11, v5.clone())?, 4u64, f18(match v5 {
                            Adt1::C0(h530, _, _, _, _, _, _, _, _, _, _) => {
                                h530
                            }
                        })))
                    }
                }
            }
        }
    }
}

pub fn f15(v0: Adt1, v1: Adt1) -> bool {
    let a533 = match v0.clone() {
        Adt1::C0(_, h531, _, _, _, _, _, _, _, _, _) => {
            h531
        }
    };
    let a534 = match v1.clone() {
        Adt1::C0(_, h532, _, _, _, _, _, _, _, _, _) => {
            h532
        }
    };
    if if if nat_eq(a533, a534) {
        let a537 = match v0.clone() {
            Adt1::C0(_, _, h535, _, _, _, _, _, _, _, _) => {
                h535
            }
        };
        let a538 = match v1.clone() {
            Adt1::C0(_, _, h536, _, _, _, _, _, _, _, _) => {
                h536
            }
        };
        nat_eq(a537, a538)
    } else {
        false
    } {
        let a541 = match v0.clone() {
            Adt1::C0(_, _, _, h539, _, _, _, _, _, _, _) => {
                h539
            }
        };
        let a542 = match v1.clone() {
            Adt1::C0(_, _, _, h540, _, _, _, _, _, _, _) => {
                h540
            }
        };
        nat_eq(a541, a542)
    } else {
        false
    } {
        let a545 = match v0 {
            Adt1::C0(_, _, _, _, h543, _, _, _, _, _, _) => {
                h543
            }
        };
        let a546 = match v1 {
            Adt1::C0(_, _, _, _, h544, _, _, _, _, _, _) => {
                h544
            }
        };
        nat_eq(a545, a546)
    } else {
        false
    }
}

pub fn f16(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<bool> {
    let a548 = match v1.clone() {
        Adt1::C0(h547, _, _, _, _, _, _, _, _, _, _) => {
            h547
        }
    };
    let a549 = 43u64;
    if nat_eq(a548, a549) {
        let a550 = v0;
        let a551 = 0u64;
        if if if if if if nat_eq(a550, a551) {
            let a553 = match v2.clone() {
                Adt1::C0(h552, _, _, _, _, _, _, _, _, _, _) => {
                    h552
                }
            };
            let a554 = 0u64;
            nat_eq(a553, a554)
        } else {
            false
        } {
            let a556 = match v3.clone() {
                Adt1::C0(h555, _, _, _, _, _, _, _, _, _, _) => {
                    h555
                }
            };
            let a557 = 0u64;
            if nat_eq(a556, a557) {
                true
            } else {
                let a559 = match v3.clone() {
                    Adt1::C0(h558, _, _, _, _, _, _, _, _, _, _) => {
                        h558
                    }
                };
                let a560 = 43u64;
                nat_eq(a559, a560)
            }
        } else {
            false
        } {
            f15(v1.clone(), v2.clone())
        } else {
            false
        } {
            let a562 = match v2 {
                Adt1::C0(_, _, _, _, _, h561, _, _, _, _, _) => {
                    h561
                }
            };
            let a563 = 4u64;
            nat_eq(a562, a563)
        } else {
            false
        } {
            let a566 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h564, _, _, _, _, _) => {
                    h564
                }
            };
            let a567 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h565, _, _, _, _) => {
                    h565
                }
            };
            if nat_le(a566, a567) {
                let a570 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, h568, _, _, _, _) => {
                        h568
                    }
                };
                let a571 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h569, _, _, _) => {
                        h569
                    }
                };
                if nat_le(a570, a571) {
                    let a574 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, h572, _, _, _) => {
                            h572
                        }
                    };
                    let a575 = match v3 {
                        Adt1::C0(_, _, _, _, _, _, _, _, h573, _, _) => {
                            h573
                        }
                    };
                    nat_le(a574, a575)
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        } {
            let a583 = {
                let a581 = match v1.clone() {
                    Adt1::C0(_, _, h576, _, _, _, _, _, _, _, _) => {
                        h576
                    }
                };
                let a582 = {
                    let a579 = match v1.clone() {
                        Adt1::C0(_, _, _, h577, _, _, _, _, _, _, _) => {
                            h577
                        }
                    };
                    let a580 = match v1 {
                        Adt1::C0(_, _, _, _, h578, _, _, _, _, _, _) => {
                            h578
                        }
                    };
                    nat_mul(a579, a580)?
                };
                nat_mul(a581, a582)?
            };
            let a584 = 2147483647u64;
            Ok(nat_le(a583, a584))
        } else {
            Ok(false)
        }
    } else {
        if f46(v1.clone(), v2.clone()) {
            Ok(f47(match v1 {
                Adt1::C0(h585, _, _, _, _, _, _, _, _, _, _) => {
                    h585
                }
            }, match v2 {
                Adt1::C0(h586, _, _, _, _, _, _, _, _, _, _) => {
                    h586
                }
            }, match v3 {
                Adt1::C0(h587, _, _, _, _, _, _, _, _, _, _) => {
                    h587
                }
            }))
        } else {
            Ok(false)
        }
    }
}

pub fn f17(v0: Adt1) -> bool {
    f48(match v0.clone() {
        Adt1::C0(_, h588, _, _, _, _, _, _, _, _, _) => {
            h588
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h589, _, _, _, _, _, _, _, _) => {
            h589
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h590, _, _, _, _, _, _, _) => {
            h590
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, h591, _, _, _, _, _, _) => {
            h591
        }
    })
}

pub fn f18(v0: u64) -> u64 {
    let a592 = v0;
    let a593 = 43u64;
    if nat_eq(a592, a593) {
        1u64
    } else {
        f33(v0)
    }
}

pub fn f19(v0: Adt1) -> bool {
    let v1: u64 = f18(match v0.clone() {
        Adt1::C0(h594, _, _, _, _, _, _, _, _, _, _) => {
            h594
        }
    });
    let a599 = {
        let a596 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h595) => {
                h595
            }
        };
        let a597 = v1;
        let a598 = 1u64;
        nat_rem(a596, a597, a598)
    };
    let a600 = 0u64;
    if if nat_eq(a599, a600) {
        f49(match v0.clone() {
            Adt1::C0(_, h601, _, _, _, _, _, _, _, _, _) => {
                h601
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h602, _, _, _, _, _) => {
                h602
            }
        }, v1)
    } else {
        false
    } {
        f50(match v0.clone() {
            Adt1::C0(_, _, h603, _, _, _, _, _, _, _, _) => {
                h603
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h604, _, _, _, _, _, _, _) => {
                h604
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h605, _, _, _, _, _, _) => {
                h605
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h606, _, _, _, _) => {
                h606
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h607, _, _, _) => {
                h607
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h608, _, _) => {
                h608
            }
        }, v1)
    } else {
        false
    }
}

pub fn f20(v0: Adt1, v1: Adt1) -> R<bool> {
    let a616 = {
        let a614 = match v0.clone() {
            Adt1::C0(_, _, h609, _, _, _, _, _, _, _, _) => {
                h609
            }
        };
        let a615 = {
            let a612 = match v0.clone() {
                Adt1::C0(_, _, _, h610, _, _, _, _, _, _, _) => {
                    h610
                }
            };
            let a613 = match v0.clone() {
                Adt1::C0(_, _, _, _, h611, _, _, _, _, _, _) => {
                    h611
                }
            };
            nat_mul(a612, a613)?
        };
        nat_mul(a614, a615)?
    };
    let a617 = 9223372036854775807u64;
    if nat_le(a616, a617) {
        if f30(v1.clone()) {
            let a619 = match v1 {
                Adt1::C0(_, h618, _, _, _, _, _, _, _, _, _) => {
                    h618
                }
            };
            let a620 = 2147483647u64;
            Ok(nat_le(a619, a620))
        } else {
            let a622 = match v0 {
                Adt1::C0(_, h621, _, _, _, _, _, _, _, _, _) => {
                    h621
                }
            };
            let a623 = 2147483647u64;
            Ok(nat_le(a622, a623))
        }
    } else {
        Ok(false)
    }
}

pub fn f21(v0: Adt1, v1: Adt1) -> R<bool> {
    if f35(v0.clone(), v1.clone())? {
        Ok(true)
    } else {
        Ok(f51(v0, v1))
    }
}

pub fn f22(v0: Adt1) -> R<bool> {
    f52({
        let a626 = match v0.clone() {
            Adt1::C0(_, h624, _, _, _, _, _, _, _, _, _) => {
                h624
            }
        };
        let a627 = f18(match v0.clone() {
            Adt1::C0(h625, _, _, _, _, _, _, _, _, _, _) => {
                h625
            }
        });
        nat_mul(a626, a627)?
    }, match v0.clone() {
        Adt1::C0(_, _, h628, _, _, _, _, _, _, _, _) => {
            h628
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h629, _, _, _, _, _, _, _) => {
            h629
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h630, _, _, _, _, _, _) => {
            h630
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h631, _, _, _, _) => {
            h631
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h632, _, _, _) => {
            h632
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h633, _, _) => {
            h633
        }
    })
}

pub fn f23(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a640 = {
        let a634 = v0;
        let a635 = v2;
        nat_mul(a634, a635)?
    };
    let a641 = {
        let a636 = v1;
        let a637 = 0u64;
        if nat_eq(a636, a637) {
            0u64
        } else {
            let a638 = v0;
            let a639 = 2u64;
            nat_mul(a638, a639)?
        }
    };
    nat_add(a640, a641)
}

pub fn f24(v0: Adt1) -> R<u64> {
    let a650 = {
        let a644 = match v0.clone() {
            Adt1::C0(_, h642, _, _, _, _, _, _, _, _, _) => {
                h642
            }
        };
        let a645 = match v0.clone() {
            Adt1::C0(_, _, h643, _, _, _, _, _, _, _, _) => {
                h643
            }
        };
        nat_mul(a644, a645)?
    };
    let a651 = {
        let a648 = match v0.clone() {
            Adt1::C0(_, _, _, h646, _, _, _, _, _, _, _) => {
                h646
            }
        };
        let a649 = match v0 {
            Adt1::C0(_, _, _, _, h647, _, _, _, _, _, _) => {
                h647
            }
        };
        nat_mul(a648, a649)?
    };
    nat_mul(a650, a651)
}

pub fn f25(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a662 = {
        let a658 = {
            let a654 = 1u64;
            let a655 = {
                let a652 = v0;
                let a653 = 0u64;
                if nat_eq(a652, a653) {
                    0u64
                } else {
                    1u64
                }
            };
            nat_add(a654, a655)?
        };
        let a659 = {
            let a656 = v1;
            let a657 = 0u64;
            if nat_eq(a656, a657) {
                0u64
            } else {
                1u64
            }
        };
        nat_add(a658, a659)?
    };
    let a663 = {
        let a660 = v2;
        let a661 = 0u64;
        if nat_eq(a660, a661) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a662, a663)
}

pub fn f26(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f15(v0.clone(), v1.clone()) {
        let a666 = match v0.clone() {
            Adt1::C0(h664, _, _, _, _, _, _, _, _, _, _) => {
                h664
            }
        };
        let a667 = match v1.clone() {
            Adt1::C0(h665, _, _, _, _, _, _, _, _, _, _) => {
                h665
            }
        };
        nat_eq(a666, a667)
    } else {
        false
    } {
        let a670 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h668, _, _, _, _, _) => {
                h668
            }
        };
        let a671 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h669, _, _, _, _, _) => {
                h669
            }
        };
        nat_eq(a670, a671)
    } else {
        false
    } {
        let a674 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h672, _, _, _, _) => {
                h672
            }
        };
        let a675 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h673, _, _, _, _) => {
                h673
            }
        };
        nat_eq(a674, a675)
    } else {
        false
    } {
        let a678 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h676, _, _, _) => {
                h676
            }
        };
        let a679 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h677, _, _, _) => {
                h677
            }
        };
        nat_eq(a678, a679)
    } else {
        false
    } {
        let a682 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h680, _, _) => {
                h680
            }
        };
        let a683 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h681, _, _) => {
                h681
            }
        };
        nat_eq(a682, a683)
    } else {
        false
    } {
        let a685 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h684) => {
                h684
            }
        };
        let a686 = 0u64;
        if if nat_eq(a685, a686) {
            true
        } else {
            let a688 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h687) => {
                    h687
                }
            };
            let a689 = 0u64;
            nat_eq(a688, a689)
        } {
            true
        } else {
            let a692 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h690) => {
                    h690
                }
            };
            let a693 = match v1 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h691) => {
                    h691
                }
            };
            nat_eq(a692, a693)
        }
    } else {
        false
    }
}

pub fn f27(v0: u64) -> u64 {
    if f32(v0) {
        let m694 = f53(v0);
        match m694 {
            Adt4::C0 => {
                0u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, _, h695) => {
                        h695
                    }
                }
            }
        }
    } else {
        f18(v0)
    }
}

pub fn f28(v0: u64) -> u64 {
    let a696 = v0;
    let a697 = 26u64;
    if nat_eq(a696, a697) {
        4u64
    } else {
        let a698 = v0;
        let a699 = 27u64;
        if nat_eq(a698, a699) {
            8u64
        } else {
            0u64
        }
    }
}

pub fn f29(v0: u64) -> u64 {
    if f32(v0) {
        let m700 = f53(v0);
        match m700 {
            Adt4::C0 => {
                1u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, h701, _) => {
                        h701
                    }
                }
            }
        }
    } else {
        1u64
    }
}

pub fn f30(v0: Adt1) -> bool {
    let a703 = match v0.clone() {
        Adt1::C0(_, h702, _, _, _, _, _, _, _, _, _) => {
            h702
        }
    };
    let a704 = 1u64;
    if nat_eq(a703, a704) {
        true
    } else {
        let a707 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h705, _, _, _, _, _) => {
                h705
            }
        };
        let a708 = f18(match v0 {
            Adt1::C0(h706, _, _, _, _, _, _, _, _, _, _) => {
                h706
            }
        });
        nat_eq(a707, a708)
    }
}

pub fn f31(v0: Adt1, v1: u64) -> R<u64> {
    let a714 = {
        let a710 = match v0.clone() {
            Adt1::C0(_, h709, _, _, _, _, _, _, _, _, _) => {
                h709
            }
        };
        let a711 = f29(v1);
        let a712 = 0u64;
        nat_quot(a710, a711, a712)
    };
    let a715 = f54(match v0 {
        Adt1::C0(h713, _, _, _, _, _, _, _, _, _, _) => {
            h713
        }
    }, v1);
    nat_mul(a714, a715)
}

pub fn f32(v0: u64) -> bool {
    let a716 = v0;
    let a717 = 18u64;
    if if if nat_eq(a716, a717) {
        true
    } else {
        let a718 = 20u64;
        let a719 = v0;
        if nat_le(a718, a719) {
            let a720 = v0;
            let a721 = 23u64;
            nat_le(a720, a721)
        } else {
            false
        }
    } {
        true
    } else {
        let a722 = 10u64;
        let a723 = v0;
        if nat_le(a722, a723) {
            let a724 = v0;
            let a725 = 14u64;
            nat_le(a724, a725)
        } else {
            false
        }
    } {
        true
    } else {
        let a726 = v0;
        let a727 = 2u64;
        if if if if nat_eq(a726, a727) {
            true
        } else {
            let a728 = v0;
            let a729 = 3u64;
            nat_eq(a728, a729)
        } {
            true
        } else {
            let a730 = v0;
            let a731 = 6u64;
            nat_eq(a730, a731)
        } {
            true
        } else {
            let a732 = v0;
            let a733 = 7u64;
            if nat_eq(a732, a733) {
                true
            } else {
                let a734 = v0;
                let a735 = 8u64;
                nat_eq(a734, a735)
            }
        } {
            true
        } else {
            let a736 = 34u64;
            let a737 = v0;
            if if nat_le(a736, a737) {
                let a738 = v0;
                let a739 = 35u64;
                nat_le(a738, a739)
            } else {
                false
            } {
                true
            } else {
                let a740 = 39u64;
                let a741 = v0;
                if nat_le(a740, a741) {
                    let a742 = v0;
                    let a743 = 42u64;
                    nat_le(a742, a743)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f33(v0: u64) -> u64 {
    let a744 = v0;
    let a745 = 0u64;
    if nat_eq(a744, a745) {
        4u64
    } else {
        let a746 = v0;
        let a747 = 1u64;
        if if nat_eq(a746, a747) {
            true
        } else {
            let a748 = v0;
            let a749 = 30u64;
            nat_eq(a748, a749)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f34(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a751 = match v0.clone() {
            Adt1::C0(_, h750, _, _, _, _, _, _, _, _, _) => {
                h750
            }
        };
        let a752 = 4u64;
        nat_mul(a751, a752)?
    };
    let v2: u64 = {
        let a755 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h753, _, _, _, _) => {
                h753
            }
        };
        let a756 = match v0.clone() {
            Adt1::C0(_, _, h754, _, _, _, _, _, _, _, _) => {
                h754
            }
        };
        nat_mul(a755, a756)?
    };
    let v3: u64 = {
        let a758 = v2;
        let a759 = match v0.clone() {
            Adt1::C0(_, _, _, h757, _, _, _, _, _, _, _) => {
                h757
            }
        };
        nat_mul(a758, a759)?
    };
    let a761 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h760, _, _, _, _, _) => {
            h760
        }
    };
    let a762 = 4u64;
    if if if if nat_eq(a761, a762) {
        let a767 = {
            let a764 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h763) => {
                    h763
                }
            };
            let a765 = 4u64;
            let a766 = 1u64;
            nat_rem(a764, a765, a766)
        };
        let a768 = 0u64;
        nat_eq(a767, a768)
    } else {
        false
    } {
        let a776 = {
            let a774 = match v0.clone() {
                Adt1::C0(_, _, h769, _, _, _, _, _, _, _, _) => {
                    h769
                }
            };
            let a775 = {
                let a772 = match v0.clone() {
                    Adt1::C0(_, _, _, h770, _, _, _, _, _, _, _) => {
                        h770
                    }
                };
                let a773 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h771, _, _, _, _, _, _) => {
                        h771
                    }
                };
                nat_mul(a772, a773)?
            };
            nat_mul(a774, a775)?
        };
        let a777 = 1u64;
        if nat_le(a776, a777) {
            true
        } else {
            let a779 = v1;
            let a780 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h778, _, _, _, _) => {
                    h778
                }
            };
            if nat_le(a779, a780) {
                let a785 = {
                    let a782 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, h781, _, _, _, _) => {
                            h781
                        }
                    };
                    let a783 = 4u64;
                    let a784 = 1u64;
                    nat_rem(a782, a783, a784)
                };
                let a786 = 0u64;
                nat_eq(a785, a786)
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a788 = match v0.clone() {
            Adt1::C0(_, _, _, h787, _, _, _, _, _, _, _) => {
                h787
            }
        };
        let a789 = 1u64;
        if nat_le(a788, a789) {
            true
        } else {
            let a791 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h790, _, _, _) => {
                    h790
                }
            };
            let a792 = v2;
            nat_eq(a791, a792)
        }
    } else {
        false
    } {
        let a794 = match v0.clone() {
            Adt1::C0(_, _, _, _, h793, _, _, _, _, _, _) => {
                h793
            }
        };
        let a795 = 1u64;
        if nat_le(a794, a795) {
            Ok(true)
        } else {
            let a797 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, h796, _, _) => {
                    h796
                }
            };
            let a798 = v3;
            Ok(nat_eq(a797, a798))
        }
    } else {
        Ok(false)
    }
}

pub fn f35(v0: Adt1, v1: Adt1) -> R<bool> {
    let a800 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h799, _) => {
            h799
        }
    };
    let a801 = 0u64;
    if nat_eq(a800, a801) {
        Ok(true)
    } else {
        let a803 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h802, _) => {
                h802
            }
        };
        let a804 = 0u64;
        if nat_eq(a803, a804) {
            Ok(true)
        } else {
            let a806 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h805) => {
                    h805
                }
            };
            let a807 = 0u64;
            if nat_eq(a806, a807) {
                Ok(true)
            } else {
                let a809 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h808) => {
                        h808
                    }
                };
                let a810 = 0u64;
                if nat_eq(a809, a810) {
                    Ok(true)
                } else {
                    let a816 = {
                        let a813 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h811) => {
                                h811
                            }
                        };
                        let a814 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h812, _) => {
                                h812
                            }
                        };
                        nat_add(a813, a814)?
                    };
                    let a817 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h815) => {
                            h815
                        }
                    };
                    if nat_le(a816, a817) {
                        Ok(true)
                    } else {
                        let a823 = {
                            let a820 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h818) => {
                                    h818
                                }
                            };
                            let a821 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h819, _) => {
                                    h819
                                }
                            };
                            nat_add(a820, a821)?
                        };
                        let a824 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h822) => {
                                h822
                            }
                        };
                        Ok(nat_le(a823, a824))
                    }
                }
            }
        }
    }
}

pub fn f36(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a827 = match v0.clone() {
            Adt1::C0(_, _, h825, _, _, _, _, _, _, _, _) => {
                h825
            }
        };
        let a828 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h826, _, _, _, _) => {
                h826
            }
        };
        nat_mul(a827, a828)?
    };
    if if f30(v0.clone()) {
        let a830 = match v0.clone() {
            Adt1::C0(_, _, _, h829, _, _, _, _, _, _, _) => {
                h829
            }
        };
        let a831 = 1u64;
        if nat_eq(a830, a831) {
            true
        } else {
            let a833 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h832, _, _, _) => {
                    h832
                }
            };
            let a834 = v1;
            nat_eq(a833, a834)
        }
    } else {
        false
    } {
        let a836 = match v0.clone() {
            Adt1::C0(_, _, _, _, h835, _, _, _, _, _, _) => {
                h835
            }
        };
        let a837 = 1u64;
        if nat_eq(a836, a837) {
            Ok(true)
        } else {
            let a842 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h838, _, _) => {
                    h838
                }
            };
            let a843 = {
                let a840 = v1;
                let a841 = match v0 {
                    Adt1::C0(_, _, _, h839, _, _, _, _, _, _, _) => {
                        h839
                    }
                };
                nat_mul(a840, a841)?
            };
            Ok(nat_eq(a842, a843))
        }
    } else {
        Ok(false)
    }
}

pub fn f37(v0: u64) -> u64 {
    let a844 = v0;
    let a845 = 3u64;
    if nat_eq(a844, a845) {
        16u64
    } else {
        10u64
    }
}

pub fn f38(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a847 = v0;
        let a848 = match v1.clone() {
            Adt1::C0(_, h846, _, _, _, _, _, _, _, _, _) => {
                h846
            }
        };
        let a849 = 0u64;
        nat_quot(a847, a848, a849)
    };
    let v3: u64 = {
        let a851 = v2;
        let a852 = match v1.clone() {
            Adt1::C0(_, _, h850, _, _, _, _, _, _, _, _) => {
                h850
            }
        };
        let a853 = 0u64;
        nat_quot(a851, a852, a853)
    };
    f39({
        let a855 = v0;
        let a856 = match v1.clone() {
            Adt1::C0(_, h854, _, _, _, _, _, _, _, _, _) => {
                h854
            }
        };
        let a857 = 0u64;
        nat_rem(a855, a856, a857)
    }, {
        let a859 = v2;
        let a860 = match v1.clone() {
            Adt1::C0(_, _, h858, _, _, _, _, _, _, _, _) => {
                h858
            }
        };
        let a861 = 0u64;
        nat_rem(a859, a860, a861)
    }, {
        let a863 = v3;
        let a864 = match v1.clone() {
            Adt1::C0(_, _, _, h862, _, _, _, _, _, _, _) => {
                h862
            }
        };
        let a865 = 0u64;
        nat_rem(a863, a864, a865)
    }, {
        let a867 = v3;
        let a868 = match v1.clone() {
            Adt1::C0(_, _, _, h866, _, _, _, _, _, _, _) => {
                h866
            }
        };
        let a869 = 0u64;
        nat_quot(a867, a868, a869)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h870, _, _, _, _, _) => {
            h870
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h871, _, _, _, _) => {
            h871
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h872, _, _, _) => {
            h872
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h873, _, _) => {
            h873
        }
    })
}

pub fn f39(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a886 = {
        let a882 = {
            let a878 = {
                let a874 = v0;
                let a875 = v4;
                nat_mul(a874, a875)?
            };
            let a879 = {
                let a876 = v1;
                let a877 = v5;
                nat_mul(a876, a877)?
            };
            nat_add(a878, a879)?
        };
        let a883 = {
            let a880 = v2;
            let a881 = v6;
            nat_mul(a880, a881)?
        };
        nat_add(a882, a883)?
    };
    let a887 = {
        let a884 = v3;
        let a885 = v7;
        nat_mul(a884, a885)?
    };
    nat_add(a886, a887)
}

pub fn f40(v0: u64, v1: Adt1, v2: u64, v3: u64) -> R<Adt3> {
    let v4: u64 = {
        let a888 = v0;
        let a889 = 2u64;
        let a890 = 0u64;
        nat_quot(a888, a889, a890)
    };
    let v5: u64 = f28(match v1.clone() {
        Adt1::C0(h891, _, _, _, _, _, _, _, _, _, _) => {
            h891
        }
    });
    let a895 = {
        let a892 = v0;
        let a893 = 2u64;
        let a894 = 0u64;
        nat_rem(a892, a893, a894)
    };
    let a896 = 0u64;
    if nat_eq(a895, a896) {
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 4u64, f38(v4, v1)?, 2u64, 0u64, 0u64, 2u64, v5, 8u64))
    } else {
        let a897 = v3;
        let a898 = v2;
        if if nat_lt(a897, a898) {
            let a901 = v3;
            let a902 = {
                let a899 = v5;
                let a900 = 4u64;
                if nat_eq(a899, a900) {
                    2147483648u64
                } else {
                    9223372036854775808u64
                }
            };
            nat_lt(a901, a902)
        } else {
            false
        } {
            Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, {
                let a903 = 3u64;
                let a904 = v4;
                nat_add(a903, a904)?
            }, 8u64, 8u64))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f41(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64) -> R<Adt3> {
    let v5: u64 = {
        let a906 = 1u64;
        let a907 = f31(v1.clone(), match v3.clone() {
            Adt1::C0(h905, _, _, _, _, _, _, _, _, _, _) => {
                h905
            }
        })?;
        nat_add(a906, a907)?
    };
    let v6: u64 = {
        let a908 = v0;
        let a909 = v5;
        let a910 = 0u64;
        nat_rem(a908, a909, a910)
    };
    let v7: u64 = {
        let a911 = v0;
        let a912 = v5;
        let a913 = 0u64;
        nat_quot(a911, a912, a913)
    };
    let v8: u64 = {
        let a915 = v7;
        let a916 = match v1.clone() {
            Adt1::C0(_, _, h914, _, _, _, _, _, _, _, _) => {
                h914
            }
        };
        let a917 = 0u64;
        nat_rem(a915, a916, a917)
    };
    let v9: u64 = {
        let a919 = v7;
        let a920 = match v1.clone() {
            Adt1::C0(_, _, h918, _, _, _, _, _, _, _, _) => {
                h918
            }
        };
        let a921 = 0u64;
        nat_quot(a919, a920, a921)
    };
    let v10: u64 = {
        let a923 = v9;
        let a924 = match v1.clone() {
            Adt1::C0(_, _, _, h922, _, _, _, _, _, _, _) => {
                h922
            }
        };
        let a925 = 0u64;
        nat_rem(a923, a924, a925)
    };
    let v11: u64 = {
        let a927 = v9;
        let a928 = match v1.clone() {
            Adt1::C0(_, _, _, h926, _, _, _, _, _, _, _) => {
                h926
            }
        };
        let a929 = 0u64;
        nat_quot(a927, a928, a929)
    };
    let a930 = v6;
    let a931 = 0u64;
    if nat_eq(a930, a931) {
        let v12: u64 = {
            let a948 = v8;
            let a949 = {
                let a946 = match v2.clone() {
                    Adt1::C0(_, h932, _, _, _, _, _, _, _, _, _) => {
                        h932
                    }
                };
                let a947 = {
                    let a944 = {
                        let a934 = v10;
                        let a935 = match v2.clone() {
                            Adt1::C0(_, _, h933, _, _, _, _, _, _, _, _) => {
                                h933
                            }
                        };
                        let a936 = 0u64;
                        nat_rem(a934, a935, a936)
                    };
                    let a945 = {
                        let a942 = match v2.clone() {
                            Adt1::C0(_, _, h937, _, _, _, _, _, _, _, _) => {
                                h937
                            }
                        };
                        let a943 = {
                            let a939 = v11;
                            let a940 = match v2 {
                                Adt1::C0(_, _, _, h938, _, _, _, _, _, _, _) => {
                                    h938
                                }
                            };
                            let a941 = 0u64;
                            nat_rem(a939, a940, a941)
                        };
                        nat_mul(a942, a943)?
                    };
                    nat_add(a944, a945)?
                };
                nat_mul(a946, a947)?
            };
            nat_add(a948, a949)?
        };
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, {
            let a950 = 3u64;
            let a951 = v12;
            nat_add(a950, a951)?
        }, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
    } else {
        let a953 = v4;
        let a954 = match v3.clone() {
            Adt1::C0(_, _, h952, _, _, _, _, _, _, _, _) => {
                h952
            }
        };
        if nat_lt(a953, a954) {
            let v13: u64 = {
                let a959 = {
                    let a955 = v6;
                    let a956 = 1u64;
                    nat_sub(a955, a956)
                };
                let a960 = f54(match v1.clone() {
                    Adt1::C0(h957, _, _, _, _, _, _, _, _, _, _) => {
                        h957
                    }
                }, match v3.clone() {
                    Adt1::C0(h958, _, _, _, _, _, _, _, _, _, _) => {
                        h958
                    }
                });
                let a961 = 0u64;
                nat_quot(a959, a960, a961)
            };
            let v14: u64 = f39({
                let a963 = v13;
                let a964 = f29(match v3.clone() {
                    Adt1::C0(h962, _, _, _, _, _, _, _, _, _, _) => {
                        h962
                    }
                });
                nat_mul(a963, a964)?
            }, v8, v10, v11, match v1.clone() {
                Adt1::C0(_, _, _, _, _, h965, _, _, _, _, _) => {
                    h965
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, h966, _, _, _, _) => {
                    h966
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h967, _, _, _) => {
                    h967
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h968, _, _) => {
                    h968
                }
            })?;
            let v15: u64 = f39(v13, v4, v10, v11, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h969, _, _, _, _, _) => {
                    h969
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h970, _, _, _, _) => {
                    h970
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h971, _, _, _) => {
                    h971
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h972, _, _) => {
                    h972
                }
            })?;
            if f32(match v3.clone() {
                Adt1::C0(h973, _, _, _, _, _, _, _, _, _, _) => {
                    h973
                }
            }) {
                Ok(Adt3::C6(v14, v15))
            } else {
                Ok(f55(match v1 {
                    Adt1::C0(h974, _, _, _, _, _, _, _, _, _, _) => {
                        h974
                    }
                }, match v3 {
                    Adt1::C0(h975, _, _, _, _, _, _, _, _, _, _) => {
                        h975
                    }
                }, {
                    let a976 = v6;
                    let a977 = 1u64;
                    nat_sub(a976, a977)
                }, v14, v15))
            }
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f42(v0: u64, v1: u64, v2: u64) -> Adt3 {
    Adt3::C3(2u64, 0u64, 0u64, 0u64, 1u64, v0, v1, v2, 0u64, 2u64, 4u64, 4u64)
}

pub fn f43(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt3 {
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

pub fn f44(v0: u64, v1: u64) -> Adt3 {
    let a978 = v0;
    let a979 = 8u64;
    if nat_eq(a978, a979) {
        Adt3::C3(1u64, 6u64, 0u64, 0u64, 1u64, 0u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
    } else {
        let a980 = v0;
        let a981 = 9u64;
        if nat_eq(a980, a981) {
            Adt3::C3(1u64, 2u64, 0u64, 0u64, 2u64, v1, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
        } else {
            let a982 = v0;
            let a983 = 10u64;
            if nat_eq(a982, a983) {
                Adt3::C3(5u64, 16u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
            } else {
                let a984 = v0;
                let a985 = 11u64;
                if nat_eq(a984, a985) {
                    Adt3::C3(1u64, 0u64, 0u64, 0u64, 2u64, 1065353216u64, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
                } else {
                    let a986 = v0;
                    let a987 = 12u64;
                    if nat_eq(a986, a987) {
                        Adt3::C3(1u64, 3u64, 0u64, 0u64, 1u64, 0u64, 1u64, 2u64, 0u64, 0u64, 4u64, 4u64)
                    } else {
                        let a988 = v0;
                        let a989 = 13u64;
                        if nat_eq(a988, a989) {
                            Adt3::C3(1u64, 0u64, 0u64, 0u64, 1u64, 1u64, 2u64, 1065353216u64, 0u64, 1u64, 4u64, 4u64)
                        } else {
                            let a990 = v0;
                            let a991 = 14u64;
                            if nat_eq(a990, a991) {
                                Adt3::C3(1u64, 2u64, 0u64, 0u64, 1u64, 0u64, 1u64, 1u64, 0u64, 0u64, 4u64, 4u64)
                            } else {
                                Adt3::C1
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f45(v0: u64, v1: Adt1, v2: Adt1) -> R<u64> {
    let v3: u64 = {
        let a993 = v0;
        let a994 = match v1.clone() {
            Adt1::C0(_, h992, _, _, _, _, _, _, _, _, _) => {
                h992
            }
        };
        let a995 = 0u64;
        nat_quot(a993, a994, a995)
    };
    let v4: u64 = {
        let a997 = v3;
        let a998 = match v1.clone() {
            Adt1::C0(_, _, h996, _, _, _, _, _, _, _, _) => {
                h996
            }
        };
        let a999 = 0u64;
        nat_quot(a997, a998, a999)
    };
    f39({
        let a1005 = {
            let a1001 = v0;
            let a1002 = match v1.clone() {
                Adt1::C0(_, h1000, _, _, _, _, _, _, _, _, _) => {
                    h1000
                }
            };
            let a1003 = 0u64;
            nat_rem(a1001, a1002, a1003)
        };
        let a1006 = match v2.clone() {
            Adt1::C0(_, h1004, _, _, _, _, _, _, _, _, _) => {
                h1004
            }
        };
        let a1007 = 0u64;
        nat_rem(a1005, a1006, a1007)
    }, {
        let a1013 = {
            let a1009 = v3;
            let a1010 = match v1.clone() {
                Adt1::C0(_, _, h1008, _, _, _, _, _, _, _, _) => {
                    h1008
                }
            };
            let a1011 = 0u64;
            nat_rem(a1009, a1010, a1011)
        };
        let a1014 = match v2.clone() {
            Adt1::C0(_, _, h1012, _, _, _, _, _, _, _, _) => {
                h1012
            }
        };
        let a1015 = 0u64;
        nat_rem(a1013, a1014, a1015)
    }, {
        let a1021 = {
            let a1017 = v4;
            let a1018 = match v1.clone() {
                Adt1::C0(_, _, _, h1016, _, _, _, _, _, _, _) => {
                    h1016
                }
            };
            let a1019 = 0u64;
            nat_rem(a1017, a1018, a1019)
        };
        let a1022 = match v2.clone() {
            Adt1::C0(_, _, _, h1020, _, _, _, _, _, _, _) => {
                h1020
            }
        };
        let a1023 = 0u64;
        nat_rem(a1021, a1022, a1023)
    }, {
        let a1029 = {
            let a1025 = v4;
            let a1026 = match v1 {
                Adt1::C0(_, _, _, h1024, _, _, _, _, _, _, _) => {
                    h1024
                }
            };
            let a1027 = 0u64;
            nat_quot(a1025, a1026, a1027)
        };
        let a1030 = match v2.clone() {
            Adt1::C0(_, _, _, _, h1028, _, _, _, _, _, _) => {
                h1028
            }
        };
        let a1031 = 0u64;
        nat_rem(a1029, a1030, a1031)
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h1032, _, _, _, _, _) => {
            h1032
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h1033, _, _, _, _) => {
            h1033
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1034, _, _, _) => {
            h1034
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1035, _, _) => {
            h1035
        }
    })
}

pub fn f46(v0: Adt1, v1: Adt1) -> bool {
    if f17(v1.clone()) {
        f17(v0)
    } else {
        let a1041 = {
            let a1038 = match v0.clone() {
                Adt1::C0(_, h1036, _, _, _, _, _, _, _, _, _) => {
                    h1036
                }
            };
            let a1039 = match v1.clone() {
                Adt1::C0(_, h1037, _, _, _, _, _, _, _, _, _) => {
                    h1037
                }
            };
            let a1040 = 1u64;
            nat_rem(a1038, a1039, a1040)
        };
        let a1042 = 0u64;
        if if if nat_eq(a1041, a1042) {
            let a1048 = {
                let a1045 = match v0.clone() {
                    Adt1::C0(_, _, h1043, _, _, _, _, _, _, _, _) => {
                        h1043
                    }
                };
                let a1046 = match v1.clone() {
                    Adt1::C0(_, _, h1044, _, _, _, _, _, _, _, _) => {
                        h1044
                    }
                };
                let a1047 = 1u64;
                nat_rem(a1045, a1046, a1047)
            };
            let a1049 = 0u64;
            nat_eq(a1048, a1049)
        } else {
            false
        } {
            let a1055 = {
                let a1052 = match v0.clone() {
                    Adt1::C0(_, _, _, h1050, _, _, _, _, _, _, _) => {
                        h1050
                    }
                };
                let a1053 = match v1.clone() {
                    Adt1::C0(_, _, _, h1051, _, _, _, _, _, _, _) => {
                        h1051
                    }
                };
                let a1054 = 1u64;
                nat_rem(a1052, a1053, a1054)
            };
            let a1056 = 0u64;
            nat_eq(a1055, a1056)
        } else {
            false
        } {
            let a1062 = {
                let a1059 = match v0 {
                    Adt1::C0(_, _, _, _, h1057, _, _, _, _, _, _) => {
                        h1057
                    }
                };
                let a1060 = match v1 {
                    Adt1::C0(_, _, _, _, h1058, _, _, _, _, _, _) => {
                        h1058
                    }
                };
                let a1061 = 1u64;
                nat_rem(a1059, a1060, a1061)
            };
            let a1063 = 0u64;
            nat_eq(a1062, a1063)
        } else {
            false
        }
    }
}

pub fn f47(v0: u64, v1: u64, v2: u64) -> bool {
    let a1064 = v0;
    let a1065 = 0u64;
    if if if nat_eq(a1064, a1065) {
        let a1066 = v1;
        let a1067 = 0u64;
        nat_eq(a1066, a1067)
    } else {
        false
    } {
        let a1068 = v2;
        let a1069 = 0u64;
        nat_eq(a1068, a1069)
    } else {
        false
    } {
        true
    } else {
        let a1070 = v0;
        let a1071 = 1u64;
        if if nat_eq(a1070, a1071) {
            true
        } else {
            let a1072 = v0;
            let a1073 = 30u64;
            nat_eq(a1072, a1073)
        } {
            let a1074 = v1;
            let a1075 = v0;
            if if nat_eq(a1074, a1075) {
                let a1076 = v2;
                let a1077 = v0;
                nat_eq(a1076, a1077)
            } else {
                false
            } {
                true
            } else {
                let a1078 = v1;
                let a1079 = 0u64;
                if nat_eq(a1078, a1079) {
                    let a1080 = v2;
                    let a1081 = v0;
                    if nat_eq(a1080, a1081) {
                        true
                    } else {
                        let a1082 = v2;
                        let a1083 = 0u64;
                        nat_eq(a1082, a1083)
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

pub fn f48(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1084 = v0;
    let a1085 = 0u64;
    if if if nat_eq(a1084, a1085) {
        true
    } else {
        let a1086 = v1;
        let a1087 = 0u64;
        nat_eq(a1086, a1087)
    } {
        true
    } else {
        let a1088 = v2;
        let a1089 = 0u64;
        nat_eq(a1088, a1089)
    } {
        true
    } else {
        let a1090 = v3;
        let a1091 = 0u64;
        nat_eq(a1090, a1091)
    }
}

pub fn f49(v0: u64, v1: u64, v2: u64) -> bool {
    let a1092 = v0;
    let a1093 = 1u64;
    if nat_le(a1092, a1093) {
        true
    } else {
        let a1097 = {
            let a1094 = v1;
            let a1095 = v2;
            let a1096 = 1u64;
            nat_rem(a1094, a1095, a1096)
        };
        let a1098 = 0u64;
        nat_eq(a1097, a1098)
    }
}

pub fn f50(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f49(v0, v3, v6) {
        f49(v1, v4, v6)
    } else {
        false
    } {
        f49(v2, v5, v6)
    } else {
        false
    }
}

pub fn f51(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f15(v0.clone(), v1.clone()) {
        let a1101 = match v0.clone() {
            Adt1::C0(h1099, _, _, _, _, _, _, _, _, _, _) => {
                h1099
            }
        };
        let a1102 = match v1.clone() {
            Adt1::C0(h1100, _, _, _, _, _, _, _, _, _, _) => {
                h1100
            }
        };
        nat_eq(a1101, a1102)
    } else {
        false
    } {
        let a1105 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1103) => {
                h1103
            }
        };
        let a1106 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1104) => {
                h1104
            }
        };
        nat_eq(a1105, a1106)
    } else {
        false
    } {
        let a1109 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1107, _, _, _, _, _) => {
                h1107
            }
        };
        let a1110 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1108, _, _, _, _, _) => {
                h1108
            }
        };
        nat_eq(a1109, a1110)
    } else {
        false
    } {
        let a1113 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1111, _, _, _, _) => {
                h1111
            }
        };
        let a1114 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1112, _, _, _, _) => {
                h1112
            }
        };
        nat_eq(a1113, a1114)
    } else {
        false
    } {
        let a1117 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1115, _, _, _) => {
                h1115
            }
        };
        let a1118 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1116, _, _, _) => {
                h1116
            }
        };
        nat_eq(a1117, a1118)
    } else {
        false
    } {
        let a1121 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1119, _, _) => {
                h1119
            }
        };
        let a1122 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1120, _, _) => {
                h1120
            }
        };
        nat_eq(a1121, a1122)
    } else {
        false
    }
}

pub fn f52(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f56(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f56(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f56(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f56(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f56(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f56(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f53(v0: u64) -> Adt4 {
    let a1123 = v0;
    let a1124 = 0u64;
    if nat_eq(a1123, a1124) {
        Adt4::C1(Adt5::C0(0u64, 1u64, 4u64))
    } else {
        let a1125 = v0;
        let a1126 = 1u64;
        if nat_eq(a1125, a1126) {
            Adt4::C1(Adt5::C0(1u64, 1u64, 2u64))
        } else {
            let a1127 = v0;
            let a1128 = 2u64;
            if nat_eq(a1127, a1128) {
                Adt4::C1(Adt5::C0(2u64, 32u64, 18u64))
            } else {
                let a1129 = v0;
                let a1130 = 3u64;
                if nat_eq(a1129, a1130) {
                    Adt4::C1(Adt5::C0(3u64, 32u64, 20u64))
                } else {
                    let a1131 = v0;
                    let a1132 = 6u64;
                    if nat_eq(a1131, a1132) {
                        Adt4::C1(Adt5::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1133 = v0;
                        let a1134 = 7u64;
                        if nat_eq(a1133, a1134) {
                            Adt4::C1(Adt5::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1135 = v0;
                            let a1136 = 8u64;
                            if nat_eq(a1135, a1136) {
                                Adt4::C1(Adt5::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1137 = v0;
                                let a1138 = 9u64;
                                if nat_eq(a1137, a1138) {
                                    Adt4::C1(Adt5::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1139 = v0;
                                    let a1140 = 10u64;
                                    if nat_eq(a1139, a1140) {
                                        Adt4::C1(Adt5::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1141 = v0;
                                        let a1142 = 11u64;
                                        if nat_eq(a1141, a1142) {
                                            Adt4::C1(Adt5::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1143 = v0;
                                            let a1144 = 12u64;
                                            if nat_eq(a1143, a1144) {
                                                Adt4::C1(Adt5::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1145 = v0;
                                                let a1146 = 13u64;
                                                if nat_eq(a1145, a1146) {
                                                    Adt4::C1(Adt5::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1147 = v0;
                                                    let a1148 = 14u64;
                                                    if nat_eq(a1147, a1148) {
                                                        Adt4::C1(Adt5::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1149 = v0;
                                                        let a1150 = 15u64;
                                                        if nat_eq(a1149, a1150) {
                                                            Adt4::C1(Adt5::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1151 = v0;
                                                            let a1152 = 16u64;
                                                            if nat_eq(a1151, a1152) {
                                                                Adt4::C1(Adt5::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1153 = v0;
                                                                let a1154 = 17u64;
                                                                if nat_eq(a1153, a1154) {
                                                                    Adt4::C1(Adt5::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1155 = v0;
                                                                    let a1156 = 18u64;
                                                                    if nat_eq(a1155, a1156) {
                                                                        Adt4::C1(Adt5::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1157 = v0;
                                                                        let a1158 = 19u64;
                                                                        if nat_eq(a1157, a1158) {
                                                                            Adt4::C1(Adt5::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1159 = v0;
                                                                            let a1160 = 20u64;
                                                                            if nat_eq(a1159, a1160) {
                                                                                Adt4::C1(Adt5::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1161 = v0;
                                                                                let a1162 = 21u64;
                                                                                if nat_eq(a1161, a1162) {
                                                                                    Adt4::C1(Adt5::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1163 = v0;
                                                                                    let a1164 = 22u64;
                                                                                    if nat_eq(a1163, a1164) {
                                                                                        Adt4::C1(Adt5::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1165 = v0;
                                                                                        let a1166 = 23u64;
                                                                                        if nat_eq(a1165, a1166) {
                                                                                            Adt4::C1(Adt5::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1167 = v0;
                                                                                            let a1168 = 24u64;
                                                                                            if nat_eq(a1167, a1168) {
                                                                                                Adt4::C1(Adt5::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1169 = v0;
                                                                                                let a1170 = 25u64;
                                                                                                if nat_eq(a1169, a1170) {
                                                                                                    Adt4::C1(Adt5::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1171 = v0;
                                                                                                    let a1172 = 26u64;
                                                                                                    if nat_eq(a1171, a1172) {
                                                                                                        Adt4::C1(Adt5::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1173 = v0;
                                                                                                        let a1174 = 27u64;
                                                                                                        if nat_eq(a1173, a1174) {
                                                                                                            Adt4::C1(Adt5::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1175 = v0;
                                                                                                            let a1176 = 28u64;
                                                                                                            if nat_eq(a1175, a1176) {
                                                                                                                Adt4::C1(Adt5::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1177 = v0;
                                                                                                                let a1178 = 29u64;
                                                                                                                if nat_eq(a1177, a1178) {
                                                                                                                    Adt4::C1(Adt5::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1179 = v0;
                                                                                                                    let a1180 = 30u64;
                                                                                                                    if nat_eq(a1179, a1180) {
                                                                                                                        Adt4::C1(Adt5::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1181 = v0;
                                                                                                                        let a1182 = 34u64;
                                                                                                                        if nat_eq(a1181, a1182) {
                                                                                                                            Adt4::C1(Adt5::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1183 = v0;
                                                                                                                            let a1184 = 35u64;
                                                                                                                            if nat_eq(a1183, a1184) {
                                                                                                                                Adt4::C1(Adt5::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1185 = v0;
                                                                                                                                let a1186 = 39u64;
                                                                                                                                if nat_eq(a1185, a1186) {
                                                                                                                                    Adt4::C1(Adt5::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1187 = v0;
                                                                                                                                    let a1188 = 40u64;
                                                                                                                                    if nat_eq(a1187, a1188) {
                                                                                                                                        Adt4::C1(Adt5::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1189 = v0;
                                                                                                                                        let a1190 = 41u64;
                                                                                                                                        if nat_eq(a1189, a1190) {
                                                                                                                                            Adt4::C1(Adt5::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1191 = v0;
                                                                                                                                            let a1192 = 42u64;
                                                                                                                                            if nat_eq(a1191, a1192) {
                                                                                                                                                Adt4::C1(Adt5::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1193 = v0;
                                                                                                                                                let a1194 = 43u64;
                                                                                                                                                if nat_eq(a1193, a1194) {
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

pub fn f54(v0: u64, v1: u64) -> u64 {
    let a1195 = v0;
    let a1196 = 1u64;
    if if nat_eq(a1195, a1196) {
        let a1197 = v1;
        let a1198 = 43u64;
        nat_eq(a1197, a1198)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f55(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt3 {
    let a1199 = f54(v0, v1);
    let a1200 = 2u64;
    if nat_eq(a1199, a1200) {
        let a1204 = {
            let a1201 = v2;
            let a1202 = 2u64;
            let a1203 = 0u64;
            nat_rem(a1201, a1202, a1203)
        };
        let a1205 = 0u64;
        if nat_eq(a1204, a1205) {
            Adt3::C3(0u64, 0u64, v0, 0u64, 0u64, v3, 2u64, 0u64, 1u64, 0u64, f33(v0), 8u64)
        } else {
            Adt3::C3(0u64, 0u64, 0u64, v1, 1u64, 0u64, 2u64, 0u64, 3u64, v4, 8u64, f27(v1))
        }
    } else {
        Adt3::C3(0u64, 0u64, v0, v1, 0u64, v3, 2u64, 0u64, 3u64, v4, f33(v0), f27(v1))
    }
}

pub fn f56(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a1206 = v1;
    let a1207 = 1u64;
    if if if nat_le(a1206, a1207) {
        true
    } else {
        let a1208 = v0;
        let a1209 = v2;
        nat_le(a1208, a1209)
    } {
        let a1210 = v3;
        let a1211 = 1u64;
        if nat_le(a1210, a1211) {
            true
        } else {
            let a1218 = {
                let a1216 = v0;
                let a1217 = {
                    let a1214 = {
                        let a1212 = v1;
                        let a1213 = 1u64;
                        nat_sub(a1212, a1213)
                    };
                    let a1215 = v2;
                    nat_mul(a1214, a1215)?
                };
                nat_add(a1216, a1217)?
            };
            let a1219 = v4;
            nat_le(a1218, a1219)
        }
    } else {
        false
    } {
        let a1220 = v5;
        let a1221 = 1u64;
        if nat_le(a1220, a1221) {
            Ok(true)
        } else {
            let a1234 = {
                let a1232 = {
                    let a1226 = v0;
                    let a1227 = {
                        let a1224 = {
                            let a1222 = v1;
                            let a1223 = 1u64;
                            nat_sub(a1222, a1223)
                        };
                        let a1225 = v2;
                        nat_mul(a1224, a1225)?
                    };
                    nat_add(a1226, a1227)?
                };
                let a1233 = {
                    let a1230 = {
                        let a1228 = v3;
                        let a1229 = 1u64;
                        nat_sub(a1228, a1229)
                    };
                    let a1231 = v4;
                    nat_mul(a1230, a1231)?
                };
                nat_add(a1232, a1233)?
            };
            let a1235 = v6;
            Ok(nat_le(a1234, a1235))
        }
    } else {
        Ok(false)
    }
}
