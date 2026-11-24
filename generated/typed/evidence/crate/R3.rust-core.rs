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
    if if if if nat_eq(a716, a717) {
        true
    } else {
        let a718 = v0;
        let a719 = 21u64;
        nat_eq(a718, a719)
    } {
        true
    } else {
        let a720 = v0;
        let a721 = 20u64;
        if nat_eq(a720, a721) {
            true
        } else {
            let a722 = v0;
            let a723 = 23u64;
            nat_eq(a722, a723)
        }
    } {
        true
    } else {
        let a724 = 10u64;
        let a725 = v0;
        if nat_le(a724, a725) {
            let a726 = v0;
            let a727 = 14u64;
            nat_le(a726, a727)
        } else {
            false
        }
    } {
        true
    } else {
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
            let a738 = 34u64;
            let a739 = v0;
            if if nat_le(a738, a739) {
                let a740 = v0;
                let a741 = 35u64;
                nat_le(a740, a741)
            } else {
                false
            } {
                true
            } else {
                let a742 = 39u64;
                let a743 = v0;
                if nat_le(a742, a743) {
                    let a744 = v0;
                    let a745 = 42u64;
                    nat_le(a744, a745)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f33(v0: u64) -> u64 {
    let a746 = v0;
    let a747 = 0u64;
    if nat_eq(a746, a747) {
        4u64
    } else {
        let a748 = v0;
        let a749 = 1u64;
        if if nat_eq(a748, a749) {
            true
        } else {
            let a750 = v0;
            let a751 = 30u64;
            nat_eq(a750, a751)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f34(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a753 = match v0.clone() {
            Adt1::C0(_, h752, _, _, _, _, _, _, _, _, _) => {
                h752
            }
        };
        let a754 = 4u64;
        nat_mul(a753, a754)?
    };
    let v2: u64 = {
        let a757 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h755, _, _, _, _) => {
                h755
            }
        };
        let a758 = match v0.clone() {
            Adt1::C0(_, _, h756, _, _, _, _, _, _, _, _) => {
                h756
            }
        };
        nat_mul(a757, a758)?
    };
    let v3: u64 = {
        let a760 = v2;
        let a761 = match v0.clone() {
            Adt1::C0(_, _, _, h759, _, _, _, _, _, _, _) => {
                h759
            }
        };
        nat_mul(a760, a761)?
    };
    let a763 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h762, _, _, _, _, _) => {
            h762
        }
    };
    let a764 = 4u64;
    if if if if nat_eq(a763, a764) {
        let a769 = {
            let a766 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h765) => {
                    h765
                }
            };
            let a767 = 4u64;
            let a768 = 1u64;
            nat_rem(a766, a767, a768)
        };
        let a770 = 0u64;
        nat_eq(a769, a770)
    } else {
        false
    } {
        let a778 = {
            let a776 = match v0.clone() {
                Adt1::C0(_, _, h771, _, _, _, _, _, _, _, _) => {
                    h771
                }
            };
            let a777 = {
                let a774 = match v0.clone() {
                    Adt1::C0(_, _, _, h772, _, _, _, _, _, _, _) => {
                        h772
                    }
                };
                let a775 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h773, _, _, _, _, _, _) => {
                        h773
                    }
                };
                nat_mul(a774, a775)?
            };
            nat_mul(a776, a777)?
        };
        let a779 = 1u64;
        if nat_le(a778, a779) {
            true
        } else {
            let a781 = v1;
            let a782 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h780, _, _, _, _) => {
                    h780
                }
            };
            if nat_le(a781, a782) {
                let a787 = {
                    let a784 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, h783, _, _, _, _) => {
                            h783
                        }
                    };
                    let a785 = 4u64;
                    let a786 = 1u64;
                    nat_rem(a784, a785, a786)
                };
                let a788 = 0u64;
                nat_eq(a787, a788)
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a790 = match v0.clone() {
            Adt1::C0(_, _, _, h789, _, _, _, _, _, _, _) => {
                h789
            }
        };
        let a791 = 1u64;
        if nat_le(a790, a791) {
            true
        } else {
            let a793 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h792, _, _, _) => {
                    h792
                }
            };
            let a794 = v2;
            nat_eq(a793, a794)
        }
    } else {
        false
    } {
        let a796 = match v0.clone() {
            Adt1::C0(_, _, _, _, h795, _, _, _, _, _, _) => {
                h795
            }
        };
        let a797 = 1u64;
        if nat_le(a796, a797) {
            Ok(true)
        } else {
            let a799 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, h798, _, _) => {
                    h798
                }
            };
            let a800 = v3;
            Ok(nat_eq(a799, a800))
        }
    } else {
        Ok(false)
    }
}

pub fn f35(v0: Adt1, v1: Adt1) -> R<bool> {
    let a802 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h801, _) => {
            h801
        }
    };
    let a803 = 0u64;
    if nat_eq(a802, a803) {
        Ok(true)
    } else {
        let a805 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h804, _) => {
                h804
            }
        };
        let a806 = 0u64;
        if nat_eq(a805, a806) {
            Ok(true)
        } else {
            let a808 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h807) => {
                    h807
                }
            };
            let a809 = 0u64;
            if nat_eq(a808, a809) {
                Ok(true)
            } else {
                let a811 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h810) => {
                        h810
                    }
                };
                let a812 = 0u64;
                if nat_eq(a811, a812) {
                    Ok(true)
                } else {
                    let a818 = {
                        let a815 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h813) => {
                                h813
                            }
                        };
                        let a816 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h814, _) => {
                                h814
                            }
                        };
                        nat_add(a815, a816)?
                    };
                    let a819 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h817) => {
                            h817
                        }
                    };
                    if nat_le(a818, a819) {
                        Ok(true)
                    } else {
                        let a825 = {
                            let a822 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h820) => {
                                    h820
                                }
                            };
                            let a823 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h821, _) => {
                                    h821
                                }
                            };
                            nat_add(a822, a823)?
                        };
                        let a826 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h824) => {
                                h824
                            }
                        };
                        Ok(nat_le(a825, a826))
                    }
                }
            }
        }
    }
}

pub fn f36(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a829 = match v0.clone() {
            Adt1::C0(_, _, h827, _, _, _, _, _, _, _, _) => {
                h827
            }
        };
        let a830 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h828, _, _, _, _) => {
                h828
            }
        };
        nat_mul(a829, a830)?
    };
    if if f30(v0.clone()) {
        let a832 = match v0.clone() {
            Adt1::C0(_, _, _, h831, _, _, _, _, _, _, _) => {
                h831
            }
        };
        let a833 = 1u64;
        if nat_eq(a832, a833) {
            true
        } else {
            let a835 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h834, _, _, _) => {
                    h834
                }
            };
            let a836 = v1;
            nat_eq(a835, a836)
        }
    } else {
        false
    } {
        let a838 = match v0.clone() {
            Adt1::C0(_, _, _, _, h837, _, _, _, _, _, _) => {
                h837
            }
        };
        let a839 = 1u64;
        if nat_eq(a838, a839) {
            Ok(true)
        } else {
            let a844 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h840, _, _) => {
                    h840
                }
            };
            let a845 = {
                let a842 = v1;
                let a843 = match v0 {
                    Adt1::C0(_, _, _, h841, _, _, _, _, _, _, _) => {
                        h841
                    }
                };
                nat_mul(a842, a843)?
            };
            Ok(nat_eq(a844, a845))
        }
    } else {
        Ok(false)
    }
}

pub fn f37(v0: u64) -> u64 {
    let a846 = v0;
    let a847 = 3u64;
    if nat_eq(a846, a847) {
        16u64
    } else {
        10u64
    }
}

pub fn f38(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a849 = v0;
        let a850 = match v1.clone() {
            Adt1::C0(_, h848, _, _, _, _, _, _, _, _, _) => {
                h848
            }
        };
        let a851 = 0u64;
        nat_quot(a849, a850, a851)
    };
    let v3: u64 = {
        let a853 = v2;
        let a854 = match v1.clone() {
            Adt1::C0(_, _, h852, _, _, _, _, _, _, _, _) => {
                h852
            }
        };
        let a855 = 0u64;
        nat_quot(a853, a854, a855)
    };
    f39({
        let a857 = v0;
        let a858 = match v1.clone() {
            Adt1::C0(_, h856, _, _, _, _, _, _, _, _, _) => {
                h856
            }
        };
        let a859 = 0u64;
        nat_rem(a857, a858, a859)
    }, {
        let a861 = v2;
        let a862 = match v1.clone() {
            Adt1::C0(_, _, h860, _, _, _, _, _, _, _, _) => {
                h860
            }
        };
        let a863 = 0u64;
        nat_rem(a861, a862, a863)
    }, {
        let a865 = v3;
        let a866 = match v1.clone() {
            Adt1::C0(_, _, _, h864, _, _, _, _, _, _, _) => {
                h864
            }
        };
        let a867 = 0u64;
        nat_rem(a865, a866, a867)
    }, {
        let a869 = v3;
        let a870 = match v1.clone() {
            Adt1::C0(_, _, _, h868, _, _, _, _, _, _, _) => {
                h868
            }
        };
        let a871 = 0u64;
        nat_quot(a869, a870, a871)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h872, _, _, _, _, _) => {
            h872
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h873, _, _, _, _) => {
            h873
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h874, _, _, _) => {
            h874
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h875, _, _) => {
            h875
        }
    })
}

pub fn f39(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a888 = {
        let a884 = {
            let a880 = {
                let a876 = v0;
                let a877 = v4;
                nat_mul(a876, a877)?
            };
            let a881 = {
                let a878 = v1;
                let a879 = v5;
                nat_mul(a878, a879)?
            };
            nat_add(a880, a881)?
        };
        let a885 = {
            let a882 = v2;
            let a883 = v6;
            nat_mul(a882, a883)?
        };
        nat_add(a884, a885)?
    };
    let a889 = {
        let a886 = v3;
        let a887 = v7;
        nat_mul(a886, a887)?
    };
    nat_add(a888, a889)
}

pub fn f40(v0: u64, v1: Adt1, v2: u64, v3: u64) -> R<Adt3> {
    let v4: u64 = {
        let a890 = v0;
        let a891 = 2u64;
        let a892 = 0u64;
        nat_quot(a890, a891, a892)
    };
    let v5: u64 = f28(match v1.clone() {
        Adt1::C0(h893, _, _, _, _, _, _, _, _, _, _) => {
            h893
        }
    });
    let a897 = {
        let a894 = v0;
        let a895 = 2u64;
        let a896 = 0u64;
        nat_rem(a894, a895, a896)
    };
    let a898 = 0u64;
    if nat_eq(a897, a898) {
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 4u64, f38(v4, v1)?, 2u64, 0u64, 0u64, 2u64, v5, 8u64))
    } else {
        let a899 = v3;
        let a900 = v2;
        if if nat_lt(a899, a900) {
            let a903 = v3;
            let a904 = {
                let a901 = v5;
                let a902 = 4u64;
                if nat_eq(a901, a902) {
                    2147483648u64
                } else {
                    9223372036854775808u64
                }
            };
            nat_lt(a903, a904)
        } else {
            false
        } {
            Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, {
                let a905 = 3u64;
                let a906 = v4;
                nat_add(a905, a906)?
            }, 8u64, 8u64))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f41(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64) -> R<Adt3> {
    let v5: u64 = {
        let a908 = 1u64;
        let a909 = f31(v1.clone(), match v3.clone() {
            Adt1::C0(h907, _, _, _, _, _, _, _, _, _, _) => {
                h907
            }
        })?;
        nat_add(a908, a909)?
    };
    let v6: u64 = {
        let a910 = v0;
        let a911 = v5;
        let a912 = 0u64;
        nat_rem(a910, a911, a912)
    };
    let v7: u64 = {
        let a913 = v0;
        let a914 = v5;
        let a915 = 0u64;
        nat_quot(a913, a914, a915)
    };
    let v8: u64 = {
        let a917 = v7;
        let a918 = match v1.clone() {
            Adt1::C0(_, _, h916, _, _, _, _, _, _, _, _) => {
                h916
            }
        };
        let a919 = 0u64;
        nat_rem(a917, a918, a919)
    };
    let v9: u64 = {
        let a921 = v7;
        let a922 = match v1.clone() {
            Adt1::C0(_, _, h920, _, _, _, _, _, _, _, _) => {
                h920
            }
        };
        let a923 = 0u64;
        nat_quot(a921, a922, a923)
    };
    let v10: u64 = {
        let a925 = v9;
        let a926 = match v1.clone() {
            Adt1::C0(_, _, _, h924, _, _, _, _, _, _, _) => {
                h924
            }
        };
        let a927 = 0u64;
        nat_rem(a925, a926, a927)
    };
    let v11: u64 = {
        let a929 = v9;
        let a930 = match v1.clone() {
            Adt1::C0(_, _, _, h928, _, _, _, _, _, _, _) => {
                h928
            }
        };
        let a931 = 0u64;
        nat_quot(a929, a930, a931)
    };
    let a932 = v6;
    let a933 = 0u64;
    if nat_eq(a932, a933) {
        let v12: u64 = {
            let a950 = v8;
            let a951 = {
                let a948 = match v2.clone() {
                    Adt1::C0(_, h934, _, _, _, _, _, _, _, _, _) => {
                        h934
                    }
                };
                let a949 = {
                    let a946 = {
                        let a936 = v10;
                        let a937 = match v2.clone() {
                            Adt1::C0(_, _, h935, _, _, _, _, _, _, _, _) => {
                                h935
                            }
                        };
                        let a938 = 0u64;
                        nat_rem(a936, a937, a938)
                    };
                    let a947 = {
                        let a944 = match v2.clone() {
                            Adt1::C0(_, _, h939, _, _, _, _, _, _, _, _) => {
                                h939
                            }
                        };
                        let a945 = {
                            let a941 = v11;
                            let a942 = match v2 {
                                Adt1::C0(_, _, _, h940, _, _, _, _, _, _, _) => {
                                    h940
                                }
                            };
                            let a943 = 0u64;
                            nat_rem(a941, a942, a943)
                        };
                        nat_mul(a944, a945)?
                    };
                    nat_add(a946, a947)?
                };
                nat_mul(a948, a949)?
            };
            nat_add(a950, a951)?
        };
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, {
            let a952 = 3u64;
            let a953 = v12;
            nat_add(a952, a953)?
        }, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
    } else {
        let a955 = v4;
        let a956 = match v3.clone() {
            Adt1::C0(_, _, h954, _, _, _, _, _, _, _, _) => {
                h954
            }
        };
        if nat_lt(a955, a956) {
            let v13: u64 = {
                let a961 = {
                    let a957 = v6;
                    let a958 = 1u64;
                    nat_sub(a957, a958)
                };
                let a962 = f54(match v1.clone() {
                    Adt1::C0(h959, _, _, _, _, _, _, _, _, _, _) => {
                        h959
                    }
                }, match v3.clone() {
                    Adt1::C0(h960, _, _, _, _, _, _, _, _, _, _) => {
                        h960
                    }
                });
                let a963 = 0u64;
                nat_quot(a961, a962, a963)
            };
            let v14: u64 = f39({
                let a965 = v13;
                let a966 = f29(match v3.clone() {
                    Adt1::C0(h964, _, _, _, _, _, _, _, _, _, _) => {
                        h964
                    }
                });
                nat_mul(a965, a966)?
            }, v8, v10, v11, match v1.clone() {
                Adt1::C0(_, _, _, _, _, h967, _, _, _, _, _) => {
                    h967
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, h968, _, _, _, _) => {
                    h968
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h969, _, _, _) => {
                    h969
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h970, _, _) => {
                    h970
                }
            })?;
            let v15: u64 = f39(v13, v4, v10, v11, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h971, _, _, _, _, _) => {
                    h971
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h972, _, _, _, _) => {
                    h972
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h973, _, _, _) => {
                    h973
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h974, _, _) => {
                    h974
                }
            })?;
            if f32(match v3.clone() {
                Adt1::C0(h975, _, _, _, _, _, _, _, _, _, _) => {
                    h975
                }
            }) {
                Ok(Adt3::C6(v14, v15))
            } else {
                Ok(f55(match v1 {
                    Adt1::C0(h976, _, _, _, _, _, _, _, _, _, _) => {
                        h976
                    }
                }, match v3 {
                    Adt1::C0(h977, _, _, _, _, _, _, _, _, _, _) => {
                        h977
                    }
                }, {
                    let a978 = v6;
                    let a979 = 1u64;
                    nat_sub(a978, a979)
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
    let a980 = v0;
    let a981 = 8u64;
    if nat_eq(a980, a981) {
        Adt3::C3(1u64, 6u64, 0u64, 0u64, 1u64, 0u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
    } else {
        let a982 = v0;
        let a983 = 9u64;
        if nat_eq(a982, a983) {
            Adt3::C3(1u64, 2u64, 0u64, 0u64, 2u64, v1, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
        } else {
            let a984 = v0;
            let a985 = 10u64;
            if nat_eq(a984, a985) {
                Adt3::C3(5u64, 16u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
            } else {
                let a986 = v0;
                let a987 = 11u64;
                if nat_eq(a986, a987) {
                    Adt3::C3(1u64, 0u64, 0u64, 0u64, 2u64, 1065353216u64, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
                } else {
                    let a988 = v0;
                    let a989 = 12u64;
                    if nat_eq(a988, a989) {
                        Adt3::C3(1u64, 3u64, 0u64, 0u64, 1u64, 0u64, 1u64, 2u64, 0u64, 0u64, 4u64, 4u64)
                    } else {
                        let a990 = v0;
                        let a991 = 13u64;
                        if nat_eq(a990, a991) {
                            Adt3::C3(1u64, 0u64, 0u64, 0u64, 1u64, 1u64, 2u64, 1065353216u64, 0u64, 1u64, 4u64, 4u64)
                        } else {
                            let a992 = v0;
                            let a993 = 14u64;
                            if nat_eq(a992, a993) {
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
        let a995 = v0;
        let a996 = match v1.clone() {
            Adt1::C0(_, h994, _, _, _, _, _, _, _, _, _) => {
                h994
            }
        };
        let a997 = 0u64;
        nat_quot(a995, a996, a997)
    };
    let v4: u64 = {
        let a999 = v3;
        let a1000 = match v1.clone() {
            Adt1::C0(_, _, h998, _, _, _, _, _, _, _, _) => {
                h998
            }
        };
        let a1001 = 0u64;
        nat_quot(a999, a1000, a1001)
    };
    f39({
        let a1007 = {
            let a1003 = v0;
            let a1004 = match v1.clone() {
                Adt1::C0(_, h1002, _, _, _, _, _, _, _, _, _) => {
                    h1002
                }
            };
            let a1005 = 0u64;
            nat_rem(a1003, a1004, a1005)
        };
        let a1008 = match v2.clone() {
            Adt1::C0(_, h1006, _, _, _, _, _, _, _, _, _) => {
                h1006
            }
        };
        let a1009 = 0u64;
        nat_rem(a1007, a1008, a1009)
    }, {
        let a1015 = {
            let a1011 = v3;
            let a1012 = match v1.clone() {
                Adt1::C0(_, _, h1010, _, _, _, _, _, _, _, _) => {
                    h1010
                }
            };
            let a1013 = 0u64;
            nat_rem(a1011, a1012, a1013)
        };
        let a1016 = match v2.clone() {
            Adt1::C0(_, _, h1014, _, _, _, _, _, _, _, _) => {
                h1014
            }
        };
        let a1017 = 0u64;
        nat_rem(a1015, a1016, a1017)
    }, {
        let a1023 = {
            let a1019 = v4;
            let a1020 = match v1.clone() {
                Adt1::C0(_, _, _, h1018, _, _, _, _, _, _, _) => {
                    h1018
                }
            };
            let a1021 = 0u64;
            nat_rem(a1019, a1020, a1021)
        };
        let a1024 = match v2.clone() {
            Adt1::C0(_, _, _, h1022, _, _, _, _, _, _, _) => {
                h1022
            }
        };
        let a1025 = 0u64;
        nat_rem(a1023, a1024, a1025)
    }, {
        let a1031 = {
            let a1027 = v4;
            let a1028 = match v1 {
                Adt1::C0(_, _, _, h1026, _, _, _, _, _, _, _) => {
                    h1026
                }
            };
            let a1029 = 0u64;
            nat_quot(a1027, a1028, a1029)
        };
        let a1032 = match v2.clone() {
            Adt1::C0(_, _, _, _, h1030, _, _, _, _, _, _) => {
                h1030
            }
        };
        let a1033 = 0u64;
        nat_rem(a1031, a1032, a1033)
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h1034, _, _, _, _, _) => {
            h1034
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h1035, _, _, _, _) => {
            h1035
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1036, _, _, _) => {
            h1036
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1037, _, _) => {
            h1037
        }
    })
}

pub fn f46(v0: Adt1, v1: Adt1) -> bool {
    if f17(v1.clone()) {
        f17(v0)
    } else {
        let a1043 = {
            let a1040 = match v0.clone() {
                Adt1::C0(_, h1038, _, _, _, _, _, _, _, _, _) => {
                    h1038
                }
            };
            let a1041 = match v1.clone() {
                Adt1::C0(_, h1039, _, _, _, _, _, _, _, _, _) => {
                    h1039
                }
            };
            let a1042 = 1u64;
            nat_rem(a1040, a1041, a1042)
        };
        let a1044 = 0u64;
        if if if nat_eq(a1043, a1044) {
            let a1050 = {
                let a1047 = match v0.clone() {
                    Adt1::C0(_, _, h1045, _, _, _, _, _, _, _, _) => {
                        h1045
                    }
                };
                let a1048 = match v1.clone() {
                    Adt1::C0(_, _, h1046, _, _, _, _, _, _, _, _) => {
                        h1046
                    }
                };
                let a1049 = 1u64;
                nat_rem(a1047, a1048, a1049)
            };
            let a1051 = 0u64;
            nat_eq(a1050, a1051)
        } else {
            false
        } {
            let a1057 = {
                let a1054 = match v0.clone() {
                    Adt1::C0(_, _, _, h1052, _, _, _, _, _, _, _) => {
                        h1052
                    }
                };
                let a1055 = match v1.clone() {
                    Adt1::C0(_, _, _, h1053, _, _, _, _, _, _, _) => {
                        h1053
                    }
                };
                let a1056 = 1u64;
                nat_rem(a1054, a1055, a1056)
            };
            let a1058 = 0u64;
            nat_eq(a1057, a1058)
        } else {
            false
        } {
            let a1064 = {
                let a1061 = match v0 {
                    Adt1::C0(_, _, _, _, h1059, _, _, _, _, _, _) => {
                        h1059
                    }
                };
                let a1062 = match v1 {
                    Adt1::C0(_, _, _, _, h1060, _, _, _, _, _, _) => {
                        h1060
                    }
                };
                let a1063 = 1u64;
                nat_rem(a1061, a1062, a1063)
            };
            let a1065 = 0u64;
            nat_eq(a1064, a1065)
        } else {
            false
        }
    }
}

pub fn f47(v0: u64, v1: u64, v2: u64) -> bool {
    let a1066 = v0;
    let a1067 = 0u64;
    if if if nat_eq(a1066, a1067) {
        let a1068 = v1;
        let a1069 = 0u64;
        nat_eq(a1068, a1069)
    } else {
        false
    } {
        let a1070 = v2;
        let a1071 = 0u64;
        nat_eq(a1070, a1071)
    } else {
        false
    } {
        true
    } else {
        let a1072 = v0;
        let a1073 = 1u64;
        if if nat_eq(a1072, a1073) {
            true
        } else {
            let a1074 = v0;
            let a1075 = 30u64;
            nat_eq(a1074, a1075)
        } {
            let a1076 = v1;
            let a1077 = v0;
            if if nat_eq(a1076, a1077) {
                let a1078 = v2;
                let a1079 = v0;
                nat_eq(a1078, a1079)
            } else {
                false
            } {
                true
            } else {
                let a1080 = v1;
                let a1081 = 0u64;
                if nat_eq(a1080, a1081) {
                    let a1082 = v2;
                    let a1083 = v0;
                    if nat_eq(a1082, a1083) {
                        true
                    } else {
                        let a1084 = v2;
                        let a1085 = 0u64;
                        nat_eq(a1084, a1085)
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
    let a1086 = v0;
    let a1087 = 0u64;
    if if if nat_eq(a1086, a1087) {
        true
    } else {
        let a1088 = v1;
        let a1089 = 0u64;
        nat_eq(a1088, a1089)
    } {
        true
    } else {
        let a1090 = v2;
        let a1091 = 0u64;
        nat_eq(a1090, a1091)
    } {
        true
    } else {
        let a1092 = v3;
        let a1093 = 0u64;
        nat_eq(a1092, a1093)
    }
}

pub fn f49(v0: u64, v1: u64, v2: u64) -> bool {
    let a1094 = v0;
    let a1095 = 1u64;
    if nat_le(a1094, a1095) {
        true
    } else {
        let a1099 = {
            let a1096 = v1;
            let a1097 = v2;
            let a1098 = 1u64;
            nat_rem(a1096, a1097, a1098)
        };
        let a1100 = 0u64;
        nat_eq(a1099, a1100)
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
        let a1103 = match v0.clone() {
            Adt1::C0(h1101, _, _, _, _, _, _, _, _, _, _) => {
                h1101
            }
        };
        let a1104 = match v1.clone() {
            Adt1::C0(h1102, _, _, _, _, _, _, _, _, _, _) => {
                h1102
            }
        };
        nat_eq(a1103, a1104)
    } else {
        false
    } {
        let a1107 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1105) => {
                h1105
            }
        };
        let a1108 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1106) => {
                h1106
            }
        };
        nat_eq(a1107, a1108)
    } else {
        false
    } {
        let a1111 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1109, _, _, _, _, _) => {
                h1109
            }
        };
        let a1112 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1110, _, _, _, _, _) => {
                h1110
            }
        };
        nat_eq(a1111, a1112)
    } else {
        false
    } {
        let a1115 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1113, _, _, _, _) => {
                h1113
            }
        };
        let a1116 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1114, _, _, _, _) => {
                h1114
            }
        };
        nat_eq(a1115, a1116)
    } else {
        false
    } {
        let a1119 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1117, _, _, _) => {
                h1117
            }
        };
        let a1120 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1118, _, _, _) => {
                h1118
            }
        };
        nat_eq(a1119, a1120)
    } else {
        false
    } {
        let a1123 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1121, _, _) => {
                h1121
            }
        };
        let a1124 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1122, _, _) => {
                h1122
            }
        };
        nat_eq(a1123, a1124)
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
    let a1125 = v0;
    let a1126 = 0u64;
    if nat_eq(a1125, a1126) {
        Adt4::C1(Adt5::C0(0u64, 1u64, 4u64))
    } else {
        let a1127 = v0;
        let a1128 = 1u64;
        if nat_eq(a1127, a1128) {
            Adt4::C1(Adt5::C0(1u64, 1u64, 2u64))
        } else {
            let a1129 = v0;
            let a1130 = 2u64;
            if nat_eq(a1129, a1130) {
                Adt4::C1(Adt5::C0(2u64, 32u64, 18u64))
            } else {
                let a1131 = v0;
                let a1132 = 3u64;
                if nat_eq(a1131, a1132) {
                    Adt4::C1(Adt5::C0(3u64, 32u64, 20u64))
                } else {
                    let a1133 = v0;
                    let a1134 = 6u64;
                    if nat_eq(a1133, a1134) {
                        Adt4::C1(Adt5::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1135 = v0;
                        let a1136 = 7u64;
                        if nat_eq(a1135, a1136) {
                            Adt4::C1(Adt5::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1137 = v0;
                            let a1138 = 8u64;
                            if nat_eq(a1137, a1138) {
                                Adt4::C1(Adt5::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1139 = v0;
                                let a1140 = 9u64;
                                if nat_eq(a1139, a1140) {
                                    Adt4::C1(Adt5::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1141 = v0;
                                    let a1142 = 10u64;
                                    if nat_eq(a1141, a1142) {
                                        Adt4::C1(Adt5::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1143 = v0;
                                        let a1144 = 11u64;
                                        if nat_eq(a1143, a1144) {
                                            Adt4::C1(Adt5::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1145 = v0;
                                            let a1146 = 12u64;
                                            if nat_eq(a1145, a1146) {
                                                Adt4::C1(Adt5::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1147 = v0;
                                                let a1148 = 13u64;
                                                if nat_eq(a1147, a1148) {
                                                    Adt4::C1(Adt5::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1149 = v0;
                                                    let a1150 = 14u64;
                                                    if nat_eq(a1149, a1150) {
                                                        Adt4::C1(Adt5::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1151 = v0;
                                                        let a1152 = 15u64;
                                                        if nat_eq(a1151, a1152) {
                                                            Adt4::C1(Adt5::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1153 = v0;
                                                            let a1154 = 16u64;
                                                            if nat_eq(a1153, a1154) {
                                                                Adt4::C1(Adt5::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1155 = v0;
                                                                let a1156 = 17u64;
                                                                if nat_eq(a1155, a1156) {
                                                                    Adt4::C1(Adt5::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1157 = v0;
                                                                    let a1158 = 18u64;
                                                                    if nat_eq(a1157, a1158) {
                                                                        Adt4::C1(Adt5::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1159 = v0;
                                                                        let a1160 = 19u64;
                                                                        if nat_eq(a1159, a1160) {
                                                                            Adt4::C1(Adt5::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1161 = v0;
                                                                            let a1162 = 20u64;
                                                                            if nat_eq(a1161, a1162) {
                                                                                Adt4::C1(Adt5::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1163 = v0;
                                                                                let a1164 = 21u64;
                                                                                if nat_eq(a1163, a1164) {
                                                                                    Adt4::C1(Adt5::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1165 = v0;
                                                                                    let a1166 = 22u64;
                                                                                    if nat_eq(a1165, a1166) {
                                                                                        Adt4::C1(Adt5::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1167 = v0;
                                                                                        let a1168 = 23u64;
                                                                                        if nat_eq(a1167, a1168) {
                                                                                            Adt4::C1(Adt5::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1169 = v0;
                                                                                            let a1170 = 24u64;
                                                                                            if nat_eq(a1169, a1170) {
                                                                                                Adt4::C1(Adt5::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1171 = v0;
                                                                                                let a1172 = 25u64;
                                                                                                if nat_eq(a1171, a1172) {
                                                                                                    Adt4::C1(Adt5::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1173 = v0;
                                                                                                    let a1174 = 26u64;
                                                                                                    if nat_eq(a1173, a1174) {
                                                                                                        Adt4::C1(Adt5::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1175 = v0;
                                                                                                        let a1176 = 27u64;
                                                                                                        if nat_eq(a1175, a1176) {
                                                                                                            Adt4::C1(Adt5::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1177 = v0;
                                                                                                            let a1178 = 28u64;
                                                                                                            if nat_eq(a1177, a1178) {
                                                                                                                Adt4::C1(Adt5::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1179 = v0;
                                                                                                                let a1180 = 29u64;
                                                                                                                if nat_eq(a1179, a1180) {
                                                                                                                    Adt4::C1(Adt5::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1181 = v0;
                                                                                                                    let a1182 = 30u64;
                                                                                                                    if nat_eq(a1181, a1182) {
                                                                                                                        Adt4::C1(Adt5::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1183 = v0;
                                                                                                                        let a1184 = 34u64;
                                                                                                                        if nat_eq(a1183, a1184) {
                                                                                                                            Adt4::C1(Adt5::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1185 = v0;
                                                                                                                            let a1186 = 35u64;
                                                                                                                            if nat_eq(a1185, a1186) {
                                                                                                                                Adt4::C1(Adt5::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1187 = v0;
                                                                                                                                let a1188 = 39u64;
                                                                                                                                if nat_eq(a1187, a1188) {
                                                                                                                                    Adt4::C1(Adt5::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1189 = v0;
                                                                                                                                    let a1190 = 40u64;
                                                                                                                                    if nat_eq(a1189, a1190) {
                                                                                                                                        Adt4::C1(Adt5::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1191 = v0;
                                                                                                                                        let a1192 = 41u64;
                                                                                                                                        if nat_eq(a1191, a1192) {
                                                                                                                                            Adt4::C1(Adt5::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1193 = v0;
                                                                                                                                            let a1194 = 42u64;
                                                                                                                                            if nat_eq(a1193, a1194) {
                                                                                                                                                Adt4::C1(Adt5::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1195 = v0;
                                                                                                                                                let a1196 = 43u64;
                                                                                                                                                if nat_eq(a1195, a1196) {
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
    let a1197 = v0;
    let a1198 = 1u64;
    if if nat_eq(a1197, a1198) {
        let a1199 = v1;
        let a1200 = 43u64;
        nat_eq(a1199, a1200)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f55(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt3 {
    let a1201 = f54(v0, v1);
    let a1202 = 2u64;
    if nat_eq(a1201, a1202) {
        let a1206 = {
            let a1203 = v2;
            let a1204 = 2u64;
            let a1205 = 0u64;
            nat_rem(a1203, a1204, a1205)
        };
        let a1207 = 0u64;
        if nat_eq(a1206, a1207) {
            Adt3::C3(0u64, 0u64, v0, 0u64, 0u64, v3, 2u64, 0u64, 1u64, 0u64, f33(v0), 8u64)
        } else {
            Adt3::C3(0u64, 0u64, 0u64, v1, 1u64, 0u64, 2u64, 0u64, 3u64, v4, 8u64, f27(v1))
        }
    } else {
        Adt3::C3(0u64, 0u64, v0, v1, 0u64, v3, 2u64, 0u64, 3u64, v4, f33(v0), f27(v1))
    }
}

pub fn f56(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a1208 = v1;
    let a1209 = 1u64;
    if if if nat_le(a1208, a1209) {
        true
    } else {
        let a1210 = v0;
        let a1211 = v2;
        nat_le(a1210, a1211)
    } {
        let a1212 = v3;
        let a1213 = 1u64;
        if nat_le(a1212, a1213) {
            true
        } else {
            let a1220 = {
                let a1218 = v0;
                let a1219 = {
                    let a1216 = {
                        let a1214 = v1;
                        let a1215 = 1u64;
                        nat_sub(a1214, a1215)
                    };
                    let a1217 = v2;
                    nat_mul(a1216, a1217)?
                };
                nat_add(a1218, a1219)?
            };
            let a1221 = v4;
            nat_le(a1220, a1221)
        }
    } else {
        false
    } {
        let a1222 = v5;
        let a1223 = 1u64;
        if nat_le(a1222, a1223) {
            Ok(true)
        } else {
            let a1236 = {
                let a1234 = {
                    let a1228 = v0;
                    let a1229 = {
                        let a1226 = {
                            let a1224 = v1;
                            let a1225 = 1u64;
                            nat_sub(a1224, a1225)
                        };
                        let a1227 = v2;
                        nat_mul(a1226, a1227)?
                    };
                    nat_add(a1228, a1229)?
                };
                let a1235 = {
                    let a1232 = {
                        let a1230 = v3;
                        let a1231 = 1u64;
                        nat_sub(a1230, a1231)
                    };
                    let a1233 = v4;
                    nat_mul(a1232, a1233)?
                };
                nat_add(a1234, a1235)?
            };
            let a1237 = v6;
            Ok(nat_le(a1236, a1237))
        }
    } else {
        Ok(false)
    }
}
