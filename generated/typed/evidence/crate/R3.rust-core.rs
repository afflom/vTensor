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
    if if if if f15(v1.clone(), v3.clone()) {
        f16(v1.clone(), v2.clone())
    } else {
        false
    } {
        f17(match v1.clone() {
            Adt1::C0(h42, _, _, _, _, _, _, _, _, _, _) => {
                h42
            }
        }, match v2.clone() {
            Adt1::C0(h43, _, _, _, _, _, _, _, _, _, _) => {
                h43
            }
        }, match v3.clone() {
            Adt1::C0(h44, _, _, _, _, _, _, _, _, _, _) => {
                h44
            }
        })
    } else {
        false
    } {
        if f18(v1.clone()) {
            true
        } else {
            let a47 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, h45, _, _, _, _, _) => {
                    h45
                }
            };
            let a48 = f19(match v1.clone() {
                Adt1::C0(h46, _, _, _, _, _, _, _, _, _, _) => {
                    h46
                }
            });
            if if if if if if if nat_eq(a47, a48) {
                let a51 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h49, _, _, _, _, _) => {
                        h49
                    }
                };
                let a52 = f19(match v3.clone() {
                    Adt1::C0(h50, _, _, _, _, _, _, _, _, _, _) => {
                        h50
                    }
                });
                nat_eq(a51, a52)
            } else {
                false
            } {
                f20(v1.clone())
            } else {
                false
            } {
                f20(v2.clone())
            } else {
                false
            } {
                f20(v3.clone())
            } else {
                false
            } {
                f21(v1.clone(), v2.clone())?
            } else {
                false
            } {
                f22(v1.clone(), v3.clone())?
            } else {
                false
            } {
                f22(v2.clone(), v3.clone())?
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v4: u64 = if if f18(v1.clone()) {
            true
        } else {
            f23(v3.clone())?
        } {
            0u64
        } else {
            let a55 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h53, _) => {
                    h53
                }
            };
            let a56 = f19(match v3.clone() {
                Adt1::C0(h54, _, _, _, _, _, _, _, _, _, _) => {
                    h54
                }
            });
            let a57 = 0u64;
            nat_quot(a55, a56, a57)
        };
        Ok(Adt3::C2(Adt2::C0(f24(f25(v1.clone())?, v4, f26(match v1.clone() {
            Adt1::C0(h58, _, _, _, _, _, _, _, _, _, _) => {
                h58
            }
        }, match v2.clone() {
            Adt1::C0(h59, _, _, _, _, _, _, _, _, _, _) => {
                h59
            }
        }, match v3.clone() {
            Adt1::C0(h60, _, _, _, _, _, _, _, _, _, _) => {
                h60
            }
        })?)?, {
            let a61 = 3u64;
            let a62 = v4;
            nat_add(a61, a62)?
        }, v0, 0u64, 0u64, 0u64, v1, v2, v3)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f3(v0: Adt1, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if if if if if if if if if if if if if f27(v2, v3.clone()) {
        let a64 = match v0.clone() {
            Adt1::C0(h63, _, _, _, _, _, _, _, _, _, _) => {
                h63
            }
        };
        let a65 = 0u64;
        if nat_eq(a64, a65) {
            true
        } else {
            let a67 = match v0.clone() {
                Adt1::C0(h66, _, _, _, _, _, _, _, _, _, _) => {
                    h66
                }
            };
            let a68 = 1u64;
            nat_eq(a67, a68)
        }
    } else {
        false
    } {
        let a70 = 0u64;
        let a71 = f28(match v3.clone() {
            Adt1::C0(h69, _, _, _, _, _, _, _, _, _, _) => {
                h69
            }
        });
        nat_lt(a70, a71)
    } else {
        false
    } {
        let a73 = 0u64;
        let a74 = f29(match v1.clone() {
            Adt1::C0(h72, _, _, _, _, _, _, _, _, _, _) => {
                h72
            }
        });
        nat_lt(a73, a74)
    } else {
        false
    } {
        let a77 = match v0.clone() {
            Adt1::C0(_, h75, _, _, _, _, _, _, _, _, _) => {
                h75
            }
        };
        let a78 = match v3.clone() {
            Adt1::C0(_, h76, _, _, _, _, _, _, _, _, _) => {
                h76
            }
        };
        if nat_eq(a77, a78) {
            let a84 = {
                let a81 = match v0.clone() {
                    Adt1::C0(_, h79, _, _, _, _, _, _, _, _, _) => {
                        h79
                    }
                };
                let a82 = f30(match v3.clone() {
                    Adt1::C0(h80, _, _, _, _, _, _, _, _, _, _) => {
                        h80
                    }
                });
                let a83 = 1u64;
                nat_rem(a81, a82, a83)
            };
            let a85 = 0u64;
            nat_eq(a84, a85)
        } else {
            false
        }
    } else {
        false
    } {
        let a88 = match v0.clone() {
            Adt1::C0(_, _, _, h86, _, _, _, _, _, _, _) => {
                h86
            }
        };
        let a89 = match v3.clone() {
            Adt1::C0(_, _, _, h87, _, _, _, _, _, _, _) => {
                h87
            }
        };
        nat_eq(a88, a89)
    } else {
        false
    } {
        let a92 = match v0.clone() {
            Adt1::C0(_, _, _, _, h90, _, _, _, _, _, _) => {
                h90
            }
        };
        let a93 = match v3.clone() {
            Adt1::C0(_, _, _, _, h91, _, _, _, _, _, _) => {
                h91
            }
        };
        nat_eq(a92, a93)
    } else {
        false
    } {
        let a96 = match v0.clone() {
            Adt1::C0(_, _, h94, _, _, _, _, _, _, _, _) => {
                h94
            }
        };
        let a97 = match v1.clone() {
            Adt1::C0(_, h95, _, _, _, _, _, _, _, _, _) => {
                h95
            }
        };
        nat_eq(a96, a97)
    } else {
        false
    } {
        let a99 = match v1.clone() {
            Adt1::C0(_, _, _, _, h98, _, _, _, _, _, _) => {
                h98
            }
        };
        let a100 = 1u64;
        nat_eq(a99, a100)
    } else {
        false
    } {
        let a102 = 0u64;
        let a103 = match v1.clone() {
            Adt1::C0(_, _, h101, _, _, _, _, _, _, _, _) => {
                h101
            }
        };
        nat_lt(a102, a103)
    } else {
        false
    } {
        let a105 = 0u64;
        let a106 = match v1.clone() {
            Adt1::C0(_, _, _, h104, _, _, _, _, _, _, _) => {
                h104
            }
        };
        nat_lt(a105, a106)
    } else {
        false
    } {
        let a112 = {
            let a109 = match v0.clone() {
                Adt1::C0(_, _, _, h107, _, _, _, _, _, _, _) => {
                    h107
                }
            };
            let a110 = match v1.clone() {
                Adt1::C0(_, _, h108, _, _, _, _, _, _, _, _) => {
                    h108
                }
            };
            let a111 = 0u64;
            nat_rem(a109, a110, a111)
        };
        let a113 = 0u64;
        nat_eq(a112, a113)
    } else {
        false
    } {
        let a119 = {
            let a116 = match v0.clone() {
                Adt1::C0(_, _, _, _, h114, _, _, _, _, _, _) => {
                    h114
                }
            };
            let a117 = match v1.clone() {
                Adt1::C0(_, _, _, h115, _, _, _, _, _, _, _) => {
                    h115
                }
            };
            let a118 = 0u64;
            nat_rem(a116, a117, a118)
        };
        let a120 = 0u64;
        nat_eq(a119, a120)
    } else {
        false
    } {
        f31(v0.clone())
    } else {
        false
    } {
        let a122 = match v3.clone() {
            Adt1::C0(_, h121, _, _, _, _, _, _, _, _, _) => {
                h121
            }
        };
        let a123 = 1u64;
        if nat_eq(a122, a123) {
            true
        } else {
            let a126 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h124, _, _, _, _, _) => {
                    h124
                }
            };
            let a127 = f28(match v3.clone() {
                Adt1::C0(h125, _, _, _, _, _, _, _, _, _, _) => {
                    h125
                }
            });
            nat_eq(a126, a127)
        }
    } else {
        false
    } {
        let v4: u64 = f25(v1.clone())?;
        let v5: u64 = {
            let a133 = match v0.clone() {
                Adt1::C0(_, _, h128, _, _, _, _, _, _, _, _) => {
                    h128
                }
            };
            let a134 = {
                let a131 = match v0.clone() {
                    Adt1::C0(_, _, _, h129, _, _, _, _, _, _, _) => {
                        h129
                    }
                };
                let a132 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h130, _, _, _, _, _, _) => {
                        h130
                    }
                };
                nat_mul(a131, a132)?
            };
            nat_mul(a133, a134)?
        };
        let v6: u64 = if f18(v0.clone()) {
            0u64
        } else {
            let a142 = {
                let a135 = 2u64;
                let a136 = v4;
                nat_mul(a135, a136)?
            };
            let a143 = {
                let a140 = v5;
                let a141 = {
                    let a138 = 1u64;
                    let a139 = f32(v0.clone(), match v3.clone() {
                        Adt1::C0(h137, _, _, _, _, _, _, _, _, _, _) => {
                            h137
                        }
                    })?;
                    nat_add(a138, a139)?
                };
                nat_mul(a140, a141)?
            };
            nat_add(a142, a143)?
        };
        Ok(Adt3::C2(Adt2::C0(v6, if f18(v0.clone()) {
            3u64
        } else {
            let a144 = 3u64;
            let a145 = v4;
            nat_add(a144, a145)?
        }, 41u64, 0u64, 0u64, 0u64, v0, v1, v3)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f4(v0: Adt1, v1: Adt1, v2: Adt1) -> R<Adt3> {
    let v3: u64 = f30(match v2.clone() {
        Adt1::C0(h146, _, _, _, _, _, _, _, _, _, _) => {
            h146
        }
    });
    if if if if if if if f33(match v2.clone() {
        Adt1::C0(h147, _, _, _, _, _, _, _, _, _, _) => {
            h147
        }
    }) {
        f27(v1.clone(), v2.clone())
    } else {
        false
    } {
        let a149 = 0u64;
        let a150 = f19(match v0.clone() {
            Adt1::C0(h148, _, _, _, _, _, _, _, _, _, _) => {
                h148
            }
        });
        nat_lt(a149, a150)
    } else {
        false
    } {
        let a153 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h151, _, _, _, _, _) => {
                h151
            }
        };
        let a154 = f19(match v0.clone() {
            Adt1::C0(h152, _, _, _, _, _, _, _, _, _, _) => {
                h152
            }
        });
        nat_eq(a153, a154)
    } else {
        false
    } {
        let a159 = {
            let a156 = match v0.clone() {
                Adt1::C0(_, h155, _, _, _, _, _, _, _, _, _) => {
                    h155
                }
            };
            let a157 = v3;
            let a158 = 1u64;
            nat_rem(a156, a157, a158)
        };
        let a160 = 0u64;
        if nat_eq(a159, a160) {
            let a165 = {
                let a162 = match v2.clone() {
                    Adt1::C0(_, h161, _, _, _, _, _, _, _, _, _) => {
                        h161
                    }
                };
                let a163 = v3;
                let a164 = 1u64;
                nat_rem(a162, a163, a164)
            };
            let a166 = 0u64;
            nat_eq(a165, a166)
        } else {
            false
        }
    } else {
        false
    } {
        let a169 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h167, _, _, _, _, _) => {
                h167
            }
        };
        let a170 = f28(match v2.clone() {
            Adt1::C0(h168, _, _, _, _, _, _, _, _, _, _) => {
                h168
            }
        });
        nat_eq(a169, a170)
    } else {
        false
    } {
        let a171 = f25(v0.clone())?;
        let a172 = f25(v2.clone())?;
        nat_eq(a171, a172)
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a173 = f25(v0.clone())?;
            let a174 = v3;
            let a175 = 0u64;
            nat_quot(a173, a174, a175)
        }, 3u64, 33u64, 0u64, 0u64, 0u64, v0, v1, v2)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f5(v0: u64, v1: u64, v2: Adt1, v3: Adt1) -> R<Adt3> {
    let a177 = match v2.clone() {
        Adt1::C0(h176, _, _, _, _, _, _, _, _, _, _) => {
            h176
        }
    };
    let a178 = 0u64;
    if if if if if if if if if if if nat_eq(a177, a178) {
        let a180 = match v3.clone() {
            Adt1::C0(h179, _, _, _, _, _, _, _, _, _, _) => {
                h179
            }
        };
        let a181 = 26u64;
        nat_eq(a180, a181)
    } else {
        false
    } {
        let a183 = match v2.clone() {
            Adt1::C0(_, h182, _, _, _, _, _, _, _, _, _) => {
                h182
            }
        };
        let a184 = 2147483647u64;
        nat_le(a183, a184)
    } else {
        false
    } {
        let a187 = match v3.clone() {
            Adt1::C0(_, h185, _, _, _, _, _, _, _, _, _) => {
                h185
            }
        };
        let a188 = match v2.clone() {
            Adt1::C0(_, h186, _, _, _, _, _, _, _, _, _) => {
                h186
            }
        };
        nat_le(a187, a188)
    } else {
        false
    } {
        let a191 = match v2.clone() {
            Adt1::C0(_, _, h189, _, _, _, _, _, _, _, _) => {
                h189
            }
        };
        let a192 = match v3.clone() {
            Adt1::C0(_, _, h190, _, _, _, _, _, _, _, _) => {
                h190
            }
        };
        nat_eq(a191, a192)
    } else {
        false
    } {
        let a195 = match v2.clone() {
            Adt1::C0(_, _, _, h193, _, _, _, _, _, _, _) => {
                h193
            }
        };
        let a196 = match v3.clone() {
            Adt1::C0(_, _, _, h194, _, _, _, _, _, _, _) => {
                h194
            }
        };
        nat_eq(a195, a196)
    } else {
        false
    } {
        let a199 = match v2.clone() {
            Adt1::C0(_, _, _, _, h197, _, _, _, _, _, _) => {
                h197
            }
        };
        let a200 = match v3.clone() {
            Adt1::C0(_, _, _, _, h198, _, _, _, _, _, _) => {
                h198
            }
        };
        nat_eq(a199, a200)
    } else {
        false
    } {
        let a201 = v0;
        let a202 = 67u64;
        if nat_eq(a201, a202) {
            true
        } else {
            let a203 = v1;
            let a204 = 1u64;
            if nat_le(a203, a204) {
                let a207 = match v3.clone() {
                    Adt1::C0(_, h205, _, _, _, _, _, _, _, _, _) => {
                        h205
                    }
                };
                let a208 = match v2.clone() {
                    Adt1::C0(_, h206, _, _, _, _, _, _, _, _, _) => {
                        h206
                    }
                };
                nat_eq(a207, a208)
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
            let a214 = match v3.clone() {
                Adt1::C0(_, _, h209, _, _, _, _, _, _, _, _) => {
                    h209
                }
            };
            let a215 = {
                let a212 = match v3.clone() {
                    Adt1::C0(_, _, _, h210, _, _, _, _, _, _, _) => {
                        h210
                    }
                };
                let a213 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h211, _, _, _, _, _, _) => {
                        h211
                    }
                };
                nat_mul(a212, a213)?
            };
            nat_mul(a214, a215)?
        }, 3u64, v0, {
            let a216 = v0;
            let a217 = 67u64;
            if nat_eq(a216, a217) {
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
    let a218 = v0;
    let a219 = 3u64;
    if if nat_le(a218, a219) {
        true
    } else {
        let a220 = v0;
        let a221 = 5u64;
        nat_eq(a220, a221)
    } {
        true
    } else {
        let a222 = v0;
        let a223 = 6u64;
        nat_eq(a222, a223)
    }
}

pub fn f7(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1) -> R<Adt3> {
    let a224 = v2;
    let a225 = 1u64;
    if if if if if if if if if if if if if if if nat_le(a224, a225) {
        let a227 = 0u64;
        let a228 = f19(match v5.clone() {
            Adt1::C0(h226, _, _, _, _, _, _, _, _, _, _) => {
                h226
            }
        });
        nat_lt(a227, a228)
    } else {
        false
    } {
        let a231 = match v5.clone() {
            Adt1::C0(h229, _, _, _, _, _, _, _, _, _, _) => {
                h229
            }
        };
        let a232 = match v6.clone() {
            Adt1::C0(h230, _, _, _, _, _, _, _, _, _, _) => {
                h230
            }
        };
        nat_eq(a231, a232)
    } else {
        false
    } {
        let a235 = match v5.clone() {
            Adt1::C0(h233, _, _, _, _, _, _, _, _, _, _) => {
                h233
            }
        };
        let a236 = match v7.clone() {
            Adt1::C0(h234, _, _, _, _, _, _, _, _, _, _) => {
                h234
            }
        };
        nat_eq(a235, a236)
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
        let a239 = match v5.clone() {
            Adt1::C0(_, _, h237, _, _, _, _, _, _, _, _) => {
                h237
            }
        };
        let a240 = match v7.clone() {
            Adt1::C0(_, _, h238, _, _, _, _, _, _, _, _) => {
                h238
            }
        };
        nat_eq(a239, a240)
    } else {
        false
    } {
        let a243 = match v5.clone() {
            Adt1::C0(_, _, _, h241, _, _, _, _, _, _, _) => {
                h241
            }
        };
        let a244 = match v7.clone() {
            Adt1::C0(_, _, _, h242, _, _, _, _, _, _, _) => {
                h242
            }
        };
        nat_eq(a243, a244)
    } else {
        false
    } {
        let a247 = match v5.clone() {
            Adt1::C0(_, _, _, _, h245, _, _, _, _, _, _) => {
                h245
            }
        };
        let a248 = match v7.clone() {
            Adt1::C0(_, _, _, _, h246, _, _, _, _, _, _) => {
                h246
            }
        };
        nat_eq(a247, a248)
    } else {
        false
    } {
        let a255 = match v7.clone() {
            Adt1::C0(_, h249, _, _, _, _, _, _, _, _, _) => {
                h249
            }
        };
        let a256 = if v1 {
            match v5.clone() {
                Adt1::C0(_, h250, _, _, _, _, _, _, _, _, _) => {
                    h250
                }
            }
        } else {
            let a252 = match v5.clone() {
                Adt1::C0(_, h251, _, _, _, _, _, _, _, _, _) => {
                    h251
                }
            };
            let a253 = 2u64;
            let a254 = 0u64;
            nat_quot(a252, a253, a254)
        };
        nat_eq(a255, a256)
    } else {
        false
    } {
        let a258 = match v7.clone() {
            Adt1::C0(_, h257, _, _, _, _, _, _, _, _, _) => {
                h257
            }
        };
        let a259 = 2147483647u64;
        nat_le(a258, a259)
    } else {
        false
    } {
        f22(v5.clone(), v7.clone())?
    } else {
        false
    } {
        f22(v6.clone(), v7.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0(if f11(v0) {
            let a262 = 1u64;
            let a263 = {
                let a260 = f37(v0);
                let a261 = f25(v7.clone())?;
                nat_mul(a260, a261)?
            };
            nat_add(a262, a263)?
        } else {
            let a264 = 4u64;
            let a265 = f25(v7.clone())?;
            nat_mul(a264, a265)?
        }, if f11(v0) {
            4u64
        } else {
            3u64
        }, if v1 {
            98u64
        } else {
            let a266 = 99u64;
            let a267 = v2;
            nat_add(a266, a267)?
        }, v0, v3, v4, v5, v6, v7)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f8(v0: u64, v1: Adt2) -> R<Adt3> {
    let a269 = v0;
    let a270 = match v1.clone() {
        Adt2::C0(h268, _, _, _, _, _, _, _, _) => {
            h268
        }
    };
    if nat_lt(a269, a270) {
        let v2: Adt1 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h271) => {
                h271
            }
        };
        let v3: u64 = f30(match v2.clone() {
            Adt1::C0(h272, _, _, _, _, _, _, _, _, _, _) => {
                h272
            }
        });
        let v4: u64 = {
            let a274 = match v2.clone() {
                Adt1::C0(_, h273, _, _, _, _, _, _, _, _, _) => {
                    h273
                }
            };
            let a275 = v3;
            let a276 = 0u64;
            nat_quot(a274, a275, a276)
        };
        let v5: u64 = {
            let a277 = v0;
            let a278 = v4;
            let a279 = 0u64;
            nat_quot(a277, a278, a279)
        };
        let v6: u64 = {
            let a281 = v5;
            let a282 = match v2.clone() {
                Adt1::C0(_, _, h280, _, _, _, _, _, _, _, _) => {
                    h280
                }
            };
            let a283 = 0u64;
            nat_quot(a281, a282, a283)
        };
        Ok(Adt3::C6(f38({
            let a284 = v0;
            let a285 = v3;
            nat_mul(a284, a285)?
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, h286, _, _) => {
                h286
            }
        })?, f39({
            let a287 = v0;
            let a288 = v4;
            let a289 = 0u64;
            nat_rem(a287, a288, a289)
        }, {
            let a291 = v5;
            let a292 = match v2.clone() {
                Adt1::C0(_, _, h290, _, _, _, _, _, _, _, _) => {
                    h290
                }
            };
            let a293 = 0u64;
            nat_rem(a291, a292, a293)
        }, {
            let a295 = v6;
            let a296 = match v2.clone() {
                Adt1::C0(_, _, _, h294, _, _, _, _, _, _, _) => {
                    h294
                }
            };
            let a297 = 0u64;
            nat_rem(a295, a296, a297)
        }, {
            let a299 = v6;
            let a300 = match v2.clone() {
                Adt1::C0(_, _, _, h298, _, _, _, _, _, _, _) => {
                    h298
                }
            };
            let a301 = 0u64;
            nat_quot(a299, a300, a301)
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, h302, _, _, _, _, _) => {
                h302
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h303, _, _, _, _) => {
                h303
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h304, _, _, _) => {
                h304
            }
        }, match v2 {
            Adt1::C0(_, _, _, _, _, _, _, _, h305, _, _) => {
                h305
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f9(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h306, _, _) => {
            h306
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h307) => {
            h307
        }
    };
    let a309 = v0;
    let a310 = match v1.clone() {
        Adt2::C0(h308, _, _, _, _, _, _, _, _) => {
            h308
        }
    };
    if nat_lt(a309, a310) {
        Ok(Adt3::C5({
            let a312 = v0;
            let a313 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, _, h311, _, _, _, _) => {
                    h311
                }
            };
            nat_mul(a312, a313)?
        }, {
            let a315 = v0;
            let a316 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h314, _, _, _, _) => {
                    h314
                }
            };
            nat_mul(a315, a316)?
        }, match v2 {
            Adt1::C0(_, h317, _, _, _, _, _, _, _, _, _) => {
                h317
            }
        }, match v3 {
            Adt1::C0(_, h318, _, _, _, _, _, _, _, _, _) => {
                h318
            }
        }, {
            let a320 = match v1 {
                Adt2::C0(_, _, _, h319, _, _, _, _, _) => {
                    h319
                }
            };
            let a321 = 1u64;
            nat_eq(a320, a321)
        }))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f10(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let a323 = v0;
    let a324 = match v1.clone() {
        Adt2::C0(h322, _, _, _, _, _, _, _, _) => {
            h322
        }
    };
    if nat_lt(a323, a324) {
        let v3: u64 = {
            let a326 = 2u64;
            let a327 = f25(match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h325, _) => {
                    h325
                }
            })?;
            nat_mul(a326, a327)?
        };
        let a328 = v0;
        let a329 = v3;
        if nat_lt(a328, a329) {
            f40(v0, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h330, _) => {
                    h330
                }
            }, {
                let m333 = match v1 {
                    Adt2::C0(_, _, _, _, _, _, _, _, h332) => {
                        h332
                    }
                };
                match m333 {
                    Adt1::C0(_, _, h331, _, _, _, _, _, _, _, _) => {
                        h331
                    }
                }
            }, v2)
        } else {
            f41({
                let a334 = v0;
                let a335 = v3;
                nat_sub(a334, a335)
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, h336, _, _) => {
                    h336
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h337, _) => {
                    h337
                }
            }, match v1 {
                Adt2::C0(_, _, _, _, _, _, _, _, h338) => {
                    h338
                }
            }, v2)
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f11(v0: u64) -> bool {
    let a339 = v0;
    let a340 = 3u64;
    if nat_eq(a339, a340) {
        true
    } else {
        let a341 = v0;
        let a342 = 6u64;
        nat_eq(a341, a342)
    }
}

pub fn f12(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, h343, _, _, _) => {
            h343
        }
    };
    let a345 = v0;
    let a346 = match v1.clone() {
        Adt2::C0(h344, _, _, _, _, _, _, _, _) => {
            h344
        }
    };
    if nat_lt(a345, a346) {
        let a347 = v0;
        let a348 = 0u64;
        if nat_eq(a347, a348) {
            Ok(Adt3::C3(1u64, 6u64, 0u64, 0u64, 2u64, v3, 2u64, 0u64, 0u64, 3u64, 4u64, 4u64))
        } else {
            let v4: u64 = {
                let a349 = v0;
                let a350 = 1u64;
                nat_sub(a349, a350)
            };
            let v5: u64 = {
                let a352 = v4;
                let a353 = f37(match v1.clone() {
                    Adt2::C0(_, _, _, h351, _, _, _, _, _) => {
                        h351
                    }
                });
                let a354 = 0u64;
                nat_rem(a352, a353, a354)
            };
            let v6: u64 = {
                let a359 = 4u64;
                let a360 = {
                    let a356 = v4;
                    let a357 = f37(match v1.clone() {
                        Adt2::C0(_, _, _, h355, _, _, _, _, _) => {
                            h355
                        }
                    });
                    let a358 = 0u64;
                    nat_quot(a356, a357, a358)
                };
                nat_mul(a359, a360)?
            };
            let a361 = v5;
            let a362 = 2u64;
            if nat_lt(a361, a362) {
                f13({
                    let a363 = v6;
                    let a364 = v5;
                    nat_add(a363, a364)?
                }, v1)
            } else {
                let a365 = v5;
                let a366 = 2u64;
                if nat_eq(a365, a366) {
                    Ok(f42(0u64, 2u64, v3))
                } else {
                    let a367 = v5;
                    let a368 = 3u64;
                    if nat_eq(a367, a368) {
                        Ok(f43(0u64, 2u64, v3, {
                            let a369 = v2;
                            let a370 = 2u64;
                            nat_eq(a369, a370)
                        }))
                    } else {
                        let a371 = v5;
                        let a372 = 4u64;
                        if nat_eq(a371, a372) {
                            Ok(f42(1u64, 1u64, 3u64))
                        } else {
                            let a373 = v5;
                            let a374 = 5u64;
                            if nat_eq(a373, a374) {
                                Ok(f43(1u64, 1u64, 3u64, {
                                    let a375 = v2;
                                    let a376 = 0u64;
                                    nat_eq(a375, a376)
                                }))
                            } else {
                                let a377 = v5;
                                let a378 = 6u64;
                                if nat_eq(a377, a378) {
                                    Ok(f42(1u64, 2u64, v3))
                                } else {
                                    let a379 = v5;
                                    let a380 = 7u64;
                                    if nat_eq(a379, a380) {
                                        Ok(f43(1u64, 2u64, v3, {
                                            let a381 = v2;
                                            let a382 = 2u64;
                                            nat_eq(a381, a382)
                                        }))
                                    } else {
                                        let a384 = match v1.clone() {
                                            Adt2::C0(_, _, _, h383, _, _, _, _, _) => {
                                                h383
                                            }
                                        };
                                        let a385 = 3u64;
                                        if nat_eq(a384, a385) {
                                            let a386 = v5;
                                            let a387 = 15u64;
                                            if nat_eq(a386, a387) {
                                                f13({
                                                    let a388 = v6;
                                                    let a389 = 3u64;
                                                    nat_add(a388, a389)?
                                                }, v1)
                                            } else {
                                                Ok(f44(v5, match v1 {
                                                    Adt2::C0(_, _, _, _, h390, _, _, _, _) => {
                                                        h390
                                                    }
                                                }))
                                            }
                                        } else {
                                            let a391 = v5;
                                            let a392 = 8u64;
                                            if nat_eq(a391, a392) {
                                                Ok(Adt3::C4(2u64, {
                                                    let m395 = match v1 {
                                                        Adt2::C0(_, _, _, _, _, _, _, _, h394) => {
                                                            h394
                                                        }
                                                    };
                                                    match m395 {
                                                        Adt1::C0(h393, _, _, _, _, _, _, _, _, _, _) => {
                                                            h393
                                                        }
                                                    }
                                                }, 0u64, 1u64, 0u64))
                                            } else {
                                                f13({
                                                    let a396 = v6;
                                                    let a397 = 3u64;
                                                    nat_add(a396, a397)?
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
        Adt2::C0(_, _, _, _, _, _, h398, _, _) => {
            h398
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h399, _) => {
            h399
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h400) => {
            h400
        }
    };
    let v5: u64 = match v1.clone() {
        Adt2::C0(_, _, h401, _, _, _, _, _, _) => {
            h401
        }
    };
    let v6: u64 = {
        let a402 = v0;
        let a403 = 4u64;
        let a404 = 0u64;
        nat_rem(a402, a403, a404)
    };
    let v7: u64 = {
        let a405 = v0;
        let a406 = 4u64;
        let a407 = 0u64;
        nat_quot(a405, a406, a407)
    };
    let v8: u64 = {
        let a409 = v7;
        let a410 = match v4.clone() {
            Adt1::C0(_, h408, _, _, _, _, _, _, _, _, _) => {
                h408
            }
        };
        let a411 = 0u64;
        nat_quot(a409, a410, a411)
    };
    let v9: u64 = f19(match v4.clone() {
        Adt1::C0(h412, _, _, _, _, _, _, _, _, _, _) => {
            h412
        }
    });
    let v10: u64 = {
        let a417 = {
            let a414 = v7;
            let a415 = match v4.clone() {
                Adt1::C0(_, h413, _, _, _, _, _, _, _, _, _) => {
                    h413
                }
            };
            let a416 = 0u64;
            nat_rem(a414, a415, a416)
        };
        let a418 = v9;
        nat_mul(a417, a418)?
    };
    let v11: u64 = {
        let a420 = match v4.clone() {
            Adt1::C0(_, h419, _, _, _, _, _, _, _, _, _) => {
                h419
            }
        };
        let a421 = v9;
        nat_mul(a420, a421)?
    };
    let a423 = v0;
    let a424 = match v1.clone() {
        Adt2::C0(h422, _, _, _, _, _, _, _, _) => {
            h422
        }
    };
    if nat_lt(a423, a424) {
        let a425 = v6;
        let a426 = 0u64;
        if nat_eq(a425, a426) {
            Ok(Adt3::C3(0u64, 0u64, match v2.clone() {
                Adt1::C0(h427, _, _, _, _, _, _, _, _, _, _) => {
                    h427
                }
            }, 0u64, 0u64, {
                let a435 = {
                    let a431 = {
                        let a429 = v8;
                        let a430 = match v2 {
                            Adt1::C0(_, _, _, _, _, _, h428, _, _, _, _) => {
                                h428
                            }
                        };
                        nat_mul(a429, a430)?
                    };
                    let a432 = v10;
                    nat_add(a431, a432)?
                };
                let a436 = {
                    let a433 = v5;
                    let a434 = 100u64;
                    if nat_eq(a433, a434) {
                        v11
                    } else {
                        0u64
                    }
                };
                nat_add(a435, a436)?
            }, 2u64, 0u64, 0u64, 0u64, v9, 4u64))
        } else {
            let a437 = v6;
            let a438 = 1u64;
            if nat_eq(a437, a438) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h439, _, _, _, _, _, _, _, _, _, _) => {
                        h439
                    }
                }, 0u64, {
                    let a440 = v5;
                    let a441 = 98u64;
                    if nat_eq(a440, a441) {
                        4u64
                    } else {
                        0u64
                    }
                }, {
                    let a449 = {
                        let a445 = {
                            let a443 = v8;
                            let a444 = match v3 {
                                Adt1::C0(_, _, _, _, _, _, h442, _, _, _, _) => {
                                    h442
                                }
                            };
                            nat_mul(a443, a444)?
                        };
                        let a446 = v10;
                        nat_add(a445, a446)?
                    };
                    let a450 = {
                        let a447 = v5;
                        let a448 = 99u64;
                        if nat_eq(a447, a448) {
                            v11
                        } else {
                            0u64
                        }
                    };
                    nat_add(a449, a450)?
                }, 2u64, 0u64, 0u64, 1u64, v9, 4u64))
            } else {
                let a451 = v6;
                let a452 = 2u64;
                if nat_eq(a451, a452) {
                    Ok(Adt3::C4(match v1 {
                        Adt2::C0(_, _, _, h453, _, _, _, _, _) => {
                            h453
                        }
                    }, match v4 {
                        Adt1::C0(h454, _, _, _, _, _, _, _, _, _, _) => {
                            h454
                        }
                    }, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt3::C3(0u64, 0u64, 0u64, match v4.clone() {
                        Adt1::C0(h455, _, _, _, _, _, _, _, _, _, _) => {
                            h455
                        }
                    }, 1u64, 0u64, 2u64, 0u64, 3u64, {
                        let a459 = {
                            let a457 = v8;
                            let a458 = match v4 {
                                Adt1::C0(_, _, _, _, _, _, h456, _, _, _, _) => {
                                    h456
                                }
                            };
                            nat_mul(a457, a458)?
                        };
                        let a460 = v10;
                        nat_add(a459, a460)?
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
        Adt2::C0(_, _, _, _, _, _, h461, _, _) => {
            h461
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h462, _) => {
            h462
        }
    };
    let v5: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h463) => {
            h463
        }
    };
    let v6: u64 = {
        let a465 = match v1.clone() {
            Adt2::C0(_, h464, _, _, _, _, _, _, _) => {
                h464
            }
        };
        let a466 = 3u64;
        if nat_eq(a465, a466) {
            0u64
        } else {
            let a467 = f25(v3.clone())?;
            let a468 = 2u64;
            nat_mul(a467, a468)?
        }
    };
    let a470 = match v1.clone() {
        Adt2::C0(h469, _, _, _, _, _, _, _, _) => {
            h469
        }
    };
    let a471 = v0;
    if nat_le(a470, a471) {
        Ok(Adt3::C1)
    } else {
        let a472 = v0;
        let a473 = v6;
        if nat_lt(a472, a473) {
            let v7: u64 = {
                let a474 = v0;
                let a475 = 2u64;
                let a476 = 0u64;
                nat_quot(a474, a475, a476)
            };
            let v8: u64 = {
                let a481 = 3u64;
                let a482 = {
                    let a478 = f38(v7, v5.clone())?;
                    let a479 = f19(match v5 {
                        Adt1::C0(h477, _, _, _, _, _, _, _, _, _, _) => {
                            h477
                        }
                    });
                    let a480 = 0u64;
                    nat_quot(a478, a479, a480)
                };
                nat_add(a481, a482)?
            };
            let a486 = {
                let a483 = v0;
                let a484 = 2u64;
                let a485 = 0u64;
                nat_rem(a483, a484, a485)
            };
            let a487 = 0u64;
            if nat_eq(a486, a487) {
                Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, v8, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
            } else {
                let a488 = v2;
                let a489 = 0u64;
                if nat_eq(a488, a489) {
                    Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 2u64, 1u64, 2u64, 0u64, 0u64, v8, 8u64, 8u64))
                } else {
                    Ok(Adt3::C1)
                }
            }
        } else {
            let v9: u64 = {
                let a490 = v0;
                let a491 = v6;
                nat_sub(a490, a491)
            };
            let v10: u64 = f26(match v3.clone() {
                Adt1::C0(h492, _, _, _, _, _, _, _, _, _, _) => {
                    h492
                }
            }, match v4.clone() {
                Adt1::C0(h493, _, _, _, _, _, _, _, _, _, _) => {
                    h493
                }
            }, match v5.clone() {
                Adt1::C0(h494, _, _, _, _, _, _, _, _, _, _) => {
                    h494
                }
            })?;
            let v11: u64 = {
                let a495 = v9;
                let a496 = v10;
                let a497 = 0u64;
                nat_quot(a495, a496, a497)
            };
            let v12: u64 = {
                let a498 = v9;
                let a499 = v10;
                let a500 = 0u64;
                nat_rem(a498, a499, a500)
            };
            let v13: u64 = {
                let a502 = match v3.clone() {
                    Adt1::C0(h501, _, _, _, _, _, _, _, _, _, _) => {
                        h501
                    }
                };
                let a503 = 0u64;
                if nat_eq(a502, a503) {
                    0u64
                } else {
                    1u64
                }
            };
            let v14: u64 = {
                let a505 = match v4.clone() {
                    Adt1::C0(h504, _, _, _, _, _, _, _, _, _, _) => {
                        h504
                    }
                };
                let a506 = 0u64;
                if nat_eq(a505, a506) {
                    0u64
                } else {
                    1u64
                }
            };
            let a507 = v12;
            let a508 = v13;
            if nat_lt(a507, a508) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h509, _, _, _, _, _, _, _, _, _, _) => {
                        h509
                    }
                }, 0u64, 0u64, f38(v11, v3.clone())?, 2u64, 0u64, 0u64, 0u64, f19(match v3 {
                    Adt1::C0(h510, _, _, _, _, _, _, _, _, _, _) => {
                        h510
                    }
                }), 4u64))
            } else {
                let a513 = v12;
                let a514 = {
                    let a511 = v13;
                    let a512 = v14;
                    nat_add(a511, a512)?
                };
                if nat_lt(a513, a514) {
                    Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                        Adt1::C0(h515, _, _, _, _, _, _, _, _, _, _) => {
                            h515
                        }
                    }, 0u64, 4u64, f45(v11, v3, v4.clone())?, 2u64, 0u64, 0u64, 1u64, f19(match v4 {
                        Adt1::C0(h516, _, _, _, _, _, _, _, _, _, _) => {
                            h516
                        }
                    }), 4u64))
                } else {
                    let a519 = v12;
                    let a520 = {
                        let a517 = v13;
                        let a518 = v14;
                        nat_add(a517, a518)?
                    };
                    if nat_eq(a519, a520) {
                        Ok(Adt3::C3(1u64, match v1 {
                            Adt2::C0(_, _, h521, _, _, _, _, _, _) => {
                                h521
                            }
                        }, 0u64, 0u64, {
                            let a522 = v13;
                            let a523 = 0u64;
                            if nat_eq(a522, a523) {
                                0u64
                            } else {
                                1u64
                            }
                        }, {
                            let a524 = v13;
                            let a525 = 0u64;
                            if nat_eq(a524, a525) {
                                f38(v11, v3.clone())?
                            } else {
                                0u64
                            }
                        }, {
                            let a526 = v14;
                            let a527 = 0u64;
                            if nat_eq(a526, a527) {
                                4u64
                            } else {
                                1u64
                            }
                        }, {
                            let a528 = v14;
                            let a529 = 0u64;
                            if nat_eq(a528, a529) {
                                f45(v11, v3, v4)?
                            } else {
                                1u64
                            }
                        }, {
                            let a531 = match v5.clone() {
                                Adt1::C0(h530, _, _, _, _, _, _, _, _, _, _) => {
                                    h530
                                }
                            };
                            let a532 = 0u64;
                            if nat_eq(a531, a532) {
                                3u64
                            } else {
                                0u64
                            }
                        }, {
                            let a534 = match v5.clone() {
                                Adt1::C0(h533, _, _, _, _, _, _, _, _, _, _) => {
                                    h533
                                }
                            };
                            let a535 = 0u64;
                            if nat_eq(a534, a535) {
                                f38(v11, v5)?
                            } else {
                                0u64
                            }
                        }, 4u64, 4u64))
                    } else {
                        Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                            Adt1::C0(h536, _, _, _, _, _, _, _, _, _, _) => {
                                h536
                            }
                        }, 1u64, 0u64, 2u64, 0u64, 3u64, f38(v11, v5.clone())?, 4u64, f19(match v5 {
                            Adt1::C0(h537, _, _, _, _, _, _, _, _, _, _) => {
                                h537
                            }
                        })))
                    }
                }
            }
        }
    }
}

pub fn f15(v0: Adt1, v1: Adt1) -> bool {
    let a540 = match v0.clone() {
        Adt1::C0(_, h538, _, _, _, _, _, _, _, _, _) => {
            h538
        }
    };
    let a541 = match v1.clone() {
        Adt1::C0(_, h539, _, _, _, _, _, _, _, _, _) => {
            h539
        }
    };
    if if if nat_eq(a540, a541) {
        let a544 = match v0.clone() {
            Adt1::C0(_, _, h542, _, _, _, _, _, _, _, _) => {
                h542
            }
        };
        let a545 = match v1.clone() {
            Adt1::C0(_, _, h543, _, _, _, _, _, _, _, _) => {
                h543
            }
        };
        nat_eq(a544, a545)
    } else {
        false
    } {
        let a548 = match v0.clone() {
            Adt1::C0(_, _, _, h546, _, _, _, _, _, _, _) => {
                h546
            }
        };
        let a549 = match v1.clone() {
            Adt1::C0(_, _, _, h547, _, _, _, _, _, _, _) => {
                h547
            }
        };
        nat_eq(a548, a549)
    } else {
        false
    } {
        let a552 = match v0 {
            Adt1::C0(_, _, _, _, h550, _, _, _, _, _, _) => {
                h550
            }
        };
        let a553 = match v1 {
            Adt1::C0(_, _, _, _, h551, _, _, _, _, _, _) => {
                h551
            }
        };
        nat_eq(a552, a553)
    } else {
        false
    }
}

pub fn f16(v0: Adt1, v1: Adt1) -> bool {
    if f18(v1.clone()) {
        f18(v0)
    } else {
        let a559 = {
            let a556 = match v0.clone() {
                Adt1::C0(_, h554, _, _, _, _, _, _, _, _, _) => {
                    h554
                }
            };
            let a557 = match v1.clone() {
                Adt1::C0(_, h555, _, _, _, _, _, _, _, _, _) => {
                    h555
                }
            };
            let a558 = 1u64;
            nat_rem(a556, a557, a558)
        };
        let a560 = 0u64;
        if if if nat_eq(a559, a560) {
            let a566 = {
                let a563 = match v0.clone() {
                    Adt1::C0(_, _, h561, _, _, _, _, _, _, _, _) => {
                        h561
                    }
                };
                let a564 = match v1.clone() {
                    Adt1::C0(_, _, h562, _, _, _, _, _, _, _, _) => {
                        h562
                    }
                };
                let a565 = 1u64;
                nat_rem(a563, a564, a565)
            };
            let a567 = 0u64;
            nat_eq(a566, a567)
        } else {
            false
        } {
            let a573 = {
                let a570 = match v0.clone() {
                    Adt1::C0(_, _, _, h568, _, _, _, _, _, _, _) => {
                        h568
                    }
                };
                let a571 = match v1.clone() {
                    Adt1::C0(_, _, _, h569, _, _, _, _, _, _, _) => {
                        h569
                    }
                };
                let a572 = 1u64;
                nat_rem(a570, a571, a572)
            };
            let a574 = 0u64;
            nat_eq(a573, a574)
        } else {
            false
        } {
            let a580 = {
                let a577 = match v0 {
                    Adt1::C0(_, _, _, _, h575, _, _, _, _, _, _) => {
                        h575
                    }
                };
                let a578 = match v1 {
                    Adt1::C0(_, _, _, _, h576, _, _, _, _, _, _) => {
                        h576
                    }
                };
                let a579 = 1u64;
                nat_rem(a577, a578, a579)
            };
            let a581 = 0u64;
            nat_eq(a580, a581)
        } else {
            false
        }
    }
}

pub fn f17(v0: u64, v1: u64, v2: u64) -> bool {
    let a582 = v0;
    let a583 = 0u64;
    if if if nat_eq(a582, a583) {
        let a584 = v1;
        let a585 = 0u64;
        nat_eq(a584, a585)
    } else {
        false
    } {
        let a586 = v2;
        let a587 = 0u64;
        nat_eq(a586, a587)
    } else {
        false
    } {
        true
    } else {
        let a588 = v0;
        let a589 = 1u64;
        if if nat_eq(a588, a589) {
            true
        } else {
            let a590 = v0;
            let a591 = 30u64;
            nat_eq(a590, a591)
        } {
            let a592 = v1;
            let a593 = v0;
            if if nat_eq(a592, a593) {
                let a594 = v2;
                let a595 = v0;
                nat_eq(a594, a595)
            } else {
                false
            } {
                true
            } else {
                let a596 = v1;
                let a597 = 0u64;
                if nat_eq(a596, a597) {
                    let a598 = v2;
                    let a599 = v0;
                    if nat_eq(a598, a599) {
                        true
                    } else {
                        let a600 = v2;
                        let a601 = 0u64;
                        nat_eq(a600, a601)
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

pub fn f18(v0: Adt1) -> bool {
    f46(match v0.clone() {
        Adt1::C0(_, h602, _, _, _, _, _, _, _, _, _) => {
            h602
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h603, _, _, _, _, _, _, _, _) => {
            h603
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h604, _, _, _, _, _, _, _) => {
            h604
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, h605, _, _, _, _, _, _) => {
            h605
        }
    })
}

pub fn f19(v0: u64) -> u64 {
    let a606 = v0;
    let a607 = 0u64;
    if nat_eq(a606, a607) {
        4u64
    } else {
        let a608 = v0;
        let a609 = 1u64;
        if if nat_eq(a608, a609) {
            true
        } else {
            let a610 = v0;
            let a611 = 30u64;
            nat_eq(a610, a611)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f20(v0: Adt1) -> bool {
    let v1: u64 = f19(match v0.clone() {
        Adt1::C0(h612, _, _, _, _, _, _, _, _, _, _) => {
            h612
        }
    });
    let a617 = {
        let a614 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h613) => {
                h613
            }
        };
        let a615 = v1;
        let a616 = 1u64;
        nat_rem(a614, a615, a616)
    };
    let a618 = 0u64;
    if if nat_eq(a617, a618) {
        f47(match v0.clone() {
            Adt1::C0(_, h619, _, _, _, _, _, _, _, _, _) => {
                h619
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h620, _, _, _, _, _) => {
                h620
            }
        }, v1)
    } else {
        false
    } {
        f48(match v0.clone() {
            Adt1::C0(_, _, h621, _, _, _, _, _, _, _, _) => {
                h621
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h622, _, _, _, _, _, _, _) => {
                h622
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h623, _, _, _, _, _, _) => {
                h623
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h624, _, _, _, _) => {
                h624
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h625, _, _, _) => {
                h625
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h626, _, _) => {
                h626
            }
        }, v1)
    } else {
        false
    }
}

pub fn f21(v0: Adt1, v1: Adt1) -> R<bool> {
    let a634 = {
        let a632 = match v0.clone() {
            Adt1::C0(_, _, h627, _, _, _, _, _, _, _, _) => {
                h627
            }
        };
        let a633 = {
            let a630 = match v0.clone() {
                Adt1::C0(_, _, _, h628, _, _, _, _, _, _, _) => {
                    h628
                }
            };
            let a631 = match v0.clone() {
                Adt1::C0(_, _, _, _, h629, _, _, _, _, _, _) => {
                    h629
                }
            };
            nat_mul(a630, a631)?
        };
        nat_mul(a632, a633)?
    };
    let a635 = 9223372036854775807u64;
    if nat_le(a634, a635) {
        if f31(v1.clone()) {
            let a637 = match v1 {
                Adt1::C0(_, h636, _, _, _, _, _, _, _, _, _) => {
                    h636
                }
            };
            let a638 = 2147483647u64;
            Ok(nat_le(a637, a638))
        } else {
            let a640 = match v0 {
                Adt1::C0(_, h639, _, _, _, _, _, _, _, _, _) => {
                    h639
                }
            };
            let a641 = 2147483647u64;
            Ok(nat_le(a640, a641))
        }
    } else {
        Ok(false)
    }
}

pub fn f22(v0: Adt1, v1: Adt1) -> R<bool> {
    if f35(v0.clone(), v1.clone())? {
        Ok(true)
    } else {
        Ok(f49(v0, v1))
    }
}

pub fn f23(v0: Adt1) -> R<bool> {
    f50({
        let a644 = match v0.clone() {
            Adt1::C0(_, h642, _, _, _, _, _, _, _, _, _) => {
                h642
            }
        };
        let a645 = f19(match v0.clone() {
            Adt1::C0(h643, _, _, _, _, _, _, _, _, _, _) => {
                h643
            }
        });
        nat_mul(a644, a645)?
    }, match v0.clone() {
        Adt1::C0(_, _, h646, _, _, _, _, _, _, _, _) => {
            h646
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h647, _, _, _, _, _, _, _) => {
            h647
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h648, _, _, _, _, _, _) => {
            h648
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h649, _, _, _, _) => {
            h649
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h650, _, _, _) => {
            h650
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h651, _, _) => {
            h651
        }
    })
}

pub fn f24(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a658 = {
        let a652 = v0;
        let a653 = v2;
        nat_mul(a652, a653)?
    };
    let a659 = {
        let a654 = v1;
        let a655 = 0u64;
        if nat_eq(a654, a655) {
            0u64
        } else {
            let a656 = v0;
            let a657 = 2u64;
            nat_mul(a656, a657)?
        }
    };
    nat_add(a658, a659)
}

pub fn f25(v0: Adt1) -> R<u64> {
    let a668 = {
        let a662 = match v0.clone() {
            Adt1::C0(_, h660, _, _, _, _, _, _, _, _, _) => {
                h660
            }
        };
        let a663 = match v0.clone() {
            Adt1::C0(_, _, h661, _, _, _, _, _, _, _, _) => {
                h661
            }
        };
        nat_mul(a662, a663)?
    };
    let a669 = {
        let a666 = match v0.clone() {
            Adt1::C0(_, _, _, h664, _, _, _, _, _, _, _) => {
                h664
            }
        };
        let a667 = match v0 {
            Adt1::C0(_, _, _, _, h665, _, _, _, _, _, _) => {
                h665
            }
        };
        nat_mul(a666, a667)?
    };
    nat_mul(a668, a669)
}

pub fn f26(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a680 = {
        let a676 = {
            let a672 = 1u64;
            let a673 = {
                let a670 = v0;
                let a671 = 0u64;
                if nat_eq(a670, a671) {
                    0u64
                } else {
                    1u64
                }
            };
            nat_add(a672, a673)?
        };
        let a677 = {
            let a674 = v1;
            let a675 = 0u64;
            if nat_eq(a674, a675) {
                0u64
            } else {
                1u64
            }
        };
        nat_add(a676, a677)?
    };
    let a681 = {
        let a678 = v2;
        let a679 = 0u64;
        if nat_eq(a678, a679) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a680, a681)
}

pub fn f27(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f15(v0.clone(), v1.clone()) {
        let a684 = match v0.clone() {
            Adt1::C0(h682, _, _, _, _, _, _, _, _, _, _) => {
                h682
            }
        };
        let a685 = match v1.clone() {
            Adt1::C0(h683, _, _, _, _, _, _, _, _, _, _) => {
                h683
            }
        };
        nat_eq(a684, a685)
    } else {
        false
    } {
        let a688 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h686, _, _, _, _, _) => {
                h686
            }
        };
        let a689 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h687, _, _, _, _, _) => {
                h687
            }
        };
        nat_eq(a688, a689)
    } else {
        false
    } {
        let a692 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h690, _, _, _, _) => {
                h690
            }
        };
        let a693 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h691, _, _, _, _) => {
                h691
            }
        };
        nat_eq(a692, a693)
    } else {
        false
    } {
        let a696 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h694, _, _, _) => {
                h694
            }
        };
        let a697 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h695, _, _, _) => {
                h695
            }
        };
        nat_eq(a696, a697)
    } else {
        false
    } {
        let a700 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h698, _, _) => {
                h698
            }
        };
        let a701 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h699, _, _) => {
                h699
            }
        };
        nat_eq(a700, a701)
    } else {
        false
    } {
        let a703 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h702) => {
                h702
            }
        };
        let a704 = 0u64;
        if if nat_eq(a703, a704) {
            true
        } else {
            let a706 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h705) => {
                    h705
                }
            };
            let a707 = 0u64;
            nat_eq(a706, a707)
        } {
            true
        } else {
            let a710 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h708) => {
                    h708
                }
            };
            let a711 = match v1 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h709) => {
                    h709
                }
            };
            nat_eq(a710, a711)
        }
    } else {
        false
    }
}

pub fn f28(v0: u64) -> u64 {
    if f33(v0) {
        let m712 = f51(v0);
        match m712 {
            Adt4::C0 => {
                0u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, _, h713) => {
                        h713
                    }
                }
            }
        }
    } else {
        let a714 = v0;
        let a715 = 43u64;
        if nat_eq(a714, a715) {
            1u64
        } else {
            f19(v0)
        }
    }
}

pub fn f29(v0: u64) -> u64 {
    let a716 = v0;
    let a717 = 26u64;
    if nat_eq(a716, a717) {
        4u64
    } else {
        let a718 = v0;
        let a719 = 27u64;
        if nat_eq(a718, a719) {
            8u64
        } else {
            0u64
        }
    }
}

pub fn f30(v0: u64) -> u64 {
    if f33(v0) {
        let m720 = f51(v0);
        match m720 {
            Adt4::C0 => {
                1u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, h721, _) => {
                        h721
                    }
                }
            }
        }
    } else {
        1u64
    }
}

pub fn f31(v0: Adt1) -> bool {
    let a723 = match v0.clone() {
        Adt1::C0(_, h722, _, _, _, _, _, _, _, _, _) => {
            h722
        }
    };
    let a724 = 1u64;
    if nat_eq(a723, a724) {
        true
    } else {
        let a727 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h725, _, _, _, _, _) => {
                h725
            }
        };
        let a728 = f19(match v0 {
            Adt1::C0(h726, _, _, _, _, _, _, _, _, _, _) => {
                h726
            }
        });
        nat_eq(a727, a728)
    }
}

pub fn f32(v0: Adt1, v1: u64) -> R<u64> {
    let a734 = {
        let a730 = match v0.clone() {
            Adt1::C0(_, h729, _, _, _, _, _, _, _, _, _) => {
                h729
            }
        };
        let a731 = f30(v1);
        let a732 = 0u64;
        nat_quot(a730, a731, a732)
    };
    let a735 = f52(match v0 {
        Adt1::C0(h733, _, _, _, _, _, _, _, _, _, _) => {
            h733
        }
    }, v1);
    nat_mul(a734, a735)
}

pub fn f33(v0: u64) -> bool {
    let a736 = v0;
    let a737 = 2u64;
    if if if if nat_eq(a736, a737) {
        true
    } else {
        let a738 = v0;
        let a739 = 3u64;
        nat_eq(a738, a739)
    } {
        true
    } else {
        let a740 = v0;
        let a741 = 6u64;
        nat_eq(a740, a741)
    } {
        true
    } else {
        let a742 = v0;
        let a743 = 7u64;
        if nat_eq(a742, a743) {
            true
        } else {
            let a744 = v0;
            let a745 = 8u64;
            nat_eq(a744, a745)
        }
    } {
        true
    } else {
        let a746 = v0;
        let a747 = 41u64;
        if nat_eq(a746, a747) {
            true
        } else {
            let a748 = v0;
            let a749 = 42u64;
            nat_eq(a748, a749)
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
    if if f31(v0.clone()) {
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
    let v5: u64 = f29(match v1.clone() {
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
        let a907 = f32(v1.clone(), match v3.clone() {
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
                let a960 = f52(match v1.clone() {
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
                let a964 = f30(match v3.clone() {
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
            if f33(match v3.clone() {
                Adt1::C0(h973, _, _, _, _, _, _, _, _, _, _) => {
                    h973
                }
            }) {
                Ok(Adt3::C6(v14, v15))
            } else {
                Ok(f53(match v1 {
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

pub fn f46(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1036 = v0;
    let a1037 = 0u64;
    if if if nat_eq(a1036, a1037) {
        true
    } else {
        let a1038 = v1;
        let a1039 = 0u64;
        nat_eq(a1038, a1039)
    } {
        true
    } else {
        let a1040 = v2;
        let a1041 = 0u64;
        nat_eq(a1040, a1041)
    } {
        true
    } else {
        let a1042 = v3;
        let a1043 = 0u64;
        nat_eq(a1042, a1043)
    }
}

pub fn f47(v0: u64, v1: u64, v2: u64) -> bool {
    let a1044 = v0;
    let a1045 = 1u64;
    if nat_le(a1044, a1045) {
        true
    } else {
        let a1049 = {
            let a1046 = v1;
            let a1047 = v2;
            let a1048 = 1u64;
            nat_rem(a1046, a1047, a1048)
        };
        let a1050 = 0u64;
        nat_eq(a1049, a1050)
    }
}

pub fn f48(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f47(v0, v3, v6) {
        f47(v1, v4, v6)
    } else {
        false
    } {
        f47(v2, v5, v6)
    } else {
        false
    }
}

pub fn f49(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f15(v0.clone(), v1.clone()) {
        let a1053 = match v0.clone() {
            Adt1::C0(h1051, _, _, _, _, _, _, _, _, _, _) => {
                h1051
            }
        };
        let a1054 = match v1.clone() {
            Adt1::C0(h1052, _, _, _, _, _, _, _, _, _, _) => {
                h1052
            }
        };
        nat_eq(a1053, a1054)
    } else {
        false
    } {
        let a1057 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1055) => {
                h1055
            }
        };
        let a1058 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1056) => {
                h1056
            }
        };
        nat_eq(a1057, a1058)
    } else {
        false
    } {
        let a1061 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1059, _, _, _, _, _) => {
                h1059
            }
        };
        let a1062 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1060, _, _, _, _, _) => {
                h1060
            }
        };
        nat_eq(a1061, a1062)
    } else {
        false
    } {
        let a1065 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1063, _, _, _, _) => {
                h1063
            }
        };
        let a1066 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1064, _, _, _, _) => {
                h1064
            }
        };
        nat_eq(a1065, a1066)
    } else {
        false
    } {
        let a1069 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1067, _, _, _) => {
                h1067
            }
        };
        let a1070 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1068, _, _, _) => {
                h1068
            }
        };
        nat_eq(a1069, a1070)
    } else {
        false
    } {
        let a1073 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1071, _, _) => {
                h1071
            }
        };
        let a1074 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1072, _, _) => {
                h1072
            }
        };
        nat_eq(a1073, a1074)
    } else {
        false
    }
}

pub fn f50(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f54(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f54(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f54(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f54(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f54(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f54(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f51(v0: u64) -> Adt4 {
    let a1075 = v0;
    let a1076 = 0u64;
    if nat_eq(a1075, a1076) {
        Adt4::C1(Adt5::C0(0u64, 1u64, 4u64))
    } else {
        let a1077 = v0;
        let a1078 = 1u64;
        if nat_eq(a1077, a1078) {
            Adt4::C1(Adt5::C0(1u64, 1u64, 2u64))
        } else {
            let a1079 = v0;
            let a1080 = 2u64;
            if nat_eq(a1079, a1080) {
                Adt4::C1(Adt5::C0(2u64, 32u64, 18u64))
            } else {
                let a1081 = v0;
                let a1082 = 3u64;
                if nat_eq(a1081, a1082) {
                    Adt4::C1(Adt5::C0(3u64, 32u64, 20u64))
                } else {
                    let a1083 = v0;
                    let a1084 = 6u64;
                    if nat_eq(a1083, a1084) {
                        Adt4::C1(Adt5::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1085 = v0;
                        let a1086 = 7u64;
                        if nat_eq(a1085, a1086) {
                            Adt4::C1(Adt5::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1087 = v0;
                            let a1088 = 8u64;
                            if nat_eq(a1087, a1088) {
                                Adt4::C1(Adt5::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1089 = v0;
                                let a1090 = 9u64;
                                if nat_eq(a1089, a1090) {
                                    Adt4::C1(Adt5::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1091 = v0;
                                    let a1092 = 10u64;
                                    if nat_eq(a1091, a1092) {
                                        Adt4::C1(Adt5::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1093 = v0;
                                        let a1094 = 11u64;
                                        if nat_eq(a1093, a1094) {
                                            Adt4::C1(Adt5::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1095 = v0;
                                            let a1096 = 12u64;
                                            if nat_eq(a1095, a1096) {
                                                Adt4::C1(Adt5::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1097 = v0;
                                                let a1098 = 13u64;
                                                if nat_eq(a1097, a1098) {
                                                    Adt4::C1(Adt5::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1099 = v0;
                                                    let a1100 = 14u64;
                                                    if nat_eq(a1099, a1100) {
                                                        Adt4::C1(Adt5::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1101 = v0;
                                                        let a1102 = 15u64;
                                                        if nat_eq(a1101, a1102) {
                                                            Adt4::C1(Adt5::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1103 = v0;
                                                            let a1104 = 16u64;
                                                            if nat_eq(a1103, a1104) {
                                                                Adt4::C1(Adt5::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1105 = v0;
                                                                let a1106 = 17u64;
                                                                if nat_eq(a1105, a1106) {
                                                                    Adt4::C1(Adt5::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1107 = v0;
                                                                    let a1108 = 18u64;
                                                                    if nat_eq(a1107, a1108) {
                                                                        Adt4::C1(Adt5::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1109 = v0;
                                                                        let a1110 = 19u64;
                                                                        if nat_eq(a1109, a1110) {
                                                                            Adt4::C1(Adt5::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1111 = v0;
                                                                            let a1112 = 20u64;
                                                                            if nat_eq(a1111, a1112) {
                                                                                Adt4::C1(Adt5::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1113 = v0;
                                                                                let a1114 = 21u64;
                                                                                if nat_eq(a1113, a1114) {
                                                                                    Adt4::C1(Adt5::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1115 = v0;
                                                                                    let a1116 = 22u64;
                                                                                    if nat_eq(a1115, a1116) {
                                                                                        Adt4::C1(Adt5::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1117 = v0;
                                                                                        let a1118 = 23u64;
                                                                                        if nat_eq(a1117, a1118) {
                                                                                            Adt4::C1(Adt5::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1119 = v0;
                                                                                            let a1120 = 24u64;
                                                                                            if nat_eq(a1119, a1120) {
                                                                                                Adt4::C1(Adt5::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1121 = v0;
                                                                                                let a1122 = 25u64;
                                                                                                if nat_eq(a1121, a1122) {
                                                                                                    Adt4::C1(Adt5::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1123 = v0;
                                                                                                    let a1124 = 26u64;
                                                                                                    if nat_eq(a1123, a1124) {
                                                                                                        Adt4::C1(Adt5::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1125 = v0;
                                                                                                        let a1126 = 27u64;
                                                                                                        if nat_eq(a1125, a1126) {
                                                                                                            Adt4::C1(Adt5::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1127 = v0;
                                                                                                            let a1128 = 28u64;
                                                                                                            if nat_eq(a1127, a1128) {
                                                                                                                Adt4::C1(Adt5::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1129 = v0;
                                                                                                                let a1130 = 29u64;
                                                                                                                if nat_eq(a1129, a1130) {
                                                                                                                    Adt4::C1(Adt5::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1131 = v0;
                                                                                                                    let a1132 = 30u64;
                                                                                                                    if nat_eq(a1131, a1132) {
                                                                                                                        Adt4::C1(Adt5::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1133 = v0;
                                                                                                                        let a1134 = 34u64;
                                                                                                                        if nat_eq(a1133, a1134) {
                                                                                                                            Adt4::C1(Adt5::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1135 = v0;
                                                                                                                            let a1136 = 35u64;
                                                                                                                            if nat_eq(a1135, a1136) {
                                                                                                                                Adt4::C1(Adt5::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1137 = v0;
                                                                                                                                let a1138 = 39u64;
                                                                                                                                if nat_eq(a1137, a1138) {
                                                                                                                                    Adt4::C1(Adt5::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1139 = v0;
                                                                                                                                    let a1140 = 40u64;
                                                                                                                                    if nat_eq(a1139, a1140) {
                                                                                                                                        Adt4::C1(Adt5::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1141 = v0;
                                                                                                                                        let a1142 = 41u64;
                                                                                                                                        if nat_eq(a1141, a1142) {
                                                                                                                                            Adt4::C1(Adt5::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1143 = v0;
                                                                                                                                            let a1144 = 42u64;
                                                                                                                                            if nat_eq(a1143, a1144) {
                                                                                                                                                Adt4::C1(Adt5::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1145 = v0;
                                                                                                                                                let a1146 = 43u64;
                                                                                                                                                if nat_eq(a1145, a1146) {
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

pub fn f52(v0: u64, v1: u64) -> u64 {
    let a1147 = v0;
    let a1148 = 1u64;
    if if nat_eq(a1147, a1148) {
        let a1149 = v1;
        let a1150 = 43u64;
        nat_eq(a1149, a1150)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f53(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt3 {
    let a1151 = f52(v0, v1);
    let a1152 = 2u64;
    if nat_eq(a1151, a1152) {
        let a1156 = {
            let a1153 = v2;
            let a1154 = 2u64;
            let a1155 = 0u64;
            nat_rem(a1153, a1154, a1155)
        };
        let a1157 = 0u64;
        if nat_eq(a1156, a1157) {
            Adt3::C3(0u64, 0u64, v0, 0u64, 0u64, v3, 2u64, 0u64, 1u64, 0u64, f19(v0), 8u64)
        } else {
            Adt3::C3(0u64, 0u64, 0u64, v1, 1u64, 0u64, 2u64, 0u64, 3u64, v4, 8u64, f28(v1))
        }
    } else {
        Adt3::C3(0u64, 0u64, v0, v1, 0u64, v3, 2u64, 0u64, 3u64, v4, f19(v0), f28(v1))
    }
}

pub fn f54(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a1158 = v1;
    let a1159 = 1u64;
    if if if nat_le(a1158, a1159) {
        true
    } else {
        let a1160 = v0;
        let a1161 = v2;
        nat_le(a1160, a1161)
    } {
        let a1162 = v3;
        let a1163 = 1u64;
        if nat_le(a1162, a1163) {
            true
        } else {
            let a1170 = {
                let a1168 = v0;
                let a1169 = {
                    let a1166 = {
                        let a1164 = v1;
                        let a1165 = 1u64;
                        nat_sub(a1164, a1165)
                    };
                    let a1167 = v2;
                    nat_mul(a1166, a1167)?
                };
                nat_add(a1168, a1169)?
            };
            let a1171 = v4;
            nat_le(a1170, a1171)
        }
    } else {
        false
    } {
        let a1172 = v5;
        let a1173 = 1u64;
        if nat_le(a1172, a1173) {
            Ok(true)
        } else {
            let a1186 = {
                let a1184 = {
                    let a1178 = v0;
                    let a1179 = {
                        let a1176 = {
                            let a1174 = v1;
                            let a1175 = 1u64;
                            nat_sub(a1174, a1175)
                        };
                        let a1177 = v2;
                        nat_mul(a1176, a1177)?
                    };
                    nat_add(a1178, a1179)?
                };
                let a1185 = {
                    let a1182 = {
                        let a1180 = v3;
                        let a1181 = 1u64;
                        nat_sub(a1180, a1181)
                    };
                    let a1183 = v4;
                    nat_mul(a1182, a1183)?
                };
                nat_add(a1184, a1185)?
            };
            let a1187 = v6;
            Ok(nat_le(a1186, a1187))
        }
    } else {
        Ok(false)
    }
}
