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
    C0(u64, Adt1, Adt2, Option<Adt2>, Adt2),
    C1(u64, Adt3, Adt3, Adt4),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt4 {
    C0,
    C1,
    C2(u64, Adt3),
    C3(u64, u64, u64, u64, bool),
    C4(u64),
    C5(u64, u64, u64, u64),
    C6(u64, u64, bool),
    C7(u64),
    C8(u64),
    C9(u64, u64),
    C10(u64, bool, Adt3),
    C11(u64, u64),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Adt6),
    C1(bool, u64, u64),
    C2(u64, u64, u64, u64, u64, u64),
    C3(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt6 {
    C0,
    C1,
    C2(u64, bool, u64, u64, u64, u64, u64),
    C3(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
    C4(u64, u64, Adt3, Adt3, Adt4),
}

#[derive(Clone)]
pub enum Adt7 {
    C0,
    C1(Adt8),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt9 {
    C0,
    C1,
    C2(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

pub fn f0(v0: Adt0) -> R<Adt5> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1, v2, v3, v4, v5) => {
            let v6: u64 = match v2.clone() {
                Adt1::C0(h2, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2
                }
            };
            Ok(Adt5::C0(if f1(v1) {
                f2(v1, v3, {
                    let m3 = v4;
                    match m3 {
                        None => {
                            false
                        }
                        Some(_) => {
                            true
                        }
                    }
                }, v5)?
            } else {
                let a4 = v1;
                let a5 = 19u64;
                if nat_eq(a4, a5) {
                    f3(v3, v5)?
                } else {
                    let a6 = v1;
                    let a7 = 21u64;
                    if nat_eq(a6, a7) {
                        f4(v6, v3, v4, v5)?
                    } else {
                        let a8 = v1;
                        let a9 = 61u64;
                        if if if nat_eq(a8, a9) {
                            true
                        } else {
                            let a10 = v1;
                            let a11 = 62u64;
                            nat_eq(a10, a11)
                        } {
                            true
                        } else {
                            let a12 = v1;
                            let a13 = 63u64;
                            nat_eq(a12, a13)
                        } {
                            f5(v1, v2, v3, v5)?
                        } else {
                            let a14 = v1;
                            let a15 = 4u64;
                            if if nat_eq(a14, a15) {
                                true
                            } else {
                                let a16 = v1;
                                let a17 = 32u64;
                                nat_eq(a16, a17)
                            } {
                                f6(v1, v2, v3, v4, v5)?
                            } else {
                                let a18 = v1;
                                let a19 = 20u64;
                                if nat_eq(a18, a19) {
                                    f7(v3, v5)?
                                } else {
                                    let a20 = v1;
                                    let a21 = 42u64;
                                    if nat_eq(a20, a21) {
                                        f8(4u64, v3, v5)?
                                    } else {
                                        let a22 = v1;
                                        let a23 = 69u64;
                                        if nat_eq(a22, a23) {
                                            let a24 = v6;
                                            let a25 = 4u64;
                                            if nat_lt(a24, a25) {
                                                f8(v6, v3, v5)?
                                            } else {
                                                Adt6::C1
                                            }
                                        } else {
                                            let a26 = v1;
                                            let a27 = 70u64;
                                            if nat_eq(a26, a27) {
                                                f9(v6, v3, v5)?
                                            } else {
                                                f10(v1, v6, v3, v4, v5)?
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }))
        }
        Adt0::C1(v8, v9, v10, v11) => {
            f11(v8, v9, v10, v11)
        }
    }
}

pub fn f1(v0: u64) -> bool {
    let a28 = v0;
    let a29 = 1u64;
    if if nat_eq(a28, a29) {
        true
    } else {
        let a30 = v0;
        let a31 = 33u64;
        nat_eq(a30, a31)
    } {
        true
    } else {
        let a32 = v0;
        let a33 = 34u64;
        nat_eq(a32, a33)
    }
}

pub fn f2(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a36 = match v1.clone() {
        Adt2::C0(h34, _, _, _, _, _, _, _, _, _, _) => {
            h34
        }
    };
    let a37 = match v3.clone() {
        Adt2::C0(h35, _, _, _, _, _, _, _, _, _, _) => {
            h35
        }
    };
    if nat_eq(a36, a37) {
        f12(v0, v1, v2, v3)
    } else {
        if if if if f13(match v1.clone() {
            Adt2::C0(h38, _, _, _, _, _, _, _, _, _, _) => {
                h38
            }
        }) {
            f13(match v3.clone() {
                Adt2::C0(h39, _, _, _, _, _, _, _, _, _, _) => {
                    h39
                }
            })
        } else {
            false
        } {
            let a40 = v0;
            let a41 = 33u64;
            if nat_eq(a40, a41) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a42 = f14(v1.clone(), 1u64)?;
            let a43 = f14(v3.clone(), 1u64)?;
            nat_eq(a42, a43)
        } else {
            false
        } {
            let m45 = f15(match v3.clone() {
                Adt2::C0(h44, _, _, _, _, _, _, _, _, _, _) => {
                    h44
                }
            });
            match m45 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v4) => {
                    Ok(Adt6::C4(f14(v3.clone(), 1u64)?, match v4 {
                        Adt8::C0(_, _, h46) => {
                            h46
                        }
                    }, f16(v1.clone(), 1u64), f16(v3.clone(), 1u64), Adt4::C11(match v1 {
                        Adt2::C0(h47, _, _, _, _, _, _, _, _, _, _) => {
                            h47
                        }
                    }, match v3 {
                        Adt2::C0(h48, _, _, _, _, _, _, _, _, _, _) => {
                            h48
                        }
                    })))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    }
}

pub fn f3(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a50 = match v0.clone() {
        Adt2::C0(h49, _, _, _, _, _, _, _, _, _, _) => {
            h49
        }
    };
    let a51 = 0u64;
    if if if if nat_eq(a50, a51) {
        true
    } else {
        let a53 = match v0.clone() {
            Adt2::C0(h52, _, _, _, _, _, _, _, _, _, _) => {
                h52
            }
        };
        let a54 = 1u64;
        nat_eq(a53, a54)
    } {
        true
    } else {
        let a56 = match v0.clone() {
            Adt2::C0(h55, _, _, _, _, _, _, _, _, _, _) => {
                h55
            }
        };
        let a57 = 25u64;
        if nat_eq(a56, a57) {
            true
        } else {
            let a59 = match v0.clone() {
                Adt2::C0(h58, _, _, _, _, _, _, _, _, _, _) => {
                    h58
                }
            };
            let a60 = 26u64;
            nat_eq(a59, a60)
        }
    } {
        true
    } else {
        let a62 = match v0.clone() {
            Adt2::C0(h61, _, _, _, _, _, _, _, _, _, _) => {
                h61
            }
        };
        let a63 = 30u64;
        nat_eq(a62, a63)
    } {
        let a66 = match v0.clone() {
            Adt2::C0(h64, _, _, _, _, _, _, _, _, _, _) => {
                h64
            }
        };
        let a67 = match v1.clone() {
            Adt2::C0(h65, _, _, _, _, _, _, _, _, _, _) => {
                h65
            }
        };
        if if nat_eq(a66, a67) {
            f17(match v1.clone() {
                Adt2::C0(_, h68, _, _, _, _, _, _, _, _, _) => {
                    h68
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h69, _, _, _, _, _, _, _, _) => {
                    h69
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h70, _, _, _, _, _, _, _) => {
                    h70
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h71, _, _, _, _, _, _) => {
                    h71
                }
            }, match v0.clone() {
                Adt2::C0(_, h72, _, _, _, _, _, _, _, _, _) => {
                    h72
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h73, _, _, _, _, _, _, _, _) => {
                    h73
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h74, _, _, _, _, _, _, _) => {
                    h74
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h75, _, _, _, _, _, _) => {
                    h75
                }
            })
        } else {
            false
        } {
            let m77 = f15(match v0.clone() {
                Adt2::C0(h76, _, _, _, _, _, _, _, _, _, _) => {
                    h76
                }
            });
            match m77 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v2) => {
                    Ok(Adt6::C4(f14(v1.clone(), 1u64)?, match v2 {
                        Adt8::C0(_, _, h78) => {
                            h78
                        }
                    }, f16(v0, 1u64), f16(v1, 1u64), Adt4::C1))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f4(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt6> {
    let m79 = v2;
    match m79 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let a80 = v0;
            let a81 = 4u64;
            if if if nat_lt(a80, a81) {
                let a84 = match v1.clone() {
                    Adt2::C0(h82, _, _, _, _, _, _, _, _, _, _) => {
                        h82
                    }
                };
                let a85 = match v4.clone() {
                    Adt2::C0(h83, _, _, _, _, _, _, _, _, _, _) => {
                        h83
                    }
                };
                if nat_eq(a84, a85) {
                    let a88 = match v1.clone() {
                        Adt2::C0(h86, _, _, _, _, _, _, _, _, _, _) => {
                            h86
                        }
                    };
                    let a89 = match v3.clone() {
                        Adt2::C0(h87, _, _, _, _, _, _, _, _, _, _) => {
                            h87
                        }
                    };
                    nat_eq(a88, a89)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f18(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h90, _, _, _, _, _, _, _, _, _) => {
                        h90
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h91, _, _, _, _, _, _, _, _, _) => {
                        h91
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h92, _, _, _, _, _, _, _, _, _) => {
                        h92
                    }
                })? {
                    f18(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h93, _, _, _, _, _, _, _, _) => {
                            h93
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h94, _, _, _, _, _, _, _, _) => {
                            h94
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h95, _, _, _, _, _, _, _, _) => {
                            h95
                        }
                    })?
                } else {
                    false
                } {
                    f18(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h96, _, _, _, _, _, _, _) => {
                            h96
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h97, _, _, _, _, _, _, _) => {
                            h97
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h98, _, _, _, _, _, _, _) => {
                            h98
                        }
                    })?
                } else {
                    false
                } {
                    f18(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h99, _, _, _, _, _, _) => {
                            h99
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h100, _, _, _, _, _, _) => {
                            h100
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h101, _, _, _, _, _, _) => {
                            h101
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m103 = f15(match v1.clone() {
                    Adt2::C0(h102, _, _, _, _, _, _, _, _, _, _) => {
                        h102
                    }
                });
                match m103 {
                    Adt7::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt7::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt8::C0(_, h104, _) => {
                                h104
                            }
                        };
                        let a109 = {
                            let a106 = match v1.clone() {
                                Adt2::C0(_, h105, _, _, _, _, _, _, _, _, _) => {
                                    h105
                                }
                            };
                            let a107 = v6;
                            let a108 = 1u64;
                            nat_rem(a106, a107, a108)
                        };
                        let a110 = 0u64;
                        if if if nat_eq(a109, a110) {
                            let a115 = {
                                let a112 = match v4.clone() {
                                    Adt2::C0(_, h111, _, _, _, _, _, _, _, _, _) => {
                                        h111
                                    }
                                };
                                let a113 = v6;
                                let a114 = 1u64;
                                nat_rem(a112, a113, a114)
                            };
                            let a116 = 0u64;
                            nat_eq(a115, a116)
                        } else {
                            false
                        } {
                            let a121 = {
                                let a118 = match v3.clone() {
                                    Adt2::C0(_, h117, _, _, _, _, _, _, _, _, _) => {
                                        h117
                                    }
                                };
                                let a119 = v6;
                                let a120 = 1u64;
                                nat_rem(a118, a119, a120)
                            };
                            let a122 = 0u64;
                            nat_eq(a121, a122)
                        } else {
                            false
                        } {
                            Ok(Adt6::C4(f14(v3.clone(), v6)?, match v5 {
                                Adt8::C0(_, _, h123) => {
                                    h123
                                }
                            }, f16(v1, v6), f16(v3, v6), Adt4::C2(v0, f16(v4, v6))))
                        } else {
                            Ok(Adt6::C1)
                        }
                    }
                }
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f5(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let a125 = match v2.clone() {
        Adt2::C0(h124, _, _, _, _, _, _, _, _, _, _) => {
            h124
        }
    };
    let a126 = 0u64;
    if if nat_eq(a125, a126) {
        let a128 = match v3.clone() {
            Adt2::C0(h127, _, _, _, _, _, _, _, _, _, _) => {
                h127
            }
        };
        let a129 = 0u64;
        nat_eq(a128, a129)
    } else {
        false
    } {
        let a130 = v0;
        let a131 = 61u64;
        if nat_eq(a130, a131) {
            let v4: bool = {
                let a133 = 0u64;
                let a134 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h132, _, _, _, _, _, _, _) => {
                        h132
                    }
                };
                nat_lt(a133, a134)
            };
            if if if if if if f19(match v2.clone() {
                Adt2::C0(_, h135, _, _, _, _, _, _, _, _, _) => {
                    h135
                }
            }, match v1.clone() {
                Adt1::C0(h136, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h136
                }
            }, match v1.clone() {
                Adt1::C0(_, h137, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h137
                }
            }, match v3.clone() {
                Adt2::C0(_, h138, _, _, _, _, _, _, _, _, _) => {
                    h138
                }
            })? {
                f19(match v2.clone() {
                    Adt2::C0(_, _, h139, _, _, _, _, _, _, _, _) => {
                        h139
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h140, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h140
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h141, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h141
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h142, _, _, _, _, _, _, _, _) => {
                        h142
                    }
                })?
            } else {
                false
            } {
                f19(match v2.clone() {
                    Adt2::C0(_, _, _, h143, _, _, _, _, _, _, _) => {
                        h143
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h144, _, _, _, _, _, _, _, _, _, _, _) => {
                        h144
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h145, _, _, _, _, _, _, _, _, _, _) => {
                        h145
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h146, _, _, _, _, _, _, _) => {
                        h146
                    }
                })?
            } else {
                false
            } {
                f19(match v2.clone() {
                    Adt2::C0(_, _, _, _, h147, _, _, _, _, _, _) => {
                        h147
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h148, _, _, _, _, _, _, _, _, _) => {
                        h148
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h149, _, _, _, _, _, _, _, _) => {
                        h149
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h150, _, _, _, _, _, _) => {
                        h150
                    }
                })?
            } else {
                false
            } {
                let a152 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h151, _, _, _, _, _, _, _) => {
                        h151
                    }
                };
                let a153 = 4294967295u64;
                nat_le(a152, a153)
            } else {
                false
            } {
                if v4 {
                    !f20(match v2.clone() {
                        Adt2::C0(_, h154, _, _, _, _, _, _, _, _, _) => {
                            h154
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h155, _, _, _, _, _, _, _, _) => {
                            h155
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h156, _, _, _, _, _, _, _) => {
                            h156
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h157, _, _, _, _, _, _) => {
                            h157
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt6::C4(f14(v3.clone(), 1u64)?, 4u64, f16(v2, 1u64), f16(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h158, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h158
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h159, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h159
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h160, _, _, _, _, _, _, _, _, _, _, _) => {
                        h160
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h161, _, _, _, _, _, _, _, _, _) => {
                        h161
                    }
                }, v4)))
            } else {
                Ok(Adt6::C1)
            }
        } else {
            let a162 = v0;
            let a163 = 62u64;
            if nat_eq(a162, a163) {
                let a165 = match v1.clone() {
                    Adt1::C0(h164, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h164
                    }
                };
                let a166 = 2147483648u64;
                if if if if if nat_lt(a165, a166) {
                    let a168 = match v1.clone() {
                        Adt1::C0(_, h167, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h167
                        }
                    };
                    let a169 = 2147483648u64;
                    nat_lt(a168, a169)
                } else {
                    false
                } {
                    let a172 = match v1.clone() {
                        Adt1::C0(h170, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h170
                        }
                    };
                    let a173 = match v2.clone() {
                        Adt2::C0(_, h171, _, _, _, _, _, _, _, _, _) => {
                            h171
                        }
                    };
                    nat_lt(a172, a173)
                } else {
                    false
                } {
                    let a176 = match v1.clone() {
                        Adt1::C0(_, h174, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h174
                        }
                    };
                    let a177 = match v2.clone() {
                        Adt2::C0(_, h175, _, _, _, _, _, _, _, _, _) => {
                            h175
                        }
                    };
                    nat_lt(a176, a177)
                } else {
                    false
                } {
                    let a180 = match v2.clone() {
                        Adt2::C0(_, _, h178, _, _, _, _, _, _, _, _) => {
                            h178
                        }
                    };
                    let a181 = match v3.clone() {
                        Adt2::C0(_, _, h179, _, _, _, _, _, _, _, _) => {
                            h179
                        }
                    };
                    if if if nat_eq(a180, a181) {
                        let a184 = match v2.clone() {
                            Adt2::C0(_, _, _, h182, _, _, _, _, _, _, _) => {
                                h182
                            }
                        };
                        let a185 = match v3.clone() {
                            Adt2::C0(_, _, _, h183, _, _, _, _, _, _, _) => {
                                h183
                            }
                        };
                        nat_eq(a184, a185)
                    } else {
                        false
                    } {
                        let a188 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h186, _, _, _, _, _, _) => {
                                h186
                            }
                        };
                        let a189 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h187, _, _, _, _, _, _) => {
                                h187
                            }
                        };
                        nat_eq(a188, a189)
                    } else {
                        false
                    } {
                        let a198 = match v3.clone() {
                            Adt2::C0(_, h190, _, _, _, _, _, _, _, _, _) => {
                                h190
                            }
                        };
                        let a199 = {
                            let a196 = {
                                let a193 = match v2.clone() {
                                    Adt2::C0(_, h191, _, _, _, _, _, _, _, _, _) => {
                                        h191
                                    }
                                };
                                let a194 = match v1.clone() {
                                    Adt1::C0(h192, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h192
                                    }
                                };
                                nat_add(a193, a194)?
                            };
                            let a197 = match v1.clone() {
                                Adt1::C0(_, h195, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h195
                                }
                            };
                            nat_add(a196, a197)?
                        };
                        nat_eq(a198, a199)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt6::C4(f14(v3.clone(), 1u64)?, 4u64, f16(v2, 1u64), f16(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h200, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h200
                        }
                    })))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                let a201 = v0;
                let a202 = 63u64;
                if nat_eq(a201, a202) {
                    if if if if if f21(match v2.clone() {
                        Adt2::C0(_, h203, _, _, _, _, _, _, _, _, _) => {
                            h203
                        }
                    }, match v1.clone() {
                        Adt1::C0(h204, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h204
                        }
                    })? {
                        f21(match v2.clone() {
                            Adt2::C0(_, _, h205, _, _, _, _, _, _, _, _) => {
                                h205
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h206, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h206
                            }
                        })?
                    } else {
                        false
                    } {
                        f21(match v2.clone() {
                            Adt2::C0(_, _, _, h207, _, _, _, _, _, _, _) => {
                                h207
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h208, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h208
                            }
                        })?
                    } else {
                        false
                    } {
                        f21(match v2.clone() {
                            Adt2::C0(_, _, _, _, h209, _, _, _, _, _, _) => {
                                h209
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h210, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h210
                            }
                        })?
                    } else {
                        false
                    } {
                        let a213 = match v2.clone() {
                            Adt2::C0(_, h211, _, _, _, _, _, _, _, _, _) => {
                                h211
                            }
                        };
                        let a214 = match v3.clone() {
                            Adt2::C0(_, h212, _, _, _, _, _, _, _, _, _) => {
                                h212
                            }
                        };
                        if if if nat_eq(a213, a214) {
                            let a217 = match v2.clone() {
                                Adt2::C0(_, _, h215, _, _, _, _, _, _, _, _) => {
                                    h215
                                }
                            };
                            let a218 = match v3.clone() {
                                Adt2::C0(_, _, h216, _, _, _, _, _, _, _, _) => {
                                    h216
                                }
                            };
                            nat_eq(a217, a218)
                        } else {
                            false
                        } {
                            let a221 = match v2.clone() {
                                Adt2::C0(_, _, _, h219, _, _, _, _, _, _, _) => {
                                    h219
                                }
                            };
                            let a222 = match v3.clone() {
                                Adt2::C0(_, _, _, h220, _, _, _, _, _, _, _) => {
                                    h220
                                }
                            };
                            nat_eq(a221, a222)
                        } else {
                            false
                        } {
                            let a225 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h223, _, _, _, _, _, _) => {
                                    h223
                                }
                            };
                            let a226 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h224, _, _, _, _, _, _) => {
                                    h224
                                }
                            };
                            nat_eq(a225, a226)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f14(v3.clone(), 1u64)?, 4u64, f16(v2, 1u64), f16(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h227, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h227
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h228, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h228
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h229, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h229
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h230, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h230
                            }
                        })))
                    } else {
                        Ok(Adt6::C1)
                    }
                } else {
                    Ok(Adt6::C0)
                }
            }
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f6(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m231 = v3;
    match m231 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a232 = v0;
                let a233 = 4u64;
                nat_eq(a232, a233)
            };
            let v7: bool = {
                let a235 = 0u64;
                let a236 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h234, _, _, _, _, _, _, _, _, _, _, _) => {
                        h234
                    }
                };
                nat_lt(a235, a236)
            };
            let v8: u64 = f14(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h237, _, _, _, _, _, _, _, _, _) => {
                    h237
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h238, _, _, _, _, _, _, _, _) => {
                    h238
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h239, _, _, _, _, _, _, _) => {
                    h239
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h240, _, _, _, _, _, _) => {
                    h240
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h241, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h241
                }
            }, match v1.clone() {
                Adt1::C0(_, h242, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h242
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h243, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h243
                }
            });
            if if if if if if if if if if if if v6 {
                let a245 = match v2.clone() {
                    Adt2::C0(h244, _, _, _, _, _, _, _, _, _, _) => {
                        h244
                    }
                };
                let a246 = 0u64;
                if nat_eq(a245, a246) {
                    let a248 = match v5.clone() {
                        Adt2::C0(h247, _, _, _, _, _, _, _, _, _, _) => {
                            h247
                        }
                    };
                    let a249 = 0u64;
                    nat_eq(a248, a249)
                } else {
                    false
                }
            } else {
                let a251 = match v2.clone() {
                    Adt2::C0(h250, _, _, _, _, _, _, _, _, _, _) => {
                        h250
                    }
                };
                let a252 = 0u64;
                if if nat_eq(a251, a252) {
                    true
                } else {
                    let a254 = match v2.clone() {
                        Adt2::C0(h253, _, _, _, _, _, _, _, _, _, _) => {
                            h253
                        }
                    };
                    let a255 = 26u64;
                    nat_eq(a254, a255)
                } {
                    let a257 = match v5.clone() {
                        Adt2::C0(h256, _, _, _, _, _, _, _, _, _, _) => {
                            h256
                        }
                    };
                    let a258 = 0u64;
                    if nat_eq(a257, a258) {
                        true
                    } else {
                        let a260 = match v5.clone() {
                            Adt2::C0(h259, _, _, _, _, _, _, _, _, _, _) => {
                                h259
                            }
                        };
                        let a261 = 26u64;
                        nat_eq(a260, a261)
                    }
                } else {
                    false
                }
            } {
                let a264 = match v2.clone() {
                    Adt2::C0(h262, _, _, _, _, _, _, _, _, _, _) => {
                        h262
                    }
                };
                let a265 = match v4.clone() {
                    Adt2::C0(h263, _, _, _, _, _, _, _, _, _, _) => {
                        h263
                    }
                };
                nat_eq(a264, a265)
            } else {
                false
            } {
                f22(v2.clone(), v4.clone())
            } else {
                false
            } {
                f23(v2.clone())?
            } else {
                false
            } {
                f23(v4.clone())?
            } else {
                false
            } {
                let a267 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h266, _, _, _, _, _) => {
                        h266
                    }
                };
                let a268 = 4u64;
                nat_eq(a267, a268)
            } else {
                false
            } {
                let a269 = v8;
                let a270 = f14(v2.clone(), 1u64)?;
                nat_le(a269, a270)
            } else {
                false
            } {
                let a272 = match v1.clone() {
                    Adt1::C0(_, _, _, h271, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h271
                    }
                };
                let a273 = 1073741824u64;
                nat_lt(a272, a273)
            } else {
                false
            } {
                let a275 = match v1.clone() {
                    Adt1::C0(h274, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h274
                    }
                };
                let a276 = 4294967295u64;
                if if if if nat_le(a275, a276) {
                    let a278 = match v1.clone() {
                        Adt1::C0(_, h277, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h277
                        }
                    };
                    let a279 = 4294967295u64;
                    nat_le(a278, a279)
                } else {
                    false
                } {
                    let a281 = match v1.clone() {
                        Adt1::C0(_, _, h280, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h280
                        }
                    };
                    let a282 = 4294967295u64;
                    nat_le(a281, a282)
                } else {
                    false
                } {
                    let a284 = match v1.clone() {
                        Adt1::C0(_, _, _, h283, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h283
                        }
                    };
                    let a285 = 4294967295u64;
                    nat_le(a284, a285)
                } else {
                    false
                } {
                    let a287 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h286, _, _, _, _, _, _, _, _, _, _, _) => {
                            h286
                        }
                    };
                    let a288 = 4294967295u64;
                    nat_le(a287, a288)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a291 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h289) => {
                            h289
                        }
                    };
                    let a292 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h290) => {
                            h290
                        }
                    };
                    nat_eq(a291, a292)
                } else {
                    true
                }
            } else {
                false
            } {
                f24(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h293, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h293
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h294, _) => {
                        h294
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f14(v2, 1u64)?
                };
                Ok(Adt6::C4({
                    let a295 = v10;
                    let a296 = v8;
                    nat_add(a295, a296)?
                }, 4u64, f16(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h297, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h297
                    }
                }, v6)))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f7(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a299 = match v0.clone() {
        Adt2::C0(h298, _, _, _, _, _, _, _, _, _, _) => {
            h298
        }
    };
    let a300 = 0u64;
    if if if nat_eq(a299, a300) {
        let a302 = match v1.clone() {
            Adt2::C0(h301, _, _, _, _, _, _, _, _, _, _) => {
                h301
            }
        };
        let a303 = 0u64;
        nat_eq(a302, a303)
    } else {
        false
    } {
        f17(match v0.clone() {
            Adt2::C0(_, h304, _, _, _, _, _, _, _, _, _) => {
                h304
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h305, _, _, _, _, _, _, _, _) => {
                h305
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h306, _, _, _, _, _, _, _) => {
                h306
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h307, _, _, _, _, _, _) => {
                h307
            }
        }, match v1.clone() {
            Adt2::C0(_, h308, _, _, _, _, _, _, _, _, _) => {
                h308
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h309, _, _, _, _, _, _, _, _) => {
                h309
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h310, _, _, _, _, _, _, _) => {
                h310
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h311, _, _, _, _, _, _) => {
                h311
            }
        })
    } else {
        false
    } {
        let v2: u64 = f14(v1.clone(), 1u64)?;
        Ok(Adt6::C4({
            let a312 = v2;
            let a313 = f14(v0.clone(), 1u64)?;
            nat_add(a312, a313)?
        }, 4u64, f16(v0, 1u64), f16(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f8(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a315 = match v1.clone() {
        Adt2::C0(h314, _, _, _, _, _, _, _, _, _, _) => {
            h314
        }
    };
    let a316 = 0u64;
    if if if nat_eq(a315, a316) {
        true
    } else {
        let a318 = match v1.clone() {
            Adt2::C0(h317, _, _, _, _, _, _, _, _, _, _) => {
                h317
            }
        };
        let a319 = 1u64;
        nat_eq(a318, a319)
    } {
        true
    } else {
        let a321 = match v1.clone() {
            Adt2::C0(h320, _, _, _, _, _, _, _, _, _, _) => {
                h320
            }
        };
        let a322 = 30u64;
        nat_eq(a321, a322)
    } {
        let a325 = match v1.clone() {
            Adt2::C0(h323, _, _, _, _, _, _, _, _, _, _) => {
                h323
            }
        };
        let a326 = match v2.clone() {
            Adt2::C0(h324, _, _, _, _, _, _, _, _, _, _) => {
                h324
            }
        };
        if if if if if nat_eq(a325, a326) {
            let a329 = match v1.clone() {
                Adt2::C0(_, h327, _, _, _, _, _, _, _, _, _) => {
                    h327
                }
            };
            let a330 = match v2.clone() {
                Adt2::C0(_, h328, _, _, _, _, _, _, _, _, _) => {
                    h328
                }
            };
            nat_eq(a329, a330)
        } else {
            false
        } {
            let a333 = match v1.clone() {
                Adt2::C0(_, _, _, h331, _, _, _, _, _, _, _) => {
                    h331
                }
            };
            let a334 = match v2.clone() {
                Adt2::C0(_, _, _, h332, _, _, _, _, _, _, _) => {
                    h332
                }
            };
            nat_eq(a333, a334)
        } else {
            false
        } {
            let a337 = match v1.clone() {
                Adt2::C0(_, _, _, _, h335, _, _, _, _, _, _) => {
                    h335
                }
            };
            let a338 = match v2.clone() {
                Adt2::C0(_, _, _, _, h336, _, _, _, _, _, _) => {
                    h336
                }
            };
            nat_eq(a337, a338)
        } else {
            false
        } {
            let a339 = v0;
            let a340 = 4u64;
            if nat_eq(a339, a340) {
                let a342 = match v1.clone() {
                    Adt2::C0(_, _, h341, _, _, _, _, _, _, _, _) => {
                        h341
                    }
                };
                let a343 = 1u64;
                if nat_eq(a342, a343) {
                    let a346 = match v1.clone() {
                        Adt2::C0(_, h344, _, _, _, _, _, _, _, _, _) => {
                            h344
                        }
                    };
                    let a347 = match v2.clone() {
                        Adt2::C0(_, _, h345, _, _, _, _, _, _, _, _) => {
                            h345
                        }
                    };
                    nat_eq(a346, a347)
                } else {
                    false
                }
            } else {
                let a348 = v0;
                let a349 = 4u64;
                if if nat_lt(a348, a349) {
                    let a352 = match v1.clone() {
                        Adt2::C0(_, h350, _, _, _, _, _, _, _, _, _) => {
                            h350
                        }
                    };
                    let a353 = match v1.clone() {
                        Adt2::C0(_, _, h351, _, _, _, _, _, _, _, _) => {
                            h351
                        }
                    };
                    nat_eq(a352, a353)
                } else {
                    false
                } {
                    let a356 = match v1.clone() {
                        Adt2::C0(_, _, h354, _, _, _, _, _, _, _, _) => {
                            h354
                        }
                    };
                    let a357 = match v2.clone() {
                        Adt2::C0(_, _, h355, _, _, _, _, _, _, _, _) => {
                            h355
                        }
                    };
                    nat_eq(a356, a357)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m359 = f15(match v1.clone() {
                Adt2::C0(h358, _, _, _, _, _, _, _, _, _, _) => {
                    h358
                }
            });
            match m359 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v3) => {
                    Ok(Adt6::C4(f14(v2.clone(), 1u64)?, match v3 {
                        Adt8::C0(_, _, h360) => {
                            h360
                        }
                    }, f16(v1, 1u64), f16(v2, 1u64), Adt4::C8(v0)))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f9(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if if if f25(match v1.clone() {
        Adt2::C0(h361, _, _, _, _, _, _, _, _, _, _) => {
            h361
        }
    }, v0) {
        let a364 = match v1.clone() {
            Adt2::C0(h362, _, _, _, _, _, _, _, _, _, _) => {
                h362
            }
        };
        let a365 = match v2.clone() {
            Adt2::C0(h363, _, _, _, _, _, _, _, _, _, _) => {
                h363
            }
        };
        nat_eq(a364, a365)
    } else {
        false
    } {
        let a368 = match v1.clone() {
            Adt2::C0(_, h366, _, _, _, _, _, _, _, _, _) => {
                h366
            }
        };
        let a369 = match v2.clone() {
            Adt2::C0(_, h367, _, _, _, _, _, _, _, _, _) => {
                h367
            }
        };
        if if if nat_eq(a368, a369) {
            let a372 = match v1.clone() {
                Adt2::C0(_, _, h370, _, _, _, _, _, _, _, _) => {
                    h370
                }
            };
            let a373 = match v2.clone() {
                Adt2::C0(_, _, h371, _, _, _, _, _, _, _, _) => {
                    h371
                }
            };
            nat_eq(a372, a373)
        } else {
            false
        } {
            let a376 = match v1.clone() {
                Adt2::C0(_, _, _, h374, _, _, _, _, _, _, _) => {
                    h374
                }
            };
            let a377 = match v2.clone() {
                Adt2::C0(_, _, _, h375, _, _, _, _, _, _, _) => {
                    h375
                }
            };
            nat_eq(a376, a377)
        } else {
            false
        } {
            let a380 = match v1.clone() {
                Adt2::C0(_, _, _, _, h378, _, _, _, _, _, _) => {
                    h378
                }
            };
            let a381 = match v2.clone() {
                Adt2::C0(_, _, _, _, h379, _, _, _, _, _, _) => {
                    h379
                }
            };
            nat_eq(a380, a381)
        } else {
            false
        }
    } else {
        false
    } {
        let m383 = f15(match v1.clone() {
            Adt2::C0(h382, _, _, _, _, _, _, _, _, _, _) => {
                h382
            }
        });
        match m383 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v3) => {
                Ok(Adt6::C4(f14(v2.clone(), 1u64)?, match v3 {
                    Adt8::C0(_, _, h384) => {
                        h384
                    }
                }, f16(v1.clone(), 1u64), f16(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h385, _, _, _, _, _, _, _, _, _, _) => {
                        h385
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f10(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m386 = f26(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m386 {
        Adt6::C0 => {
            Ok(Adt6::C0)
        }
        Adt6::C1 => {
            Ok(Adt6::C1)
        }
        Adt6::C2(v5, v6, _, _, _, _, v11) => {
            Ok(Adt6::C4(v11, 4u64, f16(v2.clone(), 1u64), f16(v4, 1u64), Adt4::C10(v5, v6, {
                let m387 = v3;
                match m387 {
                    None => {
                        f16(v2, 1u64)
                    }
                    Some(v12) => {
                        f16(v12, 1u64)
                    }
                }
            })))
        }
        Adt6::C3(v13, v14, v15, v16, v17, v18, v19, v20, v21, v22) => {
            Ok(Adt6::C3(v13, v14, v15, v16, v17, v18, v19, v20, v21, v22))
        }
        Adt6::C4(v23, v24, v25, v26, v27) => {
            Ok(Adt6::C4(v23, v24, v25, v26, v27))
        }
    }
}

pub fn f11(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt5> {
    let m388 = v3;
    match m388 {
        Adt4::C0 => {
            Ok(Adt5::C1(false, f27(v0, v1)?, f27(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt5::C1(false, f28(v0, v1, v2.clone())?, f27(v0, v2)?))
        }
        Adt4::C2(v4, v5) => {
            f29(v0, v1, v5, v2, v4)
        }
        Adt4::C3(v6, v7, v8, v9, v10) => {
            f30(v0, v1, v2, v6, v7, v8, v9, v10)
        }
        Adt4::C4(v11) => {
            f31(v0, v1, v2, v11)
        }
        Adt4::C5(v12, v13, v14, v15) => {
            f30(v0, v1, v2, v12, v13, v14, v15, true)
        }
        Adt4::C6(v16, v17, v18) => {
            f32(v0, v1, v2, v16, v17, v18)
        }
        Adt4::C7(v19) => {
            f33(v0, v1, v2, v19)
        }
        Adt4::C8(v20) => {
            f34(v20, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt5::C1(true, 0u64, f27(v0, v2)?))
        }
        Adt4::C10(v23, v24, v25) => {
            f35(v0, v1, v2, v23, v24, v25)
        }
        Adt4::C11(v26, v27) => {
            Ok(Adt5::C3(v26, v27, f27(v0, v1)?, f27(v0, v2)?))
        }
    }
}

pub fn f12(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a391 = match v1.clone() {
        Adt2::C0(h389, _, _, _, _, _, _, _, _, _, _) => {
            h389
        }
    };
    let a392 = match v3.clone() {
        Adt2::C0(h390, _, _, _, _, _, _, _, _, _, _) => {
            h390
        }
    };
    if if nat_eq(a391, a392) {
        let a393 = v0;
        let a394 = 33u64;
        if nat_eq(a393, a394) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m396 = f15(match v1.clone() {
            Adt2::C0(h395, _, _, _, _, _, _, _, _, _, _) => {
                h395
            }
        });
        match m396 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt8::C0(_, h397, _) => {
                        h397
                    }
                };
                let v6: u64 = f14(v1.clone(), v5)?;
                let a402 = {
                    let a399 = match v1.clone() {
                        Adt2::C0(_, h398, _, _, _, _, _, _, _, _, _) => {
                            h398
                        }
                    };
                    let a400 = v5;
                    let a401 = 1u64;
                    nat_rem(a399, a400, a401)
                };
                let a403 = 0u64;
                if if if nat_eq(a402, a403) {
                    let a408 = {
                        let a405 = match v3.clone() {
                            Adt2::C0(_, h404, _, _, _, _, _, _, _, _, _) => {
                                h404
                            }
                        };
                        let a406 = v5;
                        let a407 = 1u64;
                        nat_rem(a405, a406, a407)
                    };
                    let a409 = 0u64;
                    nat_eq(a408, a409)
                } else {
                    false
                } {
                    let a410 = v6;
                    let a411 = f14(v3.clone(), v5)?;
                    nat_eq(a410, a411)
                } else {
                    false
                } {
                    Ok(Adt6::C4(v6, match v4 {
                        Adt8::C0(_, _, h412) => {
                            h412
                        }
                    }, f16(v1, v5), f16(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt6::C1)
                }
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f13(v0: u64) -> bool {
    let a413 = v0;
    let a414 = 0u64;
    if if nat_eq(a413, a414) {
        true
    } else {
        let a415 = v0;
        let a416 = 1u64;
        nat_eq(a415, a416)
    } {
        true
    } else {
        let a417 = v0;
        let a418 = 30u64;
        nat_eq(a417, a418)
    }
}

pub fn f14(v0: Adt2, v1: u64) -> R<u64> {
    let a430 = {
        let a424 = {
            let a420 = match v0.clone() {
                Adt2::C0(_, h419, _, _, _, _, _, _, _, _, _) => {
                    h419
                }
            };
            let a421 = v1;
            let a422 = 0u64;
            nat_quot(a420, a421, a422)
        };
        let a425 = match v0.clone() {
            Adt2::C0(_, _, h423, _, _, _, _, _, _, _, _) => {
                h423
            }
        };
        nat_mul(a424, a425)?
    };
    let a431 = {
        let a428 = match v0.clone() {
            Adt2::C0(_, _, _, h426, _, _, _, _, _, _, _) => {
                h426
            }
        };
        let a429 = match v0 {
            Adt2::C0(_, _, _, _, h427, _, _, _, _, _, _) => {
                h427
            }
        };
        nat_mul(a428, a429)?
    };
    nat_mul(a430, a431)
}

pub fn f15(v0: u64) -> Adt7 {
    let a432 = v0;
    let a433 = 0u64;
    if nat_eq(a432, a433) {
        Adt7::C1(Adt8::C0(0u64, 1u64, 4u64))
    } else {
        let a434 = v0;
        let a435 = 1u64;
        if nat_eq(a434, a435) {
            Adt7::C1(Adt8::C0(1u64, 1u64, 2u64))
        } else {
            let a436 = v0;
            let a437 = 2u64;
            if nat_eq(a436, a437) {
                Adt7::C1(Adt8::C0(2u64, 32u64, 18u64))
            } else {
                let a438 = v0;
                let a439 = 3u64;
                if nat_eq(a438, a439) {
                    Adt7::C1(Adt8::C0(3u64, 32u64, 20u64))
                } else {
                    let a440 = v0;
                    let a441 = 6u64;
                    if nat_eq(a440, a441) {
                        Adt7::C1(Adt8::C0(6u64, 32u64, 22u64))
                    } else {
                        let a442 = v0;
                        let a443 = 7u64;
                        if nat_eq(a442, a443) {
                            Adt7::C1(Adt8::C0(7u64, 32u64, 24u64))
                        } else {
                            let a444 = v0;
                            let a445 = 8u64;
                            if nat_eq(a444, a445) {
                                Adt7::C1(Adt8::C0(8u64, 32u64, 34u64))
                            } else {
                                let a446 = v0;
                                let a447 = 9u64;
                                if nat_eq(a446, a447) {
                                    Adt7::C1(Adt8::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a448 = v0;
                                    let a449 = 10u64;
                                    if nat_eq(a448, a449) {
                                        Adt7::C1(Adt8::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a450 = v0;
                                        let a451 = 11u64;
                                        if nat_eq(a450, a451) {
                                            Adt7::C1(Adt8::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a452 = v0;
                                            let a453 = 12u64;
                                            if nat_eq(a452, a453) {
                                                Adt7::C1(Adt8::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a454 = v0;
                                                let a455 = 13u64;
                                                if nat_eq(a454, a455) {
                                                    Adt7::C1(Adt8::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a456 = v0;
                                                    let a457 = 14u64;
                                                    if nat_eq(a456, a457) {
                                                        Adt7::C1(Adt8::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a458 = v0;
                                                        let a459 = 15u64;
                                                        if nat_eq(a458, a459) {
                                                            Adt7::C1(Adt8::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a460 = v0;
                                                            let a461 = 16u64;
                                                            if nat_eq(a460, a461) {
                                                                Adt7::C1(Adt8::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a462 = v0;
                                                                let a463 = 17u64;
                                                                if nat_eq(a462, a463) {
                                                                    Adt7::C1(Adt8::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a464 = v0;
                                                                    let a465 = 18u64;
                                                                    if nat_eq(a464, a465) {
                                                                        Adt7::C1(Adt8::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a466 = v0;
                                                                        let a467 = 19u64;
                                                                        if nat_eq(a466, a467) {
                                                                            Adt7::C1(Adt8::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a468 = v0;
                                                                            let a469 = 20u64;
                                                                            if nat_eq(a468, a469) {
                                                                                Adt7::C1(Adt8::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a470 = v0;
                                                                                let a471 = 21u64;
                                                                                if nat_eq(a470, a471) {
                                                                                    Adt7::C1(Adt8::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a472 = v0;
                                                                                    let a473 = 22u64;
                                                                                    if nat_eq(a472, a473) {
                                                                                        Adt7::C1(Adt8::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a474 = v0;
                                                                                        let a475 = 23u64;
                                                                                        if nat_eq(a474, a475) {
                                                                                            Adt7::C1(Adt8::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a476 = v0;
                                                                                            let a477 = 24u64;
                                                                                            if nat_eq(a476, a477) {
                                                                                                Adt7::C1(Adt8::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a478 = v0;
                                                                                                let a479 = 25u64;
                                                                                                if nat_eq(a478, a479) {
                                                                                                    Adt7::C1(Adt8::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a480 = v0;
                                                                                                    let a481 = 26u64;
                                                                                                    if nat_eq(a480, a481) {
                                                                                                        Adt7::C1(Adt8::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a482 = v0;
                                                                                                        let a483 = 27u64;
                                                                                                        if nat_eq(a482, a483) {
                                                                                                            Adt7::C1(Adt8::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a484 = v0;
                                                                                                            let a485 = 28u64;
                                                                                                            if nat_eq(a484, a485) {
                                                                                                                Adt7::C1(Adt8::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a486 = v0;
                                                                                                                let a487 = 29u64;
                                                                                                                if nat_eq(a486, a487) {
                                                                                                                    Adt7::C1(Adt8::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a488 = v0;
                                                                                                                    let a489 = 30u64;
                                                                                                                    if nat_eq(a488, a489) {
                                                                                                                        Adt7::C1(Adt8::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a490 = v0;
                                                                                                                        let a491 = 34u64;
                                                                                                                        if nat_eq(a490, a491) {
                                                                                                                            Adt7::C1(Adt8::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a492 = v0;
                                                                                                                            let a493 = 35u64;
                                                                                                                            if nat_eq(a492, a493) {
                                                                                                                                Adt7::C1(Adt8::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a494 = v0;
                                                                                                                                let a495 = 39u64;
                                                                                                                                if nat_eq(a494, a495) {
                                                                                                                                    Adt7::C1(Adt8::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a496 = v0;
                                                                                                                                    let a497 = 40u64;
                                                                                                                                    if nat_eq(a496, a497) {
                                                                                                                                        Adt7::C1(Adt8::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a498 = v0;
                                                                                                                                        let a499 = 41u64;
                                                                                                                                        if nat_eq(a498, a499) {
                                                                                                                                            Adt7::C1(Adt8::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a500 = v0;
                                                                                                                                            let a501 = 42u64;
                                                                                                                                            if nat_eq(a500, a501) {
                                                                                                                                                Adt7::C1(Adt8::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a502 = v0;
                                                                                                                                                let a503 = 43u64;
                                                                                                                                                if nat_eq(a502, a503) {
                                                                                                                                                    Adt7::C1(Adt8::C0(43u64, 1u64, 1u64))
                                                                                                                                                } else {
                                                                                                                                                    Adt7::C0
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

pub fn f16(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a505 = match v0.clone() {
            Adt2::C0(_, h504, _, _, _, _, _, _, _, _, _) => {
                h504
            }
        };
        let a506 = v1;
        let a507 = 0u64;
        nat_quot(a505, a506, a507)
    }, match v0.clone() {
        Adt2::C0(_, _, h508, _, _, _, _, _, _, _, _) => {
            h508
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h509, _, _, _, _, _, _, _) => {
            h509
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h510, _, _, _, _, _, _) => {
            h510
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h511, _, _, _, _, _) => {
            h511
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h512, _, _, _, _) => {
            h512
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h513, _, _, _) => {
            h513
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h514, _, _) => {
            h514
        }
    })
}

pub fn f17(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f20(v4, v5, v6, v7) {
        f20(v0, v1, v2, v3)
    } else {
        let a518 = {
            let a515 = v0;
            let a516 = v4;
            let a517 = 1u64;
            nat_rem(a515, a516, a517)
        };
        let a519 = 0u64;
        if if if nat_eq(a518, a519) {
            let a523 = {
                let a520 = v1;
                let a521 = v5;
                let a522 = 1u64;
                nat_rem(a520, a521, a522)
            };
            let a524 = 0u64;
            nat_eq(a523, a524)
        } else {
            false
        } {
            let a528 = {
                let a525 = v2;
                let a526 = v6;
                let a527 = 1u64;
                nat_rem(a525, a526, a527)
            };
            let a529 = 0u64;
            nat_eq(a528, a529)
        } else {
            false
        } {
            let a533 = {
                let a530 = v3;
                let a531 = v7;
                let a532 = 1u64;
                nat_rem(a530, a531, a532)
            };
            let a534 = 0u64;
            nat_eq(a533, a534)
        } else {
            false
        }
    }
}

pub fn f18(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a535 = v0;
    let a536 = v1;
    if nat_eq(a535, a536) {
        let a539 = v4;
        let a540 = {
            let a537 = v2;
            let a538 = v3;
            nat_add(a537, a538)?
        };
        Ok(nat_eq(a539, a540))
    } else {
        let a541 = v2;
        let a542 = v3;
        if nat_eq(a541, a542) {
            let a543 = v4;
            let a544 = v2;
            Ok(nat_eq(a543, a544))
        } else {
            Ok(false)
        }
    }
}

pub fn f19(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a547 = {
            let a545 = v0;
            let a546 = f36(v1);
            nat_add(a545, a546)?
        };
        let a548 = f36(v2);
        nat_add(a547, a548)?
    };
    let v5: u64 = {
        let a549 = f37(v1);
        let a550 = f37(v2);
        nat_add(a549, a550)?
    };
    let a551 = v1;
    let a552 = 4294967295u64;
    if if nat_le(a551, a552) {
        let a553 = v2;
        let a554 = 4294967295u64;
        nat_le(a553, a554)
    } else {
        false
    } {
        let a555 = v5;
        let a556 = v4;
        if nat_le(a555, a556) {
            let a559 = v3;
            let a560 = {
                let a557 = v4;
                let a558 = v5;
                nat_sub(a557, a558)
            };
            Ok(nat_eq(a559, a560))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f20(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a561 = v0;
    let a562 = 0u64;
    if if if nat_eq(a561, a562) {
        true
    } else {
        let a563 = v1;
        let a564 = 0u64;
        nat_eq(a563, a564)
    } {
        true
    } else {
        let a565 = v2;
        let a566 = 0u64;
        nat_eq(a565, a566)
    } {
        true
    } else {
        let a567 = v3;
        let a568 = 0u64;
        nat_eq(a567, a568)
    }
}

pub fn f21(v0: u64, v1: u64) -> R<bool> {
    let a569 = v1;
    let a570 = 4294967295u64;
    if nat_le(a569, a570) {
        let a573 = {
            let a571 = f36(v1);
            let a572 = f37(v1);
            nat_add(a571, a572)?
        };
        let a574 = v0;
        Ok(nat_lt(a573, a574))
    } else {
        Ok(false)
    }
}

pub fn f22(v0: Adt2, v1: Adt2) -> bool {
    let a577 = match v0.clone() {
        Adt2::C0(_, h575, _, _, _, _, _, _, _, _, _) => {
            h575
        }
    };
    let a578 = match v1.clone() {
        Adt2::C0(_, h576, _, _, _, _, _, _, _, _, _) => {
            h576
        }
    };
    if if if nat_eq(a577, a578) {
        let a581 = match v0.clone() {
            Adt2::C0(_, _, h579, _, _, _, _, _, _, _, _) => {
                h579
            }
        };
        let a582 = match v1.clone() {
            Adt2::C0(_, _, h580, _, _, _, _, _, _, _, _) => {
                h580
            }
        };
        nat_eq(a581, a582)
    } else {
        false
    } {
        let a585 = match v0.clone() {
            Adt2::C0(_, _, _, h583, _, _, _, _, _, _, _) => {
                h583
            }
        };
        let a586 = match v1.clone() {
            Adt2::C0(_, _, _, h584, _, _, _, _, _, _, _) => {
                h584
            }
        };
        nat_eq(a585, a586)
    } else {
        false
    } {
        let a589 = match v0 {
            Adt2::C0(_, _, _, _, h587, _, _, _, _, _, _) => {
                h587
            }
        };
        let a590 = match v1 {
            Adt2::C0(_, _, _, _, h588, _, _, _, _, _, _) => {
                h588
            }
        };
        nat_eq(a589, a590)
    } else {
        false
    }
}

pub fn f23(v0: Adt2) -> R<bool> {
    let a592 = match v0.clone() {
        Adt2::C0(_, h591, _, _, _, _, _, _, _, _, _) => {
            h591
        }
    };
    let a593 = 1u64;
    if if if if nat_eq(a592, a593) {
        true
    } else {
        let a595 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h594, _, _, _, _, _) => {
                h594
            }
        };
        let a596 = 4u64;
        nat_eq(a595, a596)
    } {
        let a598 = match v0.clone() {
            Adt2::C0(_, _, h597, _, _, _, _, _, _, _, _) => {
                h597
            }
        };
        let a599 = 1u64;
        if nat_eq(a598, a599) {
            true
        } else {
            let a604 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h600, _, _, _, _) => {
                    h600
                }
            };
            let a605 = {
                let a602 = match v0.clone() {
                    Adt2::C0(_, h601, _, _, _, _, _, _, _, _, _) => {
                        h601
                    }
                };
                let a603 = 4u64;
                nat_mul(a602, a603)?
            };
            nat_eq(a604, a605)
        }
    } else {
        false
    } {
        let a607 = match v0.clone() {
            Adt2::C0(_, _, _, h606, _, _, _, _, _, _, _) => {
                h606
            }
        };
        let a608 = 1u64;
        if nat_eq(a607, a608) {
            true
        } else {
            let a616 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h609, _, _, _) => {
                    h609
                }
            };
            let a617 = {
                let a614 = {
                    let a612 = match v0.clone() {
                        Adt2::C0(_, h610, _, _, _, _, _, _, _, _, _) => {
                            h610
                        }
                    };
                    let a613 = match v0.clone() {
                        Adt2::C0(_, _, h611, _, _, _, _, _, _, _, _) => {
                            h611
                        }
                    };
                    nat_mul(a612, a613)?
                };
                let a615 = 4u64;
                nat_mul(a614, a615)?
            };
            nat_eq(a616, a617)
        }
    } else {
        false
    } {
        let a619 = match v0.clone() {
            Adt2::C0(_, _, _, _, h618, _, _, _, _, _, _) => {
                h618
            }
        };
        let a620 = 1u64;
        if nat_eq(a619, a620) {
            Ok(true)
        } else {
            let a631 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h621, _, _) => {
                    h621
                }
            };
            let a632 = {
                let a629 = {
                    let a627 = {
                        let a624 = match v0.clone() {
                            Adt2::C0(_, h622, _, _, _, _, _, _, _, _, _) => {
                                h622
                            }
                        };
                        let a625 = match v0.clone() {
                            Adt2::C0(_, _, h623, _, _, _, _, _, _, _, _) => {
                                h623
                            }
                        };
                        nat_mul(a624, a625)?
                    };
                    let a628 = match v0 {
                        Adt2::C0(_, _, _, h626, _, _, _, _, _, _, _) => {
                            h626
                        }
                    };
                    nat_mul(a627, a628)?
                };
                let a630 = 4u64;
                nat_mul(a629, a630)?
            };
            Ok(nat_eq(a631, a632))
        }
    } else {
        Ok(false)
    }
}

pub fn f24(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f38(v0.clone(), false)?;
    let v6: u64 = f38(v0, true)?;
    let a633 = v6;
    let a634 = v1;
    if if nat_le(a633, a634) {
        let a641 = {
            let a639 = {
                let a635 = v1;
                let a636 = v5;
                nat_add(a635, a636)?
            };
            let a640 = {
                let a637 = v3;
                let a638 = 0u64;
                if nat_eq(a637, a638) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a639, a640)?
        };
        let a642 = v2;
        nat_le(a641, a642)
    } else {
        false
    } {
        if v4 {
            let a647 = {
                let a645 = {
                    let a643 = v1;
                    let a644 = v5;
                    nat_add(a643, a644)?
                };
                let a646 = v6;
                nat_sub(a645, a646)
            };
            let a648 = v2;
            Ok(nat_lt(a647, a648))
        } else {
            let a653 = {
                let a651 = {
                    let a649 = v1;
                    let a650 = v5;
                    nat_add(a649, a650)?
                };
                let a652 = v6;
                nat_sub(a651, a652)
            };
            let a654 = v2;
            Ok(nat_le(a653, a654))
        }
    } else {
        Ok(false)
    }
}

pub fn f25(v0: u64, v1: u64) -> bool {
    let a655 = v0;
    let a656 = 0u64;
    if if nat_eq(a655, a656) {
        true
    } else {
        let a657 = v0;
        let a658 = 1u64;
        nat_eq(a657, a658)
    } {
        let a659 = v1;
        let a660 = 4294967296u64;
        nat_lt(a659, a660)
    } else {
        false
    }
}

pub fn f26(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m661 = v3;
    match m661 {
        None => {
            f39(v0, v1, match v2.clone() {
                Adt2::C0(h662, _, _, _, _, _, _, _, _, _, _) => {
                    h662
                }
            }, 0u64, match v4.clone() {
                Adt2::C0(h663, _, _, _, _, _, _, _, _, _, _) => {
                    h663
                }
            }, false, match v2.clone() {
                Adt2::C0(_, h664, _, _, _, _, _, _, _, _, _) => {
                    h664
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h665, _, _, _, _, _, _, _, _) => {
                    h665
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h666, _, _, _, _, _, _, _) => {
                    h666
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h667, _, _, _, _, _, _) => {
                    h667
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt2::C0(_, h668, _, _, _, _, _, _, _, _, _) => {
                    h668
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h669, _, _, _, _, _, _, _, _) => {
                    h669
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h670, _, _, _, _, _, _, _) => {
                    h670
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h671, _, _, _, _, _, _) => {
                    h671
                }
            })
        }
        Some(v5) => {
            f39(v0, v1, match v2.clone() {
                Adt2::C0(h672, _, _, _, _, _, _, _, _, _, _) => {
                    h672
                }
            }, match v5.clone() {
                Adt2::C0(h673, _, _, _, _, _, _, _, _, _, _) => {
                    h673
                }
            }, match v4.clone() {
                Adt2::C0(h674, _, _, _, _, _, _, _, _, _, _) => {
                    h674
                }
            }, true, match v2.clone() {
                Adt2::C0(_, h675, _, _, _, _, _, _, _, _, _) => {
                    h675
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h676, _, _, _, _, _, _, _, _) => {
                    h676
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h677, _, _, _, _, _, _, _) => {
                    h677
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h678, _, _, _, _, _, _) => {
                    h678
                }
            }, match v5.clone() {
                Adt2::C0(_, h679, _, _, _, _, _, _, _, _, _) => {
                    h679
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h680, _, _, _, _, _, _, _, _) => {
                    h680
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h681, _, _, _, _, _, _, _) => {
                    h681
                }
            }, match v5 {
                Adt2::C0(_, _, _, _, h682, _, _, _, _, _, _) => {
                    h682
                }
            }, match v4.clone() {
                Adt2::C0(_, h683, _, _, _, _, _, _, _, _, _) => {
                    h683
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h684, _, _, _, _, _, _, _, _) => {
                    h684
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h685, _, _, _, _, _, _, _) => {
                    h685
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h686, _, _, _, _, _, _) => {
                    h686
                }
            })
        }
    }
}

pub fn f27(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a688 = v0;
        let a689 = match v1.clone() {
            Adt3::C0(h687, _, _, _, _, _, _, _) => {
                h687
            }
        };
        let a690 = 0u64;
        nat_quot(a688, a689, a690)
    };
    let v3: u64 = {
        let a692 = v2;
        let a693 = match v1.clone() {
            Adt3::C0(_, h691, _, _, _, _, _, _) => {
                h691
            }
        };
        let a694 = 0u64;
        nat_quot(a692, a693, a694)
    };
    f40({
        let a696 = v0;
        let a697 = match v1.clone() {
            Adt3::C0(h695, _, _, _, _, _, _, _) => {
                h695
            }
        };
        let a698 = 0u64;
        nat_rem(a696, a697, a698)
    }, {
        let a700 = v2;
        let a701 = match v1.clone() {
            Adt3::C0(_, h699, _, _, _, _, _, _) => {
                h699
            }
        };
        let a702 = 0u64;
        nat_rem(a700, a701, a702)
    }, {
        let a704 = v3;
        let a705 = match v1.clone() {
            Adt3::C0(_, _, h703, _, _, _, _, _) => {
                h703
            }
        };
        let a706 = 0u64;
        nat_rem(a704, a705, a706)
    }, {
        let a708 = v3;
        let a709 = match v1.clone() {
            Adt3::C0(_, _, h707, _, _, _, _, _) => {
                h707
            }
        };
        let a710 = 0u64;
        nat_quot(a708, a709, a710)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h711, _, _, _) => {
            h711
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h712, _, _) => {
            h712
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h713, _) => {
            h713
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h714) => {
            h714
        }
    })
}

pub fn f28(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a716 = v0;
        let a717 = match v2.clone() {
            Adt3::C0(h715, _, _, _, _, _, _, _) => {
                h715
            }
        };
        let a718 = 0u64;
        nat_quot(a716, a717, a718)
    };
    let v4: u64 = {
        let a720 = v3;
        let a721 = match v2.clone() {
            Adt3::C0(_, h719, _, _, _, _, _, _) => {
                h719
            }
        };
        let a722 = 0u64;
        nat_quot(a720, a721, a722)
    };
    f41({
        let a724 = v0;
        let a725 = match v2.clone() {
            Adt3::C0(h723, _, _, _, _, _, _, _) => {
                h723
            }
        };
        let a726 = 0u64;
        nat_rem(a724, a725, a726)
    }, {
        let a728 = v3;
        let a729 = match v2.clone() {
            Adt3::C0(_, h727, _, _, _, _, _, _) => {
                h727
            }
        };
        let a730 = 0u64;
        nat_rem(a728, a729, a730)
    }, {
        let a732 = v4;
        let a733 = match v2.clone() {
            Adt3::C0(_, _, h731, _, _, _, _, _) => {
                h731
            }
        };
        let a734 = 0u64;
        nat_rem(a732, a733, a734)
    }, {
        let a736 = v4;
        let a737 = match v2 {
            Adt3::C0(_, _, h735, _, _, _, _, _) => {
                h735
            }
        };
        let a738 = 0u64;
        nat_quot(a736, a737, a738)
    }, match v1.clone() {
        Adt3::C0(h739, _, _, _, _, _, _, _) => {
            h739
        }
    }, match v1.clone() {
        Adt3::C0(_, h740, _, _, _, _, _, _) => {
            h740
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h741, _, _, _, _, _) => {
            h741
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h742, _, _, _, _) => {
            h742
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h743, _, _, _) => {
            h743
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h744, _, _) => {
            h744
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h745, _) => {
            h745
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h746) => {
            h746
        }
    })
}

pub fn f29(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a748 = v0;
        let a749 = match v3.clone() {
            Adt3::C0(h747, _, _, _, _, _, _, _) => {
                h747
            }
        };
        let a750 = 0u64;
        nat_quot(a748, a749, a750)
    };
    let v6: u64 = {
        let a752 = v5;
        let a753 = match v3.clone() {
            Adt3::C0(_, h751, _, _, _, _, _, _) => {
                h751
            }
        };
        let a754 = 0u64;
        nat_quot(a752, a753, a754)
    };
    let v7: u64 = {
        let a756 = v0;
        let a757 = match v3.clone() {
            Adt3::C0(h755, _, _, _, _, _, _, _) => {
                h755
            }
        };
        let a758 = 0u64;
        nat_rem(a756, a757, a758)
    };
    let v8: u64 = {
        let a760 = v5;
        let a761 = match v3.clone() {
            Adt3::C0(_, h759, _, _, _, _, _, _) => {
                h759
            }
        };
        let a762 = 0u64;
        nat_rem(a760, a761, a762)
    };
    let v9: u64 = {
        let a764 = v6;
        let a765 = match v3.clone() {
            Adt3::C0(_, _, h763, _, _, _, _, _) => {
                h763
            }
        };
        let a766 = 0u64;
        nat_rem(a764, a765, a766)
    };
    let v10: u64 = {
        let a768 = v6;
        let a769 = match v3.clone() {
            Adt3::C0(_, _, h767, _, _, _, _, _) => {
                h767
            }
        };
        let a770 = 0u64;
        nat_quot(a768, a769, a770)
    };
    let v11: u64 = {
        let a771 = v4;
        let a772 = 0u64;
        if nat_eq(a771, a772) {
            match v1.clone() {
                Adt3::C0(h773, _, _, _, _, _, _, _) => {
                    h773
                }
            }
        } else {
            let a774 = v4;
            let a775 = 1u64;
            if nat_eq(a774, a775) {
                match v1.clone() {
                    Adt3::C0(_, h776, _, _, _, _, _, _) => {
                        h776
                    }
                }
            } else {
                let a777 = v4;
                let a778 = 2u64;
                if nat_eq(a777, a778) {
                    match v1.clone() {
                        Adt3::C0(_, _, h779, _, _, _, _, _) => {
                            h779
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h780, _, _, _, _) => {
                            h780
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a787 = v11;
        let a788 = {
            let a781 = v4;
            let a782 = 0u64;
            if nat_eq(a781, a782) {
                v7
            } else {
                let a783 = v4;
                let a784 = 1u64;
                if nat_eq(a783, a784) {
                    v8
                } else {
                    let a785 = v4;
                    let a786 = 2u64;
                    if nat_eq(a785, a786) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a787, a788)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt5::C1(v12, f40(if if v12 {
        let a789 = v4;
        let a790 = 0u64;
        nat_eq(a789, a790)
    } else {
        false
    } {
        let a791 = v7;
        let a792 = v11;
        nat_sub(a791, a792)
    } else {
        v7
    }, if if v12 {
        let a793 = v4;
        let a794 = 1u64;
        nat_eq(a793, a794)
    } else {
        false
    } {
        let a795 = v8;
        let a796 = v11;
        nat_sub(a795, a796)
    } else {
        v8
    }, if if v12 {
        let a797 = v4;
        let a798 = 2u64;
        nat_eq(a797, a798)
    } else {
        false
    } {
        let a799 = v9;
        let a800 = v11;
        nat_sub(a799, a800)
    } else {
        v9
    }, if if v12 {
        let a801 = v4;
        let a802 = 3u64;
        nat_eq(a801, a802)
    } else {
        false
    } {
        let a803 = v10;
        let a804 = v11;
        nat_sub(a803, a804)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h805, _, _, _) => {
            h805
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h806, _, _) => {
            h806
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h807, _) => {
            h807
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h808) => {
            h808
        }
    })?, f27(v0, v3)?))
}

pub fn f30(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt5> {
    let v8: u64 = f42(v0, v2.clone(), 0u64);
    let v9: u64 = f42(v0, v2.clone(), 1u64);
    let v10: u64 = f42(v0, v2.clone(), 2u64);
    let v11: u64 = f42(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt5::C1(false, f43(v1.clone(), f44(v8, v3, match v1.clone() {
            Adt3::C0(h809, _, _, _, _, _, _, _) => {
                h809
            }
        })?, f44(v9, v4, match v1.clone() {
            Adt3::C0(_, h810, _, _, _, _, _, _) => {
                h810
            }
        })?, f44(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h811, _, _, _, _, _) => {
                h811
            }
        })?, f44(v11, v6, match v1 {
            Adt3::C0(_, _, _, h812, _, _, _, _) => {
                h812
            }
        })?)?, f27(v0, v2)?))
    } else {
        if if if if f45(v8, v3, match v1.clone() {
            Adt3::C0(h813, _, _, _, _, _, _, _) => {
                h813
            }
        })? {
            f45(v9, v4, match v1.clone() {
                Adt3::C0(_, h814, _, _, _, _, _, _) => {
                    h814
                }
            })?
        } else {
            false
        } {
            f45(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h815, _, _, _, _, _) => {
                    h815
                }
            })?
        } else {
            false
        } {
            f45(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h816, _, _, _, _) => {
                    h816
                }
            })?
        } else {
            false
        } {
            Ok(Adt5::C1(false, f43(v1, f46(v8, v3)?, f46(v9, v4)?, f46(v10, v5)?, f46(v11, v6)?)?, f27(v0, v2)?))
        } else {
            Ok(Adt5::C1(true, 0u64, f27(v0, v2)?))
        }
    }
}

pub fn f31(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f42(v0, v2.clone(), 0u64);
    let v5: u64 = f42(v0, v2.clone(), 1u64);
    let v6: u64 = f42(v0, v2.clone(), 2u64);
    let v7: u64 = f42(v0, v2.clone(), 3u64);
    Ok(Adt5::C1(false, f43(v1.clone(), f47(v4, v3, match v1 {
        Adt3::C0(h817, _, _, _, _, _, _, _) => {
            h817
        }
    })?, v5, v6, v7)?, f27(v0, v2)?))
}

pub fn f32(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt5> {
    let a818 = v0;
    let a819 = v3;
    if nat_lt(a818, a819) {
        Ok(Adt5::C1(false, {
            let a820 = v0;
            let a821 = 4u64;
            nat_mul(a820, a821)?
        }, {
            let a822 = v0;
            let a823 = 4u64;
            nat_mul(a822, a823)?
        }))
    } else {
        let v6: u64 = {
            let a824 = v0;
            let a825 = v3;
            nat_sub(a824, a825)
        };
        let v7: u64 = f27(v6, v1)?;
        let v8: u64 = f48(v6, v2, v4)?;
        if v5 {
            Ok(Adt5::C2(0u64, 0u64, v8, 1u64, v7, v8))
        } else {
            Ok(Adt5::C1(true, v7, v8))
        }
    }
}

pub fn f33(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a826 = v0;
    let a827 = v3;
    if nat_lt(a826, a827) {
        Ok(Adt5::C1(true, 0u64, f27(v0, v2)?))
    } else {
        let v4: u64 = {
            let a828 = v0;
            let a829 = v3;
            nat_sub(a828, a829)
        };
        let v5: u64 = f28(v4, v2, v1.clone())?;
        Ok(Adt5::C2(0u64, 2u64, v5, 0u64, f27(v4, v1)?, v5))
    }
}

pub fn f34(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt5> {
    let v4: u64 = {
        let a831 = v1;
        let a832 = match v3.clone() {
            Adt3::C0(h830, _, _, _, _, _, _, _) => {
                h830
            }
        };
        let a833 = 0u64;
        nat_rem(a831, a832, a833)
    };
    let v5: u64 = {
        let a839 = {
            let a835 = v1;
            let a836 = match v3.clone() {
                Adt3::C0(h834, _, _, _, _, _, _, _) => {
                    h834
                }
            };
            let a837 = 0u64;
            nat_quot(a835, a836, a837)
        };
        let a840 = match v3.clone() {
            Adt3::C0(_, h838, _, _, _, _, _, _) => {
                h838
            }
        };
        let a841 = 0u64;
        nat_rem(a839, a840, a841)
    };
    let v6: bool = f49(v0, v4, v5);
    Ok(Adt5::C1(!v6, if v6 {
        f28(v1, v2, v3.clone())?
    } else {
        0u64
    }, f27(v1, v3)?))
}

pub fn f35(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt5> {
    Ok(Adt5::C2(v3, 0u64, f27(v0, v1)?, if v4 {
        3u64
    } else {
        1u64
    }, if v4 {
        0u64
    } else {
        f28(v0, v5, v2.clone())?
    }, f27(v0, v2)?))
}

pub fn f36(v0: u64) -> u64 {
    let a842 = v0;
    let a843 = 2147483648u64;
    if nat_lt(a842, a843) {
        v0
    } else {
        0u64
    }
}

pub fn f37(v0: u64) -> u64 {
    let a844 = v0;
    let a845 = 2147483648u64;
    if nat_lt(a844, a845) {
        0u64
    } else {
        let a846 = 4294967296u64;
        let a847 = v0;
        nat_sub(a846, a847)
    }
}

pub fn f38(v0: Adt3, v1: bool) -> R<u64> {
    f50(v0.clone(), {
        let a849 = match v0.clone() {
            Adt3::C0(h848, _, _, _, _, _, _, _) => {
                h848
            }
        };
        let a850 = 1u64;
        nat_sub(a849, a850)
    }, {
        let a852 = match v0.clone() {
            Adt3::C0(_, h851, _, _, _, _, _, _) => {
                h851
            }
        };
        let a853 = 1u64;
        nat_sub(a852, a853)
    }, {
        let a855 = match v0.clone() {
            Adt3::C0(_, _, h854, _, _, _, _, _) => {
                h854
            }
        };
        let a856 = 1u64;
        nat_sub(a855, a856)
    }, {
        let a858 = match v0 {
            Adt3::C0(_, _, _, h857, _, _, _, _) => {
                h857
            }
        };
        let a859 = 1u64;
        nat_sub(a858, a859)
    }, v1)
}

pub fn f39(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt6> {
    let m860 = f51(v0, v1);
    match m860 {
        None => {
            if v5 {
                let m861 = f52(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m861 {
                    Adt9::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt9::C1 => {
                        Ok(Adt6::C1)
                    }
                    Adt9::C2(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27) => {
                        Ok(Adt6::C3(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27))
                    }
                }
            } else {
                Ok(Adt6::C1)
            }
        }
        Some(v28) => {
            let v29: bool = {
                let a862 = 4u64;
                let a863 = v28;
                nat_le(a862, a863)
            };
            let a864 = v2;
            let a865 = 0u64;
            if if if if if nat_eq(a864, a865) {
                let a866 = v4;
                let a867 = 0u64;
                nat_eq(a866, a867)
            } else {
                false
            } {
                let a868 = v6;
                let a869 = v14;
                if nat_eq(a868, a869) {
                    let a870 = v7;
                    let a871 = v15;
                    nat_eq(a870, a871)
                } else {
                    false
                }
            } else {
                false
            } {
                let a872 = v8;
                let a873 = v16;
                if nat_eq(a872, a873) {
                    let a874 = v9;
                    let a875 = v17;
                    nat_eq(a874, a875)
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
                        let a876 = v3;
                        let a877 = 0u64;
                        nat_eq(a876, a877)
                    } else {
                        false
                    } {
                        f17(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt6::C2(v28, v29, v6, v7, v8, v9, {
                    let a882 = {
                        let a878 = v6;
                        let a879 = v7;
                        nat_mul(a878, a879)?
                    };
                    let a883 = {
                        let a880 = v8;
                        let a881 = v9;
                        nat_mul(a880, a881)?
                    };
                    nat_mul(a882, a883)?
                }))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f40(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a896 = {
        let a892 = {
            let a888 = {
                let a884 = v0;
                let a885 = v4;
                nat_mul(a884, a885)?
            };
            let a889 = {
                let a886 = v1;
                let a887 = v5;
                nat_mul(a886, a887)?
            };
            nat_add(a888, a889)?
        };
        let a893 = {
            let a890 = v2;
            let a891 = v6;
            nat_mul(a890, a891)?
        };
        nat_add(a892, a893)?
    };
    let a897 = {
        let a894 = v3;
        let a895 = v7;
        nat_mul(a894, a895)?
    };
    nat_add(a896, a897)
}

pub fn f41(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f40({
        let a898 = v0;
        let a899 = v4;
        let a900 = 0u64;
        nat_rem(a898, a899, a900)
    }, {
        let a901 = v1;
        let a902 = v5;
        let a903 = 0u64;
        nat_rem(a901, a902, a903)
    }, {
        let a904 = v2;
        let a905 = v6;
        let a906 = 0u64;
        nat_rem(a904, a905, a906)
    }, {
        let a907 = v3;
        let a908 = v7;
        let a909 = 0u64;
        nat_rem(a907, a908, a909)
    }, v8, v9, v10, v11)
}

pub fn f42(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a911 = v0;
        let a912 = match v1.clone() {
            Adt3::C0(h910, _, _, _, _, _, _, _) => {
                h910
            }
        };
        let a913 = 0u64;
        nat_quot(a911, a912, a913)
    };
    let v4: u64 = {
        let a915 = v3;
        let a916 = match v1.clone() {
            Adt3::C0(_, h914, _, _, _, _, _, _) => {
                h914
            }
        };
        let a917 = 0u64;
        nat_quot(a915, a916, a917)
    };
    let a918 = v2;
    let a919 = 0u64;
    if nat_eq(a918, a919) {
        let a921 = v0;
        let a922 = match v1 {
            Adt3::C0(h920, _, _, _, _, _, _, _) => {
                h920
            }
        };
        let a923 = 0u64;
        nat_rem(a921, a922, a923)
    } else {
        let a924 = v2;
        let a925 = 1u64;
        if nat_eq(a924, a925) {
            let a927 = v3;
            let a928 = match v1 {
                Adt3::C0(_, h926, _, _, _, _, _, _) => {
                    h926
                }
            };
            let a929 = 0u64;
            nat_rem(a927, a928, a929)
        } else {
            let a930 = v2;
            let a931 = 2u64;
            if nat_eq(a930, a931) {
                let a933 = v4;
                let a934 = match v1 {
                    Adt3::C0(_, _, h932, _, _, _, _, _) => {
                        h932
                    }
                };
                let a935 = 0u64;
                nat_rem(a933, a934, a935)
            } else {
                let a937 = v4;
                let a938 = match v1 {
                    Adt3::C0(_, _, h936, _, _, _, _, _) => {
                        h936
                    }
                };
                let a939 = 0u64;
                nat_quot(a937, a938, a939)
            }
        }
    }
}

pub fn f43(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f40(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h940, _, _, _) => {
            h940
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h941, _, _) => {
            h941
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h942, _) => {
            h942
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h943) => {
            h943
        }
    })
}

pub fn f44(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a944 = v1;
    let a945 = 2147483648u64;
    if nat_lt(a944, a945) {
        let a956 = {
            let a954 = {
                let a949 = {
                    let a946 = v0;
                    let a947 = v2;
                    let a948 = 0u64;
                    nat_rem(a946, a947, a948)
                };
                let a950 = v2;
                nat_add(a949, a950)?
            };
            let a955 = {
                let a951 = v1;
                let a952 = v2;
                let a953 = 0u64;
                nat_rem(a951, a952, a953)
            };
            nat_sub(a954, a955)
        };
        let a957 = v2;
        let a958 = 0u64;
        Ok(nat_rem(a956, a957, a958))
    } else {
        let a961 = {
            let a959 = v0;
            let a960 = f37(v1);
            nat_add(a959, a960)?
        };
        let a962 = v2;
        let a963 = 0u64;
        Ok(nat_rem(a961, a962, a963))
    }
}

pub fn f45(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a966 = f36(v1);
    let a967 = {
        let a964 = v0;
        let a965 = f37(v1);
        nat_add(a964, a965)?
    };
    if nat_le(a966, a967) {
        let a968 = f46(v0, v1)?;
        let a969 = v2;
        Ok(nat_lt(a968, a969))
    } else {
        Ok(false)
    }
}

pub fn f46(v0: u64, v1: u64) -> R<u64> {
    let a972 = {
        let a970 = v0;
        let a971 = f37(v1);
        nat_add(a970, a971)?
    };
    let a973 = f36(v1);
    Ok(nat_sub(a972, a973))
}

pub fn f47(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a974 = v0;
    let a975 = v1;
    if nat_lt(a974, a975) {
        let a976 = v1;
        let a977 = v0;
        Ok(nat_sub(a976, a977))
    } else {
        let a980 = v0;
        let a981 = {
            let a978 = v1;
            let a979 = v2;
            nat_add(a978, a979)?
        };
        if nat_lt(a980, a981) {
            let a982 = v0;
            let a983 = v1;
            Ok(nat_sub(a982, a983))
        } else {
            let a990 = {
                let a986 = 2u64;
                let a987 = {
                    let a984 = v2;
                    let a985 = 1u64;
                    nat_sub(a984, a985)
                };
                nat_mul(a986, a987)?
            };
            let a991 = {
                let a988 = v0;
                let a989 = v1;
                nat_sub(a988, a989)
            };
            Ok(nat_sub(a990, a991))
        }
    }
}

pub fn f48(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a994 = {
        let a992 = v2;
        let a993 = f50(v1.clone(), f42(v0, v1.clone(), 0u64), f42(v0, v1.clone(), 1u64), f42(v0, v1.clone(), 2u64), f42(v0, v1.clone(), 3u64), false)?;
        nat_add(a992, a993)?
    };
    let a995 = f50(v1.clone(), f42(v0, v1.clone(), 0u64), f42(v0, v1.clone(), 1u64), f42(v0, v1.clone(), 2u64), f42(v0, v1, 3u64), true)?;
    Ok(nat_sub(a994, a995))
}

pub fn f49(v0: u64, v1: u64, v2: u64) -> bool {
    let a996 = v0;
    let a997 = 0u64;
    if nat_eq(a996, a997) {
        let a998 = v2;
        let a999 = v1;
        nat_le(a998, a999)
    } else {
        let a1000 = v0;
        let a1001 = 1u64;
        if nat_eq(a1000, a1001) {
            let a1002 = v2;
            let a1003 = v1;
            nat_lt(a1002, a1003)
        } else {
            let a1004 = v0;
            let a1005 = 2u64;
            if nat_eq(a1004, a1005) {
                let a1006 = v1;
                let a1007 = v2;
                nat_le(a1006, a1007)
            } else {
                let a1008 = v0;
                let a1009 = 3u64;
                if nat_eq(a1008, a1009) {
                    let a1010 = v1;
                    let a1011 = v2;
                    nat_lt(a1010, a1011)
                } else {
                    let a1012 = v1;
                    let a1013 = v2;
                    nat_eq(a1012, a1013)
                }
            }
        }
    }
}

pub fn f50(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a1034 = {
        let a1028 = {
            let a1022 = {
                let a1016 = v1;
                let a1017 = if v5 {
                    f37(match v0.clone() {
                        Adt3::C0(_, _, _, _, h1014, _, _, _) => {
                            h1014
                        }
                    })
                } else {
                    f36(match v0.clone() {
                        Adt3::C0(_, _, _, _, h1015, _, _, _) => {
                            h1015
                        }
                    })
                };
                nat_mul(a1016, a1017)?
            };
            let a1023 = {
                let a1020 = v2;
                let a1021 = if v5 {
                    f37(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h1018, _, _) => {
                            h1018
                        }
                    })
                } else {
                    f36(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h1019, _, _) => {
                            h1019
                        }
                    })
                };
                nat_mul(a1020, a1021)?
            };
            nat_add(a1022, a1023)?
        };
        let a1029 = {
            let a1026 = v3;
            let a1027 = if v5 {
                f37(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h1024, _) => {
                        h1024
                    }
                })
            } else {
                f36(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h1025, _) => {
                        h1025
                    }
                })
            };
            nat_mul(a1026, a1027)?
        };
        nat_add(a1028, a1029)?
    };
    let a1035 = {
        let a1032 = v4;
        let a1033 = if v5 {
            f37(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h1030) => {
                    h1030
                }
            })
        } else {
            f36(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h1031) => {
                    h1031
                }
            })
        };
        nat_mul(a1032, a1033)?
    };
    nat_add(a1034, a1035)
}

pub fn f51(v0: u64, v1: u64) -> Option<u64> {
    let a1036 = v0;
    let a1037 = 2u64;
    if nat_eq(a1036, a1037) {
        Some(0u64)
    } else {
        let a1038 = v0;
        let a1039 = 5u64;
        if nat_eq(a1038, a1039) {
            Some(1u64)
        } else {
            let a1040 = v0;
            let a1041 = 6u64;
            if nat_eq(a1040, a1041) {
                Some(2u64)
            } else {
                let a1042 = v0;
                let a1043 = 7u64;
                if nat_eq(a1042, a1043) {
                    Some(3u64)
                } else {
                    let a1044 = v0;
                    let a1045 = 9u64;
                    if nat_eq(a1044, a1045) {
                        Some(4u64)
                    } else {
                        let a1046 = v0;
                        let a1047 = 8u64;
                        if nat_eq(a1046, a1047) {
                            Some(5u64)
                        } else {
                            let a1048 = v0;
                            let a1049 = 89u64;
                            if nat_eq(a1048, a1049) {
                                let a1050 = v1;
                                let a1051 = 2u64;
                                if nat_eq(a1050, a1051) {
                                    Some(6u64)
                                } else {
                                    let a1052 = v1;
                                    let a1053 = 0u64;
                                    if nat_eq(a1052, a1053) {
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

pub fn f52(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt9> {
    let a1054 = v0;
    let a1055 = 28u64;
    if if if if nat_eq(a1054, a1055) {
        let a1056 = v1;
        let a1057 = 0u64;
        nat_eq(a1056, a1057)
    } else {
        false
    } {
        let a1058 = v2;
        let a1059 = 0u64;
        nat_eq(a1058, a1059)
    } else {
        false
    } {
        let a1060 = v3;
        let a1061 = 0u64;
        nat_eq(a1060, a1061)
    } else {
        false
    } {
        let a1062 = v4;
        let a1063 = v8;
        if if if if if if if if nat_eq(a1062, a1063) {
            let a1064 = v5;
            let a1065 = v12;
            nat_eq(a1064, a1065)
        } else {
            false
        } {
            let a1066 = v9;
            let a1067 = v13;
            nat_eq(a1066, a1067)
        } else {
            false
        } {
            let a1068 = v10;
            let a1069 = v14;
            nat_eq(a1068, a1069)
        } else {
            false
        } {
            let a1070 = v11;
            let a1071 = v15;
            nat_eq(a1070, a1071)
        } else {
            false
        } {
            let a1072 = 0u64;
            let a1073 = v6;
            nat_lt(a1072, a1073)
        } else {
            false
        } {
            let a1074 = 0u64;
            let a1075 = v7;
            nat_lt(a1074, a1075)
        } else {
            false
        } {
            let a1079 = {
                let a1076 = v10;
                let a1077 = v6;
                let a1078 = 1u64;
                nat_rem(a1076, a1077, a1078)
            };
            let a1080 = 0u64;
            if nat_eq(a1079, a1080) {
                let a1084 = {
                    let a1081 = v11;
                    let a1082 = v7;
                    let a1083 = 1u64;
                    nat_rem(a1081, a1082, a1083)
                };
                let a1085 = 0u64;
                nat_eq(a1084, a1085)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt9::C2(v5, v4, v9, v10, v11, {
                let a1086 = v10;
                let a1087 = v6;
                let a1088 = 0u64;
                nat_quot(a1086, a1087, a1088)
            }, {
                let a1089 = v11;
                let a1090 = v7;
                let a1091 = 0u64;
                nat_quot(a1089, a1090, a1091)
            }, {
                let a1092 = v5;
                let a1093 = v4;
                nat_mul(a1092, a1093)?
            }, {
                let a1094 = v4;
                let a1095 = v9;
                nat_mul(a1094, a1095)?
            }, {
                let a1096 = v5;
                let a1097 = v9;
                nat_mul(a1096, a1097)?
            }))
        } else {
            Ok(Adt9::C1)
        }
    } else {
        Ok(Adt9::C0)
    }
}
