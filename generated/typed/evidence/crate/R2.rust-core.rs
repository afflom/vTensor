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
        Adt0::C1(v8, v9, v10) => {
            f3(v8, v9, v10)
        }
    }
}

pub fn f1(v0: u64) -> Option<u64> {
    let a5 = v0;
    let a6 = 2u64;
    if nat_eq(a5, a6) {
        Some(0u64)
    } else {
        let a7 = v0;
        let a8 = 5u64;
        if nat_eq(a7, a8) {
            Some(1u64)
        } else {
            let a9 = v0;
            let a10 = 6u64;
            if nat_eq(a9, a10) {
                Some(2u64)
            } else {
                let a11 = v0;
                let a12 = 7u64;
                if nat_eq(a11, a12) {
                    Some(3u64)
                } else {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f2(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if if f4(v1.clone(), v3.clone()) {
        f5(v1.clone(), v2.clone())
    } else {
        false
    } {
        f6(match v1.clone() {
            Adt1::C0(h13, _, _, _, _, _, _, _, _, _, _) => {
                h13
            }
        }, match v2.clone() {
            Adt1::C0(h14, _, _, _, _, _, _, _, _, _, _) => {
                h14
            }
        }, match v3.clone() {
            Adt1::C0(h15, _, _, _, _, _, _, _, _, _, _) => {
                h15
            }
        })
    } else {
        false
    } {
        if f7(v1.clone()) {
            true
        } else {
            let a18 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, h16, _, _, _, _, _) => {
                    h16
                }
            };
            let a19 = f8(match v1.clone() {
                Adt1::C0(h17, _, _, _, _, _, _, _, _, _, _) => {
                    h17
                }
            });
            if if if if if if if nat_eq(a18, a19) {
                let a22 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h20, _, _, _, _, _) => {
                        h20
                    }
                };
                let a23 = f8(match v3.clone() {
                    Adt1::C0(h21, _, _, _, _, _, _, _, _, _, _) => {
                        h21
                    }
                });
                nat_eq(a22, a23)
            } else {
                false
            } {
                f9(v1.clone())
            } else {
                false
            } {
                f9(v2.clone())
            } else {
                false
            } {
                f9(v3.clone())
            } else {
                false
            } {
                f10(v1.clone(), v2.clone())?
            } else {
                false
            } {
                f11(v1.clone(), v3.clone())?
            } else {
                false
            } {
                f11(v2.clone(), v3.clone())?
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v4: u64 = if if f7(v1.clone()) {
            true
        } else {
            f12(v3.clone())?
        } {
            0u64
        } else {
            let a26 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h24, _) => {
                    h24
                }
            };
            let a27 = f8(match v3.clone() {
                Adt1::C0(h25, _, _, _, _, _, _, _, _, _, _) => {
                    h25
                }
            });
            let a28 = 0u64;
            nat_quot(a26, a27, a28)
        };
        Ok(Adt3::C2(Adt2::C0(f13(f14(v1.clone())?, v4, f15(match v1.clone() {
            Adt1::C0(h29, _, _, _, _, _, _, _, _, _, _) => {
                h29
            }
        }, match v2.clone() {
            Adt1::C0(h30, _, _, _, _, _, _, _, _, _, _) => {
                h30
            }
        }, match v3.clone() {
            Adt1::C0(h31, _, _, _, _, _, _, _, _, _, _) => {
                h31
            }
        })?)?, {
            let a32 = 3u64;
            let a33 = v4;
            nat_add(a32, a33)?
        }, v0, v1, v2, v3)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f3(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, h34, _, _) => {
            h34
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, h35, _) => {
            h35
        }
    };
    let v5: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, h36) => {
            h36
        }
    };
    let v6: u64 = {
        let a38 = match v1.clone() {
            Adt2::C0(_, h37, _, _, _, _) => {
                h37
            }
        };
        let a39 = 3u64;
        if nat_eq(a38, a39) {
            0u64
        } else {
            let a40 = f14(v3.clone())?;
            let a41 = 2u64;
            nat_mul(a40, a41)?
        }
    };
    let a43 = match v1.clone() {
        Adt2::C0(h42, _, _, _, _, _) => {
            h42
        }
    };
    let a44 = v0;
    if nat_le(a43, a44) {
        Ok(Adt3::C1)
    } else {
        let a45 = v0;
        let a46 = v6;
        if nat_lt(a45, a46) {
            let v7: u64 = {
                let a47 = v0;
                let a48 = 2u64;
                let a49 = 0u64;
                nat_quot(a47, a48, a49)
            };
            let v8: u64 = {
                let a54 = 3u64;
                let a55 = {
                    let a51 = f16(v7, v5.clone())?;
                    let a52 = f8(match v5 {
                        Adt1::C0(h50, _, _, _, _, _, _, _, _, _, _) => {
                            h50
                        }
                    });
                    let a53 = 0u64;
                    nat_quot(a51, a52, a53)
                };
                nat_add(a54, a55)?
            };
            let a59 = {
                let a56 = v0;
                let a57 = 2u64;
                let a58 = 0u64;
                nat_rem(a56, a57, a58)
            };
            let a60 = 0u64;
            if nat_eq(a59, a60) {
                Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, v8, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
            } else {
                let a61 = v2;
                let a62 = 0u64;
                if nat_eq(a61, a62) {
                    Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 2u64, 1u64, 2u64, 0u64, 0u64, v8, 8u64, 8u64))
                } else {
                    Ok(Adt3::C1)
                }
            }
        } else {
            let v9: u64 = {
                let a63 = v0;
                let a64 = v6;
                nat_sub(a63, a64)
            };
            let v10: u64 = f15(match v3.clone() {
                Adt1::C0(h65, _, _, _, _, _, _, _, _, _, _) => {
                    h65
                }
            }, match v4.clone() {
                Adt1::C0(h66, _, _, _, _, _, _, _, _, _, _) => {
                    h66
                }
            }, match v5.clone() {
                Adt1::C0(h67, _, _, _, _, _, _, _, _, _, _) => {
                    h67
                }
            })?;
            let v11: u64 = {
                let a68 = v9;
                let a69 = v10;
                let a70 = 0u64;
                nat_quot(a68, a69, a70)
            };
            let v12: u64 = {
                let a71 = v9;
                let a72 = v10;
                let a73 = 0u64;
                nat_rem(a71, a72, a73)
            };
            let v13: u64 = {
                let a75 = match v3.clone() {
                    Adt1::C0(h74, _, _, _, _, _, _, _, _, _, _) => {
                        h74
                    }
                };
                let a76 = 0u64;
                if nat_eq(a75, a76) {
                    0u64
                } else {
                    1u64
                }
            };
            let v14: u64 = {
                let a78 = match v4.clone() {
                    Adt1::C0(h77, _, _, _, _, _, _, _, _, _, _) => {
                        h77
                    }
                };
                let a79 = 0u64;
                if nat_eq(a78, a79) {
                    0u64
                } else {
                    1u64
                }
            };
            let a80 = v12;
            let a81 = v13;
            if nat_lt(a80, a81) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h82, _, _, _, _, _, _, _, _, _, _) => {
                        h82
                    }
                }, 0u64, 0u64, f16(v11, v3.clone())?, 2u64, 0u64, 0u64, 0u64, f8(match v3 {
                    Adt1::C0(h83, _, _, _, _, _, _, _, _, _, _) => {
                        h83
                    }
                }), 4u64))
            } else {
                let a86 = v12;
                let a87 = {
                    let a84 = v13;
                    let a85 = v14;
                    nat_add(a84, a85)?
                };
                if nat_lt(a86, a87) {
                    Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                        Adt1::C0(h88, _, _, _, _, _, _, _, _, _, _) => {
                            h88
                        }
                    }, 0u64, 4u64, f17(v11, v3, v4.clone())?, 2u64, 0u64, 0u64, 1u64, f8(match v4 {
                        Adt1::C0(h89, _, _, _, _, _, _, _, _, _, _) => {
                            h89
                        }
                    }), 4u64))
                } else {
                    let a92 = v12;
                    let a93 = {
                        let a90 = v13;
                        let a91 = v14;
                        nat_add(a90, a91)?
                    };
                    if nat_eq(a92, a93) {
                        Ok(Adt3::C3(1u64, match v1 {
                            Adt2::C0(_, _, h94, _, _, _) => {
                                h94
                            }
                        }, 0u64, 0u64, {
                            let a95 = v13;
                            let a96 = 0u64;
                            if nat_eq(a95, a96) {
                                0u64
                            } else {
                                1u64
                            }
                        }, {
                            let a97 = v13;
                            let a98 = 0u64;
                            if nat_eq(a97, a98) {
                                f16(v11, v3.clone())?
                            } else {
                                0u64
                            }
                        }, {
                            let a99 = v14;
                            let a100 = 0u64;
                            if nat_eq(a99, a100) {
                                4u64
                            } else {
                                1u64
                            }
                        }, {
                            let a101 = v14;
                            let a102 = 0u64;
                            if nat_eq(a101, a102) {
                                f17(v11, v3, v4)?
                            } else {
                                1u64
                            }
                        }, {
                            let a104 = match v5.clone() {
                                Adt1::C0(h103, _, _, _, _, _, _, _, _, _, _) => {
                                    h103
                                }
                            };
                            let a105 = 0u64;
                            if nat_eq(a104, a105) {
                                3u64
                            } else {
                                0u64
                            }
                        }, {
                            let a107 = match v5.clone() {
                                Adt1::C0(h106, _, _, _, _, _, _, _, _, _, _) => {
                                    h106
                                }
                            };
                            let a108 = 0u64;
                            if nat_eq(a107, a108) {
                                f16(v11, v5)?
                            } else {
                                0u64
                            }
                        }, 4u64, 4u64))
                    } else {
                        Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                            Adt1::C0(h109, _, _, _, _, _, _, _, _, _, _) => {
                                h109
                            }
                        }, 1u64, 0u64, 2u64, 0u64, 3u64, f16(v11, v5.clone())?, 4u64, f8(match v5 {
                            Adt1::C0(h110, _, _, _, _, _, _, _, _, _, _) => {
                                h110
                            }
                        })))
                    }
                }
            }
        }
    }
}

pub fn f4(v0: Adt1, v1: Adt1) -> bool {
    let a113 = match v0.clone() {
        Adt1::C0(_, h111, _, _, _, _, _, _, _, _, _) => {
            h111
        }
    };
    let a114 = match v1.clone() {
        Adt1::C0(_, h112, _, _, _, _, _, _, _, _, _) => {
            h112
        }
    };
    if if if nat_eq(a113, a114) {
        let a117 = match v0.clone() {
            Adt1::C0(_, _, h115, _, _, _, _, _, _, _, _) => {
                h115
            }
        };
        let a118 = match v1.clone() {
            Adt1::C0(_, _, h116, _, _, _, _, _, _, _, _) => {
                h116
            }
        };
        nat_eq(a117, a118)
    } else {
        false
    } {
        let a121 = match v0.clone() {
            Adt1::C0(_, _, _, h119, _, _, _, _, _, _, _) => {
                h119
            }
        };
        let a122 = match v1.clone() {
            Adt1::C0(_, _, _, h120, _, _, _, _, _, _, _) => {
                h120
            }
        };
        nat_eq(a121, a122)
    } else {
        false
    } {
        let a125 = match v0 {
            Adt1::C0(_, _, _, _, h123, _, _, _, _, _, _) => {
                h123
            }
        };
        let a126 = match v1 {
            Adt1::C0(_, _, _, _, h124, _, _, _, _, _, _) => {
                h124
            }
        };
        nat_eq(a125, a126)
    } else {
        false
    }
}

pub fn f5(v0: Adt1, v1: Adt1) -> bool {
    if f7(v1.clone()) {
        f7(v0)
    } else {
        let a132 = {
            let a129 = match v0.clone() {
                Adt1::C0(_, h127, _, _, _, _, _, _, _, _, _) => {
                    h127
                }
            };
            let a130 = match v1.clone() {
                Adt1::C0(_, h128, _, _, _, _, _, _, _, _, _) => {
                    h128
                }
            };
            let a131 = 1u64;
            nat_rem(a129, a130, a131)
        };
        let a133 = 0u64;
        if if if nat_eq(a132, a133) {
            let a139 = {
                let a136 = match v0.clone() {
                    Adt1::C0(_, _, h134, _, _, _, _, _, _, _, _) => {
                        h134
                    }
                };
                let a137 = match v1.clone() {
                    Adt1::C0(_, _, h135, _, _, _, _, _, _, _, _) => {
                        h135
                    }
                };
                let a138 = 1u64;
                nat_rem(a136, a137, a138)
            };
            let a140 = 0u64;
            nat_eq(a139, a140)
        } else {
            false
        } {
            let a146 = {
                let a143 = match v0.clone() {
                    Adt1::C0(_, _, _, h141, _, _, _, _, _, _, _) => {
                        h141
                    }
                };
                let a144 = match v1.clone() {
                    Adt1::C0(_, _, _, h142, _, _, _, _, _, _, _) => {
                        h142
                    }
                };
                let a145 = 1u64;
                nat_rem(a143, a144, a145)
            };
            let a147 = 0u64;
            nat_eq(a146, a147)
        } else {
            false
        } {
            let a153 = {
                let a150 = match v0 {
                    Adt1::C0(_, _, _, _, h148, _, _, _, _, _, _) => {
                        h148
                    }
                };
                let a151 = match v1 {
                    Adt1::C0(_, _, _, _, h149, _, _, _, _, _, _) => {
                        h149
                    }
                };
                let a152 = 1u64;
                nat_rem(a150, a151, a152)
            };
            let a154 = 0u64;
            nat_eq(a153, a154)
        } else {
            false
        }
    }
}

pub fn f6(v0: u64, v1: u64, v2: u64) -> bool {
    let a155 = v0;
    let a156 = 0u64;
    if if if nat_eq(a155, a156) {
        let a157 = v1;
        let a158 = 0u64;
        nat_eq(a157, a158)
    } else {
        false
    } {
        let a159 = v2;
        let a160 = 0u64;
        nat_eq(a159, a160)
    } else {
        false
    } {
        true
    } else {
        let a161 = v0;
        let a162 = 1u64;
        if if nat_eq(a161, a162) {
            true
        } else {
            let a163 = v0;
            let a164 = 30u64;
            nat_eq(a163, a164)
        } {
            let a165 = v1;
            let a166 = v0;
            if if nat_eq(a165, a166) {
                let a167 = v2;
                let a168 = v0;
                nat_eq(a167, a168)
            } else {
                false
            } {
                true
            } else {
                let a169 = v1;
                let a170 = 0u64;
                if nat_eq(a169, a170) {
                    let a171 = v2;
                    let a172 = v0;
                    if nat_eq(a171, a172) {
                        true
                    } else {
                        let a173 = v2;
                        let a174 = 0u64;
                        nat_eq(a173, a174)
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

pub fn f7(v0: Adt1) -> bool {
    f18(match v0.clone() {
        Adt1::C0(_, h175, _, _, _, _, _, _, _, _, _) => {
            h175
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h176, _, _, _, _, _, _, _, _) => {
            h176
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h177, _, _, _, _, _, _, _) => {
            h177
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, h178, _, _, _, _, _, _) => {
            h178
        }
    })
}

pub fn f8(v0: u64) -> u64 {
    let a179 = v0;
    let a180 = 0u64;
    if nat_eq(a179, a180) {
        4u64
    } else {
        let a181 = v0;
        let a182 = 1u64;
        if if nat_eq(a181, a182) {
            true
        } else {
            let a183 = v0;
            let a184 = 30u64;
            nat_eq(a183, a184)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f9(v0: Adt1) -> bool {
    let v1: u64 = f8(match v0.clone() {
        Adt1::C0(h185, _, _, _, _, _, _, _, _, _, _) => {
            h185
        }
    });
    let a190 = {
        let a187 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h186) => {
                h186
            }
        };
        let a188 = v1;
        let a189 = 1u64;
        nat_rem(a187, a188, a189)
    };
    let a191 = 0u64;
    if if nat_eq(a190, a191) {
        f19(match v0.clone() {
            Adt1::C0(_, h192, _, _, _, _, _, _, _, _, _) => {
                h192
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h193, _, _, _, _, _) => {
                h193
            }
        }, v1)
    } else {
        false
    } {
        f20(match v0.clone() {
            Adt1::C0(_, _, h194, _, _, _, _, _, _, _, _) => {
                h194
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h195, _, _, _, _, _, _, _) => {
                h195
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h196, _, _, _, _, _, _) => {
                h196
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h197, _, _, _, _) => {
                h197
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h198, _, _, _) => {
                h198
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h199, _, _) => {
                h199
            }
        }, v1)
    } else {
        false
    }
}

pub fn f10(v0: Adt1, v1: Adt1) -> R<bool> {
    let a207 = {
        let a205 = match v0.clone() {
            Adt1::C0(_, _, h200, _, _, _, _, _, _, _, _) => {
                h200
            }
        };
        let a206 = {
            let a203 = match v0.clone() {
                Adt1::C0(_, _, _, h201, _, _, _, _, _, _, _) => {
                    h201
                }
            };
            let a204 = match v0.clone() {
                Adt1::C0(_, _, _, _, h202, _, _, _, _, _, _) => {
                    h202
                }
            };
            nat_mul(a203, a204)?
        };
        nat_mul(a205, a206)?
    };
    let a208 = 9223372036854775807u64;
    if nat_le(a207, a208) {
        if f21(v1.clone()) {
            let a210 = match v1 {
                Adt1::C0(_, h209, _, _, _, _, _, _, _, _, _) => {
                    h209
                }
            };
            let a211 = 2147483647u64;
            Ok(nat_le(a210, a211))
        } else {
            let a213 = match v0 {
                Adt1::C0(_, h212, _, _, _, _, _, _, _, _, _) => {
                    h212
                }
            };
            let a214 = 2147483647u64;
            Ok(nat_le(a213, a214))
        }
    } else {
        Ok(false)
    }
}

pub fn f11(v0: Adt1, v1: Adt1) -> R<bool> {
    if f22(v0.clone(), v1.clone())? {
        Ok(true)
    } else {
        Ok(f23(v0, v1))
    }
}

pub fn f12(v0: Adt1) -> R<bool> {
    f24({
        let a217 = match v0.clone() {
            Adt1::C0(_, h215, _, _, _, _, _, _, _, _, _) => {
                h215
            }
        };
        let a218 = f8(match v0.clone() {
            Adt1::C0(h216, _, _, _, _, _, _, _, _, _, _) => {
                h216
            }
        });
        nat_mul(a217, a218)?
    }, match v0.clone() {
        Adt1::C0(_, _, h219, _, _, _, _, _, _, _, _) => {
            h219
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h220, _, _, _, _, _, _, _) => {
            h220
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h221, _, _, _, _, _, _) => {
            h221
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h222, _, _, _, _) => {
            h222
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h223, _, _, _) => {
            h223
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h224, _, _) => {
            h224
        }
    })
}

pub fn f13(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a231 = {
        let a225 = v0;
        let a226 = v2;
        nat_mul(a225, a226)?
    };
    let a232 = {
        let a227 = v1;
        let a228 = 0u64;
        if nat_eq(a227, a228) {
            0u64
        } else {
            let a229 = v0;
            let a230 = 2u64;
            nat_mul(a229, a230)?
        }
    };
    nat_add(a231, a232)
}

pub fn f14(v0: Adt1) -> R<u64> {
    let a241 = {
        let a235 = match v0.clone() {
            Adt1::C0(_, h233, _, _, _, _, _, _, _, _, _) => {
                h233
            }
        };
        let a236 = match v0.clone() {
            Adt1::C0(_, _, h234, _, _, _, _, _, _, _, _) => {
                h234
            }
        };
        nat_mul(a235, a236)?
    };
    let a242 = {
        let a239 = match v0.clone() {
            Adt1::C0(_, _, _, h237, _, _, _, _, _, _, _) => {
                h237
            }
        };
        let a240 = match v0 {
            Adt1::C0(_, _, _, _, h238, _, _, _, _, _, _) => {
                h238
            }
        };
        nat_mul(a239, a240)?
    };
    nat_mul(a241, a242)
}

pub fn f15(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a253 = {
        let a249 = {
            let a245 = 1u64;
            let a246 = {
                let a243 = v0;
                let a244 = 0u64;
                if nat_eq(a243, a244) {
                    0u64
                } else {
                    1u64
                }
            };
            nat_add(a245, a246)?
        };
        let a250 = {
            let a247 = v1;
            let a248 = 0u64;
            if nat_eq(a247, a248) {
                0u64
            } else {
                1u64
            }
        };
        nat_add(a249, a250)?
    };
    let a254 = {
        let a251 = v2;
        let a252 = 0u64;
        if nat_eq(a251, a252) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a253, a254)
}

pub fn f16(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a256 = v0;
        let a257 = match v1.clone() {
            Adt1::C0(_, h255, _, _, _, _, _, _, _, _, _) => {
                h255
            }
        };
        let a258 = 0u64;
        nat_quot(a256, a257, a258)
    };
    let v3: u64 = {
        let a260 = v2;
        let a261 = match v1.clone() {
            Adt1::C0(_, _, h259, _, _, _, _, _, _, _, _) => {
                h259
            }
        };
        let a262 = 0u64;
        nat_quot(a260, a261, a262)
    };
    f25({
        let a264 = v0;
        let a265 = match v1.clone() {
            Adt1::C0(_, h263, _, _, _, _, _, _, _, _, _) => {
                h263
            }
        };
        let a266 = 0u64;
        nat_rem(a264, a265, a266)
    }, {
        let a268 = v2;
        let a269 = match v1.clone() {
            Adt1::C0(_, _, h267, _, _, _, _, _, _, _, _) => {
                h267
            }
        };
        let a270 = 0u64;
        nat_rem(a268, a269, a270)
    }, {
        let a272 = v3;
        let a273 = match v1.clone() {
            Adt1::C0(_, _, _, h271, _, _, _, _, _, _, _) => {
                h271
            }
        };
        let a274 = 0u64;
        nat_rem(a272, a273, a274)
    }, {
        let a276 = v3;
        let a277 = match v1.clone() {
            Adt1::C0(_, _, _, h275, _, _, _, _, _, _, _) => {
                h275
            }
        };
        let a278 = 0u64;
        nat_quot(a276, a277, a278)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h279, _, _, _, _, _) => {
            h279
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h280, _, _, _, _) => {
            h280
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h281, _, _, _) => {
            h281
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h282, _, _) => {
            h282
        }
    })
}

pub fn f17(v0: u64, v1: Adt1, v2: Adt1) -> R<u64> {
    let v3: u64 = {
        let a284 = v0;
        let a285 = match v1.clone() {
            Adt1::C0(_, h283, _, _, _, _, _, _, _, _, _) => {
                h283
            }
        };
        let a286 = 0u64;
        nat_quot(a284, a285, a286)
    };
    let v4: u64 = {
        let a288 = v3;
        let a289 = match v1.clone() {
            Adt1::C0(_, _, h287, _, _, _, _, _, _, _, _) => {
                h287
            }
        };
        let a290 = 0u64;
        nat_quot(a288, a289, a290)
    };
    f25({
        let a296 = {
            let a292 = v0;
            let a293 = match v1.clone() {
                Adt1::C0(_, h291, _, _, _, _, _, _, _, _, _) => {
                    h291
                }
            };
            let a294 = 0u64;
            nat_rem(a292, a293, a294)
        };
        let a297 = match v2.clone() {
            Adt1::C0(_, h295, _, _, _, _, _, _, _, _, _) => {
                h295
            }
        };
        let a298 = 0u64;
        nat_rem(a296, a297, a298)
    }, {
        let a304 = {
            let a300 = v3;
            let a301 = match v1.clone() {
                Adt1::C0(_, _, h299, _, _, _, _, _, _, _, _) => {
                    h299
                }
            };
            let a302 = 0u64;
            nat_rem(a300, a301, a302)
        };
        let a305 = match v2.clone() {
            Adt1::C0(_, _, h303, _, _, _, _, _, _, _, _) => {
                h303
            }
        };
        let a306 = 0u64;
        nat_rem(a304, a305, a306)
    }, {
        let a312 = {
            let a308 = v4;
            let a309 = match v1.clone() {
                Adt1::C0(_, _, _, h307, _, _, _, _, _, _, _) => {
                    h307
                }
            };
            let a310 = 0u64;
            nat_rem(a308, a309, a310)
        };
        let a313 = match v2.clone() {
            Adt1::C0(_, _, _, h311, _, _, _, _, _, _, _) => {
                h311
            }
        };
        let a314 = 0u64;
        nat_rem(a312, a313, a314)
    }, {
        let a320 = {
            let a316 = v4;
            let a317 = match v1 {
                Adt1::C0(_, _, _, h315, _, _, _, _, _, _, _) => {
                    h315
                }
            };
            let a318 = 0u64;
            nat_quot(a316, a317, a318)
        };
        let a321 = match v2.clone() {
            Adt1::C0(_, _, _, _, h319, _, _, _, _, _, _) => {
                h319
            }
        };
        let a322 = 0u64;
        nat_rem(a320, a321, a322)
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h323, _, _, _, _, _) => {
            h323
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h324, _, _, _, _) => {
            h324
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h325, _, _, _) => {
            h325
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h326, _, _) => {
            h326
        }
    })
}

pub fn f18(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a327 = v0;
    let a328 = 0u64;
    if if if nat_eq(a327, a328) {
        true
    } else {
        let a329 = v1;
        let a330 = 0u64;
        nat_eq(a329, a330)
    } {
        true
    } else {
        let a331 = v2;
        let a332 = 0u64;
        nat_eq(a331, a332)
    } {
        true
    } else {
        let a333 = v3;
        let a334 = 0u64;
        nat_eq(a333, a334)
    }
}

pub fn f19(v0: u64, v1: u64, v2: u64) -> bool {
    let a335 = v0;
    let a336 = 1u64;
    if nat_le(a335, a336) {
        true
    } else {
        let a340 = {
            let a337 = v1;
            let a338 = v2;
            let a339 = 1u64;
            nat_rem(a337, a338, a339)
        };
        let a341 = 0u64;
        nat_eq(a340, a341)
    }
}

pub fn f20(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f19(v0, v3, v6) {
        f19(v1, v4, v6)
    } else {
        false
    } {
        f19(v2, v5, v6)
    } else {
        false
    }
}

pub fn f21(v0: Adt1) -> bool {
    let a343 = match v0.clone() {
        Adt1::C0(_, h342, _, _, _, _, _, _, _, _, _) => {
            h342
        }
    };
    let a344 = 1u64;
    if nat_eq(a343, a344) {
        true
    } else {
        let a347 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h345, _, _, _, _, _) => {
                h345
            }
        };
        let a348 = f8(match v0 {
            Adt1::C0(h346, _, _, _, _, _, _, _, _, _, _) => {
                h346
            }
        });
        nat_eq(a347, a348)
    }
}

pub fn f22(v0: Adt1, v1: Adt1) -> R<bool> {
    let a350 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h349, _) => {
            h349
        }
    };
    let a351 = 0u64;
    if nat_eq(a350, a351) {
        Ok(true)
    } else {
        let a353 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h352, _) => {
                h352
            }
        };
        let a354 = 0u64;
        if nat_eq(a353, a354) {
            Ok(true)
        } else {
            let a356 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h355) => {
                    h355
                }
            };
            let a357 = 0u64;
            if nat_eq(a356, a357) {
                Ok(true)
            } else {
                let a359 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h358) => {
                        h358
                    }
                };
                let a360 = 0u64;
                if nat_eq(a359, a360) {
                    Ok(true)
                } else {
                    let a366 = {
                        let a363 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h361) => {
                                h361
                            }
                        };
                        let a364 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h362, _) => {
                                h362
                            }
                        };
                        nat_add(a363, a364)?
                    };
                    let a367 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h365) => {
                            h365
                        }
                    };
                    if nat_le(a366, a367) {
                        Ok(true)
                    } else {
                        let a373 = {
                            let a370 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h368) => {
                                    h368
                                }
                            };
                            let a371 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h369, _) => {
                                    h369
                                }
                            };
                            nat_add(a370, a371)?
                        };
                        let a374 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h372) => {
                                h372
                            }
                        };
                        Ok(nat_le(a373, a374))
                    }
                }
            }
        }
    }
}

pub fn f23(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f4(v0.clone(), v1.clone()) {
        let a377 = match v0.clone() {
            Adt1::C0(h375, _, _, _, _, _, _, _, _, _, _) => {
                h375
            }
        };
        let a378 = match v1.clone() {
            Adt1::C0(h376, _, _, _, _, _, _, _, _, _, _) => {
                h376
            }
        };
        nat_eq(a377, a378)
    } else {
        false
    } {
        let a381 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h379) => {
                h379
            }
        };
        let a382 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h380) => {
                h380
            }
        };
        nat_eq(a381, a382)
    } else {
        false
    } {
        let a385 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h383, _, _, _, _, _) => {
                h383
            }
        };
        let a386 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h384, _, _, _, _, _) => {
                h384
            }
        };
        nat_eq(a385, a386)
    } else {
        false
    } {
        let a389 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h387, _, _, _, _) => {
                h387
            }
        };
        let a390 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h388, _, _, _, _) => {
                h388
            }
        };
        nat_eq(a389, a390)
    } else {
        false
    } {
        let a393 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h391, _, _, _) => {
                h391
            }
        };
        let a394 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h392, _, _, _) => {
                h392
            }
        };
        nat_eq(a393, a394)
    } else {
        false
    } {
        let a397 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h395, _, _) => {
                h395
            }
        };
        let a398 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h396, _, _) => {
                h396
            }
        };
        nat_eq(a397, a398)
    } else {
        false
    }
}

pub fn f24(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f26(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f26(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f26(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f26(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f26(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f26(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f25(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a411 = {
        let a407 = {
            let a403 = {
                let a399 = v0;
                let a400 = v4;
                nat_mul(a399, a400)?
            };
            let a404 = {
                let a401 = v1;
                let a402 = v5;
                nat_mul(a401, a402)?
            };
            nat_add(a403, a404)?
        };
        let a408 = {
            let a405 = v2;
            let a406 = v6;
            nat_mul(a405, a406)?
        };
        nat_add(a407, a408)?
    };
    let a412 = {
        let a409 = v3;
        let a410 = v7;
        nat_mul(a409, a410)?
    };
    nat_add(a411, a412)
}

pub fn f26(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a413 = v1;
    let a414 = 1u64;
    if if if nat_le(a413, a414) {
        true
    } else {
        let a415 = v0;
        let a416 = v2;
        nat_le(a415, a416)
    } {
        let a417 = v3;
        let a418 = 1u64;
        if nat_le(a417, a418) {
            true
        } else {
            let a425 = {
                let a423 = v0;
                let a424 = {
                    let a421 = {
                        let a419 = v1;
                        let a420 = 1u64;
                        nat_sub(a419, a420)
                    };
                    let a422 = v2;
                    nat_mul(a421, a422)?
                };
                nat_add(a423, a424)?
            };
            let a426 = v4;
            nat_le(a425, a426)
        }
    } else {
        false
    } {
        let a427 = v5;
        let a428 = 1u64;
        if nat_le(a427, a428) {
            Ok(true)
        } else {
            let a441 = {
                let a439 = {
                    let a433 = v0;
                    let a434 = {
                        let a431 = {
                            let a429 = v1;
                            let a430 = 1u64;
                            nat_sub(a429, a430)
                        };
                        let a432 = v2;
                        nat_mul(a431, a432)?
                    };
                    nat_add(a433, a434)?
                };
                let a440 = {
                    let a437 = {
                        let a435 = v3;
                        let a436 = 1u64;
                        nat_sub(a435, a436)
                    };
                    let a438 = v4;
                    nat_mul(a437, a438)?
                };
                nat_add(a439, a440)?
            };
            let a442 = v6;
            Ok(nat_le(a441, a442))
        }
    } else {
        Ok(false)
    }
}
