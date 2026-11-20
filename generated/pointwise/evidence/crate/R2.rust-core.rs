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
    C8,
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
                                    let a20 = v1;
                                    let a21 = 42u64;
                                    if nat_eq(a20, a21) {
                                        f8(v3, v5)?
                                    } else {
                                        f9(v1, v6, v3, v4, v5)?
                                    }
                                }
                            }
                        }
                    }
                }
            }))
        }
        Adt0::C1(v8, v9, v10, v11) => {
            f10(v8, v9, v10, v11)
        }
    }
}

pub fn f1(v0: u64) -> bool {
    let a22 = v0;
    let a23 = 1u64;
    if if nat_eq(a22, a23) {
        true
    } else {
        let a24 = v0;
        let a25 = 33u64;
        nat_eq(a24, a25)
    } {
        true
    } else {
        let a26 = v0;
        let a27 = 34u64;
        nat_eq(a26, a27)
    }
}

pub fn f2(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a30 = match v1.clone() {
        Adt2::C0(h28, _, _, _, _, _, _, _, _, _, _) => {
            h28
        }
    };
    let a31 = match v3.clone() {
        Adt2::C0(h29, _, _, _, _, _, _, _, _, _, _) => {
            h29
        }
    };
    if if nat_eq(a30, a31) {
        let a32 = v0;
        let a33 = 33u64;
        if nat_eq(a32, a33) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m35 = f11(match v1.clone() {
            Adt2::C0(h34, _, _, _, _, _, _, _, _, _, _) => {
                h34
            }
        });
        match m35 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt8::C0(_, h36, _) => {
                        h36
                    }
                };
                let v6: u64 = f12(v1.clone(), v5)?;
                let a41 = {
                    let a38 = match v1.clone() {
                        Adt2::C0(_, h37, _, _, _, _, _, _, _, _, _) => {
                            h37
                        }
                    };
                    let a39 = v5;
                    let a40 = 1u64;
                    nat_rem(a38, a39, a40)
                };
                let a42 = 0u64;
                if if if nat_eq(a41, a42) {
                    let a47 = {
                        let a44 = match v3.clone() {
                            Adt2::C0(_, h43, _, _, _, _, _, _, _, _, _) => {
                                h43
                            }
                        };
                        let a45 = v5;
                        let a46 = 1u64;
                        nat_rem(a44, a45, a46)
                    };
                    let a48 = 0u64;
                    nat_eq(a47, a48)
                } else {
                    false
                } {
                    let a49 = v6;
                    let a50 = f12(v3.clone(), v5)?;
                    nat_eq(a49, a50)
                } else {
                    false
                } {
                    Ok(Adt6::C4(v6, match v4 {
                        Adt8::C0(_, _, h51) => {
                            h51
                        }
                    }, f13(v1, v5), f13(v3, v5), Adt4::C0))
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
    let a53 = match v0.clone() {
        Adt2::C0(h52, _, _, _, _, _, _, _, _, _, _) => {
            h52
        }
    };
    let a54 = 0u64;
    if if if if nat_eq(a53, a54) {
        true
    } else {
        let a56 = match v0.clone() {
            Adt2::C0(h55, _, _, _, _, _, _, _, _, _, _) => {
                h55
            }
        };
        let a57 = 1u64;
        nat_eq(a56, a57)
    } {
        true
    } else {
        let a59 = match v0.clone() {
            Adt2::C0(h58, _, _, _, _, _, _, _, _, _, _) => {
                h58
            }
        };
        let a60 = 25u64;
        if nat_eq(a59, a60) {
            true
        } else {
            let a62 = match v0.clone() {
                Adt2::C0(h61, _, _, _, _, _, _, _, _, _, _) => {
                    h61
                }
            };
            let a63 = 26u64;
            nat_eq(a62, a63)
        }
    } {
        true
    } else {
        let a65 = match v0.clone() {
            Adt2::C0(h64, _, _, _, _, _, _, _, _, _, _) => {
                h64
            }
        };
        let a66 = 30u64;
        nat_eq(a65, a66)
    } {
        let a69 = match v0.clone() {
            Adt2::C0(h67, _, _, _, _, _, _, _, _, _, _) => {
                h67
            }
        };
        let a70 = match v1.clone() {
            Adt2::C0(h68, _, _, _, _, _, _, _, _, _, _) => {
                h68
            }
        };
        if if nat_eq(a69, a70) {
            f14(match v1.clone() {
                Adt2::C0(_, h71, _, _, _, _, _, _, _, _, _) => {
                    h71
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h72, _, _, _, _, _, _, _, _) => {
                    h72
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h73, _, _, _, _, _, _, _) => {
                    h73
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h74, _, _, _, _, _, _) => {
                    h74
                }
            }, match v0.clone() {
                Adt2::C0(_, h75, _, _, _, _, _, _, _, _, _) => {
                    h75
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h76, _, _, _, _, _, _, _, _) => {
                    h76
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h77, _, _, _, _, _, _, _) => {
                    h77
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h78, _, _, _, _, _, _) => {
                    h78
                }
            })
        } else {
            false
        } {
            let m80 = f11(match v0.clone() {
                Adt2::C0(h79, _, _, _, _, _, _, _, _, _, _) => {
                    h79
                }
            });
            match m80 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v2) => {
                    Ok(Adt6::C4(f12(v1.clone(), 1u64)?, match v2 {
                        Adt8::C0(_, _, h81) => {
                            h81
                        }
                    }, f13(v0, 1u64), f13(v1, 1u64), Adt4::C1))
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
    let m82 = v2;
    match m82 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let a83 = v0;
            let a84 = 4u64;
            if if if nat_lt(a83, a84) {
                let a87 = match v1.clone() {
                    Adt2::C0(h85, _, _, _, _, _, _, _, _, _, _) => {
                        h85
                    }
                };
                let a88 = match v4.clone() {
                    Adt2::C0(h86, _, _, _, _, _, _, _, _, _, _) => {
                        h86
                    }
                };
                if nat_eq(a87, a88) {
                    let a91 = match v1.clone() {
                        Adt2::C0(h89, _, _, _, _, _, _, _, _, _, _) => {
                            h89
                        }
                    };
                    let a92 = match v3.clone() {
                        Adt2::C0(h90, _, _, _, _, _, _, _, _, _, _) => {
                            h90
                        }
                    };
                    nat_eq(a91, a92)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f15(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h93, _, _, _, _, _, _, _, _, _) => {
                        h93
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h94, _, _, _, _, _, _, _, _, _) => {
                        h94
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h95, _, _, _, _, _, _, _, _, _) => {
                        h95
                    }
                })? {
                    f15(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h96, _, _, _, _, _, _, _, _) => {
                            h96
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h97, _, _, _, _, _, _, _, _) => {
                            h97
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h98, _, _, _, _, _, _, _, _) => {
                            h98
                        }
                    })?
                } else {
                    false
                } {
                    f15(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h99, _, _, _, _, _, _, _) => {
                            h99
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h100, _, _, _, _, _, _, _) => {
                            h100
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h101, _, _, _, _, _, _, _) => {
                            h101
                        }
                    })?
                } else {
                    false
                } {
                    f15(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h102, _, _, _, _, _, _) => {
                            h102
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h103, _, _, _, _, _, _) => {
                            h103
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h104, _, _, _, _, _, _) => {
                            h104
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m106 = f11(match v1.clone() {
                    Adt2::C0(h105, _, _, _, _, _, _, _, _, _, _) => {
                        h105
                    }
                });
                match m106 {
                    Adt7::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt7::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt8::C0(_, h107, _) => {
                                h107
                            }
                        };
                        let a112 = {
                            let a109 = match v1.clone() {
                                Adt2::C0(_, h108, _, _, _, _, _, _, _, _, _) => {
                                    h108
                                }
                            };
                            let a110 = v6;
                            let a111 = 1u64;
                            nat_rem(a109, a110, a111)
                        };
                        let a113 = 0u64;
                        if if if nat_eq(a112, a113) {
                            let a118 = {
                                let a115 = match v4.clone() {
                                    Adt2::C0(_, h114, _, _, _, _, _, _, _, _, _) => {
                                        h114
                                    }
                                };
                                let a116 = v6;
                                let a117 = 1u64;
                                nat_rem(a115, a116, a117)
                            };
                            let a119 = 0u64;
                            nat_eq(a118, a119)
                        } else {
                            false
                        } {
                            let a124 = {
                                let a121 = match v3.clone() {
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
                            Ok(Adt6::C4(f12(v3.clone(), v6)?, match v5 {
                                Adt8::C0(_, _, h126) => {
                                    h126
                                }
                            }, f13(v1, v6), f13(v3, v6), Adt4::C2(v0, f13(v4, v6))))
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
    let a128 = match v2.clone() {
        Adt2::C0(h127, _, _, _, _, _, _, _, _, _, _) => {
            h127
        }
    };
    let a129 = 0u64;
    if if nat_eq(a128, a129) {
        let a131 = match v3.clone() {
            Adt2::C0(h130, _, _, _, _, _, _, _, _, _, _) => {
                h130
            }
        };
        let a132 = 0u64;
        nat_eq(a131, a132)
    } else {
        false
    } {
        let a133 = v0;
        let a134 = 61u64;
        if nat_eq(a133, a134) {
            let v4: bool = {
                let a136 = 0u64;
                let a137 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h135, _, _, _, _, _, _, _) => {
                        h135
                    }
                };
                nat_lt(a136, a137)
            };
            if if if if if if f16(match v2.clone() {
                Adt2::C0(_, h138, _, _, _, _, _, _, _, _, _) => {
                    h138
                }
            }, match v1.clone() {
                Adt1::C0(h139, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h139
                }
            }, match v1.clone() {
                Adt1::C0(_, h140, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h140
                }
            }, match v3.clone() {
                Adt2::C0(_, h141, _, _, _, _, _, _, _, _, _) => {
                    h141
                }
            })? {
                f16(match v2.clone() {
                    Adt2::C0(_, _, h142, _, _, _, _, _, _, _, _) => {
                        h142
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h143, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h143
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h144, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h144
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h145, _, _, _, _, _, _, _, _) => {
                        h145
                    }
                })?
            } else {
                false
            } {
                f16(match v2.clone() {
                    Adt2::C0(_, _, _, h146, _, _, _, _, _, _, _) => {
                        h146
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h147, _, _, _, _, _, _, _, _, _, _, _) => {
                        h147
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h148, _, _, _, _, _, _, _, _, _, _) => {
                        h148
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h149, _, _, _, _, _, _, _) => {
                        h149
                    }
                })?
            } else {
                false
            } {
                f16(match v2.clone() {
                    Adt2::C0(_, _, _, _, h150, _, _, _, _, _, _) => {
                        h150
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h151, _, _, _, _, _, _, _, _, _) => {
                        h151
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h152, _, _, _, _, _, _, _, _) => {
                        h152
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h153, _, _, _, _, _, _) => {
                        h153
                    }
                })?
            } else {
                false
            } {
                let a155 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h154, _, _, _, _, _, _, _) => {
                        h154
                    }
                };
                let a156 = 4294967295u64;
                nat_le(a155, a156)
            } else {
                false
            } {
                if v4 {
                    !f17(match v2.clone() {
                        Adt2::C0(_, h157, _, _, _, _, _, _, _, _, _) => {
                            h157
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h158, _, _, _, _, _, _, _, _) => {
                            h158
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h159, _, _, _, _, _, _, _) => {
                            h159
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h160, _, _, _, _, _, _) => {
                            h160
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt6::C4(f12(v3.clone(), 1u64)?, 4u64, f13(v2, 1u64), f13(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h161, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h161
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h162, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h162
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h163, _, _, _, _, _, _, _, _, _, _, _) => {
                        h163
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h164, _, _, _, _, _, _, _, _, _) => {
                        h164
                    }
                }, v4)))
            } else {
                Ok(Adt6::C1)
            }
        } else {
            let a165 = v0;
            let a166 = 62u64;
            if nat_eq(a165, a166) {
                let a168 = match v1.clone() {
                    Adt1::C0(h167, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h167
                    }
                };
                let a169 = 2147483648u64;
                if if if if if nat_lt(a168, a169) {
                    let a171 = match v1.clone() {
                        Adt1::C0(_, h170, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h170
                        }
                    };
                    let a172 = 2147483648u64;
                    nat_lt(a171, a172)
                } else {
                    false
                } {
                    let a175 = match v1.clone() {
                        Adt1::C0(h173, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h173
                        }
                    };
                    let a176 = match v2.clone() {
                        Adt2::C0(_, h174, _, _, _, _, _, _, _, _, _) => {
                            h174
                        }
                    };
                    nat_lt(a175, a176)
                } else {
                    false
                } {
                    let a179 = match v1.clone() {
                        Adt1::C0(_, h177, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h177
                        }
                    };
                    let a180 = match v2.clone() {
                        Adt2::C0(_, h178, _, _, _, _, _, _, _, _, _) => {
                            h178
                        }
                    };
                    nat_lt(a179, a180)
                } else {
                    false
                } {
                    let a183 = match v2.clone() {
                        Adt2::C0(_, _, h181, _, _, _, _, _, _, _, _) => {
                            h181
                        }
                    };
                    let a184 = match v3.clone() {
                        Adt2::C0(_, _, h182, _, _, _, _, _, _, _, _) => {
                            h182
                        }
                    };
                    if if if nat_eq(a183, a184) {
                        let a187 = match v2.clone() {
                            Adt2::C0(_, _, _, h185, _, _, _, _, _, _, _) => {
                                h185
                            }
                        };
                        let a188 = match v3.clone() {
                            Adt2::C0(_, _, _, h186, _, _, _, _, _, _, _) => {
                                h186
                            }
                        };
                        nat_eq(a187, a188)
                    } else {
                        false
                    } {
                        let a191 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h189, _, _, _, _, _, _) => {
                                h189
                            }
                        };
                        let a192 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h190, _, _, _, _, _, _) => {
                                h190
                            }
                        };
                        nat_eq(a191, a192)
                    } else {
                        false
                    } {
                        let a201 = match v3.clone() {
                            Adt2::C0(_, h193, _, _, _, _, _, _, _, _, _) => {
                                h193
                            }
                        };
                        let a202 = {
                            let a199 = {
                                let a196 = match v2.clone() {
                                    Adt2::C0(_, h194, _, _, _, _, _, _, _, _, _) => {
                                        h194
                                    }
                                };
                                let a197 = match v1.clone() {
                                    Adt1::C0(h195, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h195
                                    }
                                };
                                nat_add(a196, a197)?
                            };
                            let a200 = match v1.clone() {
                                Adt1::C0(_, h198, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h198
                                }
                            };
                            nat_add(a199, a200)?
                        };
                        nat_eq(a201, a202)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt6::C4(f12(v3.clone(), 1u64)?, 4u64, f13(v2, 1u64), f13(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h203, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h203
                        }
                    })))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                let a204 = v0;
                let a205 = 63u64;
                if nat_eq(a204, a205) {
                    if if if if if f18(match v2.clone() {
                        Adt2::C0(_, h206, _, _, _, _, _, _, _, _, _) => {
                            h206
                        }
                    }, match v1.clone() {
                        Adt1::C0(h207, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h207
                        }
                    })? {
                        f18(match v2.clone() {
                            Adt2::C0(_, _, h208, _, _, _, _, _, _, _, _) => {
                                h208
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h209, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h209
                            }
                        })?
                    } else {
                        false
                    } {
                        f18(match v2.clone() {
                            Adt2::C0(_, _, _, h210, _, _, _, _, _, _, _) => {
                                h210
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h211, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h211
                            }
                        })?
                    } else {
                        false
                    } {
                        f18(match v2.clone() {
                            Adt2::C0(_, _, _, _, h212, _, _, _, _, _, _) => {
                                h212
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h213, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h213
                            }
                        })?
                    } else {
                        false
                    } {
                        let a216 = match v2.clone() {
                            Adt2::C0(_, h214, _, _, _, _, _, _, _, _, _) => {
                                h214
                            }
                        };
                        let a217 = match v3.clone() {
                            Adt2::C0(_, h215, _, _, _, _, _, _, _, _, _) => {
                                h215
                            }
                        };
                        if if if nat_eq(a216, a217) {
                            let a220 = match v2.clone() {
                                Adt2::C0(_, _, h218, _, _, _, _, _, _, _, _) => {
                                    h218
                                }
                            };
                            let a221 = match v3.clone() {
                                Adt2::C0(_, _, h219, _, _, _, _, _, _, _, _) => {
                                    h219
                                }
                            };
                            nat_eq(a220, a221)
                        } else {
                            false
                        } {
                            let a224 = match v2.clone() {
                                Adt2::C0(_, _, _, h222, _, _, _, _, _, _, _) => {
                                    h222
                                }
                            };
                            let a225 = match v3.clone() {
                                Adt2::C0(_, _, _, h223, _, _, _, _, _, _, _) => {
                                    h223
                                }
                            };
                            nat_eq(a224, a225)
                        } else {
                            false
                        } {
                            let a228 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h226, _, _, _, _, _, _) => {
                                    h226
                                }
                            };
                            let a229 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h227, _, _, _, _, _, _) => {
                                    h227
                                }
                            };
                            nat_eq(a228, a229)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f12(v3.clone(), 1u64)?, 4u64, f13(v2, 1u64), f13(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h230, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h230
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h231, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h231
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h232, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h232
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h233, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h233
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
    let m234 = v3;
    match m234 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a235 = v0;
                let a236 = 4u64;
                nat_eq(a235, a236)
            };
            let v7: bool = {
                let a238 = 0u64;
                let a239 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h237, _, _, _, _, _, _, _, _, _, _, _) => {
                        h237
                    }
                };
                nat_lt(a238, a239)
            };
            let v8: u64 = f12(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h240, _, _, _, _, _, _, _, _, _) => {
                    h240
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h241, _, _, _, _, _, _, _, _) => {
                    h241
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h242, _, _, _, _, _, _, _) => {
                    h242
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h243, _, _, _, _, _, _) => {
                    h243
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h244, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h244
                }
            }, match v1.clone() {
                Adt1::C0(_, h245, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h245
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h246, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h246
                }
            });
            if if if if if if if if if if if if v6 {
                let a248 = match v2.clone() {
                    Adt2::C0(h247, _, _, _, _, _, _, _, _, _, _) => {
                        h247
                    }
                };
                let a249 = 0u64;
                if nat_eq(a248, a249) {
                    let a251 = match v5.clone() {
                        Adt2::C0(h250, _, _, _, _, _, _, _, _, _, _) => {
                            h250
                        }
                    };
                    let a252 = 0u64;
                    nat_eq(a251, a252)
                } else {
                    false
                }
            } else {
                let a254 = match v2.clone() {
                    Adt2::C0(h253, _, _, _, _, _, _, _, _, _, _) => {
                        h253
                    }
                };
                let a255 = 0u64;
                if if nat_eq(a254, a255) {
                    true
                } else {
                    let a257 = match v2.clone() {
                        Adt2::C0(h256, _, _, _, _, _, _, _, _, _, _) => {
                            h256
                        }
                    };
                    let a258 = 26u64;
                    nat_eq(a257, a258)
                } {
                    let a260 = match v5.clone() {
                        Adt2::C0(h259, _, _, _, _, _, _, _, _, _, _) => {
                            h259
                        }
                    };
                    let a261 = 0u64;
                    if nat_eq(a260, a261) {
                        true
                    } else {
                        let a263 = match v5.clone() {
                            Adt2::C0(h262, _, _, _, _, _, _, _, _, _, _) => {
                                h262
                            }
                        };
                        let a264 = 26u64;
                        nat_eq(a263, a264)
                    }
                } else {
                    false
                }
            } {
                let a267 = match v2.clone() {
                    Adt2::C0(h265, _, _, _, _, _, _, _, _, _, _) => {
                        h265
                    }
                };
                let a268 = match v4.clone() {
                    Adt2::C0(h266, _, _, _, _, _, _, _, _, _, _) => {
                        h266
                    }
                };
                nat_eq(a267, a268)
            } else {
                false
            } {
                f19(v2.clone(), v4.clone())
            } else {
                false
            } {
                f20(v2.clone())?
            } else {
                false
            } {
                f20(v4.clone())?
            } else {
                false
            } {
                let a270 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h269, _, _, _, _, _) => {
                        h269
                    }
                };
                let a271 = 4u64;
                nat_eq(a270, a271)
            } else {
                false
            } {
                let a272 = v8;
                let a273 = f12(v2.clone(), 1u64)?;
                nat_le(a272, a273)
            } else {
                false
            } {
                let a275 = match v1.clone() {
                    Adt1::C0(_, _, _, h274, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h274
                    }
                };
                let a276 = 1073741824u64;
                nat_lt(a275, a276)
            } else {
                false
            } {
                let a278 = match v1.clone() {
                    Adt1::C0(h277, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h277
                    }
                };
                let a279 = 4294967295u64;
                if if if if nat_le(a278, a279) {
                    let a281 = match v1.clone() {
                        Adt1::C0(_, h280, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h280
                        }
                    };
                    let a282 = 4294967295u64;
                    nat_le(a281, a282)
                } else {
                    false
                } {
                    let a284 = match v1.clone() {
                        Adt1::C0(_, _, h283, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h283
                        }
                    };
                    let a285 = 4294967295u64;
                    nat_le(a284, a285)
                } else {
                    false
                } {
                    let a287 = match v1.clone() {
                        Adt1::C0(_, _, _, h286, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h286
                        }
                    };
                    let a288 = 4294967295u64;
                    nat_le(a287, a288)
                } else {
                    false
                } {
                    let a290 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h289, _, _, _, _, _, _, _, _, _, _, _) => {
                            h289
                        }
                    };
                    let a291 = 4294967295u64;
                    nat_le(a290, a291)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a294 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h292) => {
                            h292
                        }
                    };
                    let a295 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h293) => {
                            h293
                        }
                    };
                    nat_eq(a294, a295)
                } else {
                    true
                }
            } else {
                false
            } {
                f21(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h296, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h296
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h297, _) => {
                        h297
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f12(v2, 1u64)?
                };
                Ok(Adt6::C4({
                    let a298 = v10;
                    let a299 = v8;
                    nat_add(a298, a299)?
                }, 4u64, f13(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h300, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h300
                    }
                }, v6)))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f7(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a302 = match v0.clone() {
        Adt2::C0(h301, _, _, _, _, _, _, _, _, _, _) => {
            h301
        }
    };
    let a303 = 0u64;
    if if if nat_eq(a302, a303) {
        let a305 = match v1.clone() {
            Adt2::C0(h304, _, _, _, _, _, _, _, _, _, _) => {
                h304
            }
        };
        let a306 = 0u64;
        nat_eq(a305, a306)
    } else {
        false
    } {
        f14(match v0.clone() {
            Adt2::C0(_, h307, _, _, _, _, _, _, _, _, _) => {
                h307
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h308, _, _, _, _, _, _, _, _) => {
                h308
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h309, _, _, _, _, _, _, _) => {
                h309
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h310, _, _, _, _, _, _) => {
                h310
            }
        }, match v1.clone() {
            Adt2::C0(_, h311, _, _, _, _, _, _, _, _, _) => {
                h311
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h312, _, _, _, _, _, _, _, _) => {
                h312
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h313, _, _, _, _, _, _, _) => {
                h313
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h314, _, _, _, _, _, _) => {
                h314
            }
        })
    } else {
        false
    } {
        let v2: u64 = f12(v1.clone(), 1u64)?;
        Ok(Adt6::C4({
            let a315 = v2;
            let a316 = f12(v0.clone(), 1u64)?;
            nat_add(a315, a316)?
        }, 4u64, f13(v0, 1u64), f13(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f8(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a318 = match v0.clone() {
        Adt2::C0(h317, _, _, _, _, _, _, _, _, _, _) => {
            h317
        }
    };
    let a319 = 0u64;
    if if if nat_eq(a318, a319) {
        true
    } else {
        let a321 = match v0.clone() {
            Adt2::C0(h320, _, _, _, _, _, _, _, _, _, _) => {
                h320
            }
        };
        let a322 = 1u64;
        nat_eq(a321, a322)
    } {
        true
    } else {
        let a324 = match v0.clone() {
            Adt2::C0(h323, _, _, _, _, _, _, _, _, _, _) => {
                h323
            }
        };
        let a325 = 30u64;
        nat_eq(a324, a325)
    } {
        let a328 = match v0.clone() {
            Adt2::C0(h326, _, _, _, _, _, _, _, _, _, _) => {
                h326
            }
        };
        let a329 = match v1.clone() {
            Adt2::C0(h327, _, _, _, _, _, _, _, _, _, _) => {
                h327
            }
        };
        if if if if if nat_eq(a328, a329) {
            let a331 = match v0.clone() {
                Adt2::C0(_, _, h330, _, _, _, _, _, _, _, _) => {
                    h330
                }
            };
            let a332 = 1u64;
            nat_eq(a331, a332)
        } else {
            false
        } {
            let a335 = match v0.clone() {
                Adt2::C0(_, h333, _, _, _, _, _, _, _, _, _) => {
                    h333
                }
            };
            let a336 = match v1.clone() {
                Adt2::C0(_, h334, _, _, _, _, _, _, _, _, _) => {
                    h334
                }
            };
            nat_eq(a335, a336)
        } else {
            false
        } {
            let a339 = match v0.clone() {
                Adt2::C0(_, h337, _, _, _, _, _, _, _, _, _) => {
                    h337
                }
            };
            let a340 = match v1.clone() {
                Adt2::C0(_, _, h338, _, _, _, _, _, _, _, _) => {
                    h338
                }
            };
            if nat_eq(a339, a340) {
                let a343 = match v0.clone() {
                    Adt2::C0(_, _, _, h341, _, _, _, _, _, _, _) => {
                        h341
                    }
                };
                let a344 = match v1.clone() {
                    Adt2::C0(_, _, _, h342, _, _, _, _, _, _, _) => {
                        h342
                    }
                };
                nat_eq(a343, a344)
            } else {
                false
            }
        } else {
            false
        } {
            let a347 = match v0.clone() {
                Adt2::C0(_, _, _, _, h345, _, _, _, _, _, _) => {
                    h345
                }
            };
            let a348 = match v1.clone() {
                Adt2::C0(_, _, _, _, h346, _, _, _, _, _, _) => {
                    h346
                }
            };
            nat_eq(a347, a348)
        } else {
            false
        } {
            let m350 = f11(match v0.clone() {
                Adt2::C0(h349, _, _, _, _, _, _, _, _, _, _) => {
                    h349
                }
            });
            match m350 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v2) => {
                    Ok(Adt6::C4(f12(v1.clone(), 1u64)?, match v2 {
                        Adt8::C0(_, _, h351) => {
                            h351
                        }
                    }, f13(v0, 1u64), f13(v1, 1u64), Adt4::C8))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f9(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m352 = v3;
    match m352 {
        None => {
            f22(v0, v1, match v2.clone() {
                Adt2::C0(h353, _, _, _, _, _, _, _, _, _, _) => {
                    h353
                }
            }, 0u64, match v4.clone() {
                Adt2::C0(h354, _, _, _, _, _, _, _, _, _, _) => {
                    h354
                }
            }, false, match v2.clone() {
                Adt2::C0(_, h355, _, _, _, _, _, _, _, _, _) => {
                    h355
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h356, _, _, _, _, _, _, _, _) => {
                    h356
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h357, _, _, _, _, _, _, _) => {
                    h357
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h358, _, _, _, _, _, _) => {
                    h358
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt2::C0(_, h359, _, _, _, _, _, _, _, _, _) => {
                    h359
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h360, _, _, _, _, _, _, _, _) => {
                    h360
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h361, _, _, _, _, _, _, _) => {
                    h361
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h362, _, _, _, _, _, _) => {
                    h362
                }
            })
        }
        Some(v5) => {
            f22(v0, v1, match v2.clone() {
                Adt2::C0(h363, _, _, _, _, _, _, _, _, _, _) => {
                    h363
                }
            }, match v5.clone() {
                Adt2::C0(h364, _, _, _, _, _, _, _, _, _, _) => {
                    h364
                }
            }, match v4.clone() {
                Adt2::C0(h365, _, _, _, _, _, _, _, _, _, _) => {
                    h365
                }
            }, true, match v2.clone() {
                Adt2::C0(_, h366, _, _, _, _, _, _, _, _, _) => {
                    h366
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h367, _, _, _, _, _, _, _, _) => {
                    h367
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h368, _, _, _, _, _, _, _) => {
                    h368
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h369, _, _, _, _, _, _) => {
                    h369
                }
            }, match v5.clone() {
                Adt2::C0(_, h370, _, _, _, _, _, _, _, _, _) => {
                    h370
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h371, _, _, _, _, _, _, _, _) => {
                    h371
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h372, _, _, _, _, _, _, _) => {
                    h372
                }
            }, match v5 {
                Adt2::C0(_, _, _, _, h373, _, _, _, _, _, _) => {
                    h373
                }
            }, match v4.clone() {
                Adt2::C0(_, h374, _, _, _, _, _, _, _, _, _) => {
                    h374
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h375, _, _, _, _, _, _, _, _) => {
                    h375
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h376, _, _, _, _, _, _, _) => {
                    h376
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h377, _, _, _, _, _, _) => {
                    h377
                }
            })
        }
    }
}

pub fn f10(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt5> {
    let m378 = v3;
    match m378 {
        Adt4::C0 => {
            Ok(Adt5::C1(false, f23(v0, v1)?, f23(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt5::C1(false, f24(v0, v1, v2.clone())?, f23(v0, v2)?))
        }
        Adt4::C2(v4, v5) => {
            f25(v0, v1, v5, v2, v4)
        }
        Adt4::C3(v6, v7, v8, v9, v10) => {
            f26(v0, v1, v2, v6, v7, v8, v9, v10)
        }
        Adt4::C4(v11) => {
            f27(v0, v1, v2, v11)
        }
        Adt4::C5(v12, v13, v14, v15) => {
            f26(v0, v1, v2, v12, v13, v14, v15, true)
        }
        Adt4::C6(v16, v17, v18) => {
            f28(v0, v1, v2, v16, v17, v18)
        }
        Adt4::C7(v19) => {
            f29(v0, v1, v2, v19)
        }
        Adt4::C8 => {
            f30(v0, v1, v2)
        }
    }
}

pub fn f11(v0: u64) -> Adt7 {
    let a379 = v0;
    let a380 = 0u64;
    if nat_eq(a379, a380) {
        Adt7::C1(Adt8::C0(0u64, 1u64, 4u64))
    } else {
        let a381 = v0;
        let a382 = 1u64;
        if nat_eq(a381, a382) {
            Adt7::C1(Adt8::C0(1u64, 1u64, 2u64))
        } else {
            let a383 = v0;
            let a384 = 2u64;
            if nat_eq(a383, a384) {
                Adt7::C1(Adt8::C0(2u64, 32u64, 18u64))
            } else {
                let a385 = v0;
                let a386 = 3u64;
                if nat_eq(a385, a386) {
                    Adt7::C1(Adt8::C0(3u64, 32u64, 20u64))
                } else {
                    let a387 = v0;
                    let a388 = 6u64;
                    if nat_eq(a387, a388) {
                        Adt7::C1(Adt8::C0(6u64, 32u64, 22u64))
                    } else {
                        let a389 = v0;
                        let a390 = 7u64;
                        if nat_eq(a389, a390) {
                            Adt7::C1(Adt8::C0(7u64, 32u64, 24u64))
                        } else {
                            let a391 = v0;
                            let a392 = 8u64;
                            if nat_eq(a391, a392) {
                                Adt7::C1(Adt8::C0(8u64, 32u64, 34u64))
                            } else {
                                let a393 = v0;
                                let a394 = 9u64;
                                if nat_eq(a393, a394) {
                                    Adt7::C1(Adt8::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a395 = v0;
                                    let a396 = 10u64;
                                    if nat_eq(a395, a396) {
                                        Adt7::C1(Adt8::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a397 = v0;
                                        let a398 = 11u64;
                                        if nat_eq(a397, a398) {
                                            Adt7::C1(Adt8::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a399 = v0;
                                            let a400 = 12u64;
                                            if nat_eq(a399, a400) {
                                                Adt7::C1(Adt8::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a401 = v0;
                                                let a402 = 13u64;
                                                if nat_eq(a401, a402) {
                                                    Adt7::C1(Adt8::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a403 = v0;
                                                    let a404 = 14u64;
                                                    if nat_eq(a403, a404) {
                                                        Adt7::C1(Adt8::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a405 = v0;
                                                        let a406 = 15u64;
                                                        if nat_eq(a405, a406) {
                                                            Adt7::C1(Adt8::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a407 = v0;
                                                            let a408 = 16u64;
                                                            if nat_eq(a407, a408) {
                                                                Adt7::C1(Adt8::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a409 = v0;
                                                                let a410 = 17u64;
                                                                if nat_eq(a409, a410) {
                                                                    Adt7::C1(Adt8::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a411 = v0;
                                                                    let a412 = 18u64;
                                                                    if nat_eq(a411, a412) {
                                                                        Adt7::C1(Adt8::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a413 = v0;
                                                                        let a414 = 19u64;
                                                                        if nat_eq(a413, a414) {
                                                                            Adt7::C1(Adt8::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a415 = v0;
                                                                            let a416 = 20u64;
                                                                            if nat_eq(a415, a416) {
                                                                                Adt7::C1(Adt8::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a417 = v0;
                                                                                let a418 = 21u64;
                                                                                if nat_eq(a417, a418) {
                                                                                    Adt7::C1(Adt8::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a419 = v0;
                                                                                    let a420 = 22u64;
                                                                                    if nat_eq(a419, a420) {
                                                                                        Adt7::C1(Adt8::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a421 = v0;
                                                                                        let a422 = 23u64;
                                                                                        if nat_eq(a421, a422) {
                                                                                            Adt7::C1(Adt8::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a423 = v0;
                                                                                            let a424 = 24u64;
                                                                                            if nat_eq(a423, a424) {
                                                                                                Adt7::C1(Adt8::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a425 = v0;
                                                                                                let a426 = 25u64;
                                                                                                if nat_eq(a425, a426) {
                                                                                                    Adt7::C1(Adt8::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a427 = v0;
                                                                                                    let a428 = 26u64;
                                                                                                    if nat_eq(a427, a428) {
                                                                                                        Adt7::C1(Adt8::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a429 = v0;
                                                                                                        let a430 = 27u64;
                                                                                                        if nat_eq(a429, a430) {
                                                                                                            Adt7::C1(Adt8::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a431 = v0;
                                                                                                            let a432 = 28u64;
                                                                                                            if nat_eq(a431, a432) {
                                                                                                                Adt7::C1(Adt8::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a433 = v0;
                                                                                                                let a434 = 29u64;
                                                                                                                if nat_eq(a433, a434) {
                                                                                                                    Adt7::C1(Adt8::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a435 = v0;
                                                                                                                    let a436 = 30u64;
                                                                                                                    if nat_eq(a435, a436) {
                                                                                                                        Adt7::C1(Adt8::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a437 = v0;
                                                                                                                        let a438 = 34u64;
                                                                                                                        if nat_eq(a437, a438) {
                                                                                                                            Adt7::C1(Adt8::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a439 = v0;
                                                                                                                            let a440 = 35u64;
                                                                                                                            if nat_eq(a439, a440) {
                                                                                                                                Adt7::C1(Adt8::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a441 = v0;
                                                                                                                                let a442 = 39u64;
                                                                                                                                if nat_eq(a441, a442) {
                                                                                                                                    Adt7::C1(Adt8::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a443 = v0;
                                                                                                                                    let a444 = 40u64;
                                                                                                                                    if nat_eq(a443, a444) {
                                                                                                                                        Adt7::C1(Adt8::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a445 = v0;
                                                                                                                                        let a446 = 41u64;
                                                                                                                                        if nat_eq(a445, a446) {
                                                                                                                                            Adt7::C1(Adt8::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a447 = v0;
                                                                                                                                            let a448 = 42u64;
                                                                                                                                            if nat_eq(a447, a448) {
                                                                                                                                                Adt7::C1(Adt8::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a449 = v0;
                                                                                                                                                let a450 = 43u64;
                                                                                                                                                if nat_eq(a449, a450) {
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

pub fn f12(v0: Adt2, v1: u64) -> R<u64> {
    let a462 = {
        let a456 = {
            let a452 = match v0.clone() {
                Adt2::C0(_, h451, _, _, _, _, _, _, _, _, _) => {
                    h451
                }
            };
            let a453 = v1;
            let a454 = 0u64;
            nat_quot(a452, a453, a454)
        };
        let a457 = match v0.clone() {
            Adt2::C0(_, _, h455, _, _, _, _, _, _, _, _) => {
                h455
            }
        };
        nat_mul(a456, a457)?
    };
    let a463 = {
        let a460 = match v0.clone() {
            Adt2::C0(_, _, _, h458, _, _, _, _, _, _, _) => {
                h458
            }
        };
        let a461 = match v0 {
            Adt2::C0(_, _, _, _, h459, _, _, _, _, _, _) => {
                h459
            }
        };
        nat_mul(a460, a461)?
    };
    nat_mul(a462, a463)
}

pub fn f13(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a465 = match v0.clone() {
            Adt2::C0(_, h464, _, _, _, _, _, _, _, _, _) => {
                h464
            }
        };
        let a466 = v1;
        let a467 = 0u64;
        nat_quot(a465, a466, a467)
    }, match v0.clone() {
        Adt2::C0(_, _, h468, _, _, _, _, _, _, _, _) => {
            h468
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h469, _, _, _, _, _, _, _) => {
            h469
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h470, _, _, _, _, _, _) => {
            h470
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h471, _, _, _, _, _) => {
            h471
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h472, _, _, _, _) => {
            h472
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h473, _, _, _) => {
            h473
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h474, _, _) => {
            h474
        }
    })
}

pub fn f14(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f17(v4, v5, v6, v7) {
        f17(v0, v1, v2, v3)
    } else {
        let a478 = {
            let a475 = v0;
            let a476 = v4;
            let a477 = 1u64;
            nat_rem(a475, a476, a477)
        };
        let a479 = 0u64;
        if if if nat_eq(a478, a479) {
            let a483 = {
                let a480 = v1;
                let a481 = v5;
                let a482 = 1u64;
                nat_rem(a480, a481, a482)
            };
            let a484 = 0u64;
            nat_eq(a483, a484)
        } else {
            false
        } {
            let a488 = {
                let a485 = v2;
                let a486 = v6;
                let a487 = 1u64;
                nat_rem(a485, a486, a487)
            };
            let a489 = 0u64;
            nat_eq(a488, a489)
        } else {
            false
        } {
            let a493 = {
                let a490 = v3;
                let a491 = v7;
                let a492 = 1u64;
                nat_rem(a490, a491, a492)
            };
            let a494 = 0u64;
            nat_eq(a493, a494)
        } else {
            false
        }
    }
}

pub fn f15(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a495 = v0;
    let a496 = v1;
    if nat_eq(a495, a496) {
        let a499 = v4;
        let a500 = {
            let a497 = v2;
            let a498 = v3;
            nat_add(a497, a498)?
        };
        Ok(nat_eq(a499, a500))
    } else {
        let a501 = v2;
        let a502 = v3;
        if nat_eq(a501, a502) {
            let a503 = v4;
            let a504 = v2;
            Ok(nat_eq(a503, a504))
        } else {
            Ok(false)
        }
    }
}

pub fn f16(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a507 = {
            let a505 = v0;
            let a506 = f31(v1);
            nat_add(a505, a506)?
        };
        let a508 = f31(v2);
        nat_add(a507, a508)?
    };
    let v5: u64 = {
        let a509 = f32(v1);
        let a510 = f32(v2);
        nat_add(a509, a510)?
    };
    let a511 = v1;
    let a512 = 4294967295u64;
    if if nat_le(a511, a512) {
        let a513 = v2;
        let a514 = 4294967295u64;
        nat_le(a513, a514)
    } else {
        false
    } {
        let a515 = v5;
        let a516 = v4;
        if nat_le(a515, a516) {
            let a519 = v3;
            let a520 = {
                let a517 = v4;
                let a518 = v5;
                nat_sub(a517, a518)
            };
            Ok(nat_eq(a519, a520))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f17(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a521 = v0;
    let a522 = 0u64;
    if if if nat_eq(a521, a522) {
        true
    } else {
        let a523 = v1;
        let a524 = 0u64;
        nat_eq(a523, a524)
    } {
        true
    } else {
        let a525 = v2;
        let a526 = 0u64;
        nat_eq(a525, a526)
    } {
        true
    } else {
        let a527 = v3;
        let a528 = 0u64;
        nat_eq(a527, a528)
    }
}

pub fn f18(v0: u64, v1: u64) -> R<bool> {
    let a529 = v1;
    let a530 = 4294967295u64;
    if nat_le(a529, a530) {
        let a533 = {
            let a531 = f31(v1);
            let a532 = f32(v1);
            nat_add(a531, a532)?
        };
        let a534 = v0;
        Ok(nat_lt(a533, a534))
    } else {
        Ok(false)
    }
}

pub fn f19(v0: Adt2, v1: Adt2) -> bool {
    let a537 = match v0.clone() {
        Adt2::C0(_, h535, _, _, _, _, _, _, _, _, _) => {
            h535
        }
    };
    let a538 = match v1.clone() {
        Adt2::C0(_, h536, _, _, _, _, _, _, _, _, _) => {
            h536
        }
    };
    if if if nat_eq(a537, a538) {
        let a541 = match v0.clone() {
            Adt2::C0(_, _, h539, _, _, _, _, _, _, _, _) => {
                h539
            }
        };
        let a542 = match v1.clone() {
            Adt2::C0(_, _, h540, _, _, _, _, _, _, _, _) => {
                h540
            }
        };
        nat_eq(a541, a542)
    } else {
        false
    } {
        let a545 = match v0.clone() {
            Adt2::C0(_, _, _, h543, _, _, _, _, _, _, _) => {
                h543
            }
        };
        let a546 = match v1.clone() {
            Adt2::C0(_, _, _, h544, _, _, _, _, _, _, _) => {
                h544
            }
        };
        nat_eq(a545, a546)
    } else {
        false
    } {
        let a549 = match v0 {
            Adt2::C0(_, _, _, _, h547, _, _, _, _, _, _) => {
                h547
            }
        };
        let a550 = match v1 {
            Adt2::C0(_, _, _, _, h548, _, _, _, _, _, _) => {
                h548
            }
        };
        nat_eq(a549, a550)
    } else {
        false
    }
}

pub fn f20(v0: Adt2) -> R<bool> {
    let a552 = match v0.clone() {
        Adt2::C0(_, h551, _, _, _, _, _, _, _, _, _) => {
            h551
        }
    };
    let a553 = 1u64;
    if if if if nat_eq(a552, a553) {
        true
    } else {
        let a555 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h554, _, _, _, _, _) => {
                h554
            }
        };
        let a556 = 4u64;
        nat_eq(a555, a556)
    } {
        let a558 = match v0.clone() {
            Adt2::C0(_, _, h557, _, _, _, _, _, _, _, _) => {
                h557
            }
        };
        let a559 = 1u64;
        if nat_eq(a558, a559) {
            true
        } else {
            let a564 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h560, _, _, _, _) => {
                    h560
                }
            };
            let a565 = {
                let a562 = match v0.clone() {
                    Adt2::C0(_, h561, _, _, _, _, _, _, _, _, _) => {
                        h561
                    }
                };
                let a563 = 4u64;
                nat_mul(a562, a563)?
            };
            nat_eq(a564, a565)
        }
    } else {
        false
    } {
        let a567 = match v0.clone() {
            Adt2::C0(_, _, _, h566, _, _, _, _, _, _, _) => {
                h566
            }
        };
        let a568 = 1u64;
        if nat_eq(a567, a568) {
            true
        } else {
            let a576 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h569, _, _, _) => {
                    h569
                }
            };
            let a577 = {
                let a574 = {
                    let a572 = match v0.clone() {
                        Adt2::C0(_, h570, _, _, _, _, _, _, _, _, _) => {
                            h570
                        }
                    };
                    let a573 = match v0.clone() {
                        Adt2::C0(_, _, h571, _, _, _, _, _, _, _, _) => {
                            h571
                        }
                    };
                    nat_mul(a572, a573)?
                };
                let a575 = 4u64;
                nat_mul(a574, a575)?
            };
            nat_eq(a576, a577)
        }
    } else {
        false
    } {
        let a579 = match v0.clone() {
            Adt2::C0(_, _, _, _, h578, _, _, _, _, _, _) => {
                h578
            }
        };
        let a580 = 1u64;
        if nat_eq(a579, a580) {
            Ok(true)
        } else {
            let a591 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h581, _, _) => {
                    h581
                }
            };
            let a592 = {
                let a589 = {
                    let a587 = {
                        let a584 = match v0.clone() {
                            Adt2::C0(_, h582, _, _, _, _, _, _, _, _, _) => {
                                h582
                            }
                        };
                        let a585 = match v0.clone() {
                            Adt2::C0(_, _, h583, _, _, _, _, _, _, _, _) => {
                                h583
                            }
                        };
                        nat_mul(a584, a585)?
                    };
                    let a588 = match v0 {
                        Adt2::C0(_, _, _, h586, _, _, _, _, _, _, _) => {
                            h586
                        }
                    };
                    nat_mul(a587, a588)?
                };
                let a590 = 4u64;
                nat_mul(a589, a590)?
            };
            Ok(nat_eq(a591, a592))
        }
    } else {
        Ok(false)
    }
}

pub fn f21(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f33(v0.clone(), false)?;
    let v6: u64 = f33(v0, true)?;
    let a593 = v6;
    let a594 = v1;
    if if nat_le(a593, a594) {
        let a601 = {
            let a599 = {
                let a595 = v1;
                let a596 = v5;
                nat_add(a595, a596)?
            };
            let a600 = {
                let a597 = v3;
                let a598 = 0u64;
                if nat_eq(a597, a598) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a599, a600)?
        };
        let a602 = v2;
        nat_le(a601, a602)
    } else {
        false
    } {
        if v4 {
            let a607 = {
                let a605 = {
                    let a603 = v1;
                    let a604 = v5;
                    nat_add(a603, a604)?
                };
                let a606 = v6;
                nat_sub(a605, a606)
            };
            let a608 = v2;
            Ok(nat_lt(a607, a608))
        } else {
            let a613 = {
                let a611 = {
                    let a609 = v1;
                    let a610 = v5;
                    nat_add(a609, a610)?
                };
                let a612 = v6;
                nat_sub(a611, a612)
            };
            let a614 = v2;
            Ok(nat_le(a613, a614))
        }
    } else {
        Ok(false)
    }
}

pub fn f22(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt6> {
    let m615 = f34(v0, v1);
    match m615 {
        None => {
            if v5 {
                let m616 = f35(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m616 {
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
                let a617 = 4u64;
                let a618 = v28;
                nat_le(a617, a618)
            };
            let a619 = v2;
            let a620 = 0u64;
            if if if if if nat_eq(a619, a620) {
                let a621 = v4;
                let a622 = 0u64;
                nat_eq(a621, a622)
            } else {
                false
            } {
                let a623 = v6;
                let a624 = v14;
                if nat_eq(a623, a624) {
                    let a625 = v7;
                    let a626 = v15;
                    nat_eq(a625, a626)
                } else {
                    false
                }
            } else {
                false
            } {
                let a627 = v8;
                let a628 = v16;
                if nat_eq(a627, a628) {
                    let a629 = v9;
                    let a630 = v17;
                    nat_eq(a629, a630)
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
                        let a631 = v3;
                        let a632 = 0u64;
                        nat_eq(a631, a632)
                    } else {
                        false
                    } {
                        f14(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt6::C2(v28, v29, v6, v7, v8, v9, {
                    let a637 = {
                        let a633 = v6;
                        let a634 = v7;
                        nat_mul(a633, a634)?
                    };
                    let a638 = {
                        let a635 = v8;
                        let a636 = v9;
                        nat_mul(a635, a636)?
                    };
                    nat_mul(a637, a638)?
                }))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f23(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a640 = v0;
        let a641 = match v1.clone() {
            Adt3::C0(h639, _, _, _, _, _, _, _) => {
                h639
            }
        };
        let a642 = 0u64;
        nat_quot(a640, a641, a642)
    };
    let v3: u64 = {
        let a644 = v2;
        let a645 = match v1.clone() {
            Adt3::C0(_, h643, _, _, _, _, _, _) => {
                h643
            }
        };
        let a646 = 0u64;
        nat_quot(a644, a645, a646)
    };
    f36({
        let a648 = v0;
        let a649 = match v1.clone() {
            Adt3::C0(h647, _, _, _, _, _, _, _) => {
                h647
            }
        };
        let a650 = 0u64;
        nat_rem(a648, a649, a650)
    }, {
        let a652 = v2;
        let a653 = match v1.clone() {
            Adt3::C0(_, h651, _, _, _, _, _, _) => {
                h651
            }
        };
        let a654 = 0u64;
        nat_rem(a652, a653, a654)
    }, {
        let a656 = v3;
        let a657 = match v1.clone() {
            Adt3::C0(_, _, h655, _, _, _, _, _) => {
                h655
            }
        };
        let a658 = 0u64;
        nat_rem(a656, a657, a658)
    }, {
        let a660 = v3;
        let a661 = match v1.clone() {
            Adt3::C0(_, _, h659, _, _, _, _, _) => {
                h659
            }
        };
        let a662 = 0u64;
        nat_quot(a660, a661, a662)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h663, _, _, _) => {
            h663
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h664, _, _) => {
            h664
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h665, _) => {
            h665
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h666) => {
            h666
        }
    })
}

pub fn f24(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a668 = v0;
        let a669 = match v2.clone() {
            Adt3::C0(h667, _, _, _, _, _, _, _) => {
                h667
            }
        };
        let a670 = 0u64;
        nat_quot(a668, a669, a670)
    };
    let v4: u64 = {
        let a672 = v3;
        let a673 = match v2.clone() {
            Adt3::C0(_, h671, _, _, _, _, _, _) => {
                h671
            }
        };
        let a674 = 0u64;
        nat_quot(a672, a673, a674)
    };
    f37({
        let a676 = v0;
        let a677 = match v2.clone() {
            Adt3::C0(h675, _, _, _, _, _, _, _) => {
                h675
            }
        };
        let a678 = 0u64;
        nat_rem(a676, a677, a678)
    }, {
        let a680 = v3;
        let a681 = match v2.clone() {
            Adt3::C0(_, h679, _, _, _, _, _, _) => {
                h679
            }
        };
        let a682 = 0u64;
        nat_rem(a680, a681, a682)
    }, {
        let a684 = v4;
        let a685 = match v2.clone() {
            Adt3::C0(_, _, h683, _, _, _, _, _) => {
                h683
            }
        };
        let a686 = 0u64;
        nat_rem(a684, a685, a686)
    }, {
        let a688 = v4;
        let a689 = match v2 {
            Adt3::C0(_, _, h687, _, _, _, _, _) => {
                h687
            }
        };
        let a690 = 0u64;
        nat_quot(a688, a689, a690)
    }, match v1.clone() {
        Adt3::C0(h691, _, _, _, _, _, _, _) => {
            h691
        }
    }, match v1.clone() {
        Adt3::C0(_, h692, _, _, _, _, _, _) => {
            h692
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h693, _, _, _, _, _) => {
            h693
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h694, _, _, _, _) => {
            h694
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h695, _, _, _) => {
            h695
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h696, _, _) => {
            h696
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h697, _) => {
            h697
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h698) => {
            h698
        }
    })
}

pub fn f25(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a700 = v0;
        let a701 = match v3.clone() {
            Adt3::C0(h699, _, _, _, _, _, _, _) => {
                h699
            }
        };
        let a702 = 0u64;
        nat_quot(a700, a701, a702)
    };
    let v6: u64 = {
        let a704 = v5;
        let a705 = match v3.clone() {
            Adt3::C0(_, h703, _, _, _, _, _, _) => {
                h703
            }
        };
        let a706 = 0u64;
        nat_quot(a704, a705, a706)
    };
    let v7: u64 = {
        let a708 = v0;
        let a709 = match v3.clone() {
            Adt3::C0(h707, _, _, _, _, _, _, _) => {
                h707
            }
        };
        let a710 = 0u64;
        nat_rem(a708, a709, a710)
    };
    let v8: u64 = {
        let a712 = v5;
        let a713 = match v3.clone() {
            Adt3::C0(_, h711, _, _, _, _, _, _) => {
                h711
            }
        };
        let a714 = 0u64;
        nat_rem(a712, a713, a714)
    };
    let v9: u64 = {
        let a716 = v6;
        let a717 = match v3.clone() {
            Adt3::C0(_, _, h715, _, _, _, _, _) => {
                h715
            }
        };
        let a718 = 0u64;
        nat_rem(a716, a717, a718)
    };
    let v10: u64 = {
        let a720 = v6;
        let a721 = match v3.clone() {
            Adt3::C0(_, _, h719, _, _, _, _, _) => {
                h719
            }
        };
        let a722 = 0u64;
        nat_quot(a720, a721, a722)
    };
    let v11: u64 = {
        let a723 = v4;
        let a724 = 0u64;
        if nat_eq(a723, a724) {
            match v1.clone() {
                Adt3::C0(h725, _, _, _, _, _, _, _) => {
                    h725
                }
            }
        } else {
            let a726 = v4;
            let a727 = 1u64;
            if nat_eq(a726, a727) {
                match v1.clone() {
                    Adt3::C0(_, h728, _, _, _, _, _, _) => {
                        h728
                    }
                }
            } else {
                let a729 = v4;
                let a730 = 2u64;
                if nat_eq(a729, a730) {
                    match v1.clone() {
                        Adt3::C0(_, _, h731, _, _, _, _, _) => {
                            h731
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h732, _, _, _, _) => {
                            h732
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a739 = v11;
        let a740 = {
            let a733 = v4;
            let a734 = 0u64;
            if nat_eq(a733, a734) {
                v7
            } else {
                let a735 = v4;
                let a736 = 1u64;
                if nat_eq(a735, a736) {
                    v8
                } else {
                    let a737 = v4;
                    let a738 = 2u64;
                    if nat_eq(a737, a738) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a739, a740)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt5::C1(v12, f36(if if v12 {
        let a741 = v4;
        let a742 = 0u64;
        nat_eq(a741, a742)
    } else {
        false
    } {
        let a743 = v7;
        let a744 = v11;
        nat_sub(a743, a744)
    } else {
        v7
    }, if if v12 {
        let a745 = v4;
        let a746 = 1u64;
        nat_eq(a745, a746)
    } else {
        false
    } {
        let a747 = v8;
        let a748 = v11;
        nat_sub(a747, a748)
    } else {
        v8
    }, if if v12 {
        let a749 = v4;
        let a750 = 2u64;
        nat_eq(a749, a750)
    } else {
        false
    } {
        let a751 = v9;
        let a752 = v11;
        nat_sub(a751, a752)
    } else {
        v9
    }, if if v12 {
        let a753 = v4;
        let a754 = 3u64;
        nat_eq(a753, a754)
    } else {
        false
    } {
        let a755 = v10;
        let a756 = v11;
        nat_sub(a755, a756)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h757, _, _, _) => {
            h757
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h758, _, _) => {
            h758
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h759, _) => {
            h759
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h760) => {
            h760
        }
    })?, f23(v0, v3)?))
}

pub fn f26(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt5> {
    let v8: u64 = f38(v0, v2.clone(), 0u64);
    let v9: u64 = f38(v0, v2.clone(), 1u64);
    let v10: u64 = f38(v0, v2.clone(), 2u64);
    let v11: u64 = f38(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt5::C1(false, f39(v1.clone(), f40(v8, v3, match v1.clone() {
            Adt3::C0(h761, _, _, _, _, _, _, _) => {
                h761
            }
        })?, f40(v9, v4, match v1.clone() {
            Adt3::C0(_, h762, _, _, _, _, _, _) => {
                h762
            }
        })?, f40(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h763, _, _, _, _, _) => {
                h763
            }
        })?, f40(v11, v6, match v1 {
            Adt3::C0(_, _, _, h764, _, _, _, _) => {
                h764
            }
        })?)?, f23(v0, v2)?))
    } else {
        if if if if f41(v8, v3, match v1.clone() {
            Adt3::C0(h765, _, _, _, _, _, _, _) => {
                h765
            }
        })? {
            f41(v9, v4, match v1.clone() {
                Adt3::C0(_, h766, _, _, _, _, _, _) => {
                    h766
                }
            })?
        } else {
            false
        } {
            f41(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h767, _, _, _, _, _) => {
                    h767
                }
            })?
        } else {
            false
        } {
            f41(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h768, _, _, _, _) => {
                    h768
                }
            })?
        } else {
            false
        } {
            Ok(Adt5::C1(false, f39(v1, f42(v8, v3)?, f42(v9, v4)?, f42(v10, v5)?, f42(v11, v6)?)?, f23(v0, v2)?))
        } else {
            Ok(Adt5::C1(true, 0u64, f23(v0, v2)?))
        }
    }
}

pub fn f27(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f38(v0, v2.clone(), 0u64);
    let v5: u64 = f38(v0, v2.clone(), 1u64);
    let v6: u64 = f38(v0, v2.clone(), 2u64);
    let v7: u64 = f38(v0, v2.clone(), 3u64);
    Ok(Adt5::C1(false, f39(v1.clone(), f43(v4, v3, match v1 {
        Adt3::C0(h769, _, _, _, _, _, _, _) => {
            h769
        }
    })?, v5, v6, v7)?, f23(v0, v2)?))
}

pub fn f28(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt5> {
    let a770 = v0;
    let a771 = v3;
    if nat_lt(a770, a771) {
        Ok(Adt5::C1(false, {
            let a772 = v0;
            let a773 = 4u64;
            nat_mul(a772, a773)?
        }, {
            let a774 = v0;
            let a775 = 4u64;
            nat_mul(a774, a775)?
        }))
    } else {
        let v6: u64 = {
            let a776 = v0;
            let a777 = v3;
            nat_sub(a776, a777)
        };
        let v7: u64 = f23(v6, v1)?;
        let v8: u64 = f44(v6, v2, v4)?;
        if v5 {
            Ok(Adt5::C2(v7, v8))
        } else {
            Ok(Adt5::C1(true, v7, v8))
        }
    }
}

pub fn f29(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a778 = v0;
    let a779 = v3;
    if nat_lt(a778, a779) {
        Ok(Adt5::C1(true, 0u64, f23(v0, v2)?))
    } else {
        let v4: u64 = {
            let a780 = v0;
            let a781 = v3;
            nat_sub(a780, a781)
        };
        Ok(Adt5::C3(f23(v4, v1.clone())?, f24(v4, v2, v1)?))
    }
}

pub fn f30(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt5> {
    let v3: u64 = {
        let a783 = v0;
        let a784 = match v2.clone() {
            Adt3::C0(h782, _, _, _, _, _, _, _) => {
                h782
            }
        };
        let a785 = 0u64;
        nat_rem(a783, a784, a785)
    };
    let v4: u64 = {
        let a791 = {
            let a787 = v0;
            let a788 = match v2.clone() {
                Adt3::C0(h786, _, _, _, _, _, _, _) => {
                    h786
                }
            };
            let a789 = 0u64;
            nat_quot(a787, a788, a789)
        };
        let a792 = match v2.clone() {
            Adt3::C0(_, h790, _, _, _, _, _, _) => {
                h790
            }
        };
        let a793 = 0u64;
        nat_rem(a791, a792, a793)
    };
    let v5: bool = {
        let a794 = v3;
        let a795 = v4;
        nat_eq(a794, a795)
    };
    Ok(Adt5::C1(!v5, if v5 {
        f24(v0, v1, v2.clone())?
    } else {
        0u64
    }, f23(v0, v2)?))
}

pub fn f31(v0: u64) -> u64 {
    let a796 = v0;
    let a797 = 2147483648u64;
    if nat_lt(a796, a797) {
        v0
    } else {
        0u64
    }
}

pub fn f32(v0: u64) -> u64 {
    let a798 = v0;
    let a799 = 2147483648u64;
    if nat_lt(a798, a799) {
        0u64
    } else {
        let a800 = 4294967296u64;
        let a801 = v0;
        nat_sub(a800, a801)
    }
}

pub fn f33(v0: Adt3, v1: bool) -> R<u64> {
    f45(v0.clone(), {
        let a803 = match v0.clone() {
            Adt3::C0(h802, _, _, _, _, _, _, _) => {
                h802
            }
        };
        let a804 = 1u64;
        nat_sub(a803, a804)
    }, {
        let a806 = match v0.clone() {
            Adt3::C0(_, h805, _, _, _, _, _, _) => {
                h805
            }
        };
        let a807 = 1u64;
        nat_sub(a806, a807)
    }, {
        let a809 = match v0.clone() {
            Adt3::C0(_, _, h808, _, _, _, _, _) => {
                h808
            }
        };
        let a810 = 1u64;
        nat_sub(a809, a810)
    }, {
        let a812 = match v0 {
            Adt3::C0(_, _, _, h811, _, _, _, _) => {
                h811
            }
        };
        let a813 = 1u64;
        nat_sub(a812, a813)
    }, v1)
}

pub fn f34(v0: u64, v1: u64) -> Option<u64> {
    let a814 = v0;
    let a815 = 2u64;
    if nat_eq(a814, a815) {
        Some(0u64)
    } else {
        let a816 = v0;
        let a817 = 5u64;
        if nat_eq(a816, a817) {
            Some(1u64)
        } else {
            let a818 = v0;
            let a819 = 6u64;
            if nat_eq(a818, a819) {
                Some(2u64)
            } else {
                let a820 = v0;
                let a821 = 7u64;
                if nat_eq(a820, a821) {
                    Some(3u64)
                } else {
                    let a822 = v0;
                    let a823 = 9u64;
                    if nat_eq(a822, a823) {
                        Some(4u64)
                    } else {
                        let a824 = v0;
                        let a825 = 8u64;
                        if nat_eq(a824, a825) {
                            Some(5u64)
                        } else {
                            let a826 = v0;
                            let a827 = 89u64;
                            if nat_eq(a826, a827) {
                                let a828 = v1;
                                let a829 = 2u64;
                                if nat_eq(a828, a829) {
                                    Some(6u64)
                                } else {
                                    let a830 = v1;
                                    let a831 = 0u64;
                                    if nat_eq(a830, a831) {
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

pub fn f35(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt9> {
    let a832 = v0;
    let a833 = 28u64;
    if if if if nat_eq(a832, a833) {
        let a834 = v1;
        let a835 = 0u64;
        nat_eq(a834, a835)
    } else {
        false
    } {
        let a836 = v2;
        let a837 = 0u64;
        nat_eq(a836, a837)
    } else {
        false
    } {
        let a838 = v3;
        let a839 = 0u64;
        nat_eq(a838, a839)
    } else {
        false
    } {
        let a840 = v4;
        let a841 = v8;
        if if if if if if if if nat_eq(a840, a841) {
            let a842 = v5;
            let a843 = v12;
            nat_eq(a842, a843)
        } else {
            false
        } {
            let a844 = v9;
            let a845 = v13;
            nat_eq(a844, a845)
        } else {
            false
        } {
            let a846 = v10;
            let a847 = v14;
            nat_eq(a846, a847)
        } else {
            false
        } {
            let a848 = v11;
            let a849 = v15;
            nat_eq(a848, a849)
        } else {
            false
        } {
            let a850 = 0u64;
            let a851 = v6;
            nat_lt(a850, a851)
        } else {
            false
        } {
            let a852 = 0u64;
            let a853 = v7;
            nat_lt(a852, a853)
        } else {
            false
        } {
            let a857 = {
                let a854 = v10;
                let a855 = v6;
                let a856 = 1u64;
                nat_rem(a854, a855, a856)
            };
            let a858 = 0u64;
            if nat_eq(a857, a858) {
                let a862 = {
                    let a859 = v11;
                    let a860 = v7;
                    let a861 = 1u64;
                    nat_rem(a859, a860, a861)
                };
                let a863 = 0u64;
                nat_eq(a862, a863)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt9::C2(v5, v4, v9, v10, v11, {
                let a864 = v10;
                let a865 = v6;
                let a866 = 0u64;
                nat_quot(a864, a865, a866)
            }, {
                let a867 = v11;
                let a868 = v7;
                let a869 = 0u64;
                nat_quot(a867, a868, a869)
            }, {
                let a870 = v5;
                let a871 = v4;
                nat_mul(a870, a871)?
            }, {
                let a872 = v4;
                let a873 = v9;
                nat_mul(a872, a873)?
            }, {
                let a874 = v5;
                let a875 = v9;
                nat_mul(a874, a875)?
            }))
        } else {
            Ok(Adt9::C1)
        }
    } else {
        Ok(Adt9::C0)
    }
}

pub fn f36(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a888 = {
        let a884 = {
            let a880 = {
                let a876 = v0;
                let a877 = v4;
                nat_mul(a876, a877)?
            };
            let a881 = {
                let a878 = v1;
                let a879 = v5;
                nat_mul(a878, a879)?
            };
            nat_add(a880, a881)?
        };
        let a885 = {
            let a882 = v2;
            let a883 = v6;
            nat_mul(a882, a883)?
        };
        nat_add(a884, a885)?
    };
    let a889 = {
        let a886 = v3;
        let a887 = v7;
        nat_mul(a886, a887)?
    };
    nat_add(a888, a889)
}

pub fn f37(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f36({
        let a890 = v0;
        let a891 = v4;
        let a892 = 0u64;
        nat_rem(a890, a891, a892)
    }, {
        let a893 = v1;
        let a894 = v5;
        let a895 = 0u64;
        nat_rem(a893, a894, a895)
    }, {
        let a896 = v2;
        let a897 = v6;
        let a898 = 0u64;
        nat_rem(a896, a897, a898)
    }, {
        let a899 = v3;
        let a900 = v7;
        let a901 = 0u64;
        nat_rem(a899, a900, a901)
    }, v8, v9, v10, v11)
}

pub fn f38(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a903 = v0;
        let a904 = match v1.clone() {
            Adt3::C0(h902, _, _, _, _, _, _, _) => {
                h902
            }
        };
        let a905 = 0u64;
        nat_quot(a903, a904, a905)
    };
    let v4: u64 = {
        let a907 = v3;
        let a908 = match v1.clone() {
            Adt3::C0(_, h906, _, _, _, _, _, _) => {
                h906
            }
        };
        let a909 = 0u64;
        nat_quot(a907, a908, a909)
    };
    let a910 = v2;
    let a911 = 0u64;
    if nat_eq(a910, a911) {
        let a913 = v0;
        let a914 = match v1 {
            Adt3::C0(h912, _, _, _, _, _, _, _) => {
                h912
            }
        };
        let a915 = 0u64;
        nat_rem(a913, a914, a915)
    } else {
        let a916 = v2;
        let a917 = 1u64;
        if nat_eq(a916, a917) {
            let a919 = v3;
            let a920 = match v1 {
                Adt3::C0(_, h918, _, _, _, _, _, _) => {
                    h918
                }
            };
            let a921 = 0u64;
            nat_rem(a919, a920, a921)
        } else {
            let a922 = v2;
            let a923 = 2u64;
            if nat_eq(a922, a923) {
                let a925 = v4;
                let a926 = match v1 {
                    Adt3::C0(_, _, h924, _, _, _, _, _) => {
                        h924
                    }
                };
                let a927 = 0u64;
                nat_rem(a925, a926, a927)
            } else {
                let a929 = v4;
                let a930 = match v1 {
                    Adt3::C0(_, _, h928, _, _, _, _, _) => {
                        h928
                    }
                };
                let a931 = 0u64;
                nat_quot(a929, a930, a931)
            }
        }
    }
}

pub fn f39(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f36(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h932, _, _, _) => {
            h932
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h933, _, _) => {
            h933
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h934, _) => {
            h934
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h935) => {
            h935
        }
    })
}

pub fn f40(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a936 = v1;
    let a937 = 2147483648u64;
    if nat_lt(a936, a937) {
        let a948 = {
            let a946 = {
                let a941 = {
                    let a938 = v0;
                    let a939 = v2;
                    let a940 = 0u64;
                    nat_rem(a938, a939, a940)
                };
                let a942 = v2;
                nat_add(a941, a942)?
            };
            let a947 = {
                let a943 = v1;
                let a944 = v2;
                let a945 = 0u64;
                nat_rem(a943, a944, a945)
            };
            nat_sub(a946, a947)
        };
        let a949 = v2;
        let a950 = 0u64;
        Ok(nat_rem(a948, a949, a950))
    } else {
        let a953 = {
            let a951 = v0;
            let a952 = f32(v1);
            nat_add(a951, a952)?
        };
        let a954 = v2;
        let a955 = 0u64;
        Ok(nat_rem(a953, a954, a955))
    }
}

pub fn f41(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a958 = f31(v1);
    let a959 = {
        let a956 = v0;
        let a957 = f32(v1);
        nat_add(a956, a957)?
    };
    if nat_le(a958, a959) {
        let a960 = f42(v0, v1)?;
        let a961 = v2;
        Ok(nat_lt(a960, a961))
    } else {
        Ok(false)
    }
}

pub fn f42(v0: u64, v1: u64) -> R<u64> {
    let a964 = {
        let a962 = v0;
        let a963 = f32(v1);
        nat_add(a962, a963)?
    };
    let a965 = f31(v1);
    Ok(nat_sub(a964, a965))
}

pub fn f43(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a966 = v0;
    let a967 = v1;
    if nat_lt(a966, a967) {
        let a968 = v1;
        let a969 = v0;
        Ok(nat_sub(a968, a969))
    } else {
        let a972 = v0;
        let a973 = {
            let a970 = v1;
            let a971 = v2;
            nat_add(a970, a971)?
        };
        if nat_lt(a972, a973) {
            let a974 = v0;
            let a975 = v1;
            Ok(nat_sub(a974, a975))
        } else {
            let a982 = {
                let a978 = 2u64;
                let a979 = {
                    let a976 = v2;
                    let a977 = 1u64;
                    nat_sub(a976, a977)
                };
                nat_mul(a978, a979)?
            };
            let a983 = {
                let a980 = v0;
                let a981 = v1;
                nat_sub(a980, a981)
            };
            Ok(nat_sub(a982, a983))
        }
    }
}

pub fn f44(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a986 = {
        let a984 = v2;
        let a985 = f45(v1.clone(), f38(v0, v1.clone(), 0u64), f38(v0, v1.clone(), 1u64), f38(v0, v1.clone(), 2u64), f38(v0, v1.clone(), 3u64), false)?;
        nat_add(a984, a985)?
    };
    let a987 = f45(v1.clone(), f38(v0, v1.clone(), 0u64), f38(v0, v1.clone(), 1u64), f38(v0, v1.clone(), 2u64), f38(v0, v1, 3u64), true)?;
    Ok(nat_sub(a986, a987))
}

pub fn f45(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a1008 = {
        let a1002 = {
            let a996 = {
                let a990 = v1;
                let a991 = if v5 {
                    f32(match v0.clone() {
                        Adt3::C0(_, _, _, _, h988, _, _, _) => {
                            h988
                        }
                    })
                } else {
                    f31(match v0.clone() {
                        Adt3::C0(_, _, _, _, h989, _, _, _) => {
                            h989
                        }
                    })
                };
                nat_mul(a990, a991)?
            };
            let a997 = {
                let a994 = v2;
                let a995 = if v5 {
                    f32(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h992, _, _) => {
                            h992
                        }
                    })
                } else {
                    f31(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h993, _, _) => {
                            h993
                        }
                    })
                };
                nat_mul(a994, a995)?
            };
            nat_add(a996, a997)?
        };
        let a1003 = {
            let a1000 = v3;
            let a1001 = if v5 {
                f32(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h998, _) => {
                        h998
                    }
                })
            } else {
                f31(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h999, _) => {
                        h999
                    }
                })
            };
            nat_mul(a1000, a1001)?
        };
        nat_add(a1002, a1003)?
    };
    let a1009 = {
        let a1006 = v4;
        let a1007 = if v5 {
            f32(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h1004) => {
                    h1004
                }
            })
        } else {
            f31(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h1005) => {
                    h1005
                }
            })
        };
        nat_mul(a1006, a1007)?
    };
    nat_add(a1008, a1009)
}
