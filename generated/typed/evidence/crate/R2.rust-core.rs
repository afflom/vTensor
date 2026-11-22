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
        Adt0::C1(v14, v15, v16) => {
            let a8 = match v15.clone() {
                Adt2::C0(_, _, h7, _, _, _) => {
                    h7
                }
            };
            let a9 = 41u64;
            if nat_eq(a8, a9) {
                f4(v14, v15, v16)
            } else {
                f5(v14, v15, v16)
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
    }
}

pub fn f1(v0: u64) -> Option<u64> {
    let a10 = v0;
    let a11 = 2u64;
    if nat_eq(a10, a11) {
        Some(0u64)
    } else {
        let a12 = v0;
        let a13 = 5u64;
        if nat_eq(a12, a13) {
            Some(1u64)
        } else {
            let a14 = v0;
            let a15 = 6u64;
            if nat_eq(a14, a15) {
                Some(2u64)
            } else {
                let a16 = v0;
                let a17 = 7u64;
                if nat_eq(a16, a17) {
                    Some(3u64)
                } else {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f2(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if if f6(v1.clone(), v3.clone()) {
        f7(v1.clone(), v2.clone())
    } else {
        false
    } {
        f8(match v1.clone() {
            Adt1::C0(h18, _, _, _, _, _, _, _, _, _, _) => {
                h18
            }
        }, match v2.clone() {
            Adt1::C0(h19, _, _, _, _, _, _, _, _, _, _) => {
                h19
            }
        }, match v3.clone() {
            Adt1::C0(h20, _, _, _, _, _, _, _, _, _, _) => {
                h20
            }
        })
    } else {
        false
    } {
        if f9(v1.clone()) {
            true
        } else {
            let a23 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, h21, _, _, _, _, _) => {
                    h21
                }
            };
            let a24 = f10(match v1.clone() {
                Adt1::C0(h22, _, _, _, _, _, _, _, _, _, _) => {
                    h22
                }
            });
            if if if if if if if nat_eq(a23, a24) {
                let a27 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h25, _, _, _, _, _) => {
                        h25
                    }
                };
                let a28 = f10(match v3.clone() {
                    Adt1::C0(h26, _, _, _, _, _, _, _, _, _, _) => {
                        h26
                    }
                });
                nat_eq(a27, a28)
            } else {
                false
            } {
                f11(v1.clone())
            } else {
                false
            } {
                f11(v2.clone())
            } else {
                false
            } {
                f11(v3.clone())
            } else {
                false
            } {
                f12(v1.clone(), v2.clone())?
            } else {
                false
            } {
                f13(v1.clone(), v3.clone())?
            } else {
                false
            } {
                f13(v2.clone(), v3.clone())?
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v4: u64 = if if f9(v1.clone()) {
            true
        } else {
            f14(v3.clone())?
        } {
            0u64
        } else {
            let a31 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h29, _) => {
                    h29
                }
            };
            let a32 = f10(match v3.clone() {
                Adt1::C0(h30, _, _, _, _, _, _, _, _, _, _) => {
                    h30
                }
            });
            let a33 = 0u64;
            nat_quot(a31, a32, a33)
        };
        Ok(Adt3::C2(Adt2::C0(f15(f16(v1.clone())?, v4, f17(match v1.clone() {
            Adt1::C0(h34, _, _, _, _, _, _, _, _, _, _) => {
                h34
            }
        }, match v2.clone() {
            Adt1::C0(h35, _, _, _, _, _, _, _, _, _, _) => {
                h35
            }
        }, match v3.clone() {
            Adt1::C0(h36, _, _, _, _, _, _, _, _, _, _) => {
                h36
            }
        })?)?, {
            let a37 = 3u64;
            let a38 = v4;
            nat_add(a37, a38)?
        }, v0, v1, v2, v3)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f3(v0: Adt1, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if if if if if if if if if if if if if f18(v2, v3.clone()) {
        let a40 = match v0.clone() {
            Adt1::C0(h39, _, _, _, _, _, _, _, _, _, _) => {
                h39
            }
        };
        let a41 = 0u64;
        if nat_eq(a40, a41) {
            true
        } else {
            let a43 = match v0.clone() {
                Adt1::C0(h42, _, _, _, _, _, _, _, _, _, _) => {
                    h42
                }
            };
            let a44 = 1u64;
            nat_eq(a43, a44)
        }
    } else {
        false
    } {
        let a46 = 0u64;
        let a47 = f10(match v3.clone() {
            Adt1::C0(h45, _, _, _, _, _, _, _, _, _, _) => {
                h45
            }
        });
        nat_lt(a46, a47)
    } else {
        false
    } {
        let a49 = 0u64;
        let a50 = f19(match v1.clone() {
            Adt1::C0(h48, _, _, _, _, _, _, _, _, _, _) => {
                h48
            }
        });
        nat_lt(a49, a50)
    } else {
        false
    } {
        let a53 = match v0.clone() {
            Adt1::C0(_, h51, _, _, _, _, _, _, _, _, _) => {
                h51
            }
        };
        let a54 = match v3.clone() {
            Adt1::C0(_, h52, _, _, _, _, _, _, _, _, _) => {
                h52
            }
        };
        nat_eq(a53, a54)
    } else {
        false
    } {
        let a57 = match v0.clone() {
            Adt1::C0(_, _, _, h55, _, _, _, _, _, _, _) => {
                h55
            }
        };
        let a58 = match v3.clone() {
            Adt1::C0(_, _, _, h56, _, _, _, _, _, _, _) => {
                h56
            }
        };
        nat_eq(a57, a58)
    } else {
        false
    } {
        let a61 = match v0.clone() {
            Adt1::C0(_, _, _, _, h59, _, _, _, _, _, _) => {
                h59
            }
        };
        let a62 = match v3.clone() {
            Adt1::C0(_, _, _, _, h60, _, _, _, _, _, _) => {
                h60
            }
        };
        nat_eq(a61, a62)
    } else {
        false
    } {
        let a65 = match v0.clone() {
            Adt1::C0(_, _, h63, _, _, _, _, _, _, _, _) => {
                h63
            }
        };
        let a66 = match v1.clone() {
            Adt1::C0(_, h64, _, _, _, _, _, _, _, _, _) => {
                h64
            }
        };
        nat_eq(a65, a66)
    } else {
        false
    } {
        let a68 = match v1.clone() {
            Adt1::C0(_, _, _, _, h67, _, _, _, _, _, _) => {
                h67
            }
        };
        let a69 = 1u64;
        nat_eq(a68, a69)
    } else {
        false
    } {
        let a71 = 0u64;
        let a72 = match v1.clone() {
            Adt1::C0(_, _, h70, _, _, _, _, _, _, _, _) => {
                h70
            }
        };
        nat_lt(a71, a72)
    } else {
        false
    } {
        let a74 = 0u64;
        let a75 = match v1.clone() {
            Adt1::C0(_, _, _, h73, _, _, _, _, _, _, _) => {
                h73
            }
        };
        nat_lt(a74, a75)
    } else {
        false
    } {
        let a81 = {
            let a78 = match v0.clone() {
                Adt1::C0(_, _, _, h76, _, _, _, _, _, _, _) => {
                    h76
                }
            };
            let a79 = match v1.clone() {
                Adt1::C0(_, _, h77, _, _, _, _, _, _, _, _) => {
                    h77
                }
            };
            let a80 = 0u64;
            nat_rem(a78, a79, a80)
        };
        let a82 = 0u64;
        nat_eq(a81, a82)
    } else {
        false
    } {
        let a88 = {
            let a85 = match v0.clone() {
                Adt1::C0(_, _, _, _, h83, _, _, _, _, _, _) => {
                    h83
                }
            };
            let a86 = match v1.clone() {
                Adt1::C0(_, _, _, h84, _, _, _, _, _, _, _) => {
                    h84
                }
            };
            let a87 = 0u64;
            nat_rem(a85, a86, a87)
        };
        let a89 = 0u64;
        nat_eq(a88, a89)
    } else {
        false
    } {
        f20(v0.clone())
    } else {
        false
    } {
        f20(v3.clone())
    } else {
        false
    } {
        let v4: u64 = f16(v1.clone())?;
        let v5: u64 = {
            let a95 = match v0.clone() {
                Adt1::C0(_, _, h90, _, _, _, _, _, _, _, _) => {
                    h90
                }
            };
            let a96 = {
                let a93 = match v0.clone() {
                    Adt1::C0(_, _, _, h91, _, _, _, _, _, _, _) => {
                        h91
                    }
                };
                let a94 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h92, _, _, _, _, _, _) => {
                        h92
                    }
                };
                nat_mul(a93, a94)?
            };
            nat_mul(a95, a96)?
        };
        let v6: u64 = if f9(v0.clone()) {
            0u64
        } else {
            let a104 = {
                let a97 = 2u64;
                let a98 = v4;
                nat_mul(a97, a98)?
            };
            let a105 = {
                let a102 = v5;
                let a103 = {
                    let a100 = 1u64;
                    let a101 = match v0.clone() {
                        Adt1::C0(_, h99, _, _, _, _, _, _, _, _, _) => {
                            h99
                        }
                    };
                    nat_add(a100, a101)?
                };
                nat_mul(a102, a103)?
            };
            nat_add(a104, a105)?
        };
        Ok(Adt3::C2(Adt2::C0(v6, if f9(v0.clone()) {
            3u64
        } else {
            let a106 = 3u64;
            let a107 = v4;
            nat_add(a106, a107)?
        }, 41u64, v0, v1, v3)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f4(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let a109 = v0;
    let a110 = match v1.clone() {
        Adt2::C0(h108, _, _, _, _, _) => {
            h108
        }
    };
    if nat_lt(a109, a110) {
        let v3: u64 = {
            let a112 = 2u64;
            let a113 = f16(match v1.clone() {
                Adt2::C0(_, _, _, _, h111, _) => {
                    h111
                }
            })?;
            nat_mul(a112, a113)?
        };
        let a114 = v0;
        let a115 = v3;
        if nat_lt(a114, a115) {
            f21(v0, match v1.clone() {
                Adt2::C0(_, _, _, _, h116, _) => {
                    h116
                }
            }, {
                let m119 = match v1 {
                    Adt2::C0(_, _, _, _, _, h118) => {
                        h118
                    }
                };
                match m119 {
                    Adt1::C0(_, _, h117, _, _, _, _, _, _, _, _) => {
                        h117
                    }
                }
            }, v2)
        } else {
            f22({
                let a120 = v0;
                let a121 = v3;
                nat_sub(a120, a121)
            }, match v1.clone() {
                Adt2::C0(_, _, _, h122, _, _) => {
                    h122
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h123, _) => {
                    h123
                }
            }, match v1 {
                Adt2::C0(_, _, _, _, _, h124) => {
                    h124
                }
            }, v2)
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f5(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, h125, _, _) => {
            h125
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, h126, _) => {
            h126
        }
    };
    let v5: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, h127) => {
            h127
        }
    };
    let v6: u64 = {
        let a129 = match v1.clone() {
            Adt2::C0(_, h128, _, _, _, _) => {
                h128
            }
        };
        let a130 = 3u64;
        if nat_eq(a129, a130) {
            0u64
        } else {
            let a131 = f16(v3.clone())?;
            let a132 = 2u64;
            nat_mul(a131, a132)?
        }
    };
    let a134 = match v1.clone() {
        Adt2::C0(h133, _, _, _, _, _) => {
            h133
        }
    };
    let a135 = v0;
    if nat_le(a134, a135) {
        Ok(Adt3::C1)
    } else {
        let a136 = v0;
        let a137 = v6;
        if nat_lt(a136, a137) {
            let v7: u64 = {
                let a138 = v0;
                let a139 = 2u64;
                let a140 = 0u64;
                nat_quot(a138, a139, a140)
            };
            let v8: u64 = {
                let a145 = 3u64;
                let a146 = {
                    let a142 = f23(v7, v5.clone())?;
                    let a143 = f10(match v5 {
                        Adt1::C0(h141, _, _, _, _, _, _, _, _, _, _) => {
                            h141
                        }
                    });
                    let a144 = 0u64;
                    nat_quot(a142, a143, a144)
                };
                nat_add(a145, a146)?
            };
            let a150 = {
                let a147 = v0;
                let a148 = 2u64;
                let a149 = 0u64;
                nat_rem(a147, a148, a149)
            };
            let a151 = 0u64;
            if nat_eq(a150, a151) {
                Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, v8, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
            } else {
                let a152 = v2;
                let a153 = 0u64;
                if nat_eq(a152, a153) {
                    Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 2u64, 1u64, 2u64, 0u64, 0u64, v8, 8u64, 8u64))
                } else {
                    Ok(Adt3::C1)
                }
            }
        } else {
            let v9: u64 = {
                let a154 = v0;
                let a155 = v6;
                nat_sub(a154, a155)
            };
            let v10: u64 = f17(match v3.clone() {
                Adt1::C0(h156, _, _, _, _, _, _, _, _, _, _) => {
                    h156
                }
            }, match v4.clone() {
                Adt1::C0(h157, _, _, _, _, _, _, _, _, _, _) => {
                    h157
                }
            }, match v5.clone() {
                Adt1::C0(h158, _, _, _, _, _, _, _, _, _, _) => {
                    h158
                }
            })?;
            let v11: u64 = {
                let a159 = v9;
                let a160 = v10;
                let a161 = 0u64;
                nat_quot(a159, a160, a161)
            };
            let v12: u64 = {
                let a162 = v9;
                let a163 = v10;
                let a164 = 0u64;
                nat_rem(a162, a163, a164)
            };
            let v13: u64 = {
                let a166 = match v3.clone() {
                    Adt1::C0(h165, _, _, _, _, _, _, _, _, _, _) => {
                        h165
                    }
                };
                let a167 = 0u64;
                if nat_eq(a166, a167) {
                    0u64
                } else {
                    1u64
                }
            };
            let v14: u64 = {
                let a169 = match v4.clone() {
                    Adt1::C0(h168, _, _, _, _, _, _, _, _, _, _) => {
                        h168
                    }
                };
                let a170 = 0u64;
                if nat_eq(a169, a170) {
                    0u64
                } else {
                    1u64
                }
            };
            let a171 = v12;
            let a172 = v13;
            if nat_lt(a171, a172) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h173, _, _, _, _, _, _, _, _, _, _) => {
                        h173
                    }
                }, 0u64, 0u64, f23(v11, v3.clone())?, 2u64, 0u64, 0u64, 0u64, f10(match v3 {
                    Adt1::C0(h174, _, _, _, _, _, _, _, _, _, _) => {
                        h174
                    }
                }), 4u64))
            } else {
                let a177 = v12;
                let a178 = {
                    let a175 = v13;
                    let a176 = v14;
                    nat_add(a175, a176)?
                };
                if nat_lt(a177, a178) {
                    Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                        Adt1::C0(h179, _, _, _, _, _, _, _, _, _, _) => {
                            h179
                        }
                    }, 0u64, 4u64, f24(v11, v3, v4.clone())?, 2u64, 0u64, 0u64, 1u64, f10(match v4 {
                        Adt1::C0(h180, _, _, _, _, _, _, _, _, _, _) => {
                            h180
                        }
                    }), 4u64))
                } else {
                    let a183 = v12;
                    let a184 = {
                        let a181 = v13;
                        let a182 = v14;
                        nat_add(a181, a182)?
                    };
                    if nat_eq(a183, a184) {
                        Ok(Adt3::C3(1u64, match v1 {
                            Adt2::C0(_, _, h185, _, _, _) => {
                                h185
                            }
                        }, 0u64, 0u64, {
                            let a186 = v13;
                            let a187 = 0u64;
                            if nat_eq(a186, a187) {
                                0u64
                            } else {
                                1u64
                            }
                        }, {
                            let a188 = v13;
                            let a189 = 0u64;
                            if nat_eq(a188, a189) {
                                f23(v11, v3.clone())?
                            } else {
                                0u64
                            }
                        }, {
                            let a190 = v14;
                            let a191 = 0u64;
                            if nat_eq(a190, a191) {
                                4u64
                            } else {
                                1u64
                            }
                        }, {
                            let a192 = v14;
                            let a193 = 0u64;
                            if nat_eq(a192, a193) {
                                f24(v11, v3, v4)?
                            } else {
                                1u64
                            }
                        }, {
                            let a195 = match v5.clone() {
                                Adt1::C0(h194, _, _, _, _, _, _, _, _, _, _) => {
                                    h194
                                }
                            };
                            let a196 = 0u64;
                            if nat_eq(a195, a196) {
                                3u64
                            } else {
                                0u64
                            }
                        }, {
                            let a198 = match v5.clone() {
                                Adt1::C0(h197, _, _, _, _, _, _, _, _, _, _) => {
                                    h197
                                }
                            };
                            let a199 = 0u64;
                            if nat_eq(a198, a199) {
                                f23(v11, v5)?
                            } else {
                                0u64
                            }
                        }, 4u64, 4u64))
                    } else {
                        Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                            Adt1::C0(h200, _, _, _, _, _, _, _, _, _, _) => {
                                h200
                            }
                        }, 1u64, 0u64, 2u64, 0u64, 3u64, f23(v11, v5.clone())?, 4u64, f10(match v5 {
                            Adt1::C0(h201, _, _, _, _, _, _, _, _, _, _) => {
                                h201
                            }
                        })))
                    }
                }
            }
        }
    }
}

pub fn f6(v0: Adt1, v1: Adt1) -> bool {
    let a204 = match v0.clone() {
        Adt1::C0(_, h202, _, _, _, _, _, _, _, _, _) => {
            h202
        }
    };
    let a205 = match v1.clone() {
        Adt1::C0(_, h203, _, _, _, _, _, _, _, _, _) => {
            h203
        }
    };
    if if if nat_eq(a204, a205) {
        let a208 = match v0.clone() {
            Adt1::C0(_, _, h206, _, _, _, _, _, _, _, _) => {
                h206
            }
        };
        let a209 = match v1.clone() {
            Adt1::C0(_, _, h207, _, _, _, _, _, _, _, _) => {
                h207
            }
        };
        nat_eq(a208, a209)
    } else {
        false
    } {
        let a212 = match v0.clone() {
            Adt1::C0(_, _, _, h210, _, _, _, _, _, _, _) => {
                h210
            }
        };
        let a213 = match v1.clone() {
            Adt1::C0(_, _, _, h211, _, _, _, _, _, _, _) => {
                h211
            }
        };
        nat_eq(a212, a213)
    } else {
        false
    } {
        let a216 = match v0 {
            Adt1::C0(_, _, _, _, h214, _, _, _, _, _, _) => {
                h214
            }
        };
        let a217 = match v1 {
            Adt1::C0(_, _, _, _, h215, _, _, _, _, _, _) => {
                h215
            }
        };
        nat_eq(a216, a217)
    } else {
        false
    }
}

pub fn f7(v0: Adt1, v1: Adt1) -> bool {
    if f9(v1.clone()) {
        f9(v0)
    } else {
        let a223 = {
            let a220 = match v0.clone() {
                Adt1::C0(_, h218, _, _, _, _, _, _, _, _, _) => {
                    h218
                }
            };
            let a221 = match v1.clone() {
                Adt1::C0(_, h219, _, _, _, _, _, _, _, _, _) => {
                    h219
                }
            };
            let a222 = 1u64;
            nat_rem(a220, a221, a222)
        };
        let a224 = 0u64;
        if if if nat_eq(a223, a224) {
            let a230 = {
                let a227 = match v0.clone() {
                    Adt1::C0(_, _, h225, _, _, _, _, _, _, _, _) => {
                        h225
                    }
                };
                let a228 = match v1.clone() {
                    Adt1::C0(_, _, h226, _, _, _, _, _, _, _, _) => {
                        h226
                    }
                };
                let a229 = 1u64;
                nat_rem(a227, a228, a229)
            };
            let a231 = 0u64;
            nat_eq(a230, a231)
        } else {
            false
        } {
            let a237 = {
                let a234 = match v0.clone() {
                    Adt1::C0(_, _, _, h232, _, _, _, _, _, _, _) => {
                        h232
                    }
                };
                let a235 = match v1.clone() {
                    Adt1::C0(_, _, _, h233, _, _, _, _, _, _, _) => {
                        h233
                    }
                };
                let a236 = 1u64;
                nat_rem(a234, a235, a236)
            };
            let a238 = 0u64;
            nat_eq(a237, a238)
        } else {
            false
        } {
            let a244 = {
                let a241 = match v0 {
                    Adt1::C0(_, _, _, _, h239, _, _, _, _, _, _) => {
                        h239
                    }
                };
                let a242 = match v1 {
                    Adt1::C0(_, _, _, _, h240, _, _, _, _, _, _) => {
                        h240
                    }
                };
                let a243 = 1u64;
                nat_rem(a241, a242, a243)
            };
            let a245 = 0u64;
            nat_eq(a244, a245)
        } else {
            false
        }
    }
}

pub fn f8(v0: u64, v1: u64, v2: u64) -> bool {
    let a246 = v0;
    let a247 = 0u64;
    if if if nat_eq(a246, a247) {
        let a248 = v1;
        let a249 = 0u64;
        nat_eq(a248, a249)
    } else {
        false
    } {
        let a250 = v2;
        let a251 = 0u64;
        nat_eq(a250, a251)
    } else {
        false
    } {
        true
    } else {
        let a252 = v0;
        let a253 = 1u64;
        if if nat_eq(a252, a253) {
            true
        } else {
            let a254 = v0;
            let a255 = 30u64;
            nat_eq(a254, a255)
        } {
            let a256 = v1;
            let a257 = v0;
            if if nat_eq(a256, a257) {
                let a258 = v2;
                let a259 = v0;
                nat_eq(a258, a259)
            } else {
                false
            } {
                true
            } else {
                let a260 = v1;
                let a261 = 0u64;
                if nat_eq(a260, a261) {
                    let a262 = v2;
                    let a263 = v0;
                    if nat_eq(a262, a263) {
                        true
                    } else {
                        let a264 = v2;
                        let a265 = 0u64;
                        nat_eq(a264, a265)
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

pub fn f9(v0: Adt1) -> bool {
    f25(match v0.clone() {
        Adt1::C0(_, h266, _, _, _, _, _, _, _, _, _) => {
            h266
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h267, _, _, _, _, _, _, _, _) => {
            h267
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h268, _, _, _, _, _, _, _) => {
            h268
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, h269, _, _, _, _, _, _) => {
            h269
        }
    })
}

pub fn f10(v0: u64) -> u64 {
    let a270 = v0;
    let a271 = 0u64;
    if nat_eq(a270, a271) {
        4u64
    } else {
        let a272 = v0;
        let a273 = 1u64;
        if if nat_eq(a272, a273) {
            true
        } else {
            let a274 = v0;
            let a275 = 30u64;
            nat_eq(a274, a275)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f11(v0: Adt1) -> bool {
    let v1: u64 = f10(match v0.clone() {
        Adt1::C0(h276, _, _, _, _, _, _, _, _, _, _) => {
            h276
        }
    });
    let a281 = {
        let a278 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h277) => {
                h277
            }
        };
        let a279 = v1;
        let a280 = 1u64;
        nat_rem(a278, a279, a280)
    };
    let a282 = 0u64;
    if if nat_eq(a281, a282) {
        f26(match v0.clone() {
            Adt1::C0(_, h283, _, _, _, _, _, _, _, _, _) => {
                h283
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h284, _, _, _, _, _) => {
                h284
            }
        }, v1)
    } else {
        false
    } {
        f27(match v0.clone() {
            Adt1::C0(_, _, h285, _, _, _, _, _, _, _, _) => {
                h285
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h286, _, _, _, _, _, _, _) => {
                h286
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h287, _, _, _, _, _, _) => {
                h287
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h288, _, _, _, _) => {
                h288
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h289, _, _, _) => {
                h289
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h290, _, _) => {
                h290
            }
        }, v1)
    } else {
        false
    }
}

pub fn f12(v0: Adt1, v1: Adt1) -> R<bool> {
    let a298 = {
        let a296 = match v0.clone() {
            Adt1::C0(_, _, h291, _, _, _, _, _, _, _, _) => {
                h291
            }
        };
        let a297 = {
            let a294 = match v0.clone() {
                Adt1::C0(_, _, _, h292, _, _, _, _, _, _, _) => {
                    h292
                }
            };
            let a295 = match v0.clone() {
                Adt1::C0(_, _, _, _, h293, _, _, _, _, _, _) => {
                    h293
                }
            };
            nat_mul(a294, a295)?
        };
        nat_mul(a296, a297)?
    };
    let a299 = 9223372036854775807u64;
    if nat_le(a298, a299) {
        if f20(v1.clone()) {
            let a301 = match v1 {
                Adt1::C0(_, h300, _, _, _, _, _, _, _, _, _) => {
                    h300
                }
            };
            let a302 = 2147483647u64;
            Ok(nat_le(a301, a302))
        } else {
            let a304 = match v0 {
                Adt1::C0(_, h303, _, _, _, _, _, _, _, _, _) => {
                    h303
                }
            };
            let a305 = 2147483647u64;
            Ok(nat_le(a304, a305))
        }
    } else {
        Ok(false)
    }
}

pub fn f13(v0: Adt1, v1: Adt1) -> R<bool> {
    if f28(v0.clone(), v1.clone())? {
        Ok(true)
    } else {
        Ok(f29(v0, v1))
    }
}

pub fn f14(v0: Adt1) -> R<bool> {
    f30({
        let a308 = match v0.clone() {
            Adt1::C0(_, h306, _, _, _, _, _, _, _, _, _) => {
                h306
            }
        };
        let a309 = f10(match v0.clone() {
            Adt1::C0(h307, _, _, _, _, _, _, _, _, _, _) => {
                h307
            }
        });
        nat_mul(a308, a309)?
    }, match v0.clone() {
        Adt1::C0(_, _, h310, _, _, _, _, _, _, _, _) => {
            h310
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h311, _, _, _, _, _, _, _) => {
            h311
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h312, _, _, _, _, _, _) => {
            h312
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h313, _, _, _, _) => {
            h313
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h314, _, _, _) => {
            h314
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h315, _, _) => {
            h315
        }
    })
}

pub fn f15(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a322 = {
        let a316 = v0;
        let a317 = v2;
        nat_mul(a316, a317)?
    };
    let a323 = {
        let a318 = v1;
        let a319 = 0u64;
        if nat_eq(a318, a319) {
            0u64
        } else {
            let a320 = v0;
            let a321 = 2u64;
            nat_mul(a320, a321)?
        }
    };
    nat_add(a322, a323)
}

pub fn f16(v0: Adt1) -> R<u64> {
    let a332 = {
        let a326 = match v0.clone() {
            Adt1::C0(_, h324, _, _, _, _, _, _, _, _, _) => {
                h324
            }
        };
        let a327 = match v0.clone() {
            Adt1::C0(_, _, h325, _, _, _, _, _, _, _, _) => {
                h325
            }
        };
        nat_mul(a326, a327)?
    };
    let a333 = {
        let a330 = match v0.clone() {
            Adt1::C0(_, _, _, h328, _, _, _, _, _, _, _) => {
                h328
            }
        };
        let a331 = match v0 {
            Adt1::C0(_, _, _, _, h329, _, _, _, _, _, _) => {
                h329
            }
        };
        nat_mul(a330, a331)?
    };
    nat_mul(a332, a333)
}

pub fn f17(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a344 = {
        let a340 = {
            let a336 = 1u64;
            let a337 = {
                let a334 = v0;
                let a335 = 0u64;
                if nat_eq(a334, a335) {
                    0u64
                } else {
                    1u64
                }
            };
            nat_add(a336, a337)?
        };
        let a341 = {
            let a338 = v1;
            let a339 = 0u64;
            if nat_eq(a338, a339) {
                0u64
            } else {
                1u64
            }
        };
        nat_add(a340, a341)?
    };
    let a345 = {
        let a342 = v2;
        let a343 = 0u64;
        if nat_eq(a342, a343) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a344, a345)
}

pub fn f18(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f6(v0.clone(), v1.clone()) {
        let a348 = match v0.clone() {
            Adt1::C0(h346, _, _, _, _, _, _, _, _, _, _) => {
                h346
            }
        };
        let a349 = match v1.clone() {
            Adt1::C0(h347, _, _, _, _, _, _, _, _, _, _) => {
                h347
            }
        };
        nat_eq(a348, a349)
    } else {
        false
    } {
        let a352 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h350, _, _, _, _, _) => {
                h350
            }
        };
        let a353 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h351, _, _, _, _, _) => {
                h351
            }
        };
        nat_eq(a352, a353)
    } else {
        false
    } {
        let a356 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h354, _, _, _, _) => {
                h354
            }
        };
        let a357 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h355, _, _, _, _) => {
                h355
            }
        };
        nat_eq(a356, a357)
    } else {
        false
    } {
        let a360 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h358, _, _, _) => {
                h358
            }
        };
        let a361 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h359, _, _, _) => {
                h359
            }
        };
        nat_eq(a360, a361)
    } else {
        false
    } {
        let a364 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h362, _, _) => {
                h362
            }
        };
        let a365 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h363, _, _) => {
                h363
            }
        };
        nat_eq(a364, a365)
    } else {
        false
    } {
        let a367 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h366) => {
                h366
            }
        };
        let a368 = 0u64;
        if if nat_eq(a367, a368) {
            true
        } else {
            let a370 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h369) => {
                    h369
                }
            };
            let a371 = 0u64;
            nat_eq(a370, a371)
        } {
            true
        } else {
            let a374 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h372) => {
                    h372
                }
            };
            let a375 = match v1 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h373) => {
                    h373
                }
            };
            nat_eq(a374, a375)
        }
    } else {
        false
    }
}

pub fn f19(v0: u64) -> u64 {
    let a376 = v0;
    let a377 = 26u64;
    if nat_eq(a376, a377) {
        4u64
    } else {
        let a378 = v0;
        let a379 = 27u64;
        if nat_eq(a378, a379) {
            8u64
        } else {
            0u64
        }
    }
}

pub fn f20(v0: Adt1) -> bool {
    let a381 = match v0.clone() {
        Adt1::C0(_, h380, _, _, _, _, _, _, _, _, _) => {
            h380
        }
    };
    let a382 = 1u64;
    if nat_eq(a381, a382) {
        true
    } else {
        let a385 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h383, _, _, _, _, _) => {
                h383
            }
        };
        let a386 = f10(match v0 {
            Adt1::C0(h384, _, _, _, _, _, _, _, _, _, _) => {
                h384
            }
        });
        nat_eq(a385, a386)
    }
}

pub fn f21(v0: u64, v1: Adt1, v2: u64, v3: u64) -> R<Adt3> {
    let v4: u64 = {
        let a387 = v0;
        let a388 = 2u64;
        let a389 = 0u64;
        nat_quot(a387, a388, a389)
    };
    let v5: u64 = f19(match v1.clone() {
        Adt1::C0(h390, _, _, _, _, _, _, _, _, _, _) => {
            h390
        }
    });
    let a394 = {
        let a391 = v0;
        let a392 = 2u64;
        let a393 = 0u64;
        nat_rem(a391, a392, a393)
    };
    let a395 = 0u64;
    if nat_eq(a394, a395) {
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 4u64, f23(v4, v1)?, 2u64, 0u64, 0u64, 2u64, v5, 8u64))
    } else {
        let a396 = v3;
        let a397 = v2;
        if if nat_lt(a396, a397) {
            let a400 = v3;
            let a401 = {
                let a398 = v5;
                let a399 = 4u64;
                if nat_eq(a398, a399) {
                    2147483648u64
                } else {
                    9223372036854775808u64
                }
            };
            nat_lt(a400, a401)
        } else {
            false
        } {
            Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, {
                let a402 = 3u64;
                let a403 = v4;
                nat_add(a402, a403)?
            }, 8u64, 8u64))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f22(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64) -> R<Adt3> {
    let v5: u64 = {
        let a405 = 1u64;
        let a406 = match v1.clone() {
            Adt1::C0(_, h404, _, _, _, _, _, _, _, _, _) => {
                h404
            }
        };
        nat_add(a405, a406)?
    };
    let v6: u64 = {
        let a407 = v0;
        let a408 = v5;
        let a409 = 0u64;
        nat_rem(a407, a408, a409)
    };
    let v7: u64 = {
        let a410 = v0;
        let a411 = v5;
        let a412 = 0u64;
        nat_quot(a410, a411, a412)
    };
    let v8: u64 = {
        let a414 = v7;
        let a415 = match v1.clone() {
            Adt1::C0(_, _, h413, _, _, _, _, _, _, _, _) => {
                h413
            }
        };
        let a416 = 0u64;
        nat_rem(a414, a415, a416)
    };
    let v9: u64 = {
        let a418 = v7;
        let a419 = match v1.clone() {
            Adt1::C0(_, _, h417, _, _, _, _, _, _, _, _) => {
                h417
            }
        };
        let a420 = 0u64;
        nat_quot(a418, a419, a420)
    };
    let v10: u64 = {
        let a422 = v9;
        let a423 = match v1.clone() {
            Adt1::C0(_, _, _, h421, _, _, _, _, _, _, _) => {
                h421
            }
        };
        let a424 = 0u64;
        nat_rem(a422, a423, a424)
    };
    let v11: u64 = {
        let a426 = v9;
        let a427 = match v1.clone() {
            Adt1::C0(_, _, _, h425, _, _, _, _, _, _, _) => {
                h425
            }
        };
        let a428 = 0u64;
        nat_quot(a426, a427, a428)
    };
    let a429 = v6;
    let a430 = 0u64;
    if nat_eq(a429, a430) {
        let v12: u64 = {
            let a447 = v8;
            let a448 = {
                let a445 = match v2.clone() {
                    Adt1::C0(_, h431, _, _, _, _, _, _, _, _, _) => {
                        h431
                    }
                };
                let a446 = {
                    let a443 = {
                        let a433 = v10;
                        let a434 = match v2.clone() {
                            Adt1::C0(_, _, h432, _, _, _, _, _, _, _, _) => {
                                h432
                            }
                        };
                        let a435 = 0u64;
                        nat_rem(a433, a434, a435)
                    };
                    let a444 = {
                        let a441 = match v2.clone() {
                            Adt1::C0(_, _, h436, _, _, _, _, _, _, _, _) => {
                                h436
                            }
                        };
                        let a442 = {
                            let a438 = v11;
                            let a439 = match v2 {
                                Adt1::C0(_, _, _, h437, _, _, _, _, _, _, _) => {
                                    h437
                                }
                            };
                            let a440 = 0u64;
                            nat_rem(a438, a439, a440)
                        };
                        nat_mul(a441, a442)?
                    };
                    nat_add(a443, a444)?
                };
                nat_mul(a445, a446)?
            };
            nat_add(a447, a448)?
        };
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, {
            let a449 = 3u64;
            let a450 = v12;
            nat_add(a449, a450)?
        }, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
    } else {
        let a452 = v4;
        let a453 = match v3.clone() {
            Adt1::C0(_, _, h451, _, _, _, _, _, _, _, _) => {
                h451
            }
        };
        if nat_lt(a452, a453) {
            let v13: u64 = {
                let a454 = v6;
                let a455 = 1u64;
                nat_sub(a454, a455)
            };
            let v14: u64 = f31(v13, v8, v10, v11, match v1.clone() {
                Adt1::C0(_, _, _, _, _, h456, _, _, _, _, _) => {
                    h456
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, h457, _, _, _, _) => {
                    h457
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h458, _, _, _) => {
                    h458
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h459, _, _) => {
                    h459
                }
            })?;
            let v15: u64 = f31(v13, v4, v10, v11, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h460, _, _, _, _, _) => {
                    h460
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h461, _, _, _, _) => {
                    h461
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h462, _, _, _) => {
                    h462
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h463, _, _) => {
                    h463
                }
            })?;
            Ok(Adt3::C3(0u64, 0u64, match v1.clone() {
                Adt1::C0(h464, _, _, _, _, _, _, _, _, _, _) => {
                    h464
                }
            }, match v3.clone() {
                Adt1::C0(h465, _, _, _, _, _, _, _, _, _, _) => {
                    h465
                }
            }, 0u64, v14, 2u64, 0u64, 3u64, v15, f10(match v1 {
                Adt1::C0(h466, _, _, _, _, _, _, _, _, _, _) => {
                    h466
                }
            }), f10(match v3 {
                Adt1::C0(h467, _, _, _, _, _, _, _, _, _, _) => {
                    h467
                }
            })))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f23(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a469 = v0;
        let a470 = match v1.clone() {
            Adt1::C0(_, h468, _, _, _, _, _, _, _, _, _) => {
                h468
            }
        };
        let a471 = 0u64;
        nat_quot(a469, a470, a471)
    };
    let v3: u64 = {
        let a473 = v2;
        let a474 = match v1.clone() {
            Adt1::C0(_, _, h472, _, _, _, _, _, _, _, _) => {
                h472
            }
        };
        let a475 = 0u64;
        nat_quot(a473, a474, a475)
    };
    f31({
        let a477 = v0;
        let a478 = match v1.clone() {
            Adt1::C0(_, h476, _, _, _, _, _, _, _, _, _) => {
                h476
            }
        };
        let a479 = 0u64;
        nat_rem(a477, a478, a479)
    }, {
        let a481 = v2;
        let a482 = match v1.clone() {
            Adt1::C0(_, _, h480, _, _, _, _, _, _, _, _) => {
                h480
            }
        };
        let a483 = 0u64;
        nat_rem(a481, a482, a483)
    }, {
        let a485 = v3;
        let a486 = match v1.clone() {
            Adt1::C0(_, _, _, h484, _, _, _, _, _, _, _) => {
                h484
            }
        };
        let a487 = 0u64;
        nat_rem(a485, a486, a487)
    }, {
        let a489 = v3;
        let a490 = match v1.clone() {
            Adt1::C0(_, _, _, h488, _, _, _, _, _, _, _) => {
                h488
            }
        };
        let a491 = 0u64;
        nat_quot(a489, a490, a491)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h492, _, _, _, _, _) => {
            h492
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h493, _, _, _, _) => {
            h493
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h494, _, _, _) => {
            h494
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h495, _, _) => {
            h495
        }
    })
}

pub fn f24(v0: u64, v1: Adt1, v2: Adt1) -> R<u64> {
    let v3: u64 = {
        let a497 = v0;
        let a498 = match v1.clone() {
            Adt1::C0(_, h496, _, _, _, _, _, _, _, _, _) => {
                h496
            }
        };
        let a499 = 0u64;
        nat_quot(a497, a498, a499)
    };
    let v4: u64 = {
        let a501 = v3;
        let a502 = match v1.clone() {
            Adt1::C0(_, _, h500, _, _, _, _, _, _, _, _) => {
                h500
            }
        };
        let a503 = 0u64;
        nat_quot(a501, a502, a503)
    };
    f31({
        let a509 = {
            let a505 = v0;
            let a506 = match v1.clone() {
                Adt1::C0(_, h504, _, _, _, _, _, _, _, _, _) => {
                    h504
                }
            };
            let a507 = 0u64;
            nat_rem(a505, a506, a507)
        };
        let a510 = match v2.clone() {
            Adt1::C0(_, h508, _, _, _, _, _, _, _, _, _) => {
                h508
            }
        };
        let a511 = 0u64;
        nat_rem(a509, a510, a511)
    }, {
        let a517 = {
            let a513 = v3;
            let a514 = match v1.clone() {
                Adt1::C0(_, _, h512, _, _, _, _, _, _, _, _) => {
                    h512
                }
            };
            let a515 = 0u64;
            nat_rem(a513, a514, a515)
        };
        let a518 = match v2.clone() {
            Adt1::C0(_, _, h516, _, _, _, _, _, _, _, _) => {
                h516
            }
        };
        let a519 = 0u64;
        nat_rem(a517, a518, a519)
    }, {
        let a525 = {
            let a521 = v4;
            let a522 = match v1.clone() {
                Adt1::C0(_, _, _, h520, _, _, _, _, _, _, _) => {
                    h520
                }
            };
            let a523 = 0u64;
            nat_rem(a521, a522, a523)
        };
        let a526 = match v2.clone() {
            Adt1::C0(_, _, _, h524, _, _, _, _, _, _, _) => {
                h524
            }
        };
        let a527 = 0u64;
        nat_rem(a525, a526, a527)
    }, {
        let a533 = {
            let a529 = v4;
            let a530 = match v1 {
                Adt1::C0(_, _, _, h528, _, _, _, _, _, _, _) => {
                    h528
                }
            };
            let a531 = 0u64;
            nat_quot(a529, a530, a531)
        };
        let a534 = match v2.clone() {
            Adt1::C0(_, _, _, _, h532, _, _, _, _, _, _) => {
                h532
            }
        };
        let a535 = 0u64;
        nat_rem(a533, a534, a535)
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h536, _, _, _, _, _) => {
            h536
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h537, _, _, _, _) => {
            h537
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h538, _, _, _) => {
            h538
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h539, _, _) => {
            h539
        }
    })
}

pub fn f25(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a540 = v0;
    let a541 = 0u64;
    if if if nat_eq(a540, a541) {
        true
    } else {
        let a542 = v1;
        let a543 = 0u64;
        nat_eq(a542, a543)
    } {
        true
    } else {
        let a544 = v2;
        let a545 = 0u64;
        nat_eq(a544, a545)
    } {
        true
    } else {
        let a546 = v3;
        let a547 = 0u64;
        nat_eq(a546, a547)
    }
}

pub fn f26(v0: u64, v1: u64, v2: u64) -> bool {
    let a548 = v0;
    let a549 = 1u64;
    if nat_le(a548, a549) {
        true
    } else {
        let a553 = {
            let a550 = v1;
            let a551 = v2;
            let a552 = 1u64;
            nat_rem(a550, a551, a552)
        };
        let a554 = 0u64;
        nat_eq(a553, a554)
    }
}

pub fn f27(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f26(v0, v3, v6) {
        f26(v1, v4, v6)
    } else {
        false
    } {
        f26(v2, v5, v6)
    } else {
        false
    }
}

pub fn f28(v0: Adt1, v1: Adt1) -> R<bool> {
    let a556 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h555, _) => {
            h555
        }
    };
    let a557 = 0u64;
    if nat_eq(a556, a557) {
        Ok(true)
    } else {
        let a559 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h558, _) => {
                h558
            }
        };
        let a560 = 0u64;
        if nat_eq(a559, a560) {
            Ok(true)
        } else {
            let a562 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h561) => {
                    h561
                }
            };
            let a563 = 0u64;
            if nat_eq(a562, a563) {
                Ok(true)
            } else {
                let a565 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h564) => {
                        h564
                    }
                };
                let a566 = 0u64;
                if nat_eq(a565, a566) {
                    Ok(true)
                } else {
                    let a572 = {
                        let a569 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h567) => {
                                h567
                            }
                        };
                        let a570 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h568, _) => {
                                h568
                            }
                        };
                        nat_add(a569, a570)?
                    };
                    let a573 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h571) => {
                            h571
                        }
                    };
                    if nat_le(a572, a573) {
                        Ok(true)
                    } else {
                        let a579 = {
                            let a576 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h574) => {
                                    h574
                                }
                            };
                            let a577 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h575, _) => {
                                    h575
                                }
                            };
                            nat_add(a576, a577)?
                        };
                        let a580 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h578) => {
                                h578
                            }
                        };
                        Ok(nat_le(a579, a580))
                    }
                }
            }
        }
    }
}

pub fn f29(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f6(v0.clone(), v1.clone()) {
        let a583 = match v0.clone() {
            Adt1::C0(h581, _, _, _, _, _, _, _, _, _, _) => {
                h581
            }
        };
        let a584 = match v1.clone() {
            Adt1::C0(h582, _, _, _, _, _, _, _, _, _, _) => {
                h582
            }
        };
        nat_eq(a583, a584)
    } else {
        false
    } {
        let a587 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h585) => {
                h585
            }
        };
        let a588 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h586) => {
                h586
            }
        };
        nat_eq(a587, a588)
    } else {
        false
    } {
        let a591 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h589, _, _, _, _, _) => {
                h589
            }
        };
        let a592 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h590, _, _, _, _, _) => {
                h590
            }
        };
        nat_eq(a591, a592)
    } else {
        false
    } {
        let a595 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h593, _, _, _, _) => {
                h593
            }
        };
        let a596 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h594, _, _, _, _) => {
                h594
            }
        };
        nat_eq(a595, a596)
    } else {
        false
    } {
        let a599 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h597, _, _, _) => {
                h597
            }
        };
        let a600 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h598, _, _, _) => {
                h598
            }
        };
        nat_eq(a599, a600)
    } else {
        false
    } {
        let a603 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h601, _, _) => {
                h601
            }
        };
        let a604 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h602, _, _) => {
                h602
            }
        };
        nat_eq(a603, a604)
    } else {
        false
    }
}

pub fn f30(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f32(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f32(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f32(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f32(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f32(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f32(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f31(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a617 = {
        let a613 = {
            let a609 = {
                let a605 = v0;
                let a606 = v4;
                nat_mul(a605, a606)?
            };
            let a610 = {
                let a607 = v1;
                let a608 = v5;
                nat_mul(a607, a608)?
            };
            nat_add(a609, a610)?
        };
        let a614 = {
            let a611 = v2;
            let a612 = v6;
            nat_mul(a611, a612)?
        };
        nat_add(a613, a614)?
    };
    let a618 = {
        let a615 = v3;
        let a616 = v7;
        nat_mul(a615, a616)?
    };
    nat_add(a617, a618)
}

pub fn f32(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a619 = v1;
    let a620 = 1u64;
    if if if nat_le(a619, a620) {
        true
    } else {
        let a621 = v0;
        let a622 = v2;
        nat_le(a621, a622)
    } {
        let a623 = v3;
        let a624 = 1u64;
        if nat_le(a623, a624) {
            true
        } else {
            let a631 = {
                let a629 = v0;
                let a630 = {
                    let a627 = {
                        let a625 = v1;
                        let a626 = 1u64;
                        nat_sub(a625, a626)
                    };
                    let a628 = v2;
                    nat_mul(a627, a628)?
                };
                nat_add(a629, a630)?
            };
            let a632 = v4;
            nat_le(a631, a632)
        }
    } else {
        false
    } {
        let a633 = v5;
        let a634 = 1u64;
        if nat_le(a633, a634) {
            Ok(true)
        } else {
            let a647 = {
                let a645 = {
                    let a639 = v0;
                    let a640 = {
                        let a637 = {
                            let a635 = v1;
                            let a636 = 1u64;
                            nat_sub(a635, a636)
                        };
                        let a638 = v2;
                        nat_mul(a637, a638)?
                    };
                    nat_add(a639, a640)?
                };
                let a646 = {
                    let a643 = {
                        let a641 = v3;
                        let a642 = 1u64;
                        nat_sub(a641, a642)
                    };
                    let a644 = v4;
                    nat_mul(a643, a644)?
                };
                nat_add(a645, a646)?
            };
            let a648 = v6;
            Ok(nat_le(a647, a648))
        }
    } else {
        Ok(false)
    }
}
