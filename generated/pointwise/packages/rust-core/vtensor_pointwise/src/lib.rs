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
    C0(u64, u64, Adt1, Option<Adt1>, Adt1),
    C1(u64, Adt2, Adt2),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(Adt4),
    C1(u64, u64),
}

#[derive(Clone)]
pub enum Adt4 {
    C0,
    C1,
    C2(u64, bool, u64, u64, u64, u64, u64),
    C3(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
    C4(u64, u64, Adt2, Adt2),
}

#[derive(Clone)]
pub enum Adt5 {
    C0,
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt7 {
    C0,
    C1,
    C2(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

pub fn f0(v0: Adt0) -> R<Adt3> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1, v2, v3, v4, v5) => {
            Ok(Adt3::C0(if f1(v1) {
                f2(v1, v3, {
                    let m2 = v4;
                    match m2 {
                        None => {
                            false
                        }
                        Some(_) => {
                            true
                        }
                    }
                }, v5)?
            } else {
                f3(v1, v2, v3, v4, v5)?
            }))
        }
        Adt0::C1(v7, v8, v9) => {
            Ok(Adt3::C1(f4(v7, v8)?, f4(v7, v9)?))
        }
    }
}

pub fn f1(v0: u64) -> bool {
    let a3 = v0;
    let a4 = 1u64;
    if if nat_eq(a3, a4) {
        true
    } else {
        let a5 = v0;
        let a6 = 33u64;
        nat_eq(a5, a6)
    } {
        true
    } else {
        let a7 = v0;
        let a8 = 34u64;
        nat_eq(a7, a8)
    }
}

pub fn f2(v0: u64, v1: Adt1, v2: bool, v3: Adt1) -> R<Adt4> {
    let a11 = match v1.clone() {
        Adt1::C0(h9, _, _, _, _, _, _, _, _, _) => {
            h9
        }
    };
    let a12 = match v3.clone() {
        Adt1::C0(h10, _, _, _, _, _, _, _, _, _) => {
            h10
        }
    };
    if if nat_eq(a11, a12) {
        let a13 = v0;
        let a14 = 33u64;
        if nat_eq(a13, a14) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m16 = f5(match v1.clone() {
            Adt1::C0(h15, _, _, _, _, _, _, _, _, _) => {
                h15
            }
        });
        match m16 {
            Adt5::C0 => {
                Ok(Adt4::C0)
            }
            Adt5::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt6::C0(_, h17, _) => {
                        h17
                    }
                };
                let v6: u64 = f6(v1.clone(), v5)?;
                let a22 = {
                    let a19 = match v1.clone() {
                        Adt1::C0(_, h18, _, _, _, _, _, _, _, _) => {
                            h18
                        }
                    };
                    let a20 = v5;
                    let a21 = 1u64;
                    nat_rem(a19, a20, a21)
                };
                let a23 = 0u64;
                if if if nat_eq(a22, a23) {
                    let a28 = {
                        let a25 = match v3.clone() {
                            Adt1::C0(_, h24, _, _, _, _, _, _, _, _) => {
                                h24
                            }
                        };
                        let a26 = v5;
                        let a27 = 1u64;
                        nat_rem(a25, a26, a27)
                    };
                    let a29 = 0u64;
                    nat_eq(a28, a29)
                } else {
                    false
                } {
                    let a30 = v6;
                    let a31 = f6(v3.clone(), v5)?;
                    nat_eq(a30, a31)
                } else {
                    false
                } {
                    Ok(Adt4::C4(v6, match v4 {
                        Adt6::C0(_, _, h32) => {
                            h32
                        }
                    }, f7(v1, v5), f7(v3, v5)))
                } else {
                    Ok(Adt4::C1)
                }
            }
        }
    } else {
        Ok(Adt4::C1)
    }
}

pub fn f3(v0: u64, v1: u64, v2: Adt1, v3: Option<Adt1>, v4: Adt1) -> R<Adt4> {
    let m33 = v3;
    match m33 {
        None => {
            f8(v0, v1, match v2.clone() {
                Adt1::C0(h34, _, _, _, _, _, _, _, _, _) => {
                    h34
                }
            }, 0u64, match v4.clone() {
                Adt1::C0(h35, _, _, _, _, _, _, _, _, _) => {
                    h35
                }
            }, false, match v2.clone() {
                Adt1::C0(_, h36, _, _, _, _, _, _, _, _) => {
                    h36
                }
            }, match v2.clone() {
                Adt1::C0(_, _, h37, _, _, _, _, _, _, _) => {
                    h37
                }
            }, match v2.clone() {
                Adt1::C0(_, _, _, h38, _, _, _, _, _, _) => {
                    h38
                }
            }, match v2 {
                Adt1::C0(_, _, _, _, h39, _, _, _, _, _) => {
                    h39
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt1::C0(_, h40, _, _, _, _, _, _, _, _) => {
                    h40
                }
            }, match v4.clone() {
                Adt1::C0(_, _, h41, _, _, _, _, _, _, _) => {
                    h41
                }
            }, match v4.clone() {
                Adt1::C0(_, _, _, h42, _, _, _, _, _, _) => {
                    h42
                }
            }, match v4 {
                Adt1::C0(_, _, _, _, h43, _, _, _, _, _) => {
                    h43
                }
            })
        }
        Some(v5) => {
            f8(v0, v1, match v2.clone() {
                Adt1::C0(h44, _, _, _, _, _, _, _, _, _) => {
                    h44
                }
            }, match v5.clone() {
                Adt1::C0(h45, _, _, _, _, _, _, _, _, _) => {
                    h45
                }
            }, match v4.clone() {
                Adt1::C0(h46, _, _, _, _, _, _, _, _, _) => {
                    h46
                }
            }, true, match v2.clone() {
                Adt1::C0(_, h47, _, _, _, _, _, _, _, _) => {
                    h47
                }
            }, match v2.clone() {
                Adt1::C0(_, _, h48, _, _, _, _, _, _, _) => {
                    h48
                }
            }, match v2.clone() {
                Adt1::C0(_, _, _, h49, _, _, _, _, _, _) => {
                    h49
                }
            }, match v2 {
                Adt1::C0(_, _, _, _, h50, _, _, _, _, _) => {
                    h50
                }
            }, match v5.clone() {
                Adt1::C0(_, h51, _, _, _, _, _, _, _, _) => {
                    h51
                }
            }, match v5.clone() {
                Adt1::C0(_, _, h52, _, _, _, _, _, _, _) => {
                    h52
                }
            }, match v5.clone() {
                Adt1::C0(_, _, _, h53, _, _, _, _, _, _) => {
                    h53
                }
            }, match v5 {
                Adt1::C0(_, _, _, _, h54, _, _, _, _, _) => {
                    h54
                }
            }, match v4.clone() {
                Adt1::C0(_, h55, _, _, _, _, _, _, _, _) => {
                    h55
                }
            }, match v4.clone() {
                Adt1::C0(_, _, h56, _, _, _, _, _, _, _) => {
                    h56
                }
            }, match v4.clone() {
                Adt1::C0(_, _, _, h57, _, _, _, _, _, _) => {
                    h57
                }
            }, match v4 {
                Adt1::C0(_, _, _, _, h58, _, _, _, _, _) => {
                    h58
                }
            })
        }
    }
}

pub fn f4(v0: u64, v1: Adt2) -> R<u64> {
    let v2: u64 = {
        let a60 = v0;
        let a61 = match v1.clone() {
            Adt2::C0(h59, _, _, _, _, _, _) => {
                h59
            }
        };
        let a62 = 0u64;
        nat_quot(a60, a61, a62)
    };
    let v3: u64 = {
        let a64 = v2;
        let a65 = match v1.clone() {
            Adt2::C0(_, h63, _, _, _, _, _) => {
                h63
            }
        };
        let a66 = 0u64;
        nat_quot(a64, a65, a66)
    };
    f9({
        let a68 = v0;
        let a69 = match v1.clone() {
            Adt2::C0(h67, _, _, _, _, _, _) => {
                h67
            }
        };
        let a70 = 0u64;
        nat_rem(a68, a69, a70)
    }, {
        let a72 = v2;
        let a73 = match v1.clone() {
            Adt2::C0(_, h71, _, _, _, _, _) => {
                h71
            }
        };
        let a74 = 0u64;
        nat_rem(a72, a73, a74)
    }, {
        let a76 = v3;
        let a77 = match v1.clone() {
            Adt2::C0(_, _, h75, _, _, _, _) => {
                h75
            }
        };
        let a78 = 0u64;
        nat_rem(a76, a77, a78)
    }, {
        let a80 = v3;
        let a81 = match v1.clone() {
            Adt2::C0(_, _, h79, _, _, _, _) => {
                h79
            }
        };
        let a82 = 0u64;
        nat_quot(a80, a81, a82)
    }, match v1.clone() {
        Adt2::C0(_, _, _, h83, _, _, _) => {
            h83
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h84, _, _) => {
            h84
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, _, h85, _) => {
            h85
        }
    }, match v1 {
        Adt2::C0(_, _, _, _, _, _, h86) => {
            h86
        }
    })
}

pub fn f5(v0: u64) -> Adt5 {
    let a87 = v0;
    let a88 = 0u64;
    if nat_eq(a87, a88) {
        Adt5::C1(Adt6::C0(0u64, 1u64, 4u64))
    } else {
        let a89 = v0;
        let a90 = 1u64;
        if nat_eq(a89, a90) {
            Adt5::C1(Adt6::C0(1u64, 1u64, 2u64))
        } else {
            let a91 = v0;
            let a92 = 2u64;
            if nat_eq(a91, a92) {
                Adt5::C1(Adt6::C0(2u64, 32u64, 18u64))
            } else {
                let a93 = v0;
                let a94 = 3u64;
                if nat_eq(a93, a94) {
                    Adt5::C1(Adt6::C0(3u64, 32u64, 20u64))
                } else {
                    let a95 = v0;
                    let a96 = 6u64;
                    if nat_eq(a95, a96) {
                        Adt5::C1(Adt6::C0(6u64, 32u64, 22u64))
                    } else {
                        let a97 = v0;
                        let a98 = 7u64;
                        if nat_eq(a97, a98) {
                            Adt5::C1(Adt6::C0(7u64, 32u64, 24u64))
                        } else {
                            let a99 = v0;
                            let a100 = 8u64;
                            if nat_eq(a99, a100) {
                                Adt5::C1(Adt6::C0(8u64, 32u64, 34u64))
                            } else {
                                let a101 = v0;
                                let a102 = 9u64;
                                if nat_eq(a101, a102) {
                                    Adt5::C1(Adt6::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a103 = v0;
                                    let a104 = 10u64;
                                    if nat_eq(a103, a104) {
                                        Adt5::C1(Adt6::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a105 = v0;
                                        let a106 = 11u64;
                                        if nat_eq(a105, a106) {
                                            Adt5::C1(Adt6::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a107 = v0;
                                            let a108 = 12u64;
                                            if nat_eq(a107, a108) {
                                                Adt5::C1(Adt6::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a109 = v0;
                                                let a110 = 13u64;
                                                if nat_eq(a109, a110) {
                                                    Adt5::C1(Adt6::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a111 = v0;
                                                    let a112 = 14u64;
                                                    if nat_eq(a111, a112) {
                                                        Adt5::C1(Adt6::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a113 = v0;
                                                        let a114 = 15u64;
                                                        if nat_eq(a113, a114) {
                                                            Adt5::C1(Adt6::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a115 = v0;
                                                            let a116 = 16u64;
                                                            if nat_eq(a115, a116) {
                                                                Adt5::C1(Adt6::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a117 = v0;
                                                                let a118 = 17u64;
                                                                if nat_eq(a117, a118) {
                                                                    Adt5::C1(Adt6::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a119 = v0;
                                                                    let a120 = 18u64;
                                                                    if nat_eq(a119, a120) {
                                                                        Adt5::C1(Adt6::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a121 = v0;
                                                                        let a122 = 19u64;
                                                                        if nat_eq(a121, a122) {
                                                                            Adt5::C1(Adt6::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a123 = v0;
                                                                            let a124 = 20u64;
                                                                            if nat_eq(a123, a124) {
                                                                                Adt5::C1(Adt6::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a125 = v0;
                                                                                let a126 = 21u64;
                                                                                if nat_eq(a125, a126) {
                                                                                    Adt5::C1(Adt6::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a127 = v0;
                                                                                    let a128 = 22u64;
                                                                                    if nat_eq(a127, a128) {
                                                                                        Adt5::C1(Adt6::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a129 = v0;
                                                                                        let a130 = 23u64;
                                                                                        if nat_eq(a129, a130) {
                                                                                            Adt5::C1(Adt6::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a131 = v0;
                                                                                            let a132 = 24u64;
                                                                                            if nat_eq(a131, a132) {
                                                                                                Adt5::C1(Adt6::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a133 = v0;
                                                                                                let a134 = 25u64;
                                                                                                if nat_eq(a133, a134) {
                                                                                                    Adt5::C1(Adt6::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a135 = v0;
                                                                                                    let a136 = 26u64;
                                                                                                    if nat_eq(a135, a136) {
                                                                                                        Adt5::C1(Adt6::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a137 = v0;
                                                                                                        let a138 = 27u64;
                                                                                                        if nat_eq(a137, a138) {
                                                                                                            Adt5::C1(Adt6::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a139 = v0;
                                                                                                            let a140 = 28u64;
                                                                                                            if nat_eq(a139, a140) {
                                                                                                                Adt5::C1(Adt6::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a141 = v0;
                                                                                                                let a142 = 29u64;
                                                                                                                if nat_eq(a141, a142) {
                                                                                                                    Adt5::C1(Adt6::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a143 = v0;
                                                                                                                    let a144 = 30u64;
                                                                                                                    if nat_eq(a143, a144) {
                                                                                                                        Adt5::C1(Adt6::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a145 = v0;
                                                                                                                        let a146 = 34u64;
                                                                                                                        if nat_eq(a145, a146) {
                                                                                                                            Adt5::C1(Adt6::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a147 = v0;
                                                                                                                            let a148 = 35u64;
                                                                                                                            if nat_eq(a147, a148) {
                                                                                                                                Adt5::C1(Adt6::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a149 = v0;
                                                                                                                                let a150 = 39u64;
                                                                                                                                if nat_eq(a149, a150) {
                                                                                                                                    Adt5::C1(Adt6::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a151 = v0;
                                                                                                                                    let a152 = 40u64;
                                                                                                                                    if nat_eq(a151, a152) {
                                                                                                                                        Adt5::C1(Adt6::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a153 = v0;
                                                                                                                                        let a154 = 41u64;
                                                                                                                                        if nat_eq(a153, a154) {
                                                                                                                                            Adt5::C1(Adt6::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a155 = v0;
                                                                                                                                            let a156 = 42u64;
                                                                                                                                            if nat_eq(a155, a156) {
                                                                                                                                                Adt5::C1(Adt6::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a157 = v0;
                                                                                                                                                let a158 = 43u64;
                                                                                                                                                if nat_eq(a157, a158) {
                                                                                                                                                    Adt5::C1(Adt6::C0(43u64, 1u64, 1u64))
                                                                                                                                                } else {
                                                                                                                                                    Adt5::C0
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

pub fn f6(v0: Adt1, v1: u64) -> R<u64> {
    let a170 = {
        let a164 = {
            let a160 = match v0.clone() {
                Adt1::C0(_, h159, _, _, _, _, _, _, _, _) => {
                    h159
                }
            };
            let a161 = v1;
            let a162 = 0u64;
            nat_quot(a160, a161, a162)
        };
        let a165 = match v0.clone() {
            Adt1::C0(_, _, h163, _, _, _, _, _, _, _) => {
                h163
            }
        };
        nat_mul(a164, a165)?
    };
    let a171 = {
        let a168 = match v0.clone() {
            Adt1::C0(_, _, _, h166, _, _, _, _, _, _) => {
                h166
            }
        };
        let a169 = match v0 {
            Adt1::C0(_, _, _, _, h167, _, _, _, _, _) => {
                h167
            }
        };
        nat_mul(a168, a169)?
    };
    nat_mul(a170, a171)
}

pub fn f7(v0: Adt1, v1: u64) -> Adt2 {
    Adt2::C0({
        let a173 = match v0.clone() {
            Adt1::C0(_, h172, _, _, _, _, _, _, _, _) => {
                h172
            }
        };
        let a174 = v1;
        let a175 = 0u64;
        nat_quot(a173, a174, a175)
    }, match v0.clone() {
        Adt1::C0(_, _, h176, _, _, _, _, _, _, _) => {
            h176
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h177, _, _, _, _, _, _) => {
            h177
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h178, _, _, _, _) => {
            h178
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h179, _, _, _) => {
            h179
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h180, _, _) => {
            h180
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h181, _) => {
            h181
        }
    })
}

pub fn f8(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt4> {
    let m182 = f10(v0, v1);
    match m182 {
        None => {
            if v5 {
                let m183 = f11(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m183 {
                    Adt7::C0 => {
                        Ok(Adt4::C0)
                    }
                    Adt7::C1 => {
                        Ok(Adt4::C1)
                    }
                    Adt7::C2(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27) => {
                        Ok(Adt4::C3(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27))
                    }
                }
            } else {
                Ok(Adt4::C1)
            }
        }
        Some(v28) => {
            let v29: bool = {
                let a184 = 4u64;
                let a185 = v28;
                nat_le(a184, a185)
            };
            let a186 = v2;
            let a187 = 0u64;
            if if if if if nat_eq(a186, a187) {
                let a188 = v4;
                let a189 = 0u64;
                nat_eq(a188, a189)
            } else {
                false
            } {
                let a190 = v6;
                let a191 = v14;
                if nat_eq(a190, a191) {
                    let a192 = v7;
                    let a193 = v15;
                    nat_eq(a192, a193)
                } else {
                    false
                }
            } else {
                false
            } {
                let a194 = v8;
                let a195 = v16;
                if nat_eq(a194, a195) {
                    let a196 = v9;
                    let a197 = v17;
                    nat_eq(a196, a197)
                } else {
                    false
                }
            } else {
                false
            } {
                if v29 {
                    true
                } else {
                    if if v5 {
                        let a198 = v3;
                        let a199 = 0u64;
                        nat_eq(a198, a199)
                    } else {
                        false
                    } {
                        f12(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt4::C2(v28, v29, v6, v7, v8, v9, {
                    let a204 = {
                        let a200 = v6;
                        let a201 = v7;
                        nat_mul(a200, a201)?
                    };
                    let a205 = {
                        let a202 = v8;
                        let a203 = v9;
                        nat_mul(a202, a203)?
                    };
                    nat_mul(a204, a205)?
                }))
            } else {
                Ok(Adt4::C1)
            }
        }
    }
}

pub fn f9(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a218 = {
        let a214 = {
            let a210 = {
                let a206 = v0;
                let a207 = v4;
                nat_mul(a206, a207)?
            };
            let a211 = {
                let a208 = v1;
                let a209 = v5;
                nat_mul(a208, a209)?
            };
            nat_add(a210, a211)?
        };
        let a215 = {
            let a212 = v2;
            let a213 = v6;
            nat_mul(a212, a213)?
        };
        nat_add(a214, a215)?
    };
    let a219 = {
        let a216 = v3;
        let a217 = v7;
        nat_mul(a216, a217)?
    };
    nat_add(a218, a219)
}

pub fn f10(v0: u64, v1: u64) -> Option<u64> {
    let a220 = v0;
    let a221 = 2u64;
    if nat_eq(a220, a221) {
        Some(0u64)
    } else {
        let a222 = v0;
        let a223 = 5u64;
        if nat_eq(a222, a223) {
            Some(1u64)
        } else {
            let a224 = v0;
            let a225 = 6u64;
            if nat_eq(a224, a225) {
                Some(2u64)
            } else {
                let a226 = v0;
                let a227 = 7u64;
                if nat_eq(a226, a227) {
                    Some(3u64)
                } else {
                    let a228 = v0;
                    let a229 = 9u64;
                    if nat_eq(a228, a229) {
                        Some(4u64)
                    } else {
                        let a230 = v0;
                        let a231 = 8u64;
                        if nat_eq(a230, a231) {
                            Some(5u64)
                        } else {
                            let a232 = v0;
                            let a233 = 89u64;
                            if nat_eq(a232, a233) {
                                let a234 = v1;
                                let a235 = 2u64;
                                if nat_eq(a234, a235) {
                                    Some(6u64)
                                } else {
                                    let a236 = v1;
                                    let a237 = 0u64;
                                    if nat_eq(a236, a237) {
                                        Some(7u64)
                                    } else {
                                        None::<u64>
                                    }
                                }
                            } else {
                                None::<u64>
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f11(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt7> {
    let a238 = v0;
    let a239 = 28u64;
    if if if if nat_eq(a238, a239) {
        let a240 = v1;
        let a241 = 0u64;
        nat_eq(a240, a241)
    } else {
        false
    } {
        let a242 = v2;
        let a243 = 0u64;
        nat_eq(a242, a243)
    } else {
        false
    } {
        let a244 = v3;
        let a245 = 0u64;
        nat_eq(a244, a245)
    } else {
        false
    } {
        let a246 = v4;
        let a247 = v8;
        if if if if if if if if nat_eq(a246, a247) {
            let a248 = v5;
            let a249 = v12;
            nat_eq(a248, a249)
        } else {
            false
        } {
            let a250 = v9;
            let a251 = v13;
            nat_eq(a250, a251)
        } else {
            false
        } {
            let a252 = v10;
            let a253 = v14;
            nat_eq(a252, a253)
        } else {
            false
        } {
            let a254 = v11;
            let a255 = v15;
            nat_eq(a254, a255)
        } else {
            false
        } {
            let a256 = 0u64;
            let a257 = v6;
            nat_lt(a256, a257)
        } else {
            false
        } {
            let a258 = 0u64;
            let a259 = v7;
            nat_lt(a258, a259)
        } else {
            false
        } {
            let a263 = {
                let a260 = v10;
                let a261 = v6;
                let a262 = 1u64;
                nat_rem(a260, a261, a262)
            };
            let a264 = 0u64;
            if nat_eq(a263, a264) {
                let a268 = {
                    let a265 = v11;
                    let a266 = v7;
                    let a267 = 1u64;
                    nat_rem(a265, a266, a267)
                };
                let a269 = 0u64;
                nat_eq(a268, a269)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt7::C2(v5, v4, v9, v10, v11, {
                let a270 = v10;
                let a271 = v6;
                let a272 = 0u64;
                nat_quot(a270, a271, a272)
            }, {
                let a273 = v11;
                let a274 = v7;
                let a275 = 0u64;
                nat_quot(a273, a274, a275)
            }, {
                let a276 = v5;
                let a277 = v4;
                nat_mul(a276, a277)?
            }, {
                let a278 = v4;
                let a279 = v9;
                nat_mul(a278, a279)?
            }, {
                let a280 = v5;
                let a281 = v9;
                nat_mul(a280, a281)?
            }))
        } else {
            Ok(Adt7::C1)
        }
    } else {
        Ok(Adt7::C0)
    }
}

pub fn f12(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f13(v4, v5, v6, v7) {
        f13(v0, v1, v2, v3)
    } else {
        let a285 = {
            let a282 = v0;
            let a283 = v4;
            let a284 = 1u64;
            nat_rem(a282, a283, a284)
        };
        let a286 = 0u64;
        if if if nat_eq(a285, a286) {
            let a290 = {
                let a287 = v1;
                let a288 = v5;
                let a289 = 1u64;
                nat_rem(a287, a288, a289)
            };
            let a291 = 0u64;
            nat_eq(a290, a291)
        } else {
            false
        } {
            let a295 = {
                let a292 = v2;
                let a293 = v6;
                let a294 = 1u64;
                nat_rem(a292, a293, a294)
            };
            let a296 = 0u64;
            nat_eq(a295, a296)
        } else {
            false
        } {
            let a300 = {
                let a297 = v3;
                let a298 = v7;
                let a299 = 1u64;
                nat_rem(a297, a298, a299)
            };
            let a301 = 0u64;
            nat_eq(a300, a301)
        } else {
            false
        }
    }
}

pub fn f13(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a302 = v0;
    let a303 = 0u64;
    if if if nat_eq(a302, a303) {
        true
    } else {
        let a304 = v1;
        let a305 = 0u64;
        nat_eq(a304, a305)
    } {
        true
    } else {
        let a306 = v2;
        let a307 = 0u64;
        nat_eq(a306, a307)
    } {
        true
    } else {
        let a308 = v3;
        let a309 = 0u64;
        nat_eq(a308, a309)
    }
}

pub fn run(p0: Adt0) -> R<Adt3> {
    f0(p0)
}
