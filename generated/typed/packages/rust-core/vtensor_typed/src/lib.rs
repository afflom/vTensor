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
            let a11 = match v23.clone() {
                Adt2::C0(_, _, h10, _, _, _, _) => {
                    h10
                }
            };
            let a12 = 41u64;
            if nat_eq(a11, a12) {
                f6(v22, v23, v24)
            } else {
                let a14 = 98u64;
                let a15 = match v23.clone() {
                    Adt2::C0(_, _, h13, _, _, _, _) => {
                        h13
                    }
                };
                if nat_le(a14, a15) {
                    f7(v22, v23)
                } else {
                    f8(v22, v23, v24)
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
                if if f4(v15) {
                    !v17
                } else {
                    false
                } {
                    let m9 = v19;
                    match m9 {
                        None => {
                            f5(v15, false, v16, v18.clone(), v18, v20)
                        }
                        Some(v21) => {
                            f5(v15, true, v16, v18, v21, v20)
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
    let a16 = v0;
    let a17 = 2u64;
    if nat_eq(a16, a17) {
        Some(0u64)
    } else {
        let a18 = v0;
        let a19 = 5u64;
        if nat_eq(a18, a19) {
            Some(1u64)
        } else {
            let a20 = v0;
            let a21 = 6u64;
            if nat_eq(a20, a21) {
                Some(2u64)
            } else {
                let a22 = v0;
                let a23 = 7u64;
                if nat_eq(a22, a23) {
                    Some(3u64)
                } else {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f2(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if if f9(v1.clone(), v3.clone()) {
        f10(v1.clone(), v2.clone())
    } else {
        false
    } {
        f11(match v1.clone() {
            Adt1::C0(h24, _, _, _, _, _, _, _, _, _, _) => {
                h24
            }
        }, match v2.clone() {
            Adt1::C0(h25, _, _, _, _, _, _, _, _, _, _) => {
                h25
            }
        }, match v3.clone() {
            Adt1::C0(h26, _, _, _, _, _, _, _, _, _, _) => {
                h26
            }
        })
    } else {
        false
    } {
        if f12(v1.clone()) {
            true
        } else {
            let a29 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, h27, _, _, _, _, _) => {
                    h27
                }
            };
            let a30 = f13(match v1.clone() {
                Adt1::C0(h28, _, _, _, _, _, _, _, _, _, _) => {
                    h28
                }
            });
            if if if if if if if nat_eq(a29, a30) {
                let a33 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h31, _, _, _, _, _) => {
                        h31
                    }
                };
                let a34 = f13(match v3.clone() {
                    Adt1::C0(h32, _, _, _, _, _, _, _, _, _, _) => {
                        h32
                    }
                });
                nat_eq(a33, a34)
            } else {
                false
            } {
                f14(v1.clone())
            } else {
                false
            } {
                f14(v2.clone())
            } else {
                false
            } {
                f14(v3.clone())
            } else {
                false
            } {
                f15(v1.clone(), v2.clone())?
            } else {
                false
            } {
                f16(v1.clone(), v3.clone())?
            } else {
                false
            } {
                f16(v2.clone(), v3.clone())?
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v4: u64 = if if f12(v1.clone()) {
            true
        } else {
            f17(v3.clone())?
        } {
            0u64
        } else {
            let a37 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h35, _) => {
                    h35
                }
            };
            let a38 = f13(match v3.clone() {
                Adt1::C0(h36, _, _, _, _, _, _, _, _, _, _) => {
                    h36
                }
            });
            let a39 = 0u64;
            nat_quot(a37, a38, a39)
        };
        Ok(Adt3::C2(Adt2::C0(f18(f19(v1.clone())?, v4, f20(match v1.clone() {
            Adt1::C0(h40, _, _, _, _, _, _, _, _, _, _) => {
                h40
            }
        }, match v2.clone() {
            Adt1::C0(h41, _, _, _, _, _, _, _, _, _, _) => {
                h41
            }
        }, match v3.clone() {
            Adt1::C0(h42, _, _, _, _, _, _, _, _, _, _) => {
                h42
            }
        })?)?, {
            let a43 = 3u64;
            let a44 = v4;
            nat_add(a43, a44)?
        }, v0, 0u64, v1, v2, v3)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f3(v0: Adt1, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if if if if if if if if if if if if if f21(v2, v3.clone()) {
        let a46 = match v0.clone() {
            Adt1::C0(h45, _, _, _, _, _, _, _, _, _, _) => {
                h45
            }
        };
        let a47 = 0u64;
        if nat_eq(a46, a47) {
            true
        } else {
            let a49 = match v0.clone() {
                Adt1::C0(h48, _, _, _, _, _, _, _, _, _, _) => {
                    h48
                }
            };
            let a50 = 1u64;
            nat_eq(a49, a50)
        }
    } else {
        false
    } {
        let a52 = 0u64;
        let a53 = f13(match v3.clone() {
            Adt1::C0(h51, _, _, _, _, _, _, _, _, _, _) => {
                h51
            }
        });
        nat_lt(a52, a53)
    } else {
        false
    } {
        let a55 = 0u64;
        let a56 = f22(match v1.clone() {
            Adt1::C0(h54, _, _, _, _, _, _, _, _, _, _) => {
                h54
            }
        });
        nat_lt(a55, a56)
    } else {
        false
    } {
        let a59 = match v0.clone() {
            Adt1::C0(_, h57, _, _, _, _, _, _, _, _, _) => {
                h57
            }
        };
        let a60 = match v3.clone() {
            Adt1::C0(_, h58, _, _, _, _, _, _, _, _, _) => {
                h58
            }
        };
        nat_eq(a59, a60)
    } else {
        false
    } {
        let a63 = match v0.clone() {
            Adt1::C0(_, _, _, h61, _, _, _, _, _, _, _) => {
                h61
            }
        };
        let a64 = match v3.clone() {
            Adt1::C0(_, _, _, h62, _, _, _, _, _, _, _) => {
                h62
            }
        };
        nat_eq(a63, a64)
    } else {
        false
    } {
        let a67 = match v0.clone() {
            Adt1::C0(_, _, _, _, h65, _, _, _, _, _, _) => {
                h65
            }
        };
        let a68 = match v3.clone() {
            Adt1::C0(_, _, _, _, h66, _, _, _, _, _, _) => {
                h66
            }
        };
        nat_eq(a67, a68)
    } else {
        false
    } {
        let a71 = match v0.clone() {
            Adt1::C0(_, _, h69, _, _, _, _, _, _, _, _) => {
                h69
            }
        };
        let a72 = match v1.clone() {
            Adt1::C0(_, h70, _, _, _, _, _, _, _, _, _) => {
                h70
            }
        };
        nat_eq(a71, a72)
    } else {
        false
    } {
        let a74 = match v1.clone() {
            Adt1::C0(_, _, _, _, h73, _, _, _, _, _, _) => {
                h73
            }
        };
        let a75 = 1u64;
        nat_eq(a74, a75)
    } else {
        false
    } {
        let a77 = 0u64;
        let a78 = match v1.clone() {
            Adt1::C0(_, _, h76, _, _, _, _, _, _, _, _) => {
                h76
            }
        };
        nat_lt(a77, a78)
    } else {
        false
    } {
        let a80 = 0u64;
        let a81 = match v1.clone() {
            Adt1::C0(_, _, _, h79, _, _, _, _, _, _, _) => {
                h79
            }
        };
        nat_lt(a80, a81)
    } else {
        false
    } {
        let a87 = {
            let a84 = match v0.clone() {
                Adt1::C0(_, _, _, h82, _, _, _, _, _, _, _) => {
                    h82
                }
            };
            let a85 = match v1.clone() {
                Adt1::C0(_, _, h83, _, _, _, _, _, _, _, _) => {
                    h83
                }
            };
            let a86 = 0u64;
            nat_rem(a84, a85, a86)
        };
        let a88 = 0u64;
        nat_eq(a87, a88)
    } else {
        false
    } {
        let a94 = {
            let a91 = match v0.clone() {
                Adt1::C0(_, _, _, _, h89, _, _, _, _, _, _) => {
                    h89
                }
            };
            let a92 = match v1.clone() {
                Adt1::C0(_, _, _, h90, _, _, _, _, _, _, _) => {
                    h90
                }
            };
            let a93 = 0u64;
            nat_rem(a91, a92, a93)
        };
        let a95 = 0u64;
        nat_eq(a94, a95)
    } else {
        false
    } {
        f23(v0.clone())
    } else {
        false
    } {
        f23(v3.clone())
    } else {
        false
    } {
        let v4: u64 = f19(v1.clone())?;
        let v5: u64 = {
            let a101 = match v0.clone() {
                Adt1::C0(_, _, h96, _, _, _, _, _, _, _, _) => {
                    h96
                }
            };
            let a102 = {
                let a99 = match v0.clone() {
                    Adt1::C0(_, _, _, h97, _, _, _, _, _, _, _) => {
                        h97
                    }
                };
                let a100 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h98, _, _, _, _, _, _) => {
                        h98
                    }
                };
                nat_mul(a99, a100)?
            };
            nat_mul(a101, a102)?
        };
        let v6: u64 = if f12(v0.clone()) {
            0u64
        } else {
            let a110 = {
                let a103 = 2u64;
                let a104 = v4;
                nat_mul(a103, a104)?
            };
            let a111 = {
                let a108 = v5;
                let a109 = {
                    let a106 = 1u64;
                    let a107 = match v0.clone() {
                        Adt1::C0(_, h105, _, _, _, _, _, _, _, _, _) => {
                            h105
                        }
                    };
                    nat_add(a106, a107)?
                };
                nat_mul(a108, a109)?
            };
            nat_add(a110, a111)?
        };
        Ok(Adt3::C2(Adt2::C0(v6, if f12(v0.clone()) {
            3u64
        } else {
            let a112 = 3u64;
            let a113 = v4;
            nat_add(a112, a113)?
        }, 41u64, 0u64, v0, v1, v3)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f4(v0: u64) -> bool {
    let a114 = v0;
    let a115 = 2u64;
    if nat_le(a114, a115) {
        true
    } else {
        let a116 = v0;
        let a117 = 5u64;
        nat_eq(a116, a117)
    }
}

pub fn f5(v0: u64, v1: bool, v2: u64, v3: Adt1, v4: Adt1, v5: Adt1) -> R<Adt3> {
    let a118 = v2;
    let a119 = 1u64;
    if if if if if if if if if if if if if if if nat_le(a118, a119) {
        let a121 = 0u64;
        let a122 = f13(match v3.clone() {
            Adt1::C0(h120, _, _, _, _, _, _, _, _, _, _) => {
                h120
            }
        });
        nat_lt(a121, a122)
    } else {
        false
    } {
        let a125 = match v3.clone() {
            Adt1::C0(h123, _, _, _, _, _, _, _, _, _, _) => {
                h123
            }
        };
        let a126 = match v4.clone() {
            Adt1::C0(h124, _, _, _, _, _, _, _, _, _, _) => {
                h124
            }
        };
        nat_eq(a125, a126)
    } else {
        false
    } {
        let a129 = match v3.clone() {
            Adt1::C0(h127, _, _, _, _, _, _, _, _, _, _) => {
                h127
            }
        };
        let a130 = match v5.clone() {
            Adt1::C0(h128, _, _, _, _, _, _, _, _, _, _) => {
                h128
            }
        };
        nat_eq(a129, a130)
    } else {
        false
    } {
        f9(v3.clone(), v4.clone())
    } else {
        false
    } {
        f24(v3.clone())?
    } else {
        false
    } {
        f24(v4.clone())?
    } else {
        false
    } {
        f24(v5.clone())?
    } else {
        false
    } {
        let a133 = match v3.clone() {
            Adt1::C0(_, _, h131, _, _, _, _, _, _, _, _) => {
                h131
            }
        };
        let a134 = match v5.clone() {
            Adt1::C0(_, _, h132, _, _, _, _, _, _, _, _) => {
                h132
            }
        };
        nat_eq(a133, a134)
    } else {
        false
    } {
        let a137 = match v3.clone() {
            Adt1::C0(_, _, _, h135, _, _, _, _, _, _, _) => {
                h135
            }
        };
        let a138 = match v5.clone() {
            Adt1::C0(_, _, _, h136, _, _, _, _, _, _, _) => {
                h136
            }
        };
        nat_eq(a137, a138)
    } else {
        false
    } {
        let a141 = match v3.clone() {
            Adt1::C0(_, _, _, _, h139, _, _, _, _, _, _) => {
                h139
            }
        };
        let a142 = match v5.clone() {
            Adt1::C0(_, _, _, _, h140, _, _, _, _, _, _) => {
                h140
            }
        };
        nat_eq(a141, a142)
    } else {
        false
    } {
        let a149 = match v5.clone() {
            Adt1::C0(_, h143, _, _, _, _, _, _, _, _, _) => {
                h143
            }
        };
        let a150 = if v1 {
            match v3.clone() {
                Adt1::C0(_, h144, _, _, _, _, _, _, _, _, _) => {
                    h144
                }
            }
        } else {
            let a146 = match v3.clone() {
                Adt1::C0(_, h145, _, _, _, _, _, _, _, _, _) => {
                    h145
                }
            };
            let a147 = 2u64;
            let a148 = 0u64;
            nat_quot(a146, a147, a148)
        };
        nat_eq(a149, a150)
    } else {
        false
    } {
        let a152 = match v5.clone() {
            Adt1::C0(_, h151, _, _, _, _, _, _, _, _, _) => {
                h151
            }
        };
        let a153 = 2147483647u64;
        nat_le(a152, a153)
    } else {
        false
    } {
        f16(v3.clone(), v5.clone())?
    } else {
        false
    } {
        f16(v4.clone(), v5.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a154 = 4u64;
            let a155 = f19(v5.clone())?;
            nat_mul(a154, a155)?
        }, 3u64, if v1 {
            98u64
        } else {
            let a156 = 99u64;
            let a157 = v2;
            nat_add(a156, a157)?
        }, v0, v3, v4, v5)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f6(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let a159 = v0;
    let a160 = match v1.clone() {
        Adt2::C0(h158, _, _, _, _, _, _) => {
            h158
        }
    };
    if nat_lt(a159, a160) {
        let v3: u64 = {
            let a162 = 2u64;
            let a163 = f19(match v1.clone() {
                Adt2::C0(_, _, _, _, _, h161, _) => {
                    h161
                }
            })?;
            nat_mul(a162, a163)?
        };
        let a164 = v0;
        let a165 = v3;
        if nat_lt(a164, a165) {
            f25(v0, match v1.clone() {
                Adt2::C0(_, _, _, _, _, h166, _) => {
                    h166
                }
            }, {
                let m169 = match v1 {
                    Adt2::C0(_, _, _, _, _, _, h168) => {
                        h168
                    }
                };
                match m169 {
                    Adt1::C0(_, _, h167, _, _, _, _, _, _, _, _) => {
                        h167
                    }
                }
            }, v2)
        } else {
            f26({
                let a170 = v0;
                let a171 = v3;
                nat_sub(a170, a171)
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h172, _, _) => {
                    h172
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, h173, _) => {
                    h173
                }
            }, match v1 {
                Adt2::C0(_, _, _, _, _, _, h174) => {
                    h174
                }
            }, v2)
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f7(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, h175, _, _) => {
            h175
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, h176, _) => {
            h176
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h177) => {
            h177
        }
    };
    let v5: u64 = match v1.clone() {
        Adt2::C0(_, _, h178, _, _, _, _) => {
            h178
        }
    };
    let v6: u64 = {
        let a179 = v0;
        let a180 = 4u64;
        let a181 = 0u64;
        nat_rem(a179, a180, a181)
    };
    let v7: u64 = {
        let a182 = v0;
        let a183 = 4u64;
        let a184 = 0u64;
        nat_quot(a182, a183, a184)
    };
    let v8: u64 = {
        let a186 = v7;
        let a187 = match v4.clone() {
            Adt1::C0(_, h185, _, _, _, _, _, _, _, _, _) => {
                h185
            }
        };
        let a188 = 0u64;
        nat_quot(a186, a187, a188)
    };
    let v9: u64 = f13(match v4.clone() {
        Adt1::C0(h189, _, _, _, _, _, _, _, _, _, _) => {
            h189
        }
    });
    let v10: u64 = {
        let a194 = {
            let a191 = v7;
            let a192 = match v4.clone() {
                Adt1::C0(_, h190, _, _, _, _, _, _, _, _, _) => {
                    h190
                }
            };
            let a193 = 0u64;
            nat_rem(a191, a192, a193)
        };
        let a195 = v9;
        nat_mul(a194, a195)?
    };
    let v11: u64 = {
        let a197 = match v4.clone() {
            Adt1::C0(_, h196, _, _, _, _, _, _, _, _, _) => {
                h196
            }
        };
        let a198 = v9;
        nat_mul(a197, a198)?
    };
    let a200 = v0;
    let a201 = match v1.clone() {
        Adt2::C0(h199, _, _, _, _, _, _) => {
            h199
        }
    };
    if nat_lt(a200, a201) {
        let a202 = v6;
        let a203 = 0u64;
        if nat_eq(a202, a203) {
            Ok(Adt3::C3(0u64, 0u64, match v2.clone() {
                Adt1::C0(h204, _, _, _, _, _, _, _, _, _, _) => {
                    h204
                }
            }, 0u64, 0u64, {
                let a212 = {
                    let a208 = {
                        let a206 = v8;
                        let a207 = match v2 {
                            Adt1::C0(_, _, _, _, _, _, h205, _, _, _, _) => {
                                h205
                            }
                        };
                        nat_mul(a206, a207)?
                    };
                    let a209 = v10;
                    nat_add(a208, a209)?
                };
                let a213 = {
                    let a210 = v5;
                    let a211 = 100u64;
                    if nat_eq(a210, a211) {
                        v11
                    } else {
                        0u64
                    }
                };
                nat_add(a212, a213)?
            }, 2u64, 0u64, 0u64, 0u64, v9, 4u64))
        } else {
            let a214 = v6;
            let a215 = 1u64;
            if nat_eq(a214, a215) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h216, _, _, _, _, _, _, _, _, _, _) => {
                        h216
                    }
                }, 0u64, {
                    let a217 = v5;
                    let a218 = 98u64;
                    if nat_eq(a217, a218) {
                        4u64
                    } else {
                        0u64
                    }
                }, {
                    let a226 = {
                        let a222 = {
                            let a220 = v8;
                            let a221 = match v3 {
                                Adt1::C0(_, _, _, _, _, _, h219, _, _, _, _) => {
                                    h219
                                }
                            };
                            nat_mul(a220, a221)?
                        };
                        let a223 = v10;
                        nat_add(a222, a223)?
                    };
                    let a227 = {
                        let a224 = v5;
                        let a225 = 99u64;
                        if nat_eq(a224, a225) {
                            v11
                        } else {
                            0u64
                        }
                    };
                    nat_add(a226, a227)?
                }, 2u64, 0u64, 0u64, 1u64, v9, 4u64))
            } else {
                let a228 = v6;
                let a229 = 2u64;
                if nat_eq(a228, a229) {
                    Ok(Adt3::C4(match v1 {
                        Adt2::C0(_, _, _, h230, _, _, _) => {
                            h230
                        }
                    }, match v4 {
                        Adt1::C0(h231, _, _, _, _, _, _, _, _, _, _) => {
                            h231
                        }
                    }, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt3::C3(0u64, 0u64, 0u64, match v4.clone() {
                        Adt1::C0(h232, _, _, _, _, _, _, _, _, _, _) => {
                            h232
                        }
                    }, 1u64, 0u64, 2u64, 0u64, 3u64, {
                        let a236 = {
                            let a234 = v8;
                            let a235 = match v4 {
                                Adt1::C0(_, _, _, _, _, _, h233, _, _, _, _) => {
                                    h233
                                }
                            };
                            nat_mul(a234, a235)?
                        };
                        let a237 = v10;
                        nat_add(a236, a237)?
                    }, 4u64, v9))
                }
            }
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f8(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, h238, _, _) => {
            h238
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, h239, _) => {
            h239
        }
    };
    let v5: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h240) => {
            h240
        }
    };
    let v6: u64 = {
        let a242 = match v1.clone() {
            Adt2::C0(_, h241, _, _, _, _, _) => {
                h241
            }
        };
        let a243 = 3u64;
        if nat_eq(a242, a243) {
            0u64
        } else {
            let a244 = f19(v3.clone())?;
            let a245 = 2u64;
            nat_mul(a244, a245)?
        }
    };
    let a247 = match v1.clone() {
        Adt2::C0(h246, _, _, _, _, _, _) => {
            h246
        }
    };
    let a248 = v0;
    if nat_le(a247, a248) {
        Ok(Adt3::C1)
    } else {
        let a249 = v0;
        let a250 = v6;
        if nat_lt(a249, a250) {
            let v7: u64 = {
                let a251 = v0;
                let a252 = 2u64;
                let a253 = 0u64;
                nat_quot(a251, a252, a253)
            };
            let v8: u64 = {
                let a258 = 3u64;
                let a259 = {
                    let a255 = f27(v7, v5.clone())?;
                    let a256 = f13(match v5 {
                        Adt1::C0(h254, _, _, _, _, _, _, _, _, _, _) => {
                            h254
                        }
                    });
                    let a257 = 0u64;
                    nat_quot(a255, a256, a257)
                };
                nat_add(a258, a259)?
            };
            let a263 = {
                let a260 = v0;
                let a261 = 2u64;
                let a262 = 0u64;
                nat_rem(a260, a261, a262)
            };
            let a264 = 0u64;
            if nat_eq(a263, a264) {
                Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, v8, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
            } else {
                let a265 = v2;
                let a266 = 0u64;
                if nat_eq(a265, a266) {
                    Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 2u64, 1u64, 2u64, 0u64, 0u64, v8, 8u64, 8u64))
                } else {
                    Ok(Adt3::C1)
                }
            }
        } else {
            let v9: u64 = {
                let a267 = v0;
                let a268 = v6;
                nat_sub(a267, a268)
            };
            let v10: u64 = f20(match v3.clone() {
                Adt1::C0(h269, _, _, _, _, _, _, _, _, _, _) => {
                    h269
                }
            }, match v4.clone() {
                Adt1::C0(h270, _, _, _, _, _, _, _, _, _, _) => {
                    h270
                }
            }, match v5.clone() {
                Adt1::C0(h271, _, _, _, _, _, _, _, _, _, _) => {
                    h271
                }
            })?;
            let v11: u64 = {
                let a272 = v9;
                let a273 = v10;
                let a274 = 0u64;
                nat_quot(a272, a273, a274)
            };
            let v12: u64 = {
                let a275 = v9;
                let a276 = v10;
                let a277 = 0u64;
                nat_rem(a275, a276, a277)
            };
            let v13: u64 = {
                let a279 = match v3.clone() {
                    Adt1::C0(h278, _, _, _, _, _, _, _, _, _, _) => {
                        h278
                    }
                };
                let a280 = 0u64;
                if nat_eq(a279, a280) {
                    0u64
                } else {
                    1u64
                }
            };
            let v14: u64 = {
                let a282 = match v4.clone() {
                    Adt1::C0(h281, _, _, _, _, _, _, _, _, _, _) => {
                        h281
                    }
                };
                let a283 = 0u64;
                if nat_eq(a282, a283) {
                    0u64
                } else {
                    1u64
                }
            };
            let a284 = v12;
            let a285 = v13;
            if nat_lt(a284, a285) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h286, _, _, _, _, _, _, _, _, _, _) => {
                        h286
                    }
                }, 0u64, 0u64, f27(v11, v3.clone())?, 2u64, 0u64, 0u64, 0u64, f13(match v3 {
                    Adt1::C0(h287, _, _, _, _, _, _, _, _, _, _) => {
                        h287
                    }
                }), 4u64))
            } else {
                let a290 = v12;
                let a291 = {
                    let a288 = v13;
                    let a289 = v14;
                    nat_add(a288, a289)?
                };
                if nat_lt(a290, a291) {
                    Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                        Adt1::C0(h292, _, _, _, _, _, _, _, _, _, _) => {
                            h292
                        }
                    }, 0u64, 4u64, f28(v11, v3, v4.clone())?, 2u64, 0u64, 0u64, 1u64, f13(match v4 {
                        Adt1::C0(h293, _, _, _, _, _, _, _, _, _, _) => {
                            h293
                        }
                    }), 4u64))
                } else {
                    let a296 = v12;
                    let a297 = {
                        let a294 = v13;
                        let a295 = v14;
                        nat_add(a294, a295)?
                    };
                    if nat_eq(a296, a297) {
                        Ok(Adt3::C3(1u64, match v1 {
                            Adt2::C0(_, _, h298, _, _, _, _) => {
                                h298
                            }
                        }, 0u64, 0u64, {
                            let a299 = v13;
                            let a300 = 0u64;
                            if nat_eq(a299, a300) {
                                0u64
                            } else {
                                1u64
                            }
                        }, {
                            let a301 = v13;
                            let a302 = 0u64;
                            if nat_eq(a301, a302) {
                                f27(v11, v3.clone())?
                            } else {
                                0u64
                            }
                        }, {
                            let a303 = v14;
                            let a304 = 0u64;
                            if nat_eq(a303, a304) {
                                4u64
                            } else {
                                1u64
                            }
                        }, {
                            let a305 = v14;
                            let a306 = 0u64;
                            if nat_eq(a305, a306) {
                                f28(v11, v3, v4)?
                            } else {
                                1u64
                            }
                        }, {
                            let a308 = match v5.clone() {
                                Adt1::C0(h307, _, _, _, _, _, _, _, _, _, _) => {
                                    h307
                                }
                            };
                            let a309 = 0u64;
                            if nat_eq(a308, a309) {
                                3u64
                            } else {
                                0u64
                            }
                        }, {
                            let a311 = match v5.clone() {
                                Adt1::C0(h310, _, _, _, _, _, _, _, _, _, _) => {
                                    h310
                                }
                            };
                            let a312 = 0u64;
                            if nat_eq(a311, a312) {
                                f27(v11, v5)?
                            } else {
                                0u64
                            }
                        }, 4u64, 4u64))
                    } else {
                        Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                            Adt1::C0(h313, _, _, _, _, _, _, _, _, _, _) => {
                                h313
                            }
                        }, 1u64, 0u64, 2u64, 0u64, 3u64, f27(v11, v5.clone())?, 4u64, f13(match v5 {
                            Adt1::C0(h314, _, _, _, _, _, _, _, _, _, _) => {
                                h314
                            }
                        })))
                    }
                }
            }
        }
    }
}

pub fn f9(v0: Adt1, v1: Adt1) -> bool {
    let a317 = match v0.clone() {
        Adt1::C0(_, h315, _, _, _, _, _, _, _, _, _) => {
            h315
        }
    };
    let a318 = match v1.clone() {
        Adt1::C0(_, h316, _, _, _, _, _, _, _, _, _) => {
            h316
        }
    };
    if if if nat_eq(a317, a318) {
        let a321 = match v0.clone() {
            Adt1::C0(_, _, h319, _, _, _, _, _, _, _, _) => {
                h319
            }
        };
        let a322 = match v1.clone() {
            Adt1::C0(_, _, h320, _, _, _, _, _, _, _, _) => {
                h320
            }
        };
        nat_eq(a321, a322)
    } else {
        false
    } {
        let a325 = match v0.clone() {
            Adt1::C0(_, _, _, h323, _, _, _, _, _, _, _) => {
                h323
            }
        };
        let a326 = match v1.clone() {
            Adt1::C0(_, _, _, h324, _, _, _, _, _, _, _) => {
                h324
            }
        };
        nat_eq(a325, a326)
    } else {
        false
    } {
        let a329 = match v0 {
            Adt1::C0(_, _, _, _, h327, _, _, _, _, _, _) => {
                h327
            }
        };
        let a330 = match v1 {
            Adt1::C0(_, _, _, _, h328, _, _, _, _, _, _) => {
                h328
            }
        };
        nat_eq(a329, a330)
    } else {
        false
    }
}

pub fn f10(v0: Adt1, v1: Adt1) -> bool {
    if f12(v1.clone()) {
        f12(v0)
    } else {
        let a336 = {
            let a333 = match v0.clone() {
                Adt1::C0(_, h331, _, _, _, _, _, _, _, _, _) => {
                    h331
                }
            };
            let a334 = match v1.clone() {
                Adt1::C0(_, h332, _, _, _, _, _, _, _, _, _) => {
                    h332
                }
            };
            let a335 = 1u64;
            nat_rem(a333, a334, a335)
        };
        let a337 = 0u64;
        if if if nat_eq(a336, a337) {
            let a343 = {
                let a340 = match v0.clone() {
                    Adt1::C0(_, _, h338, _, _, _, _, _, _, _, _) => {
                        h338
                    }
                };
                let a341 = match v1.clone() {
                    Adt1::C0(_, _, h339, _, _, _, _, _, _, _, _) => {
                        h339
                    }
                };
                let a342 = 1u64;
                nat_rem(a340, a341, a342)
            };
            let a344 = 0u64;
            nat_eq(a343, a344)
        } else {
            false
        } {
            let a350 = {
                let a347 = match v0.clone() {
                    Adt1::C0(_, _, _, h345, _, _, _, _, _, _, _) => {
                        h345
                    }
                };
                let a348 = match v1.clone() {
                    Adt1::C0(_, _, _, h346, _, _, _, _, _, _, _) => {
                        h346
                    }
                };
                let a349 = 1u64;
                nat_rem(a347, a348, a349)
            };
            let a351 = 0u64;
            nat_eq(a350, a351)
        } else {
            false
        } {
            let a357 = {
                let a354 = match v0 {
                    Adt1::C0(_, _, _, _, h352, _, _, _, _, _, _) => {
                        h352
                    }
                };
                let a355 = match v1 {
                    Adt1::C0(_, _, _, _, h353, _, _, _, _, _, _) => {
                        h353
                    }
                };
                let a356 = 1u64;
                nat_rem(a354, a355, a356)
            };
            let a358 = 0u64;
            nat_eq(a357, a358)
        } else {
            false
        }
    }
}

pub fn f11(v0: u64, v1: u64, v2: u64) -> bool {
    let a359 = v0;
    let a360 = 0u64;
    if if if nat_eq(a359, a360) {
        let a361 = v1;
        let a362 = 0u64;
        nat_eq(a361, a362)
    } else {
        false
    } {
        let a363 = v2;
        let a364 = 0u64;
        nat_eq(a363, a364)
    } else {
        false
    } {
        true
    } else {
        let a365 = v0;
        let a366 = 1u64;
        if if nat_eq(a365, a366) {
            true
        } else {
            let a367 = v0;
            let a368 = 30u64;
            nat_eq(a367, a368)
        } {
            let a369 = v1;
            let a370 = v0;
            if if nat_eq(a369, a370) {
                let a371 = v2;
                let a372 = v0;
                nat_eq(a371, a372)
            } else {
                false
            } {
                true
            } else {
                let a373 = v1;
                let a374 = 0u64;
                if nat_eq(a373, a374) {
                    let a375 = v2;
                    let a376 = v0;
                    if nat_eq(a375, a376) {
                        true
                    } else {
                        let a377 = v2;
                        let a378 = 0u64;
                        nat_eq(a377, a378)
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

pub fn f12(v0: Adt1) -> bool {
    f29(match v0.clone() {
        Adt1::C0(_, h379, _, _, _, _, _, _, _, _, _) => {
            h379
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h380, _, _, _, _, _, _, _, _) => {
            h380
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h381, _, _, _, _, _, _, _) => {
            h381
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, h382, _, _, _, _, _, _) => {
            h382
        }
    })
}

pub fn f13(v0: u64) -> u64 {
    let a383 = v0;
    let a384 = 0u64;
    if nat_eq(a383, a384) {
        4u64
    } else {
        let a385 = v0;
        let a386 = 1u64;
        if if nat_eq(a385, a386) {
            true
        } else {
            let a387 = v0;
            let a388 = 30u64;
            nat_eq(a387, a388)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f14(v0: Adt1) -> bool {
    let v1: u64 = f13(match v0.clone() {
        Adt1::C0(h389, _, _, _, _, _, _, _, _, _, _) => {
            h389
        }
    });
    let a394 = {
        let a391 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h390) => {
                h390
            }
        };
        let a392 = v1;
        let a393 = 1u64;
        nat_rem(a391, a392, a393)
    };
    let a395 = 0u64;
    if if nat_eq(a394, a395) {
        f30(match v0.clone() {
            Adt1::C0(_, h396, _, _, _, _, _, _, _, _, _) => {
                h396
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h397, _, _, _, _, _) => {
                h397
            }
        }, v1)
    } else {
        false
    } {
        f31(match v0.clone() {
            Adt1::C0(_, _, h398, _, _, _, _, _, _, _, _) => {
                h398
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h399, _, _, _, _, _, _, _) => {
                h399
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h400, _, _, _, _, _, _) => {
                h400
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h401, _, _, _, _) => {
                h401
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h402, _, _, _) => {
                h402
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h403, _, _) => {
                h403
            }
        }, v1)
    } else {
        false
    }
}

pub fn f15(v0: Adt1, v1: Adt1) -> R<bool> {
    let a411 = {
        let a409 = match v0.clone() {
            Adt1::C0(_, _, h404, _, _, _, _, _, _, _, _) => {
                h404
            }
        };
        let a410 = {
            let a407 = match v0.clone() {
                Adt1::C0(_, _, _, h405, _, _, _, _, _, _, _) => {
                    h405
                }
            };
            let a408 = match v0.clone() {
                Adt1::C0(_, _, _, _, h406, _, _, _, _, _, _) => {
                    h406
                }
            };
            nat_mul(a407, a408)?
        };
        nat_mul(a409, a410)?
    };
    let a412 = 9223372036854775807u64;
    if nat_le(a411, a412) {
        if f23(v1.clone()) {
            let a414 = match v1 {
                Adt1::C0(_, h413, _, _, _, _, _, _, _, _, _) => {
                    h413
                }
            };
            let a415 = 2147483647u64;
            Ok(nat_le(a414, a415))
        } else {
            let a417 = match v0 {
                Adt1::C0(_, h416, _, _, _, _, _, _, _, _, _) => {
                    h416
                }
            };
            let a418 = 2147483647u64;
            Ok(nat_le(a417, a418))
        }
    } else {
        Ok(false)
    }
}

pub fn f16(v0: Adt1, v1: Adt1) -> R<bool> {
    if f32(v0.clone(), v1.clone())? {
        Ok(true)
    } else {
        Ok(f33(v0, v1))
    }
}

pub fn f17(v0: Adt1) -> R<bool> {
    f34({
        let a421 = match v0.clone() {
            Adt1::C0(_, h419, _, _, _, _, _, _, _, _, _) => {
                h419
            }
        };
        let a422 = f13(match v0.clone() {
            Adt1::C0(h420, _, _, _, _, _, _, _, _, _, _) => {
                h420
            }
        });
        nat_mul(a421, a422)?
    }, match v0.clone() {
        Adt1::C0(_, _, h423, _, _, _, _, _, _, _, _) => {
            h423
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h424, _, _, _, _, _, _, _) => {
            h424
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h425, _, _, _, _, _, _) => {
            h425
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h426, _, _, _, _) => {
            h426
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h427, _, _, _) => {
            h427
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h428, _, _) => {
            h428
        }
    })
}

pub fn f18(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a435 = {
        let a429 = v0;
        let a430 = v2;
        nat_mul(a429, a430)?
    };
    let a436 = {
        let a431 = v1;
        let a432 = 0u64;
        if nat_eq(a431, a432) {
            0u64
        } else {
            let a433 = v0;
            let a434 = 2u64;
            nat_mul(a433, a434)?
        }
    };
    nat_add(a435, a436)
}

pub fn f19(v0: Adt1) -> R<u64> {
    let a445 = {
        let a439 = match v0.clone() {
            Adt1::C0(_, h437, _, _, _, _, _, _, _, _, _) => {
                h437
            }
        };
        let a440 = match v0.clone() {
            Adt1::C0(_, _, h438, _, _, _, _, _, _, _, _) => {
                h438
            }
        };
        nat_mul(a439, a440)?
    };
    let a446 = {
        let a443 = match v0.clone() {
            Adt1::C0(_, _, _, h441, _, _, _, _, _, _, _) => {
                h441
            }
        };
        let a444 = match v0 {
            Adt1::C0(_, _, _, _, h442, _, _, _, _, _, _) => {
                h442
            }
        };
        nat_mul(a443, a444)?
    };
    nat_mul(a445, a446)
}

pub fn f20(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a457 = {
        let a453 = {
            let a449 = 1u64;
            let a450 = {
                let a447 = v0;
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
            let a451 = v1;
            let a452 = 0u64;
            if nat_eq(a451, a452) {
                0u64
            } else {
                1u64
            }
        };
        nat_add(a453, a454)?
    };
    let a458 = {
        let a455 = v2;
        let a456 = 0u64;
        if nat_eq(a455, a456) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a457, a458)
}

pub fn f21(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f9(v0.clone(), v1.clone()) {
        let a461 = match v0.clone() {
            Adt1::C0(h459, _, _, _, _, _, _, _, _, _, _) => {
                h459
            }
        };
        let a462 = match v1.clone() {
            Adt1::C0(h460, _, _, _, _, _, _, _, _, _, _) => {
                h460
            }
        };
        nat_eq(a461, a462)
    } else {
        false
    } {
        let a465 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h463, _, _, _, _, _) => {
                h463
            }
        };
        let a466 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h464, _, _, _, _, _) => {
                h464
            }
        };
        nat_eq(a465, a466)
    } else {
        false
    } {
        let a469 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h467, _, _, _, _) => {
                h467
            }
        };
        let a470 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h468, _, _, _, _) => {
                h468
            }
        };
        nat_eq(a469, a470)
    } else {
        false
    } {
        let a473 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h471, _, _, _) => {
                h471
            }
        };
        let a474 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h472, _, _, _) => {
                h472
            }
        };
        nat_eq(a473, a474)
    } else {
        false
    } {
        let a477 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h475, _, _) => {
                h475
            }
        };
        let a478 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h476, _, _) => {
                h476
            }
        };
        nat_eq(a477, a478)
    } else {
        false
    } {
        let a480 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h479) => {
                h479
            }
        };
        let a481 = 0u64;
        if if nat_eq(a480, a481) {
            true
        } else {
            let a483 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h482) => {
                    h482
                }
            };
            let a484 = 0u64;
            nat_eq(a483, a484)
        } {
            true
        } else {
            let a487 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h485) => {
                    h485
                }
            };
            let a488 = match v1 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h486) => {
                    h486
                }
            };
            nat_eq(a487, a488)
        }
    } else {
        false
    }
}

pub fn f22(v0: u64) -> u64 {
    let a489 = v0;
    let a490 = 26u64;
    if nat_eq(a489, a490) {
        4u64
    } else {
        let a491 = v0;
        let a492 = 27u64;
        if nat_eq(a491, a492) {
            8u64
        } else {
            0u64
        }
    }
}

pub fn f23(v0: Adt1) -> bool {
    let a494 = match v0.clone() {
        Adt1::C0(_, h493, _, _, _, _, _, _, _, _, _) => {
            h493
        }
    };
    let a495 = 1u64;
    if nat_eq(a494, a495) {
        true
    } else {
        let a498 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h496, _, _, _, _, _) => {
                h496
            }
        };
        let a499 = f13(match v0 {
            Adt1::C0(h497, _, _, _, _, _, _, _, _, _, _) => {
                h497
            }
        });
        nat_eq(a498, a499)
    }
}

pub fn f24(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a502 = match v0.clone() {
            Adt1::C0(_, _, h500, _, _, _, _, _, _, _, _) => {
                h500
            }
        };
        let a503 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h501, _, _, _, _) => {
                h501
            }
        };
        nat_mul(a502, a503)?
    };
    if if f23(v0.clone()) {
        let a505 = match v0.clone() {
            Adt1::C0(_, _, _, h504, _, _, _, _, _, _, _) => {
                h504
            }
        };
        let a506 = 1u64;
        if nat_eq(a505, a506) {
            true
        } else {
            let a508 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h507, _, _, _) => {
                    h507
                }
            };
            let a509 = v1;
            nat_eq(a508, a509)
        }
    } else {
        false
    } {
        let a511 = match v0.clone() {
            Adt1::C0(_, _, _, _, h510, _, _, _, _, _, _) => {
                h510
            }
        };
        let a512 = 1u64;
        if nat_eq(a511, a512) {
            Ok(true)
        } else {
            let a517 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h513, _, _) => {
                    h513
                }
            };
            let a518 = {
                let a515 = v1;
                let a516 = match v0 {
                    Adt1::C0(_, _, _, h514, _, _, _, _, _, _, _) => {
                        h514
                    }
                };
                nat_mul(a515, a516)?
            };
            Ok(nat_eq(a517, a518))
        }
    } else {
        Ok(false)
    }
}

pub fn f25(v0: u64, v1: Adt1, v2: u64, v3: u64) -> R<Adt3> {
    let v4: u64 = {
        let a519 = v0;
        let a520 = 2u64;
        let a521 = 0u64;
        nat_quot(a519, a520, a521)
    };
    let v5: u64 = f22(match v1.clone() {
        Adt1::C0(h522, _, _, _, _, _, _, _, _, _, _) => {
            h522
        }
    });
    let a526 = {
        let a523 = v0;
        let a524 = 2u64;
        let a525 = 0u64;
        nat_rem(a523, a524, a525)
    };
    let a527 = 0u64;
    if nat_eq(a526, a527) {
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 4u64, f27(v4, v1)?, 2u64, 0u64, 0u64, 2u64, v5, 8u64))
    } else {
        let a528 = v3;
        let a529 = v2;
        if if nat_lt(a528, a529) {
            let a532 = v3;
            let a533 = {
                let a530 = v5;
                let a531 = 4u64;
                if nat_eq(a530, a531) {
                    2147483648u64
                } else {
                    9223372036854775808u64
                }
            };
            nat_lt(a532, a533)
        } else {
            false
        } {
            Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, {
                let a534 = 3u64;
                let a535 = v4;
                nat_add(a534, a535)?
            }, 8u64, 8u64))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f26(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64) -> R<Adt3> {
    let v5: u64 = {
        let a537 = 1u64;
        let a538 = match v1.clone() {
            Adt1::C0(_, h536, _, _, _, _, _, _, _, _, _) => {
                h536
            }
        };
        nat_add(a537, a538)?
    };
    let v6: u64 = {
        let a539 = v0;
        let a540 = v5;
        let a541 = 0u64;
        nat_rem(a539, a540, a541)
    };
    let v7: u64 = {
        let a542 = v0;
        let a543 = v5;
        let a544 = 0u64;
        nat_quot(a542, a543, a544)
    };
    let v8: u64 = {
        let a546 = v7;
        let a547 = match v1.clone() {
            Adt1::C0(_, _, h545, _, _, _, _, _, _, _, _) => {
                h545
            }
        };
        let a548 = 0u64;
        nat_rem(a546, a547, a548)
    };
    let v9: u64 = {
        let a550 = v7;
        let a551 = match v1.clone() {
            Adt1::C0(_, _, h549, _, _, _, _, _, _, _, _) => {
                h549
            }
        };
        let a552 = 0u64;
        nat_quot(a550, a551, a552)
    };
    let v10: u64 = {
        let a554 = v9;
        let a555 = match v1.clone() {
            Adt1::C0(_, _, _, h553, _, _, _, _, _, _, _) => {
                h553
            }
        };
        let a556 = 0u64;
        nat_rem(a554, a555, a556)
    };
    let v11: u64 = {
        let a558 = v9;
        let a559 = match v1.clone() {
            Adt1::C0(_, _, _, h557, _, _, _, _, _, _, _) => {
                h557
            }
        };
        let a560 = 0u64;
        nat_quot(a558, a559, a560)
    };
    let a561 = v6;
    let a562 = 0u64;
    if nat_eq(a561, a562) {
        let v12: u64 = {
            let a579 = v8;
            let a580 = {
                let a577 = match v2.clone() {
                    Adt1::C0(_, h563, _, _, _, _, _, _, _, _, _) => {
                        h563
                    }
                };
                let a578 = {
                    let a575 = {
                        let a565 = v10;
                        let a566 = match v2.clone() {
                            Adt1::C0(_, _, h564, _, _, _, _, _, _, _, _) => {
                                h564
                            }
                        };
                        let a567 = 0u64;
                        nat_rem(a565, a566, a567)
                    };
                    let a576 = {
                        let a573 = match v2.clone() {
                            Adt1::C0(_, _, h568, _, _, _, _, _, _, _, _) => {
                                h568
                            }
                        };
                        let a574 = {
                            let a570 = v11;
                            let a571 = match v2 {
                                Adt1::C0(_, _, _, h569, _, _, _, _, _, _, _) => {
                                    h569
                                }
                            };
                            let a572 = 0u64;
                            nat_rem(a570, a571, a572)
                        };
                        nat_mul(a573, a574)?
                    };
                    nat_add(a575, a576)?
                };
                nat_mul(a577, a578)?
            };
            nat_add(a579, a580)?
        };
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, {
            let a581 = 3u64;
            let a582 = v12;
            nat_add(a581, a582)?
        }, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
    } else {
        let a584 = v4;
        let a585 = match v3.clone() {
            Adt1::C0(_, _, h583, _, _, _, _, _, _, _, _) => {
                h583
            }
        };
        if nat_lt(a584, a585) {
            let v13: u64 = {
                let a586 = v6;
                let a587 = 1u64;
                nat_sub(a586, a587)
            };
            let v14: u64 = f35(v13, v8, v10, v11, match v1.clone() {
                Adt1::C0(_, _, _, _, _, h588, _, _, _, _, _) => {
                    h588
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, h589, _, _, _, _) => {
                    h589
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h590, _, _, _) => {
                    h590
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h591, _, _) => {
                    h591
                }
            })?;
            let v15: u64 = f35(v13, v4, v10, v11, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h592, _, _, _, _, _) => {
                    h592
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h593, _, _, _, _) => {
                    h593
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h594, _, _, _) => {
                    h594
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h595, _, _) => {
                    h595
                }
            })?;
            Ok(Adt3::C3(0u64, 0u64, match v1.clone() {
                Adt1::C0(h596, _, _, _, _, _, _, _, _, _, _) => {
                    h596
                }
            }, match v3.clone() {
                Adt1::C0(h597, _, _, _, _, _, _, _, _, _, _) => {
                    h597
                }
            }, 0u64, v14, 2u64, 0u64, 3u64, v15, f13(match v1 {
                Adt1::C0(h598, _, _, _, _, _, _, _, _, _, _) => {
                    h598
                }
            }), f13(match v3 {
                Adt1::C0(h599, _, _, _, _, _, _, _, _, _, _) => {
                    h599
                }
            })))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f27(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a601 = v0;
        let a602 = match v1.clone() {
            Adt1::C0(_, h600, _, _, _, _, _, _, _, _, _) => {
                h600
            }
        };
        let a603 = 0u64;
        nat_quot(a601, a602, a603)
    };
    let v3: u64 = {
        let a605 = v2;
        let a606 = match v1.clone() {
            Adt1::C0(_, _, h604, _, _, _, _, _, _, _, _) => {
                h604
            }
        };
        let a607 = 0u64;
        nat_quot(a605, a606, a607)
    };
    f35({
        let a609 = v0;
        let a610 = match v1.clone() {
            Adt1::C0(_, h608, _, _, _, _, _, _, _, _, _) => {
                h608
            }
        };
        let a611 = 0u64;
        nat_rem(a609, a610, a611)
    }, {
        let a613 = v2;
        let a614 = match v1.clone() {
            Adt1::C0(_, _, h612, _, _, _, _, _, _, _, _) => {
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
        nat_rem(a617, a618, a619)
    }, {
        let a621 = v3;
        let a622 = match v1.clone() {
            Adt1::C0(_, _, _, h620, _, _, _, _, _, _, _) => {
                h620
            }
        };
        let a623 = 0u64;
        nat_quot(a621, a622, a623)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h624, _, _, _, _, _) => {
            h624
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h625, _, _, _, _) => {
            h625
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h626, _, _, _) => {
            h626
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h627, _, _) => {
            h627
        }
    })
}

pub fn f28(v0: u64, v1: Adt1, v2: Adt1) -> R<u64> {
    let v3: u64 = {
        let a629 = v0;
        let a630 = match v1.clone() {
            Adt1::C0(_, h628, _, _, _, _, _, _, _, _, _) => {
                h628
            }
        };
        let a631 = 0u64;
        nat_quot(a629, a630, a631)
    };
    let v4: u64 = {
        let a633 = v3;
        let a634 = match v1.clone() {
            Adt1::C0(_, _, h632, _, _, _, _, _, _, _, _) => {
                h632
            }
        };
        let a635 = 0u64;
        nat_quot(a633, a634, a635)
    };
    f35({
        let a641 = {
            let a637 = v0;
            let a638 = match v1.clone() {
                Adt1::C0(_, h636, _, _, _, _, _, _, _, _, _) => {
                    h636
                }
            };
            let a639 = 0u64;
            nat_rem(a637, a638, a639)
        };
        let a642 = match v2.clone() {
            Adt1::C0(_, h640, _, _, _, _, _, _, _, _, _) => {
                h640
            }
        };
        let a643 = 0u64;
        nat_rem(a641, a642, a643)
    }, {
        let a649 = {
            let a645 = v3;
            let a646 = match v1.clone() {
                Adt1::C0(_, _, h644, _, _, _, _, _, _, _, _) => {
                    h644
                }
            };
            let a647 = 0u64;
            nat_rem(a645, a646, a647)
        };
        let a650 = match v2.clone() {
            Adt1::C0(_, _, h648, _, _, _, _, _, _, _, _) => {
                h648
            }
        };
        let a651 = 0u64;
        nat_rem(a649, a650, a651)
    }, {
        let a657 = {
            let a653 = v4;
            let a654 = match v1.clone() {
                Adt1::C0(_, _, _, h652, _, _, _, _, _, _, _) => {
                    h652
                }
            };
            let a655 = 0u64;
            nat_rem(a653, a654, a655)
        };
        let a658 = match v2.clone() {
            Adt1::C0(_, _, _, h656, _, _, _, _, _, _, _) => {
                h656
            }
        };
        let a659 = 0u64;
        nat_rem(a657, a658, a659)
    }, {
        let a665 = {
            let a661 = v4;
            let a662 = match v1 {
                Adt1::C0(_, _, _, h660, _, _, _, _, _, _, _) => {
                    h660
                }
            };
            let a663 = 0u64;
            nat_quot(a661, a662, a663)
        };
        let a666 = match v2.clone() {
            Adt1::C0(_, _, _, _, h664, _, _, _, _, _, _) => {
                h664
            }
        };
        let a667 = 0u64;
        nat_rem(a665, a666, a667)
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h668, _, _, _, _, _) => {
            h668
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h669, _, _, _, _) => {
            h669
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h670, _, _, _) => {
            h670
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h671, _, _) => {
            h671
        }
    })
}

pub fn f29(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a672 = v0;
    let a673 = 0u64;
    if if if nat_eq(a672, a673) {
        true
    } else {
        let a674 = v1;
        let a675 = 0u64;
        nat_eq(a674, a675)
    } {
        true
    } else {
        let a676 = v2;
        let a677 = 0u64;
        nat_eq(a676, a677)
    } {
        true
    } else {
        let a678 = v3;
        let a679 = 0u64;
        nat_eq(a678, a679)
    }
}

pub fn f30(v0: u64, v1: u64, v2: u64) -> bool {
    let a680 = v0;
    let a681 = 1u64;
    if nat_le(a680, a681) {
        true
    } else {
        let a685 = {
            let a682 = v1;
            let a683 = v2;
            let a684 = 1u64;
            nat_rem(a682, a683, a684)
        };
        let a686 = 0u64;
        nat_eq(a685, a686)
    }
}

pub fn f31(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f30(v0, v3, v6) {
        f30(v1, v4, v6)
    } else {
        false
    } {
        f30(v2, v5, v6)
    } else {
        false
    }
}

pub fn f32(v0: Adt1, v1: Adt1) -> R<bool> {
    let a688 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h687, _) => {
            h687
        }
    };
    let a689 = 0u64;
    if nat_eq(a688, a689) {
        Ok(true)
    } else {
        let a691 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h690, _) => {
                h690
            }
        };
        let a692 = 0u64;
        if nat_eq(a691, a692) {
            Ok(true)
        } else {
            let a694 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h693) => {
                    h693
                }
            };
            let a695 = 0u64;
            if nat_eq(a694, a695) {
                Ok(true)
            } else {
                let a697 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h696) => {
                        h696
                    }
                };
                let a698 = 0u64;
                if nat_eq(a697, a698) {
                    Ok(true)
                } else {
                    let a704 = {
                        let a701 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h699) => {
                                h699
                            }
                        };
                        let a702 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h700, _) => {
                                h700
                            }
                        };
                        nat_add(a701, a702)?
                    };
                    let a705 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h703) => {
                            h703
                        }
                    };
                    if nat_le(a704, a705) {
                        Ok(true)
                    } else {
                        let a711 = {
                            let a708 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h706) => {
                                    h706
                                }
                            };
                            let a709 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h707, _) => {
                                    h707
                                }
                            };
                            nat_add(a708, a709)?
                        };
                        let a712 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h710) => {
                                h710
                            }
                        };
                        Ok(nat_le(a711, a712))
                    }
                }
            }
        }
    }
}

pub fn f33(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f9(v0.clone(), v1.clone()) {
        let a715 = match v0.clone() {
            Adt1::C0(h713, _, _, _, _, _, _, _, _, _, _) => {
                h713
            }
        };
        let a716 = match v1.clone() {
            Adt1::C0(h714, _, _, _, _, _, _, _, _, _, _) => {
                h714
            }
        };
        nat_eq(a715, a716)
    } else {
        false
    } {
        let a719 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h717) => {
                h717
            }
        };
        let a720 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h718) => {
                h718
            }
        };
        nat_eq(a719, a720)
    } else {
        false
    } {
        let a723 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h721, _, _, _, _, _) => {
                h721
            }
        };
        let a724 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h722, _, _, _, _, _) => {
                h722
            }
        };
        nat_eq(a723, a724)
    } else {
        false
    } {
        let a727 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h725, _, _, _, _) => {
                h725
            }
        };
        let a728 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h726, _, _, _, _) => {
                h726
            }
        };
        nat_eq(a727, a728)
    } else {
        false
    } {
        let a731 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h729, _, _, _) => {
                h729
            }
        };
        let a732 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h730, _, _, _) => {
                h730
            }
        };
        nat_eq(a731, a732)
    } else {
        false
    } {
        let a735 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h733, _, _) => {
                h733
            }
        };
        let a736 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h734, _, _) => {
                h734
            }
        };
        nat_eq(a735, a736)
    } else {
        false
    }
}

pub fn f34(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f36(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f36(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f36(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f36(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f36(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f36(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f35(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a749 = {
        let a745 = {
            let a741 = {
                let a737 = v0;
                let a738 = v4;
                nat_mul(a737, a738)?
            };
            let a742 = {
                let a739 = v1;
                let a740 = v5;
                nat_mul(a739, a740)?
            };
            nat_add(a741, a742)?
        };
        let a746 = {
            let a743 = v2;
            let a744 = v6;
            nat_mul(a743, a744)?
        };
        nat_add(a745, a746)?
    };
    let a750 = {
        let a747 = v3;
        let a748 = v7;
        nat_mul(a747, a748)?
    };
    nat_add(a749, a750)
}

pub fn f36(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a751 = v1;
    let a752 = 1u64;
    if if if nat_le(a751, a752) {
        true
    } else {
        let a753 = v0;
        let a754 = v2;
        nat_le(a753, a754)
    } {
        let a755 = v3;
        let a756 = 1u64;
        if nat_le(a755, a756) {
            true
        } else {
            let a763 = {
                let a761 = v0;
                let a762 = {
                    let a759 = {
                        let a757 = v1;
                        let a758 = 1u64;
                        nat_sub(a757, a758)
                    };
                    let a760 = v2;
                    nat_mul(a759, a760)?
                };
                nat_add(a761, a762)?
            };
            let a764 = v4;
            nat_le(a763, a764)
        }
    } else {
        false
    } {
        let a765 = v5;
        let a766 = 1u64;
        if nat_le(a765, a766) {
            Ok(true)
        } else {
            let a779 = {
                let a777 = {
                    let a771 = v0;
                    let a772 = {
                        let a769 = {
                            let a767 = v1;
                            let a768 = 1u64;
                            nat_sub(a767, a768)
                        };
                        let a770 = v2;
                        nat_mul(a769, a770)?
                    };
                    nat_add(a771, a772)?
                };
                let a778 = {
                    let a775 = {
                        let a773 = v3;
                        let a774 = 1u64;
                        nat_sub(a773, a774)
                    };
                    let a776 = v4;
                    nat_mul(a775, a776)?
                };
                nat_add(a777, a778)?
            };
            let a780 = v6;
            Ok(nat_le(a779, a780))
        }
    } else {
        Ok(false)
    }
}

pub fn run(p0: Adt0) -> R<Adt3> {
    f0(p0)
}
