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
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Adt6),
    C1(bool, u64, u64),
    C2(u64, u64),
    C3(u64, u64),
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
                                    f8(v1, v6, v3, v4, v5)?
                                }
                            }
                        }
                    }
                }
            }))
        }
        Adt0::C1(v8, v9, v10, v11) => {
            f9(v8, v9, v10, v11)
        }
    }
}

pub fn f1(v0: u64) -> bool {
    let a20 = v0;
    let a21 = 1u64;
    if if nat_eq(a20, a21) {
        true
    } else {
        let a22 = v0;
        let a23 = 33u64;
        nat_eq(a22, a23)
    } {
        true
    } else {
        let a24 = v0;
        let a25 = 34u64;
        nat_eq(a24, a25)
    }
}

pub fn f2(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a28 = match v1.clone() {
        Adt2::C0(h26, _, _, _, _, _, _, _, _, _, _) => {
            h26
        }
    };
    let a29 = match v3.clone() {
        Adt2::C0(h27, _, _, _, _, _, _, _, _, _, _) => {
            h27
        }
    };
    if if nat_eq(a28, a29) {
        let a30 = v0;
        let a31 = 33u64;
        if nat_eq(a30, a31) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m33 = f10(match v1.clone() {
            Adt2::C0(h32, _, _, _, _, _, _, _, _, _, _) => {
                h32
            }
        });
        match m33 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt8::C0(_, h34, _) => {
                        h34
                    }
                };
                let v6: u64 = f11(v1.clone(), v5)?;
                let a39 = {
                    let a36 = match v1.clone() {
                        Adt2::C0(_, h35, _, _, _, _, _, _, _, _, _) => {
                            h35
                        }
                    };
                    let a37 = v5;
                    let a38 = 1u64;
                    nat_rem(a36, a37, a38)
                };
                let a40 = 0u64;
                if if if nat_eq(a39, a40) {
                    let a45 = {
                        let a42 = match v3.clone() {
                            Adt2::C0(_, h41, _, _, _, _, _, _, _, _, _) => {
                                h41
                            }
                        };
                        let a43 = v5;
                        let a44 = 1u64;
                        nat_rem(a42, a43, a44)
                    };
                    let a46 = 0u64;
                    nat_eq(a45, a46)
                } else {
                    false
                } {
                    let a47 = v6;
                    let a48 = f11(v3.clone(), v5)?;
                    nat_eq(a47, a48)
                } else {
                    false
                } {
                    Ok(Adt6::C4(v6, match v4 {
                        Adt8::C0(_, _, h49) => {
                            h49
                        }
                    }, f12(v1, v5), f12(v3, v5), Adt4::C0))
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
    let a51 = match v0.clone() {
        Adt2::C0(h50, _, _, _, _, _, _, _, _, _, _) => {
            h50
        }
    };
    let a52 = 0u64;
    if if if if nat_eq(a51, a52) {
        true
    } else {
        let a54 = match v0.clone() {
            Adt2::C0(h53, _, _, _, _, _, _, _, _, _, _) => {
                h53
            }
        };
        let a55 = 1u64;
        nat_eq(a54, a55)
    } {
        true
    } else {
        let a57 = match v0.clone() {
            Adt2::C0(h56, _, _, _, _, _, _, _, _, _, _) => {
                h56
            }
        };
        let a58 = 25u64;
        if nat_eq(a57, a58) {
            true
        } else {
            let a60 = match v0.clone() {
                Adt2::C0(h59, _, _, _, _, _, _, _, _, _, _) => {
                    h59
                }
            };
            let a61 = 26u64;
            nat_eq(a60, a61)
        }
    } {
        true
    } else {
        let a63 = match v0.clone() {
            Adt2::C0(h62, _, _, _, _, _, _, _, _, _, _) => {
                h62
            }
        };
        let a64 = 30u64;
        nat_eq(a63, a64)
    } {
        let a67 = match v0.clone() {
            Adt2::C0(h65, _, _, _, _, _, _, _, _, _, _) => {
                h65
            }
        };
        let a68 = match v1.clone() {
            Adt2::C0(h66, _, _, _, _, _, _, _, _, _, _) => {
                h66
            }
        };
        if if nat_eq(a67, a68) {
            f13(match v1.clone() {
                Adt2::C0(_, h69, _, _, _, _, _, _, _, _, _) => {
                    h69
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h70, _, _, _, _, _, _, _, _) => {
                    h70
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h71, _, _, _, _, _, _, _) => {
                    h71
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h72, _, _, _, _, _, _) => {
                    h72
                }
            }, match v0.clone() {
                Adt2::C0(_, h73, _, _, _, _, _, _, _, _, _) => {
                    h73
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h74, _, _, _, _, _, _, _, _) => {
                    h74
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h75, _, _, _, _, _, _, _) => {
                    h75
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h76, _, _, _, _, _, _) => {
                    h76
                }
            })
        } else {
            false
        } {
            let m78 = f10(match v0.clone() {
                Adt2::C0(h77, _, _, _, _, _, _, _, _, _, _) => {
                    h77
                }
            });
            match m78 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v2) => {
                    Ok(Adt6::C4(f11(v1.clone(), 1u64)?, match v2 {
                        Adt8::C0(_, _, h79) => {
                            h79
                        }
                    }, f12(v0, 1u64), f12(v1, 1u64), Adt4::C1))
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
    let m80 = v2;
    match m80 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let a81 = v0;
            let a82 = 4u64;
            if if if nat_lt(a81, a82) {
                let a85 = match v1.clone() {
                    Adt2::C0(h83, _, _, _, _, _, _, _, _, _, _) => {
                        h83
                    }
                };
                let a86 = match v4.clone() {
                    Adt2::C0(h84, _, _, _, _, _, _, _, _, _, _) => {
                        h84
                    }
                };
                if nat_eq(a85, a86) {
                    let a89 = match v1.clone() {
                        Adt2::C0(h87, _, _, _, _, _, _, _, _, _, _) => {
                            h87
                        }
                    };
                    let a90 = match v3.clone() {
                        Adt2::C0(h88, _, _, _, _, _, _, _, _, _, _) => {
                            h88
                        }
                    };
                    nat_eq(a89, a90)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f14(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h91, _, _, _, _, _, _, _, _, _) => {
                        h91
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h92, _, _, _, _, _, _, _, _, _) => {
                        h92
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h93, _, _, _, _, _, _, _, _, _) => {
                        h93
                    }
                })? {
                    f14(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h94, _, _, _, _, _, _, _, _) => {
                            h94
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h95, _, _, _, _, _, _, _, _) => {
                            h95
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h96, _, _, _, _, _, _, _, _) => {
                            h96
                        }
                    })?
                } else {
                    false
                } {
                    f14(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h97, _, _, _, _, _, _, _) => {
                            h97
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h98, _, _, _, _, _, _, _) => {
                            h98
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h99, _, _, _, _, _, _, _) => {
                            h99
                        }
                    })?
                } else {
                    false
                } {
                    f14(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h100, _, _, _, _, _, _) => {
                            h100
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h101, _, _, _, _, _, _) => {
                            h101
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h102, _, _, _, _, _, _) => {
                            h102
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m104 = f10(match v1.clone() {
                    Adt2::C0(h103, _, _, _, _, _, _, _, _, _, _) => {
                        h103
                    }
                });
                match m104 {
                    Adt7::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt7::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt8::C0(_, h105, _) => {
                                h105
                            }
                        };
                        let a110 = {
                            let a107 = match v1.clone() {
                                Adt2::C0(_, h106, _, _, _, _, _, _, _, _, _) => {
                                    h106
                                }
                            };
                            let a108 = v6;
                            let a109 = 1u64;
                            nat_rem(a107, a108, a109)
                        };
                        let a111 = 0u64;
                        if if if nat_eq(a110, a111) {
                            let a116 = {
                                let a113 = match v4.clone() {
                                    Adt2::C0(_, h112, _, _, _, _, _, _, _, _, _) => {
                                        h112
                                    }
                                };
                                let a114 = v6;
                                let a115 = 1u64;
                                nat_rem(a113, a114, a115)
                            };
                            let a117 = 0u64;
                            nat_eq(a116, a117)
                        } else {
                            false
                        } {
                            let a122 = {
                                let a119 = match v3.clone() {
                                    Adt2::C0(_, h118, _, _, _, _, _, _, _, _, _) => {
                                        h118
                                    }
                                };
                                let a120 = v6;
                                let a121 = 1u64;
                                nat_rem(a119, a120, a121)
                            };
                            let a123 = 0u64;
                            nat_eq(a122, a123)
                        } else {
                            false
                        } {
                            Ok(Adt6::C4(f11(v3.clone(), v6)?, match v5 {
                                Adt8::C0(_, _, h124) => {
                                    h124
                                }
                            }, f12(v1, v6), f12(v3, v6), Adt4::C2(v0, f12(v4, v6))))
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
    let a126 = match v2.clone() {
        Adt2::C0(h125, _, _, _, _, _, _, _, _, _, _) => {
            h125
        }
    };
    let a127 = 0u64;
    if if nat_eq(a126, a127) {
        let a129 = match v3.clone() {
            Adt2::C0(h128, _, _, _, _, _, _, _, _, _, _) => {
                h128
            }
        };
        let a130 = 0u64;
        nat_eq(a129, a130)
    } else {
        false
    } {
        let a131 = v0;
        let a132 = 61u64;
        if nat_eq(a131, a132) {
            let v4: bool = {
                let a134 = 0u64;
                let a135 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h133, _, _, _, _, _, _, _) => {
                        h133
                    }
                };
                nat_lt(a134, a135)
            };
            if if if if if if f15(match v2.clone() {
                Adt2::C0(_, h136, _, _, _, _, _, _, _, _, _) => {
                    h136
                }
            }, match v1.clone() {
                Adt1::C0(h137, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h137
                }
            }, match v1.clone() {
                Adt1::C0(_, h138, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h138
                }
            }, match v3.clone() {
                Adt2::C0(_, h139, _, _, _, _, _, _, _, _, _) => {
                    h139
                }
            })? {
                f15(match v2.clone() {
                    Adt2::C0(_, _, h140, _, _, _, _, _, _, _, _) => {
                        h140
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h141, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h141
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h142, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h142
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h143, _, _, _, _, _, _, _, _) => {
                        h143
                    }
                })?
            } else {
                false
            } {
                f15(match v2.clone() {
                    Adt2::C0(_, _, _, h144, _, _, _, _, _, _, _) => {
                        h144
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h145, _, _, _, _, _, _, _, _, _, _, _) => {
                        h145
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h146, _, _, _, _, _, _, _, _, _, _) => {
                        h146
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h147, _, _, _, _, _, _, _) => {
                        h147
                    }
                })?
            } else {
                false
            } {
                f15(match v2.clone() {
                    Adt2::C0(_, _, _, _, h148, _, _, _, _, _, _) => {
                        h148
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h149, _, _, _, _, _, _, _, _, _) => {
                        h149
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h150, _, _, _, _, _, _, _, _) => {
                        h150
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h151, _, _, _, _, _, _) => {
                        h151
                    }
                })?
            } else {
                false
            } {
                let a153 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h152, _, _, _, _, _, _, _) => {
                        h152
                    }
                };
                let a154 = 4294967295u64;
                nat_le(a153, a154)
            } else {
                false
            } {
                if v4 {
                    !f16(match v2.clone() {
                        Adt2::C0(_, h155, _, _, _, _, _, _, _, _, _) => {
                            h155
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h156, _, _, _, _, _, _, _, _) => {
                            h156
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h157, _, _, _, _, _, _, _) => {
                            h157
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h158, _, _, _, _, _, _) => {
                            h158
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt6::C4(f11(v3.clone(), 1u64)?, 4u64, f12(v2, 1u64), f12(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h159, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h159
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h160, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h160
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h161, _, _, _, _, _, _, _, _, _, _, _) => {
                        h161
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h162, _, _, _, _, _, _, _, _, _) => {
                        h162
                    }
                }, v4)))
            } else {
                Ok(Adt6::C1)
            }
        } else {
            let a163 = v0;
            let a164 = 62u64;
            if nat_eq(a163, a164) {
                let a166 = match v1.clone() {
                    Adt1::C0(h165, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h165
                    }
                };
                let a167 = 2147483648u64;
                if if if if if nat_lt(a166, a167) {
                    let a169 = match v1.clone() {
                        Adt1::C0(_, h168, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h168
                        }
                    };
                    let a170 = 2147483648u64;
                    nat_lt(a169, a170)
                } else {
                    false
                } {
                    let a173 = match v1.clone() {
                        Adt1::C0(h171, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h171
                        }
                    };
                    let a174 = match v2.clone() {
                        Adt2::C0(_, h172, _, _, _, _, _, _, _, _, _) => {
                            h172
                        }
                    };
                    nat_lt(a173, a174)
                } else {
                    false
                } {
                    let a177 = match v1.clone() {
                        Adt1::C0(_, h175, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h175
                        }
                    };
                    let a178 = match v2.clone() {
                        Adt2::C0(_, h176, _, _, _, _, _, _, _, _, _) => {
                            h176
                        }
                    };
                    nat_lt(a177, a178)
                } else {
                    false
                } {
                    let a181 = match v2.clone() {
                        Adt2::C0(_, _, h179, _, _, _, _, _, _, _, _) => {
                            h179
                        }
                    };
                    let a182 = match v3.clone() {
                        Adt2::C0(_, _, h180, _, _, _, _, _, _, _, _) => {
                            h180
                        }
                    };
                    if if if nat_eq(a181, a182) {
                        let a185 = match v2.clone() {
                            Adt2::C0(_, _, _, h183, _, _, _, _, _, _, _) => {
                                h183
                            }
                        };
                        let a186 = match v3.clone() {
                            Adt2::C0(_, _, _, h184, _, _, _, _, _, _, _) => {
                                h184
                            }
                        };
                        nat_eq(a185, a186)
                    } else {
                        false
                    } {
                        let a189 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h187, _, _, _, _, _, _) => {
                                h187
                            }
                        };
                        let a190 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h188, _, _, _, _, _, _) => {
                                h188
                            }
                        };
                        nat_eq(a189, a190)
                    } else {
                        false
                    } {
                        let a199 = match v3.clone() {
                            Adt2::C0(_, h191, _, _, _, _, _, _, _, _, _) => {
                                h191
                            }
                        };
                        let a200 = {
                            let a197 = {
                                let a194 = match v2.clone() {
                                    Adt2::C0(_, h192, _, _, _, _, _, _, _, _, _) => {
                                        h192
                                    }
                                };
                                let a195 = match v1.clone() {
                                    Adt1::C0(h193, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h193
                                    }
                                };
                                nat_add(a194, a195)?
                            };
                            let a198 = match v1.clone() {
                                Adt1::C0(_, h196, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h196
                                }
                            };
                            nat_add(a197, a198)?
                        };
                        nat_eq(a199, a200)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt6::C4(f11(v3.clone(), 1u64)?, 4u64, f12(v2, 1u64), f12(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h201, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h201
                        }
                    })))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                let a202 = v0;
                let a203 = 63u64;
                if nat_eq(a202, a203) {
                    if if if if if f17(match v2.clone() {
                        Adt2::C0(_, h204, _, _, _, _, _, _, _, _, _) => {
                            h204
                        }
                    }, match v1.clone() {
                        Adt1::C0(h205, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h205
                        }
                    })? {
                        f17(match v2.clone() {
                            Adt2::C0(_, _, h206, _, _, _, _, _, _, _, _) => {
                                h206
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h207, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h207
                            }
                        })?
                    } else {
                        false
                    } {
                        f17(match v2.clone() {
                            Adt2::C0(_, _, _, h208, _, _, _, _, _, _, _) => {
                                h208
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h209, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h209
                            }
                        })?
                    } else {
                        false
                    } {
                        f17(match v2.clone() {
                            Adt2::C0(_, _, _, _, h210, _, _, _, _, _, _) => {
                                h210
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h211, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h211
                            }
                        })?
                    } else {
                        false
                    } {
                        let a214 = match v2.clone() {
                            Adt2::C0(_, h212, _, _, _, _, _, _, _, _, _) => {
                                h212
                            }
                        };
                        let a215 = match v3.clone() {
                            Adt2::C0(_, h213, _, _, _, _, _, _, _, _, _) => {
                                h213
                            }
                        };
                        if if if nat_eq(a214, a215) {
                            let a218 = match v2.clone() {
                                Adt2::C0(_, _, h216, _, _, _, _, _, _, _, _) => {
                                    h216
                                }
                            };
                            let a219 = match v3.clone() {
                                Adt2::C0(_, _, h217, _, _, _, _, _, _, _, _) => {
                                    h217
                                }
                            };
                            nat_eq(a218, a219)
                        } else {
                            false
                        } {
                            let a222 = match v2.clone() {
                                Adt2::C0(_, _, _, h220, _, _, _, _, _, _, _) => {
                                    h220
                                }
                            };
                            let a223 = match v3.clone() {
                                Adt2::C0(_, _, _, h221, _, _, _, _, _, _, _) => {
                                    h221
                                }
                            };
                            nat_eq(a222, a223)
                        } else {
                            false
                        } {
                            let a226 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h224, _, _, _, _, _, _) => {
                                    h224
                                }
                            };
                            let a227 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h225, _, _, _, _, _, _) => {
                                    h225
                                }
                            };
                            nat_eq(a226, a227)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f11(v3.clone(), 1u64)?, 4u64, f12(v2, 1u64), f12(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h228, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h228
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h229, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h229
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h230, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h230
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h231, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h231
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
    let m232 = v3;
    match m232 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a233 = v0;
                let a234 = 4u64;
                nat_eq(a233, a234)
            };
            let v7: bool = {
                let a236 = 0u64;
                let a237 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h235, _, _, _, _, _, _, _, _, _, _, _) => {
                        h235
                    }
                };
                nat_lt(a236, a237)
            };
            let v8: u64 = f11(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h238, _, _, _, _, _, _, _, _, _) => {
                    h238
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h239, _, _, _, _, _, _, _, _) => {
                    h239
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h240, _, _, _, _, _, _, _) => {
                    h240
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h241, _, _, _, _, _, _) => {
                    h241
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h242, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h242
                }
            }, match v1.clone() {
                Adt1::C0(_, h243, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h243
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h244, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h244
                }
            });
            if if if if if if if if if if if if v6 {
                let a246 = match v2.clone() {
                    Adt2::C0(h245, _, _, _, _, _, _, _, _, _, _) => {
                        h245
                    }
                };
                let a247 = 0u64;
                if nat_eq(a246, a247) {
                    let a249 = match v5.clone() {
                        Adt2::C0(h248, _, _, _, _, _, _, _, _, _, _) => {
                            h248
                        }
                    };
                    let a250 = 0u64;
                    nat_eq(a249, a250)
                } else {
                    false
                }
            } else {
                let a252 = match v2.clone() {
                    Adt2::C0(h251, _, _, _, _, _, _, _, _, _, _) => {
                        h251
                    }
                };
                let a253 = 0u64;
                if if nat_eq(a252, a253) {
                    true
                } else {
                    let a255 = match v2.clone() {
                        Adt2::C0(h254, _, _, _, _, _, _, _, _, _, _) => {
                            h254
                        }
                    };
                    let a256 = 26u64;
                    nat_eq(a255, a256)
                } {
                    let a258 = match v5.clone() {
                        Adt2::C0(h257, _, _, _, _, _, _, _, _, _, _) => {
                            h257
                        }
                    };
                    let a259 = 0u64;
                    if nat_eq(a258, a259) {
                        true
                    } else {
                        let a261 = match v5.clone() {
                            Adt2::C0(h260, _, _, _, _, _, _, _, _, _, _) => {
                                h260
                            }
                        };
                        let a262 = 26u64;
                        nat_eq(a261, a262)
                    }
                } else {
                    false
                }
            } {
                let a265 = match v2.clone() {
                    Adt2::C0(h263, _, _, _, _, _, _, _, _, _, _) => {
                        h263
                    }
                };
                let a266 = match v4.clone() {
                    Adt2::C0(h264, _, _, _, _, _, _, _, _, _, _) => {
                        h264
                    }
                };
                nat_eq(a265, a266)
            } else {
                false
            } {
                f18(v2.clone(), v4.clone())
            } else {
                false
            } {
                f19(v2.clone())?
            } else {
                false
            } {
                f19(v4.clone())?
            } else {
                false
            } {
                let a268 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h267, _, _, _, _, _) => {
                        h267
                    }
                };
                let a269 = 4u64;
                nat_eq(a268, a269)
            } else {
                false
            } {
                let a270 = v8;
                let a271 = f11(v2.clone(), 1u64)?;
                nat_le(a270, a271)
            } else {
                false
            } {
                let a273 = match v1.clone() {
                    Adt1::C0(_, _, _, h272, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h272
                    }
                };
                let a274 = 1073741824u64;
                nat_lt(a273, a274)
            } else {
                false
            } {
                let a276 = match v1.clone() {
                    Adt1::C0(h275, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h275
                    }
                };
                let a277 = 4294967295u64;
                if if if if nat_le(a276, a277) {
                    let a279 = match v1.clone() {
                        Adt1::C0(_, h278, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h278
                        }
                    };
                    let a280 = 4294967295u64;
                    nat_le(a279, a280)
                } else {
                    false
                } {
                    let a282 = match v1.clone() {
                        Adt1::C0(_, _, h281, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h281
                        }
                    };
                    let a283 = 4294967295u64;
                    nat_le(a282, a283)
                } else {
                    false
                } {
                    let a285 = match v1.clone() {
                        Adt1::C0(_, _, _, h284, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h284
                        }
                    };
                    let a286 = 4294967295u64;
                    nat_le(a285, a286)
                } else {
                    false
                } {
                    let a288 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h287, _, _, _, _, _, _, _, _, _, _, _) => {
                            h287
                        }
                    };
                    let a289 = 4294967295u64;
                    nat_le(a288, a289)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a292 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h290) => {
                            h290
                        }
                    };
                    let a293 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h291) => {
                            h291
                        }
                    };
                    nat_eq(a292, a293)
                } else {
                    true
                }
            } else {
                false
            } {
                f20(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h294, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h294
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h295, _) => {
                        h295
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f11(v2, 1u64)?
                };
                Ok(Adt6::C4({
                    let a296 = v10;
                    let a297 = v8;
                    nat_add(a296, a297)?
                }, 4u64, f12(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h298, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h298
                    }
                }, v6)))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f7(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a300 = match v0.clone() {
        Adt2::C0(h299, _, _, _, _, _, _, _, _, _, _) => {
            h299
        }
    };
    let a301 = 0u64;
    if if if nat_eq(a300, a301) {
        let a303 = match v1.clone() {
            Adt2::C0(h302, _, _, _, _, _, _, _, _, _, _) => {
                h302
            }
        };
        let a304 = 0u64;
        nat_eq(a303, a304)
    } else {
        false
    } {
        f13(match v0.clone() {
            Adt2::C0(_, h305, _, _, _, _, _, _, _, _, _) => {
                h305
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h306, _, _, _, _, _, _, _, _) => {
                h306
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h307, _, _, _, _, _, _, _) => {
                h307
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h308, _, _, _, _, _, _) => {
                h308
            }
        }, match v1.clone() {
            Adt2::C0(_, h309, _, _, _, _, _, _, _, _, _) => {
                h309
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h310, _, _, _, _, _, _, _, _) => {
                h310
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h311, _, _, _, _, _, _, _) => {
                h311
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h312, _, _, _, _, _, _) => {
                h312
            }
        })
    } else {
        false
    } {
        let v2: u64 = f11(v1.clone(), 1u64)?;
        Ok(Adt6::C4({
            let a313 = v2;
            let a314 = f11(v0.clone(), 1u64)?;
            nat_add(a313, a314)?
        }, 4u64, f12(v0, 1u64), f12(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f8(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m315 = v3;
    match m315 {
        None => {
            f21(v0, v1, match v2.clone() {
                Adt2::C0(h316, _, _, _, _, _, _, _, _, _, _) => {
                    h316
                }
            }, 0u64, match v4.clone() {
                Adt2::C0(h317, _, _, _, _, _, _, _, _, _, _) => {
                    h317
                }
            }, false, match v2.clone() {
                Adt2::C0(_, h318, _, _, _, _, _, _, _, _, _) => {
                    h318
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h319, _, _, _, _, _, _, _, _) => {
                    h319
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h320, _, _, _, _, _, _, _) => {
                    h320
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h321, _, _, _, _, _, _) => {
                    h321
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt2::C0(_, h322, _, _, _, _, _, _, _, _, _) => {
                    h322
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h323, _, _, _, _, _, _, _, _) => {
                    h323
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h324, _, _, _, _, _, _, _) => {
                    h324
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h325, _, _, _, _, _, _) => {
                    h325
                }
            })
        }
        Some(v5) => {
            f21(v0, v1, match v2.clone() {
                Adt2::C0(h326, _, _, _, _, _, _, _, _, _, _) => {
                    h326
                }
            }, match v5.clone() {
                Adt2::C0(h327, _, _, _, _, _, _, _, _, _, _) => {
                    h327
                }
            }, match v4.clone() {
                Adt2::C0(h328, _, _, _, _, _, _, _, _, _, _) => {
                    h328
                }
            }, true, match v2.clone() {
                Adt2::C0(_, h329, _, _, _, _, _, _, _, _, _) => {
                    h329
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h330, _, _, _, _, _, _, _, _) => {
                    h330
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h331, _, _, _, _, _, _, _) => {
                    h331
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h332, _, _, _, _, _, _) => {
                    h332
                }
            }, match v5.clone() {
                Adt2::C0(_, h333, _, _, _, _, _, _, _, _, _) => {
                    h333
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h334, _, _, _, _, _, _, _, _) => {
                    h334
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h335, _, _, _, _, _, _, _) => {
                    h335
                }
            }, match v5 {
                Adt2::C0(_, _, _, _, h336, _, _, _, _, _, _) => {
                    h336
                }
            }, match v4.clone() {
                Adt2::C0(_, h337, _, _, _, _, _, _, _, _, _) => {
                    h337
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h338, _, _, _, _, _, _, _, _) => {
                    h338
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h339, _, _, _, _, _, _, _) => {
                    h339
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h340, _, _, _, _, _, _) => {
                    h340
                }
            })
        }
    }
}

pub fn f9(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt5> {
    let m341 = v3;
    match m341 {
        Adt4::C0 => {
            Ok(Adt5::C1(false, f22(v0, v1)?, f22(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt5::C1(false, f23(v0, v1, v2.clone())?, f22(v0, v2)?))
        }
        Adt4::C2(v4, v5) => {
            f24(v0, v1, v5, v2, v4)
        }
        Adt4::C3(v6, v7, v8, v9, v10) => {
            f25(v0, v1, v2, v6, v7, v8, v9, v10)
        }
        Adt4::C4(v11) => {
            f26(v0, v1, v2, v11)
        }
        Adt4::C5(v12, v13, v14, v15) => {
            f25(v0, v1, v2, v12, v13, v14, v15, true)
        }
        Adt4::C6(v16, v17, v18) => {
            f27(v0, v1, v2, v16, v17, v18)
        }
        Adt4::C7(v19) => {
            f28(v0, v1, v2, v19)
        }
    }
}

pub fn f10(v0: u64) -> Adt7 {
    let a342 = v0;
    let a343 = 0u64;
    if nat_eq(a342, a343) {
        Adt7::C1(Adt8::C0(0u64, 1u64, 4u64))
    } else {
        let a344 = v0;
        let a345 = 1u64;
        if nat_eq(a344, a345) {
            Adt7::C1(Adt8::C0(1u64, 1u64, 2u64))
        } else {
            let a346 = v0;
            let a347 = 2u64;
            if nat_eq(a346, a347) {
                Adt7::C1(Adt8::C0(2u64, 32u64, 18u64))
            } else {
                let a348 = v0;
                let a349 = 3u64;
                if nat_eq(a348, a349) {
                    Adt7::C1(Adt8::C0(3u64, 32u64, 20u64))
                } else {
                    let a350 = v0;
                    let a351 = 6u64;
                    if nat_eq(a350, a351) {
                        Adt7::C1(Adt8::C0(6u64, 32u64, 22u64))
                    } else {
                        let a352 = v0;
                        let a353 = 7u64;
                        if nat_eq(a352, a353) {
                            Adt7::C1(Adt8::C0(7u64, 32u64, 24u64))
                        } else {
                            let a354 = v0;
                            let a355 = 8u64;
                            if nat_eq(a354, a355) {
                                Adt7::C1(Adt8::C0(8u64, 32u64, 34u64))
                            } else {
                                let a356 = v0;
                                let a357 = 9u64;
                                if nat_eq(a356, a357) {
                                    Adt7::C1(Adt8::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a358 = v0;
                                    let a359 = 10u64;
                                    if nat_eq(a358, a359) {
                                        Adt7::C1(Adt8::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a360 = v0;
                                        let a361 = 11u64;
                                        if nat_eq(a360, a361) {
                                            Adt7::C1(Adt8::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a362 = v0;
                                            let a363 = 12u64;
                                            if nat_eq(a362, a363) {
                                                Adt7::C1(Adt8::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a364 = v0;
                                                let a365 = 13u64;
                                                if nat_eq(a364, a365) {
                                                    Adt7::C1(Adt8::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a366 = v0;
                                                    let a367 = 14u64;
                                                    if nat_eq(a366, a367) {
                                                        Adt7::C1(Adt8::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a368 = v0;
                                                        let a369 = 15u64;
                                                        if nat_eq(a368, a369) {
                                                            Adt7::C1(Adt8::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a370 = v0;
                                                            let a371 = 16u64;
                                                            if nat_eq(a370, a371) {
                                                                Adt7::C1(Adt8::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a372 = v0;
                                                                let a373 = 17u64;
                                                                if nat_eq(a372, a373) {
                                                                    Adt7::C1(Adt8::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a374 = v0;
                                                                    let a375 = 18u64;
                                                                    if nat_eq(a374, a375) {
                                                                        Adt7::C1(Adt8::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a376 = v0;
                                                                        let a377 = 19u64;
                                                                        if nat_eq(a376, a377) {
                                                                            Adt7::C1(Adt8::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a378 = v0;
                                                                            let a379 = 20u64;
                                                                            if nat_eq(a378, a379) {
                                                                                Adt7::C1(Adt8::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a380 = v0;
                                                                                let a381 = 21u64;
                                                                                if nat_eq(a380, a381) {
                                                                                    Adt7::C1(Adt8::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a382 = v0;
                                                                                    let a383 = 22u64;
                                                                                    if nat_eq(a382, a383) {
                                                                                        Adt7::C1(Adt8::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a384 = v0;
                                                                                        let a385 = 23u64;
                                                                                        if nat_eq(a384, a385) {
                                                                                            Adt7::C1(Adt8::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a386 = v0;
                                                                                            let a387 = 24u64;
                                                                                            if nat_eq(a386, a387) {
                                                                                                Adt7::C1(Adt8::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a388 = v0;
                                                                                                let a389 = 25u64;
                                                                                                if nat_eq(a388, a389) {
                                                                                                    Adt7::C1(Adt8::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a390 = v0;
                                                                                                    let a391 = 26u64;
                                                                                                    if nat_eq(a390, a391) {
                                                                                                        Adt7::C1(Adt8::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a392 = v0;
                                                                                                        let a393 = 27u64;
                                                                                                        if nat_eq(a392, a393) {
                                                                                                            Adt7::C1(Adt8::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a394 = v0;
                                                                                                            let a395 = 28u64;
                                                                                                            if nat_eq(a394, a395) {
                                                                                                                Adt7::C1(Adt8::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a396 = v0;
                                                                                                                let a397 = 29u64;
                                                                                                                if nat_eq(a396, a397) {
                                                                                                                    Adt7::C1(Adt8::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a398 = v0;
                                                                                                                    let a399 = 30u64;
                                                                                                                    if nat_eq(a398, a399) {
                                                                                                                        Adt7::C1(Adt8::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a400 = v0;
                                                                                                                        let a401 = 34u64;
                                                                                                                        if nat_eq(a400, a401) {
                                                                                                                            Adt7::C1(Adt8::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a402 = v0;
                                                                                                                            let a403 = 35u64;
                                                                                                                            if nat_eq(a402, a403) {
                                                                                                                                Adt7::C1(Adt8::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a404 = v0;
                                                                                                                                let a405 = 39u64;
                                                                                                                                if nat_eq(a404, a405) {
                                                                                                                                    Adt7::C1(Adt8::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a406 = v0;
                                                                                                                                    let a407 = 40u64;
                                                                                                                                    if nat_eq(a406, a407) {
                                                                                                                                        Adt7::C1(Adt8::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a408 = v0;
                                                                                                                                        let a409 = 41u64;
                                                                                                                                        if nat_eq(a408, a409) {
                                                                                                                                            Adt7::C1(Adt8::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a410 = v0;
                                                                                                                                            let a411 = 42u64;
                                                                                                                                            if nat_eq(a410, a411) {
                                                                                                                                                Adt7::C1(Adt8::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a412 = v0;
                                                                                                                                                let a413 = 43u64;
                                                                                                                                                if nat_eq(a412, a413) {
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

pub fn f11(v0: Adt2, v1: u64) -> R<u64> {
    let a425 = {
        let a419 = {
            let a415 = match v0.clone() {
                Adt2::C0(_, h414, _, _, _, _, _, _, _, _, _) => {
                    h414
                }
            };
            let a416 = v1;
            let a417 = 0u64;
            nat_quot(a415, a416, a417)
        };
        let a420 = match v0.clone() {
            Adt2::C0(_, _, h418, _, _, _, _, _, _, _, _) => {
                h418
            }
        };
        nat_mul(a419, a420)?
    };
    let a426 = {
        let a423 = match v0.clone() {
            Adt2::C0(_, _, _, h421, _, _, _, _, _, _, _) => {
                h421
            }
        };
        let a424 = match v0 {
            Adt2::C0(_, _, _, _, h422, _, _, _, _, _, _) => {
                h422
            }
        };
        nat_mul(a423, a424)?
    };
    nat_mul(a425, a426)
}

pub fn f12(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a428 = match v0.clone() {
            Adt2::C0(_, h427, _, _, _, _, _, _, _, _, _) => {
                h427
            }
        };
        let a429 = v1;
        let a430 = 0u64;
        nat_quot(a428, a429, a430)
    }, match v0.clone() {
        Adt2::C0(_, _, h431, _, _, _, _, _, _, _, _) => {
            h431
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h432, _, _, _, _, _, _, _) => {
            h432
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h433, _, _, _, _, _, _) => {
            h433
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h434, _, _, _, _, _) => {
            h434
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h435, _, _, _, _) => {
            h435
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h436, _, _, _) => {
            h436
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h437, _, _) => {
            h437
        }
    })
}

pub fn f13(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f16(v4, v5, v6, v7) {
        f16(v0, v1, v2, v3)
    } else {
        let a441 = {
            let a438 = v0;
            let a439 = v4;
            let a440 = 1u64;
            nat_rem(a438, a439, a440)
        };
        let a442 = 0u64;
        if if if nat_eq(a441, a442) {
            let a446 = {
                let a443 = v1;
                let a444 = v5;
                let a445 = 1u64;
                nat_rem(a443, a444, a445)
            };
            let a447 = 0u64;
            nat_eq(a446, a447)
        } else {
            false
        } {
            let a451 = {
                let a448 = v2;
                let a449 = v6;
                let a450 = 1u64;
                nat_rem(a448, a449, a450)
            };
            let a452 = 0u64;
            nat_eq(a451, a452)
        } else {
            false
        } {
            let a456 = {
                let a453 = v3;
                let a454 = v7;
                let a455 = 1u64;
                nat_rem(a453, a454, a455)
            };
            let a457 = 0u64;
            nat_eq(a456, a457)
        } else {
            false
        }
    }
}

pub fn f14(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a458 = v0;
    let a459 = v1;
    if nat_eq(a458, a459) {
        let a462 = v4;
        let a463 = {
            let a460 = v2;
            let a461 = v3;
            nat_add(a460, a461)?
        };
        Ok(nat_eq(a462, a463))
    } else {
        let a464 = v2;
        let a465 = v3;
        if nat_eq(a464, a465) {
            let a466 = v4;
            let a467 = v2;
            Ok(nat_eq(a466, a467))
        } else {
            Ok(false)
        }
    }
}

pub fn f15(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a470 = {
            let a468 = v0;
            let a469 = f29(v1);
            nat_add(a468, a469)?
        };
        let a471 = f29(v2);
        nat_add(a470, a471)?
    };
    let v5: u64 = {
        let a472 = f30(v1);
        let a473 = f30(v2);
        nat_add(a472, a473)?
    };
    let a474 = v1;
    let a475 = 4294967295u64;
    if if nat_le(a474, a475) {
        let a476 = v2;
        let a477 = 4294967295u64;
        nat_le(a476, a477)
    } else {
        false
    } {
        let a478 = v5;
        let a479 = v4;
        if nat_le(a478, a479) {
            let a482 = v3;
            let a483 = {
                let a480 = v4;
                let a481 = v5;
                nat_sub(a480, a481)
            };
            Ok(nat_eq(a482, a483))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f16(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a484 = v0;
    let a485 = 0u64;
    if if if nat_eq(a484, a485) {
        true
    } else {
        let a486 = v1;
        let a487 = 0u64;
        nat_eq(a486, a487)
    } {
        true
    } else {
        let a488 = v2;
        let a489 = 0u64;
        nat_eq(a488, a489)
    } {
        true
    } else {
        let a490 = v3;
        let a491 = 0u64;
        nat_eq(a490, a491)
    }
}

pub fn f17(v0: u64, v1: u64) -> R<bool> {
    let a492 = v1;
    let a493 = 4294967295u64;
    if nat_le(a492, a493) {
        let a496 = {
            let a494 = f29(v1);
            let a495 = f30(v1);
            nat_add(a494, a495)?
        };
        let a497 = v0;
        Ok(nat_lt(a496, a497))
    } else {
        Ok(false)
    }
}

pub fn f18(v0: Adt2, v1: Adt2) -> bool {
    let a500 = match v0.clone() {
        Adt2::C0(_, h498, _, _, _, _, _, _, _, _, _) => {
            h498
        }
    };
    let a501 = match v1.clone() {
        Adt2::C0(_, h499, _, _, _, _, _, _, _, _, _) => {
            h499
        }
    };
    if if if nat_eq(a500, a501) {
        let a504 = match v0.clone() {
            Adt2::C0(_, _, h502, _, _, _, _, _, _, _, _) => {
                h502
            }
        };
        let a505 = match v1.clone() {
            Adt2::C0(_, _, h503, _, _, _, _, _, _, _, _) => {
                h503
            }
        };
        nat_eq(a504, a505)
    } else {
        false
    } {
        let a508 = match v0.clone() {
            Adt2::C0(_, _, _, h506, _, _, _, _, _, _, _) => {
                h506
            }
        };
        let a509 = match v1.clone() {
            Adt2::C0(_, _, _, h507, _, _, _, _, _, _, _) => {
                h507
            }
        };
        nat_eq(a508, a509)
    } else {
        false
    } {
        let a512 = match v0 {
            Adt2::C0(_, _, _, _, h510, _, _, _, _, _, _) => {
                h510
            }
        };
        let a513 = match v1 {
            Adt2::C0(_, _, _, _, h511, _, _, _, _, _, _) => {
                h511
            }
        };
        nat_eq(a512, a513)
    } else {
        false
    }
}

pub fn f19(v0: Adt2) -> R<bool> {
    let a515 = match v0.clone() {
        Adt2::C0(_, h514, _, _, _, _, _, _, _, _, _) => {
            h514
        }
    };
    let a516 = 1u64;
    if if if if nat_eq(a515, a516) {
        true
    } else {
        let a518 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h517, _, _, _, _, _) => {
                h517
            }
        };
        let a519 = 4u64;
        nat_eq(a518, a519)
    } {
        let a521 = match v0.clone() {
            Adt2::C0(_, _, h520, _, _, _, _, _, _, _, _) => {
                h520
            }
        };
        let a522 = 1u64;
        if nat_eq(a521, a522) {
            true
        } else {
            let a527 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h523, _, _, _, _) => {
                    h523
                }
            };
            let a528 = {
                let a525 = match v0.clone() {
                    Adt2::C0(_, h524, _, _, _, _, _, _, _, _, _) => {
                        h524
                    }
                };
                let a526 = 4u64;
                nat_mul(a525, a526)?
            };
            nat_eq(a527, a528)
        }
    } else {
        false
    } {
        let a530 = match v0.clone() {
            Adt2::C0(_, _, _, h529, _, _, _, _, _, _, _) => {
                h529
            }
        };
        let a531 = 1u64;
        if nat_eq(a530, a531) {
            true
        } else {
            let a539 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h532, _, _, _) => {
                    h532
                }
            };
            let a540 = {
                let a537 = {
                    let a535 = match v0.clone() {
                        Adt2::C0(_, h533, _, _, _, _, _, _, _, _, _) => {
                            h533
                        }
                    };
                    let a536 = match v0.clone() {
                        Adt2::C0(_, _, h534, _, _, _, _, _, _, _, _) => {
                            h534
                        }
                    };
                    nat_mul(a535, a536)?
                };
                let a538 = 4u64;
                nat_mul(a537, a538)?
            };
            nat_eq(a539, a540)
        }
    } else {
        false
    } {
        let a542 = match v0.clone() {
            Adt2::C0(_, _, _, _, h541, _, _, _, _, _, _) => {
                h541
            }
        };
        let a543 = 1u64;
        if nat_eq(a542, a543) {
            Ok(true)
        } else {
            let a554 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h544, _, _) => {
                    h544
                }
            };
            let a555 = {
                let a552 = {
                    let a550 = {
                        let a547 = match v0.clone() {
                            Adt2::C0(_, h545, _, _, _, _, _, _, _, _, _) => {
                                h545
                            }
                        };
                        let a548 = match v0.clone() {
                            Adt2::C0(_, _, h546, _, _, _, _, _, _, _, _) => {
                                h546
                            }
                        };
                        nat_mul(a547, a548)?
                    };
                    let a551 = match v0 {
                        Adt2::C0(_, _, _, h549, _, _, _, _, _, _, _) => {
                            h549
                        }
                    };
                    nat_mul(a550, a551)?
                };
                let a553 = 4u64;
                nat_mul(a552, a553)?
            };
            Ok(nat_eq(a554, a555))
        }
    } else {
        Ok(false)
    }
}

pub fn f20(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f31(v0.clone(), false)?;
    let v6: u64 = f31(v0, true)?;
    let a556 = v6;
    let a557 = v1;
    if if nat_le(a556, a557) {
        let a564 = {
            let a562 = {
                let a558 = v1;
                let a559 = v5;
                nat_add(a558, a559)?
            };
            let a563 = {
                let a560 = v3;
                let a561 = 0u64;
                if nat_eq(a560, a561) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a562, a563)?
        };
        let a565 = v2;
        nat_le(a564, a565)
    } else {
        false
    } {
        if v4 {
            let a570 = {
                let a568 = {
                    let a566 = v1;
                    let a567 = v5;
                    nat_add(a566, a567)?
                };
                let a569 = v6;
                nat_sub(a568, a569)
            };
            let a571 = v2;
            Ok(nat_lt(a570, a571))
        } else {
            let a576 = {
                let a574 = {
                    let a572 = v1;
                    let a573 = v5;
                    nat_add(a572, a573)?
                };
                let a575 = v6;
                nat_sub(a574, a575)
            };
            let a577 = v2;
            Ok(nat_le(a576, a577))
        }
    } else {
        Ok(false)
    }
}

pub fn f21(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt6> {
    let m578 = f32(v0, v1);
    match m578 {
        None => {
            if v5 {
                let m579 = f33(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m579 {
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
                let a580 = 4u64;
                let a581 = v28;
                nat_le(a580, a581)
            };
            let a582 = v2;
            let a583 = 0u64;
            if if if if if nat_eq(a582, a583) {
                let a584 = v4;
                let a585 = 0u64;
                nat_eq(a584, a585)
            } else {
                false
            } {
                let a586 = v6;
                let a587 = v14;
                if nat_eq(a586, a587) {
                    let a588 = v7;
                    let a589 = v15;
                    nat_eq(a588, a589)
                } else {
                    false
                }
            } else {
                false
            } {
                let a590 = v8;
                let a591 = v16;
                if nat_eq(a590, a591) {
                    let a592 = v9;
                    let a593 = v17;
                    nat_eq(a592, a593)
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
                        let a594 = v3;
                        let a595 = 0u64;
                        nat_eq(a594, a595)
                    } else {
                        false
                    } {
                        f13(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt6::C2(v28, v29, v6, v7, v8, v9, {
                    let a600 = {
                        let a596 = v6;
                        let a597 = v7;
                        nat_mul(a596, a597)?
                    };
                    let a601 = {
                        let a598 = v8;
                        let a599 = v9;
                        nat_mul(a598, a599)?
                    };
                    nat_mul(a600, a601)?
                }))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f22(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a603 = v0;
        let a604 = match v1.clone() {
            Adt3::C0(h602, _, _, _, _, _, _, _) => {
                h602
            }
        };
        let a605 = 0u64;
        nat_quot(a603, a604, a605)
    };
    let v3: u64 = {
        let a607 = v2;
        let a608 = match v1.clone() {
            Adt3::C0(_, h606, _, _, _, _, _, _) => {
                h606
            }
        };
        let a609 = 0u64;
        nat_quot(a607, a608, a609)
    };
    f34({
        let a611 = v0;
        let a612 = match v1.clone() {
            Adt3::C0(h610, _, _, _, _, _, _, _) => {
                h610
            }
        };
        let a613 = 0u64;
        nat_rem(a611, a612, a613)
    }, {
        let a615 = v2;
        let a616 = match v1.clone() {
            Adt3::C0(_, h614, _, _, _, _, _, _) => {
                h614
            }
        };
        let a617 = 0u64;
        nat_rem(a615, a616, a617)
    }, {
        let a619 = v3;
        let a620 = match v1.clone() {
            Adt3::C0(_, _, h618, _, _, _, _, _) => {
                h618
            }
        };
        let a621 = 0u64;
        nat_rem(a619, a620, a621)
    }, {
        let a623 = v3;
        let a624 = match v1.clone() {
            Adt3::C0(_, _, h622, _, _, _, _, _) => {
                h622
            }
        };
        let a625 = 0u64;
        nat_quot(a623, a624, a625)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h626, _, _, _) => {
            h626
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h627, _, _) => {
            h627
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h628, _) => {
            h628
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h629) => {
            h629
        }
    })
}

pub fn f23(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a631 = v0;
        let a632 = match v2.clone() {
            Adt3::C0(h630, _, _, _, _, _, _, _) => {
                h630
            }
        };
        let a633 = 0u64;
        nat_quot(a631, a632, a633)
    };
    let v4: u64 = {
        let a635 = v3;
        let a636 = match v2.clone() {
            Adt3::C0(_, h634, _, _, _, _, _, _) => {
                h634
            }
        };
        let a637 = 0u64;
        nat_quot(a635, a636, a637)
    };
    f35({
        let a639 = v0;
        let a640 = match v2.clone() {
            Adt3::C0(h638, _, _, _, _, _, _, _) => {
                h638
            }
        };
        let a641 = 0u64;
        nat_rem(a639, a640, a641)
    }, {
        let a643 = v3;
        let a644 = match v2.clone() {
            Adt3::C0(_, h642, _, _, _, _, _, _) => {
                h642
            }
        };
        let a645 = 0u64;
        nat_rem(a643, a644, a645)
    }, {
        let a647 = v4;
        let a648 = match v2.clone() {
            Adt3::C0(_, _, h646, _, _, _, _, _) => {
                h646
            }
        };
        let a649 = 0u64;
        nat_rem(a647, a648, a649)
    }, {
        let a651 = v4;
        let a652 = match v2 {
            Adt3::C0(_, _, h650, _, _, _, _, _) => {
                h650
            }
        };
        let a653 = 0u64;
        nat_quot(a651, a652, a653)
    }, match v1.clone() {
        Adt3::C0(h654, _, _, _, _, _, _, _) => {
            h654
        }
    }, match v1.clone() {
        Adt3::C0(_, h655, _, _, _, _, _, _) => {
            h655
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h656, _, _, _, _, _) => {
            h656
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h657, _, _, _, _) => {
            h657
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h658, _, _, _) => {
            h658
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h659, _, _) => {
            h659
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h660, _) => {
            h660
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h661) => {
            h661
        }
    })
}

pub fn f24(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a663 = v0;
        let a664 = match v3.clone() {
            Adt3::C0(h662, _, _, _, _, _, _, _) => {
                h662
            }
        };
        let a665 = 0u64;
        nat_quot(a663, a664, a665)
    };
    let v6: u64 = {
        let a667 = v5;
        let a668 = match v3.clone() {
            Adt3::C0(_, h666, _, _, _, _, _, _) => {
                h666
            }
        };
        let a669 = 0u64;
        nat_quot(a667, a668, a669)
    };
    let v7: u64 = {
        let a671 = v0;
        let a672 = match v3.clone() {
            Adt3::C0(h670, _, _, _, _, _, _, _) => {
                h670
            }
        };
        let a673 = 0u64;
        nat_rem(a671, a672, a673)
    };
    let v8: u64 = {
        let a675 = v5;
        let a676 = match v3.clone() {
            Adt3::C0(_, h674, _, _, _, _, _, _) => {
                h674
            }
        };
        let a677 = 0u64;
        nat_rem(a675, a676, a677)
    };
    let v9: u64 = {
        let a679 = v6;
        let a680 = match v3.clone() {
            Adt3::C0(_, _, h678, _, _, _, _, _) => {
                h678
            }
        };
        let a681 = 0u64;
        nat_rem(a679, a680, a681)
    };
    let v10: u64 = {
        let a683 = v6;
        let a684 = match v3.clone() {
            Adt3::C0(_, _, h682, _, _, _, _, _) => {
                h682
            }
        };
        let a685 = 0u64;
        nat_quot(a683, a684, a685)
    };
    let v11: u64 = {
        let a686 = v4;
        let a687 = 0u64;
        if nat_eq(a686, a687) {
            match v1.clone() {
                Adt3::C0(h688, _, _, _, _, _, _, _) => {
                    h688
                }
            }
        } else {
            let a689 = v4;
            let a690 = 1u64;
            if nat_eq(a689, a690) {
                match v1.clone() {
                    Adt3::C0(_, h691, _, _, _, _, _, _) => {
                        h691
                    }
                }
            } else {
                let a692 = v4;
                let a693 = 2u64;
                if nat_eq(a692, a693) {
                    match v1.clone() {
                        Adt3::C0(_, _, h694, _, _, _, _, _) => {
                            h694
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h695, _, _, _, _) => {
                            h695
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a702 = v11;
        let a703 = {
            let a696 = v4;
            let a697 = 0u64;
            if nat_eq(a696, a697) {
                v7
            } else {
                let a698 = v4;
                let a699 = 1u64;
                if nat_eq(a698, a699) {
                    v8
                } else {
                    let a700 = v4;
                    let a701 = 2u64;
                    if nat_eq(a700, a701) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a702, a703)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt5::C1(v12, f34(if if v12 {
        let a704 = v4;
        let a705 = 0u64;
        nat_eq(a704, a705)
    } else {
        false
    } {
        let a706 = v7;
        let a707 = v11;
        nat_sub(a706, a707)
    } else {
        v7
    }, if if v12 {
        let a708 = v4;
        let a709 = 1u64;
        nat_eq(a708, a709)
    } else {
        false
    } {
        let a710 = v8;
        let a711 = v11;
        nat_sub(a710, a711)
    } else {
        v8
    }, if if v12 {
        let a712 = v4;
        let a713 = 2u64;
        nat_eq(a712, a713)
    } else {
        false
    } {
        let a714 = v9;
        let a715 = v11;
        nat_sub(a714, a715)
    } else {
        v9
    }, if if v12 {
        let a716 = v4;
        let a717 = 3u64;
        nat_eq(a716, a717)
    } else {
        false
    } {
        let a718 = v10;
        let a719 = v11;
        nat_sub(a718, a719)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h720, _, _, _) => {
            h720
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h721, _, _) => {
            h721
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h722, _) => {
            h722
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h723) => {
            h723
        }
    })?, f22(v0, v3)?))
}

pub fn f25(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt5> {
    let v8: u64 = f36(v0, v2.clone(), 0u64);
    let v9: u64 = f36(v0, v2.clone(), 1u64);
    let v10: u64 = f36(v0, v2.clone(), 2u64);
    let v11: u64 = f36(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt5::C1(false, f37(v1.clone(), f38(v8, v3, match v1.clone() {
            Adt3::C0(h724, _, _, _, _, _, _, _) => {
                h724
            }
        })?, f38(v9, v4, match v1.clone() {
            Adt3::C0(_, h725, _, _, _, _, _, _) => {
                h725
            }
        })?, f38(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h726, _, _, _, _, _) => {
                h726
            }
        })?, f38(v11, v6, match v1 {
            Adt3::C0(_, _, _, h727, _, _, _, _) => {
                h727
            }
        })?)?, f22(v0, v2)?))
    } else {
        if if if if f39(v8, v3, match v1.clone() {
            Adt3::C0(h728, _, _, _, _, _, _, _) => {
                h728
            }
        })? {
            f39(v9, v4, match v1.clone() {
                Adt3::C0(_, h729, _, _, _, _, _, _) => {
                    h729
                }
            })?
        } else {
            false
        } {
            f39(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h730, _, _, _, _, _) => {
                    h730
                }
            })?
        } else {
            false
        } {
            f39(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h731, _, _, _, _) => {
                    h731
                }
            })?
        } else {
            false
        } {
            Ok(Adt5::C1(false, f37(v1, f40(v8, v3)?, f40(v9, v4)?, f40(v10, v5)?, f40(v11, v6)?)?, f22(v0, v2)?))
        } else {
            Ok(Adt5::C1(true, 0u64, f22(v0, v2)?))
        }
    }
}

pub fn f26(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f36(v0, v2.clone(), 0u64);
    let v5: u64 = f36(v0, v2.clone(), 1u64);
    let v6: u64 = f36(v0, v2.clone(), 2u64);
    let v7: u64 = f36(v0, v2.clone(), 3u64);
    Ok(Adt5::C1(false, f37(v1.clone(), f41(v4, v3, match v1 {
        Adt3::C0(h732, _, _, _, _, _, _, _) => {
            h732
        }
    })?, v5, v6, v7)?, f22(v0, v2)?))
}

pub fn f27(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt5> {
    let a733 = v0;
    let a734 = v3;
    if nat_lt(a733, a734) {
        Ok(Adt5::C1(false, {
            let a735 = v0;
            let a736 = 4u64;
            nat_mul(a735, a736)?
        }, {
            let a737 = v0;
            let a738 = 4u64;
            nat_mul(a737, a738)?
        }))
    } else {
        let v6: u64 = {
            let a739 = v0;
            let a740 = v3;
            nat_sub(a739, a740)
        };
        let v7: u64 = f22(v6, v1)?;
        let v8: u64 = f42(v6, v2, v4)?;
        if v5 {
            Ok(Adt5::C2(v7, v8))
        } else {
            Ok(Adt5::C1(true, v7, v8))
        }
    }
}

pub fn f28(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a741 = v0;
    let a742 = v3;
    if nat_lt(a741, a742) {
        Ok(Adt5::C1(true, 0u64, f22(v0, v2)?))
    } else {
        let v4: u64 = {
            let a743 = v0;
            let a744 = v3;
            nat_sub(a743, a744)
        };
        Ok(Adt5::C3(f22(v4, v1.clone())?, f23(v4, v2, v1)?))
    }
}

pub fn f29(v0: u64) -> u64 {
    let a745 = v0;
    let a746 = 2147483648u64;
    if nat_lt(a745, a746) {
        v0
    } else {
        0u64
    }
}

pub fn f30(v0: u64) -> u64 {
    let a747 = v0;
    let a748 = 2147483648u64;
    if nat_lt(a747, a748) {
        0u64
    } else {
        let a749 = 4294967296u64;
        let a750 = v0;
        nat_sub(a749, a750)
    }
}

pub fn f31(v0: Adt3, v1: bool) -> R<u64> {
    f43(v0.clone(), {
        let a752 = match v0.clone() {
            Adt3::C0(h751, _, _, _, _, _, _, _) => {
                h751
            }
        };
        let a753 = 1u64;
        nat_sub(a752, a753)
    }, {
        let a755 = match v0.clone() {
            Adt3::C0(_, h754, _, _, _, _, _, _) => {
                h754
            }
        };
        let a756 = 1u64;
        nat_sub(a755, a756)
    }, {
        let a758 = match v0.clone() {
            Adt3::C0(_, _, h757, _, _, _, _, _) => {
                h757
            }
        };
        let a759 = 1u64;
        nat_sub(a758, a759)
    }, {
        let a761 = match v0 {
            Adt3::C0(_, _, _, h760, _, _, _, _) => {
                h760
            }
        };
        let a762 = 1u64;
        nat_sub(a761, a762)
    }, v1)
}

pub fn f32(v0: u64, v1: u64) -> Option<u64> {
    let a763 = v0;
    let a764 = 2u64;
    if nat_eq(a763, a764) {
        Some(0u64)
    } else {
        let a765 = v0;
        let a766 = 5u64;
        if nat_eq(a765, a766) {
            Some(1u64)
        } else {
            let a767 = v0;
            let a768 = 6u64;
            if nat_eq(a767, a768) {
                Some(2u64)
            } else {
                let a769 = v0;
                let a770 = 7u64;
                if nat_eq(a769, a770) {
                    Some(3u64)
                } else {
                    let a771 = v0;
                    let a772 = 9u64;
                    if nat_eq(a771, a772) {
                        Some(4u64)
                    } else {
                        let a773 = v0;
                        let a774 = 8u64;
                        if nat_eq(a773, a774) {
                            Some(5u64)
                        } else {
                            let a775 = v0;
                            let a776 = 89u64;
                            if nat_eq(a775, a776) {
                                let a777 = v1;
                                let a778 = 2u64;
                                if nat_eq(a777, a778) {
                                    Some(6u64)
                                } else {
                                    let a779 = v1;
                                    let a780 = 0u64;
                                    if nat_eq(a779, a780) {
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

pub fn f33(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt9> {
    let a781 = v0;
    let a782 = 28u64;
    if if if if nat_eq(a781, a782) {
        let a783 = v1;
        let a784 = 0u64;
        nat_eq(a783, a784)
    } else {
        false
    } {
        let a785 = v2;
        let a786 = 0u64;
        nat_eq(a785, a786)
    } else {
        false
    } {
        let a787 = v3;
        let a788 = 0u64;
        nat_eq(a787, a788)
    } else {
        false
    } {
        let a789 = v4;
        let a790 = v8;
        if if if if if if if if nat_eq(a789, a790) {
            let a791 = v5;
            let a792 = v12;
            nat_eq(a791, a792)
        } else {
            false
        } {
            let a793 = v9;
            let a794 = v13;
            nat_eq(a793, a794)
        } else {
            false
        } {
            let a795 = v10;
            let a796 = v14;
            nat_eq(a795, a796)
        } else {
            false
        } {
            let a797 = v11;
            let a798 = v15;
            nat_eq(a797, a798)
        } else {
            false
        } {
            let a799 = 0u64;
            let a800 = v6;
            nat_lt(a799, a800)
        } else {
            false
        } {
            let a801 = 0u64;
            let a802 = v7;
            nat_lt(a801, a802)
        } else {
            false
        } {
            let a806 = {
                let a803 = v10;
                let a804 = v6;
                let a805 = 1u64;
                nat_rem(a803, a804, a805)
            };
            let a807 = 0u64;
            if nat_eq(a806, a807) {
                let a811 = {
                    let a808 = v11;
                    let a809 = v7;
                    let a810 = 1u64;
                    nat_rem(a808, a809, a810)
                };
                let a812 = 0u64;
                nat_eq(a811, a812)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt9::C2(v5, v4, v9, v10, v11, {
                let a813 = v10;
                let a814 = v6;
                let a815 = 0u64;
                nat_quot(a813, a814, a815)
            }, {
                let a816 = v11;
                let a817 = v7;
                let a818 = 0u64;
                nat_quot(a816, a817, a818)
            }, {
                let a819 = v5;
                let a820 = v4;
                nat_mul(a819, a820)?
            }, {
                let a821 = v4;
                let a822 = v9;
                nat_mul(a821, a822)?
            }, {
                let a823 = v5;
                let a824 = v9;
                nat_mul(a823, a824)?
            }))
        } else {
            Ok(Adt9::C1)
        }
    } else {
        Ok(Adt9::C0)
    }
}

pub fn f34(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a837 = {
        let a833 = {
            let a829 = {
                let a825 = v0;
                let a826 = v4;
                nat_mul(a825, a826)?
            };
            let a830 = {
                let a827 = v1;
                let a828 = v5;
                nat_mul(a827, a828)?
            };
            nat_add(a829, a830)?
        };
        let a834 = {
            let a831 = v2;
            let a832 = v6;
            nat_mul(a831, a832)?
        };
        nat_add(a833, a834)?
    };
    let a838 = {
        let a835 = v3;
        let a836 = v7;
        nat_mul(a835, a836)?
    };
    nat_add(a837, a838)
}

pub fn f35(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f34({
        let a839 = v0;
        let a840 = v4;
        let a841 = 0u64;
        nat_rem(a839, a840, a841)
    }, {
        let a842 = v1;
        let a843 = v5;
        let a844 = 0u64;
        nat_rem(a842, a843, a844)
    }, {
        let a845 = v2;
        let a846 = v6;
        let a847 = 0u64;
        nat_rem(a845, a846, a847)
    }, {
        let a848 = v3;
        let a849 = v7;
        let a850 = 0u64;
        nat_rem(a848, a849, a850)
    }, v8, v9, v10, v11)
}

pub fn f36(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a852 = v0;
        let a853 = match v1.clone() {
            Adt3::C0(h851, _, _, _, _, _, _, _) => {
                h851
            }
        };
        let a854 = 0u64;
        nat_quot(a852, a853, a854)
    };
    let v4: u64 = {
        let a856 = v3;
        let a857 = match v1.clone() {
            Adt3::C0(_, h855, _, _, _, _, _, _) => {
                h855
            }
        };
        let a858 = 0u64;
        nat_quot(a856, a857, a858)
    };
    let a859 = v2;
    let a860 = 0u64;
    if nat_eq(a859, a860) {
        let a862 = v0;
        let a863 = match v1 {
            Adt3::C0(h861, _, _, _, _, _, _, _) => {
                h861
            }
        };
        let a864 = 0u64;
        nat_rem(a862, a863, a864)
    } else {
        let a865 = v2;
        let a866 = 1u64;
        if nat_eq(a865, a866) {
            let a868 = v3;
            let a869 = match v1 {
                Adt3::C0(_, h867, _, _, _, _, _, _) => {
                    h867
                }
            };
            let a870 = 0u64;
            nat_rem(a868, a869, a870)
        } else {
            let a871 = v2;
            let a872 = 2u64;
            if nat_eq(a871, a872) {
                let a874 = v4;
                let a875 = match v1 {
                    Adt3::C0(_, _, h873, _, _, _, _, _) => {
                        h873
                    }
                };
                let a876 = 0u64;
                nat_rem(a874, a875, a876)
            } else {
                let a878 = v4;
                let a879 = match v1 {
                    Adt3::C0(_, _, h877, _, _, _, _, _) => {
                        h877
                    }
                };
                let a880 = 0u64;
                nat_quot(a878, a879, a880)
            }
        }
    }
}

pub fn f37(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f34(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h881, _, _, _) => {
            h881
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h882, _, _) => {
            h882
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h883, _) => {
            h883
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h884) => {
            h884
        }
    })
}

pub fn f38(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a885 = v1;
    let a886 = 2147483648u64;
    if nat_lt(a885, a886) {
        let a897 = {
            let a895 = {
                let a890 = {
                    let a887 = v0;
                    let a888 = v2;
                    let a889 = 0u64;
                    nat_rem(a887, a888, a889)
                };
                let a891 = v2;
                nat_add(a890, a891)?
            };
            let a896 = {
                let a892 = v1;
                let a893 = v2;
                let a894 = 0u64;
                nat_rem(a892, a893, a894)
            };
            nat_sub(a895, a896)
        };
        let a898 = v2;
        let a899 = 0u64;
        Ok(nat_rem(a897, a898, a899))
    } else {
        let a902 = {
            let a900 = v0;
            let a901 = f30(v1);
            nat_add(a900, a901)?
        };
        let a903 = v2;
        let a904 = 0u64;
        Ok(nat_rem(a902, a903, a904))
    }
}

pub fn f39(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a907 = f29(v1);
    let a908 = {
        let a905 = v0;
        let a906 = f30(v1);
        nat_add(a905, a906)?
    };
    if nat_le(a907, a908) {
        let a909 = f40(v0, v1)?;
        let a910 = v2;
        Ok(nat_lt(a909, a910))
    } else {
        Ok(false)
    }
}

pub fn f40(v0: u64, v1: u64) -> R<u64> {
    let a913 = {
        let a911 = v0;
        let a912 = f30(v1);
        nat_add(a911, a912)?
    };
    let a914 = f29(v1);
    Ok(nat_sub(a913, a914))
}

pub fn f41(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a915 = v0;
    let a916 = v1;
    if nat_lt(a915, a916) {
        let a917 = v1;
        let a918 = v0;
        Ok(nat_sub(a917, a918))
    } else {
        let a921 = v0;
        let a922 = {
            let a919 = v1;
            let a920 = v2;
            nat_add(a919, a920)?
        };
        if nat_lt(a921, a922) {
            let a923 = v0;
            let a924 = v1;
            Ok(nat_sub(a923, a924))
        } else {
            let a931 = {
                let a927 = 2u64;
                let a928 = {
                    let a925 = v2;
                    let a926 = 1u64;
                    nat_sub(a925, a926)
                };
                nat_mul(a927, a928)?
            };
            let a932 = {
                let a929 = v0;
                let a930 = v1;
                nat_sub(a929, a930)
            };
            Ok(nat_sub(a931, a932))
        }
    }
}

pub fn f42(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a935 = {
        let a933 = v2;
        let a934 = f43(v1.clone(), f36(v0, v1.clone(), 0u64), f36(v0, v1.clone(), 1u64), f36(v0, v1.clone(), 2u64), f36(v0, v1.clone(), 3u64), false)?;
        nat_add(a933, a934)?
    };
    let a936 = f43(v1.clone(), f36(v0, v1.clone(), 0u64), f36(v0, v1.clone(), 1u64), f36(v0, v1.clone(), 2u64), f36(v0, v1, 3u64), true)?;
    Ok(nat_sub(a935, a936))
}

pub fn f43(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a957 = {
        let a951 = {
            let a945 = {
                let a939 = v1;
                let a940 = if v5 {
                    f30(match v0.clone() {
                        Adt3::C0(_, _, _, _, h937, _, _, _) => {
                            h937
                        }
                    })
                } else {
                    f29(match v0.clone() {
                        Adt3::C0(_, _, _, _, h938, _, _, _) => {
                            h938
                        }
                    })
                };
                nat_mul(a939, a940)?
            };
            let a946 = {
                let a943 = v2;
                let a944 = if v5 {
                    f30(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h941, _, _) => {
                            h941
                        }
                    })
                } else {
                    f29(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h942, _, _) => {
                            h942
                        }
                    })
                };
                nat_mul(a943, a944)?
            };
            nat_add(a945, a946)?
        };
        let a952 = {
            let a949 = v3;
            let a950 = if v5 {
                f30(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h947, _) => {
                        h947
                    }
                })
            } else {
                f29(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h948, _) => {
                        h948
                    }
                })
            };
            nat_mul(a949, a950)?
        };
        nat_add(a951, a952)?
    };
    let a958 = {
        let a955 = v4;
        let a956 = if v5 {
            f30(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h953) => {
                    h953
                }
            })
        } else {
            f29(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h954) => {
                    h954
                }
            })
        };
        nat_mul(a955, a956)?
    };
    nat_add(a957, a958)
}
