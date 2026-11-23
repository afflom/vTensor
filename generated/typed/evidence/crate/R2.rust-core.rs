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
    C3(u64, u64, u64, bool, Adt1, Option<Adt1>, Adt1),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64, u64, u64, u64, Adt1, Adt1, Adt1),
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
        Adt0::C1(v23, v24, v25) => {
            let a16 = match v24.clone() {
                Adt2::C0(_, _, h15, _, _, _, _) => {
                    h15
                }
            };
            let a17 = 66u64;
            if if nat_eq(a16, a17) {
                true
            } else {
                let a19 = match v24.clone() {
                    Adt2::C0(_, _, h18, _, _, _, _) => {
                        h18
                    }
                };
                let a20 = 67u64;
                nat_eq(a19, a20)
            } {
                f7(v23, v24)
            } else {
                let a22 = match v24.clone() {
                    Adt2::C0(_, _, h21, _, _, _, _) => {
                        h21
                    }
                };
                let a23 = 41u64;
                if nat_eq(a22, a23) {
                    f8(v23, v24, v25)
                } else {
                    let a25 = 98u64;
                    let a26 = match v24.clone() {
                        Adt2::C0(_, _, h24, _, _, _, _) => {
                            h24
                        }
                    };
                    if nat_le(a25, a26) {
                        f9(v23, v24)
                    } else {
                        f10(v23, v24, v25)
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
        Adt0::C3(v14, v15, v16, v17, v18, v19, v20) => {
            let a7 = v14;
            let a8 = 66u64;
            if if nat_eq(a7, a8) {
                true
            } else {
                let a9 = v14;
                let a10 = 67u64;
                nat_eq(a9, a10)
            } {
                if v17 {
                    Ok(Adt3::C1)
                } else {
                    let m11 = v19;
                    match m11 {
                        None => {
                            f4(v14, v15, v18, v20)
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
                        !v17
                    } else {
                        false
                    } {
                        let m14 = v19;
                        match m14 {
                            None => {
                                f6(v15, false, v16, v18.clone(), v18, v20)
                            }
                            Some(v22) => {
                                f6(v15, true, v16, v18, v22, v20)
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
    let a27 = v0;
    let a28 = 2u64;
    if nat_eq(a27, a28) {
        Some(0u64)
    } else {
        let a29 = v0;
        let a30 = 5u64;
        if nat_eq(a29, a30) {
            Some(1u64)
        } else {
            let a31 = v0;
            let a32 = 6u64;
            if nat_eq(a31, a32) {
                Some(2u64)
            } else {
                let a33 = v0;
                let a34 = 7u64;
                if nat_eq(a33, a34) {
                    Some(3u64)
                } else {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f2(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if if f11(v1.clone(), v3.clone()) {
        f12(v1.clone(), v2.clone())
    } else {
        false
    } {
        f13(match v1.clone() {
            Adt1::C0(h35, _, _, _, _, _, _, _, _, _, _) => {
                h35
            }
        }, match v2.clone() {
            Adt1::C0(h36, _, _, _, _, _, _, _, _, _, _) => {
                h36
            }
        }, match v3.clone() {
            Adt1::C0(h37, _, _, _, _, _, _, _, _, _, _) => {
                h37
            }
        })
    } else {
        false
    } {
        if f14(v1.clone()) {
            true
        } else {
            let a40 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, h38, _, _, _, _, _) => {
                    h38
                }
            };
            let a41 = f15(match v1.clone() {
                Adt1::C0(h39, _, _, _, _, _, _, _, _, _, _) => {
                    h39
                }
            });
            if if if if if if if nat_eq(a40, a41) {
                let a44 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h42, _, _, _, _, _) => {
                        h42
                    }
                };
                let a45 = f15(match v3.clone() {
                    Adt1::C0(h43, _, _, _, _, _, _, _, _, _, _) => {
                        h43
                    }
                });
                nat_eq(a44, a45)
            } else {
                false
            } {
                f16(v1.clone())
            } else {
                false
            } {
                f16(v2.clone())
            } else {
                false
            } {
                f16(v3.clone())
            } else {
                false
            } {
                f17(v1.clone(), v2.clone())?
            } else {
                false
            } {
                f18(v1.clone(), v3.clone())?
            } else {
                false
            } {
                f18(v2.clone(), v3.clone())?
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v4: u64 = if if f14(v1.clone()) {
            true
        } else {
            f19(v3.clone())?
        } {
            0u64
        } else {
            let a48 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h46, _) => {
                    h46
                }
            };
            let a49 = f15(match v3.clone() {
                Adt1::C0(h47, _, _, _, _, _, _, _, _, _, _) => {
                    h47
                }
            });
            let a50 = 0u64;
            nat_quot(a48, a49, a50)
        };
        Ok(Adt3::C2(Adt2::C0(f20(f21(v1.clone())?, v4, f22(match v1.clone() {
            Adt1::C0(h51, _, _, _, _, _, _, _, _, _, _) => {
                h51
            }
        }, match v2.clone() {
            Adt1::C0(h52, _, _, _, _, _, _, _, _, _, _) => {
                h52
            }
        }, match v3.clone() {
            Adt1::C0(h53, _, _, _, _, _, _, _, _, _, _) => {
                h53
            }
        })?)?, {
            let a54 = 3u64;
            let a55 = v4;
            nat_add(a54, a55)?
        }, v0, 0u64, v1, v2, v3)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f3(v0: Adt1, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if if if if if if if if if if if if if f23(v2, v3.clone()) {
        let a57 = match v0.clone() {
            Adt1::C0(h56, _, _, _, _, _, _, _, _, _, _) => {
                h56
            }
        };
        let a58 = 0u64;
        if nat_eq(a57, a58) {
            true
        } else {
            let a60 = match v0.clone() {
                Adt1::C0(h59, _, _, _, _, _, _, _, _, _, _) => {
                    h59
                }
            };
            let a61 = 1u64;
            nat_eq(a60, a61)
        }
    } else {
        false
    } {
        let a63 = 0u64;
        let a64 = f15(match v3.clone() {
            Adt1::C0(h62, _, _, _, _, _, _, _, _, _, _) => {
                h62
            }
        });
        nat_lt(a63, a64)
    } else {
        false
    } {
        let a66 = 0u64;
        let a67 = f24(match v1.clone() {
            Adt1::C0(h65, _, _, _, _, _, _, _, _, _, _) => {
                h65
            }
        });
        nat_lt(a66, a67)
    } else {
        false
    } {
        let a70 = match v0.clone() {
            Adt1::C0(_, h68, _, _, _, _, _, _, _, _, _) => {
                h68
            }
        };
        let a71 = match v3.clone() {
            Adt1::C0(_, h69, _, _, _, _, _, _, _, _, _) => {
                h69
            }
        };
        nat_eq(a70, a71)
    } else {
        false
    } {
        let a74 = match v0.clone() {
            Adt1::C0(_, _, _, h72, _, _, _, _, _, _, _) => {
                h72
            }
        };
        let a75 = match v3.clone() {
            Adt1::C0(_, _, _, h73, _, _, _, _, _, _, _) => {
                h73
            }
        };
        nat_eq(a74, a75)
    } else {
        false
    } {
        let a78 = match v0.clone() {
            Adt1::C0(_, _, _, _, h76, _, _, _, _, _, _) => {
                h76
            }
        };
        let a79 = match v3.clone() {
            Adt1::C0(_, _, _, _, h77, _, _, _, _, _, _) => {
                h77
            }
        };
        nat_eq(a78, a79)
    } else {
        false
    } {
        let a82 = match v0.clone() {
            Adt1::C0(_, _, h80, _, _, _, _, _, _, _, _) => {
                h80
            }
        };
        let a83 = match v1.clone() {
            Adt1::C0(_, h81, _, _, _, _, _, _, _, _, _) => {
                h81
            }
        };
        nat_eq(a82, a83)
    } else {
        false
    } {
        let a85 = match v1.clone() {
            Adt1::C0(_, _, _, _, h84, _, _, _, _, _, _) => {
                h84
            }
        };
        let a86 = 1u64;
        nat_eq(a85, a86)
    } else {
        false
    } {
        let a88 = 0u64;
        let a89 = match v1.clone() {
            Adt1::C0(_, _, h87, _, _, _, _, _, _, _, _) => {
                h87
            }
        };
        nat_lt(a88, a89)
    } else {
        false
    } {
        let a91 = 0u64;
        let a92 = match v1.clone() {
            Adt1::C0(_, _, _, h90, _, _, _, _, _, _, _) => {
                h90
            }
        };
        nat_lt(a91, a92)
    } else {
        false
    } {
        let a98 = {
            let a95 = match v0.clone() {
                Adt1::C0(_, _, _, h93, _, _, _, _, _, _, _) => {
                    h93
                }
            };
            let a96 = match v1.clone() {
                Adt1::C0(_, _, h94, _, _, _, _, _, _, _, _) => {
                    h94
                }
            };
            let a97 = 0u64;
            nat_rem(a95, a96, a97)
        };
        let a99 = 0u64;
        nat_eq(a98, a99)
    } else {
        false
    } {
        let a105 = {
            let a102 = match v0.clone() {
                Adt1::C0(_, _, _, _, h100, _, _, _, _, _, _) => {
                    h100
                }
            };
            let a103 = match v1.clone() {
                Adt1::C0(_, _, _, h101, _, _, _, _, _, _, _) => {
                    h101
                }
            };
            let a104 = 0u64;
            nat_rem(a102, a103, a104)
        };
        let a106 = 0u64;
        nat_eq(a105, a106)
    } else {
        false
    } {
        f25(v0.clone())
    } else {
        false
    } {
        f25(v3.clone())
    } else {
        false
    } {
        let v4: u64 = f21(v1.clone())?;
        let v5: u64 = {
            let a112 = match v0.clone() {
                Adt1::C0(_, _, h107, _, _, _, _, _, _, _, _) => {
                    h107
                }
            };
            let a113 = {
                let a110 = match v0.clone() {
                    Adt1::C0(_, _, _, h108, _, _, _, _, _, _, _) => {
                        h108
                    }
                };
                let a111 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h109, _, _, _, _, _, _) => {
                        h109
                    }
                };
                nat_mul(a110, a111)?
            };
            nat_mul(a112, a113)?
        };
        let v6: u64 = if f14(v0.clone()) {
            0u64
        } else {
            let a121 = {
                let a114 = 2u64;
                let a115 = v4;
                nat_mul(a114, a115)?
            };
            let a122 = {
                let a119 = v5;
                let a120 = {
                    let a117 = 1u64;
                    let a118 = match v0.clone() {
                        Adt1::C0(_, h116, _, _, _, _, _, _, _, _, _) => {
                            h116
                        }
                    };
                    nat_add(a117, a118)?
                };
                nat_mul(a119, a120)?
            };
            nat_add(a121, a122)?
        };
        Ok(Adt3::C2(Adt2::C0(v6, if f14(v0.clone()) {
            3u64
        } else {
            let a123 = 3u64;
            let a124 = v4;
            nat_add(a123, a124)?
        }, 41u64, 0u64, v0, v1, v3)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f4(v0: u64, v1: u64, v2: Adt1, v3: Adt1) -> R<Adt3> {
    let a126 = match v2.clone() {
        Adt1::C0(h125, _, _, _, _, _, _, _, _, _, _) => {
            h125
        }
    };
    let a127 = 0u64;
    if if if if if if if if if if if nat_eq(a126, a127) {
        let a129 = match v3.clone() {
            Adt1::C0(h128, _, _, _, _, _, _, _, _, _, _) => {
                h128
            }
        };
        let a130 = 26u64;
        nat_eq(a129, a130)
    } else {
        false
    } {
        let a132 = match v2.clone() {
            Adt1::C0(_, h131, _, _, _, _, _, _, _, _, _) => {
                h131
            }
        };
        let a133 = 2147483647u64;
        nat_le(a132, a133)
    } else {
        false
    } {
        let a136 = match v3.clone() {
            Adt1::C0(_, h134, _, _, _, _, _, _, _, _, _) => {
                h134
            }
        };
        let a137 = match v2.clone() {
            Adt1::C0(_, h135, _, _, _, _, _, _, _, _, _) => {
                h135
            }
        };
        nat_le(a136, a137)
    } else {
        false
    } {
        let a140 = match v2.clone() {
            Adt1::C0(_, _, h138, _, _, _, _, _, _, _, _) => {
                h138
            }
        };
        let a141 = match v3.clone() {
            Adt1::C0(_, _, h139, _, _, _, _, _, _, _, _) => {
                h139
            }
        };
        nat_eq(a140, a141)
    } else {
        false
    } {
        let a144 = match v2.clone() {
            Adt1::C0(_, _, _, h142, _, _, _, _, _, _, _) => {
                h142
            }
        };
        let a145 = match v3.clone() {
            Adt1::C0(_, _, _, h143, _, _, _, _, _, _, _) => {
                h143
            }
        };
        nat_eq(a144, a145)
    } else {
        false
    } {
        let a148 = match v2.clone() {
            Adt1::C0(_, _, _, _, h146, _, _, _, _, _, _) => {
                h146
            }
        };
        let a149 = match v3.clone() {
            Adt1::C0(_, _, _, _, h147, _, _, _, _, _, _) => {
                h147
            }
        };
        nat_eq(a148, a149)
    } else {
        false
    } {
        let a150 = v0;
        let a151 = 67u64;
        if nat_eq(a150, a151) {
            true
        } else {
            let a152 = v1;
            let a153 = 1u64;
            if nat_le(a152, a153) {
                let a156 = match v3.clone() {
                    Adt1::C0(_, h154, _, _, _, _, _, _, _, _, _) => {
                        h154
                    }
                };
                let a157 = match v2.clone() {
                    Adt1::C0(_, h155, _, _, _, _, _, _, _, _, _) => {
                        h155
                    }
                };
                nat_eq(a156, a157)
            } else {
                false
            }
        }
    } else {
        false
    } {
        f26(v2.clone())?
    } else {
        false
    } {
        f26(v3.clone())?
    } else {
        false
    } {
        f27(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a163 = match v3.clone() {
                Adt1::C0(_, _, h158, _, _, _, _, _, _, _, _) => {
                    h158
                }
            };
            let a164 = {
                let a161 = match v3.clone() {
                    Adt1::C0(_, _, _, h159, _, _, _, _, _, _, _) => {
                        h159
                    }
                };
                let a162 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h160, _, _, _, _, _, _) => {
                        h160
                    }
                };
                nat_mul(a161, a162)?
            };
            nat_mul(a163, a164)?
        }, 3u64, v0, {
            let a165 = v0;
            let a166 = 67u64;
            if nat_eq(a165, a166) {
                1u64
            } else {
                v1
            }
        }, v2.clone(), v2, v3)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f5(v0: u64) -> bool {
    let a167 = v0;
    let a168 = 2u64;
    if nat_le(a167, a168) {
        true
    } else {
        let a169 = v0;
        let a170 = 5u64;
        nat_eq(a169, a170)
    }
}

pub fn f6(v0: u64, v1: bool, v2: u64, v3: Adt1, v4: Adt1, v5: Adt1) -> R<Adt3> {
    let a171 = v2;
    let a172 = 1u64;
    if if if if if if if if if if if if if if if nat_le(a171, a172) {
        let a174 = 0u64;
        let a175 = f15(match v3.clone() {
            Adt1::C0(h173, _, _, _, _, _, _, _, _, _, _) => {
                h173
            }
        });
        nat_lt(a174, a175)
    } else {
        false
    } {
        let a178 = match v3.clone() {
            Adt1::C0(h176, _, _, _, _, _, _, _, _, _, _) => {
                h176
            }
        };
        let a179 = match v4.clone() {
            Adt1::C0(h177, _, _, _, _, _, _, _, _, _, _) => {
                h177
            }
        };
        nat_eq(a178, a179)
    } else {
        false
    } {
        let a182 = match v3.clone() {
            Adt1::C0(h180, _, _, _, _, _, _, _, _, _, _) => {
                h180
            }
        };
        let a183 = match v5.clone() {
            Adt1::C0(h181, _, _, _, _, _, _, _, _, _, _) => {
                h181
            }
        };
        nat_eq(a182, a183)
    } else {
        false
    } {
        f11(v3.clone(), v4.clone())
    } else {
        false
    } {
        f28(v3.clone())?
    } else {
        false
    } {
        f28(v4.clone())?
    } else {
        false
    } {
        f28(v5.clone())?
    } else {
        false
    } {
        let a186 = match v3.clone() {
            Adt1::C0(_, _, h184, _, _, _, _, _, _, _, _) => {
                h184
            }
        };
        let a187 = match v5.clone() {
            Adt1::C0(_, _, h185, _, _, _, _, _, _, _, _) => {
                h185
            }
        };
        nat_eq(a186, a187)
    } else {
        false
    } {
        let a190 = match v3.clone() {
            Adt1::C0(_, _, _, h188, _, _, _, _, _, _, _) => {
                h188
            }
        };
        let a191 = match v5.clone() {
            Adt1::C0(_, _, _, h189, _, _, _, _, _, _, _) => {
                h189
            }
        };
        nat_eq(a190, a191)
    } else {
        false
    } {
        let a194 = match v3.clone() {
            Adt1::C0(_, _, _, _, h192, _, _, _, _, _, _) => {
                h192
            }
        };
        let a195 = match v5.clone() {
            Adt1::C0(_, _, _, _, h193, _, _, _, _, _, _) => {
                h193
            }
        };
        nat_eq(a194, a195)
    } else {
        false
    } {
        let a202 = match v5.clone() {
            Adt1::C0(_, h196, _, _, _, _, _, _, _, _, _) => {
                h196
            }
        };
        let a203 = if v1 {
            match v3.clone() {
                Adt1::C0(_, h197, _, _, _, _, _, _, _, _, _) => {
                    h197
                }
            }
        } else {
            let a199 = match v3.clone() {
                Adt1::C0(_, h198, _, _, _, _, _, _, _, _, _) => {
                    h198
                }
            };
            let a200 = 2u64;
            let a201 = 0u64;
            nat_quot(a199, a200, a201)
        };
        nat_eq(a202, a203)
    } else {
        false
    } {
        let a205 = match v5.clone() {
            Adt1::C0(_, h204, _, _, _, _, _, _, _, _, _) => {
                h204
            }
        };
        let a206 = 2147483647u64;
        nat_le(a205, a206)
    } else {
        false
    } {
        f18(v3.clone(), v5.clone())?
    } else {
        false
    } {
        f18(v4.clone(), v5.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a207 = 4u64;
            let a208 = f21(v5.clone())?;
            nat_mul(a207, a208)?
        }, 3u64, if v1 {
            98u64
        } else {
            let a209 = 99u64;
            let a210 = v2;
            nat_add(a209, a210)?
        }, v0, v3, v4, v5)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f7(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, h211, _, _) => {
            h211
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h212) => {
            h212
        }
    };
    let a214 = v0;
    let a215 = match v1.clone() {
        Adt2::C0(h213, _, _, _, _, _, _) => {
            h213
        }
    };
    if nat_lt(a214, a215) {
        Ok(Adt3::C5({
            let a217 = v0;
            let a218 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, _, h216, _, _, _, _) => {
                    h216
                }
            };
            nat_mul(a217, a218)?
        }, {
            let a220 = v0;
            let a221 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h219, _, _, _, _) => {
                    h219
                }
            };
            nat_mul(a220, a221)?
        }, match v2 {
            Adt1::C0(_, h222, _, _, _, _, _, _, _, _, _) => {
                h222
            }
        }, match v3 {
            Adt1::C0(_, h223, _, _, _, _, _, _, _, _, _) => {
                h223
            }
        }, {
            let a225 = match v1 {
                Adt2::C0(_, _, _, h224, _, _, _) => {
                    h224
                }
            };
            let a226 = 1u64;
            nat_eq(a225, a226)
        }))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f8(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let a228 = v0;
    let a229 = match v1.clone() {
        Adt2::C0(h227, _, _, _, _, _, _) => {
            h227
        }
    };
    if nat_lt(a228, a229) {
        let v3: u64 = {
            let a231 = 2u64;
            let a232 = f21(match v1.clone() {
                Adt2::C0(_, _, _, _, _, h230, _) => {
                    h230
                }
            })?;
            nat_mul(a231, a232)?
        };
        let a233 = v0;
        let a234 = v3;
        if nat_lt(a233, a234) {
            f29(v0, match v1.clone() {
                Adt2::C0(_, _, _, _, _, h235, _) => {
                    h235
                }
            }, {
                let m238 = match v1 {
                    Adt2::C0(_, _, _, _, _, _, h237) => {
                        h237
                    }
                };
                match m238 {
                    Adt1::C0(_, _, h236, _, _, _, _, _, _, _, _) => {
                        h236
                    }
                }
            }, v2)
        } else {
            f30({
                let a239 = v0;
                let a240 = v3;
                nat_sub(a239, a240)
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h241, _, _) => {
                    h241
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, h242, _) => {
                    h242
                }
            }, match v1 {
                Adt2::C0(_, _, _, _, _, _, h243) => {
                    h243
                }
            }, v2)
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f9(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, h244, _, _) => {
            h244
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, h245, _) => {
            h245
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h246) => {
            h246
        }
    };
    let v5: u64 = match v1.clone() {
        Adt2::C0(_, _, h247, _, _, _, _) => {
            h247
        }
    };
    let v6: u64 = {
        let a248 = v0;
        let a249 = 4u64;
        let a250 = 0u64;
        nat_rem(a248, a249, a250)
    };
    let v7: u64 = {
        let a251 = v0;
        let a252 = 4u64;
        let a253 = 0u64;
        nat_quot(a251, a252, a253)
    };
    let v8: u64 = {
        let a255 = v7;
        let a256 = match v4.clone() {
            Adt1::C0(_, h254, _, _, _, _, _, _, _, _, _) => {
                h254
            }
        };
        let a257 = 0u64;
        nat_quot(a255, a256, a257)
    };
    let v9: u64 = f15(match v4.clone() {
        Adt1::C0(h258, _, _, _, _, _, _, _, _, _, _) => {
            h258
        }
    });
    let v10: u64 = {
        let a263 = {
            let a260 = v7;
            let a261 = match v4.clone() {
                Adt1::C0(_, h259, _, _, _, _, _, _, _, _, _) => {
                    h259
                }
            };
            let a262 = 0u64;
            nat_rem(a260, a261, a262)
        };
        let a264 = v9;
        nat_mul(a263, a264)?
    };
    let v11: u64 = {
        let a266 = match v4.clone() {
            Adt1::C0(_, h265, _, _, _, _, _, _, _, _, _) => {
                h265
            }
        };
        let a267 = v9;
        nat_mul(a266, a267)?
    };
    let a269 = v0;
    let a270 = match v1.clone() {
        Adt2::C0(h268, _, _, _, _, _, _) => {
            h268
        }
    };
    if nat_lt(a269, a270) {
        let a271 = v6;
        let a272 = 0u64;
        if nat_eq(a271, a272) {
            Ok(Adt3::C3(0u64, 0u64, match v2.clone() {
                Adt1::C0(h273, _, _, _, _, _, _, _, _, _, _) => {
                    h273
                }
            }, 0u64, 0u64, {
                let a281 = {
                    let a277 = {
                        let a275 = v8;
                        let a276 = match v2 {
                            Adt1::C0(_, _, _, _, _, _, h274, _, _, _, _) => {
                                h274
                            }
                        };
                        nat_mul(a275, a276)?
                    };
                    let a278 = v10;
                    nat_add(a277, a278)?
                };
                let a282 = {
                    let a279 = v5;
                    let a280 = 100u64;
                    if nat_eq(a279, a280) {
                        v11
                    } else {
                        0u64
                    }
                };
                nat_add(a281, a282)?
            }, 2u64, 0u64, 0u64, 0u64, v9, 4u64))
        } else {
            let a283 = v6;
            let a284 = 1u64;
            if nat_eq(a283, a284) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h285, _, _, _, _, _, _, _, _, _, _) => {
                        h285
                    }
                }, 0u64, {
                    let a286 = v5;
                    let a287 = 98u64;
                    if nat_eq(a286, a287) {
                        4u64
                    } else {
                        0u64
                    }
                }, {
                    let a295 = {
                        let a291 = {
                            let a289 = v8;
                            let a290 = match v3 {
                                Adt1::C0(_, _, _, _, _, _, h288, _, _, _, _) => {
                                    h288
                                }
                            };
                            nat_mul(a289, a290)?
                        };
                        let a292 = v10;
                        nat_add(a291, a292)?
                    };
                    let a296 = {
                        let a293 = v5;
                        let a294 = 99u64;
                        if nat_eq(a293, a294) {
                            v11
                        } else {
                            0u64
                        }
                    };
                    nat_add(a295, a296)?
                }, 2u64, 0u64, 0u64, 1u64, v9, 4u64))
            } else {
                let a297 = v6;
                let a298 = 2u64;
                if nat_eq(a297, a298) {
                    Ok(Adt3::C4(match v1 {
                        Adt2::C0(_, _, _, h299, _, _, _) => {
                            h299
                        }
                    }, match v4 {
                        Adt1::C0(h300, _, _, _, _, _, _, _, _, _, _) => {
                            h300
                        }
                    }, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt3::C3(0u64, 0u64, 0u64, match v4.clone() {
                        Adt1::C0(h301, _, _, _, _, _, _, _, _, _, _) => {
                            h301
                        }
                    }, 1u64, 0u64, 2u64, 0u64, 3u64, {
                        let a305 = {
                            let a303 = v8;
                            let a304 = match v4 {
                                Adt1::C0(_, _, _, _, _, _, h302, _, _, _, _) => {
                                    h302
                                }
                            };
                            nat_mul(a303, a304)?
                        };
                        let a306 = v10;
                        nat_add(a305, a306)?
                    }, 4u64, v9))
                }
            }
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f10(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, h307, _, _) => {
            h307
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, h308, _) => {
            h308
        }
    };
    let v5: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h309) => {
            h309
        }
    };
    let v6: u64 = {
        let a311 = match v1.clone() {
            Adt2::C0(_, h310, _, _, _, _, _) => {
                h310
            }
        };
        let a312 = 3u64;
        if nat_eq(a311, a312) {
            0u64
        } else {
            let a313 = f21(v3.clone())?;
            let a314 = 2u64;
            nat_mul(a313, a314)?
        }
    };
    let a316 = match v1.clone() {
        Adt2::C0(h315, _, _, _, _, _, _) => {
            h315
        }
    };
    let a317 = v0;
    if nat_le(a316, a317) {
        Ok(Adt3::C1)
    } else {
        let a318 = v0;
        let a319 = v6;
        if nat_lt(a318, a319) {
            let v7: u64 = {
                let a320 = v0;
                let a321 = 2u64;
                let a322 = 0u64;
                nat_quot(a320, a321, a322)
            };
            let v8: u64 = {
                let a327 = 3u64;
                let a328 = {
                    let a324 = f31(v7, v5.clone())?;
                    let a325 = f15(match v5 {
                        Adt1::C0(h323, _, _, _, _, _, _, _, _, _, _) => {
                            h323
                        }
                    });
                    let a326 = 0u64;
                    nat_quot(a324, a325, a326)
                };
                nat_add(a327, a328)?
            };
            let a332 = {
                let a329 = v0;
                let a330 = 2u64;
                let a331 = 0u64;
                nat_rem(a329, a330, a331)
            };
            let a333 = 0u64;
            if nat_eq(a332, a333) {
                Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, v8, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
            } else {
                let a334 = v2;
                let a335 = 0u64;
                if nat_eq(a334, a335) {
                    Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 2u64, 1u64, 2u64, 0u64, 0u64, v8, 8u64, 8u64))
                } else {
                    Ok(Adt3::C1)
                }
            }
        } else {
            let v9: u64 = {
                let a336 = v0;
                let a337 = v6;
                nat_sub(a336, a337)
            };
            let v10: u64 = f22(match v3.clone() {
                Adt1::C0(h338, _, _, _, _, _, _, _, _, _, _) => {
                    h338
                }
            }, match v4.clone() {
                Adt1::C0(h339, _, _, _, _, _, _, _, _, _, _) => {
                    h339
                }
            }, match v5.clone() {
                Adt1::C0(h340, _, _, _, _, _, _, _, _, _, _) => {
                    h340
                }
            })?;
            let v11: u64 = {
                let a341 = v9;
                let a342 = v10;
                let a343 = 0u64;
                nat_quot(a341, a342, a343)
            };
            let v12: u64 = {
                let a344 = v9;
                let a345 = v10;
                let a346 = 0u64;
                nat_rem(a344, a345, a346)
            };
            let v13: u64 = {
                let a348 = match v3.clone() {
                    Adt1::C0(h347, _, _, _, _, _, _, _, _, _, _) => {
                        h347
                    }
                };
                let a349 = 0u64;
                if nat_eq(a348, a349) {
                    0u64
                } else {
                    1u64
                }
            };
            let v14: u64 = {
                let a351 = match v4.clone() {
                    Adt1::C0(h350, _, _, _, _, _, _, _, _, _, _) => {
                        h350
                    }
                };
                let a352 = 0u64;
                if nat_eq(a351, a352) {
                    0u64
                } else {
                    1u64
                }
            };
            let a353 = v12;
            let a354 = v13;
            if nat_lt(a353, a354) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h355, _, _, _, _, _, _, _, _, _, _) => {
                        h355
                    }
                }, 0u64, 0u64, f31(v11, v3.clone())?, 2u64, 0u64, 0u64, 0u64, f15(match v3 {
                    Adt1::C0(h356, _, _, _, _, _, _, _, _, _, _) => {
                        h356
                    }
                }), 4u64))
            } else {
                let a359 = v12;
                let a360 = {
                    let a357 = v13;
                    let a358 = v14;
                    nat_add(a357, a358)?
                };
                if nat_lt(a359, a360) {
                    Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                        Adt1::C0(h361, _, _, _, _, _, _, _, _, _, _) => {
                            h361
                        }
                    }, 0u64, 4u64, f32(v11, v3, v4.clone())?, 2u64, 0u64, 0u64, 1u64, f15(match v4 {
                        Adt1::C0(h362, _, _, _, _, _, _, _, _, _, _) => {
                            h362
                        }
                    }), 4u64))
                } else {
                    let a365 = v12;
                    let a366 = {
                        let a363 = v13;
                        let a364 = v14;
                        nat_add(a363, a364)?
                    };
                    if nat_eq(a365, a366) {
                        Ok(Adt3::C3(1u64, match v1 {
                            Adt2::C0(_, _, h367, _, _, _, _) => {
                                h367
                            }
                        }, 0u64, 0u64, {
                            let a368 = v13;
                            let a369 = 0u64;
                            if nat_eq(a368, a369) {
                                0u64
                            } else {
                                1u64
                            }
                        }, {
                            let a370 = v13;
                            let a371 = 0u64;
                            if nat_eq(a370, a371) {
                                f31(v11, v3.clone())?
                            } else {
                                0u64
                            }
                        }, {
                            let a372 = v14;
                            let a373 = 0u64;
                            if nat_eq(a372, a373) {
                                4u64
                            } else {
                                1u64
                            }
                        }, {
                            let a374 = v14;
                            let a375 = 0u64;
                            if nat_eq(a374, a375) {
                                f32(v11, v3, v4)?
                            } else {
                                1u64
                            }
                        }, {
                            let a377 = match v5.clone() {
                                Adt1::C0(h376, _, _, _, _, _, _, _, _, _, _) => {
                                    h376
                                }
                            };
                            let a378 = 0u64;
                            if nat_eq(a377, a378) {
                                3u64
                            } else {
                                0u64
                            }
                        }, {
                            let a380 = match v5.clone() {
                                Adt1::C0(h379, _, _, _, _, _, _, _, _, _, _) => {
                                    h379
                                }
                            };
                            let a381 = 0u64;
                            if nat_eq(a380, a381) {
                                f31(v11, v5)?
                            } else {
                                0u64
                            }
                        }, 4u64, 4u64))
                    } else {
                        Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                            Adt1::C0(h382, _, _, _, _, _, _, _, _, _, _) => {
                                h382
                            }
                        }, 1u64, 0u64, 2u64, 0u64, 3u64, f31(v11, v5.clone())?, 4u64, f15(match v5 {
                            Adt1::C0(h383, _, _, _, _, _, _, _, _, _, _) => {
                                h383
                            }
                        })))
                    }
                }
            }
        }
    }
}

pub fn f11(v0: Adt1, v1: Adt1) -> bool {
    let a386 = match v0.clone() {
        Adt1::C0(_, h384, _, _, _, _, _, _, _, _, _) => {
            h384
        }
    };
    let a387 = match v1.clone() {
        Adt1::C0(_, h385, _, _, _, _, _, _, _, _, _) => {
            h385
        }
    };
    if if if nat_eq(a386, a387) {
        let a390 = match v0.clone() {
            Adt1::C0(_, _, h388, _, _, _, _, _, _, _, _) => {
                h388
            }
        };
        let a391 = match v1.clone() {
            Adt1::C0(_, _, h389, _, _, _, _, _, _, _, _) => {
                h389
            }
        };
        nat_eq(a390, a391)
    } else {
        false
    } {
        let a394 = match v0.clone() {
            Adt1::C0(_, _, _, h392, _, _, _, _, _, _, _) => {
                h392
            }
        };
        let a395 = match v1.clone() {
            Adt1::C0(_, _, _, h393, _, _, _, _, _, _, _) => {
                h393
            }
        };
        nat_eq(a394, a395)
    } else {
        false
    } {
        let a398 = match v0 {
            Adt1::C0(_, _, _, _, h396, _, _, _, _, _, _) => {
                h396
            }
        };
        let a399 = match v1 {
            Adt1::C0(_, _, _, _, h397, _, _, _, _, _, _) => {
                h397
            }
        };
        nat_eq(a398, a399)
    } else {
        false
    }
}

pub fn f12(v0: Adt1, v1: Adt1) -> bool {
    if f14(v1.clone()) {
        f14(v0)
    } else {
        let a405 = {
            let a402 = match v0.clone() {
                Adt1::C0(_, h400, _, _, _, _, _, _, _, _, _) => {
                    h400
                }
            };
            let a403 = match v1.clone() {
                Adt1::C0(_, h401, _, _, _, _, _, _, _, _, _) => {
                    h401
                }
            };
            let a404 = 1u64;
            nat_rem(a402, a403, a404)
        };
        let a406 = 0u64;
        if if if nat_eq(a405, a406) {
            let a412 = {
                let a409 = match v0.clone() {
                    Adt1::C0(_, _, h407, _, _, _, _, _, _, _, _) => {
                        h407
                    }
                };
                let a410 = match v1.clone() {
                    Adt1::C0(_, _, h408, _, _, _, _, _, _, _, _) => {
                        h408
                    }
                };
                let a411 = 1u64;
                nat_rem(a409, a410, a411)
            };
            let a413 = 0u64;
            nat_eq(a412, a413)
        } else {
            false
        } {
            let a419 = {
                let a416 = match v0.clone() {
                    Adt1::C0(_, _, _, h414, _, _, _, _, _, _, _) => {
                        h414
                    }
                };
                let a417 = match v1.clone() {
                    Adt1::C0(_, _, _, h415, _, _, _, _, _, _, _) => {
                        h415
                    }
                };
                let a418 = 1u64;
                nat_rem(a416, a417, a418)
            };
            let a420 = 0u64;
            nat_eq(a419, a420)
        } else {
            false
        } {
            let a426 = {
                let a423 = match v0 {
                    Adt1::C0(_, _, _, _, h421, _, _, _, _, _, _) => {
                        h421
                    }
                };
                let a424 = match v1 {
                    Adt1::C0(_, _, _, _, h422, _, _, _, _, _, _) => {
                        h422
                    }
                };
                let a425 = 1u64;
                nat_rem(a423, a424, a425)
            };
            let a427 = 0u64;
            nat_eq(a426, a427)
        } else {
            false
        }
    }
}

pub fn f13(v0: u64, v1: u64, v2: u64) -> bool {
    let a428 = v0;
    let a429 = 0u64;
    if if if nat_eq(a428, a429) {
        let a430 = v1;
        let a431 = 0u64;
        nat_eq(a430, a431)
    } else {
        false
    } {
        let a432 = v2;
        let a433 = 0u64;
        nat_eq(a432, a433)
    } else {
        false
    } {
        true
    } else {
        let a434 = v0;
        let a435 = 1u64;
        if if nat_eq(a434, a435) {
            true
        } else {
            let a436 = v0;
            let a437 = 30u64;
            nat_eq(a436, a437)
        } {
            let a438 = v1;
            let a439 = v0;
            if if nat_eq(a438, a439) {
                let a440 = v2;
                let a441 = v0;
                nat_eq(a440, a441)
            } else {
                false
            } {
                true
            } else {
                let a442 = v1;
                let a443 = 0u64;
                if nat_eq(a442, a443) {
                    let a444 = v2;
                    let a445 = v0;
                    if nat_eq(a444, a445) {
                        true
                    } else {
                        let a446 = v2;
                        let a447 = 0u64;
                        nat_eq(a446, a447)
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

pub fn f14(v0: Adt1) -> bool {
    f33(match v0.clone() {
        Adt1::C0(_, h448, _, _, _, _, _, _, _, _, _) => {
            h448
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h449, _, _, _, _, _, _, _, _) => {
            h449
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h450, _, _, _, _, _, _, _) => {
            h450
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, h451, _, _, _, _, _, _) => {
            h451
        }
    })
}

pub fn f15(v0: u64) -> u64 {
    let a452 = v0;
    let a453 = 0u64;
    if nat_eq(a452, a453) {
        4u64
    } else {
        let a454 = v0;
        let a455 = 1u64;
        if if nat_eq(a454, a455) {
            true
        } else {
            let a456 = v0;
            let a457 = 30u64;
            nat_eq(a456, a457)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f16(v0: Adt1) -> bool {
    let v1: u64 = f15(match v0.clone() {
        Adt1::C0(h458, _, _, _, _, _, _, _, _, _, _) => {
            h458
        }
    });
    let a463 = {
        let a460 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h459) => {
                h459
            }
        };
        let a461 = v1;
        let a462 = 1u64;
        nat_rem(a460, a461, a462)
    };
    let a464 = 0u64;
    if if nat_eq(a463, a464) {
        f34(match v0.clone() {
            Adt1::C0(_, h465, _, _, _, _, _, _, _, _, _) => {
                h465
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h466, _, _, _, _, _) => {
                h466
            }
        }, v1)
    } else {
        false
    } {
        f35(match v0.clone() {
            Adt1::C0(_, _, h467, _, _, _, _, _, _, _, _) => {
                h467
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h468, _, _, _, _, _, _, _) => {
                h468
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h469, _, _, _, _, _, _) => {
                h469
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h470, _, _, _, _) => {
                h470
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h471, _, _, _) => {
                h471
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h472, _, _) => {
                h472
            }
        }, v1)
    } else {
        false
    }
}

pub fn f17(v0: Adt1, v1: Adt1) -> R<bool> {
    let a480 = {
        let a478 = match v0.clone() {
            Adt1::C0(_, _, h473, _, _, _, _, _, _, _, _) => {
                h473
            }
        };
        let a479 = {
            let a476 = match v0.clone() {
                Adt1::C0(_, _, _, h474, _, _, _, _, _, _, _) => {
                    h474
                }
            };
            let a477 = match v0.clone() {
                Adt1::C0(_, _, _, _, h475, _, _, _, _, _, _) => {
                    h475
                }
            };
            nat_mul(a476, a477)?
        };
        nat_mul(a478, a479)?
    };
    let a481 = 9223372036854775807u64;
    if nat_le(a480, a481) {
        if f25(v1.clone()) {
            let a483 = match v1 {
                Adt1::C0(_, h482, _, _, _, _, _, _, _, _, _) => {
                    h482
                }
            };
            let a484 = 2147483647u64;
            Ok(nat_le(a483, a484))
        } else {
            let a486 = match v0 {
                Adt1::C0(_, h485, _, _, _, _, _, _, _, _, _) => {
                    h485
                }
            };
            let a487 = 2147483647u64;
            Ok(nat_le(a486, a487))
        }
    } else {
        Ok(false)
    }
}

pub fn f18(v0: Adt1, v1: Adt1) -> R<bool> {
    if f27(v0.clone(), v1.clone())? {
        Ok(true)
    } else {
        Ok(f36(v0, v1))
    }
}

pub fn f19(v0: Adt1) -> R<bool> {
    f37({
        let a490 = match v0.clone() {
            Adt1::C0(_, h488, _, _, _, _, _, _, _, _, _) => {
                h488
            }
        };
        let a491 = f15(match v0.clone() {
            Adt1::C0(h489, _, _, _, _, _, _, _, _, _, _) => {
                h489
            }
        });
        nat_mul(a490, a491)?
    }, match v0.clone() {
        Adt1::C0(_, _, h492, _, _, _, _, _, _, _, _) => {
            h492
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h493, _, _, _, _, _, _, _) => {
            h493
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h494, _, _, _, _, _, _) => {
            h494
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h495, _, _, _, _) => {
            h495
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h496, _, _, _) => {
            h496
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h497, _, _) => {
            h497
        }
    })
}

pub fn f20(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a504 = {
        let a498 = v0;
        let a499 = v2;
        nat_mul(a498, a499)?
    };
    let a505 = {
        let a500 = v1;
        let a501 = 0u64;
        if nat_eq(a500, a501) {
            0u64
        } else {
            let a502 = v0;
            let a503 = 2u64;
            nat_mul(a502, a503)?
        }
    };
    nat_add(a504, a505)
}

pub fn f21(v0: Adt1) -> R<u64> {
    let a514 = {
        let a508 = match v0.clone() {
            Adt1::C0(_, h506, _, _, _, _, _, _, _, _, _) => {
                h506
            }
        };
        let a509 = match v0.clone() {
            Adt1::C0(_, _, h507, _, _, _, _, _, _, _, _) => {
                h507
            }
        };
        nat_mul(a508, a509)?
    };
    let a515 = {
        let a512 = match v0.clone() {
            Adt1::C0(_, _, _, h510, _, _, _, _, _, _, _) => {
                h510
            }
        };
        let a513 = match v0 {
            Adt1::C0(_, _, _, _, h511, _, _, _, _, _, _) => {
                h511
            }
        };
        nat_mul(a512, a513)?
    };
    nat_mul(a514, a515)
}

pub fn f22(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a526 = {
        let a522 = {
            let a518 = 1u64;
            let a519 = {
                let a516 = v0;
                let a517 = 0u64;
                if nat_eq(a516, a517) {
                    0u64
                } else {
                    1u64
                }
            };
            nat_add(a518, a519)?
        };
        let a523 = {
            let a520 = v1;
            let a521 = 0u64;
            if nat_eq(a520, a521) {
                0u64
            } else {
                1u64
            }
        };
        nat_add(a522, a523)?
    };
    let a527 = {
        let a524 = v2;
        let a525 = 0u64;
        if nat_eq(a524, a525) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a526, a527)
}

pub fn f23(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f11(v0.clone(), v1.clone()) {
        let a530 = match v0.clone() {
            Adt1::C0(h528, _, _, _, _, _, _, _, _, _, _) => {
                h528
            }
        };
        let a531 = match v1.clone() {
            Adt1::C0(h529, _, _, _, _, _, _, _, _, _, _) => {
                h529
            }
        };
        nat_eq(a530, a531)
    } else {
        false
    } {
        let a534 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h532, _, _, _, _, _) => {
                h532
            }
        };
        let a535 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h533, _, _, _, _, _) => {
                h533
            }
        };
        nat_eq(a534, a535)
    } else {
        false
    } {
        let a538 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h536, _, _, _, _) => {
                h536
            }
        };
        let a539 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h537, _, _, _, _) => {
                h537
            }
        };
        nat_eq(a538, a539)
    } else {
        false
    } {
        let a542 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h540, _, _, _) => {
                h540
            }
        };
        let a543 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h541, _, _, _) => {
                h541
            }
        };
        nat_eq(a542, a543)
    } else {
        false
    } {
        let a546 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h544, _, _) => {
                h544
            }
        };
        let a547 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h545, _, _) => {
                h545
            }
        };
        nat_eq(a546, a547)
    } else {
        false
    } {
        let a549 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h548) => {
                h548
            }
        };
        let a550 = 0u64;
        if if nat_eq(a549, a550) {
            true
        } else {
            let a552 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h551) => {
                    h551
                }
            };
            let a553 = 0u64;
            nat_eq(a552, a553)
        } {
            true
        } else {
            let a556 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h554) => {
                    h554
                }
            };
            let a557 = match v1 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h555) => {
                    h555
                }
            };
            nat_eq(a556, a557)
        }
    } else {
        false
    }
}

pub fn f24(v0: u64) -> u64 {
    let a558 = v0;
    let a559 = 26u64;
    if nat_eq(a558, a559) {
        4u64
    } else {
        let a560 = v0;
        let a561 = 27u64;
        if nat_eq(a560, a561) {
            8u64
        } else {
            0u64
        }
    }
}

pub fn f25(v0: Adt1) -> bool {
    let a563 = match v0.clone() {
        Adt1::C0(_, h562, _, _, _, _, _, _, _, _, _) => {
            h562
        }
    };
    let a564 = 1u64;
    if nat_eq(a563, a564) {
        true
    } else {
        let a567 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h565, _, _, _, _, _) => {
                h565
            }
        };
        let a568 = f15(match v0 {
            Adt1::C0(h566, _, _, _, _, _, _, _, _, _, _) => {
                h566
            }
        });
        nat_eq(a567, a568)
    }
}

pub fn f26(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a570 = match v0.clone() {
            Adt1::C0(_, h569, _, _, _, _, _, _, _, _, _) => {
                h569
            }
        };
        let a571 = 4u64;
        nat_mul(a570, a571)?
    };
    let v2: u64 = {
        let a574 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h572, _, _, _, _) => {
                h572
            }
        };
        let a575 = match v0.clone() {
            Adt1::C0(_, _, h573, _, _, _, _, _, _, _, _) => {
                h573
            }
        };
        nat_mul(a574, a575)?
    };
    let v3: u64 = {
        let a577 = v2;
        let a578 = match v0.clone() {
            Adt1::C0(_, _, _, h576, _, _, _, _, _, _, _) => {
                h576
            }
        };
        nat_mul(a577, a578)?
    };
    let a580 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h579, _, _, _, _, _) => {
            h579
        }
    };
    let a581 = 4u64;
    if if if if nat_eq(a580, a581) {
        let a586 = {
            let a583 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h582) => {
                    h582
                }
            };
            let a584 = 4u64;
            let a585 = 1u64;
            nat_rem(a583, a584, a585)
        };
        let a587 = 0u64;
        nat_eq(a586, a587)
    } else {
        false
    } {
        let a595 = {
            let a593 = match v0.clone() {
                Adt1::C0(_, _, h588, _, _, _, _, _, _, _, _) => {
                    h588
                }
            };
            let a594 = {
                let a591 = match v0.clone() {
                    Adt1::C0(_, _, _, h589, _, _, _, _, _, _, _) => {
                        h589
                    }
                };
                let a592 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h590, _, _, _, _, _, _) => {
                        h590
                    }
                };
                nat_mul(a591, a592)?
            };
            nat_mul(a593, a594)?
        };
        let a596 = 1u64;
        if nat_le(a595, a596) {
            true
        } else {
            let a598 = v1;
            let a599 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h597, _, _, _, _) => {
                    h597
                }
            };
            if nat_le(a598, a599) {
                let a604 = {
                    let a601 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, h600, _, _, _, _) => {
                            h600
                        }
                    };
                    let a602 = 4u64;
                    let a603 = 1u64;
                    nat_rem(a601, a602, a603)
                };
                let a605 = 0u64;
                nat_eq(a604, a605)
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a607 = match v0.clone() {
            Adt1::C0(_, _, _, h606, _, _, _, _, _, _, _) => {
                h606
            }
        };
        let a608 = 1u64;
        if nat_le(a607, a608) {
            true
        } else {
            let a610 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h609, _, _, _) => {
                    h609
                }
            };
            let a611 = v2;
            nat_eq(a610, a611)
        }
    } else {
        false
    } {
        let a613 = match v0.clone() {
            Adt1::C0(_, _, _, _, h612, _, _, _, _, _, _) => {
                h612
            }
        };
        let a614 = 1u64;
        if nat_le(a613, a614) {
            Ok(true)
        } else {
            let a616 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, h615, _, _) => {
                    h615
                }
            };
            let a617 = v3;
            Ok(nat_eq(a616, a617))
        }
    } else {
        Ok(false)
    }
}

pub fn f27(v0: Adt1, v1: Adt1) -> R<bool> {
    let a619 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h618, _) => {
            h618
        }
    };
    let a620 = 0u64;
    if nat_eq(a619, a620) {
        Ok(true)
    } else {
        let a622 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h621, _) => {
                h621
            }
        };
        let a623 = 0u64;
        if nat_eq(a622, a623) {
            Ok(true)
        } else {
            let a625 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h624) => {
                    h624
                }
            };
            let a626 = 0u64;
            if nat_eq(a625, a626) {
                Ok(true)
            } else {
                let a628 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h627) => {
                        h627
                    }
                };
                let a629 = 0u64;
                if nat_eq(a628, a629) {
                    Ok(true)
                } else {
                    let a635 = {
                        let a632 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h630) => {
                                h630
                            }
                        };
                        let a633 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h631, _) => {
                                h631
                            }
                        };
                        nat_add(a632, a633)?
                    };
                    let a636 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h634) => {
                            h634
                        }
                    };
                    if nat_le(a635, a636) {
                        Ok(true)
                    } else {
                        let a642 = {
                            let a639 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h637) => {
                                    h637
                                }
                            };
                            let a640 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h638, _) => {
                                    h638
                                }
                            };
                            nat_add(a639, a640)?
                        };
                        let a643 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h641) => {
                                h641
                            }
                        };
                        Ok(nat_le(a642, a643))
                    }
                }
            }
        }
    }
}

pub fn f28(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a646 = match v0.clone() {
            Adt1::C0(_, _, h644, _, _, _, _, _, _, _, _) => {
                h644
            }
        };
        let a647 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h645, _, _, _, _) => {
                h645
            }
        };
        nat_mul(a646, a647)?
    };
    if if f25(v0.clone()) {
        let a649 = match v0.clone() {
            Adt1::C0(_, _, _, h648, _, _, _, _, _, _, _) => {
                h648
            }
        };
        let a650 = 1u64;
        if nat_eq(a649, a650) {
            true
        } else {
            let a652 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h651, _, _, _) => {
                    h651
                }
            };
            let a653 = v1;
            nat_eq(a652, a653)
        }
    } else {
        false
    } {
        let a655 = match v0.clone() {
            Adt1::C0(_, _, _, _, h654, _, _, _, _, _, _) => {
                h654
            }
        };
        let a656 = 1u64;
        if nat_eq(a655, a656) {
            Ok(true)
        } else {
            let a661 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h657, _, _) => {
                    h657
                }
            };
            let a662 = {
                let a659 = v1;
                let a660 = match v0 {
                    Adt1::C0(_, _, _, h658, _, _, _, _, _, _, _) => {
                        h658
                    }
                };
                nat_mul(a659, a660)?
            };
            Ok(nat_eq(a661, a662))
        }
    } else {
        Ok(false)
    }
}

pub fn f29(v0: u64, v1: Adt1, v2: u64, v3: u64) -> R<Adt3> {
    let v4: u64 = {
        let a663 = v0;
        let a664 = 2u64;
        let a665 = 0u64;
        nat_quot(a663, a664, a665)
    };
    let v5: u64 = f24(match v1.clone() {
        Adt1::C0(h666, _, _, _, _, _, _, _, _, _, _) => {
            h666
        }
    });
    let a670 = {
        let a667 = v0;
        let a668 = 2u64;
        let a669 = 0u64;
        nat_rem(a667, a668, a669)
    };
    let a671 = 0u64;
    if nat_eq(a670, a671) {
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 4u64, f31(v4, v1)?, 2u64, 0u64, 0u64, 2u64, v5, 8u64))
    } else {
        let a672 = v3;
        let a673 = v2;
        if if nat_lt(a672, a673) {
            let a676 = v3;
            let a677 = {
                let a674 = v5;
                let a675 = 4u64;
                if nat_eq(a674, a675) {
                    2147483648u64
                } else {
                    9223372036854775808u64
                }
            };
            nat_lt(a676, a677)
        } else {
            false
        } {
            Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, {
                let a678 = 3u64;
                let a679 = v4;
                nat_add(a678, a679)?
            }, 8u64, 8u64))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f30(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64) -> R<Adt3> {
    let v5: u64 = {
        let a681 = 1u64;
        let a682 = match v1.clone() {
            Adt1::C0(_, h680, _, _, _, _, _, _, _, _, _) => {
                h680
            }
        };
        nat_add(a681, a682)?
    };
    let v6: u64 = {
        let a683 = v0;
        let a684 = v5;
        let a685 = 0u64;
        nat_rem(a683, a684, a685)
    };
    let v7: u64 = {
        let a686 = v0;
        let a687 = v5;
        let a688 = 0u64;
        nat_quot(a686, a687, a688)
    };
    let v8: u64 = {
        let a690 = v7;
        let a691 = match v1.clone() {
            Adt1::C0(_, _, h689, _, _, _, _, _, _, _, _) => {
                h689
            }
        };
        let a692 = 0u64;
        nat_rem(a690, a691, a692)
    };
    let v9: u64 = {
        let a694 = v7;
        let a695 = match v1.clone() {
            Adt1::C0(_, _, h693, _, _, _, _, _, _, _, _) => {
                h693
            }
        };
        let a696 = 0u64;
        nat_quot(a694, a695, a696)
    };
    let v10: u64 = {
        let a698 = v9;
        let a699 = match v1.clone() {
            Adt1::C0(_, _, _, h697, _, _, _, _, _, _, _) => {
                h697
            }
        };
        let a700 = 0u64;
        nat_rem(a698, a699, a700)
    };
    let v11: u64 = {
        let a702 = v9;
        let a703 = match v1.clone() {
            Adt1::C0(_, _, _, h701, _, _, _, _, _, _, _) => {
                h701
            }
        };
        let a704 = 0u64;
        nat_quot(a702, a703, a704)
    };
    let a705 = v6;
    let a706 = 0u64;
    if nat_eq(a705, a706) {
        let v12: u64 = {
            let a723 = v8;
            let a724 = {
                let a721 = match v2.clone() {
                    Adt1::C0(_, h707, _, _, _, _, _, _, _, _, _) => {
                        h707
                    }
                };
                let a722 = {
                    let a719 = {
                        let a709 = v10;
                        let a710 = match v2.clone() {
                            Adt1::C0(_, _, h708, _, _, _, _, _, _, _, _) => {
                                h708
                            }
                        };
                        let a711 = 0u64;
                        nat_rem(a709, a710, a711)
                    };
                    let a720 = {
                        let a717 = match v2.clone() {
                            Adt1::C0(_, _, h712, _, _, _, _, _, _, _, _) => {
                                h712
                            }
                        };
                        let a718 = {
                            let a714 = v11;
                            let a715 = match v2 {
                                Adt1::C0(_, _, _, h713, _, _, _, _, _, _, _) => {
                                    h713
                                }
                            };
                            let a716 = 0u64;
                            nat_rem(a714, a715, a716)
                        };
                        nat_mul(a717, a718)?
                    };
                    nat_add(a719, a720)?
                };
                nat_mul(a721, a722)?
            };
            nat_add(a723, a724)?
        };
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, {
            let a725 = 3u64;
            let a726 = v12;
            nat_add(a725, a726)?
        }, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
    } else {
        let a728 = v4;
        let a729 = match v3.clone() {
            Adt1::C0(_, _, h727, _, _, _, _, _, _, _, _) => {
                h727
            }
        };
        if nat_lt(a728, a729) {
            let v13: u64 = {
                let a730 = v6;
                let a731 = 1u64;
                nat_sub(a730, a731)
            };
            let v14: u64 = f38(v13, v8, v10, v11, match v1.clone() {
                Adt1::C0(_, _, _, _, _, h732, _, _, _, _, _) => {
                    h732
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, h733, _, _, _, _) => {
                    h733
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h734, _, _, _) => {
                    h734
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h735, _, _) => {
                    h735
                }
            })?;
            let v15: u64 = f38(v13, v4, v10, v11, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h736, _, _, _, _, _) => {
                    h736
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h737, _, _, _, _) => {
                    h737
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h738, _, _, _) => {
                    h738
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h739, _, _) => {
                    h739
                }
            })?;
            Ok(Adt3::C3(0u64, 0u64, match v1.clone() {
                Adt1::C0(h740, _, _, _, _, _, _, _, _, _, _) => {
                    h740
                }
            }, match v3.clone() {
                Adt1::C0(h741, _, _, _, _, _, _, _, _, _, _) => {
                    h741
                }
            }, 0u64, v14, 2u64, 0u64, 3u64, v15, f15(match v1 {
                Adt1::C0(h742, _, _, _, _, _, _, _, _, _, _) => {
                    h742
                }
            }), f15(match v3 {
                Adt1::C0(h743, _, _, _, _, _, _, _, _, _, _) => {
                    h743
                }
            })))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f31(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a745 = v0;
        let a746 = match v1.clone() {
            Adt1::C0(_, h744, _, _, _, _, _, _, _, _, _) => {
                h744
            }
        };
        let a747 = 0u64;
        nat_quot(a745, a746, a747)
    };
    let v3: u64 = {
        let a749 = v2;
        let a750 = match v1.clone() {
            Adt1::C0(_, _, h748, _, _, _, _, _, _, _, _) => {
                h748
            }
        };
        let a751 = 0u64;
        nat_quot(a749, a750, a751)
    };
    f38({
        let a753 = v0;
        let a754 = match v1.clone() {
            Adt1::C0(_, h752, _, _, _, _, _, _, _, _, _) => {
                h752
            }
        };
        let a755 = 0u64;
        nat_rem(a753, a754, a755)
    }, {
        let a757 = v2;
        let a758 = match v1.clone() {
            Adt1::C0(_, _, h756, _, _, _, _, _, _, _, _) => {
                h756
            }
        };
        let a759 = 0u64;
        nat_rem(a757, a758, a759)
    }, {
        let a761 = v3;
        let a762 = match v1.clone() {
            Adt1::C0(_, _, _, h760, _, _, _, _, _, _, _) => {
                h760
            }
        };
        let a763 = 0u64;
        nat_rem(a761, a762, a763)
    }, {
        let a765 = v3;
        let a766 = match v1.clone() {
            Adt1::C0(_, _, _, h764, _, _, _, _, _, _, _) => {
                h764
            }
        };
        let a767 = 0u64;
        nat_quot(a765, a766, a767)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h768, _, _, _, _, _) => {
            h768
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h769, _, _, _, _) => {
            h769
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h770, _, _, _) => {
            h770
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h771, _, _) => {
            h771
        }
    })
}

pub fn f32(v0: u64, v1: Adt1, v2: Adt1) -> R<u64> {
    let v3: u64 = {
        let a773 = v0;
        let a774 = match v1.clone() {
            Adt1::C0(_, h772, _, _, _, _, _, _, _, _, _) => {
                h772
            }
        };
        let a775 = 0u64;
        nat_quot(a773, a774, a775)
    };
    let v4: u64 = {
        let a777 = v3;
        let a778 = match v1.clone() {
            Adt1::C0(_, _, h776, _, _, _, _, _, _, _, _) => {
                h776
            }
        };
        let a779 = 0u64;
        nat_quot(a777, a778, a779)
    };
    f38({
        let a785 = {
            let a781 = v0;
            let a782 = match v1.clone() {
                Adt1::C0(_, h780, _, _, _, _, _, _, _, _, _) => {
                    h780
                }
            };
            let a783 = 0u64;
            nat_rem(a781, a782, a783)
        };
        let a786 = match v2.clone() {
            Adt1::C0(_, h784, _, _, _, _, _, _, _, _, _) => {
                h784
            }
        };
        let a787 = 0u64;
        nat_rem(a785, a786, a787)
    }, {
        let a793 = {
            let a789 = v3;
            let a790 = match v1.clone() {
                Adt1::C0(_, _, h788, _, _, _, _, _, _, _, _) => {
                    h788
                }
            };
            let a791 = 0u64;
            nat_rem(a789, a790, a791)
        };
        let a794 = match v2.clone() {
            Adt1::C0(_, _, h792, _, _, _, _, _, _, _, _) => {
                h792
            }
        };
        let a795 = 0u64;
        nat_rem(a793, a794, a795)
    }, {
        let a801 = {
            let a797 = v4;
            let a798 = match v1.clone() {
                Adt1::C0(_, _, _, h796, _, _, _, _, _, _, _) => {
                    h796
                }
            };
            let a799 = 0u64;
            nat_rem(a797, a798, a799)
        };
        let a802 = match v2.clone() {
            Adt1::C0(_, _, _, h800, _, _, _, _, _, _, _) => {
                h800
            }
        };
        let a803 = 0u64;
        nat_rem(a801, a802, a803)
    }, {
        let a809 = {
            let a805 = v4;
            let a806 = match v1 {
                Adt1::C0(_, _, _, h804, _, _, _, _, _, _, _) => {
                    h804
                }
            };
            let a807 = 0u64;
            nat_quot(a805, a806, a807)
        };
        let a810 = match v2.clone() {
            Adt1::C0(_, _, _, _, h808, _, _, _, _, _, _) => {
                h808
            }
        };
        let a811 = 0u64;
        nat_rem(a809, a810, a811)
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h812, _, _, _, _, _) => {
            h812
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h813, _, _, _, _) => {
            h813
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h814, _, _, _) => {
            h814
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h815, _, _) => {
            h815
        }
    })
}

pub fn f33(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a816 = v0;
    let a817 = 0u64;
    if if if nat_eq(a816, a817) {
        true
    } else {
        let a818 = v1;
        let a819 = 0u64;
        nat_eq(a818, a819)
    } {
        true
    } else {
        let a820 = v2;
        let a821 = 0u64;
        nat_eq(a820, a821)
    } {
        true
    } else {
        let a822 = v3;
        let a823 = 0u64;
        nat_eq(a822, a823)
    }
}

pub fn f34(v0: u64, v1: u64, v2: u64) -> bool {
    let a824 = v0;
    let a825 = 1u64;
    if nat_le(a824, a825) {
        true
    } else {
        let a829 = {
            let a826 = v1;
            let a827 = v2;
            let a828 = 1u64;
            nat_rem(a826, a827, a828)
        };
        let a830 = 0u64;
        nat_eq(a829, a830)
    }
}

pub fn f35(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f34(v0, v3, v6) {
        f34(v1, v4, v6)
    } else {
        false
    } {
        f34(v2, v5, v6)
    } else {
        false
    }
}

pub fn f36(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f11(v0.clone(), v1.clone()) {
        let a833 = match v0.clone() {
            Adt1::C0(h831, _, _, _, _, _, _, _, _, _, _) => {
                h831
            }
        };
        let a834 = match v1.clone() {
            Adt1::C0(h832, _, _, _, _, _, _, _, _, _, _) => {
                h832
            }
        };
        nat_eq(a833, a834)
    } else {
        false
    } {
        let a837 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h835) => {
                h835
            }
        };
        let a838 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h836) => {
                h836
            }
        };
        nat_eq(a837, a838)
    } else {
        false
    } {
        let a841 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h839, _, _, _, _, _) => {
                h839
            }
        };
        let a842 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h840, _, _, _, _, _) => {
                h840
            }
        };
        nat_eq(a841, a842)
    } else {
        false
    } {
        let a845 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h843, _, _, _, _) => {
                h843
            }
        };
        let a846 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h844, _, _, _, _) => {
                h844
            }
        };
        nat_eq(a845, a846)
    } else {
        false
    } {
        let a849 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h847, _, _, _) => {
                h847
            }
        };
        let a850 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h848, _, _, _) => {
                h848
            }
        };
        nat_eq(a849, a850)
    } else {
        false
    } {
        let a853 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h851, _, _) => {
                h851
            }
        };
        let a854 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h852, _, _) => {
                h852
            }
        };
        nat_eq(a853, a854)
    } else {
        false
    }
}

pub fn f37(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f39(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f39(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f39(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f39(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f39(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f39(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f38(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a867 = {
        let a863 = {
            let a859 = {
                let a855 = v0;
                let a856 = v4;
                nat_mul(a855, a856)?
            };
            let a860 = {
                let a857 = v1;
                let a858 = v5;
                nat_mul(a857, a858)?
            };
            nat_add(a859, a860)?
        };
        let a864 = {
            let a861 = v2;
            let a862 = v6;
            nat_mul(a861, a862)?
        };
        nat_add(a863, a864)?
    };
    let a868 = {
        let a865 = v3;
        let a866 = v7;
        nat_mul(a865, a866)?
    };
    nat_add(a867, a868)
}

pub fn f39(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a869 = v1;
    let a870 = 1u64;
    if if if nat_le(a869, a870) {
        true
    } else {
        let a871 = v0;
        let a872 = v2;
        nat_le(a871, a872)
    } {
        let a873 = v3;
        let a874 = 1u64;
        if nat_le(a873, a874) {
            true
        } else {
            let a881 = {
                let a879 = v0;
                let a880 = {
                    let a877 = {
                        let a875 = v1;
                        let a876 = 1u64;
                        nat_sub(a875, a876)
                    };
                    let a878 = v2;
                    nat_mul(a877, a878)?
                };
                nat_add(a879, a880)?
            };
            let a882 = v4;
            nat_le(a881, a882)
        }
    } else {
        false
    } {
        let a883 = v5;
        let a884 = 1u64;
        if nat_le(a883, a884) {
            Ok(true)
        } else {
            let a897 = {
                let a895 = {
                    let a889 = v0;
                    let a890 = {
                        let a887 = {
                            let a885 = v1;
                            let a886 = 1u64;
                            nat_sub(a885, a886)
                        };
                        let a888 = v2;
                        nat_mul(a887, a888)?
                    };
                    nat_add(a889, a890)?
                };
                let a896 = {
                    let a893 = {
                        let a891 = v3;
                        let a892 = 1u64;
                        nat_sub(a891, a892)
                    };
                    let a894 = v4;
                    nat_mul(a893, a894)?
                };
                nat_add(a895, a896)?
            };
            let a898 = v6;
            Ok(nat_le(a897, a898))
        }
    } else {
        Ok(false)
    }
}
