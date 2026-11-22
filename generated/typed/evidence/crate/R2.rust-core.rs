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
    C0(u64, u64, u64, Adt1, Adt1, Adt1),
}

#[derive(Clone)]
pub enum Adt3 {
    C0,
    C1,
    C2(Adt2),
    C3(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
    C4(u64, u64, u64),
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
        Adt0::C1(v22, v23, v24) => {
            let a13 = match v23.clone() {
                Adt2::C0(_, _, h12, _, _, _) => {
                    h12
                }
            };
            let a14 = 41u64;
            if nat_eq(a13, a14) {
                f5(v22, v23, v24)
            } else {
                let a16 = 98u64;
                let a17 = match v23.clone() {
                    Adt2::C0(_, _, h15, _, _, _) => {
                        h15
                    }
                };
                if nat_le(a16, a17) {
                    f6(v22, v23)
                } else {
                    f7(v22, v23, v24)
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
            let a8 = 98u64;
            if nat_eq(a7, a8) {
                let a9 = v15;
                let a10 = 2u64;
                if if nat_eq(a9, a10) {
                    !v17
                } else {
                    false
                } {
                    let m11 = v19;
                    match m11 {
                        None => {
                            f4(false, v16, v18.clone(), v18, v20)
                        }
                        Some(v21) => {
                            f4(true, v16, v18, v21, v20)
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

pub fn f1(v0: u64) -> Option<u64> {
    let a18 = v0;
    let a19 = 2u64;
    if nat_eq(a18, a19) {
        Some(0u64)
    } else {
        let a20 = v0;
        let a21 = 5u64;
        if nat_eq(a20, a21) {
            Some(1u64)
        } else {
            let a22 = v0;
            let a23 = 6u64;
            if nat_eq(a22, a23) {
                Some(2u64)
            } else {
                let a24 = v0;
                let a25 = 7u64;
                if nat_eq(a24, a25) {
                    Some(3u64)
                } else {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f2(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if if f8(v1.clone(), v3.clone()) {
        f9(v1.clone(), v2.clone())
    } else {
        false
    } {
        f10(match v1.clone() {
            Adt1::C0(h26, _, _, _, _, _, _, _, _, _, _) => {
                h26
            }
        }, match v2.clone() {
            Adt1::C0(h27, _, _, _, _, _, _, _, _, _, _) => {
                h27
            }
        }, match v3.clone() {
            Adt1::C0(h28, _, _, _, _, _, _, _, _, _, _) => {
                h28
            }
        })
    } else {
        false
    } {
        if f11(v1.clone()) {
            true
        } else {
            let a31 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, h29, _, _, _, _, _) => {
                    h29
                }
            };
            let a32 = f12(match v1.clone() {
                Adt1::C0(h30, _, _, _, _, _, _, _, _, _, _) => {
                    h30
                }
            });
            if if if if if if if nat_eq(a31, a32) {
                let a35 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h33, _, _, _, _, _) => {
                        h33
                    }
                };
                let a36 = f12(match v3.clone() {
                    Adt1::C0(h34, _, _, _, _, _, _, _, _, _, _) => {
                        h34
                    }
                });
                nat_eq(a35, a36)
            } else {
                false
            } {
                f13(v1.clone())
            } else {
                false
            } {
                f13(v2.clone())
            } else {
                false
            } {
                f13(v3.clone())
            } else {
                false
            } {
                f14(v1.clone(), v2.clone())?
            } else {
                false
            } {
                f15(v1.clone(), v3.clone())?
            } else {
                false
            } {
                f15(v2.clone(), v3.clone())?
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v4: u64 = if if f11(v1.clone()) {
            true
        } else {
            f16(v3.clone())?
        } {
            0u64
        } else {
            let a39 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h37, _) => {
                    h37
                }
            };
            let a40 = f12(match v3.clone() {
                Adt1::C0(h38, _, _, _, _, _, _, _, _, _, _) => {
                    h38
                }
            });
            let a41 = 0u64;
            nat_quot(a39, a40, a41)
        };
        Ok(Adt3::C2(Adt2::C0(f17(f18(v1.clone())?, v4, f19(match v1.clone() {
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
        })?)?, {
            let a45 = 3u64;
            let a46 = v4;
            nat_add(a45, a46)?
        }, v0, v1, v2, v3)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f3(v0: Adt1, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if if if if if if if if if if if if if f20(v2, v3.clone()) {
        let a48 = match v0.clone() {
            Adt1::C0(h47, _, _, _, _, _, _, _, _, _, _) => {
                h47
            }
        };
        let a49 = 0u64;
        if nat_eq(a48, a49) {
            true
        } else {
            let a51 = match v0.clone() {
                Adt1::C0(h50, _, _, _, _, _, _, _, _, _, _) => {
                    h50
                }
            };
            let a52 = 1u64;
            nat_eq(a51, a52)
        }
    } else {
        false
    } {
        let a54 = 0u64;
        let a55 = f12(match v3.clone() {
            Adt1::C0(h53, _, _, _, _, _, _, _, _, _, _) => {
                h53
            }
        });
        nat_lt(a54, a55)
    } else {
        false
    } {
        let a57 = 0u64;
        let a58 = f21(match v1.clone() {
            Adt1::C0(h56, _, _, _, _, _, _, _, _, _, _) => {
                h56
            }
        });
        nat_lt(a57, a58)
    } else {
        false
    } {
        let a61 = match v0.clone() {
            Adt1::C0(_, h59, _, _, _, _, _, _, _, _, _) => {
                h59
            }
        };
        let a62 = match v3.clone() {
            Adt1::C0(_, h60, _, _, _, _, _, _, _, _, _) => {
                h60
            }
        };
        nat_eq(a61, a62)
    } else {
        false
    } {
        let a65 = match v0.clone() {
            Adt1::C0(_, _, _, h63, _, _, _, _, _, _, _) => {
                h63
            }
        };
        let a66 = match v3.clone() {
            Adt1::C0(_, _, _, h64, _, _, _, _, _, _, _) => {
                h64
            }
        };
        nat_eq(a65, a66)
    } else {
        false
    } {
        let a69 = match v0.clone() {
            Adt1::C0(_, _, _, _, h67, _, _, _, _, _, _) => {
                h67
            }
        };
        let a70 = match v3.clone() {
            Adt1::C0(_, _, _, _, h68, _, _, _, _, _, _) => {
                h68
            }
        };
        nat_eq(a69, a70)
    } else {
        false
    } {
        let a73 = match v0.clone() {
            Adt1::C0(_, _, h71, _, _, _, _, _, _, _, _) => {
                h71
            }
        };
        let a74 = match v1.clone() {
            Adt1::C0(_, h72, _, _, _, _, _, _, _, _, _) => {
                h72
            }
        };
        nat_eq(a73, a74)
    } else {
        false
    } {
        let a76 = match v1.clone() {
            Adt1::C0(_, _, _, _, h75, _, _, _, _, _, _) => {
                h75
            }
        };
        let a77 = 1u64;
        nat_eq(a76, a77)
    } else {
        false
    } {
        let a79 = 0u64;
        let a80 = match v1.clone() {
            Adt1::C0(_, _, h78, _, _, _, _, _, _, _, _) => {
                h78
            }
        };
        nat_lt(a79, a80)
    } else {
        false
    } {
        let a82 = 0u64;
        let a83 = match v1.clone() {
            Adt1::C0(_, _, _, h81, _, _, _, _, _, _, _) => {
                h81
            }
        };
        nat_lt(a82, a83)
    } else {
        false
    } {
        let a89 = {
            let a86 = match v0.clone() {
                Adt1::C0(_, _, _, h84, _, _, _, _, _, _, _) => {
                    h84
                }
            };
            let a87 = match v1.clone() {
                Adt1::C0(_, _, h85, _, _, _, _, _, _, _, _) => {
                    h85
                }
            };
            let a88 = 0u64;
            nat_rem(a86, a87, a88)
        };
        let a90 = 0u64;
        nat_eq(a89, a90)
    } else {
        false
    } {
        let a96 = {
            let a93 = match v0.clone() {
                Adt1::C0(_, _, _, _, h91, _, _, _, _, _, _) => {
                    h91
                }
            };
            let a94 = match v1.clone() {
                Adt1::C0(_, _, _, h92, _, _, _, _, _, _, _) => {
                    h92
                }
            };
            let a95 = 0u64;
            nat_rem(a93, a94, a95)
        };
        let a97 = 0u64;
        nat_eq(a96, a97)
    } else {
        false
    } {
        f22(v0.clone())
    } else {
        false
    } {
        f22(v3.clone())
    } else {
        false
    } {
        let v4: u64 = f18(v1.clone())?;
        let v5: u64 = {
            let a103 = match v0.clone() {
                Adt1::C0(_, _, h98, _, _, _, _, _, _, _, _) => {
                    h98
                }
            };
            let a104 = {
                let a101 = match v0.clone() {
                    Adt1::C0(_, _, _, h99, _, _, _, _, _, _, _) => {
                        h99
                    }
                };
                let a102 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h100, _, _, _, _, _, _) => {
                        h100
                    }
                };
                nat_mul(a101, a102)?
            };
            nat_mul(a103, a104)?
        };
        let v6: u64 = if f11(v0.clone()) {
            0u64
        } else {
            let a112 = {
                let a105 = 2u64;
                let a106 = v4;
                nat_mul(a105, a106)?
            };
            let a113 = {
                let a110 = v5;
                let a111 = {
                    let a108 = 1u64;
                    let a109 = match v0.clone() {
                        Adt1::C0(_, h107, _, _, _, _, _, _, _, _, _) => {
                            h107
                        }
                    };
                    nat_add(a108, a109)?
                };
                nat_mul(a110, a111)?
            };
            nat_add(a112, a113)?
        };
        Ok(Adt3::C2(Adt2::C0(v6, if f11(v0.clone()) {
            3u64
        } else {
            let a114 = 3u64;
            let a115 = v4;
            nat_add(a114, a115)?
        }, 41u64, v0, v1, v3)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f4(v0: bool, v1: u64, v2: Adt1, v3: Adt1, v4: Adt1) -> R<Adt3> {
    let a116 = v1;
    let a117 = 1u64;
    if if if if if if if if if if if if if if if nat_le(a116, a117) {
        let a119 = 0u64;
        let a120 = f12(match v2.clone() {
            Adt1::C0(h118, _, _, _, _, _, _, _, _, _, _) => {
                h118
            }
        });
        nat_lt(a119, a120)
    } else {
        false
    } {
        let a123 = match v2.clone() {
            Adt1::C0(h121, _, _, _, _, _, _, _, _, _, _) => {
                h121
            }
        };
        let a124 = match v3.clone() {
            Adt1::C0(h122, _, _, _, _, _, _, _, _, _, _) => {
                h122
            }
        };
        nat_eq(a123, a124)
    } else {
        false
    } {
        let a127 = match v2.clone() {
            Adt1::C0(h125, _, _, _, _, _, _, _, _, _, _) => {
                h125
            }
        };
        let a128 = match v4.clone() {
            Adt1::C0(h126, _, _, _, _, _, _, _, _, _, _) => {
                h126
            }
        };
        nat_eq(a127, a128)
    } else {
        false
    } {
        f8(v2.clone(), v3.clone())
    } else {
        false
    } {
        f23(v2.clone())?
    } else {
        false
    } {
        f23(v3.clone())?
    } else {
        false
    } {
        f23(v4.clone())?
    } else {
        false
    } {
        let a131 = match v2.clone() {
            Adt1::C0(_, _, h129, _, _, _, _, _, _, _, _) => {
                h129
            }
        };
        let a132 = match v4.clone() {
            Adt1::C0(_, _, h130, _, _, _, _, _, _, _, _) => {
                h130
            }
        };
        nat_eq(a131, a132)
    } else {
        false
    } {
        let a135 = match v2.clone() {
            Adt1::C0(_, _, _, h133, _, _, _, _, _, _, _) => {
                h133
            }
        };
        let a136 = match v4.clone() {
            Adt1::C0(_, _, _, h134, _, _, _, _, _, _, _) => {
                h134
            }
        };
        nat_eq(a135, a136)
    } else {
        false
    } {
        let a139 = match v2.clone() {
            Adt1::C0(_, _, _, _, h137, _, _, _, _, _, _) => {
                h137
            }
        };
        let a140 = match v4.clone() {
            Adt1::C0(_, _, _, _, h138, _, _, _, _, _, _) => {
                h138
            }
        };
        nat_eq(a139, a140)
    } else {
        false
    } {
        let a147 = match v4.clone() {
            Adt1::C0(_, h141, _, _, _, _, _, _, _, _, _) => {
                h141
            }
        };
        let a148 = if v0 {
            match v2.clone() {
                Adt1::C0(_, h142, _, _, _, _, _, _, _, _, _) => {
                    h142
                }
            }
        } else {
            let a144 = match v2.clone() {
                Adt1::C0(_, h143, _, _, _, _, _, _, _, _, _) => {
                    h143
                }
            };
            let a145 = 2u64;
            let a146 = 0u64;
            nat_quot(a144, a145, a146)
        };
        nat_eq(a147, a148)
    } else {
        false
    } {
        let a150 = match v4.clone() {
            Adt1::C0(_, h149, _, _, _, _, _, _, _, _, _) => {
                h149
            }
        };
        let a151 = 2147483647u64;
        nat_le(a150, a151)
    } else {
        false
    } {
        f15(v2.clone(), v4.clone())?
    } else {
        false
    } {
        f15(v3.clone(), v4.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a152 = 4u64;
            let a153 = f18(v4.clone())?;
            nat_mul(a152, a153)?
        }, 3u64, if v0 {
            98u64
        } else {
            let a154 = 99u64;
            let a155 = v1;
            nat_add(a154, a155)?
        }, v2, v3, v4)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f5(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let a157 = v0;
    let a158 = match v1.clone() {
        Adt2::C0(h156, _, _, _, _, _) => {
            h156
        }
    };
    if nat_lt(a157, a158) {
        let v3: u64 = {
            let a160 = 2u64;
            let a161 = f18(match v1.clone() {
                Adt2::C0(_, _, _, _, h159, _) => {
                    h159
                }
            })?;
            nat_mul(a160, a161)?
        };
        let a162 = v0;
        let a163 = v3;
        if nat_lt(a162, a163) {
            f24(v0, match v1.clone() {
                Adt2::C0(_, _, _, _, h164, _) => {
                    h164
                }
            }, {
                let m167 = match v1 {
                    Adt2::C0(_, _, _, _, _, h166) => {
                        h166
                    }
                };
                match m167 {
                    Adt1::C0(_, _, h165, _, _, _, _, _, _, _, _) => {
                        h165
                    }
                }
            }, v2)
        } else {
            f25({
                let a168 = v0;
                let a169 = v3;
                nat_sub(a168, a169)
            }, match v1.clone() {
                Adt2::C0(_, _, _, h170, _, _) => {
                    h170
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h171, _) => {
                    h171
                }
            }, match v1 {
                Adt2::C0(_, _, _, _, _, h172) => {
                    h172
                }
            }, v2)
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f6(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, h173, _, _) => {
            h173
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, h174, _) => {
            h174
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, h175) => {
            h175
        }
    };
    let v5: u64 = match v1.clone() {
        Adt2::C0(_, _, h176, _, _, _) => {
            h176
        }
    };
    let v6: u64 = {
        let a177 = v0;
        let a178 = 4u64;
        let a179 = 0u64;
        nat_rem(a177, a178, a179)
    };
    let v7: u64 = {
        let a180 = v0;
        let a181 = 4u64;
        let a182 = 0u64;
        nat_quot(a180, a181, a182)
    };
    let v8: u64 = {
        let a184 = v7;
        let a185 = match v4.clone() {
            Adt1::C0(_, h183, _, _, _, _, _, _, _, _, _) => {
                h183
            }
        };
        let a186 = 0u64;
        nat_quot(a184, a185, a186)
    };
    let v9: u64 = f12(match v4.clone() {
        Adt1::C0(h187, _, _, _, _, _, _, _, _, _, _) => {
            h187
        }
    });
    let v10: u64 = {
        let a192 = {
            let a189 = v7;
            let a190 = match v4.clone() {
                Adt1::C0(_, h188, _, _, _, _, _, _, _, _, _) => {
                    h188
                }
            };
            let a191 = 0u64;
            nat_rem(a189, a190, a191)
        };
        let a193 = v9;
        nat_mul(a192, a193)?
    };
    let v11: u64 = {
        let a195 = match v4.clone() {
            Adt1::C0(_, h194, _, _, _, _, _, _, _, _, _) => {
                h194
            }
        };
        let a196 = v9;
        nat_mul(a195, a196)?
    };
    let a198 = v0;
    let a199 = match v1 {
        Adt2::C0(h197, _, _, _, _, _) => {
            h197
        }
    };
    if nat_lt(a198, a199) {
        let a200 = v6;
        let a201 = 0u64;
        if nat_eq(a200, a201) {
            Ok(Adt3::C3(0u64, 0u64, match v2.clone() {
                Adt1::C0(h202, _, _, _, _, _, _, _, _, _, _) => {
                    h202
                }
            }, 0u64, 0u64, {
                let a210 = {
                    let a206 = {
                        let a204 = v8;
                        let a205 = match v2 {
                            Adt1::C0(_, _, _, _, _, _, h203, _, _, _, _) => {
                                h203
                            }
                        };
                        nat_mul(a204, a205)?
                    };
                    let a207 = v10;
                    nat_add(a206, a207)?
                };
                let a211 = {
                    let a208 = v5;
                    let a209 = 100u64;
                    if nat_eq(a208, a209) {
                        v11
                    } else {
                        0u64
                    }
                };
                nat_add(a210, a211)?
            }, 2u64, 0u64, 0u64, 0u64, v9, 4u64))
        } else {
            let a212 = v6;
            let a213 = 1u64;
            if nat_eq(a212, a213) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h214, _, _, _, _, _, _, _, _, _, _) => {
                        h214
                    }
                }, 0u64, {
                    let a215 = v5;
                    let a216 = 98u64;
                    if nat_eq(a215, a216) {
                        4u64
                    } else {
                        0u64
                    }
                }, {
                    let a224 = {
                        let a220 = {
                            let a218 = v8;
                            let a219 = match v3 {
                                Adt1::C0(_, _, _, _, _, _, h217, _, _, _, _) => {
                                    h217
                                }
                            };
                            nat_mul(a218, a219)?
                        };
                        let a221 = v10;
                        nat_add(a220, a221)?
                    };
                    let a225 = {
                        let a222 = v5;
                        let a223 = 99u64;
                        if nat_eq(a222, a223) {
                            v11
                        } else {
                            0u64
                        }
                    };
                    nat_add(a224, a225)?
                }, 2u64, 0u64, 0u64, 1u64, v9, 4u64))
            } else {
                let a226 = v6;
                let a227 = 2u64;
                if nat_eq(a226, a227) {
                    Ok(Adt3::C4(0u64, 1u64, 0u64))
                } else {
                    Ok(Adt3::C3(0u64, 0u64, 0u64, match v4.clone() {
                        Adt1::C0(h228, _, _, _, _, _, _, _, _, _, _) => {
                            h228
                        }
                    }, 1u64, 0u64, 2u64, 0u64, 3u64, {
                        let a232 = {
                            let a230 = v8;
                            let a231 = match v4 {
                                Adt1::C0(_, _, _, _, _, _, h229, _, _, _, _) => {
                                    h229
                                }
                            };
                            nat_mul(a230, a231)?
                        };
                        let a233 = v10;
                        nat_add(a232, a233)?
                    }, 4u64, v9))
                }
            }
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f7(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, h234, _, _) => {
            h234
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, h235, _) => {
            h235
        }
    };
    let v5: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, h236) => {
            h236
        }
    };
    let v6: u64 = {
        let a238 = match v1.clone() {
            Adt2::C0(_, h237, _, _, _, _) => {
                h237
            }
        };
        let a239 = 3u64;
        if nat_eq(a238, a239) {
            0u64
        } else {
            let a240 = f18(v3.clone())?;
            let a241 = 2u64;
            nat_mul(a240, a241)?
        }
    };
    let a243 = match v1.clone() {
        Adt2::C0(h242, _, _, _, _, _) => {
            h242
        }
    };
    let a244 = v0;
    if nat_le(a243, a244) {
        Ok(Adt3::C1)
    } else {
        let a245 = v0;
        let a246 = v6;
        if nat_lt(a245, a246) {
            let v7: u64 = {
                let a247 = v0;
                let a248 = 2u64;
                let a249 = 0u64;
                nat_quot(a247, a248, a249)
            };
            let v8: u64 = {
                let a254 = 3u64;
                let a255 = {
                    let a251 = f26(v7, v5.clone())?;
                    let a252 = f12(match v5 {
                        Adt1::C0(h250, _, _, _, _, _, _, _, _, _, _) => {
                            h250
                        }
                    });
                    let a253 = 0u64;
                    nat_quot(a251, a252, a253)
                };
                nat_add(a254, a255)?
            };
            let a259 = {
                let a256 = v0;
                let a257 = 2u64;
                let a258 = 0u64;
                nat_rem(a256, a257, a258)
            };
            let a260 = 0u64;
            if nat_eq(a259, a260) {
                Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, v8, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
            } else {
                let a261 = v2;
                let a262 = 0u64;
                if nat_eq(a261, a262) {
                    Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 2u64, 1u64, 2u64, 0u64, 0u64, v8, 8u64, 8u64))
                } else {
                    Ok(Adt3::C1)
                }
            }
        } else {
            let v9: u64 = {
                let a263 = v0;
                let a264 = v6;
                nat_sub(a263, a264)
            };
            let v10: u64 = f19(match v3.clone() {
                Adt1::C0(h265, _, _, _, _, _, _, _, _, _, _) => {
                    h265
                }
            }, match v4.clone() {
                Adt1::C0(h266, _, _, _, _, _, _, _, _, _, _) => {
                    h266
                }
            }, match v5.clone() {
                Adt1::C0(h267, _, _, _, _, _, _, _, _, _, _) => {
                    h267
                }
            })?;
            let v11: u64 = {
                let a268 = v9;
                let a269 = v10;
                let a270 = 0u64;
                nat_quot(a268, a269, a270)
            };
            let v12: u64 = {
                let a271 = v9;
                let a272 = v10;
                let a273 = 0u64;
                nat_rem(a271, a272, a273)
            };
            let v13: u64 = {
                let a275 = match v3.clone() {
                    Adt1::C0(h274, _, _, _, _, _, _, _, _, _, _) => {
                        h274
                    }
                };
                let a276 = 0u64;
                if nat_eq(a275, a276) {
                    0u64
                } else {
                    1u64
                }
            };
            let v14: u64 = {
                let a278 = match v4.clone() {
                    Adt1::C0(h277, _, _, _, _, _, _, _, _, _, _) => {
                        h277
                    }
                };
                let a279 = 0u64;
                if nat_eq(a278, a279) {
                    0u64
                } else {
                    1u64
                }
            };
            let a280 = v12;
            let a281 = v13;
            if nat_lt(a280, a281) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h282, _, _, _, _, _, _, _, _, _, _) => {
                        h282
                    }
                }, 0u64, 0u64, f26(v11, v3.clone())?, 2u64, 0u64, 0u64, 0u64, f12(match v3 {
                    Adt1::C0(h283, _, _, _, _, _, _, _, _, _, _) => {
                        h283
                    }
                }), 4u64))
            } else {
                let a286 = v12;
                let a287 = {
                    let a284 = v13;
                    let a285 = v14;
                    nat_add(a284, a285)?
                };
                if nat_lt(a286, a287) {
                    Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                        Adt1::C0(h288, _, _, _, _, _, _, _, _, _, _) => {
                            h288
                        }
                    }, 0u64, 4u64, f27(v11, v3, v4.clone())?, 2u64, 0u64, 0u64, 1u64, f12(match v4 {
                        Adt1::C0(h289, _, _, _, _, _, _, _, _, _, _) => {
                            h289
                        }
                    }), 4u64))
                } else {
                    let a292 = v12;
                    let a293 = {
                        let a290 = v13;
                        let a291 = v14;
                        nat_add(a290, a291)?
                    };
                    if nat_eq(a292, a293) {
                        Ok(Adt3::C3(1u64, match v1 {
                            Adt2::C0(_, _, h294, _, _, _) => {
                                h294
                            }
                        }, 0u64, 0u64, {
                            let a295 = v13;
                            let a296 = 0u64;
                            if nat_eq(a295, a296) {
                                0u64
                            } else {
                                1u64
                            }
                        }, {
                            let a297 = v13;
                            let a298 = 0u64;
                            if nat_eq(a297, a298) {
                                f26(v11, v3.clone())?
                            } else {
                                0u64
                            }
                        }, {
                            let a299 = v14;
                            let a300 = 0u64;
                            if nat_eq(a299, a300) {
                                4u64
                            } else {
                                1u64
                            }
                        }, {
                            let a301 = v14;
                            let a302 = 0u64;
                            if nat_eq(a301, a302) {
                                f27(v11, v3, v4)?
                            } else {
                                1u64
                            }
                        }, {
                            let a304 = match v5.clone() {
                                Adt1::C0(h303, _, _, _, _, _, _, _, _, _, _) => {
                                    h303
                                }
                            };
                            let a305 = 0u64;
                            if nat_eq(a304, a305) {
                                3u64
                            } else {
                                0u64
                            }
                        }, {
                            let a307 = match v5.clone() {
                                Adt1::C0(h306, _, _, _, _, _, _, _, _, _, _) => {
                                    h306
                                }
                            };
                            let a308 = 0u64;
                            if nat_eq(a307, a308) {
                                f26(v11, v5)?
                            } else {
                                0u64
                            }
                        }, 4u64, 4u64))
                    } else {
                        Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                            Adt1::C0(h309, _, _, _, _, _, _, _, _, _, _) => {
                                h309
                            }
                        }, 1u64, 0u64, 2u64, 0u64, 3u64, f26(v11, v5.clone())?, 4u64, f12(match v5 {
                            Adt1::C0(h310, _, _, _, _, _, _, _, _, _, _) => {
                                h310
                            }
                        })))
                    }
                }
            }
        }
    }
}

pub fn f8(v0: Adt1, v1: Adt1) -> bool {
    let a313 = match v0.clone() {
        Adt1::C0(_, h311, _, _, _, _, _, _, _, _, _) => {
            h311
        }
    };
    let a314 = match v1.clone() {
        Adt1::C0(_, h312, _, _, _, _, _, _, _, _, _) => {
            h312
        }
    };
    if if if nat_eq(a313, a314) {
        let a317 = match v0.clone() {
            Adt1::C0(_, _, h315, _, _, _, _, _, _, _, _) => {
                h315
            }
        };
        let a318 = match v1.clone() {
            Adt1::C0(_, _, h316, _, _, _, _, _, _, _, _) => {
                h316
            }
        };
        nat_eq(a317, a318)
    } else {
        false
    } {
        let a321 = match v0.clone() {
            Adt1::C0(_, _, _, h319, _, _, _, _, _, _, _) => {
                h319
            }
        };
        let a322 = match v1.clone() {
            Adt1::C0(_, _, _, h320, _, _, _, _, _, _, _) => {
                h320
            }
        };
        nat_eq(a321, a322)
    } else {
        false
    } {
        let a325 = match v0 {
            Adt1::C0(_, _, _, _, h323, _, _, _, _, _, _) => {
                h323
            }
        };
        let a326 = match v1 {
            Adt1::C0(_, _, _, _, h324, _, _, _, _, _, _) => {
                h324
            }
        };
        nat_eq(a325, a326)
    } else {
        false
    }
}

pub fn f9(v0: Adt1, v1: Adt1) -> bool {
    if f11(v1.clone()) {
        f11(v0)
    } else {
        let a332 = {
            let a329 = match v0.clone() {
                Adt1::C0(_, h327, _, _, _, _, _, _, _, _, _) => {
                    h327
                }
            };
            let a330 = match v1.clone() {
                Adt1::C0(_, h328, _, _, _, _, _, _, _, _, _) => {
                    h328
                }
            };
            let a331 = 1u64;
            nat_rem(a329, a330, a331)
        };
        let a333 = 0u64;
        if if if nat_eq(a332, a333) {
            let a339 = {
                let a336 = match v0.clone() {
                    Adt1::C0(_, _, h334, _, _, _, _, _, _, _, _) => {
                        h334
                    }
                };
                let a337 = match v1.clone() {
                    Adt1::C0(_, _, h335, _, _, _, _, _, _, _, _) => {
                        h335
                    }
                };
                let a338 = 1u64;
                nat_rem(a336, a337, a338)
            };
            let a340 = 0u64;
            nat_eq(a339, a340)
        } else {
            false
        } {
            let a346 = {
                let a343 = match v0.clone() {
                    Adt1::C0(_, _, _, h341, _, _, _, _, _, _, _) => {
                        h341
                    }
                };
                let a344 = match v1.clone() {
                    Adt1::C0(_, _, _, h342, _, _, _, _, _, _, _) => {
                        h342
                    }
                };
                let a345 = 1u64;
                nat_rem(a343, a344, a345)
            };
            let a347 = 0u64;
            nat_eq(a346, a347)
        } else {
            false
        } {
            let a353 = {
                let a350 = match v0 {
                    Adt1::C0(_, _, _, _, h348, _, _, _, _, _, _) => {
                        h348
                    }
                };
                let a351 = match v1 {
                    Adt1::C0(_, _, _, _, h349, _, _, _, _, _, _) => {
                        h349
                    }
                };
                let a352 = 1u64;
                nat_rem(a350, a351, a352)
            };
            let a354 = 0u64;
            nat_eq(a353, a354)
        } else {
            false
        }
    }
}

pub fn f10(v0: u64, v1: u64, v2: u64) -> bool {
    let a355 = v0;
    let a356 = 0u64;
    if if if nat_eq(a355, a356) {
        let a357 = v1;
        let a358 = 0u64;
        nat_eq(a357, a358)
    } else {
        false
    } {
        let a359 = v2;
        let a360 = 0u64;
        nat_eq(a359, a360)
    } else {
        false
    } {
        true
    } else {
        let a361 = v0;
        let a362 = 1u64;
        if if nat_eq(a361, a362) {
            true
        } else {
            let a363 = v0;
            let a364 = 30u64;
            nat_eq(a363, a364)
        } {
            let a365 = v1;
            let a366 = v0;
            if if nat_eq(a365, a366) {
                let a367 = v2;
                let a368 = v0;
                nat_eq(a367, a368)
            } else {
                false
            } {
                true
            } else {
                let a369 = v1;
                let a370 = 0u64;
                if nat_eq(a369, a370) {
                    let a371 = v2;
                    let a372 = v0;
                    if nat_eq(a371, a372) {
                        true
                    } else {
                        let a373 = v2;
                        let a374 = 0u64;
                        nat_eq(a373, a374)
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

pub fn f11(v0: Adt1) -> bool {
    f28(match v0.clone() {
        Adt1::C0(_, h375, _, _, _, _, _, _, _, _, _) => {
            h375
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h376, _, _, _, _, _, _, _, _) => {
            h376
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h377, _, _, _, _, _, _, _) => {
            h377
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, h378, _, _, _, _, _, _) => {
            h378
        }
    })
}

pub fn f12(v0: u64) -> u64 {
    let a379 = v0;
    let a380 = 0u64;
    if nat_eq(a379, a380) {
        4u64
    } else {
        let a381 = v0;
        let a382 = 1u64;
        if if nat_eq(a381, a382) {
            true
        } else {
            let a383 = v0;
            let a384 = 30u64;
            nat_eq(a383, a384)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f13(v0: Adt1) -> bool {
    let v1: u64 = f12(match v0.clone() {
        Adt1::C0(h385, _, _, _, _, _, _, _, _, _, _) => {
            h385
        }
    });
    let a390 = {
        let a387 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h386) => {
                h386
            }
        };
        let a388 = v1;
        let a389 = 1u64;
        nat_rem(a387, a388, a389)
    };
    let a391 = 0u64;
    if if nat_eq(a390, a391) {
        f29(match v0.clone() {
            Adt1::C0(_, h392, _, _, _, _, _, _, _, _, _) => {
                h392
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h393, _, _, _, _, _) => {
                h393
            }
        }, v1)
    } else {
        false
    } {
        f30(match v0.clone() {
            Adt1::C0(_, _, h394, _, _, _, _, _, _, _, _) => {
                h394
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h395, _, _, _, _, _, _, _) => {
                h395
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h396, _, _, _, _, _, _) => {
                h396
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h397, _, _, _, _) => {
                h397
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h398, _, _, _) => {
                h398
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h399, _, _) => {
                h399
            }
        }, v1)
    } else {
        false
    }
}

pub fn f14(v0: Adt1, v1: Adt1) -> R<bool> {
    let a407 = {
        let a405 = match v0.clone() {
            Adt1::C0(_, _, h400, _, _, _, _, _, _, _, _) => {
                h400
            }
        };
        let a406 = {
            let a403 = match v0.clone() {
                Adt1::C0(_, _, _, h401, _, _, _, _, _, _, _) => {
                    h401
                }
            };
            let a404 = match v0.clone() {
                Adt1::C0(_, _, _, _, h402, _, _, _, _, _, _) => {
                    h402
                }
            };
            nat_mul(a403, a404)?
        };
        nat_mul(a405, a406)?
    };
    let a408 = 9223372036854775807u64;
    if nat_le(a407, a408) {
        if f22(v1.clone()) {
            let a410 = match v1 {
                Adt1::C0(_, h409, _, _, _, _, _, _, _, _, _) => {
                    h409
                }
            };
            let a411 = 2147483647u64;
            Ok(nat_le(a410, a411))
        } else {
            let a413 = match v0 {
                Adt1::C0(_, h412, _, _, _, _, _, _, _, _, _) => {
                    h412
                }
            };
            let a414 = 2147483647u64;
            Ok(nat_le(a413, a414))
        }
    } else {
        Ok(false)
    }
}

pub fn f15(v0: Adt1, v1: Adt1) -> R<bool> {
    if f31(v0.clone(), v1.clone())? {
        Ok(true)
    } else {
        Ok(f32(v0, v1))
    }
}

pub fn f16(v0: Adt1) -> R<bool> {
    f33({
        let a417 = match v0.clone() {
            Adt1::C0(_, h415, _, _, _, _, _, _, _, _, _) => {
                h415
            }
        };
        let a418 = f12(match v0.clone() {
            Adt1::C0(h416, _, _, _, _, _, _, _, _, _, _) => {
                h416
            }
        });
        nat_mul(a417, a418)?
    }, match v0.clone() {
        Adt1::C0(_, _, h419, _, _, _, _, _, _, _, _) => {
            h419
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h420, _, _, _, _, _, _, _) => {
            h420
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h421, _, _, _, _, _, _) => {
            h421
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h422, _, _, _, _) => {
            h422
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h423, _, _, _) => {
            h423
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h424, _, _) => {
            h424
        }
    })
}

pub fn f17(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a431 = {
        let a425 = v0;
        let a426 = v2;
        nat_mul(a425, a426)?
    };
    let a432 = {
        let a427 = v1;
        let a428 = 0u64;
        if nat_eq(a427, a428) {
            0u64
        } else {
            let a429 = v0;
            let a430 = 2u64;
            nat_mul(a429, a430)?
        }
    };
    nat_add(a431, a432)
}

pub fn f18(v0: Adt1) -> R<u64> {
    let a441 = {
        let a435 = match v0.clone() {
            Adt1::C0(_, h433, _, _, _, _, _, _, _, _, _) => {
                h433
            }
        };
        let a436 = match v0.clone() {
            Adt1::C0(_, _, h434, _, _, _, _, _, _, _, _) => {
                h434
            }
        };
        nat_mul(a435, a436)?
    };
    let a442 = {
        let a439 = match v0.clone() {
            Adt1::C0(_, _, _, h437, _, _, _, _, _, _, _) => {
                h437
            }
        };
        let a440 = match v0 {
            Adt1::C0(_, _, _, _, h438, _, _, _, _, _, _) => {
                h438
            }
        };
        nat_mul(a439, a440)?
    };
    nat_mul(a441, a442)
}

pub fn f19(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a453 = {
        let a449 = {
            let a445 = 1u64;
            let a446 = {
                let a443 = v0;
                let a444 = 0u64;
                if nat_eq(a443, a444) {
                    0u64
                } else {
                    1u64
                }
            };
            nat_add(a445, a446)?
        };
        let a450 = {
            let a447 = v1;
            let a448 = 0u64;
            if nat_eq(a447, a448) {
                0u64
            } else {
                1u64
            }
        };
        nat_add(a449, a450)?
    };
    let a454 = {
        let a451 = v2;
        let a452 = 0u64;
        if nat_eq(a451, a452) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a453, a454)
}

pub fn f20(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f8(v0.clone(), v1.clone()) {
        let a457 = match v0.clone() {
            Adt1::C0(h455, _, _, _, _, _, _, _, _, _, _) => {
                h455
            }
        };
        let a458 = match v1.clone() {
            Adt1::C0(h456, _, _, _, _, _, _, _, _, _, _) => {
                h456
            }
        };
        nat_eq(a457, a458)
    } else {
        false
    } {
        let a461 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h459, _, _, _, _, _) => {
                h459
            }
        };
        let a462 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h460, _, _, _, _, _) => {
                h460
            }
        };
        nat_eq(a461, a462)
    } else {
        false
    } {
        let a465 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h463, _, _, _, _) => {
                h463
            }
        };
        let a466 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h464, _, _, _, _) => {
                h464
            }
        };
        nat_eq(a465, a466)
    } else {
        false
    } {
        let a469 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h467, _, _, _) => {
                h467
            }
        };
        let a470 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h468, _, _, _) => {
                h468
            }
        };
        nat_eq(a469, a470)
    } else {
        false
    } {
        let a473 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h471, _, _) => {
                h471
            }
        };
        let a474 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h472, _, _) => {
                h472
            }
        };
        nat_eq(a473, a474)
    } else {
        false
    } {
        let a476 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h475) => {
                h475
            }
        };
        let a477 = 0u64;
        if if nat_eq(a476, a477) {
            true
        } else {
            let a479 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h478) => {
                    h478
                }
            };
            let a480 = 0u64;
            nat_eq(a479, a480)
        } {
            true
        } else {
            let a483 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h481) => {
                    h481
                }
            };
            let a484 = match v1 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h482) => {
                    h482
                }
            };
            nat_eq(a483, a484)
        }
    } else {
        false
    }
}

pub fn f21(v0: u64) -> u64 {
    let a485 = v0;
    let a486 = 26u64;
    if nat_eq(a485, a486) {
        4u64
    } else {
        let a487 = v0;
        let a488 = 27u64;
        if nat_eq(a487, a488) {
            8u64
        } else {
            0u64
        }
    }
}

pub fn f22(v0: Adt1) -> bool {
    let a490 = match v0.clone() {
        Adt1::C0(_, h489, _, _, _, _, _, _, _, _, _) => {
            h489
        }
    };
    let a491 = 1u64;
    if nat_eq(a490, a491) {
        true
    } else {
        let a494 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h492, _, _, _, _, _) => {
                h492
            }
        };
        let a495 = f12(match v0 {
            Adt1::C0(h493, _, _, _, _, _, _, _, _, _, _) => {
                h493
            }
        });
        nat_eq(a494, a495)
    }
}

pub fn f23(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a498 = match v0.clone() {
            Adt1::C0(_, _, h496, _, _, _, _, _, _, _, _) => {
                h496
            }
        };
        let a499 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h497, _, _, _, _) => {
                h497
            }
        };
        nat_mul(a498, a499)?
    };
    if if f22(v0.clone()) {
        let a501 = match v0.clone() {
            Adt1::C0(_, _, _, h500, _, _, _, _, _, _, _) => {
                h500
            }
        };
        let a502 = 1u64;
        if nat_eq(a501, a502) {
            true
        } else {
            let a504 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h503, _, _, _) => {
                    h503
                }
            };
            let a505 = v1;
            nat_eq(a504, a505)
        }
    } else {
        false
    } {
        let a507 = match v0.clone() {
            Adt1::C0(_, _, _, _, h506, _, _, _, _, _, _) => {
                h506
            }
        };
        let a508 = 1u64;
        if nat_eq(a507, a508) {
            Ok(true)
        } else {
            let a513 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h509, _, _) => {
                    h509
                }
            };
            let a514 = {
                let a511 = v1;
                let a512 = match v0 {
                    Adt1::C0(_, _, _, h510, _, _, _, _, _, _, _) => {
                        h510
                    }
                };
                nat_mul(a511, a512)?
            };
            Ok(nat_eq(a513, a514))
        }
    } else {
        Ok(false)
    }
}

pub fn f24(v0: u64, v1: Adt1, v2: u64, v3: u64) -> R<Adt3> {
    let v4: u64 = {
        let a515 = v0;
        let a516 = 2u64;
        let a517 = 0u64;
        nat_quot(a515, a516, a517)
    };
    let v5: u64 = f21(match v1.clone() {
        Adt1::C0(h518, _, _, _, _, _, _, _, _, _, _) => {
            h518
        }
    });
    let a522 = {
        let a519 = v0;
        let a520 = 2u64;
        let a521 = 0u64;
        nat_rem(a519, a520, a521)
    };
    let a523 = 0u64;
    if nat_eq(a522, a523) {
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 4u64, f26(v4, v1)?, 2u64, 0u64, 0u64, 2u64, v5, 8u64))
    } else {
        let a524 = v3;
        let a525 = v2;
        if if nat_lt(a524, a525) {
            let a528 = v3;
            let a529 = {
                let a526 = v5;
                let a527 = 4u64;
                if nat_eq(a526, a527) {
                    2147483648u64
                } else {
                    9223372036854775808u64
                }
            };
            nat_lt(a528, a529)
        } else {
            false
        } {
            Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, {
                let a530 = 3u64;
                let a531 = v4;
                nat_add(a530, a531)?
            }, 8u64, 8u64))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f25(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64) -> R<Adt3> {
    let v5: u64 = {
        let a533 = 1u64;
        let a534 = match v1.clone() {
            Adt1::C0(_, h532, _, _, _, _, _, _, _, _, _) => {
                h532
            }
        };
        nat_add(a533, a534)?
    };
    let v6: u64 = {
        let a535 = v0;
        let a536 = v5;
        let a537 = 0u64;
        nat_rem(a535, a536, a537)
    };
    let v7: u64 = {
        let a538 = v0;
        let a539 = v5;
        let a540 = 0u64;
        nat_quot(a538, a539, a540)
    };
    let v8: u64 = {
        let a542 = v7;
        let a543 = match v1.clone() {
            Adt1::C0(_, _, h541, _, _, _, _, _, _, _, _) => {
                h541
            }
        };
        let a544 = 0u64;
        nat_rem(a542, a543, a544)
    };
    let v9: u64 = {
        let a546 = v7;
        let a547 = match v1.clone() {
            Adt1::C0(_, _, h545, _, _, _, _, _, _, _, _) => {
                h545
            }
        };
        let a548 = 0u64;
        nat_quot(a546, a547, a548)
    };
    let v10: u64 = {
        let a550 = v9;
        let a551 = match v1.clone() {
            Adt1::C0(_, _, _, h549, _, _, _, _, _, _, _) => {
                h549
            }
        };
        let a552 = 0u64;
        nat_rem(a550, a551, a552)
    };
    let v11: u64 = {
        let a554 = v9;
        let a555 = match v1.clone() {
            Adt1::C0(_, _, _, h553, _, _, _, _, _, _, _) => {
                h553
            }
        };
        let a556 = 0u64;
        nat_quot(a554, a555, a556)
    };
    let a557 = v6;
    let a558 = 0u64;
    if nat_eq(a557, a558) {
        let v12: u64 = {
            let a575 = v8;
            let a576 = {
                let a573 = match v2.clone() {
                    Adt1::C0(_, h559, _, _, _, _, _, _, _, _, _) => {
                        h559
                    }
                };
                let a574 = {
                    let a571 = {
                        let a561 = v10;
                        let a562 = match v2.clone() {
                            Adt1::C0(_, _, h560, _, _, _, _, _, _, _, _) => {
                                h560
                            }
                        };
                        let a563 = 0u64;
                        nat_rem(a561, a562, a563)
                    };
                    let a572 = {
                        let a569 = match v2.clone() {
                            Adt1::C0(_, _, h564, _, _, _, _, _, _, _, _) => {
                                h564
                            }
                        };
                        let a570 = {
                            let a566 = v11;
                            let a567 = match v2 {
                                Adt1::C0(_, _, _, h565, _, _, _, _, _, _, _) => {
                                    h565
                                }
                            };
                            let a568 = 0u64;
                            nat_rem(a566, a567, a568)
                        };
                        nat_mul(a569, a570)?
                    };
                    nat_add(a571, a572)?
                };
                nat_mul(a573, a574)?
            };
            nat_add(a575, a576)?
        };
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, {
            let a577 = 3u64;
            let a578 = v12;
            nat_add(a577, a578)?
        }, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
    } else {
        let a580 = v4;
        let a581 = match v3.clone() {
            Adt1::C0(_, _, h579, _, _, _, _, _, _, _, _) => {
                h579
            }
        };
        if nat_lt(a580, a581) {
            let v13: u64 = {
                let a582 = v6;
                let a583 = 1u64;
                nat_sub(a582, a583)
            };
            let v14: u64 = f34(v13, v8, v10, v11, match v1.clone() {
                Adt1::C0(_, _, _, _, _, h584, _, _, _, _, _) => {
                    h584
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, h585, _, _, _, _) => {
                    h585
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h586, _, _, _) => {
                    h586
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h587, _, _) => {
                    h587
                }
            })?;
            let v15: u64 = f34(v13, v4, v10, v11, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h588, _, _, _, _, _) => {
                    h588
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h589, _, _, _, _) => {
                    h589
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h590, _, _, _) => {
                    h590
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h591, _, _) => {
                    h591
                }
            })?;
            Ok(Adt3::C3(0u64, 0u64, match v1.clone() {
                Adt1::C0(h592, _, _, _, _, _, _, _, _, _, _) => {
                    h592
                }
            }, match v3.clone() {
                Adt1::C0(h593, _, _, _, _, _, _, _, _, _, _) => {
                    h593
                }
            }, 0u64, v14, 2u64, 0u64, 3u64, v15, f12(match v1 {
                Adt1::C0(h594, _, _, _, _, _, _, _, _, _, _) => {
                    h594
                }
            }), f12(match v3 {
                Adt1::C0(h595, _, _, _, _, _, _, _, _, _, _) => {
                    h595
                }
            })))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f26(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a597 = v0;
        let a598 = match v1.clone() {
            Adt1::C0(_, h596, _, _, _, _, _, _, _, _, _) => {
                h596
            }
        };
        let a599 = 0u64;
        nat_quot(a597, a598, a599)
    };
    let v3: u64 = {
        let a601 = v2;
        let a602 = match v1.clone() {
            Adt1::C0(_, _, h600, _, _, _, _, _, _, _, _) => {
                h600
            }
        };
        let a603 = 0u64;
        nat_quot(a601, a602, a603)
    };
    f34({
        let a605 = v0;
        let a606 = match v1.clone() {
            Adt1::C0(_, h604, _, _, _, _, _, _, _, _, _) => {
                h604
            }
        };
        let a607 = 0u64;
        nat_rem(a605, a606, a607)
    }, {
        let a609 = v2;
        let a610 = match v1.clone() {
            Adt1::C0(_, _, h608, _, _, _, _, _, _, _, _) => {
                h608
            }
        };
        let a611 = 0u64;
        nat_rem(a609, a610, a611)
    }, {
        let a613 = v3;
        let a614 = match v1.clone() {
            Adt1::C0(_, _, _, h612, _, _, _, _, _, _, _) => {
                h612
            }
        };
        let a615 = 0u64;
        nat_rem(a613, a614, a615)
    }, {
        let a617 = v3;
        let a618 = match v1.clone() {
            Adt1::C0(_, _, _, h616, _, _, _, _, _, _, _) => {
                h616
            }
        };
        let a619 = 0u64;
        nat_quot(a617, a618, a619)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h620, _, _, _, _, _) => {
            h620
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h621, _, _, _, _) => {
            h621
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h622, _, _, _) => {
            h622
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h623, _, _) => {
            h623
        }
    })
}

pub fn f27(v0: u64, v1: Adt1, v2: Adt1) -> R<u64> {
    let v3: u64 = {
        let a625 = v0;
        let a626 = match v1.clone() {
            Adt1::C0(_, h624, _, _, _, _, _, _, _, _, _) => {
                h624
            }
        };
        let a627 = 0u64;
        nat_quot(a625, a626, a627)
    };
    let v4: u64 = {
        let a629 = v3;
        let a630 = match v1.clone() {
            Adt1::C0(_, _, h628, _, _, _, _, _, _, _, _) => {
                h628
            }
        };
        let a631 = 0u64;
        nat_quot(a629, a630, a631)
    };
    f34({
        let a637 = {
            let a633 = v0;
            let a634 = match v1.clone() {
                Adt1::C0(_, h632, _, _, _, _, _, _, _, _, _) => {
                    h632
                }
            };
            let a635 = 0u64;
            nat_rem(a633, a634, a635)
        };
        let a638 = match v2.clone() {
            Adt1::C0(_, h636, _, _, _, _, _, _, _, _, _) => {
                h636
            }
        };
        let a639 = 0u64;
        nat_rem(a637, a638, a639)
    }, {
        let a645 = {
            let a641 = v3;
            let a642 = match v1.clone() {
                Adt1::C0(_, _, h640, _, _, _, _, _, _, _, _) => {
                    h640
                }
            };
            let a643 = 0u64;
            nat_rem(a641, a642, a643)
        };
        let a646 = match v2.clone() {
            Adt1::C0(_, _, h644, _, _, _, _, _, _, _, _) => {
                h644
            }
        };
        let a647 = 0u64;
        nat_rem(a645, a646, a647)
    }, {
        let a653 = {
            let a649 = v4;
            let a650 = match v1.clone() {
                Adt1::C0(_, _, _, h648, _, _, _, _, _, _, _) => {
                    h648
                }
            };
            let a651 = 0u64;
            nat_rem(a649, a650, a651)
        };
        let a654 = match v2.clone() {
            Adt1::C0(_, _, _, h652, _, _, _, _, _, _, _) => {
                h652
            }
        };
        let a655 = 0u64;
        nat_rem(a653, a654, a655)
    }, {
        let a661 = {
            let a657 = v4;
            let a658 = match v1 {
                Adt1::C0(_, _, _, h656, _, _, _, _, _, _, _) => {
                    h656
                }
            };
            let a659 = 0u64;
            nat_quot(a657, a658, a659)
        };
        let a662 = match v2.clone() {
            Adt1::C0(_, _, _, _, h660, _, _, _, _, _, _) => {
                h660
            }
        };
        let a663 = 0u64;
        nat_rem(a661, a662, a663)
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h664, _, _, _, _, _) => {
            h664
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h665, _, _, _, _) => {
            h665
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h666, _, _, _) => {
            h666
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h667, _, _) => {
            h667
        }
    })
}

pub fn f28(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a668 = v0;
    let a669 = 0u64;
    if if if nat_eq(a668, a669) {
        true
    } else {
        let a670 = v1;
        let a671 = 0u64;
        nat_eq(a670, a671)
    } {
        true
    } else {
        let a672 = v2;
        let a673 = 0u64;
        nat_eq(a672, a673)
    } {
        true
    } else {
        let a674 = v3;
        let a675 = 0u64;
        nat_eq(a674, a675)
    }
}

pub fn f29(v0: u64, v1: u64, v2: u64) -> bool {
    let a676 = v0;
    let a677 = 1u64;
    if nat_le(a676, a677) {
        true
    } else {
        let a681 = {
            let a678 = v1;
            let a679 = v2;
            let a680 = 1u64;
            nat_rem(a678, a679, a680)
        };
        let a682 = 0u64;
        nat_eq(a681, a682)
    }
}

pub fn f30(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f29(v0, v3, v6) {
        f29(v1, v4, v6)
    } else {
        false
    } {
        f29(v2, v5, v6)
    } else {
        false
    }
}

pub fn f31(v0: Adt1, v1: Adt1) -> R<bool> {
    let a684 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h683, _) => {
            h683
        }
    };
    let a685 = 0u64;
    if nat_eq(a684, a685) {
        Ok(true)
    } else {
        let a687 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h686, _) => {
                h686
            }
        };
        let a688 = 0u64;
        if nat_eq(a687, a688) {
            Ok(true)
        } else {
            let a690 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h689) => {
                    h689
                }
            };
            let a691 = 0u64;
            if nat_eq(a690, a691) {
                Ok(true)
            } else {
                let a693 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h692) => {
                        h692
                    }
                };
                let a694 = 0u64;
                if nat_eq(a693, a694) {
                    Ok(true)
                } else {
                    let a700 = {
                        let a697 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h695) => {
                                h695
                            }
                        };
                        let a698 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h696, _) => {
                                h696
                            }
                        };
                        nat_add(a697, a698)?
                    };
                    let a701 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h699) => {
                            h699
                        }
                    };
                    if nat_le(a700, a701) {
                        Ok(true)
                    } else {
                        let a707 = {
                            let a704 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h702) => {
                                    h702
                                }
                            };
                            let a705 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h703, _) => {
                                    h703
                                }
                            };
                            nat_add(a704, a705)?
                        };
                        let a708 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h706) => {
                                h706
                            }
                        };
                        Ok(nat_le(a707, a708))
                    }
                }
            }
        }
    }
}

pub fn f32(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f8(v0.clone(), v1.clone()) {
        let a711 = match v0.clone() {
            Adt1::C0(h709, _, _, _, _, _, _, _, _, _, _) => {
                h709
            }
        };
        let a712 = match v1.clone() {
            Adt1::C0(h710, _, _, _, _, _, _, _, _, _, _) => {
                h710
            }
        };
        nat_eq(a711, a712)
    } else {
        false
    } {
        let a715 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h713) => {
                h713
            }
        };
        let a716 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h714) => {
                h714
            }
        };
        nat_eq(a715, a716)
    } else {
        false
    } {
        let a719 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h717, _, _, _, _, _) => {
                h717
            }
        };
        let a720 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h718, _, _, _, _, _) => {
                h718
            }
        };
        nat_eq(a719, a720)
    } else {
        false
    } {
        let a723 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h721, _, _, _, _) => {
                h721
            }
        };
        let a724 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h722, _, _, _, _) => {
                h722
            }
        };
        nat_eq(a723, a724)
    } else {
        false
    } {
        let a727 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h725, _, _, _) => {
                h725
            }
        };
        let a728 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h726, _, _, _) => {
                h726
            }
        };
        nat_eq(a727, a728)
    } else {
        false
    } {
        let a731 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h729, _, _) => {
                h729
            }
        };
        let a732 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h730, _, _) => {
                h730
            }
        };
        nat_eq(a731, a732)
    } else {
        false
    }
}

pub fn f33(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f35(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f35(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f35(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f35(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f35(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f35(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f34(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a745 = {
        let a741 = {
            let a737 = {
                let a733 = v0;
                let a734 = v4;
                nat_mul(a733, a734)?
            };
            let a738 = {
                let a735 = v1;
                let a736 = v5;
                nat_mul(a735, a736)?
            };
            nat_add(a737, a738)?
        };
        let a742 = {
            let a739 = v2;
            let a740 = v6;
            nat_mul(a739, a740)?
        };
        nat_add(a741, a742)?
    };
    let a746 = {
        let a743 = v3;
        let a744 = v7;
        nat_mul(a743, a744)?
    };
    nat_add(a745, a746)
}

pub fn f35(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a747 = v1;
    let a748 = 1u64;
    if if if nat_le(a747, a748) {
        true
    } else {
        let a749 = v0;
        let a750 = v2;
        nat_le(a749, a750)
    } {
        let a751 = v3;
        let a752 = 1u64;
        if nat_le(a751, a752) {
            true
        } else {
            let a759 = {
                let a757 = v0;
                let a758 = {
                    let a755 = {
                        let a753 = v1;
                        let a754 = 1u64;
                        nat_sub(a753, a754)
                    };
                    let a756 = v2;
                    nat_mul(a755, a756)?
                };
                nat_add(a757, a758)?
            };
            let a760 = v4;
            nat_le(a759, a760)
        }
    } else {
        false
    } {
        let a761 = v5;
        let a762 = 1u64;
        if nat_le(a761, a762) {
            Ok(true)
        } else {
            let a775 = {
                let a773 = {
                    let a767 = v0;
                    let a768 = {
                        let a765 = {
                            let a763 = v1;
                            let a764 = 1u64;
                            nat_sub(a763, a764)
                        };
                        let a766 = v2;
                        nat_mul(a765, a766)?
                    };
                    nat_add(a767, a768)?
                };
                let a774 = {
                    let a771 = {
                        let a769 = v3;
                        let a770 = 1u64;
                        nat_sub(a769, a770)
                    };
                    let a772 = v4;
                    nat_mul(a771, a772)?
                };
                nat_add(a773, a774)?
            };
            let a776 = v6;
            Ok(nat_le(a775, a776))
        }
    } else {
        Ok(false)
    }
}
