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
                                        f2(match v6.clone() {
                                            Adt1::C0(h7, _, _, _, _, _, _, _, _, _, _) => {
                                                h7
                                            }
                                        })
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
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h21) => {
                    h21
                }
            } {
                f10(v26, v27)
            } else {
                let a23 = match v27.clone() {
                    Adt2::C0(_, _, _, h22, _, _, _, _, _, _, _) => {
                        h22
                    }
                };
                let a24 = 33u64;
                if nat_eq(a23, a24) {
                    f11(v26, v27)
                } else {
                    let a26 = match v27.clone() {
                        Adt2::C0(_, _, _, h25, _, _, _, _, _, _, _) => {
                            h25
                        }
                    };
                    let a27 = 66u64;
                    if if nat_eq(a26, a27) {
                        true
                    } else {
                        let a29 = match v27.clone() {
                            Adt2::C0(_, _, _, h28, _, _, _, _, _, _, _) => {
                                h28
                            }
                        };
                        let a30 = 67u64;
                        nat_eq(a29, a30)
                    } {
                        f12(v26, v27)
                    } else {
                        let a32 = match v27.clone() {
                            Adt2::C0(_, _, _, h31, _, _, _, _, _, _, _) => {
                                h31
                            }
                        };
                        let a33 = 41u64;
                        if nat_eq(a32, a33) {
                            f13(v26, v27, v28)
                        } else {
                            let a35 = 98u64;
                            let a36 = match v27.clone() {
                                Adt2::C0(_, _, _, h34, _, _, _, _, _, _, _) => {
                                    h34
                                }
                            };
                            if nat_le(a35, a36) {
                                if f14(match v27.clone() {
                                    Adt2::C0(_, _, _, _, h37, _, _, _, _, _, _) => {
                                        h37
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
            let a8 = v8;
            let a9 = 41u64;
            if nat_eq(a8, a9) {
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
            let a10 = v14;
            let a11 = 33u64;
            if nat_eq(a10, a11) {
                if v19 {
                    Ok(Adt3::C1)
                } else {
                    let m12 = v21;
                    match m12 {
                        None => {
                            Ok(Adt3::C1)
                        }
                        Some(v23) => {
                            f6(v20, v23, v22)
                        }
                    }
                }
            } else {
                let a13 = v14;
                let a14 = 66u64;
                if if nat_eq(a13, a14) {
                    true
                } else {
                    let a15 = v14;
                    let a16 = 67u64;
                    nat_eq(a15, a16)
                } {
                    if v19 {
                        Ok(Adt3::C1)
                    } else {
                        let m17 = v21;
                        match m17 {
                            None => {
                                f7(v14, v15, v20, v22)
                            }
                            Some(_) => {
                                Ok(Adt3::C1)
                            }
                        }
                    }
                } else {
                    let a18 = v14;
                    let a19 = 98u64;
                    if nat_eq(a18, a19) {
                        if if f8(v15) {
                            !v19
                        } else {
                            false
                        } {
                            let m20 = v21;
                            match m20 {
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
    let a38 = v0;
    let a39 = 2u64;
    if nat_eq(a38, a39) {
        Some(0u64)
    } else {
        let a40 = v0;
        let a41 = 5u64;
        if nat_eq(a40, a41) {
            Some(1u64)
        } else {
            let a42 = v0;
            let a43 = 6u64;
            if nat_eq(a42, a43) {
                Some(2u64)
            } else {
                let a44 = v0;
                let a45 = 7u64;
                if nat_eq(a44, a45) {
                    Some(3u64)
                } else {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f2(v0: u64) -> bool {
    let a46 = 6u64;
    let a47 = v0;
    if if if if nat_le(a46, a47) {
        let a48 = v0;
        let a49 = 23u64;
        nat_le(a48, a49)
    } else {
        false
    } {
        true
    } else {
        let a50 = v0;
        let a51 = 2u64;
        if nat_eq(a50, a51) {
            true
        } else {
            let a52 = v0;
            let a53 = 3u64;
            nat_eq(a52, a53)
        }
    } {
        true
    } else {
        let a54 = v0;
        let a55 = 29u64;
        nat_eq(a54, a55)
    } {
        true
    } else {
        let a56 = 34u64;
        let a57 = v0;
        if if nat_le(a56, a57) {
            let a58 = v0;
            let a59 = 35u64;
            nat_le(a58, a59)
        } else {
            false
        } {
            true
        } else {
            let a60 = 39u64;
            let a61 = v0;
            if nat_le(a60, a61) {
                let a62 = v0;
                let a63 = 42u64;
                nat_le(a62, a63)
            } else {
                false
            }
        }
    }
}

pub fn f3(v0: Adt1, v1: Adt1, v2: Adt1) -> R<Adt3> {
    let a66 = match v0.clone() {
        Adt1::C0(_, h64, _, _, _, _, _, _, _, _, _) => {
            h64
        }
    };
    let a67 = match v1.clone() {
        Adt1::C0(_, h65, _, _, _, _, _, _, _, _, _) => {
            h65
        }
    };
    if if if if if if if if if if if if if if if if nat_eq(a66, a67) {
        true
    } else {
        let a70 = match v0.clone() {
            Adt1::C0(h68, _, _, _, _, _, _, _, _, _, _) => {
                h68
            }
        };
        let a71 = match v2.clone() {
            Adt1::C0(h69, _, _, _, _, _, _, _, _, _, _) => {
                h69
            }
        };
        nat_eq(a70, a71)
    } {
        f18(v0.clone(), v1.clone())
    } else {
        false
    } {
        f19(v0.clone(), v2.clone())
    } else {
        false
    } {
        let a73 = match v1.clone() {
            Adt1::C0(h72, _, _, _, _, _, _, _, _, _, _) => {
                h72
            }
        };
        let a74 = 0u64;
        nat_eq(a73, a74)
    } else {
        false
    } {
        let a76 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h75, _, _, _, _, _) => {
                h75
            }
        };
        let a77 = 4u64;
        nat_eq(a76, a77)
    } else {
        false
    } {
        f2(match v0.clone() {
            Adt1::C0(h78, _, _, _, _, _, _, _, _, _, _) => {
                h78
            }
        })
    } else {
        false
    } {
        let a80 = match v0.clone() {
            Adt1::C0(h79, _, _, _, _, _, _, _, _, _, _) => {
                h79
            }
        };
        let a81 = 9u64;
        !if nat_eq(a80, a81) {
            true
        } else {
            let a83 = match v0.clone() {
                Adt1::C0(h82, _, _, _, _, _, _, _, _, _, _) => {
                    h82
                }
            };
            let a84 = 15u64;
            nat_eq(a83, a84)
        }
    } else {
        false
    } {
        let a86 = 0u64;
        let a87 = f20(match v2.clone() {
            Adt1::C0(h85, _, _, _, _, _, _, _, _, _, _) => {
                h85
            }
        });
        nat_lt(a86, a87)
    } else {
        false
    } {
        let a90 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h88, _, _, _, _, _) => {
                h88
            }
        };
        let a91 = f20(match v0.clone() {
            Adt1::C0(h89, _, _, _, _, _, _, _, _, _, _) => {
                h89
            }
        });
        nat_eq(a90, a91)
    } else {
        false
    } {
        let a94 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h92, _, _, _, _, _) => {
                h92
            }
        };
        let a95 = f20(match v2.clone() {
            Adt1::C0(h93, _, _, _, _, _, _, _, _, _, _) => {
                h93
            }
        });
        nat_eq(a94, a95)
    } else {
        false
    } {
        let a101 = {
            let a98 = match v0.clone() {
                Adt1::C0(_, h96, _, _, _, _, _, _, _, _, _) => {
                    h96
                }
            };
            let a99 = f21(match v0.clone() {
                Adt1::C0(h97, _, _, _, _, _, _, _, _, _, _) => {
                    h97
                }
            });
            let a100 = 1u64;
            nat_rem(a98, a99, a100)
        };
        let a102 = 0u64;
        nat_eq(a101, a102)
    } else {
        false
    } {
        let a108 = {
            let a105 = match v2.clone() {
                Adt1::C0(_, h103, _, _, _, _, _, _, _, _, _) => {
                    h103
                }
            };
            let a106 = f21(match v2.clone() {
                Adt1::C0(h104, _, _, _, _, _, _, _, _, _, _) => {
                    h104
                }
            });
            let a107 = 1u64;
            nat_rem(a105, a106, a107)
        };
        let a109 = 0u64;
        nat_eq(a108, a109)
    } else {
        false
    } {
        f22(v0.clone(), v2.clone())?
    } else {
        false
    } {
        f22(v1.clone(), v2.clone())?
    } else {
        false
    } {
        f23({
            let a116 = {
                let a112 = match v2.clone() {
                    Adt1::C0(_, h110, _, _, _, _, _, _, _, _, _) => {
                        h110
                    }
                };
                let a113 = f21(match v2.clone() {
                    Adt1::C0(h111, _, _, _, _, _, _, _, _, _, _) => {
                        h111
                    }
                });
                let a114 = 0u64;
                nat_quot(a112, a113, a114)
            };
            let a117 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, h115, _, _, _, _, _) => {
                    h115
                }
            };
            nat_mul(a116, a117)?
        }, match v2.clone() {
            Adt1::C0(_, _, h118, _, _, _, _, _, _, _, _) => {
                h118
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h119, _, _, _, _, _, _, _) => {
                h119
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h120, _, _, _, _, _, _) => {
                h120
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h121, _, _, _, _) => {
                h121
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h122, _, _, _) => {
                h122
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h123, _, _) => {
                h123
            }
        })?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0(if f24(v0.clone()) {
            0u64
        } else {
            let a129 = match v0.clone() {
                Adt1::C0(_, _, h124, _, _, _, _, _, _, _, _) => {
                    h124
                }
            };
            let a130 = {
                let a127 = match v0.clone() {
                    Adt1::C0(_, _, _, h125, _, _, _, _, _, _, _) => {
                        h125
                    }
                };
                let a128 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h126, _, _, _, _, _, _) => {
                        h126
                    }
                };
                nat_mul(a127, a128)?
            };
            nat_mul(a129, a130)?
        }, 3u64, f25(match v2.clone() {
            Adt1::C0(h131, _, _, _, _, _, _, _, _, _, _) => {
                h131
            }
        }), 0u64, 0u64, 0u64, 0u64, v0, v1, v2, true)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f4(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if f19(v1.clone(), v3.clone()) {
        f26(v0, v1.clone(), v2.clone(), v3.clone())?
    } else {
        false
    } {
        if f24(v1.clone()) {
            true
        } else {
            let a134 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, h132, _, _, _, _, _) => {
                    h132
                }
            };
            let a135 = f27(match v1.clone() {
                Adt1::C0(h133, _, _, _, _, _, _, _, _, _, _) => {
                    h133
                }
            });
            if if if if if if if nat_eq(a134, a135) {
                let a138 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h136, _, _, _, _, _) => {
                        h136
                    }
                };
                let a139 = f27(match v3.clone() {
                    Adt1::C0(h137, _, _, _, _, _, _, _, _, _, _) => {
                        h137
                    }
                });
                nat_eq(a138, a139)
            } else {
                false
            } {
                f28(v1.clone())
            } else {
                false
            } {
                f28(v2.clone())
            } else {
                false
            } {
                f28(v3.clone())
            } else {
                false
            } {
                f29(v1.clone(), v2.clone())?
            } else {
                false
            } {
                f22(v1.clone(), v3.clone())?
            } else {
                false
            } {
                f22(v2.clone(), v3.clone())?
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v4: u64 = if if f24(v1.clone()) {
            true
        } else {
            f30(v3.clone())?
        } {
            0u64
        } else {
            let a142 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h140, _) => {
                    h140
                }
            };
            let a143 = f27(match v3.clone() {
                Adt1::C0(h141, _, _, _, _, _, _, _, _, _, _) => {
                    h141
                }
            });
            let a144 = 0u64;
            nat_quot(a142, a143, a144)
        };
        Ok(Adt3::C2(Adt2::C0(f31(f32(v1.clone())?, v4, f33(match v1.clone() {
            Adt1::C0(h145, _, _, _, _, _, _, _, _, _, _) => {
                h145
            }
        }, match v2.clone() {
            Adt1::C0(h146, _, _, _, _, _, _, _, _, _, _) => {
                h146
            }
        }, match v3.clone() {
            Adt1::C0(h147, _, _, _, _, _, _, _, _, _, _) => {
                h147
            }
        })?)?, {
            let a148 = 3u64;
            let a149 = v4;
            nat_add(a148, a149)?
        }, None::<u64>, v0, 0u64, 0u64, 0u64, v1, v2, v3, false)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f5(v0: Adt1, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if if if if if if if if if if if if if f34(v2, v3.clone()) {
        let a151 = match v0.clone() {
            Adt1::C0(h150, _, _, _, _, _, _, _, _, _, _) => {
                h150
            }
        };
        let a152 = 0u64;
        if nat_eq(a151, a152) {
            true
        } else {
            let a154 = match v0.clone() {
                Adt1::C0(h153, _, _, _, _, _, _, _, _, _, _) => {
                    h153
                }
            };
            let a155 = 1u64;
            nat_eq(a154, a155)
        }
    } else {
        false
    } {
        let a157 = 0u64;
        let a158 = f20(match v3.clone() {
            Adt1::C0(h156, _, _, _, _, _, _, _, _, _, _) => {
                h156
            }
        });
        nat_lt(a157, a158)
    } else {
        false
    } {
        let a160 = 0u64;
        let a161 = f35(match v1.clone() {
            Adt1::C0(h159, _, _, _, _, _, _, _, _, _, _) => {
                h159
            }
        });
        nat_lt(a160, a161)
    } else {
        false
    } {
        let a164 = match v0.clone() {
            Adt1::C0(_, h162, _, _, _, _, _, _, _, _, _) => {
                h162
            }
        };
        let a165 = match v3.clone() {
            Adt1::C0(_, h163, _, _, _, _, _, _, _, _, _) => {
                h163
            }
        };
        if nat_eq(a164, a165) {
            let a171 = {
                let a168 = match v0.clone() {
                    Adt1::C0(_, h166, _, _, _, _, _, _, _, _, _) => {
                        h166
                    }
                };
                let a169 = f21(match v3.clone() {
                    Adt1::C0(h167, _, _, _, _, _, _, _, _, _, _) => {
                        h167
                    }
                });
                let a170 = 1u64;
                nat_rem(a168, a169, a170)
            };
            let a172 = 0u64;
            nat_eq(a171, a172)
        } else {
            false
        }
    } else {
        false
    } {
        let a175 = match v0.clone() {
            Adt1::C0(_, _, _, h173, _, _, _, _, _, _, _) => {
                h173
            }
        };
        let a176 = match v3.clone() {
            Adt1::C0(_, _, _, h174, _, _, _, _, _, _, _) => {
                h174
            }
        };
        nat_eq(a175, a176)
    } else {
        false
    } {
        let a179 = match v0.clone() {
            Adt1::C0(_, _, _, _, h177, _, _, _, _, _, _) => {
                h177
            }
        };
        let a180 = match v3.clone() {
            Adt1::C0(_, _, _, _, h178, _, _, _, _, _, _) => {
                h178
            }
        };
        nat_eq(a179, a180)
    } else {
        false
    } {
        let a183 = match v0.clone() {
            Adt1::C0(_, _, h181, _, _, _, _, _, _, _, _) => {
                h181
            }
        };
        let a184 = match v1.clone() {
            Adt1::C0(_, h182, _, _, _, _, _, _, _, _, _) => {
                h182
            }
        };
        nat_eq(a183, a184)
    } else {
        false
    } {
        let a186 = match v1.clone() {
            Adt1::C0(_, _, _, _, h185, _, _, _, _, _, _) => {
                h185
            }
        };
        let a187 = 1u64;
        nat_eq(a186, a187)
    } else {
        false
    } {
        let a189 = 0u64;
        let a190 = match v1.clone() {
            Adt1::C0(_, _, h188, _, _, _, _, _, _, _, _) => {
                h188
            }
        };
        nat_lt(a189, a190)
    } else {
        false
    } {
        let a192 = 0u64;
        let a193 = match v1.clone() {
            Adt1::C0(_, _, _, h191, _, _, _, _, _, _, _) => {
                h191
            }
        };
        nat_lt(a192, a193)
    } else {
        false
    } {
        let a199 = {
            let a196 = match v0.clone() {
                Adt1::C0(_, _, _, h194, _, _, _, _, _, _, _) => {
                    h194
                }
            };
            let a197 = match v1.clone() {
                Adt1::C0(_, _, h195, _, _, _, _, _, _, _, _) => {
                    h195
                }
            };
            let a198 = 0u64;
            nat_rem(a196, a197, a198)
        };
        let a200 = 0u64;
        nat_eq(a199, a200)
    } else {
        false
    } {
        let a206 = {
            let a203 = match v0.clone() {
                Adt1::C0(_, _, _, _, h201, _, _, _, _, _, _) => {
                    h201
                }
            };
            let a204 = match v1.clone() {
                Adt1::C0(_, _, _, h202, _, _, _, _, _, _, _) => {
                    h202
                }
            };
            let a205 = 0u64;
            nat_rem(a203, a204, a205)
        };
        let a207 = 0u64;
        nat_eq(a206, a207)
    } else {
        false
    } {
        f36(v0.clone())
    } else {
        false
    } {
        let a209 = match v3.clone() {
            Adt1::C0(_, h208, _, _, _, _, _, _, _, _, _) => {
                h208
            }
        };
        let a210 = 1u64;
        if nat_eq(a209, a210) {
            true
        } else {
            let a213 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h211, _, _, _, _, _) => {
                    h211
                }
            };
            let a214 = f20(match v3.clone() {
                Adt1::C0(h212, _, _, _, _, _, _, _, _, _, _) => {
                    h212
                }
            });
            nat_eq(a213, a214)
        }
    } else {
        false
    } {
        let v4: u64 = f32(v1.clone())?;
        let v5: u64 = {
            let a220 = match v0.clone() {
                Adt1::C0(_, _, h215, _, _, _, _, _, _, _, _) => {
                    h215
                }
            };
            let a221 = {
                let a218 = match v0.clone() {
                    Adt1::C0(_, _, _, h216, _, _, _, _, _, _, _) => {
                        h216
                    }
                };
                let a219 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h217, _, _, _, _, _, _) => {
                        h217
                    }
                };
                nat_mul(a218, a219)?
            };
            nat_mul(a220, a221)?
        };
        let v6: u64 = if f24(v0.clone()) {
            0u64
        } else {
            let a229 = {
                let a222 = 2u64;
                let a223 = v4;
                nat_mul(a222, a223)?
            };
            let a230 = {
                let a227 = v5;
                let a228 = {
                    let a225 = 1u64;
                    let a226 = f37(v0.clone(), match v3.clone() {
                        Adt1::C0(h224, _, _, _, _, _, _, _, _, _, _) => {
                            h224
                        }
                    })?;
                    nat_add(a225, a226)?
                };
                nat_mul(a227, a228)?
            };
            nat_add(a229, a230)?
        };
        Ok(Adt3::C2(Adt2::C0(v6, if f24(v0.clone()) {
            3u64
        } else {
            let a231 = 3u64;
            let a232 = v4;
            nat_add(a231, a232)?
        }, f25(match v3.clone() {
            Adt1::C0(h233, _, _, _, _, _, _, _, _, _, _) => {
                h233
            }
        }), 41u64, 0u64, 0u64, 0u64, v0, v1, v3, false)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f6(v0: Adt1, v1: Adt1, v2: Adt1) -> R<Adt3> {
    let v3: u64 = f21(match v2.clone() {
        Adt1::C0(h234, _, _, _, _, _, _, _, _, _, _) => {
            h234
        }
    });
    if if if if if if if f2(match v2.clone() {
        Adt1::C0(h235, _, _, _, _, _, _, _, _, _, _) => {
            h235
        }
    }) {
        f34(v1.clone(), v2.clone())
    } else {
        false
    } {
        let a237 = 0u64;
        let a238 = f38(match v0.clone() {
            Adt1::C0(h236, _, _, _, _, _, _, _, _, _, _) => {
                h236
            }
        });
        nat_lt(a237, a238)
    } else {
        false
    } {
        let a241 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h239, _, _, _, _, _) => {
                h239
            }
        };
        let a242 = f38(match v0.clone() {
            Adt1::C0(h240, _, _, _, _, _, _, _, _, _, _) => {
                h240
            }
        });
        nat_eq(a241, a242)
    } else {
        false
    } {
        let a247 = {
            let a244 = match v0.clone() {
                Adt1::C0(_, h243, _, _, _, _, _, _, _, _, _) => {
                    h243
                }
            };
            let a245 = v3;
            let a246 = 1u64;
            nat_rem(a244, a245, a246)
        };
        let a248 = 0u64;
        if nat_eq(a247, a248) {
            let a253 = {
                let a250 = match v2.clone() {
                    Adt1::C0(_, h249, _, _, _, _, _, _, _, _, _) => {
                        h249
                    }
                };
                let a251 = v3;
                let a252 = 1u64;
                nat_rem(a250, a251, a252)
            };
            let a254 = 0u64;
            nat_eq(a253, a254)
        } else {
            false
        }
    } else {
        false
    } {
        let a257 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h255, _, _, _, _, _) => {
                h255
            }
        };
        let a258 = f20(match v2.clone() {
            Adt1::C0(h256, _, _, _, _, _, _, _, _, _, _) => {
                h256
            }
        });
        nat_eq(a257, a258)
    } else {
        false
    } {
        let a259 = f32(v0.clone())?;
        let a260 = f32(v2.clone())?;
        nat_eq(a259, a260)
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a261 = f32(v0.clone())?;
            let a262 = v3;
            let a263 = 0u64;
            nat_quot(a261, a262, a263)
        }, 3u64, f25(match v2.clone() {
            Adt1::C0(h264, _, _, _, _, _, _, _, _, _, _) => {
                h264
            }
        }), 33u64, 0u64, 0u64, 0u64, v0, v1, v2, false)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f7(v0: u64, v1: u64, v2: Adt1, v3: Adt1) -> R<Adt3> {
    let a266 = match v2.clone() {
        Adt1::C0(h265, _, _, _, _, _, _, _, _, _, _) => {
            h265
        }
    };
    let a267 = 0u64;
    if if if if if if if if if if if nat_eq(a266, a267) {
        let a269 = match v3.clone() {
            Adt1::C0(h268, _, _, _, _, _, _, _, _, _, _) => {
                h268
            }
        };
        let a270 = 26u64;
        nat_eq(a269, a270)
    } else {
        false
    } {
        let a272 = match v2.clone() {
            Adt1::C0(_, h271, _, _, _, _, _, _, _, _, _) => {
                h271
            }
        };
        let a273 = 2147483647u64;
        nat_le(a272, a273)
    } else {
        false
    } {
        let a276 = match v3.clone() {
            Adt1::C0(_, h274, _, _, _, _, _, _, _, _, _) => {
                h274
            }
        };
        let a277 = match v2.clone() {
            Adt1::C0(_, h275, _, _, _, _, _, _, _, _, _) => {
                h275
            }
        };
        nat_le(a276, a277)
    } else {
        false
    } {
        let a280 = match v2.clone() {
            Adt1::C0(_, _, h278, _, _, _, _, _, _, _, _) => {
                h278
            }
        };
        let a281 = match v3.clone() {
            Adt1::C0(_, _, h279, _, _, _, _, _, _, _, _) => {
                h279
            }
        };
        nat_eq(a280, a281)
    } else {
        false
    } {
        let a284 = match v2.clone() {
            Adt1::C0(_, _, _, h282, _, _, _, _, _, _, _) => {
                h282
            }
        };
        let a285 = match v3.clone() {
            Adt1::C0(_, _, _, h283, _, _, _, _, _, _, _) => {
                h283
            }
        };
        nat_eq(a284, a285)
    } else {
        false
    } {
        let a288 = match v2.clone() {
            Adt1::C0(_, _, _, _, h286, _, _, _, _, _, _) => {
                h286
            }
        };
        let a289 = match v3.clone() {
            Adt1::C0(_, _, _, _, h287, _, _, _, _, _, _) => {
                h287
            }
        };
        nat_eq(a288, a289)
    } else {
        false
    } {
        let a290 = v0;
        let a291 = 67u64;
        if nat_eq(a290, a291) {
            true
        } else {
            let a292 = v1;
            let a293 = 1u64;
            if nat_le(a292, a293) {
                let a296 = match v3.clone() {
                    Adt1::C0(_, h294, _, _, _, _, _, _, _, _, _) => {
                        h294
                    }
                };
                let a297 = match v2.clone() {
                    Adt1::C0(_, h295, _, _, _, _, _, _, _, _, _) => {
                        h295
                    }
                };
                nat_eq(a296, a297)
            } else {
                false
            }
        }
    } else {
        false
    } {
        f39(v2.clone())?
    } else {
        false
    } {
        f39(v3.clone())?
    } else {
        false
    } {
        f40(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a303 = match v3.clone() {
                Adt1::C0(_, _, h298, _, _, _, _, _, _, _, _) => {
                    h298
                }
            };
            let a304 = {
                let a301 = match v3.clone() {
                    Adt1::C0(_, _, _, h299, _, _, _, _, _, _, _) => {
                        h299
                    }
                };
                let a302 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h300, _, _, _, _, _, _) => {
                        h300
                    }
                };
                nat_mul(a301, a302)?
            };
            nat_mul(a303, a304)?
        }, 3u64, None::<u64>, v0, {
            let a305 = v0;
            let a306 = 67u64;
            if nat_eq(a305, a306) {
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
    let a307 = v0;
    let a308 = 6u64;
    nat_le(a307, a308)
}

pub fn f9(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1) -> R<Adt3> {
    let a309 = v2;
    let a310 = 1u64;
    if if if if if if if if if if if if if if if nat_le(a309, a310) {
        let a312 = 0u64;
        let a313 = f38(match v5.clone() {
            Adt1::C0(h311, _, _, _, _, _, _, _, _, _, _) => {
                h311
            }
        });
        nat_lt(a312, a313)
    } else {
        false
    } {
        let a316 = match v5.clone() {
            Adt1::C0(h314, _, _, _, _, _, _, _, _, _, _) => {
                h314
            }
        };
        let a317 = match v6.clone() {
            Adt1::C0(h315, _, _, _, _, _, _, _, _, _, _) => {
                h315
            }
        };
        nat_eq(a316, a317)
    } else {
        false
    } {
        let a320 = match v5.clone() {
            Adt1::C0(h318, _, _, _, _, _, _, _, _, _, _) => {
                h318
            }
        };
        let a321 = match v7.clone() {
            Adt1::C0(h319, _, _, _, _, _, _, _, _, _, _) => {
                h319
            }
        };
        nat_eq(a320, a321)
    } else {
        false
    } {
        f19(v5.clone(), v6.clone())
    } else {
        false
    } {
        f41(v5.clone())?
    } else {
        false
    } {
        f41(v6.clone())?
    } else {
        false
    } {
        f41(v7.clone())?
    } else {
        false
    } {
        let a324 = match v5.clone() {
            Adt1::C0(_, _, h322, _, _, _, _, _, _, _, _) => {
                h322
            }
        };
        let a325 = match v7.clone() {
            Adt1::C0(_, _, h323, _, _, _, _, _, _, _, _) => {
                h323
            }
        };
        nat_eq(a324, a325)
    } else {
        false
    } {
        let a328 = match v5.clone() {
            Adt1::C0(_, _, _, h326, _, _, _, _, _, _, _) => {
                h326
            }
        };
        let a329 = match v7.clone() {
            Adt1::C0(_, _, _, h327, _, _, _, _, _, _, _) => {
                h327
            }
        };
        nat_eq(a328, a329)
    } else {
        false
    } {
        let a332 = match v5.clone() {
            Adt1::C0(_, _, _, _, h330, _, _, _, _, _, _) => {
                h330
            }
        };
        let a333 = match v7.clone() {
            Adt1::C0(_, _, _, _, h331, _, _, _, _, _, _) => {
                h331
            }
        };
        nat_eq(a332, a333)
    } else {
        false
    } {
        let a340 = match v7.clone() {
            Adt1::C0(_, h334, _, _, _, _, _, _, _, _, _) => {
                h334
            }
        };
        let a341 = if v1 {
            match v5.clone() {
                Adt1::C0(_, h335, _, _, _, _, _, _, _, _, _) => {
                    h335
                }
            }
        } else {
            let a337 = match v5.clone() {
                Adt1::C0(_, h336, _, _, _, _, _, _, _, _, _) => {
                    h336
                }
            };
            let a338 = 2u64;
            let a339 = 0u64;
            nat_quot(a337, a338, a339)
        };
        nat_eq(a340, a341)
    } else {
        false
    } {
        let a343 = match v7.clone() {
            Adt1::C0(_, h342, _, _, _, _, _, _, _, _, _) => {
                h342
            }
        };
        let a344 = 2147483647u64;
        nat_le(a343, a344)
    } else {
        false
    } {
        f22(v5.clone(), v7.clone())?
    } else {
        false
    } {
        f22(v6.clone(), v7.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0(if f14(v0) {
            let a347 = 1u64;
            let a348 = {
                let a345 = f42(v0);
                let a346 = f32(v7.clone())?;
                nat_mul(a345, a346)?
            };
            nat_add(a347, a348)?
        } else {
            let a349 = 4u64;
            let a350 = f32(v7.clone())?;
            nat_mul(a349, a350)?
        }, if f14(v0) {
            4u64
        } else {
            3u64
        }, None::<u64>, if v1 {
            98u64
        } else {
            let a351 = 99u64;
            let a352 = v2;
            nat_add(a351, a352)?
        }, v0, v3, v4, v5, v6, v7, false)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f10(v0: u64, v1: Adt2) -> R<Adt3> {
    let a354 = v0;
    let a355 = match v1.clone() {
        Adt2::C0(h353, _, _, _, _, _, _, _, _, _, _) => {
            h353
        }
    };
    if nat_lt(a354, a355) {
        let v2: u64 = {
            let a359 = v0;
            let a360 = {
                let m358 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h357, _, _, _) => {
                        h357
                    }
                };
                match m358 {
                    Adt1::C0(_, h356, _, _, _, _, _, _, _, _, _) => {
                        h356
                    }
                }
            };
            nat_mul(a359, a360)?
        };
        Ok(Adt3::C7(f43(v2, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h361, _, _, _) => {
                h361
            }
        })?, f44(v2, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h362, _, _, _) => {
                h362
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h363, _, _) => {
                h363
            }
        })?, f43(v2, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h364, _) => {
                h364
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f11(v0: u64, v1: Adt2) -> R<Adt3> {
    let a366 = v0;
    let a367 = match v1.clone() {
        Adt2::C0(h365, _, _, _, _, _, _, _, _, _, _) => {
            h365
        }
    };
    if nat_lt(a366, a367) {
        let v2: Adt1 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h368, _) => {
                h368
            }
        };
        let v3: u64 = f21(match v2.clone() {
            Adt1::C0(h369, _, _, _, _, _, _, _, _, _, _) => {
                h369
            }
        });
        let v4: u64 = {
            let a371 = match v2.clone() {
                Adt1::C0(_, h370, _, _, _, _, _, _, _, _, _) => {
                    h370
                }
            };
            let a372 = v3;
            let a373 = 0u64;
            nat_quot(a371, a372, a373)
        };
        let v5: u64 = {
            let a374 = v0;
            let a375 = v4;
            let a376 = 0u64;
            nat_quot(a374, a375, a376)
        };
        let v6: u64 = {
            let a378 = v5;
            let a379 = match v2.clone() {
                Adt1::C0(_, _, h377, _, _, _, _, _, _, _, _) => {
                    h377
                }
            };
            let a380 = 0u64;
            nat_quot(a378, a379, a380)
        };
        Ok(Adt3::C6(f43({
            let a381 = v0;
            let a382 = v3;
            nat_mul(a381, a382)?
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, h383, _, _, _) => {
                h383
            }
        })?, f45({
            let a384 = v0;
            let a385 = v4;
            let a386 = 0u64;
            nat_rem(a384, a385, a386)
        }, {
            let a388 = v5;
            let a389 = match v2.clone() {
                Adt1::C0(_, _, h387, _, _, _, _, _, _, _, _) => {
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
            nat_rem(a392, a393, a394)
        }, {
            let a396 = v6;
            let a397 = match v2.clone() {
                Adt1::C0(_, _, _, h395, _, _, _, _, _, _, _) => {
                    h395
                }
            };
            let a398 = 0u64;
            nat_quot(a396, a397, a398)
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, h399, _, _, _, _, _) => {
                h399
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h400, _, _, _, _) => {
                h400
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h401, _, _, _) => {
                h401
            }
        }, match v2 {
            Adt1::C0(_, _, _, _, _, _, _, _, h402, _, _) => {
                h402
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f12(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h403, _, _, _) => {
            h403
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h404, _) => {
            h404
        }
    };
    let a406 = v0;
    let a407 = match v1.clone() {
        Adt2::C0(h405, _, _, _, _, _, _, _, _, _, _) => {
            h405
        }
    };
    if nat_lt(a406, a407) {
        Ok(Adt3::C5({
            let a409 = v0;
            let a410 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, _, h408, _, _, _, _) => {
                    h408
                }
            };
            nat_mul(a409, a410)?
        }, {
            let a412 = v0;
            let a413 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h411, _, _, _, _) => {
                    h411
                }
            };
            nat_mul(a412, a413)?
        }, match v2 {
            Adt1::C0(_, h414, _, _, _, _, _, _, _, _, _) => {
                h414
            }
        }, match v3 {
            Adt1::C0(_, h415, _, _, _, _, _, _, _, _, _) => {
                h415
            }
        }, {
            let a417 = match v1 {
                Adt2::C0(_, _, _, _, h416, _, _, _, _, _, _) => {
                    h416
                }
            };
            let a418 = 1u64;
            nat_eq(a417, a418)
        }))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f13(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let a420 = v0;
    let a421 = match v1.clone() {
        Adt2::C0(h419, _, _, _, _, _, _, _, _, _, _) => {
            h419
        }
    };
    if nat_lt(a420, a421) {
        let v3: u64 = {
            let a423 = 2u64;
            let a424 = f32(match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h422, _, _) => {
                    h422
                }
            })?;
            nat_mul(a423, a424)?
        };
        let a425 = v0;
        let a426 = v3;
        if nat_lt(a425, a426) {
            f46(v0, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h427, _, _) => {
                    h427
                }
            }, {
                let m430 = match v1 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h429, _) => {
                        h429
                    }
                };
                match m430 {
                    Adt1::C0(_, _, h428, _, _, _, _, _, _, _, _) => {
                        h428
                    }
                }
            }, v2)
        } else {
            f47({
                let a431 = v0;
                let a432 = v3;
                nat_sub(a431, a432)
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h433, _, _, _) => {
                    h433
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h434, _, _) => {
                    h434
                }
            }, match v1 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h435, _) => {
                    h435
                }
            }, v2)
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f14(v0: u64) -> bool {
    let a436 = v0;
    let a437 = 3u64;
    if nat_eq(a436, a437) {
        true
    } else {
        let a438 = v0;
        let a439 = 6u64;
        nat_eq(a438, a439)
    }
}

pub fn f15(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h440, _, _, _, _) => {
            h440
        }
    };
    let a442 = v0;
    let a443 = match v1.clone() {
        Adt2::C0(h441, _, _, _, _, _, _, _, _, _, _) => {
            h441
        }
    };
    if nat_lt(a442, a443) {
        let a444 = v0;
        let a445 = 0u64;
        if nat_eq(a444, a445) {
            Ok(Adt3::C3(1u64, 6u64, 0u64, 0u64, 2u64, v3, 2u64, 0u64, 0u64, 3u64, 4u64, 4u64))
        } else {
            let v4: u64 = {
                let a446 = v0;
                let a447 = 1u64;
                nat_sub(a446, a447)
            };
            let v5: u64 = {
                let a449 = v4;
                let a450 = f42(match v1.clone() {
                    Adt2::C0(_, _, _, _, h448, _, _, _, _, _, _) => {
                        h448
                    }
                });
                let a451 = 0u64;
                nat_rem(a449, a450, a451)
            };
            let v6: u64 = {
                let a456 = 4u64;
                let a457 = {
                    let a453 = v4;
                    let a454 = f42(match v1.clone() {
                        Adt2::C0(_, _, _, _, h452, _, _, _, _, _, _) => {
                            h452
                        }
                    });
                    let a455 = 0u64;
                    nat_quot(a453, a454, a455)
                };
                nat_mul(a456, a457)?
            };
            let a458 = v5;
            let a459 = 2u64;
            if nat_lt(a458, a459) {
                f16({
                    let a460 = v6;
                    let a461 = v5;
                    nat_add(a460, a461)?
                }, v1)
            } else {
                let a462 = v5;
                let a463 = 2u64;
                if nat_eq(a462, a463) {
                    Ok(f48(0u64, 2u64, v3))
                } else {
                    let a464 = v5;
                    let a465 = 3u64;
                    if nat_eq(a464, a465) {
                        Ok(f49(0u64, 2u64, v3, {
                            let a466 = v2;
                            let a467 = 2u64;
                            nat_eq(a466, a467)
                        }))
                    } else {
                        let a468 = v5;
                        let a469 = 4u64;
                        if nat_eq(a468, a469) {
                            Ok(f48(1u64, 1u64, 3u64))
                        } else {
                            let a470 = v5;
                            let a471 = 5u64;
                            if nat_eq(a470, a471) {
                                Ok(f49(1u64, 1u64, 3u64, {
                                    let a472 = v2;
                                    let a473 = 0u64;
                                    nat_eq(a472, a473)
                                }))
                            } else {
                                let a474 = v5;
                                let a475 = 6u64;
                                if nat_eq(a474, a475) {
                                    Ok(f48(1u64, 2u64, v3))
                                } else {
                                    let a476 = v5;
                                    let a477 = 7u64;
                                    if nat_eq(a476, a477) {
                                        Ok(f49(1u64, 2u64, v3, {
                                            let a478 = v2;
                                            let a479 = 2u64;
                                            nat_eq(a478, a479)
                                        }))
                                    } else {
                                        let a481 = match v1.clone() {
                                            Adt2::C0(_, _, _, _, h480, _, _, _, _, _, _) => {
                                                h480
                                            }
                                        };
                                        let a482 = 3u64;
                                        if nat_eq(a481, a482) {
                                            let a483 = v5;
                                            let a484 = 15u64;
                                            if nat_eq(a483, a484) {
                                                f16({
                                                    let a485 = v6;
                                                    let a486 = 3u64;
                                                    nat_add(a485, a486)?
                                                }, v1)
                                            } else {
                                                Ok(f50(v5, match v1 {
                                                    Adt2::C0(_, _, _, _, _, h487, _, _, _, _, _) => {
                                                        h487
                                                    }
                                                }))
                                            }
                                        } else {
                                            let a488 = v5;
                                            let a489 = 8u64;
                                            if nat_eq(a488, a489) {
                                                Ok(Adt3::C4(2u64, {
                                                    let m492 = match v1 {
                                                        Adt2::C0(_, _, _, _, _, _, _, _, _, h491, _) => {
                                                            h491
                                                        }
                                                    };
                                                    match m492 {
                                                        Adt1::C0(h490, _, _, _, _, _, _, _, _, _, _) => {
                                                            h490
                                                        }
                                                    }
                                                }, 0u64, 1u64, 0u64))
                                            } else {
                                                f16({
                                                    let a493 = v6;
                                                    let a494 = 3u64;
                                                    nat_add(a493, a494)?
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
        Adt2::C0(_, _, _, _, _, _, _, h495, _, _, _) => {
            h495
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h496, _, _) => {
            h496
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h497, _) => {
            h497
        }
    };
    let v5: u64 = match v1.clone() {
        Adt2::C0(_, _, _, h498, _, _, _, _, _, _, _) => {
            h498
        }
    };
    let v6: u64 = {
        let a499 = v0;
        let a500 = 4u64;
        let a501 = 0u64;
        nat_rem(a499, a500, a501)
    };
    let v7: u64 = {
        let a502 = v0;
        let a503 = 4u64;
        let a504 = 0u64;
        nat_quot(a502, a503, a504)
    };
    let v8: u64 = {
        let a506 = v7;
        let a507 = match v4.clone() {
            Adt1::C0(_, h505, _, _, _, _, _, _, _, _, _) => {
                h505
            }
        };
        let a508 = 0u64;
        nat_quot(a506, a507, a508)
    };
    let v9: u64 = f38(match v4.clone() {
        Adt1::C0(h509, _, _, _, _, _, _, _, _, _, _) => {
            h509
        }
    });
    let v10: u64 = {
        let a514 = {
            let a511 = v7;
            let a512 = match v4.clone() {
                Adt1::C0(_, h510, _, _, _, _, _, _, _, _, _) => {
                    h510
                }
            };
            let a513 = 0u64;
            nat_rem(a511, a512, a513)
        };
        let a515 = v9;
        nat_mul(a514, a515)?
    };
    let v11: u64 = {
        let a517 = match v4.clone() {
            Adt1::C0(_, h516, _, _, _, _, _, _, _, _, _) => {
                h516
            }
        };
        let a518 = v9;
        nat_mul(a517, a518)?
    };
    let a520 = v0;
    let a521 = match v1.clone() {
        Adt2::C0(h519, _, _, _, _, _, _, _, _, _, _) => {
            h519
        }
    };
    if nat_lt(a520, a521) {
        let a522 = v6;
        let a523 = 0u64;
        if nat_eq(a522, a523) {
            Ok(Adt3::C3(0u64, 0u64, match v2.clone() {
                Adt1::C0(h524, _, _, _, _, _, _, _, _, _, _) => {
                    h524
                }
            }, 0u64, 0u64, {
                let a532 = {
                    let a528 = {
                        let a526 = v8;
                        let a527 = match v2 {
                            Adt1::C0(_, _, _, _, _, _, h525, _, _, _, _) => {
                                h525
                            }
                        };
                        nat_mul(a526, a527)?
                    };
                    let a529 = v10;
                    nat_add(a528, a529)?
                };
                let a533 = {
                    let a530 = v5;
                    let a531 = 100u64;
                    if nat_eq(a530, a531) {
                        v11
                    } else {
                        0u64
                    }
                };
                nat_add(a532, a533)?
            }, 2u64, 0u64, 0u64, 0u64, v9, 4u64))
        } else {
            let a534 = v6;
            let a535 = 1u64;
            if nat_eq(a534, a535) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h536, _, _, _, _, _, _, _, _, _, _) => {
                        h536
                    }
                }, 0u64, {
                    let a537 = v5;
                    let a538 = 98u64;
                    if nat_eq(a537, a538) {
                        4u64
                    } else {
                        0u64
                    }
                }, {
                    let a546 = {
                        let a542 = {
                            let a540 = v8;
                            let a541 = match v3 {
                                Adt1::C0(_, _, _, _, _, _, h539, _, _, _, _) => {
                                    h539
                                }
                            };
                            nat_mul(a540, a541)?
                        };
                        let a543 = v10;
                        nat_add(a542, a543)?
                    };
                    let a547 = {
                        let a544 = v5;
                        let a545 = 99u64;
                        if nat_eq(a544, a545) {
                            v11
                        } else {
                            0u64
                        }
                    };
                    nat_add(a546, a547)?
                }, 2u64, 0u64, 0u64, 1u64, v9, 4u64))
            } else {
                let a548 = v6;
                let a549 = 2u64;
                if nat_eq(a548, a549) {
                    Ok(Adt3::C4(match v1 {
                        Adt2::C0(_, _, _, _, h550, _, _, _, _, _, _) => {
                            h550
                        }
                    }, match v4 {
                        Adt1::C0(h551, _, _, _, _, _, _, _, _, _, _) => {
                            h551
                        }
                    }, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt3::C3(0u64, 0u64, 0u64, match v4.clone() {
                        Adt1::C0(h552, _, _, _, _, _, _, _, _, _, _) => {
                            h552
                        }
                    }, 1u64, 0u64, 2u64, 0u64, 3u64, {
                        let a556 = {
                            let a554 = v8;
                            let a555 = match v4 {
                                Adt1::C0(_, _, _, _, _, _, h553, _, _, _, _) => {
                                    h553
                                }
                            };
                            nat_mul(a554, a555)?
                        };
                        let a557 = v10;
                        nat_add(a556, a557)?
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
        Adt2::C0(_, _, _, _, _, _, _, h558, _, _, _) => {
            h558
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h559, _, _) => {
            h559
        }
    };
    let v5: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h560, _) => {
            h560
        }
    };
    let v6: u64 = {
        let a562 = match v1.clone() {
            Adt2::C0(_, h561, _, _, _, _, _, _, _, _, _) => {
                h561
            }
        };
        let a563 = 3u64;
        if nat_eq(a562, a563) {
            0u64
        } else {
            let a564 = f32(v3.clone())?;
            let a565 = 2u64;
            nat_mul(a564, a565)?
        }
    };
    let a567 = match v1.clone() {
        Adt2::C0(h566, _, _, _, _, _, _, _, _, _, _) => {
            h566
        }
    };
    let a568 = v0;
    if nat_le(a567, a568) {
        Ok(Adt3::C1)
    } else {
        let a569 = v0;
        let a570 = v6;
        if nat_lt(a569, a570) {
            let v7: u64 = {
                let a571 = v0;
                let a572 = 2u64;
                let a573 = 0u64;
                nat_quot(a571, a572, a573)
            };
            let v8: u64 = {
                let a578 = 3u64;
                let a579 = {
                    let a575 = f43(v7, v5.clone())?;
                    let a576 = f27(match v5 {
                        Adt1::C0(h574, _, _, _, _, _, _, _, _, _, _) => {
                            h574
                        }
                    });
                    let a577 = 0u64;
                    nat_quot(a575, a576, a577)
                };
                nat_add(a578, a579)?
            };
            let a583 = {
                let a580 = v0;
                let a581 = 2u64;
                let a582 = 0u64;
                nat_rem(a580, a581, a582)
            };
            let a584 = 0u64;
            if nat_eq(a583, a584) {
                Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, v8, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
            } else {
                let a585 = v2;
                let a586 = 0u64;
                if nat_eq(a585, a586) {
                    Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 2u64, 1u64, 2u64, 0u64, 0u64, v8, 8u64, 8u64))
                } else {
                    Ok(Adt3::C1)
                }
            }
        } else {
            let v9: u64 = {
                let a587 = v0;
                let a588 = v6;
                nat_sub(a587, a588)
            };
            let v10: u64 = f33(match v3.clone() {
                Adt1::C0(h589, _, _, _, _, _, _, _, _, _, _) => {
                    h589
                }
            }, match v4.clone() {
                Adt1::C0(h590, _, _, _, _, _, _, _, _, _, _) => {
                    h590
                }
            }, match v5.clone() {
                Adt1::C0(h591, _, _, _, _, _, _, _, _, _, _) => {
                    h591
                }
            })?;
            let v11: u64 = {
                let a592 = v9;
                let a593 = v10;
                let a594 = 0u64;
                nat_quot(a592, a593, a594)
            };
            let v12: u64 = {
                let a595 = v9;
                let a596 = v10;
                let a597 = 0u64;
                nat_rem(a595, a596, a597)
            };
            let v13: u64 = {
                let a599 = match v3.clone() {
                    Adt1::C0(h598, _, _, _, _, _, _, _, _, _, _) => {
                        h598
                    }
                };
                let a600 = 0u64;
                if nat_eq(a599, a600) {
                    0u64
                } else {
                    1u64
                }
            };
            let v14: u64 = {
                let a602 = match v4.clone() {
                    Adt1::C0(h601, _, _, _, _, _, _, _, _, _, _) => {
                        h601
                    }
                };
                let a603 = 0u64;
                if nat_eq(a602, a603) {
                    0u64
                } else {
                    1u64
                }
            };
            let a604 = v12;
            let a605 = v13;
            if nat_lt(a604, a605) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h606, _, _, _, _, _, _, _, _, _, _) => {
                        h606
                    }
                }, 0u64, 0u64, f43(v11, v3.clone())?, 2u64, 0u64, 0u64, 0u64, f27(match v3 {
                    Adt1::C0(h607, _, _, _, _, _, _, _, _, _, _) => {
                        h607
                    }
                }), 4u64))
            } else {
                let a610 = v12;
                let a611 = {
                    let a608 = v13;
                    let a609 = v14;
                    nat_add(a608, a609)?
                };
                if nat_lt(a610, a611) {
                    Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                        Adt1::C0(h612, _, _, _, _, _, _, _, _, _, _) => {
                            h612
                        }
                    }, 0u64, 4u64, f44(v11, v3, v4.clone())?, 2u64, 0u64, 0u64, 1u64, f27(match v4 {
                        Adt1::C0(h613, _, _, _, _, _, _, _, _, _, _) => {
                            h613
                        }
                    }), 4u64))
                } else {
                    let a616 = v12;
                    let a617 = {
                        let a614 = v13;
                        let a615 = v14;
                        nat_add(a614, a615)?
                    };
                    if nat_eq(a616, a617) {
                        Ok(Adt3::C3(1u64, match v1 {
                            Adt2::C0(_, _, _, h618, _, _, _, _, _, _, _) => {
                                h618
                            }
                        }, 0u64, 0u64, {
                            let a619 = v13;
                            let a620 = 0u64;
                            if nat_eq(a619, a620) {
                                0u64
                            } else {
                                1u64
                            }
                        }, {
                            let a621 = v13;
                            let a622 = 0u64;
                            if nat_eq(a621, a622) {
                                f43(v11, v3.clone())?
                            } else {
                                0u64
                            }
                        }, {
                            let a623 = v14;
                            let a624 = 0u64;
                            if nat_eq(a623, a624) {
                                4u64
                            } else {
                                1u64
                            }
                        }, {
                            let a625 = v14;
                            let a626 = 0u64;
                            if nat_eq(a625, a626) {
                                f44(v11, v3, v4)?
                            } else {
                                1u64
                            }
                        }, {
                            let a628 = match v5.clone() {
                                Adt1::C0(h627, _, _, _, _, _, _, _, _, _, _) => {
                                    h627
                                }
                            };
                            let a629 = 0u64;
                            if nat_eq(a628, a629) {
                                3u64
                            } else {
                                0u64
                            }
                        }, {
                            let a631 = match v5.clone() {
                                Adt1::C0(h630, _, _, _, _, _, _, _, _, _, _) => {
                                    h630
                                }
                            };
                            let a632 = 0u64;
                            if nat_eq(a631, a632) {
                                f43(v11, v5)?
                            } else {
                                0u64
                            }
                        }, 4u64, 4u64))
                    } else {
                        Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                            Adt1::C0(h633, _, _, _, _, _, _, _, _, _, _) => {
                                h633
                            }
                        }, 1u64, 0u64, 2u64, 0u64, 3u64, f43(v11, v5.clone())?, 4u64, f27(match v5 {
                            Adt1::C0(h634, _, _, _, _, _, _, _, _, _, _) => {
                                h634
                            }
                        })))
                    }
                }
            }
        }
    }
}

pub fn f18(v0: Adt1, v1: Adt1) -> bool {
    if f24(v1.clone()) {
        f24(v0)
    } else {
        let a640 = {
            let a637 = match v0.clone() {
                Adt1::C0(_, h635, _, _, _, _, _, _, _, _, _) => {
                    h635
                }
            };
            let a638 = match v1.clone() {
                Adt1::C0(_, h636, _, _, _, _, _, _, _, _, _) => {
                    h636
                }
            };
            let a639 = 1u64;
            nat_rem(a637, a638, a639)
        };
        let a641 = 0u64;
        if if if nat_eq(a640, a641) {
            let a647 = {
                let a644 = match v0.clone() {
                    Adt1::C0(_, _, h642, _, _, _, _, _, _, _, _) => {
                        h642
                    }
                };
                let a645 = match v1.clone() {
                    Adt1::C0(_, _, h643, _, _, _, _, _, _, _, _) => {
                        h643
                    }
                };
                let a646 = 1u64;
                nat_rem(a644, a645, a646)
            };
            let a648 = 0u64;
            nat_eq(a647, a648)
        } else {
            false
        } {
            let a654 = {
                let a651 = match v0.clone() {
                    Adt1::C0(_, _, _, h649, _, _, _, _, _, _, _) => {
                        h649
                    }
                };
                let a652 = match v1.clone() {
                    Adt1::C0(_, _, _, h650, _, _, _, _, _, _, _) => {
                        h650
                    }
                };
                let a653 = 1u64;
                nat_rem(a651, a652, a653)
            };
            let a655 = 0u64;
            nat_eq(a654, a655)
        } else {
            false
        } {
            let a661 = {
                let a658 = match v0 {
                    Adt1::C0(_, _, _, _, h656, _, _, _, _, _, _) => {
                        h656
                    }
                };
                let a659 = match v1 {
                    Adt1::C0(_, _, _, _, h657, _, _, _, _, _, _) => {
                        h657
                    }
                };
                let a660 = 1u64;
                nat_rem(a658, a659, a660)
            };
            let a662 = 0u64;
            nat_eq(a661, a662)
        } else {
            false
        }
    }
}

pub fn f19(v0: Adt1, v1: Adt1) -> bool {
    let a665 = match v0.clone() {
        Adt1::C0(_, h663, _, _, _, _, _, _, _, _, _) => {
            h663
        }
    };
    let a666 = match v1.clone() {
        Adt1::C0(_, h664, _, _, _, _, _, _, _, _, _) => {
            h664
        }
    };
    if if if nat_eq(a665, a666) {
        let a669 = match v0.clone() {
            Adt1::C0(_, _, h667, _, _, _, _, _, _, _, _) => {
                h667
            }
        };
        let a670 = match v1.clone() {
            Adt1::C0(_, _, h668, _, _, _, _, _, _, _, _) => {
                h668
            }
        };
        nat_eq(a669, a670)
    } else {
        false
    } {
        let a673 = match v0.clone() {
            Adt1::C0(_, _, _, h671, _, _, _, _, _, _, _) => {
                h671
            }
        };
        let a674 = match v1.clone() {
            Adt1::C0(_, _, _, h672, _, _, _, _, _, _, _) => {
                h672
            }
        };
        nat_eq(a673, a674)
    } else {
        false
    } {
        let a677 = match v0 {
            Adt1::C0(_, _, _, _, h675, _, _, _, _, _, _) => {
                h675
            }
        };
        let a678 = match v1 {
            Adt1::C0(_, _, _, _, h676, _, _, _, _, _, _) => {
                h676
            }
        };
        nat_eq(a677, a678)
    } else {
        false
    }
}

pub fn f20(v0: u64) -> u64 {
    if f2(v0) {
        let m679 = f51(v0);
        match m679 {
            Adt4::C0 => {
                0u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, _, h680) => {
                        h680
                    }
                }
            }
        }
    } else {
        let a681 = v0;
        let a682 = 26u64;
        if nat_eq(a681, a682) {
            4u64
        } else {
            f27(v0)
        }
    }
}

pub fn f21(v0: u64) -> u64 {
    if f2(v0) {
        let m683 = f51(v0);
        match m683 {
            Adt4::C0 => {
                1u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, h684, _) => {
                        h684
                    }
                }
            }
        }
    } else {
        1u64
    }
}

pub fn f22(v0: Adt1, v1: Adt1) -> R<bool> {
    if f40(v0.clone(), v1.clone())? {
        Ok(true)
    } else {
        Ok(f52(v0, v1))
    }
}

pub fn f23(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f53(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f53(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f53(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f53(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f53(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f53(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f24(v0: Adt1) -> bool {
    f54(match v0.clone() {
        Adt1::C0(_, h685, _, _, _, _, _, _, _, _, _) => {
            h685
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h686, _, _, _, _, _, _, _, _) => {
            h686
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h687, _, _, _, _, _, _, _) => {
            h687
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, h688, _, _, _, _, _, _) => {
            h688
        }
    })
}

pub fn f25(v0: u64) -> Option<u64> {
    let a689 = v0;
    let a690 = 16u64;
    if if if nat_eq(a689, a690) {
        true
    } else {
        let a691 = v0;
        let a692 = 17u64;
        nat_eq(a691, a692)
    } {
        true
    } else {
        let a693 = v0;
        let a694 = 19u64;
        nat_eq(a693, a694)
    } {
        Some(1065353216u64)
    } else {
        None::<u64>
    }
}

pub fn f26(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<bool> {
    let a696 = match v1.clone() {
        Adt1::C0(h695, _, _, _, _, _, _, _, _, _, _) => {
            h695
        }
    };
    let a697 = 43u64;
    if nat_eq(a696, a697) {
        let a698 = v0;
        let a699 = 0u64;
        if if if if if if nat_eq(a698, a699) {
            let a701 = match v2.clone() {
                Adt1::C0(h700, _, _, _, _, _, _, _, _, _, _) => {
                    h700
                }
            };
            let a702 = 0u64;
            nat_eq(a701, a702)
        } else {
            false
        } {
            let a704 = match v3.clone() {
                Adt1::C0(h703, _, _, _, _, _, _, _, _, _, _) => {
                    h703
                }
            };
            let a705 = 0u64;
            if nat_eq(a704, a705) {
                true
            } else {
                let a707 = match v3.clone() {
                    Adt1::C0(h706, _, _, _, _, _, _, _, _, _, _) => {
                        h706
                    }
                };
                let a708 = 43u64;
                nat_eq(a707, a708)
            }
        } else {
            false
        } {
            let a710 = match v3.clone() {
                Adt1::C0(h709, _, _, _, _, _, _, _, _, _, _) => {
                    h709
                }
            };
            let a711 = 43u64;
            if nat_eq(a710, a711) {
                f18(v1.clone(), v2.clone())
            } else {
                f19(v1.clone(), v2.clone())
            }
        } else {
            false
        } {
            let a713 = match v2 {
                Adt1::C0(_, _, _, _, _, h712, _, _, _, _, _) => {
                    h712
                }
            };
            let a714 = 4u64;
            nat_eq(a713, a714)
        } else {
            false
        } {
            let a717 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h715, _, _, _, _, _) => {
                    h715
                }
            };
            let a718 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h716, _, _, _, _) => {
                    h716
                }
            };
            if nat_le(a717, a718) {
                let a721 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, h719, _, _, _, _) => {
                        h719
                    }
                };
                let a722 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h720, _, _, _) => {
                        h720
                    }
                };
                if nat_le(a721, a722) {
                    let a725 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, h723, _, _, _) => {
                            h723
                        }
                    };
                    let a726 = match v3 {
                        Adt1::C0(_, _, _, _, _, _, _, _, h724, _, _) => {
                            h724
                        }
                    };
                    nat_le(a725, a726)
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        } {
            let a734 = {
                let a732 = match v1.clone() {
                    Adt1::C0(_, _, h727, _, _, _, _, _, _, _, _) => {
                        h727
                    }
                };
                let a733 = {
                    let a730 = match v1.clone() {
                        Adt1::C0(_, _, _, h728, _, _, _, _, _, _, _) => {
                            h728
                        }
                    };
                    let a731 = match v1 {
                        Adt1::C0(_, _, _, _, h729, _, _, _, _, _, _) => {
                            h729
                        }
                    };
                    nat_mul(a730, a731)?
                };
                nat_mul(a732, a733)?
            };
            let a735 = 2147483647u64;
            Ok(nat_le(a734, a735))
        } else {
            Ok(false)
        }
    } else {
        if f18(v1.clone(), v2.clone()) {
            Ok(f55(match v1 {
                Adt1::C0(h736, _, _, _, _, _, _, _, _, _, _) => {
                    h736
                }
            }, match v2 {
                Adt1::C0(h737, _, _, _, _, _, _, _, _, _, _) => {
                    h737
                }
            }, match v3 {
                Adt1::C0(h738, _, _, _, _, _, _, _, _, _, _) => {
                    h738
                }
            }))
        } else {
            Ok(false)
        }
    }
}

pub fn f27(v0: u64) -> u64 {
    let a739 = v0;
    let a740 = 43u64;
    if nat_eq(a739, a740) {
        1u64
    } else {
        f38(v0)
    }
}

pub fn f28(v0: Adt1) -> bool {
    let v1: u64 = f27(match v0.clone() {
        Adt1::C0(h741, _, _, _, _, _, _, _, _, _, _) => {
            h741
        }
    });
    let a746 = {
        let a743 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h742) => {
                h742
            }
        };
        let a744 = v1;
        let a745 = 1u64;
        nat_rem(a743, a744, a745)
    };
    let a747 = 0u64;
    if if nat_eq(a746, a747) {
        f56(match v0.clone() {
            Adt1::C0(_, h748, _, _, _, _, _, _, _, _, _) => {
                h748
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h749, _, _, _, _, _) => {
                h749
            }
        }, v1)
    } else {
        false
    } {
        f57(match v0.clone() {
            Adt1::C0(_, _, h750, _, _, _, _, _, _, _, _) => {
                h750
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h751, _, _, _, _, _, _, _) => {
                h751
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h752, _, _, _, _, _, _) => {
                h752
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h753, _, _, _, _) => {
                h753
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h754, _, _, _) => {
                h754
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h755, _, _) => {
                h755
            }
        }, v1)
    } else {
        false
    }
}

pub fn f29(v0: Adt1, v1: Adt1) -> R<bool> {
    let a763 = {
        let a761 = match v0.clone() {
            Adt1::C0(_, _, h756, _, _, _, _, _, _, _, _) => {
                h756
            }
        };
        let a762 = {
            let a759 = match v0.clone() {
                Adt1::C0(_, _, _, h757, _, _, _, _, _, _, _) => {
                    h757
                }
            };
            let a760 = match v0.clone() {
                Adt1::C0(_, _, _, _, h758, _, _, _, _, _, _) => {
                    h758
                }
            };
            nat_mul(a759, a760)?
        };
        nat_mul(a761, a762)?
    };
    let a764 = 9223372036854775807u64;
    if nat_le(a763, a764) {
        if f36(v1.clone()) {
            let a766 = match v1 {
                Adt1::C0(_, h765, _, _, _, _, _, _, _, _, _) => {
                    h765
                }
            };
            let a767 = 2147483647u64;
            Ok(nat_le(a766, a767))
        } else {
            let a769 = match v0 {
                Adt1::C0(_, h768, _, _, _, _, _, _, _, _, _) => {
                    h768
                }
            };
            let a770 = 2147483647u64;
            Ok(nat_le(a769, a770))
        }
    } else {
        Ok(false)
    }
}

pub fn f30(v0: Adt1) -> R<bool> {
    f23({
        let a773 = match v0.clone() {
            Adt1::C0(_, h771, _, _, _, _, _, _, _, _, _) => {
                h771
            }
        };
        let a774 = f27(match v0.clone() {
            Adt1::C0(h772, _, _, _, _, _, _, _, _, _, _) => {
                h772
            }
        });
        nat_mul(a773, a774)?
    }, match v0.clone() {
        Adt1::C0(_, _, h775, _, _, _, _, _, _, _, _) => {
            h775
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h776, _, _, _, _, _, _, _) => {
            h776
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h777, _, _, _, _, _, _) => {
            h777
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h778, _, _, _, _) => {
            h778
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h779, _, _, _) => {
            h779
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h780, _, _) => {
            h780
        }
    })
}

pub fn f31(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a787 = {
        let a781 = v0;
        let a782 = v2;
        nat_mul(a781, a782)?
    };
    let a788 = {
        let a783 = v1;
        let a784 = 0u64;
        if nat_eq(a783, a784) {
            0u64
        } else {
            let a785 = v0;
            let a786 = 2u64;
            nat_mul(a785, a786)?
        }
    };
    nat_add(a787, a788)
}

pub fn f32(v0: Adt1) -> R<u64> {
    let a797 = {
        let a791 = match v0.clone() {
            Adt1::C0(_, h789, _, _, _, _, _, _, _, _, _) => {
                h789
            }
        };
        let a792 = match v0.clone() {
            Adt1::C0(_, _, h790, _, _, _, _, _, _, _, _) => {
                h790
            }
        };
        nat_mul(a791, a792)?
    };
    let a798 = {
        let a795 = match v0.clone() {
            Adt1::C0(_, _, _, h793, _, _, _, _, _, _, _) => {
                h793
            }
        };
        let a796 = match v0 {
            Adt1::C0(_, _, _, _, h794, _, _, _, _, _, _) => {
                h794
            }
        };
        nat_mul(a795, a796)?
    };
    nat_mul(a797, a798)
}

pub fn f33(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a809 = {
        let a805 = {
            let a801 = 1u64;
            let a802 = {
                let a799 = v0;
                let a800 = 0u64;
                if nat_eq(a799, a800) {
                    0u64
                } else {
                    1u64
                }
            };
            nat_add(a801, a802)?
        };
        let a806 = {
            let a803 = v1;
            let a804 = 0u64;
            if nat_eq(a803, a804) {
                0u64
            } else {
                1u64
            }
        };
        nat_add(a805, a806)?
    };
    let a810 = {
        let a807 = v2;
        let a808 = 0u64;
        if nat_eq(a807, a808) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a809, a810)
}

pub fn f34(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f19(v0.clone(), v1.clone()) {
        let a813 = match v0.clone() {
            Adt1::C0(h811, _, _, _, _, _, _, _, _, _, _) => {
                h811
            }
        };
        let a814 = match v1.clone() {
            Adt1::C0(h812, _, _, _, _, _, _, _, _, _, _) => {
                h812
            }
        };
        nat_eq(a813, a814)
    } else {
        false
    } {
        let a817 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h815, _, _, _, _, _) => {
                h815
            }
        };
        let a818 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h816, _, _, _, _, _) => {
                h816
            }
        };
        nat_eq(a817, a818)
    } else {
        false
    } {
        let a821 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h819, _, _, _, _) => {
                h819
            }
        };
        let a822 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h820, _, _, _, _) => {
                h820
            }
        };
        nat_eq(a821, a822)
    } else {
        false
    } {
        let a825 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h823, _, _, _) => {
                h823
            }
        };
        let a826 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h824, _, _, _) => {
                h824
            }
        };
        nat_eq(a825, a826)
    } else {
        false
    } {
        let a829 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h827, _, _) => {
                h827
            }
        };
        let a830 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h828, _, _) => {
                h828
            }
        };
        nat_eq(a829, a830)
    } else {
        false
    } {
        let a832 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h831) => {
                h831
            }
        };
        let a833 = 0u64;
        if if nat_eq(a832, a833) {
            true
        } else {
            let a835 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h834) => {
                    h834
                }
            };
            let a836 = 0u64;
            nat_eq(a835, a836)
        } {
            true
        } else {
            let a839 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h837) => {
                    h837
                }
            };
            let a840 = match v1 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h838) => {
                    h838
                }
            };
            nat_eq(a839, a840)
        }
    } else {
        false
    }
}

pub fn f35(v0: u64) -> u64 {
    let a841 = v0;
    let a842 = 26u64;
    if nat_eq(a841, a842) {
        4u64
    } else {
        let a843 = v0;
        let a844 = 27u64;
        if nat_eq(a843, a844) {
            8u64
        } else {
            0u64
        }
    }
}

pub fn f36(v0: Adt1) -> bool {
    let a846 = match v0.clone() {
        Adt1::C0(_, h845, _, _, _, _, _, _, _, _, _) => {
            h845
        }
    };
    let a847 = 1u64;
    if nat_eq(a846, a847) {
        true
    } else {
        let a850 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h848, _, _, _, _, _) => {
                h848
            }
        };
        let a851 = f27(match v0 {
            Adt1::C0(h849, _, _, _, _, _, _, _, _, _, _) => {
                h849
            }
        });
        nat_eq(a850, a851)
    }
}

pub fn f37(v0: Adt1, v1: u64) -> R<u64> {
    let a857 = {
        let a853 = match v0.clone() {
            Adt1::C0(_, h852, _, _, _, _, _, _, _, _, _) => {
                h852
            }
        };
        let a854 = f21(v1);
        let a855 = 0u64;
        nat_quot(a853, a854, a855)
    };
    let a858 = f58(match v0 {
        Adt1::C0(h856, _, _, _, _, _, _, _, _, _, _) => {
            h856
        }
    }, v1);
    nat_mul(a857, a858)
}

pub fn f38(v0: u64) -> u64 {
    let a859 = v0;
    let a860 = 0u64;
    if nat_eq(a859, a860) {
        4u64
    } else {
        let a861 = v0;
        let a862 = 1u64;
        if if nat_eq(a861, a862) {
            true
        } else {
            let a863 = v0;
            let a864 = 30u64;
            nat_eq(a863, a864)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f39(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a866 = match v0.clone() {
            Adt1::C0(_, h865, _, _, _, _, _, _, _, _, _) => {
                h865
            }
        };
        let a867 = 4u64;
        nat_mul(a866, a867)?
    };
    let v2: u64 = {
        let a870 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h868, _, _, _, _) => {
                h868
            }
        };
        let a871 = match v0.clone() {
            Adt1::C0(_, _, h869, _, _, _, _, _, _, _, _) => {
                h869
            }
        };
        nat_mul(a870, a871)?
    };
    let v3: u64 = {
        let a873 = v2;
        let a874 = match v0.clone() {
            Adt1::C0(_, _, _, h872, _, _, _, _, _, _, _) => {
                h872
            }
        };
        nat_mul(a873, a874)?
    };
    let a876 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h875, _, _, _, _, _) => {
            h875
        }
    };
    let a877 = 4u64;
    if if if if nat_eq(a876, a877) {
        let a882 = {
            let a879 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h878) => {
                    h878
                }
            };
            let a880 = 4u64;
            let a881 = 1u64;
            nat_rem(a879, a880, a881)
        };
        let a883 = 0u64;
        nat_eq(a882, a883)
    } else {
        false
    } {
        let a891 = {
            let a889 = match v0.clone() {
                Adt1::C0(_, _, h884, _, _, _, _, _, _, _, _) => {
                    h884
                }
            };
            let a890 = {
                let a887 = match v0.clone() {
                    Adt1::C0(_, _, _, h885, _, _, _, _, _, _, _) => {
                        h885
                    }
                };
                let a888 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h886, _, _, _, _, _, _) => {
                        h886
                    }
                };
                nat_mul(a887, a888)?
            };
            nat_mul(a889, a890)?
        };
        let a892 = 1u64;
        if nat_le(a891, a892) {
            true
        } else {
            let a894 = v1;
            let a895 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h893, _, _, _, _) => {
                    h893
                }
            };
            if nat_le(a894, a895) {
                let a900 = {
                    let a897 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, h896, _, _, _, _) => {
                            h896
                        }
                    };
                    let a898 = 4u64;
                    let a899 = 1u64;
                    nat_rem(a897, a898, a899)
                };
                let a901 = 0u64;
                nat_eq(a900, a901)
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a903 = match v0.clone() {
            Adt1::C0(_, _, _, h902, _, _, _, _, _, _, _) => {
                h902
            }
        };
        let a904 = 1u64;
        if nat_le(a903, a904) {
            true
        } else {
            let a906 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h905, _, _, _) => {
                    h905
                }
            };
            let a907 = v2;
            nat_eq(a906, a907)
        }
    } else {
        false
    } {
        let a909 = match v0.clone() {
            Adt1::C0(_, _, _, _, h908, _, _, _, _, _, _) => {
                h908
            }
        };
        let a910 = 1u64;
        if nat_le(a909, a910) {
            Ok(true)
        } else {
            let a912 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, h911, _, _) => {
                    h911
                }
            };
            let a913 = v3;
            Ok(nat_eq(a912, a913))
        }
    } else {
        Ok(false)
    }
}

pub fn f40(v0: Adt1, v1: Adt1) -> R<bool> {
    let a915 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h914, _) => {
            h914
        }
    };
    let a916 = 0u64;
    if nat_eq(a915, a916) {
        Ok(true)
    } else {
        let a918 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h917, _) => {
                h917
            }
        };
        let a919 = 0u64;
        if nat_eq(a918, a919) {
            Ok(true)
        } else {
            let a921 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h920) => {
                    h920
                }
            };
            let a922 = 0u64;
            if nat_eq(a921, a922) {
                Ok(true)
            } else {
                let a924 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h923) => {
                        h923
                    }
                };
                let a925 = 0u64;
                if nat_eq(a924, a925) {
                    Ok(true)
                } else {
                    let a931 = {
                        let a928 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h926) => {
                                h926
                            }
                        };
                        let a929 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h927, _) => {
                                h927
                            }
                        };
                        nat_add(a928, a929)?
                    };
                    let a932 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h930) => {
                            h930
                        }
                    };
                    if nat_le(a931, a932) {
                        Ok(true)
                    } else {
                        let a938 = {
                            let a935 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h933) => {
                                    h933
                                }
                            };
                            let a936 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h934, _) => {
                                    h934
                                }
                            };
                            nat_add(a935, a936)?
                        };
                        let a939 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h937) => {
                                h937
                            }
                        };
                        Ok(nat_le(a938, a939))
                    }
                }
            }
        }
    }
}

pub fn f41(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a942 = match v0.clone() {
            Adt1::C0(_, _, h940, _, _, _, _, _, _, _, _) => {
                h940
            }
        };
        let a943 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h941, _, _, _, _) => {
                h941
            }
        };
        nat_mul(a942, a943)?
    };
    if if f36(v0.clone()) {
        let a945 = match v0.clone() {
            Adt1::C0(_, _, _, h944, _, _, _, _, _, _, _) => {
                h944
            }
        };
        let a946 = 1u64;
        if nat_eq(a945, a946) {
            true
        } else {
            let a948 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h947, _, _, _) => {
                    h947
                }
            };
            let a949 = v1;
            nat_eq(a948, a949)
        }
    } else {
        false
    } {
        let a951 = match v0.clone() {
            Adt1::C0(_, _, _, _, h950, _, _, _, _, _, _) => {
                h950
            }
        };
        let a952 = 1u64;
        if nat_eq(a951, a952) {
            Ok(true)
        } else {
            let a957 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h953, _, _) => {
                    h953
                }
            };
            let a958 = {
                let a955 = v1;
                let a956 = match v0 {
                    Adt1::C0(_, _, _, h954, _, _, _, _, _, _, _) => {
                        h954
                    }
                };
                nat_mul(a955, a956)?
            };
            Ok(nat_eq(a957, a958))
        }
    } else {
        Ok(false)
    }
}

pub fn f42(v0: u64) -> u64 {
    let a959 = v0;
    let a960 = 3u64;
    if nat_eq(a959, a960) {
        16u64
    } else {
        10u64
    }
}

pub fn f43(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a962 = v0;
        let a963 = match v1.clone() {
            Adt1::C0(_, h961, _, _, _, _, _, _, _, _, _) => {
                h961
            }
        };
        let a964 = 0u64;
        nat_quot(a962, a963, a964)
    };
    let v3: u64 = {
        let a966 = v2;
        let a967 = match v1.clone() {
            Adt1::C0(_, _, h965, _, _, _, _, _, _, _, _) => {
                h965
            }
        };
        let a968 = 0u64;
        nat_quot(a966, a967, a968)
    };
    f45({
        let a970 = v0;
        let a971 = match v1.clone() {
            Adt1::C0(_, h969, _, _, _, _, _, _, _, _, _) => {
                h969
            }
        };
        let a972 = 0u64;
        nat_rem(a970, a971, a972)
    }, {
        let a974 = v2;
        let a975 = match v1.clone() {
            Adt1::C0(_, _, h973, _, _, _, _, _, _, _, _) => {
                h973
            }
        };
        let a976 = 0u64;
        nat_rem(a974, a975, a976)
    }, {
        let a978 = v3;
        let a979 = match v1.clone() {
            Adt1::C0(_, _, _, h977, _, _, _, _, _, _, _) => {
                h977
            }
        };
        let a980 = 0u64;
        nat_rem(a978, a979, a980)
    }, {
        let a982 = v3;
        let a983 = match v1.clone() {
            Adt1::C0(_, _, _, h981, _, _, _, _, _, _, _) => {
                h981
            }
        };
        let a984 = 0u64;
        nat_quot(a982, a983, a984)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h985, _, _, _, _, _) => {
            h985
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h986, _, _, _, _) => {
            h986
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h987, _, _, _) => {
            h987
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h988, _, _) => {
            h988
        }
    })
}

pub fn f44(v0: u64, v1: Adt1, v2: Adt1) -> R<u64> {
    let v3: u64 = {
        let a990 = v0;
        let a991 = match v1.clone() {
            Adt1::C0(_, h989, _, _, _, _, _, _, _, _, _) => {
                h989
            }
        };
        let a992 = 0u64;
        nat_quot(a990, a991, a992)
    };
    let v4: u64 = {
        let a994 = v3;
        let a995 = match v1.clone() {
            Adt1::C0(_, _, h993, _, _, _, _, _, _, _, _) => {
                h993
            }
        };
        let a996 = 0u64;
        nat_quot(a994, a995, a996)
    };
    f45({
        let a1002 = {
            let a998 = v0;
            let a999 = match v1.clone() {
                Adt1::C0(_, h997, _, _, _, _, _, _, _, _, _) => {
                    h997
                }
            };
            let a1000 = 0u64;
            nat_rem(a998, a999, a1000)
        };
        let a1003 = match v2.clone() {
            Adt1::C0(_, h1001, _, _, _, _, _, _, _, _, _) => {
                h1001
            }
        };
        let a1004 = 0u64;
        nat_rem(a1002, a1003, a1004)
    }, {
        let a1010 = {
            let a1006 = v3;
            let a1007 = match v1.clone() {
                Adt1::C0(_, _, h1005, _, _, _, _, _, _, _, _) => {
                    h1005
                }
            };
            let a1008 = 0u64;
            nat_rem(a1006, a1007, a1008)
        };
        let a1011 = match v2.clone() {
            Adt1::C0(_, _, h1009, _, _, _, _, _, _, _, _) => {
                h1009
            }
        };
        let a1012 = 0u64;
        nat_rem(a1010, a1011, a1012)
    }, {
        let a1018 = {
            let a1014 = v4;
            let a1015 = match v1.clone() {
                Adt1::C0(_, _, _, h1013, _, _, _, _, _, _, _) => {
                    h1013
                }
            };
            let a1016 = 0u64;
            nat_rem(a1014, a1015, a1016)
        };
        let a1019 = match v2.clone() {
            Adt1::C0(_, _, _, h1017, _, _, _, _, _, _, _) => {
                h1017
            }
        };
        let a1020 = 0u64;
        nat_rem(a1018, a1019, a1020)
    }, {
        let a1026 = {
            let a1022 = v4;
            let a1023 = match v1 {
                Adt1::C0(_, _, _, h1021, _, _, _, _, _, _, _) => {
                    h1021
                }
            };
            let a1024 = 0u64;
            nat_quot(a1022, a1023, a1024)
        };
        let a1027 = match v2.clone() {
            Adt1::C0(_, _, _, _, h1025, _, _, _, _, _, _) => {
                h1025
            }
        };
        let a1028 = 0u64;
        nat_rem(a1026, a1027, a1028)
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h1029, _, _, _, _, _) => {
            h1029
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h1030, _, _, _, _) => {
            h1030
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1031, _, _, _) => {
            h1031
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1032, _, _) => {
            h1032
        }
    })
}

pub fn f45(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a1045 = {
        let a1041 = {
            let a1037 = {
                let a1033 = v0;
                let a1034 = v4;
                nat_mul(a1033, a1034)?
            };
            let a1038 = {
                let a1035 = v1;
                let a1036 = v5;
                nat_mul(a1035, a1036)?
            };
            nat_add(a1037, a1038)?
        };
        let a1042 = {
            let a1039 = v2;
            let a1040 = v6;
            nat_mul(a1039, a1040)?
        };
        nat_add(a1041, a1042)?
    };
    let a1046 = {
        let a1043 = v3;
        let a1044 = v7;
        nat_mul(a1043, a1044)?
    };
    nat_add(a1045, a1046)
}

pub fn f46(v0: u64, v1: Adt1, v2: u64, v3: u64) -> R<Adt3> {
    let v4: u64 = {
        let a1047 = v0;
        let a1048 = 2u64;
        let a1049 = 0u64;
        nat_quot(a1047, a1048, a1049)
    };
    let v5: u64 = f35(match v1.clone() {
        Adt1::C0(h1050, _, _, _, _, _, _, _, _, _, _) => {
            h1050
        }
    });
    let a1054 = {
        let a1051 = v0;
        let a1052 = 2u64;
        let a1053 = 0u64;
        nat_rem(a1051, a1052, a1053)
    };
    let a1055 = 0u64;
    if nat_eq(a1054, a1055) {
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 4u64, f43(v4, v1)?, 2u64, 0u64, 0u64, 2u64, v5, 8u64))
    } else {
        let a1056 = v3;
        let a1057 = v2;
        if if nat_lt(a1056, a1057) {
            let a1060 = v3;
            let a1061 = {
                let a1058 = v5;
                let a1059 = 4u64;
                if nat_eq(a1058, a1059) {
                    2147483648u64
                } else {
                    9223372036854775808u64
                }
            };
            nat_lt(a1060, a1061)
        } else {
            false
        } {
            Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, {
                let a1062 = 3u64;
                let a1063 = v4;
                nat_add(a1062, a1063)?
            }, 8u64, 8u64))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f47(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64) -> R<Adt3> {
    let v5: u64 = {
        let a1065 = 1u64;
        let a1066 = f37(v1.clone(), match v3.clone() {
            Adt1::C0(h1064, _, _, _, _, _, _, _, _, _, _) => {
                h1064
            }
        })?;
        nat_add(a1065, a1066)?
    };
    let v6: u64 = {
        let a1067 = v0;
        let a1068 = v5;
        let a1069 = 0u64;
        nat_rem(a1067, a1068, a1069)
    };
    let v7: u64 = {
        let a1070 = v0;
        let a1071 = v5;
        let a1072 = 0u64;
        nat_quot(a1070, a1071, a1072)
    };
    let v8: u64 = {
        let a1074 = v7;
        let a1075 = match v1.clone() {
            Adt1::C0(_, _, h1073, _, _, _, _, _, _, _, _) => {
                h1073
            }
        };
        let a1076 = 0u64;
        nat_rem(a1074, a1075, a1076)
    };
    let v9: u64 = {
        let a1078 = v7;
        let a1079 = match v1.clone() {
            Adt1::C0(_, _, h1077, _, _, _, _, _, _, _, _) => {
                h1077
            }
        };
        let a1080 = 0u64;
        nat_quot(a1078, a1079, a1080)
    };
    let v10: u64 = {
        let a1082 = v9;
        let a1083 = match v1.clone() {
            Adt1::C0(_, _, _, h1081, _, _, _, _, _, _, _) => {
                h1081
            }
        };
        let a1084 = 0u64;
        nat_rem(a1082, a1083, a1084)
    };
    let v11: u64 = {
        let a1086 = v9;
        let a1087 = match v1.clone() {
            Adt1::C0(_, _, _, h1085, _, _, _, _, _, _, _) => {
                h1085
            }
        };
        let a1088 = 0u64;
        nat_quot(a1086, a1087, a1088)
    };
    let a1089 = v6;
    let a1090 = 0u64;
    if nat_eq(a1089, a1090) {
        let v12: u64 = {
            let a1107 = v8;
            let a1108 = {
                let a1105 = match v2.clone() {
                    Adt1::C0(_, h1091, _, _, _, _, _, _, _, _, _) => {
                        h1091
                    }
                };
                let a1106 = {
                    let a1103 = {
                        let a1093 = v10;
                        let a1094 = match v2.clone() {
                            Adt1::C0(_, _, h1092, _, _, _, _, _, _, _, _) => {
                                h1092
                            }
                        };
                        let a1095 = 0u64;
                        nat_rem(a1093, a1094, a1095)
                    };
                    let a1104 = {
                        let a1101 = match v2.clone() {
                            Adt1::C0(_, _, h1096, _, _, _, _, _, _, _, _) => {
                                h1096
                            }
                        };
                        let a1102 = {
                            let a1098 = v11;
                            let a1099 = match v2 {
                                Adt1::C0(_, _, _, h1097, _, _, _, _, _, _, _) => {
                                    h1097
                                }
                            };
                            let a1100 = 0u64;
                            nat_rem(a1098, a1099, a1100)
                        };
                        nat_mul(a1101, a1102)?
                    };
                    nat_add(a1103, a1104)?
                };
                nat_mul(a1105, a1106)?
            };
            nat_add(a1107, a1108)?
        };
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, {
            let a1109 = 3u64;
            let a1110 = v12;
            nat_add(a1109, a1110)?
        }, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
    } else {
        let a1112 = v4;
        let a1113 = match v3.clone() {
            Adt1::C0(_, _, h1111, _, _, _, _, _, _, _, _) => {
                h1111
            }
        };
        if nat_lt(a1112, a1113) {
            let v13: u64 = {
                let a1118 = {
                    let a1114 = v6;
                    let a1115 = 1u64;
                    nat_sub(a1114, a1115)
                };
                let a1119 = f58(match v1.clone() {
                    Adt1::C0(h1116, _, _, _, _, _, _, _, _, _, _) => {
                        h1116
                    }
                }, match v3.clone() {
                    Adt1::C0(h1117, _, _, _, _, _, _, _, _, _, _) => {
                        h1117
                    }
                });
                let a1120 = 0u64;
                nat_quot(a1118, a1119, a1120)
            };
            let v14: u64 = f45({
                let a1122 = v13;
                let a1123 = f21(match v3.clone() {
                    Adt1::C0(h1121, _, _, _, _, _, _, _, _, _, _) => {
                        h1121
                    }
                });
                nat_mul(a1122, a1123)?
            }, v8, v10, v11, match v1.clone() {
                Adt1::C0(_, _, _, _, _, h1124, _, _, _, _, _) => {
                    h1124
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, h1125, _, _, _, _) => {
                    h1125
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1126, _, _, _) => {
                    h1126
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1127, _, _) => {
                    h1127
                }
            })?;
            let v15: u64 = f45(v13, v4, v10, v11, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h1128, _, _, _, _, _) => {
                    h1128
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h1129, _, _, _, _) => {
                    h1129
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1130, _, _, _) => {
                    h1130
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1131, _, _) => {
                    h1131
                }
            })?;
            if if f2(match v3.clone() {
                Adt1::C0(h1132, _, _, _, _, _, _, _, _, _, _) => {
                    h1132
                }
            }) {
                true
            } else {
                let a1134 = match v3.clone() {
                    Adt1::C0(h1133, _, _, _, _, _, _, _, _, _, _) => {
                        h1133
                    }
                };
                let a1135 = 26u64;
                nat_eq(a1134, a1135)
            } {
                Ok(Adt3::C6(v14, v15))
            } else {
                Ok(f59(match v1 {
                    Adt1::C0(h1136, _, _, _, _, _, _, _, _, _, _) => {
                        h1136
                    }
                }, match v3 {
                    Adt1::C0(h1137, _, _, _, _, _, _, _, _, _, _) => {
                        h1137
                    }
                }, {
                    let a1138 = v6;
                    let a1139 = 1u64;
                    nat_sub(a1138, a1139)
                }, v14, v15))
            }
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f48(v0: u64, v1: u64, v2: u64) -> Adt3 {
    Adt3::C3(2u64, 0u64, 0u64, 0u64, 1u64, v0, v1, v2, 0u64, 2u64, 4u64, 4u64)
}

pub fn f49(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt3 {
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

pub fn f50(v0: u64, v1: u64) -> Adt3 {
    let a1140 = v0;
    let a1141 = 8u64;
    if nat_eq(a1140, a1141) {
        Adt3::C3(1u64, 6u64, 0u64, 0u64, 1u64, 0u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
    } else {
        let a1142 = v0;
        let a1143 = 9u64;
        if nat_eq(a1142, a1143) {
            Adt3::C3(1u64, 2u64, 0u64, 0u64, 2u64, v1, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
        } else {
            let a1144 = v0;
            let a1145 = 10u64;
            if nat_eq(a1144, a1145) {
                Adt3::C3(5u64, 16u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
            } else {
                let a1146 = v0;
                let a1147 = 11u64;
                if nat_eq(a1146, a1147) {
                    Adt3::C3(1u64, 0u64, 0u64, 0u64, 2u64, 1065353216u64, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
                } else {
                    let a1148 = v0;
                    let a1149 = 12u64;
                    if nat_eq(a1148, a1149) {
                        Adt3::C3(1u64, 3u64, 0u64, 0u64, 1u64, 0u64, 1u64, 2u64, 0u64, 0u64, 4u64, 4u64)
                    } else {
                        let a1150 = v0;
                        let a1151 = 13u64;
                        if nat_eq(a1150, a1151) {
                            Adt3::C3(1u64, 0u64, 0u64, 0u64, 1u64, 1u64, 2u64, 1065353216u64, 0u64, 1u64, 4u64, 4u64)
                        } else {
                            let a1152 = v0;
                            let a1153 = 14u64;
                            if nat_eq(a1152, a1153) {
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

pub fn f51(v0: u64) -> Adt4 {
    let a1154 = v0;
    let a1155 = 0u64;
    if nat_eq(a1154, a1155) {
        Adt4::C1(Adt5::C0(0u64, 1u64, 4u64))
    } else {
        let a1156 = v0;
        let a1157 = 1u64;
        if nat_eq(a1156, a1157) {
            Adt4::C1(Adt5::C0(1u64, 1u64, 2u64))
        } else {
            let a1158 = v0;
            let a1159 = 2u64;
            if nat_eq(a1158, a1159) {
                Adt4::C1(Adt5::C0(2u64, 32u64, 18u64))
            } else {
                let a1160 = v0;
                let a1161 = 3u64;
                if nat_eq(a1160, a1161) {
                    Adt4::C1(Adt5::C0(3u64, 32u64, 20u64))
                } else {
                    let a1162 = v0;
                    let a1163 = 6u64;
                    if nat_eq(a1162, a1163) {
                        Adt4::C1(Adt5::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1164 = v0;
                        let a1165 = 7u64;
                        if nat_eq(a1164, a1165) {
                            Adt4::C1(Adt5::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1166 = v0;
                            let a1167 = 8u64;
                            if nat_eq(a1166, a1167) {
                                Adt4::C1(Adt5::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1168 = v0;
                                let a1169 = 9u64;
                                if nat_eq(a1168, a1169) {
                                    Adt4::C1(Adt5::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1170 = v0;
                                    let a1171 = 10u64;
                                    if nat_eq(a1170, a1171) {
                                        Adt4::C1(Adt5::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1172 = v0;
                                        let a1173 = 11u64;
                                        if nat_eq(a1172, a1173) {
                                            Adt4::C1(Adt5::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1174 = v0;
                                            let a1175 = 12u64;
                                            if nat_eq(a1174, a1175) {
                                                Adt4::C1(Adt5::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1176 = v0;
                                                let a1177 = 13u64;
                                                if nat_eq(a1176, a1177) {
                                                    Adt4::C1(Adt5::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1178 = v0;
                                                    let a1179 = 14u64;
                                                    if nat_eq(a1178, a1179) {
                                                        Adt4::C1(Adt5::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1180 = v0;
                                                        let a1181 = 15u64;
                                                        if nat_eq(a1180, a1181) {
                                                            Adt4::C1(Adt5::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1182 = v0;
                                                            let a1183 = 16u64;
                                                            if nat_eq(a1182, a1183) {
                                                                Adt4::C1(Adt5::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1184 = v0;
                                                                let a1185 = 17u64;
                                                                if nat_eq(a1184, a1185) {
                                                                    Adt4::C1(Adt5::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1186 = v0;
                                                                    let a1187 = 18u64;
                                                                    if nat_eq(a1186, a1187) {
                                                                        Adt4::C1(Adt5::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1188 = v0;
                                                                        let a1189 = 19u64;
                                                                        if nat_eq(a1188, a1189) {
                                                                            Adt4::C1(Adt5::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1190 = v0;
                                                                            let a1191 = 20u64;
                                                                            if nat_eq(a1190, a1191) {
                                                                                Adt4::C1(Adt5::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1192 = v0;
                                                                                let a1193 = 21u64;
                                                                                if nat_eq(a1192, a1193) {
                                                                                    Adt4::C1(Adt5::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1194 = v0;
                                                                                    let a1195 = 22u64;
                                                                                    if nat_eq(a1194, a1195) {
                                                                                        Adt4::C1(Adt5::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1196 = v0;
                                                                                        let a1197 = 23u64;
                                                                                        if nat_eq(a1196, a1197) {
                                                                                            Adt4::C1(Adt5::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1198 = v0;
                                                                                            let a1199 = 24u64;
                                                                                            if nat_eq(a1198, a1199) {
                                                                                                Adt4::C1(Adt5::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1200 = v0;
                                                                                                let a1201 = 25u64;
                                                                                                if nat_eq(a1200, a1201) {
                                                                                                    Adt4::C1(Adt5::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1202 = v0;
                                                                                                    let a1203 = 26u64;
                                                                                                    if nat_eq(a1202, a1203) {
                                                                                                        Adt4::C1(Adt5::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1204 = v0;
                                                                                                        let a1205 = 27u64;
                                                                                                        if nat_eq(a1204, a1205) {
                                                                                                            Adt4::C1(Adt5::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1206 = v0;
                                                                                                            let a1207 = 28u64;
                                                                                                            if nat_eq(a1206, a1207) {
                                                                                                                Adt4::C1(Adt5::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1208 = v0;
                                                                                                                let a1209 = 29u64;
                                                                                                                if nat_eq(a1208, a1209) {
                                                                                                                    Adt4::C1(Adt5::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1210 = v0;
                                                                                                                    let a1211 = 30u64;
                                                                                                                    if nat_eq(a1210, a1211) {
                                                                                                                        Adt4::C1(Adt5::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1212 = v0;
                                                                                                                        let a1213 = 34u64;
                                                                                                                        if nat_eq(a1212, a1213) {
                                                                                                                            Adt4::C1(Adt5::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1214 = v0;
                                                                                                                            let a1215 = 35u64;
                                                                                                                            if nat_eq(a1214, a1215) {
                                                                                                                                Adt4::C1(Adt5::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1216 = v0;
                                                                                                                                let a1217 = 39u64;
                                                                                                                                if nat_eq(a1216, a1217) {
                                                                                                                                    Adt4::C1(Adt5::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1218 = v0;
                                                                                                                                    let a1219 = 40u64;
                                                                                                                                    if nat_eq(a1218, a1219) {
                                                                                                                                        Adt4::C1(Adt5::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1220 = v0;
                                                                                                                                        let a1221 = 41u64;
                                                                                                                                        if nat_eq(a1220, a1221) {
                                                                                                                                            Adt4::C1(Adt5::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1222 = v0;
                                                                                                                                            let a1223 = 42u64;
                                                                                                                                            if nat_eq(a1222, a1223) {
                                                                                                                                                Adt4::C1(Adt5::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1224 = v0;
                                                                                                                                                let a1225 = 43u64;
                                                                                                                                                if nat_eq(a1224, a1225) {
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

pub fn f52(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f19(v0.clone(), v1.clone()) {
        let a1228 = match v0.clone() {
            Adt1::C0(h1226, _, _, _, _, _, _, _, _, _, _) => {
                h1226
            }
        };
        let a1229 = match v1.clone() {
            Adt1::C0(h1227, _, _, _, _, _, _, _, _, _, _) => {
                h1227
            }
        };
        nat_eq(a1228, a1229)
    } else {
        false
    } {
        let a1232 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1230) => {
                h1230
            }
        };
        let a1233 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1231) => {
                h1231
            }
        };
        nat_eq(a1232, a1233)
    } else {
        false
    } {
        let a1236 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1234, _, _, _, _, _) => {
                h1234
            }
        };
        let a1237 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1235, _, _, _, _, _) => {
                h1235
            }
        };
        nat_eq(a1236, a1237)
    } else {
        false
    } {
        let a1240 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1238, _, _, _, _) => {
                h1238
            }
        };
        let a1241 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1239, _, _, _, _) => {
                h1239
            }
        };
        nat_eq(a1240, a1241)
    } else {
        false
    } {
        let a1244 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1242, _, _, _) => {
                h1242
            }
        };
        let a1245 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1243, _, _, _) => {
                h1243
            }
        };
        nat_eq(a1244, a1245)
    } else {
        false
    } {
        let a1248 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1246, _, _) => {
                h1246
            }
        };
        let a1249 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1247, _, _) => {
                h1247
            }
        };
        nat_eq(a1248, a1249)
    } else {
        false
    }
}

pub fn f53(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a1250 = v1;
    let a1251 = 1u64;
    if if if nat_le(a1250, a1251) {
        true
    } else {
        let a1252 = v0;
        let a1253 = v2;
        nat_le(a1252, a1253)
    } {
        let a1254 = v3;
        let a1255 = 1u64;
        if nat_le(a1254, a1255) {
            true
        } else {
            let a1262 = {
                let a1260 = v0;
                let a1261 = {
                    let a1258 = {
                        let a1256 = v1;
                        let a1257 = 1u64;
                        nat_sub(a1256, a1257)
                    };
                    let a1259 = v2;
                    nat_mul(a1258, a1259)?
                };
                nat_add(a1260, a1261)?
            };
            let a1263 = v4;
            nat_le(a1262, a1263)
        }
    } else {
        false
    } {
        let a1264 = v5;
        let a1265 = 1u64;
        if nat_le(a1264, a1265) {
            Ok(true)
        } else {
            let a1278 = {
                let a1276 = {
                    let a1270 = v0;
                    let a1271 = {
                        let a1268 = {
                            let a1266 = v1;
                            let a1267 = 1u64;
                            nat_sub(a1266, a1267)
                        };
                        let a1269 = v2;
                        nat_mul(a1268, a1269)?
                    };
                    nat_add(a1270, a1271)?
                };
                let a1277 = {
                    let a1274 = {
                        let a1272 = v3;
                        let a1273 = 1u64;
                        nat_sub(a1272, a1273)
                    };
                    let a1275 = v4;
                    nat_mul(a1274, a1275)?
                };
                nat_add(a1276, a1277)?
            };
            let a1279 = v6;
            Ok(nat_le(a1278, a1279))
        }
    } else {
        Ok(false)
    }
}

pub fn f54(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1280 = v0;
    let a1281 = 0u64;
    if if if nat_eq(a1280, a1281) {
        true
    } else {
        let a1282 = v1;
        let a1283 = 0u64;
        nat_eq(a1282, a1283)
    } {
        true
    } else {
        let a1284 = v2;
        let a1285 = 0u64;
        nat_eq(a1284, a1285)
    } {
        true
    } else {
        let a1286 = v3;
        let a1287 = 0u64;
        nat_eq(a1286, a1287)
    }
}

pub fn f55(v0: u64, v1: u64, v2: u64) -> bool {
    let a1288 = v0;
    let a1289 = 0u64;
    if if if nat_eq(a1288, a1289) {
        let a1290 = v1;
        let a1291 = 0u64;
        nat_eq(a1290, a1291)
    } else {
        false
    } {
        let a1292 = v2;
        let a1293 = 0u64;
        nat_eq(a1292, a1293)
    } else {
        false
    } {
        true
    } else {
        let a1294 = v0;
        let a1295 = 1u64;
        if if nat_eq(a1294, a1295) {
            true
        } else {
            let a1296 = v0;
            let a1297 = 30u64;
            nat_eq(a1296, a1297)
        } {
            let a1298 = v1;
            let a1299 = v0;
            if if nat_eq(a1298, a1299) {
                let a1300 = v2;
                let a1301 = v0;
                nat_eq(a1300, a1301)
            } else {
                false
            } {
                true
            } else {
                let a1302 = v1;
                let a1303 = 0u64;
                if nat_eq(a1302, a1303) {
                    let a1304 = v2;
                    let a1305 = v0;
                    if nat_eq(a1304, a1305) {
                        true
                    } else {
                        let a1306 = v2;
                        let a1307 = 0u64;
                        nat_eq(a1306, a1307)
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

pub fn f56(v0: u64, v1: u64, v2: u64) -> bool {
    let a1308 = v0;
    let a1309 = 1u64;
    if nat_le(a1308, a1309) {
        true
    } else {
        let a1313 = {
            let a1310 = v1;
            let a1311 = v2;
            let a1312 = 1u64;
            nat_rem(a1310, a1311, a1312)
        };
        let a1314 = 0u64;
        nat_eq(a1313, a1314)
    }
}

pub fn f57(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f56(v0, v3, v6) {
        f56(v1, v4, v6)
    } else {
        false
    } {
        f56(v2, v5, v6)
    } else {
        false
    }
}

pub fn f58(v0: u64, v1: u64) -> u64 {
    let a1315 = v0;
    let a1316 = 1u64;
    if if nat_eq(a1315, a1316) {
        let a1317 = v1;
        let a1318 = 43u64;
        nat_eq(a1317, a1318)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f59(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt3 {
    let a1319 = f58(v0, v1);
    let a1320 = 2u64;
    if nat_eq(a1319, a1320) {
        let a1324 = {
            let a1321 = v2;
            let a1322 = 2u64;
            let a1323 = 0u64;
            nat_rem(a1321, a1322, a1323)
        };
        let a1325 = 0u64;
        if nat_eq(a1324, a1325) {
            Adt3::C3(0u64, 0u64, v0, 0u64, 0u64, v3, 2u64, 0u64, 1u64, 0u64, f38(v0), 8u64)
        } else {
            Adt3::C3(0u64, 0u64, 0u64, v1, 1u64, 0u64, 2u64, 0u64, 3u64, v4, 8u64, f20(v1))
        }
    } else {
        Adt3::C3(0u64, 0u64, v0, v1, 0u64, v3, 2u64, 0u64, 3u64, v4, f38(v0), f20(v1))
    }
}
