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
    C3(u64, u64, u64, u64, u64, bool, Adt1, Option<Adt1>, Adt1),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64, u64, Option<u64>, u64, u64, u64, u64, Adt1, Adt1, Adt1, bool),
}

#[derive(Clone)]
pub enum Adt3 {
    C0,
    C1,
    C2(Adt2),
    C3(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
    C4(u64, u64, u64, u64, u64),
    C5(u64, u64, u64, u64, bool),
    C6(u64, u64),
    C7(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt4 {
    C0,
    C1(Adt5),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(u64, u64, u64),
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
                                    let a5 = v5;
                                    let a6 = 0u64;
                                    if if nat_eq(a5, a6) {
                                        if f2(match v6.clone() {
                                            Adt1::C0(h7, _, _, _, _, _, _, _, _, _, _) => {
                                                h7
                                            }
                                        }) {
                                            true
                                        } else {
                                            f2(match v4.clone() {
                                                Adt1::C0(h8, _, _, _, _, _, _, _, _, _, _) => {
                                                    h8
                                                }
                                            })
                                        }
                                    } else {
                                        false
                                    } {
                                        f3(v6, v7, v4)
                                    } else {
                                        f4(v5, v6, v7, v4)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Adt0::C1(v26, v27, v28) => {
            if match v27.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h22) => {
                    h22
                }
            } {
                f10(v26, v27)
            } else {
                let a24 = match v27.clone() {
                    Adt2::C0(_, _, _, h23, _, _, _, _, _, _, _) => {
                        h23
                    }
                };
                let a25 = 33u64;
                if nat_eq(a24, a25) {
                    f11(v26, v27)
                } else {
                    let a27 = match v27.clone() {
                        Adt2::C0(_, _, _, h26, _, _, _, _, _, _, _) => {
                            h26
                        }
                    };
                    let a28 = 66u64;
                    if if nat_eq(a27, a28) {
                        true
                    } else {
                        let a30 = match v27.clone() {
                            Adt2::C0(_, _, _, h29, _, _, _, _, _, _, _) => {
                                h29
                            }
                        };
                        let a31 = 67u64;
                        nat_eq(a30, a31)
                    } {
                        f12(v26, v27)
                    } else {
                        let a33 = match v27.clone() {
                            Adt2::C0(_, _, _, h32, _, _, _, _, _, _, _) => {
                                h32
                            }
                        };
                        let a34 = 41u64;
                        if nat_eq(a33, a34) {
                            f13(v26, v27, v28)
                        } else {
                            let a36 = 98u64;
                            let a37 = match v27.clone() {
                                Adt2::C0(_, _, _, h35, _, _, _, _, _, _, _) => {
                                    h35
                                }
                            };
                            if nat_le(a36, a37) {
                                if f14(match v27.clone() {
                                    Adt2::C0(_, _, _, _, h38, _, _, _, _, _, _) => {
                                        h38
                                    }
                                }) {
                                    f15(v26, v27, v28)
                                } else {
                                    f16(v26, v27)
                                }
                            } else {
                                f17(v26, v27, v28)
                            }
                        }
                    }
                }
            }
        }
        Adt0::C2(v8, v9, v10, v11, v12, v13) => {
            let a9 = v8;
            let a10 = 41u64;
            if nat_eq(a9, a10) {
                if v9 {
                    Ok(Adt3::C1)
                } else {
                    f5(v10, v11, v12, v13)
                }
            } else {
                Ok(Adt3::C0)
            }
        }
        Adt0::C3(v14, v15, v16, v17, v18, v19, v20, v21, v22) => {
            let a11 = v14;
            let a12 = 33u64;
            if nat_eq(a11, a12) {
                if v19 {
                    Ok(Adt3::C1)
                } else {
                    let m13 = v21;
                    match m13 {
                        None => {
                            Ok(Adt3::C1)
                        }
                        Some(v23) => {
                            f6(v20, v23, v22)
                        }
                    }
                }
            } else {
                let a14 = v14;
                let a15 = 66u64;
                if if nat_eq(a14, a15) {
                    true
                } else {
                    let a16 = v14;
                    let a17 = 67u64;
                    nat_eq(a16, a17)
                } {
                    if v19 {
                        Ok(Adt3::C1)
                    } else {
                        let m18 = v21;
                        match m18 {
                            None => {
                                f7(v14, v15, v20, v22)
                            }
                            Some(_) => {
                                Ok(Adt3::C1)
                            }
                        }
                    }
                } else {
                    let a19 = v14;
                    let a20 = 98u64;
                    if nat_eq(a19, a20) {
                        if if f8(v15) {
                            !v19
                        } else {
                            false
                        } {
                            let m21 = v21;
                            match m21 {
                                None => {
                                    f9(v15, false, v16, v17, v18, v20.clone(), v20, v22)
                                }
                                Some(v25) => {
                                    f9(v15, true, v16, v17, v18, v20, v25, v22)
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
    }
}

pub fn f1(v0: u64) -> Option<u64> {
    let a39 = v0;
    let a40 = 2u64;
    if nat_eq(a39, a40) {
        Some(0u64)
    } else {
        let a41 = v0;
        let a42 = 5u64;
        if nat_eq(a41, a42) {
            Some(1u64)
        } else {
            let a43 = v0;
            let a44 = 6u64;
            if nat_eq(a43, a44) {
                Some(2u64)
            } else {
                let a45 = v0;
                let a46 = 7u64;
                if nat_eq(a45, a46) {
                    Some(3u64)
                } else {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f2(v0: u64) -> bool {
    let a47 = 6u64;
    let a48 = v0;
    if if if if nat_le(a47, a48) {
        let a49 = v0;
        let a50 = 23u64;
        nat_le(a49, a50)
    } else {
        false
    } {
        true
    } else {
        let a51 = v0;
        let a52 = 2u64;
        if nat_eq(a51, a52) {
            true
        } else {
            let a53 = v0;
            let a54 = 3u64;
            nat_eq(a53, a54)
        }
    } {
        true
    } else {
        let a55 = v0;
        let a56 = 29u64;
        nat_eq(a55, a56)
    } {
        true
    } else {
        let a57 = 34u64;
        let a58 = v0;
        if if nat_le(a57, a58) {
            let a59 = v0;
            let a60 = 35u64;
            nat_le(a59, a60)
        } else {
            false
        } {
            true
        } else {
            let a61 = 39u64;
            let a62 = v0;
            if nat_le(a61, a62) {
                let a63 = v0;
                let a64 = 42u64;
                nat_le(a63, a64)
            } else {
                false
            }
        }
    }
}

pub fn f3(v0: Adt1, v1: Adt1, v2: Adt1) -> R<Adt3> {
    let a67 = match v0.clone() {
        Adt1::C0(_, h65, _, _, _, _, _, _, _, _, _) => {
            h65
        }
    };
    let a68 = match v1.clone() {
        Adt1::C0(_, h66, _, _, _, _, _, _, _, _, _) => {
            h66
        }
    };
    if if if if if if if if if if if if if if if nat_eq(a67, a68) {
        true
    } else {
        let a71 = match v0.clone() {
            Adt1::C0(h69, _, _, _, _, _, _, _, _, _, _) => {
                h69
            }
        };
        let a72 = match v2.clone() {
            Adt1::C0(h70, _, _, _, _, _, _, _, _, _, _) => {
                h70
            }
        };
        nat_eq(a71, a72)
    } {
        f18(v0.clone(), v1.clone())
    } else {
        false
    } {
        f19(v0.clone(), v2.clone())
    } else {
        false
    } {
        let a74 = match v1.clone() {
            Adt1::C0(h73, _, _, _, _, _, _, _, _, _, _) => {
                h73
            }
        };
        let a75 = 0u64;
        nat_eq(a74, a75)
    } else {
        false
    } {
        let a77 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h76, _, _, _, _, _) => {
                h76
            }
        };
        let a78 = 4u64;
        nat_eq(a77, a78)
    } else {
        false
    } {
        f20(match v0.clone() {
            Adt1::C0(h79, _, _, _, _, _, _, _, _, _, _) => {
                h79
            }
        }, match v2.clone() {
            Adt1::C0(h80, _, _, _, _, _, _, _, _, _, _) => {
                h80
            }
        })
    } else {
        false
    } {
        let a82 = 0u64;
        let a83 = f21(match v2.clone() {
            Adt1::C0(h81, _, _, _, _, _, _, _, _, _, _) => {
                h81
            }
        });
        nat_lt(a82, a83)
    } else {
        false
    } {
        let a86 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h84, _, _, _, _, _) => {
                h84
            }
        };
        let a87 = f21(match v0.clone() {
            Adt1::C0(h85, _, _, _, _, _, _, _, _, _, _) => {
                h85
            }
        });
        nat_eq(a86, a87)
    } else {
        false
    } {
        let a90 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h88, _, _, _, _, _) => {
                h88
            }
        };
        let a91 = f21(match v2.clone() {
            Adt1::C0(h89, _, _, _, _, _, _, _, _, _, _) => {
                h89
            }
        });
        nat_eq(a90, a91)
    } else {
        false
    } {
        let a97 = {
            let a94 = match v0.clone() {
                Adt1::C0(_, h92, _, _, _, _, _, _, _, _, _) => {
                    h92
                }
            };
            let a95 = f22(match v0.clone() {
                Adt1::C0(h93, _, _, _, _, _, _, _, _, _, _) => {
                    h93
                }
            });
            let a96 = 1u64;
            nat_rem(a94, a95, a96)
        };
        let a98 = 0u64;
        nat_eq(a97, a98)
    } else {
        false
    } {
        let a104 = {
            let a101 = match v2.clone() {
                Adt1::C0(_, h99, _, _, _, _, _, _, _, _, _) => {
                    h99
                }
            };
            let a102 = f22(match v2.clone() {
                Adt1::C0(h100, _, _, _, _, _, _, _, _, _, _) => {
                    h100
                }
            });
            let a103 = 1u64;
            nat_rem(a101, a102, a103)
        };
        let a105 = 0u64;
        nat_eq(a104, a105)
    } else {
        false
    } {
        f23(v0.clone(), v2.clone())?
    } else {
        false
    } {
        f23(v1.clone(), v2.clone())?
    } else {
        false
    } {
        f24({
            let a112 = {
                let a108 = match v2.clone() {
                    Adt1::C0(_, h106, _, _, _, _, _, _, _, _, _) => {
                        h106
                    }
                };
                let a109 = f22(match v2.clone() {
                    Adt1::C0(h107, _, _, _, _, _, _, _, _, _, _) => {
                        h107
                    }
                });
                let a110 = 0u64;
                nat_quot(a108, a109, a110)
            };
            let a113 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, h111, _, _, _, _, _) => {
                    h111
                }
            };
            nat_mul(a112, a113)?
        }, match v2.clone() {
            Adt1::C0(_, _, h114, _, _, _, _, _, _, _, _) => {
                h114
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h115, _, _, _, _, _, _, _) => {
                h115
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h116, _, _, _, _, _, _) => {
                h116
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h117, _, _, _, _) => {
                h117
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h118, _, _, _) => {
                h118
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h119, _, _) => {
                h119
            }
        })?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0(if f25(v0.clone()) {
            0u64
        } else {
            let a125 = match v0.clone() {
                Adt1::C0(_, _, h120, _, _, _, _, _, _, _, _) => {
                    h120
                }
            };
            let a126 = {
                let a123 = match v0.clone() {
                    Adt1::C0(_, _, _, h121, _, _, _, _, _, _, _) => {
                        h121
                    }
                };
                let a124 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h122, _, _, _, _, _, _) => {
                        h122
                    }
                };
                nat_mul(a123, a124)?
            };
            nat_mul(a125, a126)?
        }, 3u64, f26(match v2.clone() {
            Adt1::C0(h127, _, _, _, _, _, _, _, _, _, _) => {
                h127
            }
        }), 0u64, 0u64, 0u64, 0u64, v0, v1, v2, true)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f4(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if f19(v1.clone(), v3.clone()) {
        f27(v0, v1.clone(), v2.clone(), v3.clone())?
    } else {
        false
    } {
        if f25(v1.clone()) {
            true
        } else {
            let a130 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, h128, _, _, _, _, _) => {
                    h128
                }
            };
            let a131 = f28(match v1.clone() {
                Adt1::C0(h129, _, _, _, _, _, _, _, _, _, _) => {
                    h129
                }
            });
            if if if if if if if nat_eq(a130, a131) {
                let a134 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h132, _, _, _, _, _) => {
                        h132
                    }
                };
                let a135 = f28(match v3.clone() {
                    Adt1::C0(h133, _, _, _, _, _, _, _, _, _, _) => {
                        h133
                    }
                });
                nat_eq(a134, a135)
            } else {
                false
            } {
                f29(v1.clone())
            } else {
                false
            } {
                f29(v2.clone())
            } else {
                false
            } {
                f29(v3.clone())
            } else {
                false
            } {
                f30(v1.clone(), v2.clone())?
            } else {
                false
            } {
                f23(v1.clone(), v3.clone())?
            } else {
                false
            } {
                f23(v2.clone(), v3.clone())?
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v4: u64 = if if f25(v1.clone()) {
            true
        } else {
            f31(v3.clone())?
        } {
            0u64
        } else {
            let a138 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h136, _) => {
                    h136
                }
            };
            let a139 = f28(match v3.clone() {
                Adt1::C0(h137, _, _, _, _, _, _, _, _, _, _) => {
                    h137
                }
            });
            let a140 = 0u64;
            nat_quot(a138, a139, a140)
        };
        Ok(Adt3::C2(Adt2::C0(f32(f33(v1.clone())?, v4, f34(match v1.clone() {
            Adt1::C0(h141, _, _, _, _, _, _, _, _, _, _) => {
                h141
            }
        }, match v2.clone() {
            Adt1::C0(h142, _, _, _, _, _, _, _, _, _, _) => {
                h142
            }
        }, match v3.clone() {
            Adt1::C0(h143, _, _, _, _, _, _, _, _, _, _) => {
                h143
            }
        })?)?, {
            let a144 = 3u64;
            let a145 = v4;
            nat_add(a144, a145)?
        }, None::<u64>, v0, 0u64, 0u64, 0u64, v1, v2, v3, false)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f5(v0: Adt1, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if if if if if if if if if if if if if f35(v2, v3.clone()) {
        let a147 = match v0.clone() {
            Adt1::C0(h146, _, _, _, _, _, _, _, _, _, _) => {
                h146
            }
        };
        let a148 = 0u64;
        if nat_eq(a147, a148) {
            true
        } else {
            let a150 = match v0.clone() {
                Adt1::C0(h149, _, _, _, _, _, _, _, _, _, _) => {
                    h149
                }
            };
            let a151 = 1u64;
            nat_eq(a150, a151)
        }
    } else {
        false
    } {
        let a153 = 0u64;
        let a154 = f21(match v3.clone() {
            Adt1::C0(h152, _, _, _, _, _, _, _, _, _, _) => {
                h152
            }
        });
        nat_lt(a153, a154)
    } else {
        false
    } {
        let a156 = 0u64;
        let a157 = f36(match v1.clone() {
            Adt1::C0(h155, _, _, _, _, _, _, _, _, _, _) => {
                h155
            }
        });
        nat_lt(a156, a157)
    } else {
        false
    } {
        let a160 = match v0.clone() {
            Adt1::C0(_, h158, _, _, _, _, _, _, _, _, _) => {
                h158
            }
        };
        let a161 = match v3.clone() {
            Adt1::C0(_, h159, _, _, _, _, _, _, _, _, _) => {
                h159
            }
        };
        if nat_eq(a160, a161) {
            let a167 = {
                let a164 = match v0.clone() {
                    Adt1::C0(_, h162, _, _, _, _, _, _, _, _, _) => {
                        h162
                    }
                };
                let a165 = f22(match v3.clone() {
                    Adt1::C0(h163, _, _, _, _, _, _, _, _, _, _) => {
                        h163
                    }
                });
                let a166 = 1u64;
                nat_rem(a164, a165, a166)
            };
            let a168 = 0u64;
            nat_eq(a167, a168)
        } else {
            false
        }
    } else {
        false
    } {
        let a171 = match v0.clone() {
            Adt1::C0(_, _, _, h169, _, _, _, _, _, _, _) => {
                h169
            }
        };
        let a172 = match v3.clone() {
            Adt1::C0(_, _, _, h170, _, _, _, _, _, _, _) => {
                h170
            }
        };
        nat_eq(a171, a172)
    } else {
        false
    } {
        let a175 = match v0.clone() {
            Adt1::C0(_, _, _, _, h173, _, _, _, _, _, _) => {
                h173
            }
        };
        let a176 = match v3.clone() {
            Adt1::C0(_, _, _, _, h174, _, _, _, _, _, _) => {
                h174
            }
        };
        nat_eq(a175, a176)
    } else {
        false
    } {
        let a179 = match v0.clone() {
            Adt1::C0(_, _, h177, _, _, _, _, _, _, _, _) => {
                h177
            }
        };
        let a180 = match v1.clone() {
            Adt1::C0(_, h178, _, _, _, _, _, _, _, _, _) => {
                h178
            }
        };
        nat_eq(a179, a180)
    } else {
        false
    } {
        let a182 = match v1.clone() {
            Adt1::C0(_, _, _, _, h181, _, _, _, _, _, _) => {
                h181
            }
        };
        let a183 = 1u64;
        nat_eq(a182, a183)
    } else {
        false
    } {
        let a185 = 0u64;
        let a186 = match v1.clone() {
            Adt1::C0(_, _, h184, _, _, _, _, _, _, _, _) => {
                h184
            }
        };
        nat_lt(a185, a186)
    } else {
        false
    } {
        let a188 = 0u64;
        let a189 = match v1.clone() {
            Adt1::C0(_, _, _, h187, _, _, _, _, _, _, _) => {
                h187
            }
        };
        nat_lt(a188, a189)
    } else {
        false
    } {
        let a195 = {
            let a192 = match v0.clone() {
                Adt1::C0(_, _, _, h190, _, _, _, _, _, _, _) => {
                    h190
                }
            };
            let a193 = match v1.clone() {
                Adt1::C0(_, _, h191, _, _, _, _, _, _, _, _) => {
                    h191
                }
            };
            let a194 = 0u64;
            nat_rem(a192, a193, a194)
        };
        let a196 = 0u64;
        nat_eq(a195, a196)
    } else {
        false
    } {
        let a202 = {
            let a199 = match v0.clone() {
                Adt1::C0(_, _, _, _, h197, _, _, _, _, _, _) => {
                    h197
                }
            };
            let a200 = match v1.clone() {
                Adt1::C0(_, _, _, h198, _, _, _, _, _, _, _) => {
                    h198
                }
            };
            let a201 = 0u64;
            nat_rem(a199, a200, a201)
        };
        let a203 = 0u64;
        nat_eq(a202, a203)
    } else {
        false
    } {
        f37(v0.clone())
    } else {
        false
    } {
        let a205 = match v3.clone() {
            Adt1::C0(_, h204, _, _, _, _, _, _, _, _, _) => {
                h204
            }
        };
        let a206 = 1u64;
        if nat_eq(a205, a206) {
            true
        } else {
            let a209 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h207, _, _, _, _, _) => {
                    h207
                }
            };
            let a210 = f21(match v3.clone() {
                Adt1::C0(h208, _, _, _, _, _, _, _, _, _, _) => {
                    h208
                }
            });
            nat_eq(a209, a210)
        }
    } else {
        false
    } {
        let v4: u64 = f33(v1.clone())?;
        let v5: u64 = {
            let a216 = match v0.clone() {
                Adt1::C0(_, _, h211, _, _, _, _, _, _, _, _) => {
                    h211
                }
            };
            let a217 = {
                let a214 = match v0.clone() {
                    Adt1::C0(_, _, _, h212, _, _, _, _, _, _, _) => {
                        h212
                    }
                };
                let a215 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h213, _, _, _, _, _, _) => {
                        h213
                    }
                };
                nat_mul(a214, a215)?
            };
            nat_mul(a216, a217)?
        };
        let v6: u64 = if f25(v0.clone()) {
            0u64
        } else {
            let a225 = {
                let a218 = 2u64;
                let a219 = v4;
                nat_mul(a218, a219)?
            };
            let a226 = {
                let a223 = v5;
                let a224 = {
                    let a221 = 1u64;
                    let a222 = f38(v0.clone(), match v3.clone() {
                        Adt1::C0(h220, _, _, _, _, _, _, _, _, _, _) => {
                            h220
                        }
                    })?;
                    nat_add(a221, a222)?
                };
                nat_mul(a223, a224)?
            };
            nat_add(a225, a226)?
        };
        Ok(Adt3::C2(Adt2::C0(v6, if f25(v0.clone()) {
            3u64
        } else {
            let a227 = 3u64;
            let a228 = v4;
            nat_add(a227, a228)?
        }, f26(match v3.clone() {
            Adt1::C0(h229, _, _, _, _, _, _, _, _, _, _) => {
                h229
            }
        }), 41u64, 0u64, 0u64, 0u64, v0, v1, v3, false)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f6(v0: Adt1, v1: Adt1, v2: Adt1) -> R<Adt3> {
    let v3: u64 = f22(match v2.clone() {
        Adt1::C0(h230, _, _, _, _, _, _, _, _, _, _) => {
            h230
        }
    });
    if if if if if if if f2(match v2.clone() {
        Adt1::C0(h231, _, _, _, _, _, _, _, _, _, _) => {
            h231
        }
    }) {
        f35(v1.clone(), v2.clone())
    } else {
        false
    } {
        let a233 = 0u64;
        let a234 = f39(match v0.clone() {
            Adt1::C0(h232, _, _, _, _, _, _, _, _, _, _) => {
                h232
            }
        });
        nat_lt(a233, a234)
    } else {
        false
    } {
        let a237 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h235, _, _, _, _, _) => {
                h235
            }
        };
        let a238 = f39(match v0.clone() {
            Adt1::C0(h236, _, _, _, _, _, _, _, _, _, _) => {
                h236
            }
        });
        nat_eq(a237, a238)
    } else {
        false
    } {
        let a243 = {
            let a240 = match v0.clone() {
                Adt1::C0(_, h239, _, _, _, _, _, _, _, _, _) => {
                    h239
                }
            };
            let a241 = v3;
            let a242 = 1u64;
            nat_rem(a240, a241, a242)
        };
        let a244 = 0u64;
        if nat_eq(a243, a244) {
            let a249 = {
                let a246 = match v2.clone() {
                    Adt1::C0(_, h245, _, _, _, _, _, _, _, _, _) => {
                        h245
                    }
                };
                let a247 = v3;
                let a248 = 1u64;
                nat_rem(a246, a247, a248)
            };
            let a250 = 0u64;
            nat_eq(a249, a250)
        } else {
            false
        }
    } else {
        false
    } {
        let a253 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h251, _, _, _, _, _) => {
                h251
            }
        };
        let a254 = f21(match v2.clone() {
            Adt1::C0(h252, _, _, _, _, _, _, _, _, _, _) => {
                h252
            }
        });
        nat_eq(a253, a254)
    } else {
        false
    } {
        let a255 = f33(v0.clone())?;
        let a256 = f33(v2.clone())?;
        nat_eq(a255, a256)
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a257 = f33(v0.clone())?;
            let a258 = v3;
            let a259 = 0u64;
            nat_quot(a257, a258, a259)
        }, 3u64, f26(match v2.clone() {
            Adt1::C0(h260, _, _, _, _, _, _, _, _, _, _) => {
                h260
            }
        }), 33u64, 0u64, 0u64, 0u64, v0, v1, v2, false)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f7(v0: u64, v1: u64, v2: Adt1, v3: Adt1) -> R<Adt3> {
    let a262 = match v2.clone() {
        Adt1::C0(h261, _, _, _, _, _, _, _, _, _, _) => {
            h261
        }
    };
    let a263 = 0u64;
    if if if if if if if if if if if nat_eq(a262, a263) {
        let a265 = match v3.clone() {
            Adt1::C0(h264, _, _, _, _, _, _, _, _, _, _) => {
                h264
            }
        };
        let a266 = 26u64;
        nat_eq(a265, a266)
    } else {
        false
    } {
        let a268 = match v2.clone() {
            Adt1::C0(_, h267, _, _, _, _, _, _, _, _, _) => {
                h267
            }
        };
        let a269 = 2147483647u64;
        nat_le(a268, a269)
    } else {
        false
    } {
        let a272 = match v3.clone() {
            Adt1::C0(_, h270, _, _, _, _, _, _, _, _, _) => {
                h270
            }
        };
        let a273 = match v2.clone() {
            Adt1::C0(_, h271, _, _, _, _, _, _, _, _, _) => {
                h271
            }
        };
        nat_le(a272, a273)
    } else {
        false
    } {
        let a276 = match v2.clone() {
            Adt1::C0(_, _, h274, _, _, _, _, _, _, _, _) => {
                h274
            }
        };
        let a277 = match v3.clone() {
            Adt1::C0(_, _, h275, _, _, _, _, _, _, _, _) => {
                h275
            }
        };
        nat_eq(a276, a277)
    } else {
        false
    } {
        let a280 = match v2.clone() {
            Adt1::C0(_, _, _, h278, _, _, _, _, _, _, _) => {
                h278
            }
        };
        let a281 = match v3.clone() {
            Adt1::C0(_, _, _, h279, _, _, _, _, _, _, _) => {
                h279
            }
        };
        nat_eq(a280, a281)
    } else {
        false
    } {
        let a284 = match v2.clone() {
            Adt1::C0(_, _, _, _, h282, _, _, _, _, _, _) => {
                h282
            }
        };
        let a285 = match v3.clone() {
            Adt1::C0(_, _, _, _, h283, _, _, _, _, _, _) => {
                h283
            }
        };
        nat_eq(a284, a285)
    } else {
        false
    } {
        let a286 = v0;
        let a287 = 67u64;
        if nat_eq(a286, a287) {
            true
        } else {
            let a288 = v1;
            let a289 = 1u64;
            if nat_le(a288, a289) {
                let a292 = match v3.clone() {
                    Adt1::C0(_, h290, _, _, _, _, _, _, _, _, _) => {
                        h290
                    }
                };
                let a293 = match v2.clone() {
                    Adt1::C0(_, h291, _, _, _, _, _, _, _, _, _) => {
                        h291
                    }
                };
                nat_eq(a292, a293)
            } else {
                false
            }
        }
    } else {
        false
    } {
        f40(v2.clone())?
    } else {
        false
    } {
        f40(v3.clone())?
    } else {
        false
    } {
        f41(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a299 = match v3.clone() {
                Adt1::C0(_, _, h294, _, _, _, _, _, _, _, _) => {
                    h294
                }
            };
            let a300 = {
                let a297 = match v3.clone() {
                    Adt1::C0(_, _, _, h295, _, _, _, _, _, _, _) => {
                        h295
                    }
                };
                let a298 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h296, _, _, _, _, _, _) => {
                        h296
                    }
                };
                nat_mul(a297, a298)?
            };
            nat_mul(a299, a300)?
        }, 3u64, None::<u64>, v0, {
            let a301 = v0;
            let a302 = 67u64;
            if nat_eq(a301, a302) {
                1u64
            } else {
                v1
            }
        }, 0u64, 0u64, v2.clone(), v2, v3, false)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f8(v0: u64) -> bool {
    let a303 = v0;
    let a304 = 6u64;
    nat_le(a303, a304)
}

pub fn f9(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1) -> R<Adt3> {
    let a305 = v2;
    let a306 = 1u64;
    if if if if if if if if if if if if if if if nat_le(a305, a306) {
        let a308 = 0u64;
        let a309 = f39(match v5.clone() {
            Adt1::C0(h307, _, _, _, _, _, _, _, _, _, _) => {
                h307
            }
        });
        nat_lt(a308, a309)
    } else {
        false
    } {
        let a312 = match v5.clone() {
            Adt1::C0(h310, _, _, _, _, _, _, _, _, _, _) => {
                h310
            }
        };
        let a313 = match v6.clone() {
            Adt1::C0(h311, _, _, _, _, _, _, _, _, _, _) => {
                h311
            }
        };
        nat_eq(a312, a313)
    } else {
        false
    } {
        let a316 = match v5.clone() {
            Adt1::C0(h314, _, _, _, _, _, _, _, _, _, _) => {
                h314
            }
        };
        let a317 = match v7.clone() {
            Adt1::C0(h315, _, _, _, _, _, _, _, _, _, _) => {
                h315
            }
        };
        nat_eq(a316, a317)
    } else {
        false
    } {
        f19(v5.clone(), v6.clone())
    } else {
        false
    } {
        f42(v5.clone())?
    } else {
        false
    } {
        f42(v6.clone())?
    } else {
        false
    } {
        f42(v7.clone())?
    } else {
        false
    } {
        let a320 = match v5.clone() {
            Adt1::C0(_, _, h318, _, _, _, _, _, _, _, _) => {
                h318
            }
        };
        let a321 = match v7.clone() {
            Adt1::C0(_, _, h319, _, _, _, _, _, _, _, _) => {
                h319
            }
        };
        nat_eq(a320, a321)
    } else {
        false
    } {
        let a324 = match v5.clone() {
            Adt1::C0(_, _, _, h322, _, _, _, _, _, _, _) => {
                h322
            }
        };
        let a325 = match v7.clone() {
            Adt1::C0(_, _, _, h323, _, _, _, _, _, _, _) => {
                h323
            }
        };
        nat_eq(a324, a325)
    } else {
        false
    } {
        let a328 = match v5.clone() {
            Adt1::C0(_, _, _, _, h326, _, _, _, _, _, _) => {
                h326
            }
        };
        let a329 = match v7.clone() {
            Adt1::C0(_, _, _, _, h327, _, _, _, _, _, _) => {
                h327
            }
        };
        nat_eq(a328, a329)
    } else {
        false
    } {
        let a336 = match v7.clone() {
            Adt1::C0(_, h330, _, _, _, _, _, _, _, _, _) => {
                h330
            }
        };
        let a337 = if v1 {
            match v5.clone() {
                Adt1::C0(_, h331, _, _, _, _, _, _, _, _, _) => {
                    h331
                }
            }
        } else {
            let a333 = match v5.clone() {
                Adt1::C0(_, h332, _, _, _, _, _, _, _, _, _) => {
                    h332
                }
            };
            let a334 = 2u64;
            let a335 = 0u64;
            nat_quot(a333, a334, a335)
        };
        nat_eq(a336, a337)
    } else {
        false
    } {
        let a339 = match v7.clone() {
            Adt1::C0(_, h338, _, _, _, _, _, _, _, _, _) => {
                h338
            }
        };
        let a340 = 2147483647u64;
        nat_le(a339, a340)
    } else {
        false
    } {
        f23(v5.clone(), v7.clone())?
    } else {
        false
    } {
        f23(v6.clone(), v7.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0(if f14(v0) {
            let a343 = 1u64;
            let a344 = {
                let a341 = f43(v0);
                let a342 = f33(v7.clone())?;
                nat_mul(a341, a342)?
            };
            nat_add(a343, a344)?
        } else {
            let a345 = 4u64;
            let a346 = f33(v7.clone())?;
            nat_mul(a345, a346)?
        }, if f14(v0) {
            4u64
        } else {
            3u64
        }, None::<u64>, if v1 {
            98u64
        } else {
            let a347 = 99u64;
            let a348 = v2;
            nat_add(a347, a348)?
        }, v0, v3, v4, v5, v6, v7, false)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f10(v0: u64, v1: Adt2) -> R<Adt3> {
    let a350 = v0;
    let a351 = match v1.clone() {
        Adt2::C0(h349, _, _, _, _, _, _, _, _, _, _) => {
            h349
        }
    };
    if nat_lt(a350, a351) {
        let v2: u64 = {
            let a355 = v0;
            let a356 = {
                let m354 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h353, _, _, _) => {
                        h353
                    }
                };
                match m354 {
                    Adt1::C0(_, h352, _, _, _, _, _, _, _, _, _) => {
                        h352
                    }
                }
            };
            nat_mul(a355, a356)?
        };
        Ok(Adt3::C7(f44(v2, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h357, _, _, _) => {
                h357
            }
        })?, f45(v2, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h358, _, _, _) => {
                h358
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h359, _, _) => {
                h359
            }
        })?, f44(v2, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h360, _) => {
                h360
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f11(v0: u64, v1: Adt2) -> R<Adt3> {
    let a362 = v0;
    let a363 = match v1.clone() {
        Adt2::C0(h361, _, _, _, _, _, _, _, _, _, _) => {
            h361
        }
    };
    if nat_lt(a362, a363) {
        let v2: Adt1 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h364, _) => {
                h364
            }
        };
        let v3: u64 = f22(match v2.clone() {
            Adt1::C0(h365, _, _, _, _, _, _, _, _, _, _) => {
                h365
            }
        });
        let v4: u64 = {
            let a367 = match v2.clone() {
                Adt1::C0(_, h366, _, _, _, _, _, _, _, _, _) => {
                    h366
                }
            };
            let a368 = v3;
            let a369 = 0u64;
            nat_quot(a367, a368, a369)
        };
        let v5: u64 = {
            let a370 = v0;
            let a371 = v4;
            let a372 = 0u64;
            nat_quot(a370, a371, a372)
        };
        let v6: u64 = {
            let a374 = v5;
            let a375 = match v2.clone() {
                Adt1::C0(_, _, h373, _, _, _, _, _, _, _, _) => {
                    h373
                }
            };
            let a376 = 0u64;
            nat_quot(a374, a375, a376)
        };
        Ok(Adt3::C6(f44({
            let a377 = v0;
            let a378 = v3;
            nat_mul(a377, a378)?
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, h379, _, _, _) => {
                h379
            }
        })?, f46({
            let a380 = v0;
            let a381 = v4;
            let a382 = 0u64;
            nat_rem(a380, a381, a382)
        }, {
            let a384 = v5;
            let a385 = match v2.clone() {
                Adt1::C0(_, _, h383, _, _, _, _, _, _, _, _) => {
                    h383
                }
            };
            let a386 = 0u64;
            nat_rem(a384, a385, a386)
        }, {
            let a388 = v6;
            let a389 = match v2.clone() {
                Adt1::C0(_, _, _, h387, _, _, _, _, _, _, _) => {
                    h387
                }
            };
            let a390 = 0u64;
            nat_rem(a388, a389, a390)
        }, {
            let a392 = v6;
            let a393 = match v2.clone() {
                Adt1::C0(_, _, _, h391, _, _, _, _, _, _, _) => {
                    h391
                }
            };
            let a394 = 0u64;
            nat_quot(a392, a393, a394)
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, h395, _, _, _, _, _) => {
                h395
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h396, _, _, _, _) => {
                h396
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h397, _, _, _) => {
                h397
            }
        }, match v2 {
            Adt1::C0(_, _, _, _, _, _, _, _, h398, _, _) => {
                h398
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f12(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h399, _, _, _) => {
            h399
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h400, _) => {
            h400
        }
    };
    let a402 = v0;
    let a403 = match v1.clone() {
        Adt2::C0(h401, _, _, _, _, _, _, _, _, _, _) => {
            h401
        }
    };
    if nat_lt(a402, a403) {
        Ok(Adt3::C5({
            let a405 = v0;
            let a406 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, _, h404, _, _, _, _) => {
                    h404
                }
            };
            nat_mul(a405, a406)?
        }, {
            let a408 = v0;
            let a409 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h407, _, _, _, _) => {
                    h407
                }
            };
            nat_mul(a408, a409)?
        }, match v2 {
            Adt1::C0(_, h410, _, _, _, _, _, _, _, _, _) => {
                h410
            }
        }, match v3 {
            Adt1::C0(_, h411, _, _, _, _, _, _, _, _, _) => {
                h411
            }
        }, {
            let a413 = match v1 {
                Adt2::C0(_, _, _, _, h412, _, _, _, _, _, _) => {
                    h412
                }
            };
            let a414 = 1u64;
            nat_eq(a413, a414)
        }))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f13(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let a416 = v0;
    let a417 = match v1.clone() {
        Adt2::C0(h415, _, _, _, _, _, _, _, _, _, _) => {
            h415
        }
    };
    if nat_lt(a416, a417) {
        let v3: u64 = {
            let a419 = 2u64;
            let a420 = f33(match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h418, _, _) => {
                    h418
                }
            })?;
            nat_mul(a419, a420)?
        };
        let a421 = v0;
        let a422 = v3;
        if nat_lt(a421, a422) {
            f47(v0, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h423, _, _) => {
                    h423
                }
            }, {
                let m426 = match v1 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h425, _) => {
                        h425
                    }
                };
                match m426 {
                    Adt1::C0(_, _, h424, _, _, _, _, _, _, _, _) => {
                        h424
                    }
                }
            }, v2)
        } else {
            f48({
                let a427 = v0;
                let a428 = v3;
                nat_sub(a427, a428)
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h429, _, _, _) => {
                    h429
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h430, _, _) => {
                    h430
                }
            }, match v1 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h431, _) => {
                    h431
                }
            }, v2)
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f14(v0: u64) -> bool {
    let a432 = v0;
    let a433 = 3u64;
    if nat_eq(a432, a433) {
        true
    } else {
        let a434 = v0;
        let a435 = 6u64;
        nat_eq(a434, a435)
    }
}

pub fn f15(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h436, _, _, _, _) => {
            h436
        }
    };
    let a438 = v0;
    let a439 = match v1.clone() {
        Adt2::C0(h437, _, _, _, _, _, _, _, _, _, _) => {
            h437
        }
    };
    if nat_lt(a438, a439) {
        let a440 = v0;
        let a441 = 0u64;
        if nat_eq(a440, a441) {
            Ok(Adt3::C3(1u64, 6u64, 0u64, 0u64, 2u64, v3, 2u64, 0u64, 0u64, 3u64, 4u64, 4u64))
        } else {
            let v4: u64 = {
                let a442 = v0;
                let a443 = 1u64;
                nat_sub(a442, a443)
            };
            let v5: u64 = {
                let a445 = v4;
                let a446 = f43(match v1.clone() {
                    Adt2::C0(_, _, _, _, h444, _, _, _, _, _, _) => {
                        h444
                    }
                });
                let a447 = 0u64;
                nat_rem(a445, a446, a447)
            };
            let v6: u64 = {
                let a452 = 4u64;
                let a453 = {
                    let a449 = v4;
                    let a450 = f43(match v1.clone() {
                        Adt2::C0(_, _, _, _, h448, _, _, _, _, _, _) => {
                            h448
                        }
                    });
                    let a451 = 0u64;
                    nat_quot(a449, a450, a451)
                };
                nat_mul(a452, a453)?
            };
            let a454 = v5;
            let a455 = 2u64;
            if nat_lt(a454, a455) {
                f16({
                    let a456 = v6;
                    let a457 = v5;
                    nat_add(a456, a457)?
                }, v1)
            } else {
                let a458 = v5;
                let a459 = 2u64;
                if nat_eq(a458, a459) {
                    Ok(f49(0u64, 2u64, v3))
                } else {
                    let a460 = v5;
                    let a461 = 3u64;
                    if nat_eq(a460, a461) {
                        Ok(f50(0u64, 2u64, v3, {
                            let a462 = v2;
                            let a463 = 2u64;
                            nat_eq(a462, a463)
                        }))
                    } else {
                        let a464 = v5;
                        let a465 = 4u64;
                        if nat_eq(a464, a465) {
                            Ok(f49(1u64, 1u64, 3u64))
                        } else {
                            let a466 = v5;
                            let a467 = 5u64;
                            if nat_eq(a466, a467) {
                                Ok(f50(1u64, 1u64, 3u64, {
                                    let a468 = v2;
                                    let a469 = 0u64;
                                    nat_eq(a468, a469)
                                }))
                            } else {
                                let a470 = v5;
                                let a471 = 6u64;
                                if nat_eq(a470, a471) {
                                    Ok(f49(1u64, 2u64, v3))
                                } else {
                                    let a472 = v5;
                                    let a473 = 7u64;
                                    if nat_eq(a472, a473) {
                                        Ok(f50(1u64, 2u64, v3, {
                                            let a474 = v2;
                                            let a475 = 2u64;
                                            nat_eq(a474, a475)
                                        }))
                                    } else {
                                        let a477 = match v1.clone() {
                                            Adt2::C0(_, _, _, _, h476, _, _, _, _, _, _) => {
                                                h476
                                            }
                                        };
                                        let a478 = 3u64;
                                        if nat_eq(a477, a478) {
                                            let a479 = v5;
                                            let a480 = 15u64;
                                            if nat_eq(a479, a480) {
                                                f16({
                                                    let a481 = v6;
                                                    let a482 = 3u64;
                                                    nat_add(a481, a482)?
                                                }, v1)
                                            } else {
                                                Ok(f51(v5, match v1 {
                                                    Adt2::C0(_, _, _, _, _, h483, _, _, _, _, _) => {
                                                        h483
                                                    }
                                                }))
                                            }
                                        } else {
                                            let a484 = v5;
                                            let a485 = 8u64;
                                            if nat_eq(a484, a485) {
                                                Ok(Adt3::C4(2u64, {
                                                    let m488 = match v1 {
                                                        Adt2::C0(_, _, _, _, _, _, _, _, _, h487, _) => {
                                                            h487
                                                        }
                                                    };
                                                    match m488 {
                                                        Adt1::C0(h486, _, _, _, _, _, _, _, _, _, _) => {
                                                            h486
                                                        }
                                                    }
                                                }, 0u64, 1u64, 0u64))
                                            } else {
                                                f16({
                                                    let a489 = v6;
                                                    let a490 = 3u64;
                                                    nat_add(a489, a490)?
                                                }, v1)
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
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f16(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h491, _, _, _) => {
            h491
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h492, _, _) => {
            h492
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h493, _) => {
            h493
        }
    };
    let v5: u64 = match v1.clone() {
        Adt2::C0(_, _, _, h494, _, _, _, _, _, _, _) => {
            h494
        }
    };
    let v6: u64 = {
        let a495 = v0;
        let a496 = 4u64;
        let a497 = 0u64;
        nat_rem(a495, a496, a497)
    };
    let v7: u64 = {
        let a498 = v0;
        let a499 = 4u64;
        let a500 = 0u64;
        nat_quot(a498, a499, a500)
    };
    let v8: u64 = {
        let a502 = v7;
        let a503 = match v4.clone() {
            Adt1::C0(_, h501, _, _, _, _, _, _, _, _, _) => {
                h501
            }
        };
        let a504 = 0u64;
        nat_quot(a502, a503, a504)
    };
    let v9: u64 = f39(match v4.clone() {
        Adt1::C0(h505, _, _, _, _, _, _, _, _, _, _) => {
            h505
        }
    });
    let v10: u64 = {
        let a510 = {
            let a507 = v7;
            let a508 = match v4.clone() {
                Adt1::C0(_, h506, _, _, _, _, _, _, _, _, _) => {
                    h506
                }
            };
            let a509 = 0u64;
            nat_rem(a507, a508, a509)
        };
        let a511 = v9;
        nat_mul(a510, a511)?
    };
    let v11: u64 = {
        let a513 = match v4.clone() {
            Adt1::C0(_, h512, _, _, _, _, _, _, _, _, _) => {
                h512
            }
        };
        let a514 = v9;
        nat_mul(a513, a514)?
    };
    let a516 = v0;
    let a517 = match v1.clone() {
        Adt2::C0(h515, _, _, _, _, _, _, _, _, _, _) => {
            h515
        }
    };
    if nat_lt(a516, a517) {
        let a518 = v6;
        let a519 = 0u64;
        if nat_eq(a518, a519) {
            Ok(Adt3::C3(0u64, 0u64, match v2.clone() {
                Adt1::C0(h520, _, _, _, _, _, _, _, _, _, _) => {
                    h520
                }
            }, 0u64, 0u64, {
                let a528 = {
                    let a524 = {
                        let a522 = v8;
                        let a523 = match v2 {
                            Adt1::C0(_, _, _, _, _, _, h521, _, _, _, _) => {
                                h521
                            }
                        };
                        nat_mul(a522, a523)?
                    };
                    let a525 = v10;
                    nat_add(a524, a525)?
                };
                let a529 = {
                    let a526 = v5;
                    let a527 = 100u64;
                    if nat_eq(a526, a527) {
                        v11
                    } else {
                        0u64
                    }
                };
                nat_add(a528, a529)?
            }, 2u64, 0u64, 0u64, 0u64, v9, 4u64))
        } else {
            let a530 = v6;
            let a531 = 1u64;
            if nat_eq(a530, a531) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h532, _, _, _, _, _, _, _, _, _, _) => {
                        h532
                    }
                }, 0u64, {
                    let a533 = v5;
                    let a534 = 98u64;
                    if nat_eq(a533, a534) {
                        4u64
                    } else {
                        0u64
                    }
                }, {
                    let a542 = {
                        let a538 = {
                            let a536 = v8;
                            let a537 = match v3 {
                                Adt1::C0(_, _, _, _, _, _, h535, _, _, _, _) => {
                                    h535
                                }
                            };
                            nat_mul(a536, a537)?
                        };
                        let a539 = v10;
                        nat_add(a538, a539)?
                    };
                    let a543 = {
                        let a540 = v5;
                        let a541 = 99u64;
                        if nat_eq(a540, a541) {
                            v11
                        } else {
                            0u64
                        }
                    };
                    nat_add(a542, a543)?
                }, 2u64, 0u64, 0u64, 1u64, v9, 4u64))
            } else {
                let a544 = v6;
                let a545 = 2u64;
                if nat_eq(a544, a545) {
                    Ok(Adt3::C4(match v1 {
                        Adt2::C0(_, _, _, _, h546, _, _, _, _, _, _) => {
                            h546
                        }
                    }, match v4 {
                        Adt1::C0(h547, _, _, _, _, _, _, _, _, _, _) => {
                            h547
                        }
                    }, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt3::C3(0u64, 0u64, 0u64, match v4.clone() {
                        Adt1::C0(h548, _, _, _, _, _, _, _, _, _, _) => {
                            h548
                        }
                    }, 1u64, 0u64, 2u64, 0u64, 3u64, {
                        let a552 = {
                            let a550 = v8;
                            let a551 = match v4 {
                                Adt1::C0(_, _, _, _, _, _, h549, _, _, _, _) => {
                                    h549
                                }
                            };
                            nat_mul(a550, a551)?
                        };
                        let a553 = v10;
                        nat_add(a552, a553)?
                    }, 4u64, v9))
                }
            }
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f17(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h554, _, _, _) => {
            h554
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h555, _, _) => {
            h555
        }
    };
    let v5: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h556, _) => {
            h556
        }
    };
    let v6: u64 = {
        let a558 = match v1.clone() {
            Adt2::C0(_, h557, _, _, _, _, _, _, _, _, _) => {
                h557
            }
        };
        let a559 = 3u64;
        if nat_eq(a558, a559) {
            0u64
        } else {
            let a560 = f33(v3.clone())?;
            let a561 = 2u64;
            nat_mul(a560, a561)?
        }
    };
    let a563 = match v1.clone() {
        Adt2::C0(h562, _, _, _, _, _, _, _, _, _, _) => {
            h562
        }
    };
    let a564 = v0;
    if nat_le(a563, a564) {
        Ok(Adt3::C1)
    } else {
        let a565 = v0;
        let a566 = v6;
        if nat_lt(a565, a566) {
            let v7: u64 = {
                let a567 = v0;
                let a568 = 2u64;
                let a569 = 0u64;
                nat_quot(a567, a568, a569)
            };
            let v8: u64 = {
                let a574 = 3u64;
                let a575 = {
                    let a571 = f44(v7, v5.clone())?;
                    let a572 = f28(match v5 {
                        Adt1::C0(h570, _, _, _, _, _, _, _, _, _, _) => {
                            h570
                        }
                    });
                    let a573 = 0u64;
                    nat_quot(a571, a572, a573)
                };
                nat_add(a574, a575)?
            };
            let a579 = {
                let a576 = v0;
                let a577 = 2u64;
                let a578 = 0u64;
                nat_rem(a576, a577, a578)
            };
            let a580 = 0u64;
            if nat_eq(a579, a580) {
                Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, v8, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
            } else {
                let a581 = v2;
                let a582 = 0u64;
                if nat_eq(a581, a582) {
                    Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 2u64, 1u64, 2u64, 0u64, 0u64, v8, 8u64, 8u64))
                } else {
                    Ok(Adt3::C1)
                }
            }
        } else {
            let v9: u64 = {
                let a583 = v0;
                let a584 = v6;
                nat_sub(a583, a584)
            };
            let v10: u64 = f34(match v3.clone() {
                Adt1::C0(h585, _, _, _, _, _, _, _, _, _, _) => {
                    h585
                }
            }, match v4.clone() {
                Adt1::C0(h586, _, _, _, _, _, _, _, _, _, _) => {
                    h586
                }
            }, match v5.clone() {
                Adt1::C0(h587, _, _, _, _, _, _, _, _, _, _) => {
                    h587
                }
            })?;
            let v11: u64 = {
                let a588 = v9;
                let a589 = v10;
                let a590 = 0u64;
                nat_quot(a588, a589, a590)
            };
            let v12: u64 = {
                let a591 = v9;
                let a592 = v10;
                let a593 = 0u64;
                nat_rem(a591, a592, a593)
            };
            let v13: u64 = {
                let a595 = match v3.clone() {
                    Adt1::C0(h594, _, _, _, _, _, _, _, _, _, _) => {
                        h594
                    }
                };
                let a596 = 0u64;
                if nat_eq(a595, a596) {
                    0u64
                } else {
                    1u64
                }
            };
            let v14: u64 = {
                let a598 = match v4.clone() {
                    Adt1::C0(h597, _, _, _, _, _, _, _, _, _, _) => {
                        h597
                    }
                };
                let a599 = 0u64;
                if nat_eq(a598, a599) {
                    0u64
                } else {
                    1u64
                }
            };
            let a600 = v12;
            let a601 = v13;
            if nat_lt(a600, a601) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h602, _, _, _, _, _, _, _, _, _, _) => {
                        h602
                    }
                }, 0u64, 0u64, f44(v11, v3.clone())?, 2u64, 0u64, 0u64, 0u64, f28(match v3 {
                    Adt1::C0(h603, _, _, _, _, _, _, _, _, _, _) => {
                        h603
                    }
                }), 4u64))
            } else {
                let a606 = v12;
                let a607 = {
                    let a604 = v13;
                    let a605 = v14;
                    nat_add(a604, a605)?
                };
                if nat_lt(a606, a607) {
                    Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                        Adt1::C0(h608, _, _, _, _, _, _, _, _, _, _) => {
                            h608
                        }
                    }, 0u64, 4u64, f45(v11, v3, v4.clone())?, 2u64, 0u64, 0u64, 1u64, f28(match v4 {
                        Adt1::C0(h609, _, _, _, _, _, _, _, _, _, _) => {
                            h609
                        }
                    }), 4u64))
                } else {
                    let a612 = v12;
                    let a613 = {
                        let a610 = v13;
                        let a611 = v14;
                        nat_add(a610, a611)?
                    };
                    if nat_eq(a612, a613) {
                        Ok(Adt3::C3(1u64, match v1 {
                            Adt2::C0(_, _, _, h614, _, _, _, _, _, _, _) => {
                                h614
                            }
                        }, 0u64, 0u64, {
                            let a615 = v13;
                            let a616 = 0u64;
                            if nat_eq(a615, a616) {
                                0u64
                            } else {
                                1u64
                            }
                        }, {
                            let a617 = v13;
                            let a618 = 0u64;
                            if nat_eq(a617, a618) {
                                f44(v11, v3.clone())?
                            } else {
                                0u64
                            }
                        }, {
                            let a619 = v14;
                            let a620 = 0u64;
                            if nat_eq(a619, a620) {
                                4u64
                            } else {
                                1u64
                            }
                        }, {
                            let a621 = v14;
                            let a622 = 0u64;
                            if nat_eq(a621, a622) {
                                f45(v11, v3, v4)?
                            } else {
                                1u64
                            }
                        }, {
                            let a624 = match v5.clone() {
                                Adt1::C0(h623, _, _, _, _, _, _, _, _, _, _) => {
                                    h623
                                }
                            };
                            let a625 = 0u64;
                            if nat_eq(a624, a625) {
                                3u64
                            } else {
                                0u64
                            }
                        }, {
                            let a627 = match v5.clone() {
                                Adt1::C0(h626, _, _, _, _, _, _, _, _, _, _) => {
                                    h626
                                }
                            };
                            let a628 = 0u64;
                            if nat_eq(a627, a628) {
                                f44(v11, v5)?
                            } else {
                                0u64
                            }
                        }, 4u64, 4u64))
                    } else {
                        Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                            Adt1::C0(h629, _, _, _, _, _, _, _, _, _, _) => {
                                h629
                            }
                        }, 1u64, 0u64, 2u64, 0u64, 3u64, f44(v11, v5.clone())?, 4u64, f28(match v5 {
                            Adt1::C0(h630, _, _, _, _, _, _, _, _, _, _) => {
                                h630
                            }
                        })))
                    }
                }
            }
        }
    }
}

pub fn f18(v0: Adt1, v1: Adt1) -> bool {
    if f25(v1.clone()) {
        f25(v0)
    } else {
        let a636 = {
            let a633 = match v0.clone() {
                Adt1::C0(_, h631, _, _, _, _, _, _, _, _, _) => {
                    h631
                }
            };
            let a634 = match v1.clone() {
                Adt1::C0(_, h632, _, _, _, _, _, _, _, _, _) => {
                    h632
                }
            };
            let a635 = 1u64;
            nat_rem(a633, a634, a635)
        };
        let a637 = 0u64;
        if if if nat_eq(a636, a637) {
            let a643 = {
                let a640 = match v0.clone() {
                    Adt1::C0(_, _, h638, _, _, _, _, _, _, _, _) => {
                        h638
                    }
                };
                let a641 = match v1.clone() {
                    Adt1::C0(_, _, h639, _, _, _, _, _, _, _, _) => {
                        h639
                    }
                };
                let a642 = 1u64;
                nat_rem(a640, a641, a642)
            };
            let a644 = 0u64;
            nat_eq(a643, a644)
        } else {
            false
        } {
            let a650 = {
                let a647 = match v0.clone() {
                    Adt1::C0(_, _, _, h645, _, _, _, _, _, _, _) => {
                        h645
                    }
                };
                let a648 = match v1.clone() {
                    Adt1::C0(_, _, _, h646, _, _, _, _, _, _, _) => {
                        h646
                    }
                };
                let a649 = 1u64;
                nat_rem(a647, a648, a649)
            };
            let a651 = 0u64;
            nat_eq(a650, a651)
        } else {
            false
        } {
            let a657 = {
                let a654 = match v0 {
                    Adt1::C0(_, _, _, _, h652, _, _, _, _, _, _) => {
                        h652
                    }
                };
                let a655 = match v1 {
                    Adt1::C0(_, _, _, _, h653, _, _, _, _, _, _) => {
                        h653
                    }
                };
                let a656 = 1u64;
                nat_rem(a654, a655, a656)
            };
            let a658 = 0u64;
            nat_eq(a657, a658)
        } else {
            false
        }
    }
}

pub fn f19(v0: Adt1, v1: Adt1) -> bool {
    let a661 = match v0.clone() {
        Adt1::C0(_, h659, _, _, _, _, _, _, _, _, _) => {
            h659
        }
    };
    let a662 = match v1.clone() {
        Adt1::C0(_, h660, _, _, _, _, _, _, _, _, _) => {
            h660
        }
    };
    if if if nat_eq(a661, a662) {
        let a665 = match v0.clone() {
            Adt1::C0(_, _, h663, _, _, _, _, _, _, _, _) => {
                h663
            }
        };
        let a666 = match v1.clone() {
            Adt1::C0(_, _, h664, _, _, _, _, _, _, _, _) => {
                h664
            }
        };
        nat_eq(a665, a666)
    } else {
        false
    } {
        let a669 = match v0.clone() {
            Adt1::C0(_, _, _, h667, _, _, _, _, _, _, _) => {
                h667
            }
        };
        let a670 = match v1.clone() {
            Adt1::C0(_, _, _, h668, _, _, _, _, _, _, _) => {
                h668
            }
        };
        nat_eq(a669, a670)
    } else {
        false
    } {
        let a673 = match v0 {
            Adt1::C0(_, _, _, _, h671, _, _, _, _, _, _) => {
                h671
            }
        };
        let a674 = match v1 {
            Adt1::C0(_, _, _, _, h672, _, _, _, _, _, _) => {
                h672
            }
        };
        nat_eq(a673, a674)
    } else {
        false
    }
}

pub fn f20(v0: u64, v1: u64) -> bool {
    if f2(v0) {
        let a675 = v0;
        let a676 = 9u64;
        !if nat_eq(a675, a676) {
            true
        } else {
            let a677 = v0;
            let a678 = 15u64;
            nat_eq(a677, a678)
        }
    } else {
        let a679 = f39(v0);
        let a680 = 2u64;
        if nat_eq(a679, a680) {
            f2(v1)
        } else {
            false
        }
    }
}

pub fn f21(v0: u64) -> u64 {
    if f2(v0) {
        let m681 = f52(v0);
        match m681 {
            Adt4::C0 => {
                0u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, _, h682) => {
                        h682
                    }
                }
            }
        }
    } else {
        let a683 = v0;
        let a684 = 26u64;
        if nat_eq(a683, a684) {
            4u64
        } else {
            f28(v0)
        }
    }
}

pub fn f22(v0: u64) -> u64 {
    if f2(v0) {
        let m685 = f52(v0);
        match m685 {
            Adt4::C0 => {
                1u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, h686, _) => {
                        h686
                    }
                }
            }
        }
    } else {
        1u64
    }
}

pub fn f23(v0: Adt1, v1: Adt1) -> R<bool> {
    if f41(v0.clone(), v1.clone())? {
        Ok(true)
    } else {
        Ok(f53(v0, v1))
    }
}

pub fn f24(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f54(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f54(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f54(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f54(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f54(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f54(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f25(v0: Adt1) -> bool {
    f55(match v0.clone() {
        Adt1::C0(_, h687, _, _, _, _, _, _, _, _, _) => {
            h687
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h688, _, _, _, _, _, _, _, _) => {
            h688
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h689, _, _, _, _, _, _, _) => {
            h689
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, h690, _, _, _, _, _, _) => {
            h690
        }
    })
}

pub fn f26(v0: u64) -> Option<u64> {
    let a691 = v0;
    let a692 = 16u64;
    if if if nat_eq(a691, a692) {
        true
    } else {
        let a693 = v0;
        let a694 = 17u64;
        nat_eq(a693, a694)
    } {
        true
    } else {
        let a695 = v0;
        let a696 = 19u64;
        nat_eq(a695, a696)
    } {
        Some(1065353216u64)
    } else {
        None::<u64>
    }
}

pub fn f27(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<bool> {
    let a698 = match v1.clone() {
        Adt1::C0(h697, _, _, _, _, _, _, _, _, _, _) => {
            h697
        }
    };
    let a699 = 43u64;
    if nat_eq(a698, a699) {
        let a700 = v0;
        let a701 = 0u64;
        if if if if if if nat_eq(a700, a701) {
            let a703 = match v2.clone() {
                Adt1::C0(h702, _, _, _, _, _, _, _, _, _, _) => {
                    h702
                }
            };
            let a704 = 0u64;
            nat_eq(a703, a704)
        } else {
            false
        } {
            let a706 = match v3.clone() {
                Adt1::C0(h705, _, _, _, _, _, _, _, _, _, _) => {
                    h705
                }
            };
            let a707 = 0u64;
            if nat_eq(a706, a707) {
                true
            } else {
                let a709 = match v3.clone() {
                    Adt1::C0(h708, _, _, _, _, _, _, _, _, _, _) => {
                        h708
                    }
                };
                let a710 = 43u64;
                nat_eq(a709, a710)
            }
        } else {
            false
        } {
            let a712 = match v3.clone() {
                Adt1::C0(h711, _, _, _, _, _, _, _, _, _, _) => {
                    h711
                }
            };
            let a713 = 43u64;
            if nat_eq(a712, a713) {
                f18(v1.clone(), v2.clone())
            } else {
                f19(v1.clone(), v2.clone())
            }
        } else {
            false
        } {
            let a715 = match v2 {
                Adt1::C0(_, _, _, _, _, h714, _, _, _, _, _) => {
                    h714
                }
            };
            let a716 = 4u64;
            nat_eq(a715, a716)
        } else {
            false
        } {
            let a719 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h717, _, _, _, _, _) => {
                    h717
                }
            };
            let a720 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h718, _, _, _, _) => {
                    h718
                }
            };
            if nat_le(a719, a720) {
                let a723 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, h721, _, _, _, _) => {
                        h721
                    }
                };
                let a724 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h722, _, _, _) => {
                        h722
                    }
                };
                if nat_le(a723, a724) {
                    let a727 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, h725, _, _, _) => {
                            h725
                        }
                    };
                    let a728 = match v3 {
                        Adt1::C0(_, _, _, _, _, _, _, _, h726, _, _) => {
                            h726
                        }
                    };
                    nat_le(a727, a728)
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        } {
            let a736 = {
                let a734 = match v1.clone() {
                    Adt1::C0(_, _, h729, _, _, _, _, _, _, _, _) => {
                        h729
                    }
                };
                let a735 = {
                    let a732 = match v1.clone() {
                        Adt1::C0(_, _, _, h730, _, _, _, _, _, _, _) => {
                            h730
                        }
                    };
                    let a733 = match v1 {
                        Adt1::C0(_, _, _, _, h731, _, _, _, _, _, _) => {
                            h731
                        }
                    };
                    nat_mul(a732, a733)?
                };
                nat_mul(a734, a735)?
            };
            let a737 = 2147483647u64;
            Ok(nat_le(a736, a737))
        } else {
            Ok(false)
        }
    } else {
        if f18(v1.clone(), v2.clone()) {
            Ok(f56(match v1 {
                Adt1::C0(h738, _, _, _, _, _, _, _, _, _, _) => {
                    h738
                }
            }, match v2 {
                Adt1::C0(h739, _, _, _, _, _, _, _, _, _, _) => {
                    h739
                }
            }, match v3 {
                Adt1::C0(h740, _, _, _, _, _, _, _, _, _, _) => {
                    h740
                }
            }))
        } else {
            Ok(false)
        }
    }
}

pub fn f28(v0: u64) -> u64 {
    let a741 = v0;
    let a742 = 43u64;
    if nat_eq(a741, a742) {
        1u64
    } else {
        f39(v0)
    }
}

pub fn f29(v0: Adt1) -> bool {
    let v1: u64 = f28(match v0.clone() {
        Adt1::C0(h743, _, _, _, _, _, _, _, _, _, _) => {
            h743
        }
    });
    let a748 = {
        let a745 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h744) => {
                h744
            }
        };
        let a746 = v1;
        let a747 = 1u64;
        nat_rem(a745, a746, a747)
    };
    let a749 = 0u64;
    if if nat_eq(a748, a749) {
        f57(match v0.clone() {
            Adt1::C0(_, h750, _, _, _, _, _, _, _, _, _) => {
                h750
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h751, _, _, _, _, _) => {
                h751
            }
        }, v1)
    } else {
        false
    } {
        f58(match v0.clone() {
            Adt1::C0(_, _, h752, _, _, _, _, _, _, _, _) => {
                h752
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h753, _, _, _, _, _, _, _) => {
                h753
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h754, _, _, _, _, _, _) => {
                h754
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h755, _, _, _, _) => {
                h755
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h756, _, _, _) => {
                h756
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h757, _, _) => {
                h757
            }
        }, v1)
    } else {
        false
    }
}

pub fn f30(v0: Adt1, v1: Adt1) -> R<bool> {
    let a765 = {
        let a763 = match v0.clone() {
            Adt1::C0(_, _, h758, _, _, _, _, _, _, _, _) => {
                h758
            }
        };
        let a764 = {
            let a761 = match v0.clone() {
                Adt1::C0(_, _, _, h759, _, _, _, _, _, _, _) => {
                    h759
                }
            };
            let a762 = match v0.clone() {
                Adt1::C0(_, _, _, _, h760, _, _, _, _, _, _) => {
                    h760
                }
            };
            nat_mul(a761, a762)?
        };
        nat_mul(a763, a764)?
    };
    let a766 = 9223372036854775807u64;
    if nat_le(a765, a766) {
        if f37(v1.clone()) {
            let a768 = match v1 {
                Adt1::C0(_, h767, _, _, _, _, _, _, _, _, _) => {
                    h767
                }
            };
            let a769 = 2147483647u64;
            Ok(nat_le(a768, a769))
        } else {
            let a771 = match v0 {
                Adt1::C0(_, h770, _, _, _, _, _, _, _, _, _) => {
                    h770
                }
            };
            let a772 = 2147483647u64;
            Ok(nat_le(a771, a772))
        }
    } else {
        Ok(false)
    }
}

pub fn f31(v0: Adt1) -> R<bool> {
    f24({
        let a775 = match v0.clone() {
            Adt1::C0(_, h773, _, _, _, _, _, _, _, _, _) => {
                h773
            }
        };
        let a776 = f28(match v0.clone() {
            Adt1::C0(h774, _, _, _, _, _, _, _, _, _, _) => {
                h774
            }
        });
        nat_mul(a775, a776)?
    }, match v0.clone() {
        Adt1::C0(_, _, h777, _, _, _, _, _, _, _, _) => {
            h777
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h778, _, _, _, _, _, _, _) => {
            h778
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h779, _, _, _, _, _, _) => {
            h779
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h780, _, _, _, _) => {
            h780
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h781, _, _, _) => {
            h781
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h782, _, _) => {
            h782
        }
    })
}

pub fn f32(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a789 = {
        let a783 = v0;
        let a784 = v2;
        nat_mul(a783, a784)?
    };
    let a790 = {
        let a785 = v1;
        let a786 = 0u64;
        if nat_eq(a785, a786) {
            0u64
        } else {
            let a787 = v0;
            let a788 = 2u64;
            nat_mul(a787, a788)?
        }
    };
    nat_add(a789, a790)
}

pub fn f33(v0: Adt1) -> R<u64> {
    let a799 = {
        let a793 = match v0.clone() {
            Adt1::C0(_, h791, _, _, _, _, _, _, _, _, _) => {
                h791
            }
        };
        let a794 = match v0.clone() {
            Adt1::C0(_, _, h792, _, _, _, _, _, _, _, _) => {
                h792
            }
        };
        nat_mul(a793, a794)?
    };
    let a800 = {
        let a797 = match v0.clone() {
            Adt1::C0(_, _, _, h795, _, _, _, _, _, _, _) => {
                h795
            }
        };
        let a798 = match v0 {
            Adt1::C0(_, _, _, _, h796, _, _, _, _, _, _) => {
                h796
            }
        };
        nat_mul(a797, a798)?
    };
    nat_mul(a799, a800)
}

pub fn f34(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a811 = {
        let a807 = {
            let a803 = 1u64;
            let a804 = {
                let a801 = v0;
                let a802 = 0u64;
                if nat_eq(a801, a802) {
                    0u64
                } else {
                    1u64
                }
            };
            nat_add(a803, a804)?
        };
        let a808 = {
            let a805 = v1;
            let a806 = 0u64;
            if nat_eq(a805, a806) {
                0u64
            } else {
                1u64
            }
        };
        nat_add(a807, a808)?
    };
    let a812 = {
        let a809 = v2;
        let a810 = 0u64;
        if nat_eq(a809, a810) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a811, a812)
}

pub fn f35(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f19(v0.clone(), v1.clone()) {
        let a815 = match v0.clone() {
            Adt1::C0(h813, _, _, _, _, _, _, _, _, _, _) => {
                h813
            }
        };
        let a816 = match v1.clone() {
            Adt1::C0(h814, _, _, _, _, _, _, _, _, _, _) => {
                h814
            }
        };
        nat_eq(a815, a816)
    } else {
        false
    } {
        let a819 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h817, _, _, _, _, _) => {
                h817
            }
        };
        let a820 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h818, _, _, _, _, _) => {
                h818
            }
        };
        nat_eq(a819, a820)
    } else {
        false
    } {
        let a823 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h821, _, _, _, _) => {
                h821
            }
        };
        let a824 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h822, _, _, _, _) => {
                h822
            }
        };
        nat_eq(a823, a824)
    } else {
        false
    } {
        let a827 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h825, _, _, _) => {
                h825
            }
        };
        let a828 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h826, _, _, _) => {
                h826
            }
        };
        nat_eq(a827, a828)
    } else {
        false
    } {
        let a831 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h829, _, _) => {
                h829
            }
        };
        let a832 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h830, _, _) => {
                h830
            }
        };
        nat_eq(a831, a832)
    } else {
        false
    } {
        let a834 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h833) => {
                h833
            }
        };
        let a835 = 0u64;
        if if nat_eq(a834, a835) {
            true
        } else {
            let a837 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h836) => {
                    h836
                }
            };
            let a838 = 0u64;
            nat_eq(a837, a838)
        } {
            true
        } else {
            let a841 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h839) => {
                    h839
                }
            };
            let a842 = match v1 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h840) => {
                    h840
                }
            };
            nat_eq(a841, a842)
        }
    } else {
        false
    }
}

pub fn f36(v0: u64) -> u64 {
    let a843 = v0;
    let a844 = 26u64;
    if nat_eq(a843, a844) {
        4u64
    } else {
        let a845 = v0;
        let a846 = 27u64;
        if nat_eq(a845, a846) {
            8u64
        } else {
            0u64
        }
    }
}

pub fn f37(v0: Adt1) -> bool {
    let a848 = match v0.clone() {
        Adt1::C0(_, h847, _, _, _, _, _, _, _, _, _) => {
            h847
        }
    };
    let a849 = 1u64;
    if nat_eq(a848, a849) {
        true
    } else {
        let a852 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h850, _, _, _, _, _) => {
                h850
            }
        };
        let a853 = f28(match v0 {
            Adt1::C0(h851, _, _, _, _, _, _, _, _, _, _) => {
                h851
            }
        });
        nat_eq(a852, a853)
    }
}

pub fn f38(v0: Adt1, v1: u64) -> R<u64> {
    let a859 = {
        let a855 = match v0.clone() {
            Adt1::C0(_, h854, _, _, _, _, _, _, _, _, _) => {
                h854
            }
        };
        let a856 = f22(v1);
        let a857 = 0u64;
        nat_quot(a855, a856, a857)
    };
    let a860 = f59(match v0 {
        Adt1::C0(h858, _, _, _, _, _, _, _, _, _, _) => {
            h858
        }
    }, v1);
    nat_mul(a859, a860)
}

pub fn f39(v0: u64) -> u64 {
    let a861 = v0;
    let a862 = 0u64;
    if nat_eq(a861, a862) {
        4u64
    } else {
        let a863 = v0;
        let a864 = 1u64;
        if if nat_eq(a863, a864) {
            true
        } else {
            let a865 = v0;
            let a866 = 30u64;
            nat_eq(a865, a866)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f40(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a868 = match v0.clone() {
            Adt1::C0(_, h867, _, _, _, _, _, _, _, _, _) => {
                h867
            }
        };
        let a869 = 4u64;
        nat_mul(a868, a869)?
    };
    let v2: u64 = {
        let a872 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h870, _, _, _, _) => {
                h870
            }
        };
        let a873 = match v0.clone() {
            Adt1::C0(_, _, h871, _, _, _, _, _, _, _, _) => {
                h871
            }
        };
        nat_mul(a872, a873)?
    };
    let v3: u64 = {
        let a875 = v2;
        let a876 = match v0.clone() {
            Adt1::C0(_, _, _, h874, _, _, _, _, _, _, _) => {
                h874
            }
        };
        nat_mul(a875, a876)?
    };
    let a878 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h877, _, _, _, _, _) => {
            h877
        }
    };
    let a879 = 4u64;
    if if if if nat_eq(a878, a879) {
        let a884 = {
            let a881 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h880) => {
                    h880
                }
            };
            let a882 = 4u64;
            let a883 = 1u64;
            nat_rem(a881, a882, a883)
        };
        let a885 = 0u64;
        nat_eq(a884, a885)
    } else {
        false
    } {
        let a893 = {
            let a891 = match v0.clone() {
                Adt1::C0(_, _, h886, _, _, _, _, _, _, _, _) => {
                    h886
                }
            };
            let a892 = {
                let a889 = match v0.clone() {
                    Adt1::C0(_, _, _, h887, _, _, _, _, _, _, _) => {
                        h887
                    }
                };
                let a890 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h888, _, _, _, _, _, _) => {
                        h888
                    }
                };
                nat_mul(a889, a890)?
            };
            nat_mul(a891, a892)?
        };
        let a894 = 1u64;
        if nat_le(a893, a894) {
            true
        } else {
            let a896 = v1;
            let a897 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h895, _, _, _, _) => {
                    h895
                }
            };
            if nat_le(a896, a897) {
                let a902 = {
                    let a899 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, h898, _, _, _, _) => {
                            h898
                        }
                    };
                    let a900 = 4u64;
                    let a901 = 1u64;
                    nat_rem(a899, a900, a901)
                };
                let a903 = 0u64;
                nat_eq(a902, a903)
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a905 = match v0.clone() {
            Adt1::C0(_, _, _, h904, _, _, _, _, _, _, _) => {
                h904
            }
        };
        let a906 = 1u64;
        if nat_le(a905, a906) {
            true
        } else {
            let a908 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h907, _, _, _) => {
                    h907
                }
            };
            let a909 = v2;
            nat_eq(a908, a909)
        }
    } else {
        false
    } {
        let a911 = match v0.clone() {
            Adt1::C0(_, _, _, _, h910, _, _, _, _, _, _) => {
                h910
            }
        };
        let a912 = 1u64;
        if nat_le(a911, a912) {
            Ok(true)
        } else {
            let a914 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, h913, _, _) => {
                    h913
                }
            };
            let a915 = v3;
            Ok(nat_eq(a914, a915))
        }
    } else {
        Ok(false)
    }
}

pub fn f41(v0: Adt1, v1: Adt1) -> R<bool> {
    let a917 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h916, _) => {
            h916
        }
    };
    let a918 = 0u64;
    if nat_eq(a917, a918) {
        Ok(true)
    } else {
        let a920 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h919, _) => {
                h919
            }
        };
        let a921 = 0u64;
        if nat_eq(a920, a921) {
            Ok(true)
        } else {
            let a923 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h922) => {
                    h922
                }
            };
            let a924 = 0u64;
            if nat_eq(a923, a924) {
                Ok(true)
            } else {
                let a926 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h925) => {
                        h925
                    }
                };
                let a927 = 0u64;
                if nat_eq(a926, a927) {
                    Ok(true)
                } else {
                    let a933 = {
                        let a930 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h928) => {
                                h928
                            }
                        };
                        let a931 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h929, _) => {
                                h929
                            }
                        };
                        nat_add(a930, a931)?
                    };
                    let a934 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h932) => {
                            h932
                        }
                    };
                    if nat_le(a933, a934) {
                        Ok(true)
                    } else {
                        let a940 = {
                            let a937 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h935) => {
                                    h935
                                }
                            };
                            let a938 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h936, _) => {
                                    h936
                                }
                            };
                            nat_add(a937, a938)?
                        };
                        let a941 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h939) => {
                                h939
                            }
                        };
                        Ok(nat_le(a940, a941))
                    }
                }
            }
        }
    }
}

pub fn f42(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a944 = match v0.clone() {
            Adt1::C0(_, _, h942, _, _, _, _, _, _, _, _) => {
                h942
            }
        };
        let a945 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h943, _, _, _, _) => {
                h943
            }
        };
        nat_mul(a944, a945)?
    };
    if if f37(v0.clone()) {
        let a947 = match v0.clone() {
            Adt1::C0(_, _, _, h946, _, _, _, _, _, _, _) => {
                h946
            }
        };
        let a948 = 1u64;
        if nat_eq(a947, a948) {
            true
        } else {
            let a950 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h949, _, _, _) => {
                    h949
                }
            };
            let a951 = v1;
            nat_eq(a950, a951)
        }
    } else {
        false
    } {
        let a953 = match v0.clone() {
            Adt1::C0(_, _, _, _, h952, _, _, _, _, _, _) => {
                h952
            }
        };
        let a954 = 1u64;
        if nat_eq(a953, a954) {
            Ok(true)
        } else {
            let a959 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h955, _, _) => {
                    h955
                }
            };
            let a960 = {
                let a957 = v1;
                let a958 = match v0 {
                    Adt1::C0(_, _, _, h956, _, _, _, _, _, _, _) => {
                        h956
                    }
                };
                nat_mul(a957, a958)?
            };
            Ok(nat_eq(a959, a960))
        }
    } else {
        Ok(false)
    }
}

pub fn f43(v0: u64) -> u64 {
    let a961 = v0;
    let a962 = 3u64;
    if nat_eq(a961, a962) {
        16u64
    } else {
        10u64
    }
}

pub fn f44(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a964 = v0;
        let a965 = match v1.clone() {
            Adt1::C0(_, h963, _, _, _, _, _, _, _, _, _) => {
                h963
            }
        };
        let a966 = 0u64;
        nat_quot(a964, a965, a966)
    };
    let v3: u64 = {
        let a968 = v2;
        let a969 = match v1.clone() {
            Adt1::C0(_, _, h967, _, _, _, _, _, _, _, _) => {
                h967
            }
        };
        let a970 = 0u64;
        nat_quot(a968, a969, a970)
    };
    f46({
        let a972 = v0;
        let a973 = match v1.clone() {
            Adt1::C0(_, h971, _, _, _, _, _, _, _, _, _) => {
                h971
            }
        };
        let a974 = 0u64;
        nat_rem(a972, a973, a974)
    }, {
        let a976 = v2;
        let a977 = match v1.clone() {
            Adt1::C0(_, _, h975, _, _, _, _, _, _, _, _) => {
                h975
            }
        };
        let a978 = 0u64;
        nat_rem(a976, a977, a978)
    }, {
        let a980 = v3;
        let a981 = match v1.clone() {
            Adt1::C0(_, _, _, h979, _, _, _, _, _, _, _) => {
                h979
            }
        };
        let a982 = 0u64;
        nat_rem(a980, a981, a982)
    }, {
        let a984 = v3;
        let a985 = match v1.clone() {
            Adt1::C0(_, _, _, h983, _, _, _, _, _, _, _) => {
                h983
            }
        };
        let a986 = 0u64;
        nat_quot(a984, a985, a986)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h987, _, _, _, _, _) => {
            h987
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h988, _, _, _, _) => {
            h988
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h989, _, _, _) => {
            h989
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h990, _, _) => {
            h990
        }
    })
}

pub fn f45(v0: u64, v1: Adt1, v2: Adt1) -> R<u64> {
    let v3: u64 = {
        let a992 = v0;
        let a993 = match v1.clone() {
            Adt1::C0(_, h991, _, _, _, _, _, _, _, _, _) => {
                h991
            }
        };
        let a994 = 0u64;
        nat_quot(a992, a993, a994)
    };
    let v4: u64 = {
        let a996 = v3;
        let a997 = match v1.clone() {
            Adt1::C0(_, _, h995, _, _, _, _, _, _, _, _) => {
                h995
            }
        };
        let a998 = 0u64;
        nat_quot(a996, a997, a998)
    };
    f46({
        let a1004 = {
            let a1000 = v0;
            let a1001 = match v1.clone() {
                Adt1::C0(_, h999, _, _, _, _, _, _, _, _, _) => {
                    h999
                }
            };
            let a1002 = 0u64;
            nat_rem(a1000, a1001, a1002)
        };
        let a1005 = match v2.clone() {
            Adt1::C0(_, h1003, _, _, _, _, _, _, _, _, _) => {
                h1003
            }
        };
        let a1006 = 0u64;
        nat_rem(a1004, a1005, a1006)
    }, {
        let a1012 = {
            let a1008 = v3;
            let a1009 = match v1.clone() {
                Adt1::C0(_, _, h1007, _, _, _, _, _, _, _, _) => {
                    h1007
                }
            };
            let a1010 = 0u64;
            nat_rem(a1008, a1009, a1010)
        };
        let a1013 = match v2.clone() {
            Adt1::C0(_, _, h1011, _, _, _, _, _, _, _, _) => {
                h1011
            }
        };
        let a1014 = 0u64;
        nat_rem(a1012, a1013, a1014)
    }, {
        let a1020 = {
            let a1016 = v4;
            let a1017 = match v1.clone() {
                Adt1::C0(_, _, _, h1015, _, _, _, _, _, _, _) => {
                    h1015
                }
            };
            let a1018 = 0u64;
            nat_rem(a1016, a1017, a1018)
        };
        let a1021 = match v2.clone() {
            Adt1::C0(_, _, _, h1019, _, _, _, _, _, _, _) => {
                h1019
            }
        };
        let a1022 = 0u64;
        nat_rem(a1020, a1021, a1022)
    }, {
        let a1028 = {
            let a1024 = v4;
            let a1025 = match v1 {
                Adt1::C0(_, _, _, h1023, _, _, _, _, _, _, _) => {
                    h1023
                }
            };
            let a1026 = 0u64;
            nat_quot(a1024, a1025, a1026)
        };
        let a1029 = match v2.clone() {
            Adt1::C0(_, _, _, _, h1027, _, _, _, _, _, _) => {
                h1027
            }
        };
        let a1030 = 0u64;
        nat_rem(a1028, a1029, a1030)
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h1031, _, _, _, _, _) => {
            h1031
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h1032, _, _, _, _) => {
            h1032
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1033, _, _, _) => {
            h1033
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1034, _, _) => {
            h1034
        }
    })
}

pub fn f46(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a1047 = {
        let a1043 = {
            let a1039 = {
                let a1035 = v0;
                let a1036 = v4;
                nat_mul(a1035, a1036)?
            };
            let a1040 = {
                let a1037 = v1;
                let a1038 = v5;
                nat_mul(a1037, a1038)?
            };
            nat_add(a1039, a1040)?
        };
        let a1044 = {
            let a1041 = v2;
            let a1042 = v6;
            nat_mul(a1041, a1042)?
        };
        nat_add(a1043, a1044)?
    };
    let a1048 = {
        let a1045 = v3;
        let a1046 = v7;
        nat_mul(a1045, a1046)?
    };
    nat_add(a1047, a1048)
}

pub fn f47(v0: u64, v1: Adt1, v2: u64, v3: u64) -> R<Adt3> {
    let v4: u64 = {
        let a1049 = v0;
        let a1050 = 2u64;
        let a1051 = 0u64;
        nat_quot(a1049, a1050, a1051)
    };
    let v5: u64 = f36(match v1.clone() {
        Adt1::C0(h1052, _, _, _, _, _, _, _, _, _, _) => {
            h1052
        }
    });
    let a1056 = {
        let a1053 = v0;
        let a1054 = 2u64;
        let a1055 = 0u64;
        nat_rem(a1053, a1054, a1055)
    };
    let a1057 = 0u64;
    if nat_eq(a1056, a1057) {
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 4u64, f44(v4, v1)?, 2u64, 0u64, 0u64, 2u64, v5, 8u64))
    } else {
        let a1058 = v3;
        let a1059 = v2;
        if if nat_lt(a1058, a1059) {
            let a1062 = v3;
            let a1063 = {
                let a1060 = v5;
                let a1061 = 4u64;
                if nat_eq(a1060, a1061) {
                    2147483648u64
                } else {
                    9223372036854775808u64
                }
            };
            nat_lt(a1062, a1063)
        } else {
            false
        } {
            Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, {
                let a1064 = 3u64;
                let a1065 = v4;
                nat_add(a1064, a1065)?
            }, 8u64, 8u64))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f48(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64) -> R<Adt3> {
    let v5: u64 = {
        let a1067 = 1u64;
        let a1068 = f38(v1.clone(), match v3.clone() {
            Adt1::C0(h1066, _, _, _, _, _, _, _, _, _, _) => {
                h1066
            }
        })?;
        nat_add(a1067, a1068)?
    };
    let v6: u64 = {
        let a1069 = v0;
        let a1070 = v5;
        let a1071 = 0u64;
        nat_rem(a1069, a1070, a1071)
    };
    let v7: u64 = {
        let a1072 = v0;
        let a1073 = v5;
        let a1074 = 0u64;
        nat_quot(a1072, a1073, a1074)
    };
    let v8: u64 = {
        let a1076 = v7;
        let a1077 = match v1.clone() {
            Adt1::C0(_, _, h1075, _, _, _, _, _, _, _, _) => {
                h1075
            }
        };
        let a1078 = 0u64;
        nat_rem(a1076, a1077, a1078)
    };
    let v9: u64 = {
        let a1080 = v7;
        let a1081 = match v1.clone() {
            Adt1::C0(_, _, h1079, _, _, _, _, _, _, _, _) => {
                h1079
            }
        };
        let a1082 = 0u64;
        nat_quot(a1080, a1081, a1082)
    };
    let v10: u64 = {
        let a1084 = v9;
        let a1085 = match v1.clone() {
            Adt1::C0(_, _, _, h1083, _, _, _, _, _, _, _) => {
                h1083
            }
        };
        let a1086 = 0u64;
        nat_rem(a1084, a1085, a1086)
    };
    let v11: u64 = {
        let a1088 = v9;
        let a1089 = match v1.clone() {
            Adt1::C0(_, _, _, h1087, _, _, _, _, _, _, _) => {
                h1087
            }
        };
        let a1090 = 0u64;
        nat_quot(a1088, a1089, a1090)
    };
    let a1091 = v6;
    let a1092 = 0u64;
    if nat_eq(a1091, a1092) {
        let v12: u64 = {
            let a1109 = v8;
            let a1110 = {
                let a1107 = match v2.clone() {
                    Adt1::C0(_, h1093, _, _, _, _, _, _, _, _, _) => {
                        h1093
                    }
                };
                let a1108 = {
                    let a1105 = {
                        let a1095 = v10;
                        let a1096 = match v2.clone() {
                            Adt1::C0(_, _, h1094, _, _, _, _, _, _, _, _) => {
                                h1094
                            }
                        };
                        let a1097 = 0u64;
                        nat_rem(a1095, a1096, a1097)
                    };
                    let a1106 = {
                        let a1103 = match v2.clone() {
                            Adt1::C0(_, _, h1098, _, _, _, _, _, _, _, _) => {
                                h1098
                            }
                        };
                        let a1104 = {
                            let a1100 = v11;
                            let a1101 = match v2 {
                                Adt1::C0(_, _, _, h1099, _, _, _, _, _, _, _) => {
                                    h1099
                                }
                            };
                            let a1102 = 0u64;
                            nat_rem(a1100, a1101, a1102)
                        };
                        nat_mul(a1103, a1104)?
                    };
                    nat_add(a1105, a1106)?
                };
                nat_mul(a1107, a1108)?
            };
            nat_add(a1109, a1110)?
        };
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, {
            let a1111 = 3u64;
            let a1112 = v12;
            nat_add(a1111, a1112)?
        }, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
    } else {
        let a1114 = v4;
        let a1115 = match v3.clone() {
            Adt1::C0(_, _, h1113, _, _, _, _, _, _, _, _) => {
                h1113
            }
        };
        if nat_lt(a1114, a1115) {
            let v13: u64 = {
                let a1120 = {
                    let a1116 = v6;
                    let a1117 = 1u64;
                    nat_sub(a1116, a1117)
                };
                let a1121 = f59(match v1.clone() {
                    Adt1::C0(h1118, _, _, _, _, _, _, _, _, _, _) => {
                        h1118
                    }
                }, match v3.clone() {
                    Adt1::C0(h1119, _, _, _, _, _, _, _, _, _, _) => {
                        h1119
                    }
                });
                let a1122 = 0u64;
                nat_quot(a1120, a1121, a1122)
            };
            let v14: u64 = f46({
                let a1124 = v13;
                let a1125 = f22(match v3.clone() {
                    Adt1::C0(h1123, _, _, _, _, _, _, _, _, _, _) => {
                        h1123
                    }
                });
                nat_mul(a1124, a1125)?
            }, v8, v10, v11, match v1.clone() {
                Adt1::C0(_, _, _, _, _, h1126, _, _, _, _, _) => {
                    h1126
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, h1127, _, _, _, _) => {
                    h1127
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1128, _, _, _) => {
                    h1128
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1129, _, _) => {
                    h1129
                }
            })?;
            let v15: u64 = f46(v13, v4, v10, v11, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h1130, _, _, _, _, _) => {
                    h1130
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h1131, _, _, _, _) => {
                    h1131
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1132, _, _, _) => {
                    h1132
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1133, _, _) => {
                    h1133
                }
            })?;
            if if f2(match v3.clone() {
                Adt1::C0(h1134, _, _, _, _, _, _, _, _, _, _) => {
                    h1134
                }
            }) {
                true
            } else {
                let a1136 = match v3.clone() {
                    Adt1::C0(h1135, _, _, _, _, _, _, _, _, _, _) => {
                        h1135
                    }
                };
                let a1137 = 26u64;
                nat_eq(a1136, a1137)
            } {
                Ok(Adt3::C6(v14, v15))
            } else {
                Ok(f60(match v1 {
                    Adt1::C0(h1138, _, _, _, _, _, _, _, _, _, _) => {
                        h1138
                    }
                }, match v3 {
                    Adt1::C0(h1139, _, _, _, _, _, _, _, _, _, _) => {
                        h1139
                    }
                }, {
                    let a1140 = v6;
                    let a1141 = 1u64;
                    nat_sub(a1140, a1141)
                }, v14, v15))
            }
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f49(v0: u64, v1: u64, v2: u64) -> Adt3 {
    Adt3::C3(2u64, 0u64, 0u64, 0u64, 1u64, v0, v1, v2, 0u64, 2u64, 4u64, 4u64)
}

pub fn f50(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt3 {
    Adt3::C3(2u64, 3u64, 0u64, 0u64, if v3 {
        v1
    } else {
        1u64
    }, if v3 {
        v2
    } else {
        v0
    }, 2u64, 0u64, 0u64, v0, 4u64, 4u64)
}

pub fn f51(v0: u64, v1: u64) -> Adt3 {
    let a1142 = v0;
    let a1143 = 8u64;
    if nat_eq(a1142, a1143) {
        Adt3::C3(1u64, 6u64, 0u64, 0u64, 1u64, 0u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
    } else {
        let a1144 = v0;
        let a1145 = 9u64;
        if nat_eq(a1144, a1145) {
            Adt3::C3(1u64, 2u64, 0u64, 0u64, 2u64, v1, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
        } else {
            let a1146 = v0;
            let a1147 = 10u64;
            if nat_eq(a1146, a1147) {
                Adt3::C3(5u64, 16u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
            } else {
                let a1148 = v0;
                let a1149 = 11u64;
                if nat_eq(a1148, a1149) {
                    Adt3::C3(1u64, 0u64, 0u64, 0u64, 2u64, 1065353216u64, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
                } else {
                    let a1150 = v0;
                    let a1151 = 12u64;
                    if nat_eq(a1150, a1151) {
                        Adt3::C3(1u64, 3u64, 0u64, 0u64, 1u64, 0u64, 1u64, 2u64, 0u64, 0u64, 4u64, 4u64)
                    } else {
                        let a1152 = v0;
                        let a1153 = 13u64;
                        if nat_eq(a1152, a1153) {
                            Adt3::C3(1u64, 0u64, 0u64, 0u64, 1u64, 1u64, 2u64, 1065353216u64, 0u64, 1u64, 4u64, 4u64)
                        } else {
                            let a1154 = v0;
                            let a1155 = 14u64;
                            if nat_eq(a1154, a1155) {
                                Adt3::C3(1u64, 2u64, 0u64, 0u64, 1u64, 0u64, 1u64, 1u64, 0u64, 0u64, 4u64, 4u64)
                            } else {
                                Adt3::C1
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f52(v0: u64) -> Adt4 {
    let a1156 = v0;
    let a1157 = 0u64;
    if nat_eq(a1156, a1157) {
        Adt4::C1(Adt5::C0(0u64, 1u64, 4u64))
    } else {
        let a1158 = v0;
        let a1159 = 1u64;
        if nat_eq(a1158, a1159) {
            Adt4::C1(Adt5::C0(1u64, 1u64, 2u64))
        } else {
            let a1160 = v0;
            let a1161 = 2u64;
            if nat_eq(a1160, a1161) {
                Adt4::C1(Adt5::C0(2u64, 32u64, 18u64))
            } else {
                let a1162 = v0;
                let a1163 = 3u64;
                if nat_eq(a1162, a1163) {
                    Adt4::C1(Adt5::C0(3u64, 32u64, 20u64))
                } else {
                    let a1164 = v0;
                    let a1165 = 6u64;
                    if nat_eq(a1164, a1165) {
                        Adt4::C1(Adt5::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1166 = v0;
                        let a1167 = 7u64;
                        if nat_eq(a1166, a1167) {
                            Adt4::C1(Adt5::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1168 = v0;
                            let a1169 = 8u64;
                            if nat_eq(a1168, a1169) {
                                Adt4::C1(Adt5::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1170 = v0;
                                let a1171 = 9u64;
                                if nat_eq(a1170, a1171) {
                                    Adt4::C1(Adt5::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1172 = v0;
                                    let a1173 = 10u64;
                                    if nat_eq(a1172, a1173) {
                                        Adt4::C1(Adt5::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1174 = v0;
                                        let a1175 = 11u64;
                                        if nat_eq(a1174, a1175) {
                                            Adt4::C1(Adt5::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1176 = v0;
                                            let a1177 = 12u64;
                                            if nat_eq(a1176, a1177) {
                                                Adt4::C1(Adt5::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1178 = v0;
                                                let a1179 = 13u64;
                                                if nat_eq(a1178, a1179) {
                                                    Adt4::C1(Adt5::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1180 = v0;
                                                    let a1181 = 14u64;
                                                    if nat_eq(a1180, a1181) {
                                                        Adt4::C1(Adt5::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1182 = v0;
                                                        let a1183 = 15u64;
                                                        if nat_eq(a1182, a1183) {
                                                            Adt4::C1(Adt5::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1184 = v0;
                                                            let a1185 = 16u64;
                                                            if nat_eq(a1184, a1185) {
                                                                Adt4::C1(Adt5::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1186 = v0;
                                                                let a1187 = 17u64;
                                                                if nat_eq(a1186, a1187) {
                                                                    Adt4::C1(Adt5::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1188 = v0;
                                                                    let a1189 = 18u64;
                                                                    if nat_eq(a1188, a1189) {
                                                                        Adt4::C1(Adt5::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1190 = v0;
                                                                        let a1191 = 19u64;
                                                                        if nat_eq(a1190, a1191) {
                                                                            Adt4::C1(Adt5::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1192 = v0;
                                                                            let a1193 = 20u64;
                                                                            if nat_eq(a1192, a1193) {
                                                                                Adt4::C1(Adt5::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1194 = v0;
                                                                                let a1195 = 21u64;
                                                                                if nat_eq(a1194, a1195) {
                                                                                    Adt4::C1(Adt5::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1196 = v0;
                                                                                    let a1197 = 22u64;
                                                                                    if nat_eq(a1196, a1197) {
                                                                                        Adt4::C1(Adt5::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1198 = v0;
                                                                                        let a1199 = 23u64;
                                                                                        if nat_eq(a1198, a1199) {
                                                                                            Adt4::C1(Adt5::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1200 = v0;
                                                                                            let a1201 = 24u64;
                                                                                            if nat_eq(a1200, a1201) {
                                                                                                Adt4::C1(Adt5::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1202 = v0;
                                                                                                let a1203 = 25u64;
                                                                                                if nat_eq(a1202, a1203) {
                                                                                                    Adt4::C1(Adt5::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1204 = v0;
                                                                                                    let a1205 = 26u64;
                                                                                                    if nat_eq(a1204, a1205) {
                                                                                                        Adt4::C1(Adt5::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1206 = v0;
                                                                                                        let a1207 = 27u64;
                                                                                                        if nat_eq(a1206, a1207) {
                                                                                                            Adt4::C1(Adt5::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1208 = v0;
                                                                                                            let a1209 = 28u64;
                                                                                                            if nat_eq(a1208, a1209) {
                                                                                                                Adt4::C1(Adt5::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1210 = v0;
                                                                                                                let a1211 = 29u64;
                                                                                                                if nat_eq(a1210, a1211) {
                                                                                                                    Adt4::C1(Adt5::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1212 = v0;
                                                                                                                    let a1213 = 30u64;
                                                                                                                    if nat_eq(a1212, a1213) {
                                                                                                                        Adt4::C1(Adt5::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1214 = v0;
                                                                                                                        let a1215 = 34u64;
                                                                                                                        if nat_eq(a1214, a1215) {
                                                                                                                            Adt4::C1(Adt5::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1216 = v0;
                                                                                                                            let a1217 = 35u64;
                                                                                                                            if nat_eq(a1216, a1217) {
                                                                                                                                Adt4::C1(Adt5::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1218 = v0;
                                                                                                                                let a1219 = 39u64;
                                                                                                                                if nat_eq(a1218, a1219) {
                                                                                                                                    Adt4::C1(Adt5::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1220 = v0;
                                                                                                                                    let a1221 = 40u64;
                                                                                                                                    if nat_eq(a1220, a1221) {
                                                                                                                                        Adt4::C1(Adt5::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1222 = v0;
                                                                                                                                        let a1223 = 41u64;
                                                                                                                                        if nat_eq(a1222, a1223) {
                                                                                                                                            Adt4::C1(Adt5::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1224 = v0;
                                                                                                                                            let a1225 = 42u64;
                                                                                                                                            if nat_eq(a1224, a1225) {
                                                                                                                                                Adt4::C1(Adt5::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1226 = v0;
                                                                                                                                                let a1227 = 43u64;
                                                                                                                                                if nat_eq(a1226, a1227) {
                                                                                                                                                    Adt4::C1(Adt5::C0(43u64, 1u64, 1u64))
                                                                                                                                                } else {
                                                                                                                                                    Adt4::C0
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

pub fn f53(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f19(v0.clone(), v1.clone()) {
        let a1230 = match v0.clone() {
            Adt1::C0(h1228, _, _, _, _, _, _, _, _, _, _) => {
                h1228
            }
        };
        let a1231 = match v1.clone() {
            Adt1::C0(h1229, _, _, _, _, _, _, _, _, _, _) => {
                h1229
            }
        };
        nat_eq(a1230, a1231)
    } else {
        false
    } {
        let a1234 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1232) => {
                h1232
            }
        };
        let a1235 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1233) => {
                h1233
            }
        };
        nat_eq(a1234, a1235)
    } else {
        false
    } {
        let a1238 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1236, _, _, _, _, _) => {
                h1236
            }
        };
        let a1239 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1237, _, _, _, _, _) => {
                h1237
            }
        };
        nat_eq(a1238, a1239)
    } else {
        false
    } {
        let a1242 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1240, _, _, _, _) => {
                h1240
            }
        };
        let a1243 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1241, _, _, _, _) => {
                h1241
            }
        };
        nat_eq(a1242, a1243)
    } else {
        false
    } {
        let a1246 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1244, _, _, _) => {
                h1244
            }
        };
        let a1247 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1245, _, _, _) => {
                h1245
            }
        };
        nat_eq(a1246, a1247)
    } else {
        false
    } {
        let a1250 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1248, _, _) => {
                h1248
            }
        };
        let a1251 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1249, _, _) => {
                h1249
            }
        };
        nat_eq(a1250, a1251)
    } else {
        false
    }
}

pub fn f54(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a1252 = v1;
    let a1253 = 1u64;
    if if if nat_le(a1252, a1253) {
        true
    } else {
        let a1254 = v0;
        let a1255 = v2;
        nat_le(a1254, a1255)
    } {
        let a1256 = v3;
        let a1257 = 1u64;
        if nat_le(a1256, a1257) {
            true
        } else {
            let a1264 = {
                let a1262 = v0;
                let a1263 = {
                    let a1260 = {
                        let a1258 = v1;
                        let a1259 = 1u64;
                        nat_sub(a1258, a1259)
                    };
                    let a1261 = v2;
                    nat_mul(a1260, a1261)?
                };
                nat_add(a1262, a1263)?
            };
            let a1265 = v4;
            nat_le(a1264, a1265)
        }
    } else {
        false
    } {
        let a1266 = v5;
        let a1267 = 1u64;
        if nat_le(a1266, a1267) {
            Ok(true)
        } else {
            let a1280 = {
                let a1278 = {
                    let a1272 = v0;
                    let a1273 = {
                        let a1270 = {
                            let a1268 = v1;
                            let a1269 = 1u64;
                            nat_sub(a1268, a1269)
                        };
                        let a1271 = v2;
                        nat_mul(a1270, a1271)?
                    };
                    nat_add(a1272, a1273)?
                };
                let a1279 = {
                    let a1276 = {
                        let a1274 = v3;
                        let a1275 = 1u64;
                        nat_sub(a1274, a1275)
                    };
                    let a1277 = v4;
                    nat_mul(a1276, a1277)?
                };
                nat_add(a1278, a1279)?
            };
            let a1281 = v6;
            Ok(nat_le(a1280, a1281))
        }
    } else {
        Ok(false)
    }
}

pub fn f55(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1282 = v0;
    let a1283 = 0u64;
    if if if nat_eq(a1282, a1283) {
        true
    } else {
        let a1284 = v1;
        let a1285 = 0u64;
        nat_eq(a1284, a1285)
    } {
        true
    } else {
        let a1286 = v2;
        let a1287 = 0u64;
        nat_eq(a1286, a1287)
    } {
        true
    } else {
        let a1288 = v3;
        let a1289 = 0u64;
        nat_eq(a1288, a1289)
    }
}

pub fn f56(v0: u64, v1: u64, v2: u64) -> bool {
    let a1290 = v0;
    let a1291 = 0u64;
    if if if nat_eq(a1290, a1291) {
        let a1292 = v1;
        let a1293 = 0u64;
        nat_eq(a1292, a1293)
    } else {
        false
    } {
        let a1294 = v2;
        let a1295 = 0u64;
        nat_eq(a1294, a1295)
    } else {
        false
    } {
        true
    } else {
        let a1296 = v0;
        let a1297 = 1u64;
        if if nat_eq(a1296, a1297) {
            true
        } else {
            let a1298 = v0;
            let a1299 = 30u64;
            nat_eq(a1298, a1299)
        } {
            let a1300 = v1;
            let a1301 = v0;
            if if nat_eq(a1300, a1301) {
                let a1302 = v2;
                let a1303 = v0;
                nat_eq(a1302, a1303)
            } else {
                false
            } {
                true
            } else {
                let a1304 = v1;
                let a1305 = 0u64;
                if nat_eq(a1304, a1305) {
                    let a1306 = v2;
                    let a1307 = v0;
                    if nat_eq(a1306, a1307) {
                        true
                    } else {
                        let a1308 = v2;
                        let a1309 = 0u64;
                        nat_eq(a1308, a1309)
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

pub fn f57(v0: u64, v1: u64, v2: u64) -> bool {
    let a1310 = v0;
    let a1311 = 1u64;
    if nat_le(a1310, a1311) {
        true
    } else {
        let a1315 = {
            let a1312 = v1;
            let a1313 = v2;
            let a1314 = 1u64;
            nat_rem(a1312, a1313, a1314)
        };
        let a1316 = 0u64;
        nat_eq(a1315, a1316)
    }
}

pub fn f58(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f57(v0, v3, v6) {
        f57(v1, v4, v6)
    } else {
        false
    } {
        f57(v2, v5, v6)
    } else {
        false
    }
}

pub fn f59(v0: u64, v1: u64) -> u64 {
    let a1317 = v0;
    let a1318 = 1u64;
    if if nat_eq(a1317, a1318) {
        let a1319 = v1;
        let a1320 = 43u64;
        nat_eq(a1319, a1320)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f60(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt3 {
    let a1321 = f59(v0, v1);
    let a1322 = 2u64;
    if nat_eq(a1321, a1322) {
        let a1326 = {
            let a1323 = v2;
            let a1324 = 2u64;
            let a1325 = 0u64;
            nat_rem(a1323, a1324, a1325)
        };
        let a1327 = 0u64;
        if nat_eq(a1326, a1327) {
            Adt3::C3(0u64, 0u64, v0, 0u64, 0u64, v3, 2u64, 0u64, 1u64, 0u64, f39(v0), 8u64)
        } else {
            Adt3::C3(0u64, 0u64, 0u64, v1, 1u64, 0u64, 2u64, 0u64, 3u64, v4, 8u64, f21(v1))
        }
    } else {
        Adt3::C3(0u64, 0u64, v0, v1, 0u64, v3, 2u64, 0u64, 3u64, v4, f39(v0), f21(v1))
    }
}
