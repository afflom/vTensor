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
    C1(u64, Adt2, Adt2, bool),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64),
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
    C4(u64, u64, Adt2, Adt2, bool),
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
                let a3 = v1;
                let a4 = 19u64;
                if nat_eq(a3, a4) {
                    f3(v3, v5)?
                } else {
                    f4(v1, v2, v3, v4, v5)?
                }
            }))
        }
        Adt0::C1(v7, v8, v9, v10) => {
            Ok(Adt3::C1(if v10 {
                f5(v7, v8, v9.clone())?
            } else {
                f6(v7, v8)?
            }, f6(v7, v9)?))
        }
    }
}

pub fn f1(v0: u64) -> bool {
    let a5 = v0;
    let a6 = 1u64;
    if if nat_eq(a5, a6) {
        true
    } else {
        let a7 = v0;
        let a8 = 33u64;
        nat_eq(a7, a8)
    } {
        true
    } else {
        let a9 = v0;
        let a10 = 34u64;
        nat_eq(a9, a10)
    }
}

pub fn f2(v0: u64, v1: Adt1, v2: bool, v3: Adt1) -> R<Adt4> {
    let a13 = match v1.clone() {
        Adt1::C0(h11, _, _, _, _, _, _, _, _, _) => {
            h11
        }
    };
    let a14 = match v3.clone() {
        Adt1::C0(h12, _, _, _, _, _, _, _, _, _) => {
            h12
        }
    };
    if if nat_eq(a13, a14) {
        let a15 = v0;
        let a16 = 33u64;
        if nat_eq(a15, a16) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m18 = f7(match v1.clone() {
            Adt1::C0(h17, _, _, _, _, _, _, _, _, _) => {
                h17
            }
        });
        match m18 {
            Adt5::C0 => {
                Ok(Adt4::C0)
            }
            Adt5::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt6::C0(_, h19, _) => {
                        h19
                    }
                };
                let v6: u64 = f8(v1.clone(), v5)?;
                let a24 = {
                    let a21 = match v1.clone() {
                        Adt1::C0(_, h20, _, _, _, _, _, _, _, _) => {
                            h20
                        }
                    };
                    let a22 = v5;
                    let a23 = 1u64;
                    nat_rem(a21, a22, a23)
                };
                let a25 = 0u64;
                if if if nat_eq(a24, a25) {
                    let a30 = {
                        let a27 = match v3.clone() {
                            Adt1::C0(_, h26, _, _, _, _, _, _, _, _) => {
                                h26
                            }
                        };
                        let a28 = v5;
                        let a29 = 1u64;
                        nat_rem(a27, a28, a29)
                    };
                    let a31 = 0u64;
                    nat_eq(a30, a31)
                } else {
                    false
                } {
                    let a32 = v6;
                    let a33 = f8(v3.clone(), v5)?;
                    nat_eq(a32, a33)
                } else {
                    false
                } {
                    Ok(Adt4::C4(v6, match v4 {
                        Adt6::C0(_, _, h34) => {
                            h34
                        }
                    }, f9(v1, v5), f9(v3, v5), false))
                } else {
                    Ok(Adt4::C1)
                }
            }
        }
    } else {
        Ok(Adt4::C1)
    }
}

pub fn f3(v0: Adt1, v1: Adt1) -> R<Adt4> {
    let a36 = match v0.clone() {
        Adt1::C0(h35, _, _, _, _, _, _, _, _, _) => {
            h35
        }
    };
    let a37 = 0u64;
    if if if if nat_eq(a36, a37) {
        true
    } else {
        let a39 = match v0.clone() {
            Adt1::C0(h38, _, _, _, _, _, _, _, _, _) => {
                h38
            }
        };
        let a40 = 1u64;
        nat_eq(a39, a40)
    } {
        true
    } else {
        let a42 = match v0.clone() {
            Adt1::C0(h41, _, _, _, _, _, _, _, _, _) => {
                h41
            }
        };
        let a43 = 25u64;
        if nat_eq(a42, a43) {
            true
        } else {
            let a45 = match v0.clone() {
                Adt1::C0(h44, _, _, _, _, _, _, _, _, _) => {
                    h44
                }
            };
            let a46 = 26u64;
            nat_eq(a45, a46)
        }
    } {
        true
    } else {
        let a48 = match v0.clone() {
            Adt1::C0(h47, _, _, _, _, _, _, _, _, _) => {
                h47
            }
        };
        let a49 = 30u64;
        nat_eq(a48, a49)
    } {
        let a52 = match v0.clone() {
            Adt1::C0(h50, _, _, _, _, _, _, _, _, _) => {
                h50
            }
        };
        let a53 = match v1.clone() {
            Adt1::C0(h51, _, _, _, _, _, _, _, _, _) => {
                h51
            }
        };
        if if nat_eq(a52, a53) {
            f10(match v1.clone() {
                Adt1::C0(_, h54, _, _, _, _, _, _, _, _) => {
                    h54
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h55, _, _, _, _, _, _, _) => {
                    h55
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, h56, _, _, _, _, _, _) => {
                    h56
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, h57, _, _, _, _, _) => {
                    h57
                }
            }, match v0.clone() {
                Adt1::C0(_, h58, _, _, _, _, _, _, _, _) => {
                    h58
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h59, _, _, _, _, _, _, _) => {
                    h59
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h60, _, _, _, _, _, _) => {
                    h60
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h61, _, _, _, _, _) => {
                    h61
                }
            })
        } else {
            false
        } {
            let m63 = f7(match v0.clone() {
                Adt1::C0(h62, _, _, _, _, _, _, _, _, _) => {
                    h62
                }
            });
            match m63 {
                Adt5::C0 => {
                    Ok(Adt4::C0)
                }
                Adt5::C1(v2) => {
                    Ok(Adt4::C4(f8(v1.clone(), 1u64)?, match v2 {
                        Adt6::C0(_, _, h64) => {
                            h64
                        }
                    }, f9(v0, 1u64), f9(v1, 1u64), true))
                }
            }
        } else {
            Ok(Adt4::C1)
        }
    } else {
        Ok(Adt4::C0)
    }
}

pub fn f4(v0: u64, v1: u64, v2: Adt1, v3: Option<Adt1>, v4: Adt1) -> R<Adt4> {
    let m65 = v3;
    match m65 {
        None => {
            f11(v0, v1, match v2.clone() {
                Adt1::C0(h66, _, _, _, _, _, _, _, _, _) => {
                    h66
                }
            }, 0u64, match v4.clone() {
                Adt1::C0(h67, _, _, _, _, _, _, _, _, _) => {
                    h67
                }
            }, false, match v2.clone() {
                Adt1::C0(_, h68, _, _, _, _, _, _, _, _) => {
                    h68
                }
            }, match v2.clone() {
                Adt1::C0(_, _, h69, _, _, _, _, _, _, _) => {
                    h69
                }
            }, match v2.clone() {
                Adt1::C0(_, _, _, h70, _, _, _, _, _, _) => {
                    h70
                }
            }, match v2 {
                Adt1::C0(_, _, _, _, h71, _, _, _, _, _) => {
                    h71
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt1::C0(_, h72, _, _, _, _, _, _, _, _) => {
                    h72
                }
            }, match v4.clone() {
                Adt1::C0(_, _, h73, _, _, _, _, _, _, _) => {
                    h73
                }
            }, match v4.clone() {
                Adt1::C0(_, _, _, h74, _, _, _, _, _, _) => {
                    h74
                }
            }, match v4 {
                Adt1::C0(_, _, _, _, h75, _, _, _, _, _) => {
                    h75
                }
            })
        }
        Some(v5) => {
            f11(v0, v1, match v2.clone() {
                Adt1::C0(h76, _, _, _, _, _, _, _, _, _) => {
                    h76
                }
            }, match v5.clone() {
                Adt1::C0(h77, _, _, _, _, _, _, _, _, _) => {
                    h77
                }
            }, match v4.clone() {
                Adt1::C0(h78, _, _, _, _, _, _, _, _, _) => {
                    h78
                }
            }, true, match v2.clone() {
                Adt1::C0(_, h79, _, _, _, _, _, _, _, _) => {
                    h79
                }
            }, match v2.clone() {
                Adt1::C0(_, _, h80, _, _, _, _, _, _, _) => {
                    h80
                }
            }, match v2.clone() {
                Adt1::C0(_, _, _, h81, _, _, _, _, _, _) => {
                    h81
                }
            }, match v2 {
                Adt1::C0(_, _, _, _, h82, _, _, _, _, _) => {
                    h82
                }
            }, match v5.clone() {
                Adt1::C0(_, h83, _, _, _, _, _, _, _, _) => {
                    h83
                }
            }, match v5.clone() {
                Adt1::C0(_, _, h84, _, _, _, _, _, _, _) => {
                    h84
                }
            }, match v5.clone() {
                Adt1::C0(_, _, _, h85, _, _, _, _, _, _) => {
                    h85
                }
            }, match v5 {
                Adt1::C0(_, _, _, _, h86, _, _, _, _, _) => {
                    h86
                }
            }, match v4.clone() {
                Adt1::C0(_, h87, _, _, _, _, _, _, _, _) => {
                    h87
                }
            }, match v4.clone() {
                Adt1::C0(_, _, h88, _, _, _, _, _, _, _) => {
                    h88
                }
            }, match v4.clone() {
                Adt1::C0(_, _, _, h89, _, _, _, _, _, _) => {
                    h89
                }
            }, match v4 {
                Adt1::C0(_, _, _, _, h90, _, _, _, _, _) => {
                    h90
                }
            })
        }
    }
}

pub fn f5(v0: u64, v1: Adt2, v2: Adt2) -> R<u64> {
    let v3: u64 = {
        let a92 = v0;
        let a93 = match v2.clone() {
            Adt2::C0(h91, _, _, _, _, _, _, _) => {
                h91
            }
        };
        let a94 = 0u64;
        nat_quot(a92, a93, a94)
    };
    let v4: u64 = {
        let a96 = v3;
        let a97 = match v2.clone() {
            Adt2::C0(_, h95, _, _, _, _, _, _) => {
                h95
            }
        };
        let a98 = 0u64;
        nat_quot(a96, a97, a98)
    };
    f12({
        let a100 = v0;
        let a101 = match v2.clone() {
            Adt2::C0(h99, _, _, _, _, _, _, _) => {
                h99
            }
        };
        let a102 = 0u64;
        nat_rem(a100, a101, a102)
    }, {
        let a104 = v3;
        let a105 = match v2.clone() {
            Adt2::C0(_, h103, _, _, _, _, _, _) => {
                h103
            }
        };
        let a106 = 0u64;
        nat_rem(a104, a105, a106)
    }, {
        let a108 = v4;
        let a109 = match v2.clone() {
            Adt2::C0(_, _, h107, _, _, _, _, _) => {
                h107
            }
        };
        let a110 = 0u64;
        nat_rem(a108, a109, a110)
    }, {
        let a112 = v4;
        let a113 = match v2 {
            Adt2::C0(_, _, h111, _, _, _, _, _) => {
                h111
            }
        };
        let a114 = 0u64;
        nat_quot(a112, a113, a114)
    }, match v1.clone() {
        Adt2::C0(h115, _, _, _, _, _, _, _) => {
            h115
        }
    }, match v1.clone() {
        Adt2::C0(_, h116, _, _, _, _, _, _) => {
            h116
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h117, _, _, _, _, _) => {
            h117
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h118, _, _, _, _) => {
            h118
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h119, _, _, _) => {
            h119
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, _, h120, _, _) => {
            h120
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h121, _) => {
            h121
        }
    }, match v1 {
        Adt2::C0(_, _, _, _, _, _, _, h122) => {
            h122
        }
    })
}

pub fn f6(v0: u64, v1: Adt2) -> R<u64> {
    let v2: u64 = {
        let a124 = v0;
        let a125 = match v1.clone() {
            Adt2::C0(h123, _, _, _, _, _, _, _) => {
                h123
            }
        };
        let a126 = 0u64;
        nat_quot(a124, a125, a126)
    };
    let v3: u64 = {
        let a128 = v2;
        let a129 = match v1.clone() {
            Adt2::C0(_, h127, _, _, _, _, _, _) => {
                h127
            }
        };
        let a130 = 0u64;
        nat_quot(a128, a129, a130)
    };
    f13({
        let a132 = v0;
        let a133 = match v1.clone() {
            Adt2::C0(h131, _, _, _, _, _, _, _) => {
                h131
            }
        };
        let a134 = 0u64;
        nat_rem(a132, a133, a134)
    }, {
        let a136 = v2;
        let a137 = match v1.clone() {
            Adt2::C0(_, h135, _, _, _, _, _, _) => {
                h135
            }
        };
        let a138 = 0u64;
        nat_rem(a136, a137, a138)
    }, {
        let a140 = v3;
        let a141 = match v1.clone() {
            Adt2::C0(_, _, h139, _, _, _, _, _) => {
                h139
            }
        };
        let a142 = 0u64;
        nat_rem(a140, a141, a142)
    }, {
        let a144 = v3;
        let a145 = match v1.clone() {
            Adt2::C0(_, _, h143, _, _, _, _, _) => {
                h143
            }
        };
        let a146 = 0u64;
        nat_quot(a144, a145, a146)
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h147, _, _, _) => {
            h147
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, _, h148, _, _) => {
            h148
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h149, _) => {
            h149
        }
    }, match v1 {
        Adt2::C0(_, _, _, _, _, _, _, h150) => {
            h150
        }
    })
}

pub fn f7(v0: u64) -> Adt5 {
    let a151 = v0;
    let a152 = 0u64;
    if nat_eq(a151, a152) {
        Adt5::C1(Adt6::C0(0u64, 1u64, 4u64))
    } else {
        let a153 = v0;
        let a154 = 1u64;
        if nat_eq(a153, a154) {
            Adt5::C1(Adt6::C0(1u64, 1u64, 2u64))
        } else {
            let a155 = v0;
            let a156 = 2u64;
            if nat_eq(a155, a156) {
                Adt5::C1(Adt6::C0(2u64, 32u64, 18u64))
            } else {
                let a157 = v0;
                let a158 = 3u64;
                if nat_eq(a157, a158) {
                    Adt5::C1(Adt6::C0(3u64, 32u64, 20u64))
                } else {
                    let a159 = v0;
                    let a160 = 6u64;
                    if nat_eq(a159, a160) {
                        Adt5::C1(Adt6::C0(6u64, 32u64, 22u64))
                    } else {
                        let a161 = v0;
                        let a162 = 7u64;
                        if nat_eq(a161, a162) {
                            Adt5::C1(Adt6::C0(7u64, 32u64, 24u64))
                        } else {
                            let a163 = v0;
                            let a164 = 8u64;
                            if nat_eq(a163, a164) {
                                Adt5::C1(Adt6::C0(8u64, 32u64, 34u64))
                            } else {
                                let a165 = v0;
                                let a166 = 9u64;
                                if nat_eq(a165, a166) {
                                    Adt5::C1(Adt6::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a167 = v0;
                                    let a168 = 10u64;
                                    if nat_eq(a167, a168) {
                                        Adt5::C1(Adt6::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a169 = v0;
                                        let a170 = 11u64;
                                        if nat_eq(a169, a170) {
                                            Adt5::C1(Adt6::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a171 = v0;
                                            let a172 = 12u64;
                                            if nat_eq(a171, a172) {
                                                Adt5::C1(Adt6::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a173 = v0;
                                                let a174 = 13u64;
                                                if nat_eq(a173, a174) {
                                                    Adt5::C1(Adt6::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a175 = v0;
                                                    let a176 = 14u64;
                                                    if nat_eq(a175, a176) {
                                                        Adt5::C1(Adt6::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a177 = v0;
                                                        let a178 = 15u64;
                                                        if nat_eq(a177, a178) {
                                                            Adt5::C1(Adt6::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a179 = v0;
                                                            let a180 = 16u64;
                                                            if nat_eq(a179, a180) {
                                                                Adt5::C1(Adt6::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a181 = v0;
                                                                let a182 = 17u64;
                                                                if nat_eq(a181, a182) {
                                                                    Adt5::C1(Adt6::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a183 = v0;
                                                                    let a184 = 18u64;
                                                                    if nat_eq(a183, a184) {
                                                                        Adt5::C1(Adt6::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a185 = v0;
                                                                        let a186 = 19u64;
                                                                        if nat_eq(a185, a186) {
                                                                            Adt5::C1(Adt6::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a187 = v0;
                                                                            let a188 = 20u64;
                                                                            if nat_eq(a187, a188) {
                                                                                Adt5::C1(Adt6::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a189 = v0;
                                                                                let a190 = 21u64;
                                                                                if nat_eq(a189, a190) {
                                                                                    Adt5::C1(Adt6::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a191 = v0;
                                                                                    let a192 = 22u64;
                                                                                    if nat_eq(a191, a192) {
                                                                                        Adt5::C1(Adt6::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a193 = v0;
                                                                                        let a194 = 23u64;
                                                                                        if nat_eq(a193, a194) {
                                                                                            Adt5::C1(Adt6::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a195 = v0;
                                                                                            let a196 = 24u64;
                                                                                            if nat_eq(a195, a196) {
                                                                                                Adt5::C1(Adt6::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a197 = v0;
                                                                                                let a198 = 25u64;
                                                                                                if nat_eq(a197, a198) {
                                                                                                    Adt5::C1(Adt6::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a199 = v0;
                                                                                                    let a200 = 26u64;
                                                                                                    if nat_eq(a199, a200) {
                                                                                                        Adt5::C1(Adt6::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a201 = v0;
                                                                                                        let a202 = 27u64;
                                                                                                        if nat_eq(a201, a202) {
                                                                                                            Adt5::C1(Adt6::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a203 = v0;
                                                                                                            let a204 = 28u64;
                                                                                                            if nat_eq(a203, a204) {
                                                                                                                Adt5::C1(Adt6::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a205 = v0;
                                                                                                                let a206 = 29u64;
                                                                                                                if nat_eq(a205, a206) {
                                                                                                                    Adt5::C1(Adt6::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a207 = v0;
                                                                                                                    let a208 = 30u64;
                                                                                                                    if nat_eq(a207, a208) {
                                                                                                                        Adt5::C1(Adt6::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a209 = v0;
                                                                                                                        let a210 = 34u64;
                                                                                                                        if nat_eq(a209, a210) {
                                                                                                                            Adt5::C1(Adt6::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a211 = v0;
                                                                                                                            let a212 = 35u64;
                                                                                                                            if nat_eq(a211, a212) {
                                                                                                                                Adt5::C1(Adt6::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a213 = v0;
                                                                                                                                let a214 = 39u64;
                                                                                                                                if nat_eq(a213, a214) {
                                                                                                                                    Adt5::C1(Adt6::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a215 = v0;
                                                                                                                                    let a216 = 40u64;
                                                                                                                                    if nat_eq(a215, a216) {
                                                                                                                                        Adt5::C1(Adt6::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a217 = v0;
                                                                                                                                        let a218 = 41u64;
                                                                                                                                        if nat_eq(a217, a218) {
                                                                                                                                            Adt5::C1(Adt6::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a219 = v0;
                                                                                                                                            let a220 = 42u64;
                                                                                                                                            if nat_eq(a219, a220) {
                                                                                                                                                Adt5::C1(Adt6::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a221 = v0;
                                                                                                                                                let a222 = 43u64;
                                                                                                                                                if nat_eq(a221, a222) {
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

pub fn f8(v0: Adt1, v1: u64) -> R<u64> {
    let a234 = {
        let a228 = {
            let a224 = match v0.clone() {
                Adt1::C0(_, h223, _, _, _, _, _, _, _, _) => {
                    h223
                }
            };
            let a225 = v1;
            let a226 = 0u64;
            nat_quot(a224, a225, a226)
        };
        let a229 = match v0.clone() {
            Adt1::C0(_, _, h227, _, _, _, _, _, _, _) => {
                h227
            }
        };
        nat_mul(a228, a229)?
    };
    let a235 = {
        let a232 = match v0.clone() {
            Adt1::C0(_, _, _, h230, _, _, _, _, _, _) => {
                h230
            }
        };
        let a233 = match v0 {
            Adt1::C0(_, _, _, _, h231, _, _, _, _, _) => {
                h231
            }
        };
        nat_mul(a232, a233)?
    };
    nat_mul(a234, a235)
}

pub fn f9(v0: Adt1, v1: u64) -> Adt2 {
    Adt2::C0({
        let a237 = match v0.clone() {
            Adt1::C0(_, h236, _, _, _, _, _, _, _, _) => {
                h236
            }
        };
        let a238 = v1;
        let a239 = 0u64;
        nat_quot(a237, a238, a239)
    }, match v0.clone() {
        Adt1::C0(_, _, h240, _, _, _, _, _, _, _) => {
            h240
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h241, _, _, _, _, _, _) => {
            h241
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h242, _, _, _, _, _) => {
            h242
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h243, _, _, _, _) => {
            h243
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h244, _, _, _) => {
            h244
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h245, _, _) => {
            h245
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h246, _) => {
            h246
        }
    })
}

pub fn f10(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f14(v4, v5, v6, v7) {
        f14(v0, v1, v2, v3)
    } else {
        let a250 = {
            let a247 = v0;
            let a248 = v4;
            let a249 = 1u64;
            nat_rem(a247, a248, a249)
        };
        let a251 = 0u64;
        if if if nat_eq(a250, a251) {
            let a255 = {
                let a252 = v1;
                let a253 = v5;
                let a254 = 1u64;
                nat_rem(a252, a253, a254)
            };
            let a256 = 0u64;
            nat_eq(a255, a256)
        } else {
            false
        } {
            let a260 = {
                let a257 = v2;
                let a258 = v6;
                let a259 = 1u64;
                nat_rem(a257, a258, a259)
            };
            let a261 = 0u64;
            nat_eq(a260, a261)
        } else {
            false
        } {
            let a265 = {
                let a262 = v3;
                let a263 = v7;
                let a264 = 1u64;
                nat_rem(a262, a263, a264)
            };
            let a266 = 0u64;
            nat_eq(a265, a266)
        } else {
            false
        }
    }
}

pub fn f11(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt4> {
    let m267 = f15(v0, v1);
    match m267 {
        None => {
            if v5 {
                let m268 = f16(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m268 {
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
                let a269 = 4u64;
                let a270 = v28;
                nat_le(a269, a270)
            };
            let a271 = v2;
            let a272 = 0u64;
            if if if if if nat_eq(a271, a272) {
                let a273 = v4;
                let a274 = 0u64;
                nat_eq(a273, a274)
            } else {
                false
            } {
                let a275 = v6;
                let a276 = v14;
                if nat_eq(a275, a276) {
                    let a277 = v7;
                    let a278 = v15;
                    nat_eq(a277, a278)
                } else {
                    false
                }
            } else {
                false
            } {
                let a279 = v8;
                let a280 = v16;
                if nat_eq(a279, a280) {
                    let a281 = v9;
                    let a282 = v17;
                    nat_eq(a281, a282)
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
                        let a283 = v3;
                        let a284 = 0u64;
                        nat_eq(a283, a284)
                    } else {
                        false
                    } {
                        f10(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt4::C2(v28, v29, v6, v7, v8, v9, {
                    let a289 = {
                        let a285 = v6;
                        let a286 = v7;
                        nat_mul(a285, a286)?
                    };
                    let a290 = {
                        let a287 = v8;
                        let a288 = v9;
                        nat_mul(a287, a288)?
                    };
                    nat_mul(a289, a290)?
                }))
            } else {
                Ok(Adt4::C1)
            }
        }
    }
}

pub fn f12(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f13({
        let a291 = v0;
        let a292 = v4;
        let a293 = 0u64;
        nat_rem(a291, a292, a293)
    }, {
        let a294 = v1;
        let a295 = v5;
        let a296 = 0u64;
        nat_rem(a294, a295, a296)
    }, {
        let a297 = v2;
        let a298 = v6;
        let a299 = 0u64;
        nat_rem(a297, a298, a299)
    }, {
        let a300 = v3;
        let a301 = v7;
        let a302 = 0u64;
        nat_rem(a300, a301, a302)
    }, v8, v9, v10, v11)
}

pub fn f13(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a315 = {
        let a311 = {
            let a307 = {
                let a303 = v0;
                let a304 = v4;
                nat_mul(a303, a304)?
            };
            let a308 = {
                let a305 = v1;
                let a306 = v5;
                nat_mul(a305, a306)?
            };
            nat_add(a307, a308)?
        };
        let a312 = {
            let a309 = v2;
            let a310 = v6;
            nat_mul(a309, a310)?
        };
        nat_add(a311, a312)?
    };
    let a316 = {
        let a313 = v3;
        let a314 = v7;
        nat_mul(a313, a314)?
    };
    nat_add(a315, a316)
}

pub fn f14(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a317 = v0;
    let a318 = 0u64;
    if if if nat_eq(a317, a318) {
        true
    } else {
        let a319 = v1;
        let a320 = 0u64;
        nat_eq(a319, a320)
    } {
        true
    } else {
        let a321 = v2;
        let a322 = 0u64;
        nat_eq(a321, a322)
    } {
        true
    } else {
        let a323 = v3;
        let a324 = 0u64;
        nat_eq(a323, a324)
    }
}

pub fn f15(v0: u64, v1: u64) -> Option<u64> {
    let a325 = v0;
    let a326 = 2u64;
    if nat_eq(a325, a326) {
        Some(0u64)
    } else {
        let a327 = v0;
        let a328 = 5u64;
        if nat_eq(a327, a328) {
            Some(1u64)
        } else {
            let a329 = v0;
            let a330 = 6u64;
            if nat_eq(a329, a330) {
                Some(2u64)
            } else {
                let a331 = v0;
                let a332 = 7u64;
                if nat_eq(a331, a332) {
                    Some(3u64)
                } else {
                    let a333 = v0;
                    let a334 = 9u64;
                    if nat_eq(a333, a334) {
                        Some(4u64)
                    } else {
                        let a335 = v0;
                        let a336 = 8u64;
                        if nat_eq(a335, a336) {
                            Some(5u64)
                        } else {
                            let a337 = v0;
                            let a338 = 89u64;
                            if nat_eq(a337, a338) {
                                let a339 = v1;
                                let a340 = 2u64;
                                if nat_eq(a339, a340) {
                                    Some(6u64)
                                } else {
                                    let a341 = v1;
                                    let a342 = 0u64;
                                    if nat_eq(a341, a342) {
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

pub fn f16(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt7> {
    let a343 = v0;
    let a344 = 28u64;
    if if if if nat_eq(a343, a344) {
        let a345 = v1;
        let a346 = 0u64;
        nat_eq(a345, a346)
    } else {
        false
    } {
        let a347 = v2;
        let a348 = 0u64;
        nat_eq(a347, a348)
    } else {
        false
    } {
        let a349 = v3;
        let a350 = 0u64;
        nat_eq(a349, a350)
    } else {
        false
    } {
        let a351 = v4;
        let a352 = v8;
        if if if if if if if if nat_eq(a351, a352) {
            let a353 = v5;
            let a354 = v12;
            nat_eq(a353, a354)
        } else {
            false
        } {
            let a355 = v9;
            let a356 = v13;
            nat_eq(a355, a356)
        } else {
            false
        } {
            let a357 = v10;
            let a358 = v14;
            nat_eq(a357, a358)
        } else {
            false
        } {
            let a359 = v11;
            let a360 = v15;
            nat_eq(a359, a360)
        } else {
            false
        } {
            let a361 = 0u64;
            let a362 = v6;
            nat_lt(a361, a362)
        } else {
            false
        } {
            let a363 = 0u64;
            let a364 = v7;
            nat_lt(a363, a364)
        } else {
            false
        } {
            let a368 = {
                let a365 = v10;
                let a366 = v6;
                let a367 = 1u64;
                nat_rem(a365, a366, a367)
            };
            let a369 = 0u64;
            if nat_eq(a368, a369) {
                let a373 = {
                    let a370 = v11;
                    let a371 = v7;
                    let a372 = 1u64;
                    nat_rem(a370, a371, a372)
                };
                let a374 = 0u64;
                nat_eq(a373, a374)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt7::C2(v5, v4, v9, v10, v11, {
                let a375 = v10;
                let a376 = v6;
                let a377 = 0u64;
                nat_quot(a375, a376, a377)
            }, {
                let a378 = v11;
                let a379 = v7;
                let a380 = 0u64;
                nat_quot(a378, a379, a380)
            }, {
                let a381 = v5;
                let a382 = v4;
                nat_mul(a381, a382)?
            }, {
                let a383 = v4;
                let a384 = v9;
                nat_mul(a383, a384)?
            }, {
                let a385 = v5;
                let a386 = v9;
                nat_mul(a385, a386)?
            }))
        } else {
            Ok(Adt7::C1)
        }
    } else {
        Ok(Adt7::C0)
    }
}
