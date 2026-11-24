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
    let a216 = 3u64;
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

pub fn f7(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1) -> R<Adt3> {
    let a221 = v2;
    let a222 = 1u64;
    if if if if if if if if if if if if if if if nat_le(a221, a222) {
        let a224 = 0u64;
        let a225 = f33(match v5.clone() {
            Adt1::C0(h223, _, _, _, _, _, _, _, _, _, _) => {
                h223
            }
        });
        nat_lt(a224, a225)
    } else {
        false
    } {
        let a228 = match v5.clone() {
            Adt1::C0(h226, _, _, _, _, _, _, _, _, _, _) => {
                h226
            }
        };
        let a229 = match v6.clone() {
            Adt1::C0(h227, _, _, _, _, _, _, _, _, _, _) => {
                h227
            }
        };
        nat_eq(a228, a229)
    } else {
        false
    } {
        let a232 = match v5.clone() {
            Adt1::C0(h230, _, _, _, _, _, _, _, _, _, _) => {
                h230
            }
        };
        let a233 = match v7.clone() {
            Adt1::C0(h231, _, _, _, _, _, _, _, _, _, _) => {
                h231
            }
        };
        nat_eq(a232, a233)
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
        let a236 = match v5.clone() {
            Adt1::C0(_, _, h234, _, _, _, _, _, _, _, _) => {
                h234
            }
        };
        let a237 = match v7.clone() {
            Adt1::C0(_, _, h235, _, _, _, _, _, _, _, _) => {
                h235
            }
        };
        nat_eq(a236, a237)
    } else {
        false
    } {
        let a240 = match v5.clone() {
            Adt1::C0(_, _, _, h238, _, _, _, _, _, _, _) => {
                h238
            }
        };
        let a241 = match v7.clone() {
            Adt1::C0(_, _, _, h239, _, _, _, _, _, _, _) => {
                h239
            }
        };
        nat_eq(a240, a241)
    } else {
        false
    } {
        let a244 = match v5.clone() {
            Adt1::C0(_, _, _, _, h242, _, _, _, _, _, _) => {
                h242
            }
        };
        let a245 = match v7.clone() {
            Adt1::C0(_, _, _, _, h243, _, _, _, _, _, _) => {
                h243
            }
        };
        nat_eq(a244, a245)
    } else {
        false
    } {
        let a252 = match v7.clone() {
            Adt1::C0(_, h246, _, _, _, _, _, _, _, _, _) => {
                h246
            }
        };
        let a253 = if v1 {
            match v5.clone() {
                Adt1::C0(_, h247, _, _, _, _, _, _, _, _, _) => {
                    h247
                }
            }
        } else {
            let a249 = match v5.clone() {
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
        let a255 = match v7.clone() {
            Adt1::C0(_, h254, _, _, _, _, _, _, _, _, _) => {
                h254
            }
        };
        let a256 = 2147483647u64;
        nat_le(a255, a256)
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
            let a259 = 1u64;
            let a260 = {
                let a257 = f37(v0);
                let a258 = f24(v7.clone())?;
                nat_mul(a257, a258)?
            };
            nat_add(a259, a260)?
        } else {
            let a261 = 4u64;
            let a262 = f24(v7.clone())?;
            nat_mul(a261, a262)?
        }, if f11(v0) {
            4u64
        } else {
            3u64
        }, if v1 {
            98u64
        } else {
            let a263 = 99u64;
            let a264 = v2;
            nat_add(a263, a264)?
        }, v0, v3, v4, v5, v6, v7)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f8(v0: u64, v1: Adt2) -> R<Adt3> {
    let a266 = v0;
    let a267 = match v1.clone() {
        Adt2::C0(h265, _, _, _, _, _, _, _, _) => {
            h265
        }
    };
    if nat_lt(a266, a267) {
        let v2: Adt1 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h268) => {
                h268
            }
        };
        let v3: u64 = f29(match v2.clone() {
            Adt1::C0(h269, _, _, _, _, _, _, _, _, _, _) => {
                h269
            }
        });
        let v4: u64 = {
            let a271 = match v2.clone() {
                Adt1::C0(_, h270, _, _, _, _, _, _, _, _, _) => {
                    h270
                }
            };
            let a272 = v3;
            let a273 = 0u64;
            nat_quot(a271, a272, a273)
        };
        let v5: u64 = {
            let a274 = v0;
            let a275 = v4;
            let a276 = 0u64;
            nat_quot(a274, a275, a276)
        };
        let v6: u64 = {
            let a278 = v5;
            let a279 = match v2.clone() {
                Adt1::C0(_, _, h277, _, _, _, _, _, _, _, _) => {
                    h277
                }
            };
            let a280 = 0u64;
            nat_quot(a278, a279, a280)
        };
        Ok(Adt3::C6(f38({
            let a281 = v0;
            let a282 = v3;
            nat_mul(a281, a282)?
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, h283, _, _) => {
                h283
            }
        })?, f39({
            let a284 = v0;
            let a285 = v4;
            let a286 = 0u64;
            nat_rem(a284, a285, a286)
        }, {
            let a288 = v5;
            let a289 = match v2.clone() {
                Adt1::C0(_, _, h287, _, _, _, _, _, _, _, _) => {
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
            nat_rem(a292, a293, a294)
        }, {
            let a296 = v6;
            let a297 = match v2.clone() {
                Adt1::C0(_, _, _, h295, _, _, _, _, _, _, _) => {
                    h295
                }
            };
            let a298 = 0u64;
            nat_quot(a296, a297, a298)
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, h299, _, _, _, _, _) => {
                h299
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h300, _, _, _, _) => {
                h300
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h301, _, _, _) => {
                h301
            }
        }, match v2 {
            Adt1::C0(_, _, _, _, _, _, _, _, h302, _, _) => {
                h302
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f9(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h303, _, _) => {
            h303
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h304) => {
            h304
        }
    };
    let a306 = v0;
    let a307 = match v1.clone() {
        Adt2::C0(h305, _, _, _, _, _, _, _, _) => {
            h305
        }
    };
    if nat_lt(a306, a307) {
        Ok(Adt3::C5({
            let a309 = v0;
            let a310 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, _, h308, _, _, _, _) => {
                    h308
                }
            };
            nat_mul(a309, a310)?
        }, {
            let a312 = v0;
            let a313 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h311, _, _, _, _) => {
                    h311
                }
            };
            nat_mul(a312, a313)?
        }, match v2 {
            Adt1::C0(_, h314, _, _, _, _, _, _, _, _, _) => {
                h314
            }
        }, match v3 {
            Adt1::C0(_, h315, _, _, _, _, _, _, _, _, _) => {
                h315
            }
        }, {
            let a317 = match v1 {
                Adt2::C0(_, _, _, h316, _, _, _, _, _) => {
                    h316
                }
            };
            let a318 = 1u64;
            nat_eq(a317, a318)
        }))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f10(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let a320 = v0;
    let a321 = match v1.clone() {
        Adt2::C0(h319, _, _, _, _, _, _, _, _) => {
            h319
        }
    };
    if nat_lt(a320, a321) {
        let v3: u64 = {
            let a323 = 2u64;
            let a324 = f24(match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h322, _) => {
                    h322
                }
            })?;
            nat_mul(a323, a324)?
        };
        let a325 = v0;
        let a326 = v3;
        if nat_lt(a325, a326) {
            f40(v0, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h327, _) => {
                    h327
                }
            }, {
                let m330 = match v1 {
                    Adt2::C0(_, _, _, _, _, _, _, _, h329) => {
                        h329
                    }
                };
                match m330 {
                    Adt1::C0(_, _, h328, _, _, _, _, _, _, _, _) => {
                        h328
                    }
                }
            }, v2)
        } else {
            f41({
                let a331 = v0;
                let a332 = v3;
                nat_sub(a331, a332)
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, h333, _, _) => {
                    h333
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h334, _) => {
                    h334
                }
            }, match v1 {
                Adt2::C0(_, _, _, _, _, _, _, _, h335) => {
                    h335
                }
            }, v2)
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f11(v0: u64) -> bool {
    let a336 = v0;
    let a337 = 3u64;
    if nat_eq(a336, a337) {
        true
    } else {
        let a338 = v0;
        let a339 = 6u64;
        nat_eq(a338, a339)
    }
}

pub fn f12(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, h340, _, _, _) => {
            h340
        }
    };
    let a342 = v0;
    let a343 = match v1.clone() {
        Adt2::C0(h341, _, _, _, _, _, _, _, _) => {
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
                let a349 = v4;
                let a350 = f37(match v1.clone() {
                    Adt2::C0(_, _, _, h348, _, _, _, _, _) => {
                        h348
                    }
                });
                let a351 = 0u64;
                nat_rem(a349, a350, a351)
            };
            let v6: u64 = {
                let a356 = 4u64;
                let a357 = {
                    let a353 = v4;
                    let a354 = f37(match v1.clone() {
                        Adt2::C0(_, _, _, h352, _, _, _, _, _) => {
                            h352
                        }
                    });
                    let a355 = 0u64;
                    nat_quot(a353, a354, a355)
                };
                nat_mul(a356, a357)?
            };
            let a358 = v5;
            let a359 = 2u64;
            if nat_lt(a358, a359) {
                f13({
                    let a360 = v6;
                    let a361 = v5;
                    nat_add(a360, a361)?
                }, v1)
            } else {
                let a362 = v5;
                let a363 = 2u64;
                if nat_eq(a362, a363) {
                    Ok(f42(0u64, 2u64, v3))
                } else {
                    let a364 = v5;
                    let a365 = 3u64;
                    if nat_eq(a364, a365) {
                        Ok(f43(0u64, 2u64, v3, {
                            let a366 = v2;
                            let a367 = 2u64;
                            nat_eq(a366, a367)
                        }))
                    } else {
                        let a368 = v5;
                        let a369 = 4u64;
                        if nat_eq(a368, a369) {
                            Ok(f42(1u64, 1u64, 3u64))
                        } else {
                            let a370 = v5;
                            let a371 = 5u64;
                            if nat_eq(a370, a371) {
                                Ok(f43(1u64, 1u64, 3u64, {
                                    let a372 = v2;
                                    let a373 = 0u64;
                                    nat_eq(a372, a373)
                                }))
                            } else {
                                let a374 = v5;
                                let a375 = 6u64;
                                if nat_eq(a374, a375) {
                                    Ok(f42(1u64, 2u64, v3))
                                } else {
                                    let a376 = v5;
                                    let a377 = 7u64;
                                    if nat_eq(a376, a377) {
                                        Ok(f43(1u64, 2u64, v3, {
                                            let a378 = v2;
                                            let a379 = 2u64;
                                            nat_eq(a378, a379)
                                        }))
                                    } else {
                                        let a381 = match v1.clone() {
                                            Adt2::C0(_, _, _, h380, _, _, _, _, _) => {
                                                h380
                                            }
                                        };
                                        let a382 = 3u64;
                                        if nat_eq(a381, a382) {
                                            let a383 = v5;
                                            let a384 = 15u64;
                                            if nat_eq(a383, a384) {
                                                f13({
                                                    let a385 = v6;
                                                    let a386 = 3u64;
                                                    nat_add(a385, a386)?
                                                }, v1)
                                            } else {
                                                Ok(f44(v5, match v1 {
                                                    Adt2::C0(_, _, _, _, h387, _, _, _, _) => {
                                                        h387
                                                    }
                                                }))
                                            }
                                        } else {
                                            let a388 = v5;
                                            let a389 = 8u64;
                                            if nat_eq(a388, a389) {
                                                Ok(Adt3::C4(2u64, {
                                                    let m392 = match v1 {
                                                        Adt2::C0(_, _, _, _, _, _, _, _, h391) => {
                                                            h391
                                                        }
                                                    };
                                                    match m392 {
                                                        Adt1::C0(h390, _, _, _, _, _, _, _, _, _, _) => {
                                                            h390
                                                        }
                                                    }
                                                }, 0u64, 1u64, 0u64))
                                            } else {
                                                f13({
                                                    let a393 = v6;
                                                    let a394 = 3u64;
                                                    nat_add(a393, a394)?
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
        Adt2::C0(_, _, _, _, _, _, h395, _, _) => {
            h395
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h396, _) => {
            h396
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h397) => {
            h397
        }
    };
    let v5: u64 = match v1.clone() {
        Adt2::C0(_, _, h398, _, _, _, _, _, _) => {
            h398
        }
    };
    let v6: u64 = {
        let a399 = v0;
        let a400 = 4u64;
        let a401 = 0u64;
        nat_rem(a399, a400, a401)
    };
    let v7: u64 = {
        let a402 = v0;
        let a403 = 4u64;
        let a404 = 0u64;
        nat_quot(a402, a403, a404)
    };
    let v8: u64 = {
        let a406 = v7;
        let a407 = match v4.clone() {
            Adt1::C0(_, h405, _, _, _, _, _, _, _, _, _) => {
                h405
            }
        };
        let a408 = 0u64;
        nat_quot(a406, a407, a408)
    };
    let v9: u64 = f33(match v4.clone() {
        Adt1::C0(h409, _, _, _, _, _, _, _, _, _, _) => {
            h409
        }
    });
    let v10: u64 = {
        let a414 = {
            let a411 = v7;
            let a412 = match v4.clone() {
                Adt1::C0(_, h410, _, _, _, _, _, _, _, _, _) => {
                    h410
                }
            };
            let a413 = 0u64;
            nat_rem(a411, a412, a413)
        };
        let a415 = v9;
        nat_mul(a414, a415)?
    };
    let v11: u64 = {
        let a417 = match v4.clone() {
            Adt1::C0(_, h416, _, _, _, _, _, _, _, _, _) => {
                h416
            }
        };
        let a418 = v9;
        nat_mul(a417, a418)?
    };
    let a420 = v0;
    let a421 = match v1.clone() {
        Adt2::C0(h419, _, _, _, _, _, _, _, _) => {
            h419
        }
    };
    if nat_lt(a420, a421) {
        let a422 = v6;
        let a423 = 0u64;
        if nat_eq(a422, a423) {
            Ok(Adt3::C3(0u64, 0u64, match v2.clone() {
                Adt1::C0(h424, _, _, _, _, _, _, _, _, _, _) => {
                    h424
                }
            }, 0u64, 0u64, {
                let a432 = {
                    let a428 = {
                        let a426 = v8;
                        let a427 = match v2 {
                            Adt1::C0(_, _, _, _, _, _, h425, _, _, _, _) => {
                                h425
                            }
                        };
                        nat_mul(a426, a427)?
                    };
                    let a429 = v10;
                    nat_add(a428, a429)?
                };
                let a433 = {
                    let a430 = v5;
                    let a431 = 100u64;
                    if nat_eq(a430, a431) {
                        v11
                    } else {
                        0u64
                    }
                };
                nat_add(a432, a433)?
            }, 2u64, 0u64, 0u64, 0u64, v9, 4u64))
        } else {
            let a434 = v6;
            let a435 = 1u64;
            if nat_eq(a434, a435) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h436, _, _, _, _, _, _, _, _, _, _) => {
                        h436
                    }
                }, 0u64, {
                    let a437 = v5;
                    let a438 = 98u64;
                    if nat_eq(a437, a438) {
                        4u64
                    } else {
                        0u64
                    }
                }, {
                    let a446 = {
                        let a442 = {
                            let a440 = v8;
                            let a441 = match v3 {
                                Adt1::C0(_, _, _, _, _, _, h439, _, _, _, _) => {
                                    h439
                                }
                            };
                            nat_mul(a440, a441)?
                        };
                        let a443 = v10;
                        nat_add(a442, a443)?
                    };
                    let a447 = {
                        let a444 = v5;
                        let a445 = 99u64;
                        if nat_eq(a444, a445) {
                            v11
                        } else {
                            0u64
                        }
                    };
                    nat_add(a446, a447)?
                }, 2u64, 0u64, 0u64, 1u64, v9, 4u64))
            } else {
                let a448 = v6;
                let a449 = 2u64;
                if nat_eq(a448, a449) {
                    Ok(Adt3::C4(match v1 {
                        Adt2::C0(_, _, _, h450, _, _, _, _, _) => {
                            h450
                        }
                    }, match v4 {
                        Adt1::C0(h451, _, _, _, _, _, _, _, _, _, _) => {
                            h451
                        }
                    }, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt3::C3(0u64, 0u64, 0u64, match v4.clone() {
                        Adt1::C0(h452, _, _, _, _, _, _, _, _, _, _) => {
                            h452
                        }
                    }, 1u64, 0u64, 2u64, 0u64, 3u64, {
                        let a456 = {
                            let a454 = v8;
                            let a455 = match v4 {
                                Adt1::C0(_, _, _, _, _, _, h453, _, _, _, _) => {
                                    h453
                                }
                            };
                            nat_mul(a454, a455)?
                        };
                        let a457 = v10;
                        nat_add(a456, a457)?
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
        Adt2::C0(_, _, _, _, _, _, h458, _, _) => {
            h458
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h459, _) => {
            h459
        }
    };
    let v5: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h460) => {
            h460
        }
    };
    let v6: u64 = {
        let a462 = match v1.clone() {
            Adt2::C0(_, h461, _, _, _, _, _, _, _) => {
                h461
            }
        };
        let a463 = 3u64;
        if nat_eq(a462, a463) {
            0u64
        } else {
            let a464 = f24(v3.clone())?;
            let a465 = 2u64;
            nat_mul(a464, a465)?
        }
    };
    let a467 = match v1.clone() {
        Adt2::C0(h466, _, _, _, _, _, _, _, _) => {
            h466
        }
    };
    let a468 = v0;
    if nat_le(a467, a468) {
        Ok(Adt3::C1)
    } else {
        let a469 = v0;
        let a470 = v6;
        if nat_lt(a469, a470) {
            let v7: u64 = {
                let a471 = v0;
                let a472 = 2u64;
                let a473 = 0u64;
                nat_quot(a471, a472, a473)
            };
            let v8: u64 = {
                let a478 = 3u64;
                let a479 = {
                    let a475 = f38(v7, v5.clone())?;
                    let a476 = f18(match v5 {
                        Adt1::C0(h474, _, _, _, _, _, _, _, _, _, _) => {
                            h474
                        }
                    });
                    let a477 = 0u64;
                    nat_quot(a475, a476, a477)
                };
                nat_add(a478, a479)?
            };
            let a483 = {
                let a480 = v0;
                let a481 = 2u64;
                let a482 = 0u64;
                nat_rem(a480, a481, a482)
            };
            let a484 = 0u64;
            if nat_eq(a483, a484) {
                Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, v8, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
            } else {
                let a485 = v2;
                let a486 = 0u64;
                if nat_eq(a485, a486) {
                    Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 2u64, 1u64, 2u64, 0u64, 0u64, v8, 8u64, 8u64))
                } else {
                    Ok(Adt3::C1)
                }
            }
        } else {
            let v9: u64 = {
                let a487 = v0;
                let a488 = v6;
                nat_sub(a487, a488)
            };
            let v10: u64 = f25(match v3.clone() {
                Adt1::C0(h489, _, _, _, _, _, _, _, _, _, _) => {
                    h489
                }
            }, match v4.clone() {
                Adt1::C0(h490, _, _, _, _, _, _, _, _, _, _) => {
                    h490
                }
            }, match v5.clone() {
                Adt1::C0(h491, _, _, _, _, _, _, _, _, _, _) => {
                    h491
                }
            })?;
            let v11: u64 = {
                let a492 = v9;
                let a493 = v10;
                let a494 = 0u64;
                nat_quot(a492, a493, a494)
            };
            let v12: u64 = {
                let a495 = v9;
                let a496 = v10;
                let a497 = 0u64;
                nat_rem(a495, a496, a497)
            };
            let v13: u64 = {
                let a499 = match v3.clone() {
                    Adt1::C0(h498, _, _, _, _, _, _, _, _, _, _) => {
                        h498
                    }
                };
                let a500 = 0u64;
                if nat_eq(a499, a500) {
                    0u64
                } else {
                    1u64
                }
            };
            let v14: u64 = {
                let a502 = match v4.clone() {
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
            let a504 = v12;
            let a505 = v13;
            if nat_lt(a504, a505) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h506, _, _, _, _, _, _, _, _, _, _) => {
                        h506
                    }
                }, 0u64, 0u64, f38(v11, v3.clone())?, 2u64, 0u64, 0u64, 0u64, f18(match v3 {
                    Adt1::C0(h507, _, _, _, _, _, _, _, _, _, _) => {
                        h507
                    }
                }), 4u64))
            } else {
                let a510 = v12;
                let a511 = {
                    let a508 = v13;
                    let a509 = v14;
                    nat_add(a508, a509)?
                };
                if nat_lt(a510, a511) {
                    Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                        Adt1::C0(h512, _, _, _, _, _, _, _, _, _, _) => {
                            h512
                        }
                    }, 0u64, 4u64, f45(v11, v3, v4.clone())?, 2u64, 0u64, 0u64, 1u64, f18(match v4 {
                        Adt1::C0(h513, _, _, _, _, _, _, _, _, _, _) => {
                            h513
                        }
                    }), 4u64))
                } else {
                    let a516 = v12;
                    let a517 = {
                        let a514 = v13;
                        let a515 = v14;
                        nat_add(a514, a515)?
                    };
                    if nat_eq(a516, a517) {
                        Ok(Adt3::C3(1u64, match v1 {
                            Adt2::C0(_, _, h518, _, _, _, _, _, _) => {
                                h518
                            }
                        }, 0u64, 0u64, {
                            let a519 = v13;
                            let a520 = 0u64;
                            if nat_eq(a519, a520) {
                                0u64
                            } else {
                                1u64
                            }
                        }, {
                            let a521 = v13;
                            let a522 = 0u64;
                            if nat_eq(a521, a522) {
                                f38(v11, v3.clone())?
                            } else {
                                0u64
                            }
                        }, {
                            let a523 = v14;
                            let a524 = 0u64;
                            if nat_eq(a523, a524) {
                                4u64
                            } else {
                                1u64
                            }
                        }, {
                            let a525 = v14;
                            let a526 = 0u64;
                            if nat_eq(a525, a526) {
                                f45(v11, v3, v4)?
                            } else {
                                1u64
                            }
                        }, {
                            let a528 = match v5.clone() {
                                Adt1::C0(h527, _, _, _, _, _, _, _, _, _, _) => {
                                    h527
                                }
                            };
                            let a529 = 0u64;
                            if nat_eq(a528, a529) {
                                3u64
                            } else {
                                0u64
                            }
                        }, {
                            let a531 = match v5.clone() {
                                Adt1::C0(h530, _, _, _, _, _, _, _, _, _, _) => {
                                    h530
                                }
                            };
                            let a532 = 0u64;
                            if nat_eq(a531, a532) {
                                f38(v11, v5)?
                            } else {
                                0u64
                            }
                        }, 4u64, 4u64))
                    } else {
                        Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                            Adt1::C0(h533, _, _, _, _, _, _, _, _, _, _) => {
                                h533
                            }
                        }, 1u64, 0u64, 2u64, 0u64, 3u64, f38(v11, v5.clone())?, 4u64, f18(match v5 {
                            Adt1::C0(h534, _, _, _, _, _, _, _, _, _, _) => {
                                h534
                            }
                        })))
                    }
                }
            }
        }
    }
}

pub fn f15(v0: Adt1, v1: Adt1) -> bool {
    let a537 = match v0.clone() {
        Adt1::C0(_, h535, _, _, _, _, _, _, _, _, _) => {
            h535
        }
    };
    let a538 = match v1.clone() {
        Adt1::C0(_, h536, _, _, _, _, _, _, _, _, _) => {
            h536
        }
    };
    if if if nat_eq(a537, a538) {
        let a541 = match v0.clone() {
            Adt1::C0(_, _, h539, _, _, _, _, _, _, _, _) => {
                h539
            }
        };
        let a542 = match v1.clone() {
            Adt1::C0(_, _, h540, _, _, _, _, _, _, _, _) => {
                h540
            }
        };
        nat_eq(a541, a542)
    } else {
        false
    } {
        let a545 = match v0.clone() {
            Adt1::C0(_, _, _, h543, _, _, _, _, _, _, _) => {
                h543
            }
        };
        let a546 = match v1.clone() {
            Adt1::C0(_, _, _, h544, _, _, _, _, _, _, _) => {
                h544
            }
        };
        nat_eq(a545, a546)
    } else {
        false
    } {
        let a549 = match v0 {
            Adt1::C0(_, _, _, _, h547, _, _, _, _, _, _) => {
                h547
            }
        };
        let a550 = match v1 {
            Adt1::C0(_, _, _, _, h548, _, _, _, _, _, _) => {
                h548
            }
        };
        nat_eq(a549, a550)
    } else {
        false
    }
}

pub fn f16(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<bool> {
    let a552 = match v1.clone() {
        Adt1::C0(h551, _, _, _, _, _, _, _, _, _, _) => {
            h551
        }
    };
    let a553 = 43u64;
    if nat_eq(a552, a553) {
        let a554 = v0;
        let a555 = 0u64;
        if if if if if if nat_eq(a554, a555) {
            let a557 = match v2.clone() {
                Adt1::C0(h556, _, _, _, _, _, _, _, _, _, _) => {
                    h556
                }
            };
            let a558 = 0u64;
            nat_eq(a557, a558)
        } else {
            false
        } {
            let a560 = match v3.clone() {
                Adt1::C0(h559, _, _, _, _, _, _, _, _, _, _) => {
                    h559
                }
            };
            let a561 = 0u64;
            if nat_eq(a560, a561) {
                true
            } else {
                let a563 = match v3.clone() {
                    Adt1::C0(h562, _, _, _, _, _, _, _, _, _, _) => {
                        h562
                    }
                };
                let a564 = 43u64;
                nat_eq(a563, a564)
            }
        } else {
            false
        } {
            f15(v1.clone(), v2.clone())
        } else {
            false
        } {
            let a566 = match v2 {
                Adt1::C0(_, _, _, _, _, h565, _, _, _, _, _) => {
                    h565
                }
            };
            let a567 = 4u64;
            nat_eq(a566, a567)
        } else {
            false
        } {
            let a570 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h568, _, _, _, _, _) => {
                    h568
                }
            };
            let a571 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h569, _, _, _, _) => {
                    h569
                }
            };
            if nat_le(a570, a571) {
                let a574 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, h572, _, _, _, _) => {
                        h572
                    }
                };
                let a575 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h573, _, _, _) => {
                        h573
                    }
                };
                if nat_le(a574, a575) {
                    let a578 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, h576, _, _, _) => {
                            h576
                        }
                    };
                    let a579 = match v3 {
                        Adt1::C0(_, _, _, _, _, _, _, _, h577, _, _) => {
                            h577
                        }
                    };
                    nat_le(a578, a579)
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        } {
            let a587 = {
                let a585 = match v1.clone() {
                    Adt1::C0(_, _, h580, _, _, _, _, _, _, _, _) => {
                        h580
                    }
                };
                let a586 = {
                    let a583 = match v1.clone() {
                        Adt1::C0(_, _, _, h581, _, _, _, _, _, _, _) => {
                            h581
                        }
                    };
                    let a584 = match v1 {
                        Adt1::C0(_, _, _, _, h582, _, _, _, _, _, _) => {
                            h582
                        }
                    };
                    nat_mul(a583, a584)?
                };
                nat_mul(a585, a586)?
            };
            let a588 = 2147483647u64;
            Ok(nat_le(a587, a588))
        } else {
            Ok(false)
        }
    } else {
        if f46(v1.clone(), v2.clone()) {
            Ok(f47(match v1 {
                Adt1::C0(h589, _, _, _, _, _, _, _, _, _, _) => {
                    h589
                }
            }, match v2 {
                Adt1::C0(h590, _, _, _, _, _, _, _, _, _, _) => {
                    h590
                }
            }, match v3 {
                Adt1::C0(h591, _, _, _, _, _, _, _, _, _, _) => {
                    h591
                }
            }))
        } else {
            Ok(false)
        }
    }
}

pub fn f17(v0: Adt1) -> bool {
    f48(match v0.clone() {
        Adt1::C0(_, h592, _, _, _, _, _, _, _, _, _) => {
            h592
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h593, _, _, _, _, _, _, _, _) => {
            h593
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h594, _, _, _, _, _, _, _) => {
            h594
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, h595, _, _, _, _, _, _) => {
            h595
        }
    })
}

pub fn f18(v0: u64) -> u64 {
    let a596 = v0;
    let a597 = 43u64;
    if nat_eq(a596, a597) {
        1u64
    } else {
        f33(v0)
    }
}

pub fn f19(v0: Adt1) -> bool {
    let v1: u64 = f18(match v0.clone() {
        Adt1::C0(h598, _, _, _, _, _, _, _, _, _, _) => {
            h598
        }
    });
    let a603 = {
        let a600 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h599) => {
                h599
            }
        };
        let a601 = v1;
        let a602 = 1u64;
        nat_rem(a600, a601, a602)
    };
    let a604 = 0u64;
    if if nat_eq(a603, a604) {
        f49(match v0.clone() {
            Adt1::C0(_, h605, _, _, _, _, _, _, _, _, _) => {
                h605
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h606, _, _, _, _, _) => {
                h606
            }
        }, v1)
    } else {
        false
    } {
        f50(match v0.clone() {
            Adt1::C0(_, _, h607, _, _, _, _, _, _, _, _) => {
                h607
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h608, _, _, _, _, _, _, _) => {
                h608
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h609, _, _, _, _, _, _) => {
                h609
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h610, _, _, _, _) => {
                h610
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h611, _, _, _) => {
                h611
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h612, _, _) => {
                h612
            }
        }, v1)
    } else {
        false
    }
}

pub fn f20(v0: Adt1, v1: Adt1) -> R<bool> {
    let a620 = {
        let a618 = match v0.clone() {
            Adt1::C0(_, _, h613, _, _, _, _, _, _, _, _) => {
                h613
            }
        };
        let a619 = {
            let a616 = match v0.clone() {
                Adt1::C0(_, _, _, h614, _, _, _, _, _, _, _) => {
                    h614
                }
            };
            let a617 = match v0.clone() {
                Adt1::C0(_, _, _, _, h615, _, _, _, _, _, _) => {
                    h615
                }
            };
            nat_mul(a616, a617)?
        };
        nat_mul(a618, a619)?
    };
    let a621 = 9223372036854775807u64;
    if nat_le(a620, a621) {
        if f30(v1.clone()) {
            let a623 = match v1 {
                Adt1::C0(_, h622, _, _, _, _, _, _, _, _, _) => {
                    h622
                }
            };
            let a624 = 2147483647u64;
            Ok(nat_le(a623, a624))
        } else {
            let a626 = match v0 {
                Adt1::C0(_, h625, _, _, _, _, _, _, _, _, _) => {
                    h625
                }
            };
            let a627 = 2147483647u64;
            Ok(nat_le(a626, a627))
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
        let a630 = match v0.clone() {
            Adt1::C0(_, h628, _, _, _, _, _, _, _, _, _) => {
                h628
            }
        };
        let a631 = f18(match v0.clone() {
            Adt1::C0(h629, _, _, _, _, _, _, _, _, _, _) => {
                h629
            }
        });
        nat_mul(a630, a631)?
    }, match v0.clone() {
        Adt1::C0(_, _, h632, _, _, _, _, _, _, _, _) => {
            h632
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h633, _, _, _, _, _, _, _) => {
            h633
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h634, _, _, _, _, _, _) => {
            h634
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h635, _, _, _, _) => {
            h635
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h636, _, _, _) => {
            h636
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h637, _, _) => {
            h637
        }
    })
}

pub fn f23(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a644 = {
        let a638 = v0;
        let a639 = v2;
        nat_mul(a638, a639)?
    };
    let a645 = {
        let a640 = v1;
        let a641 = 0u64;
        if nat_eq(a640, a641) {
            0u64
        } else {
            let a642 = v0;
            let a643 = 2u64;
            nat_mul(a642, a643)?
        }
    };
    nat_add(a644, a645)
}

pub fn f24(v0: Adt1) -> R<u64> {
    let a654 = {
        let a648 = match v0.clone() {
            Adt1::C0(_, h646, _, _, _, _, _, _, _, _, _) => {
                h646
            }
        };
        let a649 = match v0.clone() {
            Adt1::C0(_, _, h647, _, _, _, _, _, _, _, _) => {
                h647
            }
        };
        nat_mul(a648, a649)?
    };
    let a655 = {
        let a652 = match v0.clone() {
            Adt1::C0(_, _, _, h650, _, _, _, _, _, _, _) => {
                h650
            }
        };
        let a653 = match v0 {
            Adt1::C0(_, _, _, _, h651, _, _, _, _, _, _) => {
                h651
            }
        };
        nat_mul(a652, a653)?
    };
    nat_mul(a654, a655)
}

pub fn f25(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a666 = {
        let a662 = {
            let a658 = 1u64;
            let a659 = {
                let a656 = v0;
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
            let a660 = v1;
            let a661 = 0u64;
            if nat_eq(a660, a661) {
                0u64
            } else {
                1u64
            }
        };
        nat_add(a662, a663)?
    };
    let a667 = {
        let a664 = v2;
        let a665 = 0u64;
        if nat_eq(a664, a665) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a666, a667)
}

pub fn f26(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f15(v0.clone(), v1.clone()) {
        let a670 = match v0.clone() {
            Adt1::C0(h668, _, _, _, _, _, _, _, _, _, _) => {
                h668
            }
        };
        let a671 = match v1.clone() {
            Adt1::C0(h669, _, _, _, _, _, _, _, _, _, _) => {
                h669
            }
        };
        nat_eq(a670, a671)
    } else {
        false
    } {
        let a674 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h672, _, _, _, _, _) => {
                h672
            }
        };
        let a675 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h673, _, _, _, _, _) => {
                h673
            }
        };
        nat_eq(a674, a675)
    } else {
        false
    } {
        let a678 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h676, _, _, _, _) => {
                h676
            }
        };
        let a679 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h677, _, _, _, _) => {
                h677
            }
        };
        nat_eq(a678, a679)
    } else {
        false
    } {
        let a682 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h680, _, _, _) => {
                h680
            }
        };
        let a683 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h681, _, _, _) => {
                h681
            }
        };
        nat_eq(a682, a683)
    } else {
        false
    } {
        let a686 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h684, _, _) => {
                h684
            }
        };
        let a687 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h685, _, _) => {
                h685
            }
        };
        nat_eq(a686, a687)
    } else {
        false
    } {
        let a689 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h688) => {
                h688
            }
        };
        let a690 = 0u64;
        if if nat_eq(a689, a690) {
            true
        } else {
            let a692 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h691) => {
                    h691
                }
            };
            let a693 = 0u64;
            nat_eq(a692, a693)
        } {
            true
        } else {
            let a696 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h694) => {
                    h694
                }
            };
            let a697 = match v1 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h695) => {
                    h695
                }
            };
            nat_eq(a696, a697)
        }
    } else {
        false
    }
}

pub fn f27(v0: u64) -> u64 {
    if f32(v0) {
        let m698 = f53(v0);
        match m698 {
            Adt4::C0 => {
                0u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, _, h699) => {
                        h699
                    }
                }
            }
        }
    } else {
        f18(v0)
    }
}

pub fn f28(v0: u64) -> u64 {
    let a700 = v0;
    let a701 = 26u64;
    if nat_eq(a700, a701) {
        4u64
    } else {
        let a702 = v0;
        let a703 = 27u64;
        if nat_eq(a702, a703) {
            8u64
        } else {
            0u64
        }
    }
}

pub fn f29(v0: u64) -> u64 {
    if f32(v0) {
        let m704 = f53(v0);
        match m704 {
            Adt4::C0 => {
                1u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, h705, _) => {
                        h705
                    }
                }
            }
        }
    } else {
        1u64
    }
}

pub fn f30(v0: Adt1) -> bool {
    let a707 = match v0.clone() {
        Adt1::C0(_, h706, _, _, _, _, _, _, _, _, _) => {
            h706
        }
    };
    let a708 = 1u64;
    if nat_eq(a707, a708) {
        true
    } else {
        let a711 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h709, _, _, _, _, _) => {
                h709
            }
        };
        let a712 = f18(match v0 {
            Adt1::C0(h710, _, _, _, _, _, _, _, _, _, _) => {
                h710
            }
        });
        nat_eq(a711, a712)
    }
}

pub fn f31(v0: Adt1, v1: u64) -> R<u64> {
    let a718 = {
        let a714 = match v0.clone() {
            Adt1::C0(_, h713, _, _, _, _, _, _, _, _, _) => {
                h713
            }
        };
        let a715 = f29(v1);
        let a716 = 0u64;
        nat_quot(a714, a715, a716)
    };
    let a719 = f54(match v0 {
        Adt1::C0(h717, _, _, _, _, _, _, _, _, _, _) => {
            h717
        }
    }, v1);
    nat_mul(a718, a719)
}

pub fn f32(v0: u64) -> bool {
    let a720 = 10u64;
    let a721 = v0;
    if if nat_le(a720, a721) {
        let a722 = v0;
        let a723 = 14u64;
        nat_le(a722, a723)
    } else {
        false
    } {
        true
    } else {
        let a724 = v0;
        let a725 = 2u64;
        if if if if nat_eq(a724, a725) {
            true
        } else {
            let a726 = v0;
            let a727 = 3u64;
            nat_eq(a726, a727)
        } {
            true
        } else {
            let a728 = v0;
            let a729 = 6u64;
            nat_eq(a728, a729)
        } {
            true
        } else {
            let a730 = v0;
            let a731 = 7u64;
            if nat_eq(a730, a731) {
                true
            } else {
                let a732 = v0;
                let a733 = 8u64;
                nat_eq(a732, a733)
            }
        } {
            true
        } else {
            let a734 = 34u64;
            let a735 = v0;
            if if nat_le(a734, a735) {
                let a736 = v0;
                let a737 = 35u64;
                nat_le(a736, a737)
            } else {
                false
            } {
                true
            } else {
                let a738 = 39u64;
                let a739 = v0;
                if nat_le(a738, a739) {
                    let a740 = v0;
                    let a741 = 42u64;
                    nat_le(a740, a741)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f33(v0: u64) -> u64 {
    let a742 = v0;
    let a743 = 0u64;
    if nat_eq(a742, a743) {
        4u64
    } else {
        let a744 = v0;
        let a745 = 1u64;
        if if nat_eq(a744, a745) {
            true
        } else {
            let a746 = v0;
            let a747 = 30u64;
            nat_eq(a746, a747)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f34(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a749 = match v0.clone() {
            Adt1::C0(_, h748, _, _, _, _, _, _, _, _, _) => {
                h748
            }
        };
        let a750 = 4u64;
        nat_mul(a749, a750)?
    };
    let v2: u64 = {
        let a753 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h751, _, _, _, _) => {
                h751
            }
        };
        let a754 = match v0.clone() {
            Adt1::C0(_, _, h752, _, _, _, _, _, _, _, _) => {
                h752
            }
        };
        nat_mul(a753, a754)?
    };
    let v3: u64 = {
        let a756 = v2;
        let a757 = match v0.clone() {
            Adt1::C0(_, _, _, h755, _, _, _, _, _, _, _) => {
                h755
            }
        };
        nat_mul(a756, a757)?
    };
    let a759 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h758, _, _, _, _, _) => {
            h758
        }
    };
    let a760 = 4u64;
    if if if if nat_eq(a759, a760) {
        let a765 = {
            let a762 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h761) => {
                    h761
                }
            };
            let a763 = 4u64;
            let a764 = 1u64;
            nat_rem(a762, a763, a764)
        };
        let a766 = 0u64;
        nat_eq(a765, a766)
    } else {
        false
    } {
        let a774 = {
            let a772 = match v0.clone() {
                Adt1::C0(_, _, h767, _, _, _, _, _, _, _, _) => {
                    h767
                }
            };
            let a773 = {
                let a770 = match v0.clone() {
                    Adt1::C0(_, _, _, h768, _, _, _, _, _, _, _) => {
                        h768
                    }
                };
                let a771 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h769, _, _, _, _, _, _) => {
                        h769
                    }
                };
                nat_mul(a770, a771)?
            };
            nat_mul(a772, a773)?
        };
        let a775 = 1u64;
        if nat_le(a774, a775) {
            true
        } else {
            let a777 = v1;
            let a778 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h776, _, _, _, _) => {
                    h776
                }
            };
            if nat_le(a777, a778) {
                let a783 = {
                    let a780 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, h779, _, _, _, _) => {
                            h779
                        }
                    };
                    let a781 = 4u64;
                    let a782 = 1u64;
                    nat_rem(a780, a781, a782)
                };
                let a784 = 0u64;
                nat_eq(a783, a784)
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a786 = match v0.clone() {
            Adt1::C0(_, _, _, h785, _, _, _, _, _, _, _) => {
                h785
            }
        };
        let a787 = 1u64;
        if nat_le(a786, a787) {
            true
        } else {
            let a789 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h788, _, _, _) => {
                    h788
                }
            };
            let a790 = v2;
            nat_eq(a789, a790)
        }
    } else {
        false
    } {
        let a792 = match v0.clone() {
            Adt1::C0(_, _, _, _, h791, _, _, _, _, _, _) => {
                h791
            }
        };
        let a793 = 1u64;
        if nat_le(a792, a793) {
            Ok(true)
        } else {
            let a795 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, h794, _, _) => {
                    h794
                }
            };
            let a796 = v3;
            Ok(nat_eq(a795, a796))
        }
    } else {
        Ok(false)
    }
}

pub fn f35(v0: Adt1, v1: Adt1) -> R<bool> {
    let a798 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h797, _) => {
            h797
        }
    };
    let a799 = 0u64;
    if nat_eq(a798, a799) {
        Ok(true)
    } else {
        let a801 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h800, _) => {
                h800
            }
        };
        let a802 = 0u64;
        if nat_eq(a801, a802) {
            Ok(true)
        } else {
            let a804 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h803) => {
                    h803
                }
            };
            let a805 = 0u64;
            if nat_eq(a804, a805) {
                Ok(true)
            } else {
                let a807 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h806) => {
                        h806
                    }
                };
                let a808 = 0u64;
                if nat_eq(a807, a808) {
                    Ok(true)
                } else {
                    let a814 = {
                        let a811 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h809) => {
                                h809
                            }
                        };
                        let a812 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h810, _) => {
                                h810
                            }
                        };
                        nat_add(a811, a812)?
                    };
                    let a815 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h813) => {
                            h813
                        }
                    };
                    if nat_le(a814, a815) {
                        Ok(true)
                    } else {
                        let a821 = {
                            let a818 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h816) => {
                                    h816
                                }
                            };
                            let a819 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h817, _) => {
                                    h817
                                }
                            };
                            nat_add(a818, a819)?
                        };
                        let a822 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h820) => {
                                h820
                            }
                        };
                        Ok(nat_le(a821, a822))
                    }
                }
            }
        }
    }
}

pub fn f36(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a825 = match v0.clone() {
            Adt1::C0(_, _, h823, _, _, _, _, _, _, _, _) => {
                h823
            }
        };
        let a826 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h824, _, _, _, _) => {
                h824
            }
        };
        nat_mul(a825, a826)?
    };
    if if f30(v0.clone()) {
        let a828 = match v0.clone() {
            Adt1::C0(_, _, _, h827, _, _, _, _, _, _, _) => {
                h827
            }
        };
        let a829 = 1u64;
        if nat_eq(a828, a829) {
            true
        } else {
            let a831 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h830, _, _, _) => {
                    h830
                }
            };
            let a832 = v1;
            nat_eq(a831, a832)
        }
    } else {
        false
    } {
        let a834 = match v0.clone() {
            Adt1::C0(_, _, _, _, h833, _, _, _, _, _, _) => {
                h833
            }
        };
        let a835 = 1u64;
        if nat_eq(a834, a835) {
            Ok(true)
        } else {
            let a840 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h836, _, _) => {
                    h836
                }
            };
            let a841 = {
                let a838 = v1;
                let a839 = match v0 {
                    Adt1::C0(_, _, _, h837, _, _, _, _, _, _, _) => {
                        h837
                    }
                };
                nat_mul(a838, a839)?
            };
            Ok(nat_eq(a840, a841))
        }
    } else {
        Ok(false)
    }
}

pub fn f37(v0: u64) -> u64 {
    let a842 = v0;
    let a843 = 3u64;
    if nat_eq(a842, a843) {
        16u64
    } else {
        10u64
    }
}

pub fn f38(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a845 = v0;
        let a846 = match v1.clone() {
            Adt1::C0(_, h844, _, _, _, _, _, _, _, _, _) => {
                h844
            }
        };
        let a847 = 0u64;
        nat_quot(a845, a846, a847)
    };
    let v3: u64 = {
        let a849 = v2;
        let a850 = match v1.clone() {
            Adt1::C0(_, _, h848, _, _, _, _, _, _, _, _) => {
                h848
            }
        };
        let a851 = 0u64;
        nat_quot(a849, a850, a851)
    };
    f39({
        let a853 = v0;
        let a854 = match v1.clone() {
            Adt1::C0(_, h852, _, _, _, _, _, _, _, _, _) => {
                h852
            }
        };
        let a855 = 0u64;
        nat_rem(a853, a854, a855)
    }, {
        let a857 = v2;
        let a858 = match v1.clone() {
            Adt1::C0(_, _, h856, _, _, _, _, _, _, _, _) => {
                h856
            }
        };
        let a859 = 0u64;
        nat_rem(a857, a858, a859)
    }, {
        let a861 = v3;
        let a862 = match v1.clone() {
            Adt1::C0(_, _, _, h860, _, _, _, _, _, _, _) => {
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
        nat_quot(a865, a866, a867)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h868, _, _, _, _, _) => {
            h868
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h869, _, _, _, _) => {
            h869
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h870, _, _, _) => {
            h870
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h871, _, _) => {
            h871
        }
    })
}

pub fn f39(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a884 = {
        let a880 = {
            let a876 = {
                let a872 = v0;
                let a873 = v4;
                nat_mul(a872, a873)?
            };
            let a877 = {
                let a874 = v1;
                let a875 = v5;
                nat_mul(a874, a875)?
            };
            nat_add(a876, a877)?
        };
        let a881 = {
            let a878 = v2;
            let a879 = v6;
            nat_mul(a878, a879)?
        };
        nat_add(a880, a881)?
    };
    let a885 = {
        let a882 = v3;
        let a883 = v7;
        nat_mul(a882, a883)?
    };
    nat_add(a884, a885)
}

pub fn f40(v0: u64, v1: Adt1, v2: u64, v3: u64) -> R<Adt3> {
    let v4: u64 = {
        let a886 = v0;
        let a887 = 2u64;
        let a888 = 0u64;
        nat_quot(a886, a887, a888)
    };
    let v5: u64 = f28(match v1.clone() {
        Adt1::C0(h889, _, _, _, _, _, _, _, _, _, _) => {
            h889
        }
    });
    let a893 = {
        let a890 = v0;
        let a891 = 2u64;
        let a892 = 0u64;
        nat_rem(a890, a891, a892)
    };
    let a894 = 0u64;
    if nat_eq(a893, a894) {
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 4u64, f38(v4, v1)?, 2u64, 0u64, 0u64, 2u64, v5, 8u64))
    } else {
        let a895 = v3;
        let a896 = v2;
        if if nat_lt(a895, a896) {
            let a899 = v3;
            let a900 = {
                let a897 = v5;
                let a898 = 4u64;
                if nat_eq(a897, a898) {
                    2147483648u64
                } else {
                    9223372036854775808u64
                }
            };
            nat_lt(a899, a900)
        } else {
            false
        } {
            Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, {
                let a901 = 3u64;
                let a902 = v4;
                nat_add(a901, a902)?
            }, 8u64, 8u64))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f41(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64) -> R<Adt3> {
    let v5: u64 = {
        let a904 = 1u64;
        let a905 = f31(v1.clone(), match v3.clone() {
            Adt1::C0(h903, _, _, _, _, _, _, _, _, _, _) => {
                h903
            }
        })?;
        nat_add(a904, a905)?
    };
    let v6: u64 = {
        let a906 = v0;
        let a907 = v5;
        let a908 = 0u64;
        nat_rem(a906, a907, a908)
    };
    let v7: u64 = {
        let a909 = v0;
        let a910 = v5;
        let a911 = 0u64;
        nat_quot(a909, a910, a911)
    };
    let v8: u64 = {
        let a913 = v7;
        let a914 = match v1.clone() {
            Adt1::C0(_, _, h912, _, _, _, _, _, _, _, _) => {
                h912
            }
        };
        let a915 = 0u64;
        nat_rem(a913, a914, a915)
    };
    let v9: u64 = {
        let a917 = v7;
        let a918 = match v1.clone() {
            Adt1::C0(_, _, h916, _, _, _, _, _, _, _, _) => {
                h916
            }
        };
        let a919 = 0u64;
        nat_quot(a917, a918, a919)
    };
    let v10: u64 = {
        let a921 = v9;
        let a922 = match v1.clone() {
            Adt1::C0(_, _, _, h920, _, _, _, _, _, _, _) => {
                h920
            }
        };
        let a923 = 0u64;
        nat_rem(a921, a922, a923)
    };
    let v11: u64 = {
        let a925 = v9;
        let a926 = match v1.clone() {
            Adt1::C0(_, _, _, h924, _, _, _, _, _, _, _) => {
                h924
            }
        };
        let a927 = 0u64;
        nat_quot(a925, a926, a927)
    };
    let a928 = v6;
    let a929 = 0u64;
    if nat_eq(a928, a929) {
        let v12: u64 = {
            let a946 = v8;
            let a947 = {
                let a944 = match v2.clone() {
                    Adt1::C0(_, h930, _, _, _, _, _, _, _, _, _) => {
                        h930
                    }
                };
                let a945 = {
                    let a942 = {
                        let a932 = v10;
                        let a933 = match v2.clone() {
                            Adt1::C0(_, _, h931, _, _, _, _, _, _, _, _) => {
                                h931
                            }
                        };
                        let a934 = 0u64;
                        nat_rem(a932, a933, a934)
                    };
                    let a943 = {
                        let a940 = match v2.clone() {
                            Adt1::C0(_, _, h935, _, _, _, _, _, _, _, _) => {
                                h935
                            }
                        };
                        let a941 = {
                            let a937 = v11;
                            let a938 = match v2 {
                                Adt1::C0(_, _, _, h936, _, _, _, _, _, _, _) => {
                                    h936
                                }
                            };
                            let a939 = 0u64;
                            nat_rem(a937, a938, a939)
                        };
                        nat_mul(a940, a941)?
                    };
                    nat_add(a942, a943)?
                };
                nat_mul(a944, a945)?
            };
            nat_add(a946, a947)?
        };
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, {
            let a948 = 3u64;
            let a949 = v12;
            nat_add(a948, a949)?
        }, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
    } else {
        let a951 = v4;
        let a952 = match v3.clone() {
            Adt1::C0(_, _, h950, _, _, _, _, _, _, _, _) => {
                h950
            }
        };
        if nat_lt(a951, a952) {
            let v13: u64 = {
                let a957 = {
                    let a953 = v6;
                    let a954 = 1u64;
                    nat_sub(a953, a954)
                };
                let a958 = f54(match v1.clone() {
                    Adt1::C0(h955, _, _, _, _, _, _, _, _, _, _) => {
                        h955
                    }
                }, match v3.clone() {
                    Adt1::C0(h956, _, _, _, _, _, _, _, _, _, _) => {
                        h956
                    }
                });
                let a959 = 0u64;
                nat_quot(a957, a958, a959)
            };
            let v14: u64 = f39({
                let a961 = v13;
                let a962 = f29(match v3.clone() {
                    Adt1::C0(h960, _, _, _, _, _, _, _, _, _, _) => {
                        h960
                    }
                });
                nat_mul(a961, a962)?
            }, v8, v10, v11, match v1.clone() {
                Adt1::C0(_, _, _, _, _, h963, _, _, _, _, _) => {
                    h963
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, h964, _, _, _, _) => {
                    h964
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h965, _, _, _) => {
                    h965
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h966, _, _) => {
                    h966
                }
            })?;
            let v15: u64 = f39(v13, v4, v10, v11, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h967, _, _, _, _, _) => {
                    h967
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h968, _, _, _, _) => {
                    h968
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h969, _, _, _) => {
                    h969
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h970, _, _) => {
                    h970
                }
            })?;
            if f32(match v3.clone() {
                Adt1::C0(h971, _, _, _, _, _, _, _, _, _, _) => {
                    h971
                }
            }) {
                Ok(Adt3::C6(v14, v15))
            } else {
                Ok(f55(match v1 {
                    Adt1::C0(h972, _, _, _, _, _, _, _, _, _, _) => {
                        h972
                    }
                }, match v3 {
                    Adt1::C0(h973, _, _, _, _, _, _, _, _, _, _) => {
                        h973
                    }
                }, {
                    let a974 = v6;
                    let a975 = 1u64;
                    nat_sub(a974, a975)
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
    let a976 = v0;
    let a977 = 8u64;
    if nat_eq(a976, a977) {
        Adt3::C3(1u64, 6u64, 0u64, 0u64, 1u64, 0u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
    } else {
        let a978 = v0;
        let a979 = 9u64;
        if nat_eq(a978, a979) {
            Adt3::C3(1u64, 2u64, 0u64, 0u64, 2u64, v1, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
        } else {
            let a980 = v0;
            let a981 = 10u64;
            if nat_eq(a980, a981) {
                Adt3::C3(5u64, 16u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
            } else {
                let a982 = v0;
                let a983 = 11u64;
                if nat_eq(a982, a983) {
                    Adt3::C3(1u64, 0u64, 0u64, 0u64, 2u64, 1065353216u64, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
                } else {
                    let a984 = v0;
                    let a985 = 12u64;
                    if nat_eq(a984, a985) {
                        Adt3::C3(1u64, 3u64, 0u64, 0u64, 1u64, 0u64, 1u64, 2u64, 0u64, 0u64, 4u64, 4u64)
                    } else {
                        let a986 = v0;
                        let a987 = 13u64;
                        if nat_eq(a986, a987) {
                            Adt3::C3(1u64, 0u64, 0u64, 0u64, 1u64, 1u64, 2u64, 1065353216u64, 0u64, 1u64, 4u64, 4u64)
                        } else {
                            let a988 = v0;
                            let a989 = 14u64;
                            if nat_eq(a988, a989) {
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
        let a991 = v0;
        let a992 = match v1.clone() {
            Adt1::C0(_, h990, _, _, _, _, _, _, _, _, _) => {
                h990
            }
        };
        let a993 = 0u64;
        nat_quot(a991, a992, a993)
    };
    let v4: u64 = {
        let a995 = v3;
        let a996 = match v1.clone() {
            Adt1::C0(_, _, h994, _, _, _, _, _, _, _, _) => {
                h994
            }
        };
        let a997 = 0u64;
        nat_quot(a995, a996, a997)
    };
    f39({
        let a1003 = {
            let a999 = v0;
            let a1000 = match v1.clone() {
                Adt1::C0(_, h998, _, _, _, _, _, _, _, _, _) => {
                    h998
                }
            };
            let a1001 = 0u64;
            nat_rem(a999, a1000, a1001)
        };
        let a1004 = match v2.clone() {
            Adt1::C0(_, h1002, _, _, _, _, _, _, _, _, _) => {
                h1002
            }
        };
        let a1005 = 0u64;
        nat_rem(a1003, a1004, a1005)
    }, {
        let a1011 = {
            let a1007 = v3;
            let a1008 = match v1.clone() {
                Adt1::C0(_, _, h1006, _, _, _, _, _, _, _, _) => {
                    h1006
                }
            };
            let a1009 = 0u64;
            nat_rem(a1007, a1008, a1009)
        };
        let a1012 = match v2.clone() {
            Adt1::C0(_, _, h1010, _, _, _, _, _, _, _, _) => {
                h1010
            }
        };
        let a1013 = 0u64;
        nat_rem(a1011, a1012, a1013)
    }, {
        let a1019 = {
            let a1015 = v4;
            let a1016 = match v1.clone() {
                Adt1::C0(_, _, _, h1014, _, _, _, _, _, _, _) => {
                    h1014
                }
            };
            let a1017 = 0u64;
            nat_rem(a1015, a1016, a1017)
        };
        let a1020 = match v2.clone() {
            Adt1::C0(_, _, _, h1018, _, _, _, _, _, _, _) => {
                h1018
            }
        };
        let a1021 = 0u64;
        nat_rem(a1019, a1020, a1021)
    }, {
        let a1027 = {
            let a1023 = v4;
            let a1024 = match v1 {
                Adt1::C0(_, _, _, h1022, _, _, _, _, _, _, _) => {
                    h1022
                }
            };
            let a1025 = 0u64;
            nat_quot(a1023, a1024, a1025)
        };
        let a1028 = match v2.clone() {
            Adt1::C0(_, _, _, _, h1026, _, _, _, _, _, _) => {
                h1026
            }
        };
        let a1029 = 0u64;
        nat_rem(a1027, a1028, a1029)
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h1030, _, _, _, _, _) => {
            h1030
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h1031, _, _, _, _) => {
            h1031
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1032, _, _, _) => {
            h1032
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1033, _, _) => {
            h1033
        }
    })
}

pub fn f46(v0: Adt1, v1: Adt1) -> bool {
    if f17(v1.clone()) {
        f17(v0)
    } else {
        let a1039 = {
            let a1036 = match v0.clone() {
                Adt1::C0(_, h1034, _, _, _, _, _, _, _, _, _) => {
                    h1034
                }
            };
            let a1037 = match v1.clone() {
                Adt1::C0(_, h1035, _, _, _, _, _, _, _, _, _) => {
                    h1035
                }
            };
            let a1038 = 1u64;
            nat_rem(a1036, a1037, a1038)
        };
        let a1040 = 0u64;
        if if if nat_eq(a1039, a1040) {
            let a1046 = {
                let a1043 = match v0.clone() {
                    Adt1::C0(_, _, h1041, _, _, _, _, _, _, _, _) => {
                        h1041
                    }
                };
                let a1044 = match v1.clone() {
                    Adt1::C0(_, _, h1042, _, _, _, _, _, _, _, _) => {
                        h1042
                    }
                };
                let a1045 = 1u64;
                nat_rem(a1043, a1044, a1045)
            };
            let a1047 = 0u64;
            nat_eq(a1046, a1047)
        } else {
            false
        } {
            let a1053 = {
                let a1050 = match v0.clone() {
                    Adt1::C0(_, _, _, h1048, _, _, _, _, _, _, _) => {
                        h1048
                    }
                };
                let a1051 = match v1.clone() {
                    Adt1::C0(_, _, _, h1049, _, _, _, _, _, _, _) => {
                        h1049
                    }
                };
                let a1052 = 1u64;
                nat_rem(a1050, a1051, a1052)
            };
            let a1054 = 0u64;
            nat_eq(a1053, a1054)
        } else {
            false
        } {
            let a1060 = {
                let a1057 = match v0 {
                    Adt1::C0(_, _, _, _, h1055, _, _, _, _, _, _) => {
                        h1055
                    }
                };
                let a1058 = match v1 {
                    Adt1::C0(_, _, _, _, h1056, _, _, _, _, _, _) => {
                        h1056
                    }
                };
                let a1059 = 1u64;
                nat_rem(a1057, a1058, a1059)
            };
            let a1061 = 0u64;
            nat_eq(a1060, a1061)
        } else {
            false
        }
    }
}

pub fn f47(v0: u64, v1: u64, v2: u64) -> bool {
    let a1062 = v0;
    let a1063 = 0u64;
    if if if nat_eq(a1062, a1063) {
        let a1064 = v1;
        let a1065 = 0u64;
        nat_eq(a1064, a1065)
    } else {
        false
    } {
        let a1066 = v2;
        let a1067 = 0u64;
        nat_eq(a1066, a1067)
    } else {
        false
    } {
        true
    } else {
        let a1068 = v0;
        let a1069 = 1u64;
        if if nat_eq(a1068, a1069) {
            true
        } else {
            let a1070 = v0;
            let a1071 = 30u64;
            nat_eq(a1070, a1071)
        } {
            let a1072 = v1;
            let a1073 = v0;
            if if nat_eq(a1072, a1073) {
                let a1074 = v2;
                let a1075 = v0;
                nat_eq(a1074, a1075)
            } else {
                false
            } {
                true
            } else {
                let a1076 = v1;
                let a1077 = 0u64;
                if nat_eq(a1076, a1077) {
                    let a1078 = v2;
                    let a1079 = v0;
                    if nat_eq(a1078, a1079) {
                        true
                    } else {
                        let a1080 = v2;
                        let a1081 = 0u64;
                        nat_eq(a1080, a1081)
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
    let a1082 = v0;
    let a1083 = 0u64;
    if if if nat_eq(a1082, a1083) {
        true
    } else {
        let a1084 = v1;
        let a1085 = 0u64;
        nat_eq(a1084, a1085)
    } {
        true
    } else {
        let a1086 = v2;
        let a1087 = 0u64;
        nat_eq(a1086, a1087)
    } {
        true
    } else {
        let a1088 = v3;
        let a1089 = 0u64;
        nat_eq(a1088, a1089)
    }
}

pub fn f49(v0: u64, v1: u64, v2: u64) -> bool {
    let a1090 = v0;
    let a1091 = 1u64;
    if nat_le(a1090, a1091) {
        true
    } else {
        let a1095 = {
            let a1092 = v1;
            let a1093 = v2;
            let a1094 = 1u64;
            nat_rem(a1092, a1093, a1094)
        };
        let a1096 = 0u64;
        nat_eq(a1095, a1096)
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
        let a1099 = match v0.clone() {
            Adt1::C0(h1097, _, _, _, _, _, _, _, _, _, _) => {
                h1097
            }
        };
        let a1100 = match v1.clone() {
            Adt1::C0(h1098, _, _, _, _, _, _, _, _, _, _) => {
                h1098
            }
        };
        nat_eq(a1099, a1100)
    } else {
        false
    } {
        let a1103 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1101) => {
                h1101
            }
        };
        let a1104 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1102) => {
                h1102
            }
        };
        nat_eq(a1103, a1104)
    } else {
        false
    } {
        let a1107 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1105, _, _, _, _, _) => {
                h1105
            }
        };
        let a1108 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1106, _, _, _, _, _) => {
                h1106
            }
        };
        nat_eq(a1107, a1108)
    } else {
        false
    } {
        let a1111 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1109, _, _, _, _) => {
                h1109
            }
        };
        let a1112 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1110, _, _, _, _) => {
                h1110
            }
        };
        nat_eq(a1111, a1112)
    } else {
        false
    } {
        let a1115 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1113, _, _, _) => {
                h1113
            }
        };
        let a1116 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1114, _, _, _) => {
                h1114
            }
        };
        nat_eq(a1115, a1116)
    } else {
        false
    } {
        let a1119 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1117, _, _) => {
                h1117
            }
        };
        let a1120 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1118, _, _) => {
                h1118
            }
        };
        nat_eq(a1119, a1120)
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
    let a1121 = v0;
    let a1122 = 0u64;
    if nat_eq(a1121, a1122) {
        Adt4::C1(Adt5::C0(0u64, 1u64, 4u64))
    } else {
        let a1123 = v0;
        let a1124 = 1u64;
        if nat_eq(a1123, a1124) {
            Adt4::C1(Adt5::C0(1u64, 1u64, 2u64))
        } else {
            let a1125 = v0;
            let a1126 = 2u64;
            if nat_eq(a1125, a1126) {
                Adt4::C1(Adt5::C0(2u64, 32u64, 18u64))
            } else {
                let a1127 = v0;
                let a1128 = 3u64;
                if nat_eq(a1127, a1128) {
                    Adt4::C1(Adt5::C0(3u64, 32u64, 20u64))
                } else {
                    let a1129 = v0;
                    let a1130 = 6u64;
                    if nat_eq(a1129, a1130) {
                        Adt4::C1(Adt5::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1131 = v0;
                        let a1132 = 7u64;
                        if nat_eq(a1131, a1132) {
                            Adt4::C1(Adt5::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1133 = v0;
                            let a1134 = 8u64;
                            if nat_eq(a1133, a1134) {
                                Adt4::C1(Adt5::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1135 = v0;
                                let a1136 = 9u64;
                                if nat_eq(a1135, a1136) {
                                    Adt4::C1(Adt5::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1137 = v0;
                                    let a1138 = 10u64;
                                    if nat_eq(a1137, a1138) {
                                        Adt4::C1(Adt5::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1139 = v0;
                                        let a1140 = 11u64;
                                        if nat_eq(a1139, a1140) {
                                            Adt4::C1(Adt5::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1141 = v0;
                                            let a1142 = 12u64;
                                            if nat_eq(a1141, a1142) {
                                                Adt4::C1(Adt5::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1143 = v0;
                                                let a1144 = 13u64;
                                                if nat_eq(a1143, a1144) {
                                                    Adt4::C1(Adt5::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1145 = v0;
                                                    let a1146 = 14u64;
                                                    if nat_eq(a1145, a1146) {
                                                        Adt4::C1(Adt5::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1147 = v0;
                                                        let a1148 = 15u64;
                                                        if nat_eq(a1147, a1148) {
                                                            Adt4::C1(Adt5::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1149 = v0;
                                                            let a1150 = 16u64;
                                                            if nat_eq(a1149, a1150) {
                                                                Adt4::C1(Adt5::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1151 = v0;
                                                                let a1152 = 17u64;
                                                                if nat_eq(a1151, a1152) {
                                                                    Adt4::C1(Adt5::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1153 = v0;
                                                                    let a1154 = 18u64;
                                                                    if nat_eq(a1153, a1154) {
                                                                        Adt4::C1(Adt5::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1155 = v0;
                                                                        let a1156 = 19u64;
                                                                        if nat_eq(a1155, a1156) {
                                                                            Adt4::C1(Adt5::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1157 = v0;
                                                                            let a1158 = 20u64;
                                                                            if nat_eq(a1157, a1158) {
                                                                                Adt4::C1(Adt5::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1159 = v0;
                                                                                let a1160 = 21u64;
                                                                                if nat_eq(a1159, a1160) {
                                                                                    Adt4::C1(Adt5::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1161 = v0;
                                                                                    let a1162 = 22u64;
                                                                                    if nat_eq(a1161, a1162) {
                                                                                        Adt4::C1(Adt5::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1163 = v0;
                                                                                        let a1164 = 23u64;
                                                                                        if nat_eq(a1163, a1164) {
                                                                                            Adt4::C1(Adt5::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1165 = v0;
                                                                                            let a1166 = 24u64;
                                                                                            if nat_eq(a1165, a1166) {
                                                                                                Adt4::C1(Adt5::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1167 = v0;
                                                                                                let a1168 = 25u64;
                                                                                                if nat_eq(a1167, a1168) {
                                                                                                    Adt4::C1(Adt5::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1169 = v0;
                                                                                                    let a1170 = 26u64;
                                                                                                    if nat_eq(a1169, a1170) {
                                                                                                        Adt4::C1(Adt5::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1171 = v0;
                                                                                                        let a1172 = 27u64;
                                                                                                        if nat_eq(a1171, a1172) {
                                                                                                            Adt4::C1(Adt5::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1173 = v0;
                                                                                                            let a1174 = 28u64;
                                                                                                            if nat_eq(a1173, a1174) {
                                                                                                                Adt4::C1(Adt5::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1175 = v0;
                                                                                                                let a1176 = 29u64;
                                                                                                                if nat_eq(a1175, a1176) {
                                                                                                                    Adt4::C1(Adt5::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1177 = v0;
                                                                                                                    let a1178 = 30u64;
                                                                                                                    if nat_eq(a1177, a1178) {
                                                                                                                        Adt4::C1(Adt5::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1179 = v0;
                                                                                                                        let a1180 = 34u64;
                                                                                                                        if nat_eq(a1179, a1180) {
                                                                                                                            Adt4::C1(Adt5::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1181 = v0;
                                                                                                                            let a1182 = 35u64;
                                                                                                                            if nat_eq(a1181, a1182) {
                                                                                                                                Adt4::C1(Adt5::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1183 = v0;
                                                                                                                                let a1184 = 39u64;
                                                                                                                                if nat_eq(a1183, a1184) {
                                                                                                                                    Adt4::C1(Adt5::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1185 = v0;
                                                                                                                                    let a1186 = 40u64;
                                                                                                                                    if nat_eq(a1185, a1186) {
                                                                                                                                        Adt4::C1(Adt5::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1187 = v0;
                                                                                                                                        let a1188 = 41u64;
                                                                                                                                        if nat_eq(a1187, a1188) {
                                                                                                                                            Adt4::C1(Adt5::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1189 = v0;
                                                                                                                                            let a1190 = 42u64;
                                                                                                                                            if nat_eq(a1189, a1190) {
                                                                                                                                                Adt4::C1(Adt5::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1191 = v0;
                                                                                                                                                let a1192 = 43u64;
                                                                                                                                                if nat_eq(a1191, a1192) {
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
    let a1193 = v0;
    let a1194 = 1u64;
    if if nat_eq(a1193, a1194) {
        let a1195 = v1;
        let a1196 = 43u64;
        nat_eq(a1195, a1196)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f55(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt3 {
    let a1197 = f54(v0, v1);
    let a1198 = 2u64;
    if nat_eq(a1197, a1198) {
        let a1202 = {
            let a1199 = v2;
            let a1200 = 2u64;
            let a1201 = 0u64;
            nat_rem(a1199, a1200, a1201)
        };
        let a1203 = 0u64;
        if nat_eq(a1202, a1203) {
            Adt3::C3(0u64, 0u64, v0, 0u64, 0u64, v3, 2u64, 0u64, 1u64, 0u64, f33(v0), 8u64)
        } else {
            Adt3::C3(0u64, 0u64, 0u64, v1, 1u64, 0u64, 2u64, 0u64, 3u64, v4, 8u64, f27(v1))
        }
    } else {
        Adt3::C3(0u64, 0u64, v0, v1, 0u64, v3, 2u64, 0u64, 3u64, v4, f33(v0), f27(v1))
    }
}

pub fn f56(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a1204 = v1;
    let a1205 = 1u64;
    if if if nat_le(a1204, a1205) {
        true
    } else {
        let a1206 = v0;
        let a1207 = v2;
        nat_le(a1206, a1207)
    } {
        let a1208 = v3;
        let a1209 = 1u64;
        if nat_le(a1208, a1209) {
            true
        } else {
            let a1216 = {
                let a1214 = v0;
                let a1215 = {
                    let a1212 = {
                        let a1210 = v1;
                        let a1211 = 1u64;
                        nat_sub(a1210, a1211)
                    };
                    let a1213 = v2;
                    nat_mul(a1212, a1213)?
                };
                nat_add(a1214, a1215)?
            };
            let a1217 = v4;
            nat_le(a1216, a1217)
        }
    } else {
        false
    } {
        let a1218 = v5;
        let a1219 = 1u64;
        if nat_le(a1218, a1219) {
            Ok(true)
        } else {
            let a1232 = {
                let a1230 = {
                    let a1224 = v0;
                    let a1225 = {
                        let a1222 = {
                            let a1220 = v1;
                            let a1221 = 1u64;
                            nat_sub(a1220, a1221)
                        };
                        let a1223 = v2;
                        nat_mul(a1222, a1223)?
                    };
                    nat_add(a1224, a1225)?
                };
                let a1231 = {
                    let a1228 = {
                        let a1226 = v3;
                        let a1227 = 1u64;
                        nat_sub(a1226, a1227)
                    };
                    let a1229 = v4;
                    nat_mul(a1228, a1229)?
                };
                nat_add(a1230, a1231)?
            };
            let a1233 = v6;
            Ok(nat_le(a1232, a1233))
        }
    } else {
        Ok(false)
    }
}

pub fn run(p0: Adt0) -> R<Adt3> {
    f0(p0)
}
