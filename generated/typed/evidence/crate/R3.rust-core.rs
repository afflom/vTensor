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
    C0(u64, u64, Option<u64>, u64, u64, u64, u64, Adt1, Adt1, Adt1),
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
                Adt2::C0(_, _, _, h18, _, _, _, _, _, _) => {
                    h18
                }
            };
            let a20 = 33u64;
            if nat_eq(a19, a20) {
                f8(v26, v27)
            } else {
                let a22 = match v27.clone() {
                    Adt2::C0(_, _, _, h21, _, _, _, _, _, _) => {
                        h21
                    }
                };
                let a23 = 66u64;
                if if nat_eq(a22, a23) {
                    true
                } else {
                    let a25 = match v27.clone() {
                        Adt2::C0(_, _, _, h24, _, _, _, _, _, _) => {
                            h24
                        }
                    };
                    let a26 = 67u64;
                    nat_eq(a25, a26)
                } {
                    f9(v26, v27)
                } else {
                    let a28 = match v27.clone() {
                        Adt2::C0(_, _, _, h27, _, _, _, _, _, _) => {
                            h27
                        }
                    };
                    let a29 = 41u64;
                    if nat_eq(a28, a29) {
                        f10(v26, v27, v28)
                    } else {
                        let a31 = 98u64;
                        let a32 = match v27.clone() {
                            Adt2::C0(_, _, _, h30, _, _, _, _, _, _) => {
                                h30
                            }
                        };
                        if nat_le(a31, a32) {
                            if f11(match v27.clone() {
                                Adt2::C0(_, _, _, _, h33, _, _, _, _, _) => {
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
        }, None::<u64>, v0, 0u64, 0u64, 0u64, v1, v2, v3)))
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
        }, f32(match v3.clone() {
            Adt1::C0(h143, _, _, _, _, _, _, _, _, _, _) => {
                h143
            }
        }), 41u64, 0u64, 0u64, 0u64, v0, v1, v3)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f4(v0: Adt1, v1: Adt1, v2: Adt1) -> R<Adt3> {
    let v3: u64 = f29(match v2.clone() {
        Adt1::C0(h144, _, _, _, _, _, _, _, _, _, _) => {
            h144
        }
    });
    if if if if if if if f33(match v2.clone() {
        Adt1::C0(h145, _, _, _, _, _, _, _, _, _, _) => {
            h145
        }
    }) {
        f26(v1.clone(), v2.clone())
    } else {
        false
    } {
        let a147 = 0u64;
        let a148 = f34(match v0.clone() {
            Adt1::C0(h146, _, _, _, _, _, _, _, _, _, _) => {
                h146
            }
        });
        nat_lt(a147, a148)
    } else {
        false
    } {
        let a151 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h149, _, _, _, _, _) => {
                h149
            }
        };
        let a152 = f34(match v0.clone() {
            Adt1::C0(h150, _, _, _, _, _, _, _, _, _, _) => {
                h150
            }
        });
        nat_eq(a151, a152)
    } else {
        false
    } {
        let a157 = {
            let a154 = match v0.clone() {
                Adt1::C0(_, h153, _, _, _, _, _, _, _, _, _) => {
                    h153
                }
            };
            let a155 = v3;
            let a156 = 1u64;
            nat_rem(a154, a155, a156)
        };
        let a158 = 0u64;
        if nat_eq(a157, a158) {
            let a163 = {
                let a160 = match v2.clone() {
                    Adt1::C0(_, h159, _, _, _, _, _, _, _, _, _) => {
                        h159
                    }
                };
                let a161 = v3;
                let a162 = 1u64;
                nat_rem(a160, a161, a162)
            };
            let a164 = 0u64;
            nat_eq(a163, a164)
        } else {
            false
        }
    } else {
        false
    } {
        let a167 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h165, _, _, _, _, _) => {
                h165
            }
        };
        let a168 = f27(match v2.clone() {
            Adt1::C0(h166, _, _, _, _, _, _, _, _, _, _) => {
                h166
            }
        });
        nat_eq(a167, a168)
    } else {
        false
    } {
        let a169 = f24(v0.clone())?;
        let a170 = f24(v2.clone())?;
        nat_eq(a169, a170)
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a171 = f24(v0.clone())?;
            let a172 = v3;
            let a173 = 0u64;
            nat_quot(a171, a172, a173)
        }, 3u64, f32(match v2.clone() {
            Adt1::C0(h174, _, _, _, _, _, _, _, _, _, _) => {
                h174
            }
        }), 33u64, 0u64, 0u64, 0u64, v0, v1, v2)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f5(v0: u64, v1: u64, v2: Adt1, v3: Adt1) -> R<Adt3> {
    let a176 = match v2.clone() {
        Adt1::C0(h175, _, _, _, _, _, _, _, _, _, _) => {
            h175
        }
    };
    let a177 = 0u64;
    if if if if if if if if if if if nat_eq(a176, a177) {
        let a179 = match v3.clone() {
            Adt1::C0(h178, _, _, _, _, _, _, _, _, _, _) => {
                h178
            }
        };
        let a180 = 26u64;
        nat_eq(a179, a180)
    } else {
        false
    } {
        let a182 = match v2.clone() {
            Adt1::C0(_, h181, _, _, _, _, _, _, _, _, _) => {
                h181
            }
        };
        let a183 = 2147483647u64;
        nat_le(a182, a183)
    } else {
        false
    } {
        let a186 = match v3.clone() {
            Adt1::C0(_, h184, _, _, _, _, _, _, _, _, _) => {
                h184
            }
        };
        let a187 = match v2.clone() {
            Adt1::C0(_, h185, _, _, _, _, _, _, _, _, _) => {
                h185
            }
        };
        nat_le(a186, a187)
    } else {
        false
    } {
        let a190 = match v2.clone() {
            Adt1::C0(_, _, h188, _, _, _, _, _, _, _, _) => {
                h188
            }
        };
        let a191 = match v3.clone() {
            Adt1::C0(_, _, h189, _, _, _, _, _, _, _, _) => {
                h189
            }
        };
        nat_eq(a190, a191)
    } else {
        false
    } {
        let a194 = match v2.clone() {
            Adt1::C0(_, _, _, h192, _, _, _, _, _, _, _) => {
                h192
            }
        };
        let a195 = match v3.clone() {
            Adt1::C0(_, _, _, h193, _, _, _, _, _, _, _) => {
                h193
            }
        };
        nat_eq(a194, a195)
    } else {
        false
    } {
        let a198 = match v2.clone() {
            Adt1::C0(_, _, _, _, h196, _, _, _, _, _, _) => {
                h196
            }
        };
        let a199 = match v3.clone() {
            Adt1::C0(_, _, _, _, h197, _, _, _, _, _, _) => {
                h197
            }
        };
        nat_eq(a198, a199)
    } else {
        false
    } {
        let a200 = v0;
        let a201 = 67u64;
        if nat_eq(a200, a201) {
            true
        } else {
            let a202 = v1;
            let a203 = 1u64;
            if nat_le(a202, a203) {
                let a206 = match v3.clone() {
                    Adt1::C0(_, h204, _, _, _, _, _, _, _, _, _) => {
                        h204
                    }
                };
                let a207 = match v2.clone() {
                    Adt1::C0(_, h205, _, _, _, _, _, _, _, _, _) => {
                        h205
                    }
                };
                nat_eq(a206, a207)
            } else {
                false
            }
        }
    } else {
        false
    } {
        f35(v2.clone())?
    } else {
        false
    } {
        f35(v3.clone())?
    } else {
        false
    } {
        f36(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a213 = match v3.clone() {
                Adt1::C0(_, _, h208, _, _, _, _, _, _, _, _) => {
                    h208
                }
            };
            let a214 = {
                let a211 = match v3.clone() {
                    Adt1::C0(_, _, _, h209, _, _, _, _, _, _, _) => {
                        h209
                    }
                };
                let a212 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h210, _, _, _, _, _, _) => {
                        h210
                    }
                };
                nat_mul(a211, a212)?
            };
            nat_mul(a213, a214)?
        }, 3u64, None::<u64>, v0, {
            let a215 = v0;
            let a216 = 67u64;
            if nat_eq(a215, a216) {
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
    let a217 = v0;
    let a218 = 6u64;
    nat_le(a217, a218)
}

pub fn f7(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1) -> R<Adt3> {
    let a219 = v2;
    let a220 = 1u64;
    if if if if if if if if if if if if if if if nat_le(a219, a220) {
        let a222 = 0u64;
        let a223 = f34(match v5.clone() {
            Adt1::C0(h221, _, _, _, _, _, _, _, _, _, _) => {
                h221
            }
        });
        nat_lt(a222, a223)
    } else {
        false
    } {
        let a226 = match v5.clone() {
            Adt1::C0(h224, _, _, _, _, _, _, _, _, _, _) => {
                h224
            }
        };
        let a227 = match v6.clone() {
            Adt1::C0(h225, _, _, _, _, _, _, _, _, _, _) => {
                h225
            }
        };
        nat_eq(a226, a227)
    } else {
        false
    } {
        let a230 = match v5.clone() {
            Adt1::C0(h228, _, _, _, _, _, _, _, _, _, _) => {
                h228
            }
        };
        let a231 = match v7.clone() {
            Adt1::C0(h229, _, _, _, _, _, _, _, _, _, _) => {
                h229
            }
        };
        nat_eq(a230, a231)
    } else {
        false
    } {
        f15(v5.clone(), v6.clone())
    } else {
        false
    } {
        f37(v5.clone())?
    } else {
        false
    } {
        f37(v6.clone())?
    } else {
        false
    } {
        f37(v7.clone())?
    } else {
        false
    } {
        let a234 = match v5.clone() {
            Adt1::C0(_, _, h232, _, _, _, _, _, _, _, _) => {
                h232
            }
        };
        let a235 = match v7.clone() {
            Adt1::C0(_, _, h233, _, _, _, _, _, _, _, _) => {
                h233
            }
        };
        nat_eq(a234, a235)
    } else {
        false
    } {
        let a238 = match v5.clone() {
            Adt1::C0(_, _, _, h236, _, _, _, _, _, _, _) => {
                h236
            }
        };
        let a239 = match v7.clone() {
            Adt1::C0(_, _, _, h237, _, _, _, _, _, _, _) => {
                h237
            }
        };
        nat_eq(a238, a239)
    } else {
        false
    } {
        let a242 = match v5.clone() {
            Adt1::C0(_, _, _, _, h240, _, _, _, _, _, _) => {
                h240
            }
        };
        let a243 = match v7.clone() {
            Adt1::C0(_, _, _, _, h241, _, _, _, _, _, _) => {
                h241
            }
        };
        nat_eq(a242, a243)
    } else {
        false
    } {
        let a250 = match v7.clone() {
            Adt1::C0(_, h244, _, _, _, _, _, _, _, _, _) => {
                h244
            }
        };
        let a251 = if v1 {
            match v5.clone() {
                Adt1::C0(_, h245, _, _, _, _, _, _, _, _, _) => {
                    h245
                }
            }
        } else {
            let a247 = match v5.clone() {
                Adt1::C0(_, h246, _, _, _, _, _, _, _, _, _) => {
                    h246
                }
            };
            let a248 = 2u64;
            let a249 = 0u64;
            nat_quot(a247, a248, a249)
        };
        nat_eq(a250, a251)
    } else {
        false
    } {
        let a253 = match v7.clone() {
            Adt1::C0(_, h252, _, _, _, _, _, _, _, _, _) => {
                h252
            }
        };
        let a254 = 2147483647u64;
        nat_le(a253, a254)
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
            let a257 = 1u64;
            let a258 = {
                let a255 = f38(v0);
                let a256 = f24(v7.clone())?;
                nat_mul(a255, a256)?
            };
            nat_add(a257, a258)?
        } else {
            let a259 = 4u64;
            let a260 = f24(v7.clone())?;
            nat_mul(a259, a260)?
        }, if f11(v0) {
            4u64
        } else {
            3u64
        }, None::<u64>, if v1 {
            98u64
        } else {
            let a261 = 99u64;
            let a262 = v2;
            nat_add(a261, a262)?
        }, v0, v3, v4, v5, v6, v7)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f8(v0: u64, v1: Adt2) -> R<Adt3> {
    let a264 = v0;
    let a265 = match v1.clone() {
        Adt2::C0(h263, _, _, _, _, _, _, _, _, _) => {
            h263
        }
    };
    if nat_lt(a264, a265) {
        let v2: Adt1 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h266) => {
                h266
            }
        };
        let v3: u64 = f29(match v2.clone() {
            Adt1::C0(h267, _, _, _, _, _, _, _, _, _, _) => {
                h267
            }
        });
        let v4: u64 = {
            let a269 = match v2.clone() {
                Adt1::C0(_, h268, _, _, _, _, _, _, _, _, _) => {
                    h268
                }
            };
            let a270 = v3;
            let a271 = 0u64;
            nat_quot(a269, a270, a271)
        };
        let v5: u64 = {
            let a272 = v0;
            let a273 = v4;
            let a274 = 0u64;
            nat_quot(a272, a273, a274)
        };
        let v6: u64 = {
            let a276 = v5;
            let a277 = match v2.clone() {
                Adt1::C0(_, _, h275, _, _, _, _, _, _, _, _) => {
                    h275
                }
            };
            let a278 = 0u64;
            nat_quot(a276, a277, a278)
        };
        Ok(Adt3::C6(f39({
            let a279 = v0;
            let a280 = v3;
            nat_mul(a279, a280)?
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, h281, _, _) => {
                h281
            }
        })?, f40({
            let a282 = v0;
            let a283 = v4;
            let a284 = 0u64;
            nat_rem(a282, a283, a284)
        }, {
            let a286 = v5;
            let a287 = match v2.clone() {
                Adt1::C0(_, _, h285, _, _, _, _, _, _, _, _) => {
                    h285
                }
            };
            let a288 = 0u64;
            nat_rem(a286, a287, a288)
        }, {
            let a290 = v6;
            let a291 = match v2.clone() {
                Adt1::C0(_, _, _, h289, _, _, _, _, _, _, _) => {
                    h289
                }
            };
            let a292 = 0u64;
            nat_rem(a290, a291, a292)
        }, {
            let a294 = v6;
            let a295 = match v2.clone() {
                Adt1::C0(_, _, _, h293, _, _, _, _, _, _, _) => {
                    h293
                }
            };
            let a296 = 0u64;
            nat_quot(a294, a295, a296)
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, h297, _, _, _, _, _) => {
                h297
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h298, _, _, _, _) => {
                h298
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h299, _, _, _) => {
                h299
            }
        }, match v2 {
            Adt1::C0(_, _, _, _, _, _, _, _, h300, _, _) => {
                h300
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f9(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h301, _, _) => {
            h301
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h302) => {
            h302
        }
    };
    let a304 = v0;
    let a305 = match v1.clone() {
        Adt2::C0(h303, _, _, _, _, _, _, _, _, _) => {
            h303
        }
    };
    if nat_lt(a304, a305) {
        Ok(Adt3::C5({
            let a307 = v0;
            let a308 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, _, h306, _, _, _, _) => {
                    h306
                }
            };
            nat_mul(a307, a308)?
        }, {
            let a310 = v0;
            let a311 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h309, _, _, _, _) => {
                    h309
                }
            };
            nat_mul(a310, a311)?
        }, match v2 {
            Adt1::C0(_, h312, _, _, _, _, _, _, _, _, _) => {
                h312
            }
        }, match v3 {
            Adt1::C0(_, h313, _, _, _, _, _, _, _, _, _) => {
                h313
            }
        }, {
            let a315 = match v1 {
                Adt2::C0(_, _, _, _, h314, _, _, _, _, _) => {
                    h314
                }
            };
            let a316 = 1u64;
            nat_eq(a315, a316)
        }))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f10(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let a318 = v0;
    let a319 = match v1.clone() {
        Adt2::C0(h317, _, _, _, _, _, _, _, _, _) => {
            h317
        }
    };
    if nat_lt(a318, a319) {
        let v3: u64 = {
            let a321 = 2u64;
            let a322 = f24(match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h320, _) => {
                    h320
                }
            })?;
            nat_mul(a321, a322)?
        };
        let a323 = v0;
        let a324 = v3;
        if nat_lt(a323, a324) {
            f41(v0, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h325, _) => {
                    h325
                }
            }, {
                let m328 = match v1 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h327) => {
                        h327
                    }
                };
                match m328 {
                    Adt1::C0(_, _, h326, _, _, _, _, _, _, _, _) => {
                        h326
                    }
                }
            }, v2)
        } else {
            f42({
                let a329 = v0;
                let a330 = v3;
                nat_sub(a329, a330)
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h331, _, _) => {
                    h331
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h332, _) => {
                    h332
                }
            }, match v1 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h333) => {
                    h333
                }
            }, v2)
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f11(v0: u64) -> bool {
    let a334 = v0;
    let a335 = 3u64;
    if nat_eq(a334, a335) {
        true
    } else {
        let a336 = v0;
        let a337 = 6u64;
        nat_eq(a336, a337)
    }
}

pub fn f12(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h338, _, _, _) => {
            h338
        }
    };
    let a340 = v0;
    let a341 = match v1.clone() {
        Adt2::C0(h339, _, _, _, _, _, _, _, _, _) => {
            h339
        }
    };
    if nat_lt(a340, a341) {
        let a342 = v0;
        let a343 = 0u64;
        if nat_eq(a342, a343) {
            Ok(Adt3::C3(1u64, 6u64, 0u64, 0u64, 2u64, v3, 2u64, 0u64, 0u64, 3u64, 4u64, 4u64))
        } else {
            let v4: u64 = {
                let a344 = v0;
                let a345 = 1u64;
                nat_sub(a344, a345)
            };
            let v5: u64 = {
                let a347 = v4;
                let a348 = f38(match v1.clone() {
                    Adt2::C0(_, _, _, _, h346, _, _, _, _, _) => {
                        h346
                    }
                });
                let a349 = 0u64;
                nat_rem(a347, a348, a349)
            };
            let v6: u64 = {
                let a354 = 4u64;
                let a355 = {
                    let a351 = v4;
                    let a352 = f38(match v1.clone() {
                        Adt2::C0(_, _, _, _, h350, _, _, _, _, _) => {
                            h350
                        }
                    });
                    let a353 = 0u64;
                    nat_quot(a351, a352, a353)
                };
                nat_mul(a354, a355)?
            };
            let a356 = v5;
            let a357 = 2u64;
            if nat_lt(a356, a357) {
                f13({
                    let a358 = v6;
                    let a359 = v5;
                    nat_add(a358, a359)?
                }, v1)
            } else {
                let a360 = v5;
                let a361 = 2u64;
                if nat_eq(a360, a361) {
                    Ok(f43(0u64, 2u64, v3))
                } else {
                    let a362 = v5;
                    let a363 = 3u64;
                    if nat_eq(a362, a363) {
                        Ok(f44(0u64, 2u64, v3, {
                            let a364 = v2;
                            let a365 = 2u64;
                            nat_eq(a364, a365)
                        }))
                    } else {
                        let a366 = v5;
                        let a367 = 4u64;
                        if nat_eq(a366, a367) {
                            Ok(f43(1u64, 1u64, 3u64))
                        } else {
                            let a368 = v5;
                            let a369 = 5u64;
                            if nat_eq(a368, a369) {
                                Ok(f44(1u64, 1u64, 3u64, {
                                    let a370 = v2;
                                    let a371 = 0u64;
                                    nat_eq(a370, a371)
                                }))
                            } else {
                                let a372 = v5;
                                let a373 = 6u64;
                                if nat_eq(a372, a373) {
                                    Ok(f43(1u64, 2u64, v3))
                                } else {
                                    let a374 = v5;
                                    let a375 = 7u64;
                                    if nat_eq(a374, a375) {
                                        Ok(f44(1u64, 2u64, v3, {
                                            let a376 = v2;
                                            let a377 = 2u64;
                                            nat_eq(a376, a377)
                                        }))
                                    } else {
                                        let a379 = match v1.clone() {
                                            Adt2::C0(_, _, _, _, h378, _, _, _, _, _) => {
                                                h378
                                            }
                                        };
                                        let a380 = 3u64;
                                        if nat_eq(a379, a380) {
                                            let a381 = v5;
                                            let a382 = 15u64;
                                            if nat_eq(a381, a382) {
                                                f13({
                                                    let a383 = v6;
                                                    let a384 = 3u64;
                                                    nat_add(a383, a384)?
                                                }, v1)
                                            } else {
                                                Ok(f45(v5, match v1 {
                                                    Adt2::C0(_, _, _, _, _, h385, _, _, _, _) => {
                                                        h385
                                                    }
                                                }))
                                            }
                                        } else {
                                            let a386 = v5;
                                            let a387 = 8u64;
                                            if nat_eq(a386, a387) {
                                                Ok(Adt3::C4(2u64, {
                                                    let m390 = match v1 {
                                                        Adt2::C0(_, _, _, _, _, _, _, _, _, h389) => {
                                                            h389
                                                        }
                                                    };
                                                    match m390 {
                                                        Adt1::C0(h388, _, _, _, _, _, _, _, _, _, _) => {
                                                            h388
                                                        }
                                                    }
                                                }, 0u64, 1u64, 0u64))
                                            } else {
                                                f13({
                                                    let a391 = v6;
                                                    let a392 = 3u64;
                                                    nat_add(a391, a392)?
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
        Adt2::C0(_, _, _, _, _, _, _, h393, _, _) => {
            h393
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h394, _) => {
            h394
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h395) => {
            h395
        }
    };
    let v5: u64 = match v1.clone() {
        Adt2::C0(_, _, _, h396, _, _, _, _, _, _) => {
            h396
        }
    };
    let v6: u64 = {
        let a397 = v0;
        let a398 = 4u64;
        let a399 = 0u64;
        nat_rem(a397, a398, a399)
    };
    let v7: u64 = {
        let a400 = v0;
        let a401 = 4u64;
        let a402 = 0u64;
        nat_quot(a400, a401, a402)
    };
    let v8: u64 = {
        let a404 = v7;
        let a405 = match v4.clone() {
            Adt1::C0(_, h403, _, _, _, _, _, _, _, _, _) => {
                h403
            }
        };
        let a406 = 0u64;
        nat_quot(a404, a405, a406)
    };
    let v9: u64 = f34(match v4.clone() {
        Adt1::C0(h407, _, _, _, _, _, _, _, _, _, _) => {
            h407
        }
    });
    let v10: u64 = {
        let a412 = {
            let a409 = v7;
            let a410 = match v4.clone() {
                Adt1::C0(_, h408, _, _, _, _, _, _, _, _, _) => {
                    h408
                }
            };
            let a411 = 0u64;
            nat_rem(a409, a410, a411)
        };
        let a413 = v9;
        nat_mul(a412, a413)?
    };
    let v11: u64 = {
        let a415 = match v4.clone() {
            Adt1::C0(_, h414, _, _, _, _, _, _, _, _, _) => {
                h414
            }
        };
        let a416 = v9;
        nat_mul(a415, a416)?
    };
    let a418 = v0;
    let a419 = match v1.clone() {
        Adt2::C0(h417, _, _, _, _, _, _, _, _, _) => {
            h417
        }
    };
    if nat_lt(a418, a419) {
        let a420 = v6;
        let a421 = 0u64;
        if nat_eq(a420, a421) {
            Ok(Adt3::C3(0u64, 0u64, match v2.clone() {
                Adt1::C0(h422, _, _, _, _, _, _, _, _, _, _) => {
                    h422
                }
            }, 0u64, 0u64, {
                let a430 = {
                    let a426 = {
                        let a424 = v8;
                        let a425 = match v2 {
                            Adt1::C0(_, _, _, _, _, _, h423, _, _, _, _) => {
                                h423
                            }
                        };
                        nat_mul(a424, a425)?
                    };
                    let a427 = v10;
                    nat_add(a426, a427)?
                };
                let a431 = {
                    let a428 = v5;
                    let a429 = 100u64;
                    if nat_eq(a428, a429) {
                        v11
                    } else {
                        0u64
                    }
                };
                nat_add(a430, a431)?
            }, 2u64, 0u64, 0u64, 0u64, v9, 4u64))
        } else {
            let a432 = v6;
            let a433 = 1u64;
            if nat_eq(a432, a433) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h434, _, _, _, _, _, _, _, _, _, _) => {
                        h434
                    }
                }, 0u64, {
                    let a435 = v5;
                    let a436 = 98u64;
                    if nat_eq(a435, a436) {
                        4u64
                    } else {
                        0u64
                    }
                }, {
                    let a444 = {
                        let a440 = {
                            let a438 = v8;
                            let a439 = match v3 {
                                Adt1::C0(_, _, _, _, _, _, h437, _, _, _, _) => {
                                    h437
                                }
                            };
                            nat_mul(a438, a439)?
                        };
                        let a441 = v10;
                        nat_add(a440, a441)?
                    };
                    let a445 = {
                        let a442 = v5;
                        let a443 = 99u64;
                        if nat_eq(a442, a443) {
                            v11
                        } else {
                            0u64
                        }
                    };
                    nat_add(a444, a445)?
                }, 2u64, 0u64, 0u64, 1u64, v9, 4u64))
            } else {
                let a446 = v6;
                let a447 = 2u64;
                if nat_eq(a446, a447) {
                    Ok(Adt3::C4(match v1 {
                        Adt2::C0(_, _, _, _, h448, _, _, _, _, _) => {
                            h448
                        }
                    }, match v4 {
                        Adt1::C0(h449, _, _, _, _, _, _, _, _, _, _) => {
                            h449
                        }
                    }, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt3::C3(0u64, 0u64, 0u64, match v4.clone() {
                        Adt1::C0(h450, _, _, _, _, _, _, _, _, _, _) => {
                            h450
                        }
                    }, 1u64, 0u64, 2u64, 0u64, 3u64, {
                        let a454 = {
                            let a452 = v8;
                            let a453 = match v4 {
                                Adt1::C0(_, _, _, _, _, _, h451, _, _, _, _) => {
                                    h451
                                }
                            };
                            nat_mul(a452, a453)?
                        };
                        let a455 = v10;
                        nat_add(a454, a455)?
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
        Adt2::C0(_, _, _, _, _, _, _, h456, _, _) => {
            h456
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h457, _) => {
            h457
        }
    };
    let v5: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h458) => {
            h458
        }
    };
    let v6: u64 = {
        let a460 = match v1.clone() {
            Adt2::C0(_, h459, _, _, _, _, _, _, _, _) => {
                h459
            }
        };
        let a461 = 3u64;
        if nat_eq(a460, a461) {
            0u64
        } else {
            let a462 = f24(v3.clone())?;
            let a463 = 2u64;
            nat_mul(a462, a463)?
        }
    };
    let a465 = match v1.clone() {
        Adt2::C0(h464, _, _, _, _, _, _, _, _, _) => {
            h464
        }
    };
    let a466 = v0;
    if nat_le(a465, a466) {
        Ok(Adt3::C1)
    } else {
        let a467 = v0;
        let a468 = v6;
        if nat_lt(a467, a468) {
            let v7: u64 = {
                let a469 = v0;
                let a470 = 2u64;
                let a471 = 0u64;
                nat_quot(a469, a470, a471)
            };
            let v8: u64 = {
                let a476 = 3u64;
                let a477 = {
                    let a473 = f39(v7, v5.clone())?;
                    let a474 = f18(match v5 {
                        Adt1::C0(h472, _, _, _, _, _, _, _, _, _, _) => {
                            h472
                        }
                    });
                    let a475 = 0u64;
                    nat_quot(a473, a474, a475)
                };
                nat_add(a476, a477)?
            };
            let a481 = {
                let a478 = v0;
                let a479 = 2u64;
                let a480 = 0u64;
                nat_rem(a478, a479, a480)
            };
            let a482 = 0u64;
            if nat_eq(a481, a482) {
                Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, v8, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
            } else {
                let a483 = v2;
                let a484 = 0u64;
                if nat_eq(a483, a484) {
                    Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 2u64, 1u64, 2u64, 0u64, 0u64, v8, 8u64, 8u64))
                } else {
                    Ok(Adt3::C1)
                }
            }
        } else {
            let v9: u64 = {
                let a485 = v0;
                let a486 = v6;
                nat_sub(a485, a486)
            };
            let v10: u64 = f25(match v3.clone() {
                Adt1::C0(h487, _, _, _, _, _, _, _, _, _, _) => {
                    h487
                }
            }, match v4.clone() {
                Adt1::C0(h488, _, _, _, _, _, _, _, _, _, _) => {
                    h488
                }
            }, match v5.clone() {
                Adt1::C0(h489, _, _, _, _, _, _, _, _, _, _) => {
                    h489
                }
            })?;
            let v11: u64 = {
                let a490 = v9;
                let a491 = v10;
                let a492 = 0u64;
                nat_quot(a490, a491, a492)
            };
            let v12: u64 = {
                let a493 = v9;
                let a494 = v10;
                let a495 = 0u64;
                nat_rem(a493, a494, a495)
            };
            let v13: u64 = {
                let a497 = match v3.clone() {
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
            let v14: u64 = {
                let a500 = match v4.clone() {
                    Adt1::C0(h499, _, _, _, _, _, _, _, _, _, _) => {
                        h499
                    }
                };
                let a501 = 0u64;
                if nat_eq(a500, a501) {
                    0u64
                } else {
                    1u64
                }
            };
            let a502 = v12;
            let a503 = v13;
            if nat_lt(a502, a503) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h504, _, _, _, _, _, _, _, _, _, _) => {
                        h504
                    }
                }, 0u64, 0u64, f39(v11, v3.clone())?, 2u64, 0u64, 0u64, 0u64, f18(match v3 {
                    Adt1::C0(h505, _, _, _, _, _, _, _, _, _, _) => {
                        h505
                    }
                }), 4u64))
            } else {
                let a508 = v12;
                let a509 = {
                    let a506 = v13;
                    let a507 = v14;
                    nat_add(a506, a507)?
                };
                if nat_lt(a508, a509) {
                    Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                        Adt1::C0(h510, _, _, _, _, _, _, _, _, _, _) => {
                            h510
                        }
                    }, 0u64, 4u64, f46(v11, v3, v4.clone())?, 2u64, 0u64, 0u64, 1u64, f18(match v4 {
                        Adt1::C0(h511, _, _, _, _, _, _, _, _, _, _) => {
                            h511
                        }
                    }), 4u64))
                } else {
                    let a514 = v12;
                    let a515 = {
                        let a512 = v13;
                        let a513 = v14;
                        nat_add(a512, a513)?
                    };
                    if nat_eq(a514, a515) {
                        Ok(Adt3::C3(1u64, match v1 {
                            Adt2::C0(_, _, _, h516, _, _, _, _, _, _) => {
                                h516
                            }
                        }, 0u64, 0u64, {
                            let a517 = v13;
                            let a518 = 0u64;
                            if nat_eq(a517, a518) {
                                0u64
                            } else {
                                1u64
                            }
                        }, {
                            let a519 = v13;
                            let a520 = 0u64;
                            if nat_eq(a519, a520) {
                                f39(v11, v3.clone())?
                            } else {
                                0u64
                            }
                        }, {
                            let a521 = v14;
                            let a522 = 0u64;
                            if nat_eq(a521, a522) {
                                4u64
                            } else {
                                1u64
                            }
                        }, {
                            let a523 = v14;
                            let a524 = 0u64;
                            if nat_eq(a523, a524) {
                                f46(v11, v3, v4)?
                            } else {
                                1u64
                            }
                        }, {
                            let a526 = match v5.clone() {
                                Adt1::C0(h525, _, _, _, _, _, _, _, _, _, _) => {
                                    h525
                                }
                            };
                            let a527 = 0u64;
                            if nat_eq(a526, a527) {
                                3u64
                            } else {
                                0u64
                            }
                        }, {
                            let a529 = match v5.clone() {
                                Adt1::C0(h528, _, _, _, _, _, _, _, _, _, _) => {
                                    h528
                                }
                            };
                            let a530 = 0u64;
                            if nat_eq(a529, a530) {
                                f39(v11, v5)?
                            } else {
                                0u64
                            }
                        }, 4u64, 4u64))
                    } else {
                        Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                            Adt1::C0(h531, _, _, _, _, _, _, _, _, _, _) => {
                                h531
                            }
                        }, 1u64, 0u64, 2u64, 0u64, 3u64, f39(v11, v5.clone())?, 4u64, f18(match v5 {
                            Adt1::C0(h532, _, _, _, _, _, _, _, _, _, _) => {
                                h532
                            }
                        })))
                    }
                }
            }
        }
    }
}

pub fn f15(v0: Adt1, v1: Adt1) -> bool {
    let a535 = match v0.clone() {
        Adt1::C0(_, h533, _, _, _, _, _, _, _, _, _) => {
            h533
        }
    };
    let a536 = match v1.clone() {
        Adt1::C0(_, h534, _, _, _, _, _, _, _, _, _) => {
            h534
        }
    };
    if if if nat_eq(a535, a536) {
        let a539 = match v0.clone() {
            Adt1::C0(_, _, h537, _, _, _, _, _, _, _, _) => {
                h537
            }
        };
        let a540 = match v1.clone() {
            Adt1::C0(_, _, h538, _, _, _, _, _, _, _, _) => {
                h538
            }
        };
        nat_eq(a539, a540)
    } else {
        false
    } {
        let a543 = match v0.clone() {
            Adt1::C0(_, _, _, h541, _, _, _, _, _, _, _) => {
                h541
            }
        };
        let a544 = match v1.clone() {
            Adt1::C0(_, _, _, h542, _, _, _, _, _, _, _) => {
                h542
            }
        };
        nat_eq(a543, a544)
    } else {
        false
    } {
        let a547 = match v0 {
            Adt1::C0(_, _, _, _, h545, _, _, _, _, _, _) => {
                h545
            }
        };
        let a548 = match v1 {
            Adt1::C0(_, _, _, _, h546, _, _, _, _, _, _) => {
                h546
            }
        };
        nat_eq(a547, a548)
    } else {
        false
    }
}

pub fn f16(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<bool> {
    let a550 = match v1.clone() {
        Adt1::C0(h549, _, _, _, _, _, _, _, _, _, _) => {
            h549
        }
    };
    let a551 = 43u64;
    if nat_eq(a550, a551) {
        let a552 = v0;
        let a553 = 0u64;
        if if if if if if nat_eq(a552, a553) {
            let a555 = match v2.clone() {
                Adt1::C0(h554, _, _, _, _, _, _, _, _, _, _) => {
                    h554
                }
            };
            let a556 = 0u64;
            nat_eq(a555, a556)
        } else {
            false
        } {
            let a558 = match v3.clone() {
                Adt1::C0(h557, _, _, _, _, _, _, _, _, _, _) => {
                    h557
                }
            };
            let a559 = 0u64;
            if nat_eq(a558, a559) {
                true
            } else {
                let a561 = match v3.clone() {
                    Adt1::C0(h560, _, _, _, _, _, _, _, _, _, _) => {
                        h560
                    }
                };
                let a562 = 43u64;
                nat_eq(a561, a562)
            }
        } else {
            false
        } {
            f15(v1.clone(), v2.clone())
        } else {
            false
        } {
            let a564 = match v2 {
                Adt1::C0(_, _, _, _, _, h563, _, _, _, _, _) => {
                    h563
                }
            };
            let a565 = 4u64;
            nat_eq(a564, a565)
        } else {
            false
        } {
            let a568 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h566, _, _, _, _, _) => {
                    h566
                }
            };
            let a569 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h567, _, _, _, _) => {
                    h567
                }
            };
            if nat_le(a568, a569) {
                let a572 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, h570, _, _, _, _) => {
                        h570
                    }
                };
                let a573 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h571, _, _, _) => {
                        h571
                    }
                };
                if nat_le(a572, a573) {
                    let a576 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, h574, _, _, _) => {
                            h574
                        }
                    };
                    let a577 = match v3 {
                        Adt1::C0(_, _, _, _, _, _, _, _, h575, _, _) => {
                            h575
                        }
                    };
                    nat_le(a576, a577)
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        } {
            let a585 = {
                let a583 = match v1.clone() {
                    Adt1::C0(_, _, h578, _, _, _, _, _, _, _, _) => {
                        h578
                    }
                };
                let a584 = {
                    let a581 = match v1.clone() {
                        Adt1::C0(_, _, _, h579, _, _, _, _, _, _, _) => {
                            h579
                        }
                    };
                    let a582 = match v1 {
                        Adt1::C0(_, _, _, _, h580, _, _, _, _, _, _) => {
                            h580
                        }
                    };
                    nat_mul(a581, a582)?
                };
                nat_mul(a583, a584)?
            };
            let a586 = 2147483647u64;
            Ok(nat_le(a585, a586))
        } else {
            Ok(false)
        }
    } else {
        if f47(v1.clone(), v2.clone()) {
            Ok(f48(match v1 {
                Adt1::C0(h587, _, _, _, _, _, _, _, _, _, _) => {
                    h587
                }
            }, match v2 {
                Adt1::C0(h588, _, _, _, _, _, _, _, _, _, _) => {
                    h588
                }
            }, match v3 {
                Adt1::C0(h589, _, _, _, _, _, _, _, _, _, _) => {
                    h589
                }
            }))
        } else {
            Ok(false)
        }
    }
}

pub fn f17(v0: Adt1) -> bool {
    f49(match v0.clone() {
        Adt1::C0(_, h590, _, _, _, _, _, _, _, _, _) => {
            h590
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h591, _, _, _, _, _, _, _, _) => {
            h591
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h592, _, _, _, _, _, _, _) => {
            h592
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, h593, _, _, _, _, _, _) => {
            h593
        }
    })
}

pub fn f18(v0: u64) -> u64 {
    let a594 = v0;
    let a595 = 43u64;
    if nat_eq(a594, a595) {
        1u64
    } else {
        f34(v0)
    }
}

pub fn f19(v0: Adt1) -> bool {
    let v1: u64 = f18(match v0.clone() {
        Adt1::C0(h596, _, _, _, _, _, _, _, _, _, _) => {
            h596
        }
    });
    let a601 = {
        let a598 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h597) => {
                h597
            }
        };
        let a599 = v1;
        let a600 = 1u64;
        nat_rem(a598, a599, a600)
    };
    let a602 = 0u64;
    if if nat_eq(a601, a602) {
        f50(match v0.clone() {
            Adt1::C0(_, h603, _, _, _, _, _, _, _, _, _) => {
                h603
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h604, _, _, _, _, _) => {
                h604
            }
        }, v1)
    } else {
        false
    } {
        f51(match v0.clone() {
            Adt1::C0(_, _, h605, _, _, _, _, _, _, _, _) => {
                h605
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h606, _, _, _, _, _, _, _) => {
                h606
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h607, _, _, _, _, _, _) => {
                h607
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h608, _, _, _, _) => {
                h608
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h609, _, _, _) => {
                h609
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h610, _, _) => {
                h610
            }
        }, v1)
    } else {
        false
    }
}

pub fn f20(v0: Adt1, v1: Adt1) -> R<bool> {
    let a618 = {
        let a616 = match v0.clone() {
            Adt1::C0(_, _, h611, _, _, _, _, _, _, _, _) => {
                h611
            }
        };
        let a617 = {
            let a614 = match v0.clone() {
                Adt1::C0(_, _, _, h612, _, _, _, _, _, _, _) => {
                    h612
                }
            };
            let a615 = match v0.clone() {
                Adt1::C0(_, _, _, _, h613, _, _, _, _, _, _) => {
                    h613
                }
            };
            nat_mul(a614, a615)?
        };
        nat_mul(a616, a617)?
    };
    let a619 = 9223372036854775807u64;
    if nat_le(a618, a619) {
        if f30(v1.clone()) {
            let a621 = match v1 {
                Adt1::C0(_, h620, _, _, _, _, _, _, _, _, _) => {
                    h620
                }
            };
            let a622 = 2147483647u64;
            Ok(nat_le(a621, a622))
        } else {
            let a624 = match v0 {
                Adt1::C0(_, h623, _, _, _, _, _, _, _, _, _) => {
                    h623
                }
            };
            let a625 = 2147483647u64;
            Ok(nat_le(a624, a625))
        }
    } else {
        Ok(false)
    }
}

pub fn f21(v0: Adt1, v1: Adt1) -> R<bool> {
    if f36(v0.clone(), v1.clone())? {
        Ok(true)
    } else {
        Ok(f52(v0, v1))
    }
}

pub fn f22(v0: Adt1) -> R<bool> {
    f53({
        let a628 = match v0.clone() {
            Adt1::C0(_, h626, _, _, _, _, _, _, _, _, _) => {
                h626
            }
        };
        let a629 = f18(match v0.clone() {
            Adt1::C0(h627, _, _, _, _, _, _, _, _, _, _) => {
                h627
            }
        });
        nat_mul(a628, a629)?
    }, match v0.clone() {
        Adt1::C0(_, _, h630, _, _, _, _, _, _, _, _) => {
            h630
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h631, _, _, _, _, _, _, _) => {
            h631
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h632, _, _, _, _, _, _) => {
            h632
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h633, _, _, _, _) => {
            h633
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h634, _, _, _) => {
            h634
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h635, _, _) => {
            h635
        }
    })
}

pub fn f23(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a642 = {
        let a636 = v0;
        let a637 = v2;
        nat_mul(a636, a637)?
    };
    let a643 = {
        let a638 = v1;
        let a639 = 0u64;
        if nat_eq(a638, a639) {
            0u64
        } else {
            let a640 = v0;
            let a641 = 2u64;
            nat_mul(a640, a641)?
        }
    };
    nat_add(a642, a643)
}

pub fn f24(v0: Adt1) -> R<u64> {
    let a652 = {
        let a646 = match v0.clone() {
            Adt1::C0(_, h644, _, _, _, _, _, _, _, _, _) => {
                h644
            }
        };
        let a647 = match v0.clone() {
            Adt1::C0(_, _, h645, _, _, _, _, _, _, _, _) => {
                h645
            }
        };
        nat_mul(a646, a647)?
    };
    let a653 = {
        let a650 = match v0.clone() {
            Adt1::C0(_, _, _, h648, _, _, _, _, _, _, _) => {
                h648
            }
        };
        let a651 = match v0 {
            Adt1::C0(_, _, _, _, h649, _, _, _, _, _, _) => {
                h649
            }
        };
        nat_mul(a650, a651)?
    };
    nat_mul(a652, a653)
}

pub fn f25(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a664 = {
        let a660 = {
            let a656 = 1u64;
            let a657 = {
                let a654 = v0;
                let a655 = 0u64;
                if nat_eq(a654, a655) {
                    0u64
                } else {
                    1u64
                }
            };
            nat_add(a656, a657)?
        };
        let a661 = {
            let a658 = v1;
            let a659 = 0u64;
            if nat_eq(a658, a659) {
                0u64
            } else {
                1u64
            }
        };
        nat_add(a660, a661)?
    };
    let a665 = {
        let a662 = v2;
        let a663 = 0u64;
        if nat_eq(a662, a663) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a664, a665)
}

pub fn f26(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f15(v0.clone(), v1.clone()) {
        let a668 = match v0.clone() {
            Adt1::C0(h666, _, _, _, _, _, _, _, _, _, _) => {
                h666
            }
        };
        let a669 = match v1.clone() {
            Adt1::C0(h667, _, _, _, _, _, _, _, _, _, _) => {
                h667
            }
        };
        nat_eq(a668, a669)
    } else {
        false
    } {
        let a672 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h670, _, _, _, _, _) => {
                h670
            }
        };
        let a673 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h671, _, _, _, _, _) => {
                h671
            }
        };
        nat_eq(a672, a673)
    } else {
        false
    } {
        let a676 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h674, _, _, _, _) => {
                h674
            }
        };
        let a677 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h675, _, _, _, _) => {
                h675
            }
        };
        nat_eq(a676, a677)
    } else {
        false
    } {
        let a680 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h678, _, _, _) => {
                h678
            }
        };
        let a681 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h679, _, _, _) => {
                h679
            }
        };
        nat_eq(a680, a681)
    } else {
        false
    } {
        let a684 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h682, _, _) => {
                h682
            }
        };
        let a685 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h683, _, _) => {
                h683
            }
        };
        nat_eq(a684, a685)
    } else {
        false
    } {
        let a687 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h686) => {
                h686
            }
        };
        let a688 = 0u64;
        if if nat_eq(a687, a688) {
            true
        } else {
            let a690 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h689) => {
                    h689
                }
            };
            let a691 = 0u64;
            nat_eq(a690, a691)
        } {
            true
        } else {
            let a694 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h692) => {
                    h692
                }
            };
            let a695 = match v1 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h693) => {
                    h693
                }
            };
            nat_eq(a694, a695)
        }
    } else {
        false
    }
}

pub fn f27(v0: u64) -> u64 {
    if f33(v0) {
        let m696 = f54(v0);
        match m696 {
            Adt4::C0 => {
                0u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, _, h697) => {
                        h697
                    }
                }
            }
        }
    } else {
        f18(v0)
    }
}

pub fn f28(v0: u64) -> u64 {
    let a698 = v0;
    let a699 = 26u64;
    if nat_eq(a698, a699) {
        4u64
    } else {
        let a700 = v0;
        let a701 = 27u64;
        if nat_eq(a700, a701) {
            8u64
        } else {
            0u64
        }
    }
}

pub fn f29(v0: u64) -> u64 {
    if f33(v0) {
        let m702 = f54(v0);
        match m702 {
            Adt4::C0 => {
                1u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, h703, _) => {
                        h703
                    }
                }
            }
        }
    } else {
        1u64
    }
}

pub fn f30(v0: Adt1) -> bool {
    let a705 = match v0.clone() {
        Adt1::C0(_, h704, _, _, _, _, _, _, _, _, _) => {
            h704
        }
    };
    let a706 = 1u64;
    if nat_eq(a705, a706) {
        true
    } else {
        let a709 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h707, _, _, _, _, _) => {
                h707
            }
        };
        let a710 = f18(match v0 {
            Adt1::C0(h708, _, _, _, _, _, _, _, _, _, _) => {
                h708
            }
        });
        nat_eq(a709, a710)
    }
}

pub fn f31(v0: Adt1, v1: u64) -> R<u64> {
    let a716 = {
        let a712 = match v0.clone() {
            Adt1::C0(_, h711, _, _, _, _, _, _, _, _, _) => {
                h711
            }
        };
        let a713 = f29(v1);
        let a714 = 0u64;
        nat_quot(a712, a713, a714)
    };
    let a717 = f55(match v0 {
        Adt1::C0(h715, _, _, _, _, _, _, _, _, _, _) => {
            h715
        }
    }, v1);
    nat_mul(a716, a717)
}

pub fn f32(v0: u64) -> Option<u64> {
    let a718 = v0;
    let a719 = 17u64;
    if nat_eq(a718, a719) {
        Some(1065353216u64)
    } else {
        None::<u64>
    }
}

pub fn f33(v0: u64) -> bool {
    let a720 = 17u64;
    let a721 = v0;
    if if if if nat_le(a720, a721) {
        let a722 = v0;
        let a723 = 18u64;
        nat_le(a722, a723)
    } else {
        false
    } {
        true
    } else {
        let a724 = 20u64;
        let a725 = v0;
        if nat_le(a724, a725) {
            let a726 = v0;
            let a727 = 23u64;
            nat_le(a726, a727)
        } else {
            false
        }
    } {
        true
    } else {
        let a728 = 10u64;
        let a729 = v0;
        if nat_le(a728, a729) {
            let a730 = v0;
            let a731 = 14u64;
            nat_le(a730, a731)
        } else {
            false
        }
    } {
        true
    } else {
        let a732 = v0;
        let a733 = 2u64;
        if if if if nat_eq(a732, a733) {
            true
        } else {
            let a734 = v0;
            let a735 = 3u64;
            nat_eq(a734, a735)
        } {
            true
        } else {
            let a736 = v0;
            let a737 = 6u64;
            nat_eq(a736, a737)
        } {
            true
        } else {
            let a738 = v0;
            let a739 = 7u64;
            if nat_eq(a738, a739) {
                true
            } else {
                let a740 = v0;
                let a741 = 8u64;
                nat_eq(a740, a741)
            }
        } {
            true
        } else {
            let a742 = 34u64;
            let a743 = v0;
            if if nat_le(a742, a743) {
                let a744 = v0;
                let a745 = 35u64;
                nat_le(a744, a745)
            } else {
                false
            } {
                true
            } else {
                let a746 = 39u64;
                let a747 = v0;
                if nat_le(a746, a747) {
                    let a748 = v0;
                    let a749 = 42u64;
                    nat_le(a748, a749)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f34(v0: u64) -> u64 {
    let a750 = v0;
    let a751 = 0u64;
    if nat_eq(a750, a751) {
        4u64
    } else {
        let a752 = v0;
        let a753 = 1u64;
        if if nat_eq(a752, a753) {
            true
        } else {
            let a754 = v0;
            let a755 = 30u64;
            nat_eq(a754, a755)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f35(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a757 = match v0.clone() {
            Adt1::C0(_, h756, _, _, _, _, _, _, _, _, _) => {
                h756
            }
        };
        let a758 = 4u64;
        nat_mul(a757, a758)?
    };
    let v2: u64 = {
        let a761 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h759, _, _, _, _) => {
                h759
            }
        };
        let a762 = match v0.clone() {
            Adt1::C0(_, _, h760, _, _, _, _, _, _, _, _) => {
                h760
            }
        };
        nat_mul(a761, a762)?
    };
    let v3: u64 = {
        let a764 = v2;
        let a765 = match v0.clone() {
            Adt1::C0(_, _, _, h763, _, _, _, _, _, _, _) => {
                h763
            }
        };
        nat_mul(a764, a765)?
    };
    let a767 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h766, _, _, _, _, _) => {
            h766
        }
    };
    let a768 = 4u64;
    if if if if nat_eq(a767, a768) {
        let a773 = {
            let a770 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h769) => {
                    h769
                }
            };
            let a771 = 4u64;
            let a772 = 1u64;
            nat_rem(a770, a771, a772)
        };
        let a774 = 0u64;
        nat_eq(a773, a774)
    } else {
        false
    } {
        let a782 = {
            let a780 = match v0.clone() {
                Adt1::C0(_, _, h775, _, _, _, _, _, _, _, _) => {
                    h775
                }
            };
            let a781 = {
                let a778 = match v0.clone() {
                    Adt1::C0(_, _, _, h776, _, _, _, _, _, _, _) => {
                        h776
                    }
                };
                let a779 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h777, _, _, _, _, _, _) => {
                        h777
                    }
                };
                nat_mul(a778, a779)?
            };
            nat_mul(a780, a781)?
        };
        let a783 = 1u64;
        if nat_le(a782, a783) {
            true
        } else {
            let a785 = v1;
            let a786 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h784, _, _, _, _) => {
                    h784
                }
            };
            if nat_le(a785, a786) {
                let a791 = {
                    let a788 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, h787, _, _, _, _) => {
                            h787
                        }
                    };
                    let a789 = 4u64;
                    let a790 = 1u64;
                    nat_rem(a788, a789, a790)
                };
                let a792 = 0u64;
                nat_eq(a791, a792)
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a794 = match v0.clone() {
            Adt1::C0(_, _, _, h793, _, _, _, _, _, _, _) => {
                h793
            }
        };
        let a795 = 1u64;
        if nat_le(a794, a795) {
            true
        } else {
            let a797 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h796, _, _, _) => {
                    h796
                }
            };
            let a798 = v2;
            nat_eq(a797, a798)
        }
    } else {
        false
    } {
        let a800 = match v0.clone() {
            Adt1::C0(_, _, _, _, h799, _, _, _, _, _, _) => {
                h799
            }
        };
        let a801 = 1u64;
        if nat_le(a800, a801) {
            Ok(true)
        } else {
            let a803 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, h802, _, _) => {
                    h802
                }
            };
            let a804 = v3;
            Ok(nat_eq(a803, a804))
        }
    } else {
        Ok(false)
    }
}

pub fn f36(v0: Adt1, v1: Adt1) -> R<bool> {
    let a806 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h805, _) => {
            h805
        }
    };
    let a807 = 0u64;
    if nat_eq(a806, a807) {
        Ok(true)
    } else {
        let a809 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h808, _) => {
                h808
            }
        };
        let a810 = 0u64;
        if nat_eq(a809, a810) {
            Ok(true)
        } else {
            let a812 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h811) => {
                    h811
                }
            };
            let a813 = 0u64;
            if nat_eq(a812, a813) {
                Ok(true)
            } else {
                let a815 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h814) => {
                        h814
                    }
                };
                let a816 = 0u64;
                if nat_eq(a815, a816) {
                    Ok(true)
                } else {
                    let a822 = {
                        let a819 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h817) => {
                                h817
                            }
                        };
                        let a820 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h818, _) => {
                                h818
                            }
                        };
                        nat_add(a819, a820)?
                    };
                    let a823 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h821) => {
                            h821
                        }
                    };
                    if nat_le(a822, a823) {
                        Ok(true)
                    } else {
                        let a829 = {
                            let a826 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h824) => {
                                    h824
                                }
                            };
                            let a827 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h825, _) => {
                                    h825
                                }
                            };
                            nat_add(a826, a827)?
                        };
                        let a830 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h828) => {
                                h828
                            }
                        };
                        Ok(nat_le(a829, a830))
                    }
                }
            }
        }
    }
}

pub fn f37(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a833 = match v0.clone() {
            Adt1::C0(_, _, h831, _, _, _, _, _, _, _, _) => {
                h831
            }
        };
        let a834 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h832, _, _, _, _) => {
                h832
            }
        };
        nat_mul(a833, a834)?
    };
    if if f30(v0.clone()) {
        let a836 = match v0.clone() {
            Adt1::C0(_, _, _, h835, _, _, _, _, _, _, _) => {
                h835
            }
        };
        let a837 = 1u64;
        if nat_eq(a836, a837) {
            true
        } else {
            let a839 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h838, _, _, _) => {
                    h838
                }
            };
            let a840 = v1;
            nat_eq(a839, a840)
        }
    } else {
        false
    } {
        let a842 = match v0.clone() {
            Adt1::C0(_, _, _, _, h841, _, _, _, _, _, _) => {
                h841
            }
        };
        let a843 = 1u64;
        if nat_eq(a842, a843) {
            Ok(true)
        } else {
            let a848 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h844, _, _) => {
                    h844
                }
            };
            let a849 = {
                let a846 = v1;
                let a847 = match v0 {
                    Adt1::C0(_, _, _, h845, _, _, _, _, _, _, _) => {
                        h845
                    }
                };
                nat_mul(a846, a847)?
            };
            Ok(nat_eq(a848, a849))
        }
    } else {
        Ok(false)
    }
}

pub fn f38(v0: u64) -> u64 {
    let a850 = v0;
    let a851 = 3u64;
    if nat_eq(a850, a851) {
        16u64
    } else {
        10u64
    }
}

pub fn f39(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a853 = v0;
        let a854 = match v1.clone() {
            Adt1::C0(_, h852, _, _, _, _, _, _, _, _, _) => {
                h852
            }
        };
        let a855 = 0u64;
        nat_quot(a853, a854, a855)
    };
    let v3: u64 = {
        let a857 = v2;
        let a858 = match v1.clone() {
            Adt1::C0(_, _, h856, _, _, _, _, _, _, _, _) => {
                h856
            }
        };
        let a859 = 0u64;
        nat_quot(a857, a858, a859)
    };
    f40({
        let a861 = v0;
        let a862 = match v1.clone() {
            Adt1::C0(_, h860, _, _, _, _, _, _, _, _, _) => {
                h860
            }
        };
        let a863 = 0u64;
        nat_rem(a861, a862, a863)
    }, {
        let a865 = v2;
        let a866 = match v1.clone() {
            Adt1::C0(_, _, h864, _, _, _, _, _, _, _, _) => {
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
        nat_rem(a869, a870, a871)
    }, {
        let a873 = v3;
        let a874 = match v1.clone() {
            Adt1::C0(_, _, _, h872, _, _, _, _, _, _, _) => {
                h872
            }
        };
        let a875 = 0u64;
        nat_quot(a873, a874, a875)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h876, _, _, _, _, _) => {
            h876
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h877, _, _, _, _) => {
            h877
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h878, _, _, _) => {
            h878
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h879, _, _) => {
            h879
        }
    })
}

pub fn f40(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a892 = {
        let a888 = {
            let a884 = {
                let a880 = v0;
                let a881 = v4;
                nat_mul(a880, a881)?
            };
            let a885 = {
                let a882 = v1;
                let a883 = v5;
                nat_mul(a882, a883)?
            };
            nat_add(a884, a885)?
        };
        let a889 = {
            let a886 = v2;
            let a887 = v6;
            nat_mul(a886, a887)?
        };
        nat_add(a888, a889)?
    };
    let a893 = {
        let a890 = v3;
        let a891 = v7;
        nat_mul(a890, a891)?
    };
    nat_add(a892, a893)
}

pub fn f41(v0: u64, v1: Adt1, v2: u64, v3: u64) -> R<Adt3> {
    let v4: u64 = {
        let a894 = v0;
        let a895 = 2u64;
        let a896 = 0u64;
        nat_quot(a894, a895, a896)
    };
    let v5: u64 = f28(match v1.clone() {
        Adt1::C0(h897, _, _, _, _, _, _, _, _, _, _) => {
            h897
        }
    });
    let a901 = {
        let a898 = v0;
        let a899 = 2u64;
        let a900 = 0u64;
        nat_rem(a898, a899, a900)
    };
    let a902 = 0u64;
    if nat_eq(a901, a902) {
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 4u64, f39(v4, v1)?, 2u64, 0u64, 0u64, 2u64, v5, 8u64))
    } else {
        let a903 = v3;
        let a904 = v2;
        if if nat_lt(a903, a904) {
            let a907 = v3;
            let a908 = {
                let a905 = v5;
                let a906 = 4u64;
                if nat_eq(a905, a906) {
                    2147483648u64
                } else {
                    9223372036854775808u64
                }
            };
            nat_lt(a907, a908)
        } else {
            false
        } {
            Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, {
                let a909 = 3u64;
                let a910 = v4;
                nat_add(a909, a910)?
            }, 8u64, 8u64))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f42(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64) -> R<Adt3> {
    let v5: u64 = {
        let a912 = 1u64;
        let a913 = f31(v1.clone(), match v3.clone() {
            Adt1::C0(h911, _, _, _, _, _, _, _, _, _, _) => {
                h911
            }
        })?;
        nat_add(a912, a913)?
    };
    let v6: u64 = {
        let a914 = v0;
        let a915 = v5;
        let a916 = 0u64;
        nat_rem(a914, a915, a916)
    };
    let v7: u64 = {
        let a917 = v0;
        let a918 = v5;
        let a919 = 0u64;
        nat_quot(a917, a918, a919)
    };
    let v8: u64 = {
        let a921 = v7;
        let a922 = match v1.clone() {
            Adt1::C0(_, _, h920, _, _, _, _, _, _, _, _) => {
                h920
            }
        };
        let a923 = 0u64;
        nat_rem(a921, a922, a923)
    };
    let v9: u64 = {
        let a925 = v7;
        let a926 = match v1.clone() {
            Adt1::C0(_, _, h924, _, _, _, _, _, _, _, _) => {
                h924
            }
        };
        let a927 = 0u64;
        nat_quot(a925, a926, a927)
    };
    let v10: u64 = {
        let a929 = v9;
        let a930 = match v1.clone() {
            Adt1::C0(_, _, _, h928, _, _, _, _, _, _, _) => {
                h928
            }
        };
        let a931 = 0u64;
        nat_rem(a929, a930, a931)
    };
    let v11: u64 = {
        let a933 = v9;
        let a934 = match v1.clone() {
            Adt1::C0(_, _, _, h932, _, _, _, _, _, _, _) => {
                h932
            }
        };
        let a935 = 0u64;
        nat_quot(a933, a934, a935)
    };
    let a936 = v6;
    let a937 = 0u64;
    if nat_eq(a936, a937) {
        let v12: u64 = {
            let a954 = v8;
            let a955 = {
                let a952 = match v2.clone() {
                    Adt1::C0(_, h938, _, _, _, _, _, _, _, _, _) => {
                        h938
                    }
                };
                let a953 = {
                    let a950 = {
                        let a940 = v10;
                        let a941 = match v2.clone() {
                            Adt1::C0(_, _, h939, _, _, _, _, _, _, _, _) => {
                                h939
                            }
                        };
                        let a942 = 0u64;
                        nat_rem(a940, a941, a942)
                    };
                    let a951 = {
                        let a948 = match v2.clone() {
                            Adt1::C0(_, _, h943, _, _, _, _, _, _, _, _) => {
                                h943
                            }
                        };
                        let a949 = {
                            let a945 = v11;
                            let a946 = match v2 {
                                Adt1::C0(_, _, _, h944, _, _, _, _, _, _, _) => {
                                    h944
                                }
                            };
                            let a947 = 0u64;
                            nat_rem(a945, a946, a947)
                        };
                        nat_mul(a948, a949)?
                    };
                    nat_add(a950, a951)?
                };
                nat_mul(a952, a953)?
            };
            nat_add(a954, a955)?
        };
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, {
            let a956 = 3u64;
            let a957 = v12;
            nat_add(a956, a957)?
        }, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
    } else {
        let a959 = v4;
        let a960 = match v3.clone() {
            Adt1::C0(_, _, h958, _, _, _, _, _, _, _, _) => {
                h958
            }
        };
        if nat_lt(a959, a960) {
            let v13: u64 = {
                let a965 = {
                    let a961 = v6;
                    let a962 = 1u64;
                    nat_sub(a961, a962)
                };
                let a966 = f55(match v1.clone() {
                    Adt1::C0(h963, _, _, _, _, _, _, _, _, _, _) => {
                        h963
                    }
                }, match v3.clone() {
                    Adt1::C0(h964, _, _, _, _, _, _, _, _, _, _) => {
                        h964
                    }
                });
                let a967 = 0u64;
                nat_quot(a965, a966, a967)
            };
            let v14: u64 = f40({
                let a969 = v13;
                let a970 = f29(match v3.clone() {
                    Adt1::C0(h968, _, _, _, _, _, _, _, _, _, _) => {
                        h968
                    }
                });
                nat_mul(a969, a970)?
            }, v8, v10, v11, match v1.clone() {
                Adt1::C0(_, _, _, _, _, h971, _, _, _, _, _) => {
                    h971
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, h972, _, _, _, _) => {
                    h972
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h973, _, _, _) => {
                    h973
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h974, _, _) => {
                    h974
                }
            })?;
            let v15: u64 = f40(v13, v4, v10, v11, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h975, _, _, _, _, _) => {
                    h975
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h976, _, _, _, _) => {
                    h976
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h977, _, _, _) => {
                    h977
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h978, _, _) => {
                    h978
                }
            })?;
            if f33(match v3.clone() {
                Adt1::C0(h979, _, _, _, _, _, _, _, _, _, _) => {
                    h979
                }
            }) {
                Ok(Adt3::C6(v14, v15))
            } else {
                Ok(f56(match v1 {
                    Adt1::C0(h980, _, _, _, _, _, _, _, _, _, _) => {
                        h980
                    }
                }, match v3 {
                    Adt1::C0(h981, _, _, _, _, _, _, _, _, _, _) => {
                        h981
                    }
                }, {
                    let a982 = v6;
                    let a983 = 1u64;
                    nat_sub(a982, a983)
                }, v14, v15))
            }
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f43(v0: u64, v1: u64, v2: u64) -> Adt3 {
    Adt3::C3(2u64, 0u64, 0u64, 0u64, 1u64, v0, v1, v2, 0u64, 2u64, 4u64, 4u64)
}

pub fn f44(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt3 {
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

pub fn f45(v0: u64, v1: u64) -> Adt3 {
    let a984 = v0;
    let a985 = 8u64;
    if nat_eq(a984, a985) {
        Adt3::C3(1u64, 6u64, 0u64, 0u64, 1u64, 0u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
    } else {
        let a986 = v0;
        let a987 = 9u64;
        if nat_eq(a986, a987) {
            Adt3::C3(1u64, 2u64, 0u64, 0u64, 2u64, v1, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
        } else {
            let a988 = v0;
            let a989 = 10u64;
            if nat_eq(a988, a989) {
                Adt3::C3(5u64, 16u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
            } else {
                let a990 = v0;
                let a991 = 11u64;
                if nat_eq(a990, a991) {
                    Adt3::C3(1u64, 0u64, 0u64, 0u64, 2u64, 1065353216u64, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
                } else {
                    let a992 = v0;
                    let a993 = 12u64;
                    if nat_eq(a992, a993) {
                        Adt3::C3(1u64, 3u64, 0u64, 0u64, 1u64, 0u64, 1u64, 2u64, 0u64, 0u64, 4u64, 4u64)
                    } else {
                        let a994 = v0;
                        let a995 = 13u64;
                        if nat_eq(a994, a995) {
                            Adt3::C3(1u64, 0u64, 0u64, 0u64, 1u64, 1u64, 2u64, 1065353216u64, 0u64, 1u64, 4u64, 4u64)
                        } else {
                            let a996 = v0;
                            let a997 = 14u64;
                            if nat_eq(a996, a997) {
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

pub fn f46(v0: u64, v1: Adt1, v2: Adt1) -> R<u64> {
    let v3: u64 = {
        let a999 = v0;
        let a1000 = match v1.clone() {
            Adt1::C0(_, h998, _, _, _, _, _, _, _, _, _) => {
                h998
            }
        };
        let a1001 = 0u64;
        nat_quot(a999, a1000, a1001)
    };
    let v4: u64 = {
        let a1003 = v3;
        let a1004 = match v1.clone() {
            Adt1::C0(_, _, h1002, _, _, _, _, _, _, _, _) => {
                h1002
            }
        };
        let a1005 = 0u64;
        nat_quot(a1003, a1004, a1005)
    };
    f40({
        let a1011 = {
            let a1007 = v0;
            let a1008 = match v1.clone() {
                Adt1::C0(_, h1006, _, _, _, _, _, _, _, _, _) => {
                    h1006
                }
            };
            let a1009 = 0u64;
            nat_rem(a1007, a1008, a1009)
        };
        let a1012 = match v2.clone() {
            Adt1::C0(_, h1010, _, _, _, _, _, _, _, _, _) => {
                h1010
            }
        };
        let a1013 = 0u64;
        nat_rem(a1011, a1012, a1013)
    }, {
        let a1019 = {
            let a1015 = v3;
            let a1016 = match v1.clone() {
                Adt1::C0(_, _, h1014, _, _, _, _, _, _, _, _) => {
                    h1014
                }
            };
            let a1017 = 0u64;
            nat_rem(a1015, a1016, a1017)
        };
        let a1020 = match v2.clone() {
            Adt1::C0(_, _, h1018, _, _, _, _, _, _, _, _) => {
                h1018
            }
        };
        let a1021 = 0u64;
        nat_rem(a1019, a1020, a1021)
    }, {
        let a1027 = {
            let a1023 = v4;
            let a1024 = match v1.clone() {
                Adt1::C0(_, _, _, h1022, _, _, _, _, _, _, _) => {
                    h1022
                }
            };
            let a1025 = 0u64;
            nat_rem(a1023, a1024, a1025)
        };
        let a1028 = match v2.clone() {
            Adt1::C0(_, _, _, h1026, _, _, _, _, _, _, _) => {
                h1026
            }
        };
        let a1029 = 0u64;
        nat_rem(a1027, a1028, a1029)
    }, {
        let a1035 = {
            let a1031 = v4;
            let a1032 = match v1 {
                Adt1::C0(_, _, _, h1030, _, _, _, _, _, _, _) => {
                    h1030
                }
            };
            let a1033 = 0u64;
            nat_quot(a1031, a1032, a1033)
        };
        let a1036 = match v2.clone() {
            Adt1::C0(_, _, _, _, h1034, _, _, _, _, _, _) => {
                h1034
            }
        };
        let a1037 = 0u64;
        nat_rem(a1035, a1036, a1037)
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h1038, _, _, _, _, _) => {
            h1038
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h1039, _, _, _, _) => {
            h1039
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1040, _, _, _) => {
            h1040
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1041, _, _) => {
            h1041
        }
    })
}

pub fn f47(v0: Adt1, v1: Adt1) -> bool {
    if f17(v1.clone()) {
        f17(v0)
    } else {
        let a1047 = {
            let a1044 = match v0.clone() {
                Adt1::C0(_, h1042, _, _, _, _, _, _, _, _, _) => {
                    h1042
                }
            };
            let a1045 = match v1.clone() {
                Adt1::C0(_, h1043, _, _, _, _, _, _, _, _, _) => {
                    h1043
                }
            };
            let a1046 = 1u64;
            nat_rem(a1044, a1045, a1046)
        };
        let a1048 = 0u64;
        if if if nat_eq(a1047, a1048) {
            let a1054 = {
                let a1051 = match v0.clone() {
                    Adt1::C0(_, _, h1049, _, _, _, _, _, _, _, _) => {
                        h1049
                    }
                };
                let a1052 = match v1.clone() {
                    Adt1::C0(_, _, h1050, _, _, _, _, _, _, _, _) => {
                        h1050
                    }
                };
                let a1053 = 1u64;
                nat_rem(a1051, a1052, a1053)
            };
            let a1055 = 0u64;
            nat_eq(a1054, a1055)
        } else {
            false
        } {
            let a1061 = {
                let a1058 = match v0.clone() {
                    Adt1::C0(_, _, _, h1056, _, _, _, _, _, _, _) => {
                        h1056
                    }
                };
                let a1059 = match v1.clone() {
                    Adt1::C0(_, _, _, h1057, _, _, _, _, _, _, _) => {
                        h1057
                    }
                };
                let a1060 = 1u64;
                nat_rem(a1058, a1059, a1060)
            };
            let a1062 = 0u64;
            nat_eq(a1061, a1062)
        } else {
            false
        } {
            let a1068 = {
                let a1065 = match v0 {
                    Adt1::C0(_, _, _, _, h1063, _, _, _, _, _, _) => {
                        h1063
                    }
                };
                let a1066 = match v1 {
                    Adt1::C0(_, _, _, _, h1064, _, _, _, _, _, _) => {
                        h1064
                    }
                };
                let a1067 = 1u64;
                nat_rem(a1065, a1066, a1067)
            };
            let a1069 = 0u64;
            nat_eq(a1068, a1069)
        } else {
            false
        }
    }
}

pub fn f48(v0: u64, v1: u64, v2: u64) -> bool {
    let a1070 = v0;
    let a1071 = 0u64;
    if if if nat_eq(a1070, a1071) {
        let a1072 = v1;
        let a1073 = 0u64;
        nat_eq(a1072, a1073)
    } else {
        false
    } {
        let a1074 = v2;
        let a1075 = 0u64;
        nat_eq(a1074, a1075)
    } else {
        false
    } {
        true
    } else {
        let a1076 = v0;
        let a1077 = 1u64;
        if if nat_eq(a1076, a1077) {
            true
        } else {
            let a1078 = v0;
            let a1079 = 30u64;
            nat_eq(a1078, a1079)
        } {
            let a1080 = v1;
            let a1081 = v0;
            if if nat_eq(a1080, a1081) {
                let a1082 = v2;
                let a1083 = v0;
                nat_eq(a1082, a1083)
            } else {
                false
            } {
                true
            } else {
                let a1084 = v1;
                let a1085 = 0u64;
                if nat_eq(a1084, a1085) {
                    let a1086 = v2;
                    let a1087 = v0;
                    if nat_eq(a1086, a1087) {
                        true
                    } else {
                        let a1088 = v2;
                        let a1089 = 0u64;
                        nat_eq(a1088, a1089)
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

pub fn f49(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1090 = v0;
    let a1091 = 0u64;
    if if if nat_eq(a1090, a1091) {
        true
    } else {
        let a1092 = v1;
        let a1093 = 0u64;
        nat_eq(a1092, a1093)
    } {
        true
    } else {
        let a1094 = v2;
        let a1095 = 0u64;
        nat_eq(a1094, a1095)
    } {
        true
    } else {
        let a1096 = v3;
        let a1097 = 0u64;
        nat_eq(a1096, a1097)
    }
}

pub fn f50(v0: u64, v1: u64, v2: u64) -> bool {
    let a1098 = v0;
    let a1099 = 1u64;
    if nat_le(a1098, a1099) {
        true
    } else {
        let a1103 = {
            let a1100 = v1;
            let a1101 = v2;
            let a1102 = 1u64;
            nat_rem(a1100, a1101, a1102)
        };
        let a1104 = 0u64;
        nat_eq(a1103, a1104)
    }
}

pub fn f51(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f50(v0, v3, v6) {
        f50(v1, v4, v6)
    } else {
        false
    } {
        f50(v2, v5, v6)
    } else {
        false
    }
}

pub fn f52(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f15(v0.clone(), v1.clone()) {
        let a1107 = match v0.clone() {
            Adt1::C0(h1105, _, _, _, _, _, _, _, _, _, _) => {
                h1105
            }
        };
        let a1108 = match v1.clone() {
            Adt1::C0(h1106, _, _, _, _, _, _, _, _, _, _) => {
                h1106
            }
        };
        nat_eq(a1107, a1108)
    } else {
        false
    } {
        let a1111 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1109) => {
                h1109
            }
        };
        let a1112 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1110) => {
                h1110
            }
        };
        nat_eq(a1111, a1112)
    } else {
        false
    } {
        let a1115 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1113, _, _, _, _, _) => {
                h1113
            }
        };
        let a1116 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1114, _, _, _, _, _) => {
                h1114
            }
        };
        nat_eq(a1115, a1116)
    } else {
        false
    } {
        let a1119 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1117, _, _, _, _) => {
                h1117
            }
        };
        let a1120 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1118, _, _, _, _) => {
                h1118
            }
        };
        nat_eq(a1119, a1120)
    } else {
        false
    } {
        let a1123 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1121, _, _, _) => {
                h1121
            }
        };
        let a1124 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1122, _, _, _) => {
                h1122
            }
        };
        nat_eq(a1123, a1124)
    } else {
        false
    } {
        let a1127 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1125, _, _) => {
                h1125
            }
        };
        let a1128 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1126, _, _) => {
                h1126
            }
        };
        nat_eq(a1127, a1128)
    } else {
        false
    }
}

pub fn f53(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f57(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f57(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f57(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f57(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f57(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f57(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f54(v0: u64) -> Adt4 {
    let a1129 = v0;
    let a1130 = 0u64;
    if nat_eq(a1129, a1130) {
        Adt4::C1(Adt5::C0(0u64, 1u64, 4u64))
    } else {
        let a1131 = v0;
        let a1132 = 1u64;
        if nat_eq(a1131, a1132) {
            Adt4::C1(Adt5::C0(1u64, 1u64, 2u64))
        } else {
            let a1133 = v0;
            let a1134 = 2u64;
            if nat_eq(a1133, a1134) {
                Adt4::C1(Adt5::C0(2u64, 32u64, 18u64))
            } else {
                let a1135 = v0;
                let a1136 = 3u64;
                if nat_eq(a1135, a1136) {
                    Adt4::C1(Adt5::C0(3u64, 32u64, 20u64))
                } else {
                    let a1137 = v0;
                    let a1138 = 6u64;
                    if nat_eq(a1137, a1138) {
                        Adt4::C1(Adt5::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1139 = v0;
                        let a1140 = 7u64;
                        if nat_eq(a1139, a1140) {
                            Adt4::C1(Adt5::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1141 = v0;
                            let a1142 = 8u64;
                            if nat_eq(a1141, a1142) {
                                Adt4::C1(Adt5::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1143 = v0;
                                let a1144 = 9u64;
                                if nat_eq(a1143, a1144) {
                                    Adt4::C1(Adt5::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1145 = v0;
                                    let a1146 = 10u64;
                                    if nat_eq(a1145, a1146) {
                                        Adt4::C1(Adt5::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1147 = v0;
                                        let a1148 = 11u64;
                                        if nat_eq(a1147, a1148) {
                                            Adt4::C1(Adt5::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1149 = v0;
                                            let a1150 = 12u64;
                                            if nat_eq(a1149, a1150) {
                                                Adt4::C1(Adt5::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1151 = v0;
                                                let a1152 = 13u64;
                                                if nat_eq(a1151, a1152) {
                                                    Adt4::C1(Adt5::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1153 = v0;
                                                    let a1154 = 14u64;
                                                    if nat_eq(a1153, a1154) {
                                                        Adt4::C1(Adt5::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1155 = v0;
                                                        let a1156 = 15u64;
                                                        if nat_eq(a1155, a1156) {
                                                            Adt4::C1(Adt5::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1157 = v0;
                                                            let a1158 = 16u64;
                                                            if nat_eq(a1157, a1158) {
                                                                Adt4::C1(Adt5::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1159 = v0;
                                                                let a1160 = 17u64;
                                                                if nat_eq(a1159, a1160) {
                                                                    Adt4::C1(Adt5::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1161 = v0;
                                                                    let a1162 = 18u64;
                                                                    if nat_eq(a1161, a1162) {
                                                                        Adt4::C1(Adt5::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1163 = v0;
                                                                        let a1164 = 19u64;
                                                                        if nat_eq(a1163, a1164) {
                                                                            Adt4::C1(Adt5::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1165 = v0;
                                                                            let a1166 = 20u64;
                                                                            if nat_eq(a1165, a1166) {
                                                                                Adt4::C1(Adt5::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1167 = v0;
                                                                                let a1168 = 21u64;
                                                                                if nat_eq(a1167, a1168) {
                                                                                    Adt4::C1(Adt5::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1169 = v0;
                                                                                    let a1170 = 22u64;
                                                                                    if nat_eq(a1169, a1170) {
                                                                                        Adt4::C1(Adt5::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1171 = v0;
                                                                                        let a1172 = 23u64;
                                                                                        if nat_eq(a1171, a1172) {
                                                                                            Adt4::C1(Adt5::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1173 = v0;
                                                                                            let a1174 = 24u64;
                                                                                            if nat_eq(a1173, a1174) {
                                                                                                Adt4::C1(Adt5::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1175 = v0;
                                                                                                let a1176 = 25u64;
                                                                                                if nat_eq(a1175, a1176) {
                                                                                                    Adt4::C1(Adt5::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1177 = v0;
                                                                                                    let a1178 = 26u64;
                                                                                                    if nat_eq(a1177, a1178) {
                                                                                                        Adt4::C1(Adt5::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1179 = v0;
                                                                                                        let a1180 = 27u64;
                                                                                                        if nat_eq(a1179, a1180) {
                                                                                                            Adt4::C1(Adt5::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1181 = v0;
                                                                                                            let a1182 = 28u64;
                                                                                                            if nat_eq(a1181, a1182) {
                                                                                                                Adt4::C1(Adt5::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1183 = v0;
                                                                                                                let a1184 = 29u64;
                                                                                                                if nat_eq(a1183, a1184) {
                                                                                                                    Adt4::C1(Adt5::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1185 = v0;
                                                                                                                    let a1186 = 30u64;
                                                                                                                    if nat_eq(a1185, a1186) {
                                                                                                                        Adt4::C1(Adt5::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1187 = v0;
                                                                                                                        let a1188 = 34u64;
                                                                                                                        if nat_eq(a1187, a1188) {
                                                                                                                            Adt4::C1(Adt5::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1189 = v0;
                                                                                                                            let a1190 = 35u64;
                                                                                                                            if nat_eq(a1189, a1190) {
                                                                                                                                Adt4::C1(Adt5::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1191 = v0;
                                                                                                                                let a1192 = 39u64;
                                                                                                                                if nat_eq(a1191, a1192) {
                                                                                                                                    Adt4::C1(Adt5::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1193 = v0;
                                                                                                                                    let a1194 = 40u64;
                                                                                                                                    if nat_eq(a1193, a1194) {
                                                                                                                                        Adt4::C1(Adt5::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1195 = v0;
                                                                                                                                        let a1196 = 41u64;
                                                                                                                                        if nat_eq(a1195, a1196) {
                                                                                                                                            Adt4::C1(Adt5::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1197 = v0;
                                                                                                                                            let a1198 = 42u64;
                                                                                                                                            if nat_eq(a1197, a1198) {
                                                                                                                                                Adt4::C1(Adt5::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1199 = v0;
                                                                                                                                                let a1200 = 43u64;
                                                                                                                                                if nat_eq(a1199, a1200) {
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

pub fn f55(v0: u64, v1: u64) -> u64 {
    let a1201 = v0;
    let a1202 = 1u64;
    if if nat_eq(a1201, a1202) {
        let a1203 = v1;
        let a1204 = 43u64;
        nat_eq(a1203, a1204)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f56(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt3 {
    let a1205 = f55(v0, v1);
    let a1206 = 2u64;
    if nat_eq(a1205, a1206) {
        let a1210 = {
            let a1207 = v2;
            let a1208 = 2u64;
            let a1209 = 0u64;
            nat_rem(a1207, a1208, a1209)
        };
        let a1211 = 0u64;
        if nat_eq(a1210, a1211) {
            Adt3::C3(0u64, 0u64, v0, 0u64, 0u64, v3, 2u64, 0u64, 1u64, 0u64, f34(v0), 8u64)
        } else {
            Adt3::C3(0u64, 0u64, 0u64, v1, 1u64, 0u64, 2u64, 0u64, 3u64, v4, 8u64, f27(v1))
        }
    } else {
        Adt3::C3(0u64, 0u64, v0, v1, 0u64, v3, 2u64, 0u64, 3u64, v4, f34(v0), f27(v1))
    }
}

pub fn f57(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a1212 = v1;
    let a1213 = 1u64;
    if if if nat_le(a1212, a1213) {
        true
    } else {
        let a1214 = v0;
        let a1215 = v2;
        nat_le(a1214, a1215)
    } {
        let a1216 = v3;
        let a1217 = 1u64;
        if nat_le(a1216, a1217) {
            true
        } else {
            let a1224 = {
                let a1222 = v0;
                let a1223 = {
                    let a1220 = {
                        let a1218 = v1;
                        let a1219 = 1u64;
                        nat_sub(a1218, a1219)
                    };
                    let a1221 = v2;
                    nat_mul(a1220, a1221)?
                };
                nat_add(a1222, a1223)?
            };
            let a1225 = v4;
            nat_le(a1224, a1225)
        }
    } else {
        false
    } {
        let a1226 = v5;
        let a1227 = 1u64;
        if nat_le(a1226, a1227) {
            Ok(true)
        } else {
            let a1240 = {
                let a1238 = {
                    let a1232 = v0;
                    let a1233 = {
                        let a1230 = {
                            let a1228 = v1;
                            let a1229 = 1u64;
                            nat_sub(a1228, a1229)
                        };
                        let a1231 = v2;
                        nat_mul(a1230, a1231)?
                    };
                    nat_add(a1232, a1233)?
                };
                let a1239 = {
                    let a1236 = {
                        let a1234 = v3;
                        let a1235 = 1u64;
                        nat_sub(a1234, a1235)
                    };
                    let a1237 = v4;
                    nat_mul(a1236, a1237)?
                };
                nat_add(a1238, a1239)?
            };
            let a1241 = v6;
            Ok(nat_le(a1240, a1241))
        }
    } else {
        Ok(false)
    }
}
