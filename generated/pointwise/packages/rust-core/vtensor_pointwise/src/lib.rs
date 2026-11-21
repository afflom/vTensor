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
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Adt6),
    C1(bool, u64, u64),
    C2(u64, u64, u64, u64, u64, u64),
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
    if if nat_eq(a36, a37) {
        let a38 = v0;
        let a39 = 33u64;
        if nat_eq(a38, a39) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m41 = f12(match v1.clone() {
            Adt2::C0(h40, _, _, _, _, _, _, _, _, _, _) => {
                h40
            }
        });
        match m41 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt8::C0(_, h42, _) => {
                        h42
                    }
                };
                let v6: u64 = f13(v1.clone(), v5)?;
                let a47 = {
                    let a44 = match v1.clone() {
                        Adt2::C0(_, h43, _, _, _, _, _, _, _, _, _) => {
                            h43
                        }
                    };
                    let a45 = v5;
                    let a46 = 1u64;
                    nat_rem(a44, a45, a46)
                };
                let a48 = 0u64;
                if if if nat_eq(a47, a48) {
                    let a53 = {
                        let a50 = match v3.clone() {
                            Adt2::C0(_, h49, _, _, _, _, _, _, _, _, _) => {
                                h49
                            }
                        };
                        let a51 = v5;
                        let a52 = 1u64;
                        nat_rem(a50, a51, a52)
                    };
                    let a54 = 0u64;
                    nat_eq(a53, a54)
                } else {
                    false
                } {
                    let a55 = v6;
                    let a56 = f13(v3.clone(), v5)?;
                    nat_eq(a55, a56)
                } else {
                    false
                } {
                    Ok(Adt6::C4(v6, match v4 {
                        Adt8::C0(_, _, h57) => {
                            h57
                        }
                    }, f14(v1, v5), f14(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt6::C1)
                }
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f3(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a59 = match v0.clone() {
        Adt2::C0(h58, _, _, _, _, _, _, _, _, _, _) => {
            h58
        }
    };
    let a60 = 0u64;
    if if if if nat_eq(a59, a60) {
        true
    } else {
        let a62 = match v0.clone() {
            Adt2::C0(h61, _, _, _, _, _, _, _, _, _, _) => {
                h61
            }
        };
        let a63 = 1u64;
        nat_eq(a62, a63)
    } {
        true
    } else {
        let a65 = match v0.clone() {
            Adt2::C0(h64, _, _, _, _, _, _, _, _, _, _) => {
                h64
            }
        };
        let a66 = 25u64;
        if nat_eq(a65, a66) {
            true
        } else {
            let a68 = match v0.clone() {
                Adt2::C0(h67, _, _, _, _, _, _, _, _, _, _) => {
                    h67
                }
            };
            let a69 = 26u64;
            nat_eq(a68, a69)
        }
    } {
        true
    } else {
        let a71 = match v0.clone() {
            Adt2::C0(h70, _, _, _, _, _, _, _, _, _, _) => {
                h70
            }
        };
        let a72 = 30u64;
        nat_eq(a71, a72)
    } {
        let a75 = match v0.clone() {
            Adt2::C0(h73, _, _, _, _, _, _, _, _, _, _) => {
                h73
            }
        };
        let a76 = match v1.clone() {
            Adt2::C0(h74, _, _, _, _, _, _, _, _, _, _) => {
                h74
            }
        };
        if if nat_eq(a75, a76) {
            f15(match v1.clone() {
                Adt2::C0(_, h77, _, _, _, _, _, _, _, _, _) => {
                    h77
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h78, _, _, _, _, _, _, _, _) => {
                    h78
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h79, _, _, _, _, _, _, _) => {
                    h79
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h80, _, _, _, _, _, _) => {
                    h80
                }
            }, match v0.clone() {
                Adt2::C0(_, h81, _, _, _, _, _, _, _, _, _) => {
                    h81
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h82, _, _, _, _, _, _, _, _) => {
                    h82
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h83, _, _, _, _, _, _, _) => {
                    h83
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h84, _, _, _, _, _, _) => {
                    h84
                }
            })
        } else {
            false
        } {
            let m86 = f12(match v0.clone() {
                Adt2::C0(h85, _, _, _, _, _, _, _, _, _, _) => {
                    h85
                }
            });
            match m86 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v2) => {
                    Ok(Adt6::C4(f13(v1.clone(), 1u64)?, match v2 {
                        Adt8::C0(_, _, h87) => {
                            h87
                        }
                    }, f14(v0, 1u64), f14(v1, 1u64), Adt4::C1))
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
    let m88 = v2;
    match m88 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let a89 = v0;
            let a90 = 4u64;
            if if if nat_lt(a89, a90) {
                let a93 = match v1.clone() {
                    Adt2::C0(h91, _, _, _, _, _, _, _, _, _, _) => {
                        h91
                    }
                };
                let a94 = match v4.clone() {
                    Adt2::C0(h92, _, _, _, _, _, _, _, _, _, _) => {
                        h92
                    }
                };
                if nat_eq(a93, a94) {
                    let a97 = match v1.clone() {
                        Adt2::C0(h95, _, _, _, _, _, _, _, _, _, _) => {
                            h95
                        }
                    };
                    let a98 = match v3.clone() {
                        Adt2::C0(h96, _, _, _, _, _, _, _, _, _, _) => {
                            h96
                        }
                    };
                    nat_eq(a97, a98)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f16(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h99, _, _, _, _, _, _, _, _, _) => {
                        h99
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h100, _, _, _, _, _, _, _, _, _) => {
                        h100
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h101, _, _, _, _, _, _, _, _, _) => {
                        h101
                    }
                })? {
                    f16(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h102, _, _, _, _, _, _, _, _) => {
                            h102
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h103, _, _, _, _, _, _, _, _) => {
                            h103
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h104, _, _, _, _, _, _, _, _) => {
                            h104
                        }
                    })?
                } else {
                    false
                } {
                    f16(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h105, _, _, _, _, _, _, _) => {
                            h105
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h106, _, _, _, _, _, _, _) => {
                            h106
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h107, _, _, _, _, _, _, _) => {
                            h107
                        }
                    })?
                } else {
                    false
                } {
                    f16(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h108, _, _, _, _, _, _) => {
                            h108
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h109, _, _, _, _, _, _) => {
                            h109
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h110, _, _, _, _, _, _) => {
                            h110
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m112 = f12(match v1.clone() {
                    Adt2::C0(h111, _, _, _, _, _, _, _, _, _, _) => {
                        h111
                    }
                });
                match m112 {
                    Adt7::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt7::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt8::C0(_, h113, _) => {
                                h113
                            }
                        };
                        let a118 = {
                            let a115 = match v1.clone() {
                                Adt2::C0(_, h114, _, _, _, _, _, _, _, _, _) => {
                                    h114
                                }
                            };
                            let a116 = v6;
                            let a117 = 1u64;
                            nat_rem(a115, a116, a117)
                        };
                        let a119 = 0u64;
                        if if if nat_eq(a118, a119) {
                            let a124 = {
                                let a121 = match v4.clone() {
                                    Adt2::C0(_, h120, _, _, _, _, _, _, _, _, _) => {
                                        h120
                                    }
                                };
                                let a122 = v6;
                                let a123 = 1u64;
                                nat_rem(a121, a122, a123)
                            };
                            let a125 = 0u64;
                            nat_eq(a124, a125)
                        } else {
                            false
                        } {
                            let a130 = {
                                let a127 = match v3.clone() {
                                    Adt2::C0(_, h126, _, _, _, _, _, _, _, _, _) => {
                                        h126
                                    }
                                };
                                let a128 = v6;
                                let a129 = 1u64;
                                nat_rem(a127, a128, a129)
                            };
                            let a131 = 0u64;
                            nat_eq(a130, a131)
                        } else {
                            false
                        } {
                            Ok(Adt6::C4(f13(v3.clone(), v6)?, match v5 {
                                Adt8::C0(_, _, h132) => {
                                    h132
                                }
                            }, f14(v1, v6), f14(v3, v6), Adt4::C2(v0, f14(v4, v6))))
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
    let a134 = match v2.clone() {
        Adt2::C0(h133, _, _, _, _, _, _, _, _, _, _) => {
            h133
        }
    };
    let a135 = 0u64;
    if if nat_eq(a134, a135) {
        let a137 = match v3.clone() {
            Adt2::C0(h136, _, _, _, _, _, _, _, _, _, _) => {
                h136
            }
        };
        let a138 = 0u64;
        nat_eq(a137, a138)
    } else {
        false
    } {
        let a139 = v0;
        let a140 = 61u64;
        if nat_eq(a139, a140) {
            let v4: bool = {
                let a142 = 0u64;
                let a143 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h141, _, _, _, _, _, _, _) => {
                        h141
                    }
                };
                nat_lt(a142, a143)
            };
            if if if if if if f17(match v2.clone() {
                Adt2::C0(_, h144, _, _, _, _, _, _, _, _, _) => {
                    h144
                }
            }, match v1.clone() {
                Adt1::C0(h145, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h145
                }
            }, match v1.clone() {
                Adt1::C0(_, h146, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h146
                }
            }, match v3.clone() {
                Adt2::C0(_, h147, _, _, _, _, _, _, _, _, _) => {
                    h147
                }
            })? {
                f17(match v2.clone() {
                    Adt2::C0(_, _, h148, _, _, _, _, _, _, _, _) => {
                        h148
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h149, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h149
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h150, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h150
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h151, _, _, _, _, _, _, _, _) => {
                        h151
                    }
                })?
            } else {
                false
            } {
                f17(match v2.clone() {
                    Adt2::C0(_, _, _, h152, _, _, _, _, _, _, _) => {
                        h152
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h153, _, _, _, _, _, _, _, _, _, _, _) => {
                        h153
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h154, _, _, _, _, _, _, _, _, _, _) => {
                        h154
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h155, _, _, _, _, _, _, _) => {
                        h155
                    }
                })?
            } else {
                false
            } {
                f17(match v2.clone() {
                    Adt2::C0(_, _, _, _, h156, _, _, _, _, _, _) => {
                        h156
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h157, _, _, _, _, _, _, _, _, _) => {
                        h157
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h158, _, _, _, _, _, _, _, _) => {
                        h158
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h159, _, _, _, _, _, _) => {
                        h159
                    }
                })?
            } else {
                false
            } {
                let a161 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h160, _, _, _, _, _, _, _) => {
                        h160
                    }
                };
                let a162 = 4294967295u64;
                nat_le(a161, a162)
            } else {
                false
            } {
                if v4 {
                    !f18(match v2.clone() {
                        Adt2::C0(_, h163, _, _, _, _, _, _, _, _, _) => {
                            h163
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h164, _, _, _, _, _, _, _, _) => {
                            h164
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h165, _, _, _, _, _, _, _) => {
                            h165
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h166, _, _, _, _, _, _) => {
                            h166
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt6::C4(f13(v3.clone(), 1u64)?, 4u64, f14(v2, 1u64), f14(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h167, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h167
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h168, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h168
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h169, _, _, _, _, _, _, _, _, _, _, _) => {
                        h169
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h170, _, _, _, _, _, _, _, _, _) => {
                        h170
                    }
                }, v4)))
            } else {
                Ok(Adt6::C1)
            }
        } else {
            let a171 = v0;
            let a172 = 62u64;
            if nat_eq(a171, a172) {
                let a174 = match v1.clone() {
                    Adt1::C0(h173, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h173
                    }
                };
                let a175 = 2147483648u64;
                if if if if if nat_lt(a174, a175) {
                    let a177 = match v1.clone() {
                        Adt1::C0(_, h176, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h176
                        }
                    };
                    let a178 = 2147483648u64;
                    nat_lt(a177, a178)
                } else {
                    false
                } {
                    let a181 = match v1.clone() {
                        Adt1::C0(h179, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h179
                        }
                    };
                    let a182 = match v2.clone() {
                        Adt2::C0(_, h180, _, _, _, _, _, _, _, _, _) => {
                            h180
                        }
                    };
                    nat_lt(a181, a182)
                } else {
                    false
                } {
                    let a185 = match v1.clone() {
                        Adt1::C0(_, h183, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h183
                        }
                    };
                    let a186 = match v2.clone() {
                        Adt2::C0(_, h184, _, _, _, _, _, _, _, _, _) => {
                            h184
                        }
                    };
                    nat_lt(a185, a186)
                } else {
                    false
                } {
                    let a189 = match v2.clone() {
                        Adt2::C0(_, _, h187, _, _, _, _, _, _, _, _) => {
                            h187
                        }
                    };
                    let a190 = match v3.clone() {
                        Adt2::C0(_, _, h188, _, _, _, _, _, _, _, _) => {
                            h188
                        }
                    };
                    if if if nat_eq(a189, a190) {
                        let a193 = match v2.clone() {
                            Adt2::C0(_, _, _, h191, _, _, _, _, _, _, _) => {
                                h191
                            }
                        };
                        let a194 = match v3.clone() {
                            Adt2::C0(_, _, _, h192, _, _, _, _, _, _, _) => {
                                h192
                            }
                        };
                        nat_eq(a193, a194)
                    } else {
                        false
                    } {
                        let a197 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h195, _, _, _, _, _, _) => {
                                h195
                            }
                        };
                        let a198 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h196, _, _, _, _, _, _) => {
                                h196
                            }
                        };
                        nat_eq(a197, a198)
                    } else {
                        false
                    } {
                        let a207 = match v3.clone() {
                            Adt2::C0(_, h199, _, _, _, _, _, _, _, _, _) => {
                                h199
                            }
                        };
                        let a208 = {
                            let a205 = {
                                let a202 = match v2.clone() {
                                    Adt2::C0(_, h200, _, _, _, _, _, _, _, _, _) => {
                                        h200
                                    }
                                };
                                let a203 = match v1.clone() {
                                    Adt1::C0(h201, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h201
                                    }
                                };
                                nat_add(a202, a203)?
                            };
                            let a206 = match v1.clone() {
                                Adt1::C0(_, h204, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h204
                                }
                            };
                            nat_add(a205, a206)?
                        };
                        nat_eq(a207, a208)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt6::C4(f13(v3.clone(), 1u64)?, 4u64, f14(v2, 1u64), f14(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h209, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h209
                        }
                    })))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                let a210 = v0;
                let a211 = 63u64;
                if nat_eq(a210, a211) {
                    if if if if if f19(match v2.clone() {
                        Adt2::C0(_, h212, _, _, _, _, _, _, _, _, _) => {
                            h212
                        }
                    }, match v1.clone() {
                        Adt1::C0(h213, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h213
                        }
                    })? {
                        f19(match v2.clone() {
                            Adt2::C0(_, _, h214, _, _, _, _, _, _, _, _) => {
                                h214
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h215, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h215
                            }
                        })?
                    } else {
                        false
                    } {
                        f19(match v2.clone() {
                            Adt2::C0(_, _, _, h216, _, _, _, _, _, _, _) => {
                                h216
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h217, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h217
                            }
                        })?
                    } else {
                        false
                    } {
                        f19(match v2.clone() {
                            Adt2::C0(_, _, _, _, h218, _, _, _, _, _, _) => {
                                h218
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h219, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h219
                            }
                        })?
                    } else {
                        false
                    } {
                        let a222 = match v2.clone() {
                            Adt2::C0(_, h220, _, _, _, _, _, _, _, _, _) => {
                                h220
                            }
                        };
                        let a223 = match v3.clone() {
                            Adt2::C0(_, h221, _, _, _, _, _, _, _, _, _) => {
                                h221
                            }
                        };
                        if if if nat_eq(a222, a223) {
                            let a226 = match v2.clone() {
                                Adt2::C0(_, _, h224, _, _, _, _, _, _, _, _) => {
                                    h224
                                }
                            };
                            let a227 = match v3.clone() {
                                Adt2::C0(_, _, h225, _, _, _, _, _, _, _, _) => {
                                    h225
                                }
                            };
                            nat_eq(a226, a227)
                        } else {
                            false
                        } {
                            let a230 = match v2.clone() {
                                Adt2::C0(_, _, _, h228, _, _, _, _, _, _, _) => {
                                    h228
                                }
                            };
                            let a231 = match v3.clone() {
                                Adt2::C0(_, _, _, h229, _, _, _, _, _, _, _) => {
                                    h229
                                }
                            };
                            nat_eq(a230, a231)
                        } else {
                            false
                        } {
                            let a234 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h232, _, _, _, _, _, _) => {
                                    h232
                                }
                            };
                            let a235 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h233, _, _, _, _, _, _) => {
                                    h233
                                }
                            };
                            nat_eq(a234, a235)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f13(v3.clone(), 1u64)?, 4u64, f14(v2, 1u64), f14(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h236, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h236
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h237, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h237
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h238, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h238
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h239, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h239
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
    let m240 = v3;
    match m240 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a241 = v0;
                let a242 = 4u64;
                nat_eq(a241, a242)
            };
            let v7: bool = {
                let a244 = 0u64;
                let a245 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h243, _, _, _, _, _, _, _, _, _, _, _) => {
                        h243
                    }
                };
                nat_lt(a244, a245)
            };
            let v8: u64 = f13(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h246, _, _, _, _, _, _, _, _, _) => {
                    h246
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h247, _, _, _, _, _, _, _, _) => {
                    h247
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h248, _, _, _, _, _, _, _) => {
                    h248
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h249, _, _, _, _, _, _) => {
                    h249
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h250, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h250
                }
            }, match v1.clone() {
                Adt1::C0(_, h251, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h251
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h252, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h252
                }
            });
            if if if if if if if if if if if if v6 {
                let a254 = match v2.clone() {
                    Adt2::C0(h253, _, _, _, _, _, _, _, _, _, _) => {
                        h253
                    }
                };
                let a255 = 0u64;
                if nat_eq(a254, a255) {
                    let a257 = match v5.clone() {
                        Adt2::C0(h256, _, _, _, _, _, _, _, _, _, _) => {
                            h256
                        }
                    };
                    let a258 = 0u64;
                    nat_eq(a257, a258)
                } else {
                    false
                }
            } else {
                let a260 = match v2.clone() {
                    Adt2::C0(h259, _, _, _, _, _, _, _, _, _, _) => {
                        h259
                    }
                };
                let a261 = 0u64;
                if if nat_eq(a260, a261) {
                    true
                } else {
                    let a263 = match v2.clone() {
                        Adt2::C0(h262, _, _, _, _, _, _, _, _, _, _) => {
                            h262
                        }
                    };
                    let a264 = 26u64;
                    nat_eq(a263, a264)
                } {
                    let a266 = match v5.clone() {
                        Adt2::C0(h265, _, _, _, _, _, _, _, _, _, _) => {
                            h265
                        }
                    };
                    let a267 = 0u64;
                    if nat_eq(a266, a267) {
                        true
                    } else {
                        let a269 = match v5.clone() {
                            Adt2::C0(h268, _, _, _, _, _, _, _, _, _, _) => {
                                h268
                            }
                        };
                        let a270 = 26u64;
                        nat_eq(a269, a270)
                    }
                } else {
                    false
                }
            } {
                let a273 = match v2.clone() {
                    Adt2::C0(h271, _, _, _, _, _, _, _, _, _, _) => {
                        h271
                    }
                };
                let a274 = match v4.clone() {
                    Adt2::C0(h272, _, _, _, _, _, _, _, _, _, _) => {
                        h272
                    }
                };
                nat_eq(a273, a274)
            } else {
                false
            } {
                f20(v2.clone(), v4.clone())
            } else {
                false
            } {
                f21(v2.clone())?
            } else {
                false
            } {
                f21(v4.clone())?
            } else {
                false
            } {
                let a276 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h275, _, _, _, _, _) => {
                        h275
                    }
                };
                let a277 = 4u64;
                nat_eq(a276, a277)
            } else {
                false
            } {
                let a278 = v8;
                let a279 = f13(v2.clone(), 1u64)?;
                nat_le(a278, a279)
            } else {
                false
            } {
                let a281 = match v1.clone() {
                    Adt1::C0(_, _, _, h280, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h280
                    }
                };
                let a282 = 1073741824u64;
                nat_lt(a281, a282)
            } else {
                false
            } {
                let a284 = match v1.clone() {
                    Adt1::C0(h283, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h283
                    }
                };
                let a285 = 4294967295u64;
                if if if if nat_le(a284, a285) {
                    let a287 = match v1.clone() {
                        Adt1::C0(_, h286, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h286
                        }
                    };
                    let a288 = 4294967295u64;
                    nat_le(a287, a288)
                } else {
                    false
                } {
                    let a290 = match v1.clone() {
                        Adt1::C0(_, _, h289, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h289
                        }
                    };
                    let a291 = 4294967295u64;
                    nat_le(a290, a291)
                } else {
                    false
                } {
                    let a293 = match v1.clone() {
                        Adt1::C0(_, _, _, h292, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h292
                        }
                    };
                    let a294 = 4294967295u64;
                    nat_le(a293, a294)
                } else {
                    false
                } {
                    let a296 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h295, _, _, _, _, _, _, _, _, _, _, _) => {
                            h295
                        }
                    };
                    let a297 = 4294967295u64;
                    nat_le(a296, a297)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a300 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h298) => {
                            h298
                        }
                    };
                    let a301 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h299) => {
                            h299
                        }
                    };
                    nat_eq(a300, a301)
                } else {
                    true
                }
            } else {
                false
            } {
                f22(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h302, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h302
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h303, _) => {
                        h303
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f13(v2, 1u64)?
                };
                Ok(Adt6::C4({
                    let a304 = v10;
                    let a305 = v8;
                    nat_add(a304, a305)?
                }, 4u64, f14(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h306, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h306
                    }
                }, v6)))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f7(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a308 = match v0.clone() {
        Adt2::C0(h307, _, _, _, _, _, _, _, _, _, _) => {
            h307
        }
    };
    let a309 = 0u64;
    if if if nat_eq(a308, a309) {
        let a311 = match v1.clone() {
            Adt2::C0(h310, _, _, _, _, _, _, _, _, _, _) => {
                h310
            }
        };
        let a312 = 0u64;
        nat_eq(a311, a312)
    } else {
        false
    } {
        f15(match v0.clone() {
            Adt2::C0(_, h313, _, _, _, _, _, _, _, _, _) => {
                h313
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h314, _, _, _, _, _, _, _, _) => {
                h314
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h315, _, _, _, _, _, _, _) => {
                h315
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h316, _, _, _, _, _, _) => {
                h316
            }
        }, match v1.clone() {
            Adt2::C0(_, h317, _, _, _, _, _, _, _, _, _) => {
                h317
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h318, _, _, _, _, _, _, _, _) => {
                h318
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h319, _, _, _, _, _, _, _) => {
                h319
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h320, _, _, _, _, _, _) => {
                h320
            }
        })
    } else {
        false
    } {
        let v2: u64 = f13(v1.clone(), 1u64)?;
        Ok(Adt6::C4({
            let a321 = v2;
            let a322 = f13(v0.clone(), 1u64)?;
            nat_add(a321, a322)?
        }, 4u64, f14(v0, 1u64), f14(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f8(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a324 = match v1.clone() {
        Adt2::C0(h323, _, _, _, _, _, _, _, _, _, _) => {
            h323
        }
    };
    let a325 = 0u64;
    if if if nat_eq(a324, a325) {
        true
    } else {
        let a327 = match v1.clone() {
            Adt2::C0(h326, _, _, _, _, _, _, _, _, _, _) => {
                h326
            }
        };
        let a328 = 1u64;
        nat_eq(a327, a328)
    } {
        true
    } else {
        let a330 = match v1.clone() {
            Adt2::C0(h329, _, _, _, _, _, _, _, _, _, _) => {
                h329
            }
        };
        let a331 = 30u64;
        nat_eq(a330, a331)
    } {
        let a334 = match v1.clone() {
            Adt2::C0(h332, _, _, _, _, _, _, _, _, _, _) => {
                h332
            }
        };
        let a335 = match v2.clone() {
            Adt2::C0(h333, _, _, _, _, _, _, _, _, _, _) => {
                h333
            }
        };
        if if if if if nat_eq(a334, a335) {
            let a338 = match v1.clone() {
                Adt2::C0(_, h336, _, _, _, _, _, _, _, _, _) => {
                    h336
                }
            };
            let a339 = match v2.clone() {
                Adt2::C0(_, h337, _, _, _, _, _, _, _, _, _) => {
                    h337
                }
            };
            nat_eq(a338, a339)
        } else {
            false
        } {
            let a342 = match v1.clone() {
                Adt2::C0(_, _, _, h340, _, _, _, _, _, _, _) => {
                    h340
                }
            };
            let a343 = match v2.clone() {
                Adt2::C0(_, _, _, h341, _, _, _, _, _, _, _) => {
                    h341
                }
            };
            nat_eq(a342, a343)
        } else {
            false
        } {
            let a346 = match v1.clone() {
                Adt2::C0(_, _, _, _, h344, _, _, _, _, _, _) => {
                    h344
                }
            };
            let a347 = match v2.clone() {
                Adt2::C0(_, _, _, _, h345, _, _, _, _, _, _) => {
                    h345
                }
            };
            nat_eq(a346, a347)
        } else {
            false
        } {
            let a348 = v0;
            let a349 = 4u64;
            if nat_eq(a348, a349) {
                let a351 = match v1.clone() {
                    Adt2::C0(_, _, h350, _, _, _, _, _, _, _, _) => {
                        h350
                    }
                };
                let a352 = 1u64;
                if nat_eq(a351, a352) {
                    let a355 = match v1.clone() {
                        Adt2::C0(_, h353, _, _, _, _, _, _, _, _, _) => {
                            h353
                        }
                    };
                    let a356 = match v2.clone() {
                        Adt2::C0(_, _, h354, _, _, _, _, _, _, _, _) => {
                            h354
                        }
                    };
                    nat_eq(a355, a356)
                } else {
                    false
                }
            } else {
                let a357 = v0;
                let a358 = 4u64;
                if if nat_lt(a357, a358) {
                    let a361 = match v1.clone() {
                        Adt2::C0(_, h359, _, _, _, _, _, _, _, _, _) => {
                            h359
                        }
                    };
                    let a362 = match v1.clone() {
                        Adt2::C0(_, _, h360, _, _, _, _, _, _, _, _) => {
                            h360
                        }
                    };
                    nat_eq(a361, a362)
                } else {
                    false
                } {
                    let a365 = match v1.clone() {
                        Adt2::C0(_, _, h363, _, _, _, _, _, _, _, _) => {
                            h363
                        }
                    };
                    let a366 = match v2.clone() {
                        Adt2::C0(_, _, h364, _, _, _, _, _, _, _, _) => {
                            h364
                        }
                    };
                    nat_eq(a365, a366)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m368 = f12(match v1.clone() {
                Adt2::C0(h367, _, _, _, _, _, _, _, _, _, _) => {
                    h367
                }
            });
            match m368 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v3) => {
                    Ok(Adt6::C4(f13(v2.clone(), 1u64)?, match v3 {
                        Adt8::C0(_, _, h369) => {
                            h369
                        }
                    }, f14(v1, 1u64), f14(v2, 1u64), Adt4::C8(v0)))
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
    if if if f23(match v1.clone() {
        Adt2::C0(h370, _, _, _, _, _, _, _, _, _, _) => {
            h370
        }
    }, v0) {
        let a373 = match v1.clone() {
            Adt2::C0(h371, _, _, _, _, _, _, _, _, _, _) => {
                h371
            }
        };
        let a374 = match v2.clone() {
            Adt2::C0(h372, _, _, _, _, _, _, _, _, _, _) => {
                h372
            }
        };
        nat_eq(a373, a374)
    } else {
        false
    } {
        let a377 = match v1.clone() {
            Adt2::C0(_, h375, _, _, _, _, _, _, _, _, _) => {
                h375
            }
        };
        let a378 = match v2.clone() {
            Adt2::C0(_, h376, _, _, _, _, _, _, _, _, _) => {
                h376
            }
        };
        if if if nat_eq(a377, a378) {
            let a381 = match v1.clone() {
                Adt2::C0(_, _, h379, _, _, _, _, _, _, _, _) => {
                    h379
                }
            };
            let a382 = match v2.clone() {
                Adt2::C0(_, _, h380, _, _, _, _, _, _, _, _) => {
                    h380
                }
            };
            nat_eq(a381, a382)
        } else {
            false
        } {
            let a385 = match v1.clone() {
                Adt2::C0(_, _, _, h383, _, _, _, _, _, _, _) => {
                    h383
                }
            };
            let a386 = match v2.clone() {
                Adt2::C0(_, _, _, h384, _, _, _, _, _, _, _) => {
                    h384
                }
            };
            nat_eq(a385, a386)
        } else {
            false
        } {
            let a389 = match v1.clone() {
                Adt2::C0(_, _, _, _, h387, _, _, _, _, _, _) => {
                    h387
                }
            };
            let a390 = match v2.clone() {
                Adt2::C0(_, _, _, _, h388, _, _, _, _, _, _) => {
                    h388
                }
            };
            nat_eq(a389, a390)
        } else {
            false
        }
    } else {
        false
    } {
        let m392 = f12(match v1.clone() {
            Adt2::C0(h391, _, _, _, _, _, _, _, _, _, _) => {
                h391
            }
        });
        match m392 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v3) => {
                Ok(Adt6::C4(f13(v2.clone(), 1u64)?, match v3 {
                    Adt8::C0(_, _, h393) => {
                        h393
                    }
                }, f14(v1.clone(), 1u64), f14(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h394, _, _, _, _, _, _, _, _, _, _) => {
                        h394
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f10(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m395 = f24(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m395 {
        Adt6::C0 => {
            Ok(Adt6::C0)
        }
        Adt6::C1 => {
            Ok(Adt6::C1)
        }
        Adt6::C2(v5, v6, _, _, _, _, v11) => {
            Ok(Adt6::C4(v11, 4u64, f14(v2.clone(), 1u64), f14(v4, 1u64), Adt4::C10(v5, v6, {
                let m396 = v3;
                match m396 {
                    None => {
                        f14(v2, 1u64)
                    }
                    Some(v12) => {
                        f14(v12, 1u64)
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
    let m397 = v3;
    match m397 {
        Adt4::C0 => {
            Ok(Adt5::C1(false, f25(v0, v1)?, f25(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt5::C1(false, f26(v0, v1, v2.clone())?, f25(v0, v2)?))
        }
        Adt4::C2(v4, v5) => {
            f27(v0, v1, v5, v2, v4)
        }
        Adt4::C3(v6, v7, v8, v9, v10) => {
            f28(v0, v1, v2, v6, v7, v8, v9, v10)
        }
        Adt4::C4(v11) => {
            f29(v0, v1, v2, v11)
        }
        Adt4::C5(v12, v13, v14, v15) => {
            f28(v0, v1, v2, v12, v13, v14, v15, true)
        }
        Adt4::C6(v16, v17, v18) => {
            f30(v0, v1, v2, v16, v17, v18)
        }
        Adt4::C7(v19) => {
            f31(v0, v1, v2, v19)
        }
        Adt4::C8(v20) => {
            f32(v20, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt5::C1(true, 0u64, f25(v0, v2)?))
        }
        Adt4::C10(v23, v24, v25) => {
            f33(v0, v1, v2, v23, v24, v25)
        }
    }
}

pub fn f12(v0: u64) -> Adt7 {
    let a398 = v0;
    let a399 = 0u64;
    if nat_eq(a398, a399) {
        Adt7::C1(Adt8::C0(0u64, 1u64, 4u64))
    } else {
        let a400 = v0;
        let a401 = 1u64;
        if nat_eq(a400, a401) {
            Adt7::C1(Adt8::C0(1u64, 1u64, 2u64))
        } else {
            let a402 = v0;
            let a403 = 2u64;
            if nat_eq(a402, a403) {
                Adt7::C1(Adt8::C0(2u64, 32u64, 18u64))
            } else {
                let a404 = v0;
                let a405 = 3u64;
                if nat_eq(a404, a405) {
                    Adt7::C1(Adt8::C0(3u64, 32u64, 20u64))
                } else {
                    let a406 = v0;
                    let a407 = 6u64;
                    if nat_eq(a406, a407) {
                        Adt7::C1(Adt8::C0(6u64, 32u64, 22u64))
                    } else {
                        let a408 = v0;
                        let a409 = 7u64;
                        if nat_eq(a408, a409) {
                            Adt7::C1(Adt8::C0(7u64, 32u64, 24u64))
                        } else {
                            let a410 = v0;
                            let a411 = 8u64;
                            if nat_eq(a410, a411) {
                                Adt7::C1(Adt8::C0(8u64, 32u64, 34u64))
                            } else {
                                let a412 = v0;
                                let a413 = 9u64;
                                if nat_eq(a412, a413) {
                                    Adt7::C1(Adt8::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a414 = v0;
                                    let a415 = 10u64;
                                    if nat_eq(a414, a415) {
                                        Adt7::C1(Adt8::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a416 = v0;
                                        let a417 = 11u64;
                                        if nat_eq(a416, a417) {
                                            Adt7::C1(Adt8::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a418 = v0;
                                            let a419 = 12u64;
                                            if nat_eq(a418, a419) {
                                                Adt7::C1(Adt8::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a420 = v0;
                                                let a421 = 13u64;
                                                if nat_eq(a420, a421) {
                                                    Adt7::C1(Adt8::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a422 = v0;
                                                    let a423 = 14u64;
                                                    if nat_eq(a422, a423) {
                                                        Adt7::C1(Adt8::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a424 = v0;
                                                        let a425 = 15u64;
                                                        if nat_eq(a424, a425) {
                                                            Adt7::C1(Adt8::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a426 = v0;
                                                            let a427 = 16u64;
                                                            if nat_eq(a426, a427) {
                                                                Adt7::C1(Adt8::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a428 = v0;
                                                                let a429 = 17u64;
                                                                if nat_eq(a428, a429) {
                                                                    Adt7::C1(Adt8::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a430 = v0;
                                                                    let a431 = 18u64;
                                                                    if nat_eq(a430, a431) {
                                                                        Adt7::C1(Adt8::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a432 = v0;
                                                                        let a433 = 19u64;
                                                                        if nat_eq(a432, a433) {
                                                                            Adt7::C1(Adt8::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a434 = v0;
                                                                            let a435 = 20u64;
                                                                            if nat_eq(a434, a435) {
                                                                                Adt7::C1(Adt8::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a436 = v0;
                                                                                let a437 = 21u64;
                                                                                if nat_eq(a436, a437) {
                                                                                    Adt7::C1(Adt8::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a438 = v0;
                                                                                    let a439 = 22u64;
                                                                                    if nat_eq(a438, a439) {
                                                                                        Adt7::C1(Adt8::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a440 = v0;
                                                                                        let a441 = 23u64;
                                                                                        if nat_eq(a440, a441) {
                                                                                            Adt7::C1(Adt8::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a442 = v0;
                                                                                            let a443 = 24u64;
                                                                                            if nat_eq(a442, a443) {
                                                                                                Adt7::C1(Adt8::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a444 = v0;
                                                                                                let a445 = 25u64;
                                                                                                if nat_eq(a444, a445) {
                                                                                                    Adt7::C1(Adt8::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a446 = v0;
                                                                                                    let a447 = 26u64;
                                                                                                    if nat_eq(a446, a447) {
                                                                                                        Adt7::C1(Adt8::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a448 = v0;
                                                                                                        let a449 = 27u64;
                                                                                                        if nat_eq(a448, a449) {
                                                                                                            Adt7::C1(Adt8::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a450 = v0;
                                                                                                            let a451 = 28u64;
                                                                                                            if nat_eq(a450, a451) {
                                                                                                                Adt7::C1(Adt8::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a452 = v0;
                                                                                                                let a453 = 29u64;
                                                                                                                if nat_eq(a452, a453) {
                                                                                                                    Adt7::C1(Adt8::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a454 = v0;
                                                                                                                    let a455 = 30u64;
                                                                                                                    if nat_eq(a454, a455) {
                                                                                                                        Adt7::C1(Adt8::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a456 = v0;
                                                                                                                        let a457 = 34u64;
                                                                                                                        if nat_eq(a456, a457) {
                                                                                                                            Adt7::C1(Adt8::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a458 = v0;
                                                                                                                            let a459 = 35u64;
                                                                                                                            if nat_eq(a458, a459) {
                                                                                                                                Adt7::C1(Adt8::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a460 = v0;
                                                                                                                                let a461 = 39u64;
                                                                                                                                if nat_eq(a460, a461) {
                                                                                                                                    Adt7::C1(Adt8::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a462 = v0;
                                                                                                                                    let a463 = 40u64;
                                                                                                                                    if nat_eq(a462, a463) {
                                                                                                                                        Adt7::C1(Adt8::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a464 = v0;
                                                                                                                                        let a465 = 41u64;
                                                                                                                                        if nat_eq(a464, a465) {
                                                                                                                                            Adt7::C1(Adt8::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a466 = v0;
                                                                                                                                            let a467 = 42u64;
                                                                                                                                            if nat_eq(a466, a467) {
                                                                                                                                                Adt7::C1(Adt8::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a468 = v0;
                                                                                                                                                let a469 = 43u64;
                                                                                                                                                if nat_eq(a468, a469) {
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

pub fn f13(v0: Adt2, v1: u64) -> R<u64> {
    let a481 = {
        let a475 = {
            let a471 = match v0.clone() {
                Adt2::C0(_, h470, _, _, _, _, _, _, _, _, _) => {
                    h470
                }
            };
            let a472 = v1;
            let a473 = 0u64;
            nat_quot(a471, a472, a473)
        };
        let a476 = match v0.clone() {
            Adt2::C0(_, _, h474, _, _, _, _, _, _, _, _) => {
                h474
            }
        };
        nat_mul(a475, a476)?
    };
    let a482 = {
        let a479 = match v0.clone() {
            Adt2::C0(_, _, _, h477, _, _, _, _, _, _, _) => {
                h477
            }
        };
        let a480 = match v0 {
            Adt2::C0(_, _, _, _, h478, _, _, _, _, _, _) => {
                h478
            }
        };
        nat_mul(a479, a480)?
    };
    nat_mul(a481, a482)
}

pub fn f14(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a484 = match v0.clone() {
            Adt2::C0(_, h483, _, _, _, _, _, _, _, _, _) => {
                h483
            }
        };
        let a485 = v1;
        let a486 = 0u64;
        nat_quot(a484, a485, a486)
    }, match v0.clone() {
        Adt2::C0(_, _, h487, _, _, _, _, _, _, _, _) => {
            h487
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h488, _, _, _, _, _, _, _) => {
            h488
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h489, _, _, _, _, _, _) => {
            h489
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h490, _, _, _, _, _) => {
            h490
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h491, _, _, _, _) => {
            h491
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h492, _, _, _) => {
            h492
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h493, _, _) => {
            h493
        }
    })
}

pub fn f15(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f18(v4, v5, v6, v7) {
        f18(v0, v1, v2, v3)
    } else {
        let a497 = {
            let a494 = v0;
            let a495 = v4;
            let a496 = 1u64;
            nat_rem(a494, a495, a496)
        };
        let a498 = 0u64;
        if if if nat_eq(a497, a498) {
            let a502 = {
                let a499 = v1;
                let a500 = v5;
                let a501 = 1u64;
                nat_rem(a499, a500, a501)
            };
            let a503 = 0u64;
            nat_eq(a502, a503)
        } else {
            false
        } {
            let a507 = {
                let a504 = v2;
                let a505 = v6;
                let a506 = 1u64;
                nat_rem(a504, a505, a506)
            };
            let a508 = 0u64;
            nat_eq(a507, a508)
        } else {
            false
        } {
            let a512 = {
                let a509 = v3;
                let a510 = v7;
                let a511 = 1u64;
                nat_rem(a509, a510, a511)
            };
            let a513 = 0u64;
            nat_eq(a512, a513)
        } else {
            false
        }
    }
}

pub fn f16(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a514 = v0;
    let a515 = v1;
    if nat_eq(a514, a515) {
        let a518 = v4;
        let a519 = {
            let a516 = v2;
            let a517 = v3;
            nat_add(a516, a517)?
        };
        Ok(nat_eq(a518, a519))
    } else {
        let a520 = v2;
        let a521 = v3;
        if nat_eq(a520, a521) {
            let a522 = v4;
            let a523 = v2;
            Ok(nat_eq(a522, a523))
        } else {
            Ok(false)
        }
    }
}

pub fn f17(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a526 = {
            let a524 = v0;
            let a525 = f34(v1);
            nat_add(a524, a525)?
        };
        let a527 = f34(v2);
        nat_add(a526, a527)?
    };
    let v5: u64 = {
        let a528 = f35(v1);
        let a529 = f35(v2);
        nat_add(a528, a529)?
    };
    let a530 = v1;
    let a531 = 4294967295u64;
    if if nat_le(a530, a531) {
        let a532 = v2;
        let a533 = 4294967295u64;
        nat_le(a532, a533)
    } else {
        false
    } {
        let a534 = v5;
        let a535 = v4;
        if nat_le(a534, a535) {
            let a538 = v3;
            let a539 = {
                let a536 = v4;
                let a537 = v5;
                nat_sub(a536, a537)
            };
            Ok(nat_eq(a538, a539))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f18(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
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

pub fn f19(v0: u64, v1: u64) -> R<bool> {
    let a548 = v1;
    let a549 = 4294967295u64;
    if nat_le(a548, a549) {
        let a552 = {
            let a550 = f34(v1);
            let a551 = f35(v1);
            nat_add(a550, a551)?
        };
        let a553 = v0;
        Ok(nat_lt(a552, a553))
    } else {
        Ok(false)
    }
}

pub fn f20(v0: Adt2, v1: Adt2) -> bool {
    let a556 = match v0.clone() {
        Adt2::C0(_, h554, _, _, _, _, _, _, _, _, _) => {
            h554
        }
    };
    let a557 = match v1.clone() {
        Adt2::C0(_, h555, _, _, _, _, _, _, _, _, _) => {
            h555
        }
    };
    if if if nat_eq(a556, a557) {
        let a560 = match v0.clone() {
            Adt2::C0(_, _, h558, _, _, _, _, _, _, _, _) => {
                h558
            }
        };
        let a561 = match v1.clone() {
            Adt2::C0(_, _, h559, _, _, _, _, _, _, _, _) => {
                h559
            }
        };
        nat_eq(a560, a561)
    } else {
        false
    } {
        let a564 = match v0.clone() {
            Adt2::C0(_, _, _, h562, _, _, _, _, _, _, _) => {
                h562
            }
        };
        let a565 = match v1.clone() {
            Adt2::C0(_, _, _, h563, _, _, _, _, _, _, _) => {
                h563
            }
        };
        nat_eq(a564, a565)
    } else {
        false
    } {
        let a568 = match v0 {
            Adt2::C0(_, _, _, _, h566, _, _, _, _, _, _) => {
                h566
            }
        };
        let a569 = match v1 {
            Adt2::C0(_, _, _, _, h567, _, _, _, _, _, _) => {
                h567
            }
        };
        nat_eq(a568, a569)
    } else {
        false
    }
}

pub fn f21(v0: Adt2) -> R<bool> {
    let a571 = match v0.clone() {
        Adt2::C0(_, h570, _, _, _, _, _, _, _, _, _) => {
            h570
        }
    };
    let a572 = 1u64;
    if if if if nat_eq(a571, a572) {
        true
    } else {
        let a574 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h573, _, _, _, _, _) => {
                h573
            }
        };
        let a575 = 4u64;
        nat_eq(a574, a575)
    } {
        let a577 = match v0.clone() {
            Adt2::C0(_, _, h576, _, _, _, _, _, _, _, _) => {
                h576
            }
        };
        let a578 = 1u64;
        if nat_eq(a577, a578) {
            true
        } else {
            let a583 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h579, _, _, _, _) => {
                    h579
                }
            };
            let a584 = {
                let a581 = match v0.clone() {
                    Adt2::C0(_, h580, _, _, _, _, _, _, _, _, _) => {
                        h580
                    }
                };
                let a582 = 4u64;
                nat_mul(a581, a582)?
            };
            nat_eq(a583, a584)
        }
    } else {
        false
    } {
        let a586 = match v0.clone() {
            Adt2::C0(_, _, _, h585, _, _, _, _, _, _, _) => {
                h585
            }
        };
        let a587 = 1u64;
        if nat_eq(a586, a587) {
            true
        } else {
            let a595 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h588, _, _, _) => {
                    h588
                }
            };
            let a596 = {
                let a593 = {
                    let a591 = match v0.clone() {
                        Adt2::C0(_, h589, _, _, _, _, _, _, _, _, _) => {
                            h589
                        }
                    };
                    let a592 = match v0.clone() {
                        Adt2::C0(_, _, h590, _, _, _, _, _, _, _, _) => {
                            h590
                        }
                    };
                    nat_mul(a591, a592)?
                };
                let a594 = 4u64;
                nat_mul(a593, a594)?
            };
            nat_eq(a595, a596)
        }
    } else {
        false
    } {
        let a598 = match v0.clone() {
            Adt2::C0(_, _, _, _, h597, _, _, _, _, _, _) => {
                h597
            }
        };
        let a599 = 1u64;
        if nat_eq(a598, a599) {
            Ok(true)
        } else {
            let a610 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h600, _, _) => {
                    h600
                }
            };
            let a611 = {
                let a608 = {
                    let a606 = {
                        let a603 = match v0.clone() {
                            Adt2::C0(_, h601, _, _, _, _, _, _, _, _, _) => {
                                h601
                            }
                        };
                        let a604 = match v0.clone() {
                            Adt2::C0(_, _, h602, _, _, _, _, _, _, _, _) => {
                                h602
                            }
                        };
                        nat_mul(a603, a604)?
                    };
                    let a607 = match v0 {
                        Adt2::C0(_, _, _, h605, _, _, _, _, _, _, _) => {
                            h605
                        }
                    };
                    nat_mul(a606, a607)?
                };
                let a609 = 4u64;
                nat_mul(a608, a609)?
            };
            Ok(nat_eq(a610, a611))
        }
    } else {
        Ok(false)
    }
}

pub fn f22(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f36(v0.clone(), false)?;
    let v6: u64 = f36(v0, true)?;
    let a612 = v6;
    let a613 = v1;
    if if nat_le(a612, a613) {
        let a620 = {
            let a618 = {
                let a614 = v1;
                let a615 = v5;
                nat_add(a614, a615)?
            };
            let a619 = {
                let a616 = v3;
                let a617 = 0u64;
                if nat_eq(a616, a617) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a618, a619)?
        };
        let a621 = v2;
        nat_le(a620, a621)
    } else {
        false
    } {
        if v4 {
            let a626 = {
                let a624 = {
                    let a622 = v1;
                    let a623 = v5;
                    nat_add(a622, a623)?
                };
                let a625 = v6;
                nat_sub(a624, a625)
            };
            let a627 = v2;
            Ok(nat_lt(a626, a627))
        } else {
            let a632 = {
                let a630 = {
                    let a628 = v1;
                    let a629 = v5;
                    nat_add(a628, a629)?
                };
                let a631 = v6;
                nat_sub(a630, a631)
            };
            let a633 = v2;
            Ok(nat_le(a632, a633))
        }
    } else {
        Ok(false)
    }
}

pub fn f23(v0: u64, v1: u64) -> bool {
    let a634 = v0;
    let a635 = 0u64;
    if if nat_eq(a634, a635) {
        true
    } else {
        let a636 = v0;
        let a637 = 1u64;
        nat_eq(a636, a637)
    } {
        let a638 = v1;
        let a639 = 4294967296u64;
        nat_lt(a638, a639)
    } else {
        false
    }
}

pub fn f24(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m640 = v3;
    match m640 {
        None => {
            f37(v0, v1, match v2.clone() {
                Adt2::C0(h641, _, _, _, _, _, _, _, _, _, _) => {
                    h641
                }
            }, 0u64, match v4.clone() {
                Adt2::C0(h642, _, _, _, _, _, _, _, _, _, _) => {
                    h642
                }
            }, false, match v2.clone() {
                Adt2::C0(_, h643, _, _, _, _, _, _, _, _, _) => {
                    h643
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h644, _, _, _, _, _, _, _, _) => {
                    h644
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h645, _, _, _, _, _, _, _) => {
                    h645
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h646, _, _, _, _, _, _) => {
                    h646
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt2::C0(_, h647, _, _, _, _, _, _, _, _, _) => {
                    h647
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h648, _, _, _, _, _, _, _, _) => {
                    h648
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h649, _, _, _, _, _, _, _) => {
                    h649
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h650, _, _, _, _, _, _) => {
                    h650
                }
            })
        }
        Some(v5) => {
            f37(v0, v1, match v2.clone() {
                Adt2::C0(h651, _, _, _, _, _, _, _, _, _, _) => {
                    h651
                }
            }, match v5.clone() {
                Adt2::C0(h652, _, _, _, _, _, _, _, _, _, _) => {
                    h652
                }
            }, match v4.clone() {
                Adt2::C0(h653, _, _, _, _, _, _, _, _, _, _) => {
                    h653
                }
            }, true, match v2.clone() {
                Adt2::C0(_, h654, _, _, _, _, _, _, _, _, _) => {
                    h654
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h655, _, _, _, _, _, _, _, _) => {
                    h655
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h656, _, _, _, _, _, _, _) => {
                    h656
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h657, _, _, _, _, _, _) => {
                    h657
                }
            }, match v5.clone() {
                Adt2::C0(_, h658, _, _, _, _, _, _, _, _, _) => {
                    h658
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h659, _, _, _, _, _, _, _, _) => {
                    h659
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h660, _, _, _, _, _, _, _) => {
                    h660
                }
            }, match v5 {
                Adt2::C0(_, _, _, _, h661, _, _, _, _, _, _) => {
                    h661
                }
            }, match v4.clone() {
                Adt2::C0(_, h662, _, _, _, _, _, _, _, _, _) => {
                    h662
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h663, _, _, _, _, _, _, _, _) => {
                    h663
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h664, _, _, _, _, _, _, _) => {
                    h664
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h665, _, _, _, _, _, _) => {
                    h665
                }
            })
        }
    }
}

pub fn f25(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a667 = v0;
        let a668 = match v1.clone() {
            Adt3::C0(h666, _, _, _, _, _, _, _) => {
                h666
            }
        };
        let a669 = 0u64;
        nat_quot(a667, a668, a669)
    };
    let v3: u64 = {
        let a671 = v2;
        let a672 = match v1.clone() {
            Adt3::C0(_, h670, _, _, _, _, _, _) => {
                h670
            }
        };
        let a673 = 0u64;
        nat_quot(a671, a672, a673)
    };
    f38({
        let a675 = v0;
        let a676 = match v1.clone() {
            Adt3::C0(h674, _, _, _, _, _, _, _) => {
                h674
            }
        };
        let a677 = 0u64;
        nat_rem(a675, a676, a677)
    }, {
        let a679 = v2;
        let a680 = match v1.clone() {
            Adt3::C0(_, h678, _, _, _, _, _, _) => {
                h678
            }
        };
        let a681 = 0u64;
        nat_rem(a679, a680, a681)
    }, {
        let a683 = v3;
        let a684 = match v1.clone() {
            Adt3::C0(_, _, h682, _, _, _, _, _) => {
                h682
            }
        };
        let a685 = 0u64;
        nat_rem(a683, a684, a685)
    }, {
        let a687 = v3;
        let a688 = match v1.clone() {
            Adt3::C0(_, _, h686, _, _, _, _, _) => {
                h686
            }
        };
        let a689 = 0u64;
        nat_quot(a687, a688, a689)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h690, _, _, _) => {
            h690
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h691, _, _) => {
            h691
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h692, _) => {
            h692
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h693) => {
            h693
        }
    })
}

pub fn f26(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a695 = v0;
        let a696 = match v2.clone() {
            Adt3::C0(h694, _, _, _, _, _, _, _) => {
                h694
            }
        };
        let a697 = 0u64;
        nat_quot(a695, a696, a697)
    };
    let v4: u64 = {
        let a699 = v3;
        let a700 = match v2.clone() {
            Adt3::C0(_, h698, _, _, _, _, _, _) => {
                h698
            }
        };
        let a701 = 0u64;
        nat_quot(a699, a700, a701)
    };
    f39({
        let a703 = v0;
        let a704 = match v2.clone() {
            Adt3::C0(h702, _, _, _, _, _, _, _) => {
                h702
            }
        };
        let a705 = 0u64;
        nat_rem(a703, a704, a705)
    }, {
        let a707 = v3;
        let a708 = match v2.clone() {
            Adt3::C0(_, h706, _, _, _, _, _, _) => {
                h706
            }
        };
        let a709 = 0u64;
        nat_rem(a707, a708, a709)
    }, {
        let a711 = v4;
        let a712 = match v2.clone() {
            Adt3::C0(_, _, h710, _, _, _, _, _) => {
                h710
            }
        };
        let a713 = 0u64;
        nat_rem(a711, a712, a713)
    }, {
        let a715 = v4;
        let a716 = match v2 {
            Adt3::C0(_, _, h714, _, _, _, _, _) => {
                h714
            }
        };
        let a717 = 0u64;
        nat_quot(a715, a716, a717)
    }, match v1.clone() {
        Adt3::C0(h718, _, _, _, _, _, _, _) => {
            h718
        }
    }, match v1.clone() {
        Adt3::C0(_, h719, _, _, _, _, _, _) => {
            h719
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h720, _, _, _, _, _) => {
            h720
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h721, _, _, _, _) => {
            h721
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h722, _, _, _) => {
            h722
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h723, _, _) => {
            h723
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h724, _) => {
            h724
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h725) => {
            h725
        }
    })
}

pub fn f27(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a727 = v0;
        let a728 = match v3.clone() {
            Adt3::C0(h726, _, _, _, _, _, _, _) => {
                h726
            }
        };
        let a729 = 0u64;
        nat_quot(a727, a728, a729)
    };
    let v6: u64 = {
        let a731 = v5;
        let a732 = match v3.clone() {
            Adt3::C0(_, h730, _, _, _, _, _, _) => {
                h730
            }
        };
        let a733 = 0u64;
        nat_quot(a731, a732, a733)
    };
    let v7: u64 = {
        let a735 = v0;
        let a736 = match v3.clone() {
            Adt3::C0(h734, _, _, _, _, _, _, _) => {
                h734
            }
        };
        let a737 = 0u64;
        nat_rem(a735, a736, a737)
    };
    let v8: u64 = {
        let a739 = v5;
        let a740 = match v3.clone() {
            Adt3::C0(_, h738, _, _, _, _, _, _) => {
                h738
            }
        };
        let a741 = 0u64;
        nat_rem(a739, a740, a741)
    };
    let v9: u64 = {
        let a743 = v6;
        let a744 = match v3.clone() {
            Adt3::C0(_, _, h742, _, _, _, _, _) => {
                h742
            }
        };
        let a745 = 0u64;
        nat_rem(a743, a744, a745)
    };
    let v10: u64 = {
        let a747 = v6;
        let a748 = match v3.clone() {
            Adt3::C0(_, _, h746, _, _, _, _, _) => {
                h746
            }
        };
        let a749 = 0u64;
        nat_quot(a747, a748, a749)
    };
    let v11: u64 = {
        let a750 = v4;
        let a751 = 0u64;
        if nat_eq(a750, a751) {
            match v1.clone() {
                Adt3::C0(h752, _, _, _, _, _, _, _) => {
                    h752
                }
            }
        } else {
            let a753 = v4;
            let a754 = 1u64;
            if nat_eq(a753, a754) {
                match v1.clone() {
                    Adt3::C0(_, h755, _, _, _, _, _, _) => {
                        h755
                    }
                }
            } else {
                let a756 = v4;
                let a757 = 2u64;
                if nat_eq(a756, a757) {
                    match v1.clone() {
                        Adt3::C0(_, _, h758, _, _, _, _, _) => {
                            h758
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h759, _, _, _, _) => {
                            h759
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a766 = v11;
        let a767 = {
            let a760 = v4;
            let a761 = 0u64;
            if nat_eq(a760, a761) {
                v7
            } else {
                let a762 = v4;
                let a763 = 1u64;
                if nat_eq(a762, a763) {
                    v8
                } else {
                    let a764 = v4;
                    let a765 = 2u64;
                    if nat_eq(a764, a765) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a766, a767)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt5::C1(v12, f38(if if v12 {
        let a768 = v4;
        let a769 = 0u64;
        nat_eq(a768, a769)
    } else {
        false
    } {
        let a770 = v7;
        let a771 = v11;
        nat_sub(a770, a771)
    } else {
        v7
    }, if if v12 {
        let a772 = v4;
        let a773 = 1u64;
        nat_eq(a772, a773)
    } else {
        false
    } {
        let a774 = v8;
        let a775 = v11;
        nat_sub(a774, a775)
    } else {
        v8
    }, if if v12 {
        let a776 = v4;
        let a777 = 2u64;
        nat_eq(a776, a777)
    } else {
        false
    } {
        let a778 = v9;
        let a779 = v11;
        nat_sub(a778, a779)
    } else {
        v9
    }, if if v12 {
        let a780 = v4;
        let a781 = 3u64;
        nat_eq(a780, a781)
    } else {
        false
    } {
        let a782 = v10;
        let a783 = v11;
        nat_sub(a782, a783)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h784, _, _, _) => {
            h784
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h785, _, _) => {
            h785
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h786, _) => {
            h786
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h787) => {
            h787
        }
    })?, f25(v0, v3)?))
}

pub fn f28(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt5> {
    let v8: u64 = f40(v0, v2.clone(), 0u64);
    let v9: u64 = f40(v0, v2.clone(), 1u64);
    let v10: u64 = f40(v0, v2.clone(), 2u64);
    let v11: u64 = f40(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt5::C1(false, f41(v1.clone(), f42(v8, v3, match v1.clone() {
            Adt3::C0(h788, _, _, _, _, _, _, _) => {
                h788
            }
        })?, f42(v9, v4, match v1.clone() {
            Adt3::C0(_, h789, _, _, _, _, _, _) => {
                h789
            }
        })?, f42(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h790, _, _, _, _, _) => {
                h790
            }
        })?, f42(v11, v6, match v1 {
            Adt3::C0(_, _, _, h791, _, _, _, _) => {
                h791
            }
        })?)?, f25(v0, v2)?))
    } else {
        if if if if f43(v8, v3, match v1.clone() {
            Adt3::C0(h792, _, _, _, _, _, _, _) => {
                h792
            }
        })? {
            f43(v9, v4, match v1.clone() {
                Adt3::C0(_, h793, _, _, _, _, _, _) => {
                    h793
                }
            })?
        } else {
            false
        } {
            f43(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h794, _, _, _, _, _) => {
                    h794
                }
            })?
        } else {
            false
        } {
            f43(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h795, _, _, _, _) => {
                    h795
                }
            })?
        } else {
            false
        } {
            Ok(Adt5::C1(false, f41(v1, f44(v8, v3)?, f44(v9, v4)?, f44(v10, v5)?, f44(v11, v6)?)?, f25(v0, v2)?))
        } else {
            Ok(Adt5::C1(true, 0u64, f25(v0, v2)?))
        }
    }
}

pub fn f29(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f40(v0, v2.clone(), 0u64);
    let v5: u64 = f40(v0, v2.clone(), 1u64);
    let v6: u64 = f40(v0, v2.clone(), 2u64);
    let v7: u64 = f40(v0, v2.clone(), 3u64);
    Ok(Adt5::C1(false, f41(v1.clone(), f45(v4, v3, match v1 {
        Adt3::C0(h796, _, _, _, _, _, _, _) => {
            h796
        }
    })?, v5, v6, v7)?, f25(v0, v2)?))
}

pub fn f30(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt5> {
    let a797 = v0;
    let a798 = v3;
    if nat_lt(a797, a798) {
        Ok(Adt5::C1(false, {
            let a799 = v0;
            let a800 = 4u64;
            nat_mul(a799, a800)?
        }, {
            let a801 = v0;
            let a802 = 4u64;
            nat_mul(a801, a802)?
        }))
    } else {
        let v6: u64 = {
            let a803 = v0;
            let a804 = v3;
            nat_sub(a803, a804)
        };
        let v7: u64 = f25(v6, v1)?;
        let v8: u64 = f46(v6, v2, v4)?;
        if v5 {
            Ok(Adt5::C2(0u64, 0u64, v8, 1u64, v7, v8))
        } else {
            Ok(Adt5::C1(true, v7, v8))
        }
    }
}

pub fn f31(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a805 = v0;
    let a806 = v3;
    if nat_lt(a805, a806) {
        Ok(Adt5::C1(true, 0u64, f25(v0, v2)?))
    } else {
        let v4: u64 = {
            let a807 = v0;
            let a808 = v3;
            nat_sub(a807, a808)
        };
        let v5: u64 = f26(v4, v2, v1.clone())?;
        Ok(Adt5::C2(0u64, 2u64, v5, 0u64, f25(v4, v1)?, v5))
    }
}

pub fn f32(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt5> {
    let v4: u64 = {
        let a810 = v1;
        let a811 = match v3.clone() {
            Adt3::C0(h809, _, _, _, _, _, _, _) => {
                h809
            }
        };
        let a812 = 0u64;
        nat_rem(a810, a811, a812)
    };
    let v5: u64 = {
        let a818 = {
            let a814 = v1;
            let a815 = match v3.clone() {
                Adt3::C0(h813, _, _, _, _, _, _, _) => {
                    h813
                }
            };
            let a816 = 0u64;
            nat_quot(a814, a815, a816)
        };
        let a819 = match v3.clone() {
            Adt3::C0(_, h817, _, _, _, _, _, _) => {
                h817
            }
        };
        let a820 = 0u64;
        nat_rem(a818, a819, a820)
    };
    let v6: bool = f47(v0, v4, v5);
    Ok(Adt5::C1(!v6, if v6 {
        f26(v1, v2, v3.clone())?
    } else {
        0u64
    }, f25(v1, v3)?))
}

pub fn f33(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt5> {
    Ok(Adt5::C2(v3, 0u64, f25(v0, v1)?, if v4 {
        3u64
    } else {
        1u64
    }, if v4 {
        0u64
    } else {
        f26(v0, v5, v2.clone())?
    }, f25(v0, v2)?))
}

pub fn f34(v0: u64) -> u64 {
    let a821 = v0;
    let a822 = 2147483648u64;
    if nat_lt(a821, a822) {
        v0
    } else {
        0u64
    }
}

pub fn f35(v0: u64) -> u64 {
    let a823 = v0;
    let a824 = 2147483648u64;
    if nat_lt(a823, a824) {
        0u64
    } else {
        let a825 = 4294967296u64;
        let a826 = v0;
        nat_sub(a825, a826)
    }
}

pub fn f36(v0: Adt3, v1: bool) -> R<u64> {
    f48(v0.clone(), {
        let a828 = match v0.clone() {
            Adt3::C0(h827, _, _, _, _, _, _, _) => {
                h827
            }
        };
        let a829 = 1u64;
        nat_sub(a828, a829)
    }, {
        let a831 = match v0.clone() {
            Adt3::C0(_, h830, _, _, _, _, _, _) => {
                h830
            }
        };
        let a832 = 1u64;
        nat_sub(a831, a832)
    }, {
        let a834 = match v0.clone() {
            Adt3::C0(_, _, h833, _, _, _, _, _) => {
                h833
            }
        };
        let a835 = 1u64;
        nat_sub(a834, a835)
    }, {
        let a837 = match v0 {
            Adt3::C0(_, _, _, h836, _, _, _, _) => {
                h836
            }
        };
        let a838 = 1u64;
        nat_sub(a837, a838)
    }, v1)
}

pub fn f37(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt6> {
    let m839 = f49(v0, v1);
    match m839 {
        None => {
            if v5 {
                let m840 = f50(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m840 {
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
                let a841 = 4u64;
                let a842 = v28;
                nat_le(a841, a842)
            };
            let a843 = v2;
            let a844 = 0u64;
            if if if if if nat_eq(a843, a844) {
                let a845 = v4;
                let a846 = 0u64;
                nat_eq(a845, a846)
            } else {
                false
            } {
                let a847 = v6;
                let a848 = v14;
                if nat_eq(a847, a848) {
                    let a849 = v7;
                    let a850 = v15;
                    nat_eq(a849, a850)
                } else {
                    false
                }
            } else {
                false
            } {
                let a851 = v8;
                let a852 = v16;
                if nat_eq(a851, a852) {
                    let a853 = v9;
                    let a854 = v17;
                    nat_eq(a853, a854)
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
                        let a855 = v3;
                        let a856 = 0u64;
                        nat_eq(a855, a856)
                    } else {
                        false
                    } {
                        f15(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt6::C2(v28, v29, v6, v7, v8, v9, {
                    let a861 = {
                        let a857 = v6;
                        let a858 = v7;
                        nat_mul(a857, a858)?
                    };
                    let a862 = {
                        let a859 = v8;
                        let a860 = v9;
                        nat_mul(a859, a860)?
                    };
                    nat_mul(a861, a862)?
                }))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f38(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a875 = {
        let a871 = {
            let a867 = {
                let a863 = v0;
                let a864 = v4;
                nat_mul(a863, a864)?
            };
            let a868 = {
                let a865 = v1;
                let a866 = v5;
                nat_mul(a865, a866)?
            };
            nat_add(a867, a868)?
        };
        let a872 = {
            let a869 = v2;
            let a870 = v6;
            nat_mul(a869, a870)?
        };
        nat_add(a871, a872)?
    };
    let a876 = {
        let a873 = v3;
        let a874 = v7;
        nat_mul(a873, a874)?
    };
    nat_add(a875, a876)
}

pub fn f39(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f38({
        let a877 = v0;
        let a878 = v4;
        let a879 = 0u64;
        nat_rem(a877, a878, a879)
    }, {
        let a880 = v1;
        let a881 = v5;
        let a882 = 0u64;
        nat_rem(a880, a881, a882)
    }, {
        let a883 = v2;
        let a884 = v6;
        let a885 = 0u64;
        nat_rem(a883, a884, a885)
    }, {
        let a886 = v3;
        let a887 = v7;
        let a888 = 0u64;
        nat_rem(a886, a887, a888)
    }, v8, v9, v10, v11)
}

pub fn f40(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a890 = v0;
        let a891 = match v1.clone() {
            Adt3::C0(h889, _, _, _, _, _, _, _) => {
                h889
            }
        };
        let a892 = 0u64;
        nat_quot(a890, a891, a892)
    };
    let v4: u64 = {
        let a894 = v3;
        let a895 = match v1.clone() {
            Adt3::C0(_, h893, _, _, _, _, _, _) => {
                h893
            }
        };
        let a896 = 0u64;
        nat_quot(a894, a895, a896)
    };
    let a897 = v2;
    let a898 = 0u64;
    if nat_eq(a897, a898) {
        let a900 = v0;
        let a901 = match v1 {
            Adt3::C0(h899, _, _, _, _, _, _, _) => {
                h899
            }
        };
        let a902 = 0u64;
        nat_rem(a900, a901, a902)
    } else {
        let a903 = v2;
        let a904 = 1u64;
        if nat_eq(a903, a904) {
            let a906 = v3;
            let a907 = match v1 {
                Adt3::C0(_, h905, _, _, _, _, _, _) => {
                    h905
                }
            };
            let a908 = 0u64;
            nat_rem(a906, a907, a908)
        } else {
            let a909 = v2;
            let a910 = 2u64;
            if nat_eq(a909, a910) {
                let a912 = v4;
                let a913 = match v1 {
                    Adt3::C0(_, _, h911, _, _, _, _, _) => {
                        h911
                    }
                };
                let a914 = 0u64;
                nat_rem(a912, a913, a914)
            } else {
                let a916 = v4;
                let a917 = match v1 {
                    Adt3::C0(_, _, h915, _, _, _, _, _) => {
                        h915
                    }
                };
                let a918 = 0u64;
                nat_quot(a916, a917, a918)
            }
        }
    }
}

pub fn f41(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f38(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h919, _, _, _) => {
            h919
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h920, _, _) => {
            h920
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h921, _) => {
            h921
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h922) => {
            h922
        }
    })
}

pub fn f42(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a923 = v1;
    let a924 = 2147483648u64;
    if nat_lt(a923, a924) {
        let a935 = {
            let a933 = {
                let a928 = {
                    let a925 = v0;
                    let a926 = v2;
                    let a927 = 0u64;
                    nat_rem(a925, a926, a927)
                };
                let a929 = v2;
                nat_add(a928, a929)?
            };
            let a934 = {
                let a930 = v1;
                let a931 = v2;
                let a932 = 0u64;
                nat_rem(a930, a931, a932)
            };
            nat_sub(a933, a934)
        };
        let a936 = v2;
        let a937 = 0u64;
        Ok(nat_rem(a935, a936, a937))
    } else {
        let a940 = {
            let a938 = v0;
            let a939 = f35(v1);
            nat_add(a938, a939)?
        };
        let a941 = v2;
        let a942 = 0u64;
        Ok(nat_rem(a940, a941, a942))
    }
}

pub fn f43(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a945 = f34(v1);
    let a946 = {
        let a943 = v0;
        let a944 = f35(v1);
        nat_add(a943, a944)?
    };
    if nat_le(a945, a946) {
        let a947 = f44(v0, v1)?;
        let a948 = v2;
        Ok(nat_lt(a947, a948))
    } else {
        Ok(false)
    }
}

pub fn f44(v0: u64, v1: u64) -> R<u64> {
    let a951 = {
        let a949 = v0;
        let a950 = f35(v1);
        nat_add(a949, a950)?
    };
    let a952 = f34(v1);
    Ok(nat_sub(a951, a952))
}

pub fn f45(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a953 = v0;
    let a954 = v1;
    if nat_lt(a953, a954) {
        let a955 = v1;
        let a956 = v0;
        Ok(nat_sub(a955, a956))
    } else {
        let a959 = v0;
        let a960 = {
            let a957 = v1;
            let a958 = v2;
            nat_add(a957, a958)?
        };
        if nat_lt(a959, a960) {
            let a961 = v0;
            let a962 = v1;
            Ok(nat_sub(a961, a962))
        } else {
            let a969 = {
                let a965 = 2u64;
                let a966 = {
                    let a963 = v2;
                    let a964 = 1u64;
                    nat_sub(a963, a964)
                };
                nat_mul(a965, a966)?
            };
            let a970 = {
                let a967 = v0;
                let a968 = v1;
                nat_sub(a967, a968)
            };
            Ok(nat_sub(a969, a970))
        }
    }
}

pub fn f46(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a973 = {
        let a971 = v2;
        let a972 = f48(v1.clone(), f40(v0, v1.clone(), 0u64), f40(v0, v1.clone(), 1u64), f40(v0, v1.clone(), 2u64), f40(v0, v1.clone(), 3u64), false)?;
        nat_add(a971, a972)?
    };
    let a974 = f48(v1.clone(), f40(v0, v1.clone(), 0u64), f40(v0, v1.clone(), 1u64), f40(v0, v1.clone(), 2u64), f40(v0, v1, 3u64), true)?;
    Ok(nat_sub(a973, a974))
}

pub fn f47(v0: u64, v1: u64, v2: u64) -> bool {
    let a975 = v0;
    let a976 = 0u64;
    if nat_eq(a975, a976) {
        let a977 = v2;
        let a978 = v1;
        nat_le(a977, a978)
    } else {
        let a979 = v0;
        let a980 = 1u64;
        if nat_eq(a979, a980) {
            let a981 = v2;
            let a982 = v1;
            nat_lt(a981, a982)
        } else {
            let a983 = v0;
            let a984 = 2u64;
            if nat_eq(a983, a984) {
                let a985 = v1;
                let a986 = v2;
                nat_le(a985, a986)
            } else {
                let a987 = v0;
                let a988 = 3u64;
                if nat_eq(a987, a988) {
                    let a989 = v1;
                    let a990 = v2;
                    nat_lt(a989, a990)
                } else {
                    let a991 = v1;
                    let a992 = v2;
                    nat_eq(a991, a992)
                }
            }
        }
    }
}

pub fn f48(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a1013 = {
        let a1007 = {
            let a1001 = {
                let a995 = v1;
                let a996 = if v5 {
                    f35(match v0.clone() {
                        Adt3::C0(_, _, _, _, h993, _, _, _) => {
                            h993
                        }
                    })
                } else {
                    f34(match v0.clone() {
                        Adt3::C0(_, _, _, _, h994, _, _, _) => {
                            h994
                        }
                    })
                };
                nat_mul(a995, a996)?
            };
            let a1002 = {
                let a999 = v2;
                let a1000 = if v5 {
                    f35(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h997, _, _) => {
                            h997
                        }
                    })
                } else {
                    f34(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h998, _, _) => {
                            h998
                        }
                    })
                };
                nat_mul(a999, a1000)?
            };
            nat_add(a1001, a1002)?
        };
        let a1008 = {
            let a1005 = v3;
            let a1006 = if v5 {
                f35(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h1003, _) => {
                        h1003
                    }
                })
            } else {
                f34(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h1004, _) => {
                        h1004
                    }
                })
            };
            nat_mul(a1005, a1006)?
        };
        nat_add(a1007, a1008)?
    };
    let a1014 = {
        let a1011 = v4;
        let a1012 = if v5 {
            f35(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h1009) => {
                    h1009
                }
            })
        } else {
            f34(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h1010) => {
                    h1010
                }
            })
        };
        nat_mul(a1011, a1012)?
    };
    nat_add(a1013, a1014)
}

pub fn f49(v0: u64, v1: u64) -> Option<u64> {
    let a1015 = v0;
    let a1016 = 2u64;
    if nat_eq(a1015, a1016) {
        Some(0u64)
    } else {
        let a1017 = v0;
        let a1018 = 5u64;
        if nat_eq(a1017, a1018) {
            Some(1u64)
        } else {
            let a1019 = v0;
            let a1020 = 6u64;
            if nat_eq(a1019, a1020) {
                Some(2u64)
            } else {
                let a1021 = v0;
                let a1022 = 7u64;
                if nat_eq(a1021, a1022) {
                    Some(3u64)
                } else {
                    let a1023 = v0;
                    let a1024 = 9u64;
                    if nat_eq(a1023, a1024) {
                        Some(4u64)
                    } else {
                        let a1025 = v0;
                        let a1026 = 8u64;
                        if nat_eq(a1025, a1026) {
                            Some(5u64)
                        } else {
                            let a1027 = v0;
                            let a1028 = 89u64;
                            if nat_eq(a1027, a1028) {
                                let a1029 = v1;
                                let a1030 = 2u64;
                                if nat_eq(a1029, a1030) {
                                    Some(6u64)
                                } else {
                                    let a1031 = v1;
                                    let a1032 = 0u64;
                                    if nat_eq(a1031, a1032) {
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

pub fn f50(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt9> {
    let a1033 = v0;
    let a1034 = 28u64;
    if if if if nat_eq(a1033, a1034) {
        let a1035 = v1;
        let a1036 = 0u64;
        nat_eq(a1035, a1036)
    } else {
        false
    } {
        let a1037 = v2;
        let a1038 = 0u64;
        nat_eq(a1037, a1038)
    } else {
        false
    } {
        let a1039 = v3;
        let a1040 = 0u64;
        nat_eq(a1039, a1040)
    } else {
        false
    } {
        let a1041 = v4;
        let a1042 = v8;
        if if if if if if if if nat_eq(a1041, a1042) {
            let a1043 = v5;
            let a1044 = v12;
            nat_eq(a1043, a1044)
        } else {
            false
        } {
            let a1045 = v9;
            let a1046 = v13;
            nat_eq(a1045, a1046)
        } else {
            false
        } {
            let a1047 = v10;
            let a1048 = v14;
            nat_eq(a1047, a1048)
        } else {
            false
        } {
            let a1049 = v11;
            let a1050 = v15;
            nat_eq(a1049, a1050)
        } else {
            false
        } {
            let a1051 = 0u64;
            let a1052 = v6;
            nat_lt(a1051, a1052)
        } else {
            false
        } {
            let a1053 = 0u64;
            let a1054 = v7;
            nat_lt(a1053, a1054)
        } else {
            false
        } {
            let a1058 = {
                let a1055 = v10;
                let a1056 = v6;
                let a1057 = 1u64;
                nat_rem(a1055, a1056, a1057)
            };
            let a1059 = 0u64;
            if nat_eq(a1058, a1059) {
                let a1063 = {
                    let a1060 = v11;
                    let a1061 = v7;
                    let a1062 = 1u64;
                    nat_rem(a1060, a1061, a1062)
                };
                let a1064 = 0u64;
                nat_eq(a1063, a1064)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt9::C2(v5, v4, v9, v10, v11, {
                let a1065 = v10;
                let a1066 = v6;
                let a1067 = 0u64;
                nat_quot(a1065, a1066, a1067)
            }, {
                let a1068 = v11;
                let a1069 = v7;
                let a1070 = 0u64;
                nat_quot(a1068, a1069, a1070)
            }, {
                let a1071 = v5;
                let a1072 = v4;
                nat_mul(a1071, a1072)?
            }, {
                let a1073 = v4;
                let a1074 = v9;
                nat_mul(a1073, a1074)?
            }, {
                let a1075 = v5;
                let a1076 = v9;
                nat_mul(a1075, a1076)?
            }))
        } else {
            Ok(Adt9::C1)
        }
    } else {
        Ok(Adt9::C0)
    }
}

pub fn run(p0: Adt0) -> R<Adt5> {
    f0(p0)
}
