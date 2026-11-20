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
    C1(u64, Adt2, Adt2, Adt3),
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
    C0,
    C1,
    C2(u64, Adt2),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(Adt5),
    C1(bool, u64, u64),
}

#[derive(Clone)]
pub enum Adt5 {
    C0,
    C1,
    C2(u64, bool, u64, u64, u64, u64, u64),
    C3(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
    C4(u64, u64, Adt2, Adt2, Adt3),
}

#[derive(Clone)]
pub enum Adt6 {
    C0,
    C1(Adt7),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt8 {
    C0,
    C1,
    C2(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

pub fn f0(v0: Adt0) -> R<Adt4> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1, v2, v3, v4, v5) => {
            Ok(Adt4::C0(if f1(v1) {
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
                    let a5 = v1;
                    let a6 = 21u64;
                    if nat_eq(a5, a6) {
                        f4(v2, v3, v4, v5)?
                    } else {
                        f5(v1, v2, v3, v4, v5)?
                    }
                }
            }))
        }
        Adt0::C1(v7, v8, v9, v10) => {
            f6(v7, v8, v9, v10)
        }
    }
}

pub fn f1(v0: u64) -> bool {
    let a7 = v0;
    let a8 = 1u64;
    if if nat_eq(a7, a8) {
        true
    } else {
        let a9 = v0;
        let a10 = 33u64;
        nat_eq(a9, a10)
    } {
        true
    } else {
        let a11 = v0;
        let a12 = 34u64;
        nat_eq(a11, a12)
    }
}

pub fn f2(v0: u64, v1: Adt1, v2: bool, v3: Adt1) -> R<Adt5> {
    let a15 = match v1.clone() {
        Adt1::C0(h13, _, _, _, _, _, _, _, _, _) => {
            h13
        }
    };
    let a16 = match v3.clone() {
        Adt1::C0(h14, _, _, _, _, _, _, _, _, _) => {
            h14
        }
    };
    if if nat_eq(a15, a16) {
        let a17 = v0;
        let a18 = 33u64;
        if nat_eq(a17, a18) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m20 = f7(match v1.clone() {
            Adt1::C0(h19, _, _, _, _, _, _, _, _, _) => {
                h19
            }
        });
        match m20 {
            Adt6::C0 => {
                Ok(Adt5::C0)
            }
            Adt6::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt7::C0(_, h21, _) => {
                        h21
                    }
                };
                let v6: u64 = f8(v1.clone(), v5)?;
                let a26 = {
                    let a23 = match v1.clone() {
                        Adt1::C0(_, h22, _, _, _, _, _, _, _, _) => {
                            h22
                        }
                    };
                    let a24 = v5;
                    let a25 = 1u64;
                    nat_rem(a23, a24, a25)
                };
                let a27 = 0u64;
                if if if nat_eq(a26, a27) {
                    let a32 = {
                        let a29 = match v3.clone() {
                            Adt1::C0(_, h28, _, _, _, _, _, _, _, _) => {
                                h28
                            }
                        };
                        let a30 = v5;
                        let a31 = 1u64;
                        nat_rem(a29, a30, a31)
                    };
                    let a33 = 0u64;
                    nat_eq(a32, a33)
                } else {
                    false
                } {
                    let a34 = v6;
                    let a35 = f8(v3.clone(), v5)?;
                    nat_eq(a34, a35)
                } else {
                    false
                } {
                    Ok(Adt5::C4(v6, match v4 {
                        Adt7::C0(_, _, h36) => {
                            h36
                        }
                    }, f9(v1, v5), f9(v3, v5), Adt3::C0))
                } else {
                    Ok(Adt5::C1)
                }
            }
        }
    } else {
        Ok(Adt5::C1)
    }
}

pub fn f3(v0: Adt1, v1: Adt1) -> R<Adt5> {
    let a38 = match v0.clone() {
        Adt1::C0(h37, _, _, _, _, _, _, _, _, _) => {
            h37
        }
    };
    let a39 = 0u64;
    if if if if nat_eq(a38, a39) {
        true
    } else {
        let a41 = match v0.clone() {
            Adt1::C0(h40, _, _, _, _, _, _, _, _, _) => {
                h40
            }
        };
        let a42 = 1u64;
        nat_eq(a41, a42)
    } {
        true
    } else {
        let a44 = match v0.clone() {
            Adt1::C0(h43, _, _, _, _, _, _, _, _, _) => {
                h43
            }
        };
        let a45 = 25u64;
        if nat_eq(a44, a45) {
            true
        } else {
            let a47 = match v0.clone() {
                Adt1::C0(h46, _, _, _, _, _, _, _, _, _) => {
                    h46
                }
            };
            let a48 = 26u64;
            nat_eq(a47, a48)
        }
    } {
        true
    } else {
        let a50 = match v0.clone() {
            Adt1::C0(h49, _, _, _, _, _, _, _, _, _) => {
                h49
            }
        };
        let a51 = 30u64;
        nat_eq(a50, a51)
    } {
        let a54 = match v0.clone() {
            Adt1::C0(h52, _, _, _, _, _, _, _, _, _) => {
                h52
            }
        };
        let a55 = match v1.clone() {
            Adt1::C0(h53, _, _, _, _, _, _, _, _, _) => {
                h53
            }
        };
        if if nat_eq(a54, a55) {
            f10(match v1.clone() {
                Adt1::C0(_, h56, _, _, _, _, _, _, _, _) => {
                    h56
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h57, _, _, _, _, _, _, _) => {
                    h57
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, h58, _, _, _, _, _, _) => {
                    h58
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, h59, _, _, _, _, _) => {
                    h59
                }
            }, match v0.clone() {
                Adt1::C0(_, h60, _, _, _, _, _, _, _, _) => {
                    h60
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h61, _, _, _, _, _, _, _) => {
                    h61
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h62, _, _, _, _, _, _) => {
                    h62
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h63, _, _, _, _, _) => {
                    h63
                }
            })
        } else {
            false
        } {
            let m65 = f7(match v0.clone() {
                Adt1::C0(h64, _, _, _, _, _, _, _, _, _) => {
                    h64
                }
            });
            match m65 {
                Adt6::C0 => {
                    Ok(Adt5::C0)
                }
                Adt6::C1(v2) => {
                    Ok(Adt5::C4(f8(v1.clone(), 1u64)?, match v2 {
                        Adt7::C0(_, _, h66) => {
                            h66
                        }
                    }, f9(v0, 1u64), f9(v1, 1u64), Adt3::C1))
                }
            }
        } else {
            Ok(Adt5::C1)
        }
    } else {
        Ok(Adt5::C0)
    }
}

pub fn f4(v0: u64, v1: Adt1, v2: Option<Adt1>, v3: Adt1) -> R<Adt5> {
    let m67 = v2;
    match m67 {
        None => {
            Ok(Adt5::C1)
        }
        Some(v4) => {
            let a68 = v0;
            let a69 = 4u64;
            if if if nat_lt(a68, a69) {
                let a72 = match v1.clone() {
                    Adt1::C0(h70, _, _, _, _, _, _, _, _, _) => {
                        h70
                    }
                };
                let a73 = match v4.clone() {
                    Adt1::C0(h71, _, _, _, _, _, _, _, _, _) => {
                        h71
                    }
                };
                if nat_eq(a72, a73) {
                    let a76 = match v1.clone() {
                        Adt1::C0(h74, _, _, _, _, _, _, _, _, _) => {
                            h74
                        }
                    };
                    let a77 = match v3.clone() {
                        Adt1::C0(h75, _, _, _, _, _, _, _, _, _) => {
                            h75
                        }
                    };
                    nat_eq(a76, a77)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f11(v0, 0u64, match v1.clone() {
                    Adt1::C0(_, h78, _, _, _, _, _, _, _, _) => {
                        h78
                    }
                }, match v4.clone() {
                    Adt1::C0(_, h79, _, _, _, _, _, _, _, _) => {
                        h79
                    }
                }, match v3.clone() {
                    Adt1::C0(_, h80, _, _, _, _, _, _, _, _) => {
                        h80
                    }
                })? {
                    f11(v0, 1u64, match v1.clone() {
                        Adt1::C0(_, _, h81, _, _, _, _, _, _, _) => {
                            h81
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, _, h82, _, _, _, _, _, _, _) => {
                            h82
                        }
                    }, match v3.clone() {
                        Adt1::C0(_, _, h83, _, _, _, _, _, _, _) => {
                            h83
                        }
                    })?
                } else {
                    false
                } {
                    f11(v0, 2u64, match v1.clone() {
                        Adt1::C0(_, _, _, h84, _, _, _, _, _, _) => {
                            h84
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, _, _, h85, _, _, _, _, _, _) => {
                            h85
                        }
                    }, match v3.clone() {
                        Adt1::C0(_, _, _, h86, _, _, _, _, _, _) => {
                            h86
                        }
                    })?
                } else {
                    false
                } {
                    f11(v0, 3u64, match v1.clone() {
                        Adt1::C0(_, _, _, _, h87, _, _, _, _, _) => {
                            h87
                        }
                    }, match v4.clone() {
                        Adt1::C0(_, _, _, _, h88, _, _, _, _, _) => {
                            h88
                        }
                    }, match v3.clone() {
                        Adt1::C0(_, _, _, _, h89, _, _, _, _, _) => {
                            h89
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m91 = f7(match v1.clone() {
                    Adt1::C0(h90, _, _, _, _, _, _, _, _, _) => {
                        h90
                    }
                });
                match m91 {
                    Adt6::C0 => {
                        Ok(Adt5::C0)
                    }
                    Adt6::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt7::C0(_, h92, _) => {
                                h92
                            }
                        };
                        let a97 = {
                            let a94 = match v1.clone() {
                                Adt1::C0(_, h93, _, _, _, _, _, _, _, _) => {
                                    h93
                                }
                            };
                            let a95 = v6;
                            let a96 = 1u64;
                            nat_rem(a94, a95, a96)
                        };
                        let a98 = 0u64;
                        if if if nat_eq(a97, a98) {
                            let a103 = {
                                let a100 = match v4.clone() {
                                    Adt1::C0(_, h99, _, _, _, _, _, _, _, _) => {
                                        h99
                                    }
                                };
                                let a101 = v6;
                                let a102 = 1u64;
                                nat_rem(a100, a101, a102)
                            };
                            let a104 = 0u64;
                            nat_eq(a103, a104)
                        } else {
                            false
                        } {
                            let a109 = {
                                let a106 = match v3.clone() {
                                    Adt1::C0(_, h105, _, _, _, _, _, _, _, _) => {
                                        h105
                                    }
                                };
                                let a107 = v6;
                                let a108 = 1u64;
                                nat_rem(a106, a107, a108)
                            };
                            let a110 = 0u64;
                            nat_eq(a109, a110)
                        } else {
                            false
                        } {
                            Ok(Adt5::C4(f8(v3.clone(), v6)?, match v5 {
                                Adt7::C0(_, _, h111) => {
                                    h111
                                }
                            }, f9(v1, v6), f9(v3, v6), Adt3::C2(v0, f9(v4, v6))))
                        } else {
                            Ok(Adt5::C1)
                        }
                    }
                }
            } else {
                Ok(Adt5::C1)
            }
        }
    }
}

pub fn f5(v0: u64, v1: u64, v2: Adt1, v3: Option<Adt1>, v4: Adt1) -> R<Adt5> {
    let m112 = v3;
    match m112 {
        None => {
            f12(v0, v1, match v2.clone() {
                Adt1::C0(h113, _, _, _, _, _, _, _, _, _) => {
                    h113
                }
            }, 0u64, match v4.clone() {
                Adt1::C0(h114, _, _, _, _, _, _, _, _, _) => {
                    h114
                }
            }, false, match v2.clone() {
                Adt1::C0(_, h115, _, _, _, _, _, _, _, _) => {
                    h115
                }
            }, match v2.clone() {
                Adt1::C0(_, _, h116, _, _, _, _, _, _, _) => {
                    h116
                }
            }, match v2.clone() {
                Adt1::C0(_, _, _, h117, _, _, _, _, _, _) => {
                    h117
                }
            }, match v2 {
                Adt1::C0(_, _, _, _, h118, _, _, _, _, _) => {
                    h118
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt1::C0(_, h119, _, _, _, _, _, _, _, _) => {
                    h119
                }
            }, match v4.clone() {
                Adt1::C0(_, _, h120, _, _, _, _, _, _, _) => {
                    h120
                }
            }, match v4.clone() {
                Adt1::C0(_, _, _, h121, _, _, _, _, _, _) => {
                    h121
                }
            }, match v4 {
                Adt1::C0(_, _, _, _, h122, _, _, _, _, _) => {
                    h122
                }
            })
        }
        Some(v5) => {
            f12(v0, v1, match v2.clone() {
                Adt1::C0(h123, _, _, _, _, _, _, _, _, _) => {
                    h123
                }
            }, match v5.clone() {
                Adt1::C0(h124, _, _, _, _, _, _, _, _, _) => {
                    h124
                }
            }, match v4.clone() {
                Adt1::C0(h125, _, _, _, _, _, _, _, _, _) => {
                    h125
                }
            }, true, match v2.clone() {
                Adt1::C0(_, h126, _, _, _, _, _, _, _, _) => {
                    h126
                }
            }, match v2.clone() {
                Adt1::C0(_, _, h127, _, _, _, _, _, _, _) => {
                    h127
                }
            }, match v2.clone() {
                Adt1::C0(_, _, _, h128, _, _, _, _, _, _) => {
                    h128
                }
            }, match v2 {
                Adt1::C0(_, _, _, _, h129, _, _, _, _, _) => {
                    h129
                }
            }, match v5.clone() {
                Adt1::C0(_, h130, _, _, _, _, _, _, _, _) => {
                    h130
                }
            }, match v5.clone() {
                Adt1::C0(_, _, h131, _, _, _, _, _, _, _) => {
                    h131
                }
            }, match v5.clone() {
                Adt1::C0(_, _, _, h132, _, _, _, _, _, _) => {
                    h132
                }
            }, match v5 {
                Adt1::C0(_, _, _, _, h133, _, _, _, _, _) => {
                    h133
                }
            }, match v4.clone() {
                Adt1::C0(_, h134, _, _, _, _, _, _, _, _) => {
                    h134
                }
            }, match v4.clone() {
                Adt1::C0(_, _, h135, _, _, _, _, _, _, _) => {
                    h135
                }
            }, match v4.clone() {
                Adt1::C0(_, _, _, h136, _, _, _, _, _, _) => {
                    h136
                }
            }, match v4 {
                Adt1::C0(_, _, _, _, h137, _, _, _, _, _) => {
                    h137
                }
            })
        }
    }
}

pub fn f6(v0: u64, v1: Adt2, v2: Adt2, v3: Adt3) -> R<Adt4> {
    let m138 = v3;
    match m138 {
        Adt3::C0 => {
            Ok(Adt4::C1(false, f13(v0, v1)?, f13(v0, v2)?))
        }
        Adt3::C1 => {
            Ok(Adt4::C1(false, f14(v0, v1, v2.clone())?, f13(v0, v2)?))
        }
        Adt3::C2(v4, v5) => {
            f15(v0, v1, v5, v2, v4)
        }
    }
}

pub fn f7(v0: u64) -> Adt6 {
    let a139 = v0;
    let a140 = 0u64;
    if nat_eq(a139, a140) {
        Adt6::C1(Adt7::C0(0u64, 1u64, 4u64))
    } else {
        let a141 = v0;
        let a142 = 1u64;
        if nat_eq(a141, a142) {
            Adt6::C1(Adt7::C0(1u64, 1u64, 2u64))
        } else {
            let a143 = v0;
            let a144 = 2u64;
            if nat_eq(a143, a144) {
                Adt6::C1(Adt7::C0(2u64, 32u64, 18u64))
            } else {
                let a145 = v0;
                let a146 = 3u64;
                if nat_eq(a145, a146) {
                    Adt6::C1(Adt7::C0(3u64, 32u64, 20u64))
                } else {
                    let a147 = v0;
                    let a148 = 6u64;
                    if nat_eq(a147, a148) {
                        Adt6::C1(Adt7::C0(6u64, 32u64, 22u64))
                    } else {
                        let a149 = v0;
                        let a150 = 7u64;
                        if nat_eq(a149, a150) {
                            Adt6::C1(Adt7::C0(7u64, 32u64, 24u64))
                        } else {
                            let a151 = v0;
                            let a152 = 8u64;
                            if nat_eq(a151, a152) {
                                Adt6::C1(Adt7::C0(8u64, 32u64, 34u64))
                            } else {
                                let a153 = v0;
                                let a154 = 9u64;
                                if nat_eq(a153, a154) {
                                    Adt6::C1(Adt7::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a155 = v0;
                                    let a156 = 10u64;
                                    if nat_eq(a155, a156) {
                                        Adt6::C1(Adt7::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a157 = v0;
                                        let a158 = 11u64;
                                        if nat_eq(a157, a158) {
                                            Adt6::C1(Adt7::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a159 = v0;
                                            let a160 = 12u64;
                                            if nat_eq(a159, a160) {
                                                Adt6::C1(Adt7::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a161 = v0;
                                                let a162 = 13u64;
                                                if nat_eq(a161, a162) {
                                                    Adt6::C1(Adt7::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a163 = v0;
                                                    let a164 = 14u64;
                                                    if nat_eq(a163, a164) {
                                                        Adt6::C1(Adt7::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a165 = v0;
                                                        let a166 = 15u64;
                                                        if nat_eq(a165, a166) {
                                                            Adt6::C1(Adt7::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a167 = v0;
                                                            let a168 = 16u64;
                                                            if nat_eq(a167, a168) {
                                                                Adt6::C1(Adt7::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a169 = v0;
                                                                let a170 = 17u64;
                                                                if nat_eq(a169, a170) {
                                                                    Adt6::C1(Adt7::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a171 = v0;
                                                                    let a172 = 18u64;
                                                                    if nat_eq(a171, a172) {
                                                                        Adt6::C1(Adt7::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a173 = v0;
                                                                        let a174 = 19u64;
                                                                        if nat_eq(a173, a174) {
                                                                            Adt6::C1(Adt7::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a175 = v0;
                                                                            let a176 = 20u64;
                                                                            if nat_eq(a175, a176) {
                                                                                Adt6::C1(Adt7::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a177 = v0;
                                                                                let a178 = 21u64;
                                                                                if nat_eq(a177, a178) {
                                                                                    Adt6::C1(Adt7::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a179 = v0;
                                                                                    let a180 = 22u64;
                                                                                    if nat_eq(a179, a180) {
                                                                                        Adt6::C1(Adt7::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a181 = v0;
                                                                                        let a182 = 23u64;
                                                                                        if nat_eq(a181, a182) {
                                                                                            Adt6::C1(Adt7::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a183 = v0;
                                                                                            let a184 = 24u64;
                                                                                            if nat_eq(a183, a184) {
                                                                                                Adt6::C1(Adt7::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a185 = v0;
                                                                                                let a186 = 25u64;
                                                                                                if nat_eq(a185, a186) {
                                                                                                    Adt6::C1(Adt7::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a187 = v0;
                                                                                                    let a188 = 26u64;
                                                                                                    if nat_eq(a187, a188) {
                                                                                                        Adt6::C1(Adt7::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a189 = v0;
                                                                                                        let a190 = 27u64;
                                                                                                        if nat_eq(a189, a190) {
                                                                                                            Adt6::C1(Adt7::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a191 = v0;
                                                                                                            let a192 = 28u64;
                                                                                                            if nat_eq(a191, a192) {
                                                                                                                Adt6::C1(Adt7::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a193 = v0;
                                                                                                                let a194 = 29u64;
                                                                                                                if nat_eq(a193, a194) {
                                                                                                                    Adt6::C1(Adt7::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a195 = v0;
                                                                                                                    let a196 = 30u64;
                                                                                                                    if nat_eq(a195, a196) {
                                                                                                                        Adt6::C1(Adt7::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a197 = v0;
                                                                                                                        let a198 = 34u64;
                                                                                                                        if nat_eq(a197, a198) {
                                                                                                                            Adt6::C1(Adt7::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a199 = v0;
                                                                                                                            let a200 = 35u64;
                                                                                                                            if nat_eq(a199, a200) {
                                                                                                                                Adt6::C1(Adt7::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a201 = v0;
                                                                                                                                let a202 = 39u64;
                                                                                                                                if nat_eq(a201, a202) {
                                                                                                                                    Adt6::C1(Adt7::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a203 = v0;
                                                                                                                                    let a204 = 40u64;
                                                                                                                                    if nat_eq(a203, a204) {
                                                                                                                                        Adt6::C1(Adt7::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a205 = v0;
                                                                                                                                        let a206 = 41u64;
                                                                                                                                        if nat_eq(a205, a206) {
                                                                                                                                            Adt6::C1(Adt7::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a207 = v0;
                                                                                                                                            let a208 = 42u64;
                                                                                                                                            if nat_eq(a207, a208) {
                                                                                                                                                Adt6::C1(Adt7::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a209 = v0;
                                                                                                                                                let a210 = 43u64;
                                                                                                                                                if nat_eq(a209, a210) {
                                                                                                                                                    Adt6::C1(Adt7::C0(43u64, 1u64, 1u64))
                                                                                                                                                } else {
                                                                                                                                                    Adt6::C0
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
    let a222 = {
        let a216 = {
            let a212 = match v0.clone() {
                Adt1::C0(_, h211, _, _, _, _, _, _, _, _) => {
                    h211
                }
            };
            let a213 = v1;
            let a214 = 0u64;
            nat_quot(a212, a213, a214)
        };
        let a217 = match v0.clone() {
            Adt1::C0(_, _, h215, _, _, _, _, _, _, _) => {
                h215
            }
        };
        nat_mul(a216, a217)?
    };
    let a223 = {
        let a220 = match v0.clone() {
            Adt1::C0(_, _, _, h218, _, _, _, _, _, _) => {
                h218
            }
        };
        let a221 = match v0 {
            Adt1::C0(_, _, _, _, h219, _, _, _, _, _) => {
                h219
            }
        };
        nat_mul(a220, a221)?
    };
    nat_mul(a222, a223)
}

pub fn f9(v0: Adt1, v1: u64) -> Adt2 {
    Adt2::C0({
        let a225 = match v0.clone() {
            Adt1::C0(_, h224, _, _, _, _, _, _, _, _) => {
                h224
            }
        };
        let a226 = v1;
        let a227 = 0u64;
        nat_quot(a225, a226, a227)
    }, match v0.clone() {
        Adt1::C0(_, _, h228, _, _, _, _, _, _, _) => {
            h228
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h229, _, _, _, _, _, _) => {
            h229
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h230, _, _, _, _, _) => {
            h230
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, h231, _, _, _, _) => {
            h231
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h232, _, _, _) => {
            h232
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h233, _, _) => {
            h233
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h234, _) => {
            h234
        }
    })
}

pub fn f10(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f16(v4, v5, v6, v7) {
        f16(v0, v1, v2, v3)
    } else {
        let a238 = {
            let a235 = v0;
            let a236 = v4;
            let a237 = 1u64;
            nat_rem(a235, a236, a237)
        };
        let a239 = 0u64;
        if if if nat_eq(a238, a239) {
            let a243 = {
                let a240 = v1;
                let a241 = v5;
                let a242 = 1u64;
                nat_rem(a240, a241, a242)
            };
            let a244 = 0u64;
            nat_eq(a243, a244)
        } else {
            false
        } {
            let a248 = {
                let a245 = v2;
                let a246 = v6;
                let a247 = 1u64;
                nat_rem(a245, a246, a247)
            };
            let a249 = 0u64;
            nat_eq(a248, a249)
        } else {
            false
        } {
            let a253 = {
                let a250 = v3;
                let a251 = v7;
                let a252 = 1u64;
                nat_rem(a250, a251, a252)
            };
            let a254 = 0u64;
            nat_eq(a253, a254)
        } else {
            false
        }
    }
}

pub fn f11(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a255 = v0;
    let a256 = v1;
    if nat_eq(a255, a256) {
        let a259 = v4;
        let a260 = {
            let a257 = v2;
            let a258 = v3;
            nat_add(a257, a258)?
        };
        Ok(nat_eq(a259, a260))
    } else {
        let a261 = v2;
        let a262 = v3;
        if nat_eq(a261, a262) {
            let a263 = v4;
            let a264 = v2;
            Ok(nat_eq(a263, a264))
        } else {
            Ok(false)
        }
    }
}

pub fn f12(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt5> {
    let m265 = f17(v0, v1);
    match m265 {
        None => {
            if v5 {
                let m266 = f18(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m266 {
                    Adt8::C0 => {
                        Ok(Adt5::C0)
                    }
                    Adt8::C1 => {
                        Ok(Adt5::C1)
                    }
                    Adt8::C2(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27) => {
                        Ok(Adt5::C3(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27))
                    }
                }
            } else {
                Ok(Adt5::C1)
            }
        }
        Some(v28) => {
            let v29: bool = {
                let a267 = 4u64;
                let a268 = v28;
                nat_le(a267, a268)
            };
            let a269 = v2;
            let a270 = 0u64;
            if if if if if nat_eq(a269, a270) {
                let a271 = v4;
                let a272 = 0u64;
                nat_eq(a271, a272)
            } else {
                false
            } {
                let a273 = v6;
                let a274 = v14;
                if nat_eq(a273, a274) {
                    let a275 = v7;
                    let a276 = v15;
                    nat_eq(a275, a276)
                } else {
                    false
                }
            } else {
                false
            } {
                let a277 = v8;
                let a278 = v16;
                if nat_eq(a277, a278) {
                    let a279 = v9;
                    let a280 = v17;
                    nat_eq(a279, a280)
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
                        let a281 = v3;
                        let a282 = 0u64;
                        nat_eq(a281, a282)
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
                Ok(Adt5::C2(v28, v29, v6, v7, v8, v9, {
                    let a287 = {
                        let a283 = v6;
                        let a284 = v7;
                        nat_mul(a283, a284)?
                    };
                    let a288 = {
                        let a285 = v8;
                        let a286 = v9;
                        nat_mul(a285, a286)?
                    };
                    nat_mul(a287, a288)?
                }))
            } else {
                Ok(Adt5::C1)
            }
        }
    }
}

pub fn f13(v0: u64, v1: Adt2) -> R<u64> {
    let v2: u64 = {
        let a290 = v0;
        let a291 = match v1.clone() {
            Adt2::C0(h289, _, _, _, _, _, _, _) => {
                h289
            }
        };
        let a292 = 0u64;
        nat_quot(a290, a291, a292)
    };
    let v3: u64 = {
        let a294 = v2;
        let a295 = match v1.clone() {
            Adt2::C0(_, h293, _, _, _, _, _, _) => {
                h293
            }
        };
        let a296 = 0u64;
        nat_quot(a294, a295, a296)
    };
    f19({
        let a298 = v0;
        let a299 = match v1.clone() {
            Adt2::C0(h297, _, _, _, _, _, _, _) => {
                h297
            }
        };
        let a300 = 0u64;
        nat_rem(a298, a299, a300)
    }, {
        let a302 = v2;
        let a303 = match v1.clone() {
            Adt2::C0(_, h301, _, _, _, _, _, _) => {
                h301
            }
        };
        let a304 = 0u64;
        nat_rem(a302, a303, a304)
    }, {
        let a306 = v3;
        let a307 = match v1.clone() {
            Adt2::C0(_, _, h305, _, _, _, _, _) => {
                h305
            }
        };
        let a308 = 0u64;
        nat_rem(a306, a307, a308)
    }, {
        let a310 = v3;
        let a311 = match v1.clone() {
            Adt2::C0(_, _, h309, _, _, _, _, _) => {
                h309
            }
        };
        let a312 = 0u64;
        nat_quot(a310, a311, a312)
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h313, _, _, _) => {
            h313
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, _, h314, _, _) => {
            h314
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h315, _) => {
            h315
        }
    }, match v1 {
        Adt2::C0(_, _, _, _, _, _, _, h316) => {
            h316
        }
    })
}

pub fn f14(v0: u64, v1: Adt2, v2: Adt2) -> R<u64> {
    let v3: u64 = {
        let a318 = v0;
        let a319 = match v2.clone() {
            Adt2::C0(h317, _, _, _, _, _, _, _) => {
                h317
            }
        };
        let a320 = 0u64;
        nat_quot(a318, a319, a320)
    };
    let v4: u64 = {
        let a322 = v3;
        let a323 = match v2.clone() {
            Adt2::C0(_, h321, _, _, _, _, _, _) => {
                h321
            }
        };
        let a324 = 0u64;
        nat_quot(a322, a323, a324)
    };
    f20({
        let a326 = v0;
        let a327 = match v2.clone() {
            Adt2::C0(h325, _, _, _, _, _, _, _) => {
                h325
            }
        };
        let a328 = 0u64;
        nat_rem(a326, a327, a328)
    }, {
        let a330 = v3;
        let a331 = match v2.clone() {
            Adt2::C0(_, h329, _, _, _, _, _, _) => {
                h329
            }
        };
        let a332 = 0u64;
        nat_rem(a330, a331, a332)
    }, {
        let a334 = v4;
        let a335 = match v2.clone() {
            Adt2::C0(_, _, h333, _, _, _, _, _) => {
                h333
            }
        };
        let a336 = 0u64;
        nat_rem(a334, a335, a336)
    }, {
        let a338 = v4;
        let a339 = match v2 {
            Adt2::C0(_, _, h337, _, _, _, _, _) => {
                h337
            }
        };
        let a340 = 0u64;
        nat_quot(a338, a339, a340)
    }, match v1.clone() {
        Adt2::C0(h341, _, _, _, _, _, _, _) => {
            h341
        }
    }, match v1.clone() {
        Adt2::C0(_, h342, _, _, _, _, _, _) => {
            h342
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h343, _, _, _, _, _) => {
            h343
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h344, _, _, _, _) => {
            h344
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h345, _, _, _) => {
            h345
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, _, h346, _, _) => {
            h346
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h347, _) => {
            h347
        }
    }, match v1 {
        Adt2::C0(_, _, _, _, _, _, _, h348) => {
            h348
        }
    })
}

pub fn f15(v0: u64, v1: Adt2, v2: Adt2, v3: Adt2, v4: u64) -> R<Adt4> {
    let v5: u64 = {
        let a350 = v0;
        let a351 = match v3.clone() {
            Adt2::C0(h349, _, _, _, _, _, _, _) => {
                h349
            }
        };
        let a352 = 0u64;
        nat_quot(a350, a351, a352)
    };
    let v6: u64 = {
        let a354 = v5;
        let a355 = match v3.clone() {
            Adt2::C0(_, h353, _, _, _, _, _, _) => {
                h353
            }
        };
        let a356 = 0u64;
        nat_quot(a354, a355, a356)
    };
    let v7: u64 = {
        let a358 = v0;
        let a359 = match v3.clone() {
            Adt2::C0(h357, _, _, _, _, _, _, _) => {
                h357
            }
        };
        let a360 = 0u64;
        nat_rem(a358, a359, a360)
    };
    let v8: u64 = {
        let a362 = v5;
        let a363 = match v3.clone() {
            Adt2::C0(_, h361, _, _, _, _, _, _) => {
                h361
            }
        };
        let a364 = 0u64;
        nat_rem(a362, a363, a364)
    };
    let v9: u64 = {
        let a366 = v6;
        let a367 = match v3.clone() {
            Adt2::C0(_, _, h365, _, _, _, _, _) => {
                h365
            }
        };
        let a368 = 0u64;
        nat_rem(a366, a367, a368)
    };
    let v10: u64 = {
        let a370 = v6;
        let a371 = match v3.clone() {
            Adt2::C0(_, _, h369, _, _, _, _, _) => {
                h369
            }
        };
        let a372 = 0u64;
        nat_quot(a370, a371, a372)
    };
    let v11: u64 = {
        let a373 = v4;
        let a374 = 0u64;
        if nat_eq(a373, a374) {
            match v1.clone() {
                Adt2::C0(h375, _, _, _, _, _, _, _) => {
                    h375
                }
            }
        } else {
            let a376 = v4;
            let a377 = 1u64;
            if nat_eq(a376, a377) {
                match v1.clone() {
                    Adt2::C0(_, h378, _, _, _, _, _, _) => {
                        h378
                    }
                }
            } else {
                let a379 = v4;
                let a380 = 2u64;
                if nat_eq(a379, a380) {
                    match v1.clone() {
                        Adt2::C0(_, _, h381, _, _, _, _, _) => {
                            h381
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt2::C0(_, _, _, h382, _, _, _, _) => {
                            h382
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a389 = v11;
        let a390 = {
            let a383 = v4;
            let a384 = 0u64;
            if nat_eq(a383, a384) {
                v7
            } else {
                let a385 = v4;
                let a386 = 1u64;
                if nat_eq(a385, a386) {
                    v8
                } else {
                    let a387 = v4;
                    let a388 = 2u64;
                    if nat_eq(a387, a388) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a389, a390)
    };
    let v13: Adt2 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt4::C1(v12, f19(if if v12 {
        let a391 = v4;
        let a392 = 0u64;
        nat_eq(a391, a392)
    } else {
        false
    } {
        let a393 = v7;
        let a394 = v11;
        nat_sub(a393, a394)
    } else {
        v7
    }, if if v12 {
        let a395 = v4;
        let a396 = 1u64;
        nat_eq(a395, a396)
    } else {
        false
    } {
        let a397 = v8;
        let a398 = v11;
        nat_sub(a397, a398)
    } else {
        v8
    }, if if v12 {
        let a399 = v4;
        let a400 = 2u64;
        nat_eq(a399, a400)
    } else {
        false
    } {
        let a401 = v9;
        let a402 = v11;
        nat_sub(a401, a402)
    } else {
        v9
    }, if if v12 {
        let a403 = v4;
        let a404 = 3u64;
        nat_eq(a403, a404)
    } else {
        false
    } {
        let a405 = v10;
        let a406 = v11;
        nat_sub(a405, a406)
    } else {
        v10
    }, match v13.clone() {
        Adt2::C0(_, _, _, _, h407, _, _, _) => {
            h407
        }
    }, match v13.clone() {
        Adt2::C0(_, _, _, _, _, h408, _, _) => {
            h408
        }
    }, match v13.clone() {
        Adt2::C0(_, _, _, _, _, _, h409, _) => {
            h409
        }
    }, match v13 {
        Adt2::C0(_, _, _, _, _, _, _, h410) => {
            h410
        }
    })?, f13(v0, v3)?))
}

pub fn f16(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a411 = v0;
    let a412 = 0u64;
    if if if nat_eq(a411, a412) {
        true
    } else {
        let a413 = v1;
        let a414 = 0u64;
        nat_eq(a413, a414)
    } {
        true
    } else {
        let a415 = v2;
        let a416 = 0u64;
        nat_eq(a415, a416)
    } {
        true
    } else {
        let a417 = v3;
        let a418 = 0u64;
        nat_eq(a417, a418)
    }
}

pub fn f17(v0: u64, v1: u64) -> Option<u64> {
    let a419 = v0;
    let a420 = 2u64;
    if nat_eq(a419, a420) {
        Some(0u64)
    } else {
        let a421 = v0;
        let a422 = 5u64;
        if nat_eq(a421, a422) {
            Some(1u64)
        } else {
            let a423 = v0;
            let a424 = 6u64;
            if nat_eq(a423, a424) {
                Some(2u64)
            } else {
                let a425 = v0;
                let a426 = 7u64;
                if nat_eq(a425, a426) {
                    Some(3u64)
                } else {
                    let a427 = v0;
                    let a428 = 9u64;
                    if nat_eq(a427, a428) {
                        Some(4u64)
                    } else {
                        let a429 = v0;
                        let a430 = 8u64;
                        if nat_eq(a429, a430) {
                            Some(5u64)
                        } else {
                            let a431 = v0;
                            let a432 = 89u64;
                            if nat_eq(a431, a432) {
                                let a433 = v1;
                                let a434 = 2u64;
                                if nat_eq(a433, a434) {
                                    Some(6u64)
                                } else {
                                    let a435 = v1;
                                    let a436 = 0u64;
                                    if nat_eq(a435, a436) {
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

pub fn f18(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt8> {
    let a437 = v0;
    let a438 = 28u64;
    if if if if nat_eq(a437, a438) {
        let a439 = v1;
        let a440 = 0u64;
        nat_eq(a439, a440)
    } else {
        false
    } {
        let a441 = v2;
        let a442 = 0u64;
        nat_eq(a441, a442)
    } else {
        false
    } {
        let a443 = v3;
        let a444 = 0u64;
        nat_eq(a443, a444)
    } else {
        false
    } {
        let a445 = v4;
        let a446 = v8;
        if if if if if if if if nat_eq(a445, a446) {
            let a447 = v5;
            let a448 = v12;
            nat_eq(a447, a448)
        } else {
            false
        } {
            let a449 = v9;
            let a450 = v13;
            nat_eq(a449, a450)
        } else {
            false
        } {
            let a451 = v10;
            let a452 = v14;
            nat_eq(a451, a452)
        } else {
            false
        } {
            let a453 = v11;
            let a454 = v15;
            nat_eq(a453, a454)
        } else {
            false
        } {
            let a455 = 0u64;
            let a456 = v6;
            nat_lt(a455, a456)
        } else {
            false
        } {
            let a457 = 0u64;
            let a458 = v7;
            nat_lt(a457, a458)
        } else {
            false
        } {
            let a462 = {
                let a459 = v10;
                let a460 = v6;
                let a461 = 1u64;
                nat_rem(a459, a460, a461)
            };
            let a463 = 0u64;
            if nat_eq(a462, a463) {
                let a467 = {
                    let a464 = v11;
                    let a465 = v7;
                    let a466 = 1u64;
                    nat_rem(a464, a465, a466)
                };
                let a468 = 0u64;
                nat_eq(a467, a468)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt8::C2(v5, v4, v9, v10, v11, {
                let a469 = v10;
                let a470 = v6;
                let a471 = 0u64;
                nat_quot(a469, a470, a471)
            }, {
                let a472 = v11;
                let a473 = v7;
                let a474 = 0u64;
                nat_quot(a472, a473, a474)
            }, {
                let a475 = v5;
                let a476 = v4;
                nat_mul(a475, a476)?
            }, {
                let a477 = v4;
                let a478 = v9;
                nat_mul(a477, a478)?
            }, {
                let a479 = v5;
                let a480 = v9;
                nat_mul(a479, a480)?
            }))
        } else {
            Ok(Adt8::C1)
        }
    } else {
        Ok(Adt8::C0)
    }
}

pub fn f19(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a493 = {
        let a489 = {
            let a485 = {
                let a481 = v0;
                let a482 = v4;
                nat_mul(a481, a482)?
            };
            let a486 = {
                let a483 = v1;
                let a484 = v5;
                nat_mul(a483, a484)?
            };
            nat_add(a485, a486)?
        };
        let a490 = {
            let a487 = v2;
            let a488 = v6;
            nat_mul(a487, a488)?
        };
        nat_add(a489, a490)?
    };
    let a494 = {
        let a491 = v3;
        let a492 = v7;
        nat_mul(a491, a492)?
    };
    nat_add(a493, a494)
}

pub fn f20(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f19({
        let a495 = v0;
        let a496 = v4;
        let a497 = 0u64;
        nat_rem(a495, a496, a497)
    }, {
        let a498 = v1;
        let a499 = v5;
        let a500 = 0u64;
        nat_rem(a498, a499, a500)
    }, {
        let a501 = v2;
        let a502 = v6;
        let a503 = 0u64;
        nat_rem(a501, a502, a503)
    }, {
        let a504 = v3;
        let a505 = v7;
        let a506 = 0u64;
        nat_rem(a504, a505, a506)
    }, v8, v9, v10, v11)
}
