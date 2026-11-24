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
    if if if if if if if if if if if if if if if if if nat_eq(a66, a67) {
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
        let a89 = 16u64;
        let a90 = match v2.clone() {
            Adt1::C0(h88, _, _, _, _, _, _, _, _, _, _) => {
                h88
            }
        };
        !if if if nat_le(a89, a90) {
            let a92 = match v2.clone() {
                Adt1::C0(h91, _, _, _, _, _, _, _, _, _, _) => {
                    h91
                }
            };
            let a93 = 19u64;
            nat_le(a92, a93)
        } else {
            false
        } {
            true
        } else {
            let a95 = match v2.clone() {
                Adt1::C0(h94, _, _, _, _, _, _, _, _, _, _) => {
                    h94
                }
            };
            let a96 = 21u64;
            if nat_eq(a95, a96) {
                true
            } else {
                let a98 = match v2.clone() {
                    Adt1::C0(h97, _, _, _, _, _, _, _, _, _, _) => {
                        h97
                    }
                };
                let a99 = 22u64;
                nat_eq(a98, a99)
            }
        } {
            true
        } else {
            let a101 = match v2.clone() {
                Adt1::C0(h100, _, _, _, _, _, _, _, _, _, _) => {
                    h100
                }
            };
            let a102 = 29u64;
            nat_eq(a101, a102)
        }
    } else {
        false
    } {
        let a105 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h103, _, _, _, _, _) => {
                h103
            }
        };
        let a106 = f20(match v0.clone() {
            Adt1::C0(h104, _, _, _, _, _, _, _, _, _, _) => {
                h104
            }
        });
        nat_eq(a105, a106)
    } else {
        false
    } {
        let a109 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h107, _, _, _, _, _) => {
                h107
            }
        };
        let a110 = f20(match v2.clone() {
            Adt1::C0(h108, _, _, _, _, _, _, _, _, _, _) => {
                h108
            }
        });
        nat_eq(a109, a110)
    } else {
        false
    } {
        let a116 = {
            let a113 = match v0.clone() {
                Adt1::C0(_, h111, _, _, _, _, _, _, _, _, _) => {
                    h111
                }
            };
            let a114 = f21(match v0.clone() {
                Adt1::C0(h112, _, _, _, _, _, _, _, _, _, _) => {
                    h112
                }
            });
            let a115 = 1u64;
            nat_rem(a113, a114, a115)
        };
        let a117 = 0u64;
        nat_eq(a116, a117)
    } else {
        false
    } {
        let a123 = {
            let a120 = match v2.clone() {
                Adt1::C0(_, h118, _, _, _, _, _, _, _, _, _) => {
                    h118
                }
            };
            let a121 = f21(match v2.clone() {
                Adt1::C0(h119, _, _, _, _, _, _, _, _, _, _) => {
                    h119
                }
            });
            let a122 = 1u64;
            nat_rem(a120, a121, a122)
        };
        let a124 = 0u64;
        nat_eq(a123, a124)
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
            let a131 = {
                let a127 = match v2.clone() {
                    Adt1::C0(_, h125, _, _, _, _, _, _, _, _, _) => {
                        h125
                    }
                };
                let a128 = f21(match v2.clone() {
                    Adt1::C0(h126, _, _, _, _, _, _, _, _, _, _) => {
                        h126
                    }
                });
                let a129 = 0u64;
                nat_quot(a127, a128, a129)
            };
            let a132 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, h130, _, _, _, _, _) => {
                    h130
                }
            };
            nat_mul(a131, a132)?
        }, match v2.clone() {
            Adt1::C0(_, _, h133, _, _, _, _, _, _, _, _) => {
                h133
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h134, _, _, _, _, _, _, _) => {
                h134
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h135, _, _, _, _, _, _) => {
                h135
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h136, _, _, _, _) => {
                h136
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h137, _, _, _) => {
                h137
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h138, _, _) => {
                h138
            }
        })?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0(if f24(v0.clone()) {
            0u64
        } else {
            let a144 = match v0.clone() {
                Adt1::C0(_, _, h139, _, _, _, _, _, _, _, _) => {
                    h139
                }
            };
            let a145 = {
                let a142 = match v0.clone() {
                    Adt1::C0(_, _, _, h140, _, _, _, _, _, _, _) => {
                        h140
                    }
                };
                let a143 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h141, _, _, _, _, _, _) => {
                        h141
                    }
                };
                nat_mul(a142, a143)?
            };
            nat_mul(a144, a145)?
        }, 3u64, None::<u64>, 0u64, 0u64, 0u64, 0u64, v0, v1, v2, true)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f4(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if f19(v1.clone(), v3.clone()) {
        f25(v0, v1.clone(), v2.clone(), v3.clone())?
    } else {
        false
    } {
        if f24(v1.clone()) {
            true
        } else {
            let a148 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, h146, _, _, _, _, _) => {
                    h146
                }
            };
            let a149 = f26(match v1.clone() {
                Adt1::C0(h147, _, _, _, _, _, _, _, _, _, _) => {
                    h147
                }
            });
            if if if if if if if nat_eq(a148, a149) {
                let a152 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h150, _, _, _, _, _) => {
                        h150
                    }
                };
                let a153 = f26(match v3.clone() {
                    Adt1::C0(h151, _, _, _, _, _, _, _, _, _, _) => {
                        h151
                    }
                });
                nat_eq(a152, a153)
            } else {
                false
            } {
                f27(v1.clone())
            } else {
                false
            } {
                f27(v2.clone())
            } else {
                false
            } {
                f27(v3.clone())
            } else {
                false
            } {
                f28(v1.clone(), v2.clone())?
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
            f29(v3.clone())?
        } {
            0u64
        } else {
            let a156 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h154, _) => {
                    h154
                }
            };
            let a157 = f26(match v3.clone() {
                Adt1::C0(h155, _, _, _, _, _, _, _, _, _, _) => {
                    h155
                }
            });
            let a158 = 0u64;
            nat_quot(a156, a157, a158)
        };
        Ok(Adt3::C2(Adt2::C0(f30(f31(v1.clone())?, v4, f32(match v1.clone() {
            Adt1::C0(h159, _, _, _, _, _, _, _, _, _, _) => {
                h159
            }
        }, match v2.clone() {
            Adt1::C0(h160, _, _, _, _, _, _, _, _, _, _) => {
                h160
            }
        }, match v3.clone() {
            Adt1::C0(h161, _, _, _, _, _, _, _, _, _, _) => {
                h161
            }
        })?)?, {
            let a162 = 3u64;
            let a163 = v4;
            nat_add(a162, a163)?
        }, None::<u64>, v0, 0u64, 0u64, 0u64, v1, v2, v3, false)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f5(v0: Adt1, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if if if if if if if if if if if if if f33(v2, v3.clone()) {
        let a165 = match v0.clone() {
            Adt1::C0(h164, _, _, _, _, _, _, _, _, _, _) => {
                h164
            }
        };
        let a166 = 0u64;
        if nat_eq(a165, a166) {
            true
        } else {
            let a168 = match v0.clone() {
                Adt1::C0(h167, _, _, _, _, _, _, _, _, _, _) => {
                    h167
                }
            };
            let a169 = 1u64;
            nat_eq(a168, a169)
        }
    } else {
        false
    } {
        let a171 = 0u64;
        let a172 = f20(match v3.clone() {
            Adt1::C0(h170, _, _, _, _, _, _, _, _, _, _) => {
                h170
            }
        });
        nat_lt(a171, a172)
    } else {
        false
    } {
        let a174 = 0u64;
        let a175 = f34(match v1.clone() {
            Adt1::C0(h173, _, _, _, _, _, _, _, _, _, _) => {
                h173
            }
        });
        nat_lt(a174, a175)
    } else {
        false
    } {
        let a178 = match v0.clone() {
            Adt1::C0(_, h176, _, _, _, _, _, _, _, _, _) => {
                h176
            }
        };
        let a179 = match v3.clone() {
            Adt1::C0(_, h177, _, _, _, _, _, _, _, _, _) => {
                h177
            }
        };
        if nat_eq(a178, a179) {
            let a185 = {
                let a182 = match v0.clone() {
                    Adt1::C0(_, h180, _, _, _, _, _, _, _, _, _) => {
                        h180
                    }
                };
                let a183 = f21(match v3.clone() {
                    Adt1::C0(h181, _, _, _, _, _, _, _, _, _, _) => {
                        h181
                    }
                });
                let a184 = 1u64;
                nat_rem(a182, a183, a184)
            };
            let a186 = 0u64;
            nat_eq(a185, a186)
        } else {
            false
        }
    } else {
        false
    } {
        let a189 = match v0.clone() {
            Adt1::C0(_, _, _, h187, _, _, _, _, _, _, _) => {
                h187
            }
        };
        let a190 = match v3.clone() {
            Adt1::C0(_, _, _, h188, _, _, _, _, _, _, _) => {
                h188
            }
        };
        nat_eq(a189, a190)
    } else {
        false
    } {
        let a193 = match v0.clone() {
            Adt1::C0(_, _, _, _, h191, _, _, _, _, _, _) => {
                h191
            }
        };
        let a194 = match v3.clone() {
            Adt1::C0(_, _, _, _, h192, _, _, _, _, _, _) => {
                h192
            }
        };
        nat_eq(a193, a194)
    } else {
        false
    } {
        let a197 = match v0.clone() {
            Adt1::C0(_, _, h195, _, _, _, _, _, _, _, _) => {
                h195
            }
        };
        let a198 = match v1.clone() {
            Adt1::C0(_, h196, _, _, _, _, _, _, _, _, _) => {
                h196
            }
        };
        nat_eq(a197, a198)
    } else {
        false
    } {
        let a200 = match v1.clone() {
            Adt1::C0(_, _, _, _, h199, _, _, _, _, _, _) => {
                h199
            }
        };
        let a201 = 1u64;
        nat_eq(a200, a201)
    } else {
        false
    } {
        let a203 = 0u64;
        let a204 = match v1.clone() {
            Adt1::C0(_, _, h202, _, _, _, _, _, _, _, _) => {
                h202
            }
        };
        nat_lt(a203, a204)
    } else {
        false
    } {
        let a206 = 0u64;
        let a207 = match v1.clone() {
            Adt1::C0(_, _, _, h205, _, _, _, _, _, _, _) => {
                h205
            }
        };
        nat_lt(a206, a207)
    } else {
        false
    } {
        let a213 = {
            let a210 = match v0.clone() {
                Adt1::C0(_, _, _, h208, _, _, _, _, _, _, _) => {
                    h208
                }
            };
            let a211 = match v1.clone() {
                Adt1::C0(_, _, h209, _, _, _, _, _, _, _, _) => {
                    h209
                }
            };
            let a212 = 0u64;
            nat_rem(a210, a211, a212)
        };
        let a214 = 0u64;
        nat_eq(a213, a214)
    } else {
        false
    } {
        let a220 = {
            let a217 = match v0.clone() {
                Adt1::C0(_, _, _, _, h215, _, _, _, _, _, _) => {
                    h215
                }
            };
            let a218 = match v1.clone() {
                Adt1::C0(_, _, _, h216, _, _, _, _, _, _, _) => {
                    h216
                }
            };
            let a219 = 0u64;
            nat_rem(a217, a218, a219)
        };
        let a221 = 0u64;
        nat_eq(a220, a221)
    } else {
        false
    } {
        f35(v0.clone())
    } else {
        false
    } {
        let a223 = match v3.clone() {
            Adt1::C0(_, h222, _, _, _, _, _, _, _, _, _) => {
                h222
            }
        };
        let a224 = 1u64;
        if nat_eq(a223, a224) {
            true
        } else {
            let a227 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h225, _, _, _, _, _) => {
                    h225
                }
            };
            let a228 = f20(match v3.clone() {
                Adt1::C0(h226, _, _, _, _, _, _, _, _, _, _) => {
                    h226
                }
            });
            nat_eq(a227, a228)
        }
    } else {
        false
    } {
        let v4: u64 = f31(v1.clone())?;
        let v5: u64 = {
            let a234 = match v0.clone() {
                Adt1::C0(_, _, h229, _, _, _, _, _, _, _, _) => {
                    h229
                }
            };
            let a235 = {
                let a232 = match v0.clone() {
                    Adt1::C0(_, _, _, h230, _, _, _, _, _, _, _) => {
                        h230
                    }
                };
                let a233 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h231, _, _, _, _, _, _) => {
                        h231
                    }
                };
                nat_mul(a232, a233)?
            };
            nat_mul(a234, a235)?
        };
        let v6: u64 = if f24(v0.clone()) {
            0u64
        } else {
            let a243 = {
                let a236 = 2u64;
                let a237 = v4;
                nat_mul(a236, a237)?
            };
            let a244 = {
                let a241 = v5;
                let a242 = {
                    let a239 = 1u64;
                    let a240 = f36(v0.clone(), match v3.clone() {
                        Adt1::C0(h238, _, _, _, _, _, _, _, _, _, _) => {
                            h238
                        }
                    })?;
                    nat_add(a239, a240)?
                };
                nat_mul(a241, a242)?
            };
            nat_add(a243, a244)?
        };
        Ok(Adt3::C2(Adt2::C0(v6, if f24(v0.clone()) {
            3u64
        } else {
            let a245 = 3u64;
            let a246 = v4;
            nat_add(a245, a246)?
        }, f37(match v3.clone() {
            Adt1::C0(h247, _, _, _, _, _, _, _, _, _, _) => {
                h247
            }
        }), 41u64, 0u64, 0u64, 0u64, v0, v1, v3, false)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f6(v0: Adt1, v1: Adt1, v2: Adt1) -> R<Adt3> {
    let v3: u64 = f21(match v2.clone() {
        Adt1::C0(h248, _, _, _, _, _, _, _, _, _, _) => {
            h248
        }
    });
    if if if if if if if f2(match v2.clone() {
        Adt1::C0(h249, _, _, _, _, _, _, _, _, _, _) => {
            h249
        }
    }) {
        f33(v1.clone(), v2.clone())
    } else {
        false
    } {
        let a251 = 0u64;
        let a252 = f38(match v0.clone() {
            Adt1::C0(h250, _, _, _, _, _, _, _, _, _, _) => {
                h250
            }
        });
        nat_lt(a251, a252)
    } else {
        false
    } {
        let a255 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h253, _, _, _, _, _) => {
                h253
            }
        };
        let a256 = f38(match v0.clone() {
            Adt1::C0(h254, _, _, _, _, _, _, _, _, _, _) => {
                h254
            }
        });
        nat_eq(a255, a256)
    } else {
        false
    } {
        let a261 = {
            let a258 = match v0.clone() {
                Adt1::C0(_, h257, _, _, _, _, _, _, _, _, _) => {
                    h257
                }
            };
            let a259 = v3;
            let a260 = 1u64;
            nat_rem(a258, a259, a260)
        };
        let a262 = 0u64;
        if nat_eq(a261, a262) {
            let a267 = {
                let a264 = match v2.clone() {
                    Adt1::C0(_, h263, _, _, _, _, _, _, _, _, _) => {
                        h263
                    }
                };
                let a265 = v3;
                let a266 = 1u64;
                nat_rem(a264, a265, a266)
            };
            let a268 = 0u64;
            nat_eq(a267, a268)
        } else {
            false
        }
    } else {
        false
    } {
        let a271 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h269, _, _, _, _, _) => {
                h269
            }
        };
        let a272 = f20(match v2.clone() {
            Adt1::C0(h270, _, _, _, _, _, _, _, _, _, _) => {
                h270
            }
        });
        nat_eq(a271, a272)
    } else {
        false
    } {
        let a273 = f31(v0.clone())?;
        let a274 = f31(v2.clone())?;
        nat_eq(a273, a274)
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a275 = f31(v0.clone())?;
            let a276 = v3;
            let a277 = 0u64;
            nat_quot(a275, a276, a277)
        }, 3u64, f37(match v2.clone() {
            Adt1::C0(h278, _, _, _, _, _, _, _, _, _, _) => {
                h278
            }
        }), 33u64, 0u64, 0u64, 0u64, v0, v1, v2, false)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f7(v0: u64, v1: u64, v2: Adt1, v3: Adt1) -> R<Adt3> {
    let a280 = match v2.clone() {
        Adt1::C0(h279, _, _, _, _, _, _, _, _, _, _) => {
            h279
        }
    };
    let a281 = 0u64;
    if if if if if if if if if if if nat_eq(a280, a281) {
        let a283 = match v3.clone() {
            Adt1::C0(h282, _, _, _, _, _, _, _, _, _, _) => {
                h282
            }
        };
        let a284 = 26u64;
        nat_eq(a283, a284)
    } else {
        false
    } {
        let a286 = match v2.clone() {
            Adt1::C0(_, h285, _, _, _, _, _, _, _, _, _) => {
                h285
            }
        };
        let a287 = 2147483647u64;
        nat_le(a286, a287)
    } else {
        false
    } {
        let a290 = match v3.clone() {
            Adt1::C0(_, h288, _, _, _, _, _, _, _, _, _) => {
                h288
            }
        };
        let a291 = match v2.clone() {
            Adt1::C0(_, h289, _, _, _, _, _, _, _, _, _) => {
                h289
            }
        };
        nat_le(a290, a291)
    } else {
        false
    } {
        let a294 = match v2.clone() {
            Adt1::C0(_, _, h292, _, _, _, _, _, _, _, _) => {
                h292
            }
        };
        let a295 = match v3.clone() {
            Adt1::C0(_, _, h293, _, _, _, _, _, _, _, _) => {
                h293
            }
        };
        nat_eq(a294, a295)
    } else {
        false
    } {
        let a298 = match v2.clone() {
            Adt1::C0(_, _, _, h296, _, _, _, _, _, _, _) => {
                h296
            }
        };
        let a299 = match v3.clone() {
            Adt1::C0(_, _, _, h297, _, _, _, _, _, _, _) => {
                h297
            }
        };
        nat_eq(a298, a299)
    } else {
        false
    } {
        let a302 = match v2.clone() {
            Adt1::C0(_, _, _, _, h300, _, _, _, _, _, _) => {
                h300
            }
        };
        let a303 = match v3.clone() {
            Adt1::C0(_, _, _, _, h301, _, _, _, _, _, _) => {
                h301
            }
        };
        nat_eq(a302, a303)
    } else {
        false
    } {
        let a304 = v0;
        let a305 = 67u64;
        if nat_eq(a304, a305) {
            true
        } else {
            let a306 = v1;
            let a307 = 1u64;
            if nat_le(a306, a307) {
                let a310 = match v3.clone() {
                    Adt1::C0(_, h308, _, _, _, _, _, _, _, _, _) => {
                        h308
                    }
                };
                let a311 = match v2.clone() {
                    Adt1::C0(_, h309, _, _, _, _, _, _, _, _, _) => {
                        h309
                    }
                };
                nat_eq(a310, a311)
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
            let a317 = match v3.clone() {
                Adt1::C0(_, _, h312, _, _, _, _, _, _, _, _) => {
                    h312
                }
            };
            let a318 = {
                let a315 = match v3.clone() {
                    Adt1::C0(_, _, _, h313, _, _, _, _, _, _, _) => {
                        h313
                    }
                };
                let a316 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h314, _, _, _, _, _, _) => {
                        h314
                    }
                };
                nat_mul(a315, a316)?
            };
            nat_mul(a317, a318)?
        }, 3u64, None::<u64>, v0, {
            let a319 = v0;
            let a320 = 67u64;
            if nat_eq(a319, a320) {
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
    let a321 = v0;
    let a322 = 6u64;
    nat_le(a321, a322)
}

pub fn f9(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1) -> R<Adt3> {
    let a323 = v2;
    let a324 = 1u64;
    if if if if if if if if if if if if if if if nat_le(a323, a324) {
        let a326 = 0u64;
        let a327 = f38(match v5.clone() {
            Adt1::C0(h325, _, _, _, _, _, _, _, _, _, _) => {
                h325
            }
        });
        nat_lt(a326, a327)
    } else {
        false
    } {
        let a330 = match v5.clone() {
            Adt1::C0(h328, _, _, _, _, _, _, _, _, _, _) => {
                h328
            }
        };
        let a331 = match v6.clone() {
            Adt1::C0(h329, _, _, _, _, _, _, _, _, _, _) => {
                h329
            }
        };
        nat_eq(a330, a331)
    } else {
        false
    } {
        let a334 = match v5.clone() {
            Adt1::C0(h332, _, _, _, _, _, _, _, _, _, _) => {
                h332
            }
        };
        let a335 = match v7.clone() {
            Adt1::C0(h333, _, _, _, _, _, _, _, _, _, _) => {
                h333
            }
        };
        nat_eq(a334, a335)
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
        let a338 = match v5.clone() {
            Adt1::C0(_, _, h336, _, _, _, _, _, _, _, _) => {
                h336
            }
        };
        let a339 = match v7.clone() {
            Adt1::C0(_, _, h337, _, _, _, _, _, _, _, _) => {
                h337
            }
        };
        nat_eq(a338, a339)
    } else {
        false
    } {
        let a342 = match v5.clone() {
            Adt1::C0(_, _, _, h340, _, _, _, _, _, _, _) => {
                h340
            }
        };
        let a343 = match v7.clone() {
            Adt1::C0(_, _, _, h341, _, _, _, _, _, _, _) => {
                h341
            }
        };
        nat_eq(a342, a343)
    } else {
        false
    } {
        let a346 = match v5.clone() {
            Adt1::C0(_, _, _, _, h344, _, _, _, _, _, _) => {
                h344
            }
        };
        let a347 = match v7.clone() {
            Adt1::C0(_, _, _, _, h345, _, _, _, _, _, _) => {
                h345
            }
        };
        nat_eq(a346, a347)
    } else {
        false
    } {
        let a354 = match v7.clone() {
            Adt1::C0(_, h348, _, _, _, _, _, _, _, _, _) => {
                h348
            }
        };
        let a355 = if v1 {
            match v5.clone() {
                Adt1::C0(_, h349, _, _, _, _, _, _, _, _, _) => {
                    h349
                }
            }
        } else {
            let a351 = match v5.clone() {
                Adt1::C0(_, h350, _, _, _, _, _, _, _, _, _) => {
                    h350
                }
            };
            let a352 = 2u64;
            let a353 = 0u64;
            nat_quot(a351, a352, a353)
        };
        nat_eq(a354, a355)
    } else {
        false
    } {
        let a357 = match v7.clone() {
            Adt1::C0(_, h356, _, _, _, _, _, _, _, _, _) => {
                h356
            }
        };
        let a358 = 2147483647u64;
        nat_le(a357, a358)
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
            let a361 = 1u64;
            let a362 = {
                let a359 = f42(v0);
                let a360 = f31(v7.clone())?;
                nat_mul(a359, a360)?
            };
            nat_add(a361, a362)?
        } else {
            let a363 = 4u64;
            let a364 = f31(v7.clone())?;
            nat_mul(a363, a364)?
        }, if f14(v0) {
            4u64
        } else {
            3u64
        }, None::<u64>, if v1 {
            98u64
        } else {
            let a365 = 99u64;
            let a366 = v2;
            nat_add(a365, a366)?
        }, v0, v3, v4, v5, v6, v7, false)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f10(v0: u64, v1: Adt2) -> R<Adt3> {
    let a368 = v0;
    let a369 = match v1.clone() {
        Adt2::C0(h367, _, _, _, _, _, _, _, _, _, _) => {
            h367
        }
    };
    if nat_lt(a368, a369) {
        let v2: u64 = {
            let a373 = v0;
            let a374 = {
                let m372 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h371, _, _, _) => {
                        h371
                    }
                };
                match m372 {
                    Adt1::C0(_, h370, _, _, _, _, _, _, _, _, _) => {
                        h370
                    }
                }
            };
            nat_mul(a373, a374)?
        };
        Ok(Adt3::C7(f43(v2, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h375, _, _, _) => {
                h375
            }
        })?, f44(v2, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h376, _, _, _) => {
                h376
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h377, _, _) => {
                h377
            }
        })?, f43(v2, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h378, _) => {
                h378
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f11(v0: u64, v1: Adt2) -> R<Adt3> {
    let a380 = v0;
    let a381 = match v1.clone() {
        Adt2::C0(h379, _, _, _, _, _, _, _, _, _, _) => {
            h379
        }
    };
    if nat_lt(a380, a381) {
        let v2: Adt1 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h382, _) => {
                h382
            }
        };
        let v3: u64 = f21(match v2.clone() {
            Adt1::C0(h383, _, _, _, _, _, _, _, _, _, _) => {
                h383
            }
        });
        let v4: u64 = {
            let a385 = match v2.clone() {
                Adt1::C0(_, h384, _, _, _, _, _, _, _, _, _) => {
                    h384
                }
            };
            let a386 = v3;
            let a387 = 0u64;
            nat_quot(a385, a386, a387)
        };
        let v5: u64 = {
            let a388 = v0;
            let a389 = v4;
            let a390 = 0u64;
            nat_quot(a388, a389, a390)
        };
        let v6: u64 = {
            let a392 = v5;
            let a393 = match v2.clone() {
                Adt1::C0(_, _, h391, _, _, _, _, _, _, _, _) => {
                    h391
                }
            };
            let a394 = 0u64;
            nat_quot(a392, a393, a394)
        };
        Ok(Adt3::C6(f43({
            let a395 = v0;
            let a396 = v3;
            nat_mul(a395, a396)?
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, h397, _, _, _) => {
                h397
            }
        })?, f45({
            let a398 = v0;
            let a399 = v4;
            let a400 = 0u64;
            nat_rem(a398, a399, a400)
        }, {
            let a402 = v5;
            let a403 = match v2.clone() {
                Adt1::C0(_, _, h401, _, _, _, _, _, _, _, _) => {
                    h401
                }
            };
            let a404 = 0u64;
            nat_rem(a402, a403, a404)
        }, {
            let a406 = v6;
            let a407 = match v2.clone() {
                Adt1::C0(_, _, _, h405, _, _, _, _, _, _, _) => {
                    h405
                }
            };
            let a408 = 0u64;
            nat_rem(a406, a407, a408)
        }, {
            let a410 = v6;
            let a411 = match v2.clone() {
                Adt1::C0(_, _, _, h409, _, _, _, _, _, _, _) => {
                    h409
                }
            };
            let a412 = 0u64;
            nat_quot(a410, a411, a412)
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, h413, _, _, _, _, _) => {
                h413
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h414, _, _, _, _) => {
                h414
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h415, _, _, _) => {
                h415
            }
        }, match v2 {
            Adt1::C0(_, _, _, _, _, _, _, _, h416, _, _) => {
                h416
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f12(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h417, _, _, _) => {
            h417
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h418, _) => {
            h418
        }
    };
    let a420 = v0;
    let a421 = match v1.clone() {
        Adt2::C0(h419, _, _, _, _, _, _, _, _, _, _) => {
            h419
        }
    };
    if nat_lt(a420, a421) {
        Ok(Adt3::C5({
            let a423 = v0;
            let a424 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, _, h422, _, _, _, _) => {
                    h422
                }
            };
            nat_mul(a423, a424)?
        }, {
            let a426 = v0;
            let a427 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h425, _, _, _, _) => {
                    h425
                }
            };
            nat_mul(a426, a427)?
        }, match v2 {
            Adt1::C0(_, h428, _, _, _, _, _, _, _, _, _) => {
                h428
            }
        }, match v3 {
            Adt1::C0(_, h429, _, _, _, _, _, _, _, _, _) => {
                h429
            }
        }, {
            let a431 = match v1 {
                Adt2::C0(_, _, _, _, h430, _, _, _, _, _, _) => {
                    h430
                }
            };
            let a432 = 1u64;
            nat_eq(a431, a432)
        }))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f13(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let a434 = v0;
    let a435 = match v1.clone() {
        Adt2::C0(h433, _, _, _, _, _, _, _, _, _, _) => {
            h433
        }
    };
    if nat_lt(a434, a435) {
        let v3: u64 = {
            let a437 = 2u64;
            let a438 = f31(match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h436, _, _) => {
                    h436
                }
            })?;
            nat_mul(a437, a438)?
        };
        let a439 = v0;
        let a440 = v3;
        if nat_lt(a439, a440) {
            f46(v0, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h441, _, _) => {
                    h441
                }
            }, {
                let m444 = match v1 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h443, _) => {
                        h443
                    }
                };
                match m444 {
                    Adt1::C0(_, _, h442, _, _, _, _, _, _, _, _) => {
                        h442
                    }
                }
            }, v2)
        } else {
            f47({
                let a445 = v0;
                let a446 = v3;
                nat_sub(a445, a446)
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h447, _, _, _) => {
                    h447
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h448, _, _) => {
                    h448
                }
            }, match v1 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h449, _) => {
                    h449
                }
            }, v2)
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f14(v0: u64) -> bool {
    let a450 = v0;
    let a451 = 3u64;
    if nat_eq(a450, a451) {
        true
    } else {
        let a452 = v0;
        let a453 = 6u64;
        nat_eq(a452, a453)
    }
}

pub fn f15(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h454, _, _, _, _) => {
            h454
        }
    };
    let a456 = v0;
    let a457 = match v1.clone() {
        Adt2::C0(h455, _, _, _, _, _, _, _, _, _, _) => {
            h455
        }
    };
    if nat_lt(a456, a457) {
        let a458 = v0;
        let a459 = 0u64;
        if nat_eq(a458, a459) {
            Ok(Adt3::C3(1u64, 6u64, 0u64, 0u64, 2u64, v3, 2u64, 0u64, 0u64, 3u64, 4u64, 4u64))
        } else {
            let v4: u64 = {
                let a460 = v0;
                let a461 = 1u64;
                nat_sub(a460, a461)
            };
            let v5: u64 = {
                let a463 = v4;
                let a464 = f42(match v1.clone() {
                    Adt2::C0(_, _, _, _, h462, _, _, _, _, _, _) => {
                        h462
                    }
                });
                let a465 = 0u64;
                nat_rem(a463, a464, a465)
            };
            let v6: u64 = {
                let a470 = 4u64;
                let a471 = {
                    let a467 = v4;
                    let a468 = f42(match v1.clone() {
                        Adt2::C0(_, _, _, _, h466, _, _, _, _, _, _) => {
                            h466
                        }
                    });
                    let a469 = 0u64;
                    nat_quot(a467, a468, a469)
                };
                nat_mul(a470, a471)?
            };
            let a472 = v5;
            let a473 = 2u64;
            if nat_lt(a472, a473) {
                f16({
                    let a474 = v6;
                    let a475 = v5;
                    nat_add(a474, a475)?
                }, v1)
            } else {
                let a476 = v5;
                let a477 = 2u64;
                if nat_eq(a476, a477) {
                    Ok(f48(0u64, 2u64, v3))
                } else {
                    let a478 = v5;
                    let a479 = 3u64;
                    if nat_eq(a478, a479) {
                        Ok(f49(0u64, 2u64, v3, {
                            let a480 = v2;
                            let a481 = 2u64;
                            nat_eq(a480, a481)
                        }))
                    } else {
                        let a482 = v5;
                        let a483 = 4u64;
                        if nat_eq(a482, a483) {
                            Ok(f48(1u64, 1u64, 3u64))
                        } else {
                            let a484 = v5;
                            let a485 = 5u64;
                            if nat_eq(a484, a485) {
                                Ok(f49(1u64, 1u64, 3u64, {
                                    let a486 = v2;
                                    let a487 = 0u64;
                                    nat_eq(a486, a487)
                                }))
                            } else {
                                let a488 = v5;
                                let a489 = 6u64;
                                if nat_eq(a488, a489) {
                                    Ok(f48(1u64, 2u64, v3))
                                } else {
                                    let a490 = v5;
                                    let a491 = 7u64;
                                    if nat_eq(a490, a491) {
                                        Ok(f49(1u64, 2u64, v3, {
                                            let a492 = v2;
                                            let a493 = 2u64;
                                            nat_eq(a492, a493)
                                        }))
                                    } else {
                                        let a495 = match v1.clone() {
                                            Adt2::C0(_, _, _, _, h494, _, _, _, _, _, _) => {
                                                h494
                                            }
                                        };
                                        let a496 = 3u64;
                                        if nat_eq(a495, a496) {
                                            let a497 = v5;
                                            let a498 = 15u64;
                                            if nat_eq(a497, a498) {
                                                f16({
                                                    let a499 = v6;
                                                    let a500 = 3u64;
                                                    nat_add(a499, a500)?
                                                }, v1)
                                            } else {
                                                Ok(f50(v5, match v1 {
                                                    Adt2::C0(_, _, _, _, _, h501, _, _, _, _, _) => {
                                                        h501
                                                    }
                                                }))
                                            }
                                        } else {
                                            let a502 = v5;
                                            let a503 = 8u64;
                                            if nat_eq(a502, a503) {
                                                Ok(Adt3::C4(2u64, {
                                                    let m506 = match v1 {
                                                        Adt2::C0(_, _, _, _, _, _, _, _, _, h505, _) => {
                                                            h505
                                                        }
                                                    };
                                                    match m506 {
                                                        Adt1::C0(h504, _, _, _, _, _, _, _, _, _, _) => {
                                                            h504
                                                        }
                                                    }
                                                }, 0u64, 1u64, 0u64))
                                            } else {
                                                f16({
                                                    let a507 = v6;
                                                    let a508 = 3u64;
                                                    nat_add(a507, a508)?
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
        Adt2::C0(_, _, _, _, _, _, _, h509, _, _, _) => {
            h509
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h510, _, _) => {
            h510
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h511, _) => {
            h511
        }
    };
    let v5: u64 = match v1.clone() {
        Adt2::C0(_, _, _, h512, _, _, _, _, _, _, _) => {
            h512
        }
    };
    let v6: u64 = {
        let a513 = v0;
        let a514 = 4u64;
        let a515 = 0u64;
        nat_rem(a513, a514, a515)
    };
    let v7: u64 = {
        let a516 = v0;
        let a517 = 4u64;
        let a518 = 0u64;
        nat_quot(a516, a517, a518)
    };
    let v8: u64 = {
        let a520 = v7;
        let a521 = match v4.clone() {
            Adt1::C0(_, h519, _, _, _, _, _, _, _, _, _) => {
                h519
            }
        };
        let a522 = 0u64;
        nat_quot(a520, a521, a522)
    };
    let v9: u64 = f38(match v4.clone() {
        Adt1::C0(h523, _, _, _, _, _, _, _, _, _, _) => {
            h523
        }
    });
    let v10: u64 = {
        let a528 = {
            let a525 = v7;
            let a526 = match v4.clone() {
                Adt1::C0(_, h524, _, _, _, _, _, _, _, _, _) => {
                    h524
                }
            };
            let a527 = 0u64;
            nat_rem(a525, a526, a527)
        };
        let a529 = v9;
        nat_mul(a528, a529)?
    };
    let v11: u64 = {
        let a531 = match v4.clone() {
            Adt1::C0(_, h530, _, _, _, _, _, _, _, _, _) => {
                h530
            }
        };
        let a532 = v9;
        nat_mul(a531, a532)?
    };
    let a534 = v0;
    let a535 = match v1.clone() {
        Adt2::C0(h533, _, _, _, _, _, _, _, _, _, _) => {
            h533
        }
    };
    if nat_lt(a534, a535) {
        let a536 = v6;
        let a537 = 0u64;
        if nat_eq(a536, a537) {
            Ok(Adt3::C3(0u64, 0u64, match v2.clone() {
                Adt1::C0(h538, _, _, _, _, _, _, _, _, _, _) => {
                    h538
                }
            }, 0u64, 0u64, {
                let a546 = {
                    let a542 = {
                        let a540 = v8;
                        let a541 = match v2 {
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
                    let a545 = 100u64;
                    if nat_eq(a544, a545) {
                        v11
                    } else {
                        0u64
                    }
                };
                nat_add(a546, a547)?
            }, 2u64, 0u64, 0u64, 0u64, v9, 4u64))
        } else {
            let a548 = v6;
            let a549 = 1u64;
            if nat_eq(a548, a549) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h550, _, _, _, _, _, _, _, _, _, _) => {
                        h550
                    }
                }, 0u64, {
                    let a551 = v5;
                    let a552 = 98u64;
                    if nat_eq(a551, a552) {
                        4u64
                    } else {
                        0u64
                    }
                }, {
                    let a560 = {
                        let a556 = {
                            let a554 = v8;
                            let a555 = match v3 {
                                Adt1::C0(_, _, _, _, _, _, h553, _, _, _, _) => {
                                    h553
                                }
                            };
                            nat_mul(a554, a555)?
                        };
                        let a557 = v10;
                        nat_add(a556, a557)?
                    };
                    let a561 = {
                        let a558 = v5;
                        let a559 = 99u64;
                        if nat_eq(a558, a559) {
                            v11
                        } else {
                            0u64
                        }
                    };
                    nat_add(a560, a561)?
                }, 2u64, 0u64, 0u64, 1u64, v9, 4u64))
            } else {
                let a562 = v6;
                let a563 = 2u64;
                if nat_eq(a562, a563) {
                    Ok(Adt3::C4(match v1 {
                        Adt2::C0(_, _, _, _, h564, _, _, _, _, _, _) => {
                            h564
                        }
                    }, match v4 {
                        Adt1::C0(h565, _, _, _, _, _, _, _, _, _, _) => {
                            h565
                        }
                    }, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt3::C3(0u64, 0u64, 0u64, match v4.clone() {
                        Adt1::C0(h566, _, _, _, _, _, _, _, _, _, _) => {
                            h566
                        }
                    }, 1u64, 0u64, 2u64, 0u64, 3u64, {
                        let a570 = {
                            let a568 = v8;
                            let a569 = match v4 {
                                Adt1::C0(_, _, _, _, _, _, h567, _, _, _, _) => {
                                    h567
                                }
                            };
                            nat_mul(a568, a569)?
                        };
                        let a571 = v10;
                        nat_add(a570, a571)?
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
        Adt2::C0(_, _, _, _, _, _, _, h572, _, _, _) => {
            h572
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h573, _, _) => {
            h573
        }
    };
    let v5: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h574, _) => {
            h574
        }
    };
    let v6: u64 = {
        let a576 = match v1.clone() {
            Adt2::C0(_, h575, _, _, _, _, _, _, _, _, _) => {
                h575
            }
        };
        let a577 = 3u64;
        if nat_eq(a576, a577) {
            0u64
        } else {
            let a578 = f31(v3.clone())?;
            let a579 = 2u64;
            nat_mul(a578, a579)?
        }
    };
    let a581 = match v1.clone() {
        Adt2::C0(h580, _, _, _, _, _, _, _, _, _, _) => {
            h580
        }
    };
    let a582 = v0;
    if nat_le(a581, a582) {
        Ok(Adt3::C1)
    } else {
        let a583 = v0;
        let a584 = v6;
        if nat_lt(a583, a584) {
            let v7: u64 = {
                let a585 = v0;
                let a586 = 2u64;
                let a587 = 0u64;
                nat_quot(a585, a586, a587)
            };
            let v8: u64 = {
                let a592 = 3u64;
                let a593 = {
                    let a589 = f43(v7, v5.clone())?;
                    let a590 = f26(match v5 {
                        Adt1::C0(h588, _, _, _, _, _, _, _, _, _, _) => {
                            h588
                        }
                    });
                    let a591 = 0u64;
                    nat_quot(a589, a590, a591)
                };
                nat_add(a592, a593)?
            };
            let a597 = {
                let a594 = v0;
                let a595 = 2u64;
                let a596 = 0u64;
                nat_rem(a594, a595, a596)
            };
            let a598 = 0u64;
            if nat_eq(a597, a598) {
                Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, v8, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
            } else {
                let a599 = v2;
                let a600 = 0u64;
                if nat_eq(a599, a600) {
                    Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 2u64, 1u64, 2u64, 0u64, 0u64, v8, 8u64, 8u64))
                } else {
                    Ok(Adt3::C1)
                }
            }
        } else {
            let v9: u64 = {
                let a601 = v0;
                let a602 = v6;
                nat_sub(a601, a602)
            };
            let v10: u64 = f32(match v3.clone() {
                Adt1::C0(h603, _, _, _, _, _, _, _, _, _, _) => {
                    h603
                }
            }, match v4.clone() {
                Adt1::C0(h604, _, _, _, _, _, _, _, _, _, _) => {
                    h604
                }
            }, match v5.clone() {
                Adt1::C0(h605, _, _, _, _, _, _, _, _, _, _) => {
                    h605
                }
            })?;
            let v11: u64 = {
                let a606 = v9;
                let a607 = v10;
                let a608 = 0u64;
                nat_quot(a606, a607, a608)
            };
            let v12: u64 = {
                let a609 = v9;
                let a610 = v10;
                let a611 = 0u64;
                nat_rem(a609, a610, a611)
            };
            let v13: u64 = {
                let a613 = match v3.clone() {
                    Adt1::C0(h612, _, _, _, _, _, _, _, _, _, _) => {
                        h612
                    }
                };
                let a614 = 0u64;
                if nat_eq(a613, a614) {
                    0u64
                } else {
                    1u64
                }
            };
            let v14: u64 = {
                let a616 = match v4.clone() {
                    Adt1::C0(h615, _, _, _, _, _, _, _, _, _, _) => {
                        h615
                    }
                };
                let a617 = 0u64;
                if nat_eq(a616, a617) {
                    0u64
                } else {
                    1u64
                }
            };
            let a618 = v12;
            let a619 = v13;
            if nat_lt(a618, a619) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h620, _, _, _, _, _, _, _, _, _, _) => {
                        h620
                    }
                }, 0u64, 0u64, f43(v11, v3.clone())?, 2u64, 0u64, 0u64, 0u64, f26(match v3 {
                    Adt1::C0(h621, _, _, _, _, _, _, _, _, _, _) => {
                        h621
                    }
                }), 4u64))
            } else {
                let a624 = v12;
                let a625 = {
                    let a622 = v13;
                    let a623 = v14;
                    nat_add(a622, a623)?
                };
                if nat_lt(a624, a625) {
                    Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                        Adt1::C0(h626, _, _, _, _, _, _, _, _, _, _) => {
                            h626
                        }
                    }, 0u64, 4u64, f44(v11, v3, v4.clone())?, 2u64, 0u64, 0u64, 1u64, f26(match v4 {
                        Adt1::C0(h627, _, _, _, _, _, _, _, _, _, _) => {
                            h627
                        }
                    }), 4u64))
                } else {
                    let a630 = v12;
                    let a631 = {
                        let a628 = v13;
                        let a629 = v14;
                        nat_add(a628, a629)?
                    };
                    if nat_eq(a630, a631) {
                        Ok(Adt3::C3(1u64, match v1 {
                            Adt2::C0(_, _, _, h632, _, _, _, _, _, _, _) => {
                                h632
                            }
                        }, 0u64, 0u64, {
                            let a633 = v13;
                            let a634 = 0u64;
                            if nat_eq(a633, a634) {
                                0u64
                            } else {
                                1u64
                            }
                        }, {
                            let a635 = v13;
                            let a636 = 0u64;
                            if nat_eq(a635, a636) {
                                f43(v11, v3.clone())?
                            } else {
                                0u64
                            }
                        }, {
                            let a637 = v14;
                            let a638 = 0u64;
                            if nat_eq(a637, a638) {
                                4u64
                            } else {
                                1u64
                            }
                        }, {
                            let a639 = v14;
                            let a640 = 0u64;
                            if nat_eq(a639, a640) {
                                f44(v11, v3, v4)?
                            } else {
                                1u64
                            }
                        }, {
                            let a642 = match v5.clone() {
                                Adt1::C0(h641, _, _, _, _, _, _, _, _, _, _) => {
                                    h641
                                }
                            };
                            let a643 = 0u64;
                            if nat_eq(a642, a643) {
                                3u64
                            } else {
                                0u64
                            }
                        }, {
                            let a645 = match v5.clone() {
                                Adt1::C0(h644, _, _, _, _, _, _, _, _, _, _) => {
                                    h644
                                }
                            };
                            let a646 = 0u64;
                            if nat_eq(a645, a646) {
                                f43(v11, v5)?
                            } else {
                                0u64
                            }
                        }, 4u64, 4u64))
                    } else {
                        Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                            Adt1::C0(h647, _, _, _, _, _, _, _, _, _, _) => {
                                h647
                            }
                        }, 1u64, 0u64, 2u64, 0u64, 3u64, f43(v11, v5.clone())?, 4u64, f26(match v5 {
                            Adt1::C0(h648, _, _, _, _, _, _, _, _, _, _) => {
                                h648
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
        let a654 = {
            let a651 = match v0.clone() {
                Adt1::C0(_, h649, _, _, _, _, _, _, _, _, _) => {
                    h649
                }
            };
            let a652 = match v1.clone() {
                Adt1::C0(_, h650, _, _, _, _, _, _, _, _, _) => {
                    h650
                }
            };
            let a653 = 1u64;
            nat_rem(a651, a652, a653)
        };
        let a655 = 0u64;
        if if if nat_eq(a654, a655) {
            let a661 = {
                let a658 = match v0.clone() {
                    Adt1::C0(_, _, h656, _, _, _, _, _, _, _, _) => {
                        h656
                    }
                };
                let a659 = match v1.clone() {
                    Adt1::C0(_, _, h657, _, _, _, _, _, _, _, _) => {
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
        } {
            let a668 = {
                let a665 = match v0.clone() {
                    Adt1::C0(_, _, _, h663, _, _, _, _, _, _, _) => {
                        h663
                    }
                };
                let a666 = match v1.clone() {
                    Adt1::C0(_, _, _, h664, _, _, _, _, _, _, _) => {
                        h664
                    }
                };
                let a667 = 1u64;
                nat_rem(a665, a666, a667)
            };
            let a669 = 0u64;
            nat_eq(a668, a669)
        } else {
            false
        } {
            let a675 = {
                let a672 = match v0 {
                    Adt1::C0(_, _, _, _, h670, _, _, _, _, _, _) => {
                        h670
                    }
                };
                let a673 = match v1 {
                    Adt1::C0(_, _, _, _, h671, _, _, _, _, _, _) => {
                        h671
                    }
                };
                let a674 = 1u64;
                nat_rem(a672, a673, a674)
            };
            let a676 = 0u64;
            nat_eq(a675, a676)
        } else {
            false
        }
    }
}

pub fn f19(v0: Adt1, v1: Adt1) -> bool {
    let a679 = match v0.clone() {
        Adt1::C0(_, h677, _, _, _, _, _, _, _, _, _) => {
            h677
        }
    };
    let a680 = match v1.clone() {
        Adt1::C0(_, h678, _, _, _, _, _, _, _, _, _) => {
            h678
        }
    };
    if if if nat_eq(a679, a680) {
        let a683 = match v0.clone() {
            Adt1::C0(_, _, h681, _, _, _, _, _, _, _, _) => {
                h681
            }
        };
        let a684 = match v1.clone() {
            Adt1::C0(_, _, h682, _, _, _, _, _, _, _, _) => {
                h682
            }
        };
        nat_eq(a683, a684)
    } else {
        false
    } {
        let a687 = match v0.clone() {
            Adt1::C0(_, _, _, h685, _, _, _, _, _, _, _) => {
                h685
            }
        };
        let a688 = match v1.clone() {
            Adt1::C0(_, _, _, h686, _, _, _, _, _, _, _) => {
                h686
            }
        };
        nat_eq(a687, a688)
    } else {
        false
    } {
        let a691 = match v0 {
            Adt1::C0(_, _, _, _, h689, _, _, _, _, _, _) => {
                h689
            }
        };
        let a692 = match v1 {
            Adt1::C0(_, _, _, _, h690, _, _, _, _, _, _) => {
                h690
            }
        };
        nat_eq(a691, a692)
    } else {
        false
    }
}

pub fn f20(v0: u64) -> u64 {
    if f2(v0) {
        let m693 = f51(v0);
        match m693 {
            Adt4::C0 => {
                0u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, _, h694) => {
                        h694
                    }
                }
            }
        }
    } else {
        let a695 = v0;
        let a696 = 26u64;
        if nat_eq(a695, a696) {
            4u64
        } else {
            f26(v0)
        }
    }
}

pub fn f21(v0: u64) -> u64 {
    if f2(v0) {
        let m697 = f51(v0);
        match m697 {
            Adt4::C0 => {
                1u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, h698, _) => {
                        h698
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
        Adt1::C0(_, h699, _, _, _, _, _, _, _, _, _) => {
            h699
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h700, _, _, _, _, _, _, _, _) => {
            h700
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h701, _, _, _, _, _, _, _) => {
            h701
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, h702, _, _, _, _, _, _) => {
            h702
        }
    })
}

pub fn f25(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<bool> {
    let a704 = match v1.clone() {
        Adt1::C0(h703, _, _, _, _, _, _, _, _, _, _) => {
            h703
        }
    };
    let a705 = 43u64;
    if nat_eq(a704, a705) {
        let a706 = v0;
        let a707 = 0u64;
        if if if if if if nat_eq(a706, a707) {
            let a709 = match v2.clone() {
                Adt1::C0(h708, _, _, _, _, _, _, _, _, _, _) => {
                    h708
                }
            };
            let a710 = 0u64;
            nat_eq(a709, a710)
        } else {
            false
        } {
            let a712 = match v3.clone() {
                Adt1::C0(h711, _, _, _, _, _, _, _, _, _, _) => {
                    h711
                }
            };
            let a713 = 0u64;
            if nat_eq(a712, a713) {
                true
            } else {
                let a715 = match v3.clone() {
                    Adt1::C0(h714, _, _, _, _, _, _, _, _, _, _) => {
                        h714
                    }
                };
                let a716 = 43u64;
                nat_eq(a715, a716)
            }
        } else {
            false
        } {
            let a718 = match v3.clone() {
                Adt1::C0(h717, _, _, _, _, _, _, _, _, _, _) => {
                    h717
                }
            };
            let a719 = 43u64;
            if nat_eq(a718, a719) {
                f18(v1.clone(), v2.clone())
            } else {
                f19(v1.clone(), v2.clone())
            }
        } else {
            false
        } {
            let a721 = match v2 {
                Adt1::C0(_, _, _, _, _, h720, _, _, _, _, _) => {
                    h720
                }
            };
            let a722 = 4u64;
            nat_eq(a721, a722)
        } else {
            false
        } {
            let a725 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h723, _, _, _, _, _) => {
                    h723
                }
            };
            let a726 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h724, _, _, _, _) => {
                    h724
                }
            };
            if nat_le(a725, a726) {
                let a729 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, h727, _, _, _, _) => {
                        h727
                    }
                };
                let a730 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h728, _, _, _) => {
                        h728
                    }
                };
                if nat_le(a729, a730) {
                    let a733 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, h731, _, _, _) => {
                            h731
                        }
                    };
                    let a734 = match v3 {
                        Adt1::C0(_, _, _, _, _, _, _, _, h732, _, _) => {
                            h732
                        }
                    };
                    nat_le(a733, a734)
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        } {
            let a742 = {
                let a740 = match v1.clone() {
                    Adt1::C0(_, _, h735, _, _, _, _, _, _, _, _) => {
                        h735
                    }
                };
                let a741 = {
                    let a738 = match v1.clone() {
                        Adt1::C0(_, _, _, h736, _, _, _, _, _, _, _) => {
                            h736
                        }
                    };
                    let a739 = match v1 {
                        Adt1::C0(_, _, _, _, h737, _, _, _, _, _, _) => {
                            h737
                        }
                    };
                    nat_mul(a738, a739)?
                };
                nat_mul(a740, a741)?
            };
            let a743 = 2147483647u64;
            Ok(nat_le(a742, a743))
        } else {
            Ok(false)
        }
    } else {
        if f18(v1.clone(), v2.clone()) {
            Ok(f55(match v1 {
                Adt1::C0(h744, _, _, _, _, _, _, _, _, _, _) => {
                    h744
                }
            }, match v2 {
                Adt1::C0(h745, _, _, _, _, _, _, _, _, _, _) => {
                    h745
                }
            }, match v3 {
                Adt1::C0(h746, _, _, _, _, _, _, _, _, _, _) => {
                    h746
                }
            }))
        } else {
            Ok(false)
        }
    }
}

pub fn f26(v0: u64) -> u64 {
    let a747 = v0;
    let a748 = 43u64;
    if nat_eq(a747, a748) {
        1u64
    } else {
        f38(v0)
    }
}

pub fn f27(v0: Adt1) -> bool {
    let v1: u64 = f26(match v0.clone() {
        Adt1::C0(h749, _, _, _, _, _, _, _, _, _, _) => {
            h749
        }
    });
    let a754 = {
        let a751 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h750) => {
                h750
            }
        };
        let a752 = v1;
        let a753 = 1u64;
        nat_rem(a751, a752, a753)
    };
    let a755 = 0u64;
    if if nat_eq(a754, a755) {
        f56(match v0.clone() {
            Adt1::C0(_, h756, _, _, _, _, _, _, _, _, _) => {
                h756
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h757, _, _, _, _, _) => {
                h757
            }
        }, v1)
    } else {
        false
    } {
        f57(match v0.clone() {
            Adt1::C0(_, _, h758, _, _, _, _, _, _, _, _) => {
                h758
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h759, _, _, _, _, _, _, _) => {
                h759
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h760, _, _, _, _, _, _) => {
                h760
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h761, _, _, _, _) => {
                h761
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h762, _, _, _) => {
                h762
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h763, _, _) => {
                h763
            }
        }, v1)
    } else {
        false
    }
}

pub fn f28(v0: Adt1, v1: Adt1) -> R<bool> {
    let a771 = {
        let a769 = match v0.clone() {
            Adt1::C0(_, _, h764, _, _, _, _, _, _, _, _) => {
                h764
            }
        };
        let a770 = {
            let a767 = match v0.clone() {
                Adt1::C0(_, _, _, h765, _, _, _, _, _, _, _) => {
                    h765
                }
            };
            let a768 = match v0.clone() {
                Adt1::C0(_, _, _, _, h766, _, _, _, _, _, _) => {
                    h766
                }
            };
            nat_mul(a767, a768)?
        };
        nat_mul(a769, a770)?
    };
    let a772 = 9223372036854775807u64;
    if nat_le(a771, a772) {
        if f35(v1.clone()) {
            let a774 = match v1 {
                Adt1::C0(_, h773, _, _, _, _, _, _, _, _, _) => {
                    h773
                }
            };
            let a775 = 2147483647u64;
            Ok(nat_le(a774, a775))
        } else {
            let a777 = match v0 {
                Adt1::C0(_, h776, _, _, _, _, _, _, _, _, _) => {
                    h776
                }
            };
            let a778 = 2147483647u64;
            Ok(nat_le(a777, a778))
        }
    } else {
        Ok(false)
    }
}

pub fn f29(v0: Adt1) -> R<bool> {
    f23({
        let a781 = match v0.clone() {
            Adt1::C0(_, h779, _, _, _, _, _, _, _, _, _) => {
                h779
            }
        };
        let a782 = f26(match v0.clone() {
            Adt1::C0(h780, _, _, _, _, _, _, _, _, _, _) => {
                h780
            }
        });
        nat_mul(a781, a782)?
    }, match v0.clone() {
        Adt1::C0(_, _, h783, _, _, _, _, _, _, _, _) => {
            h783
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h784, _, _, _, _, _, _, _) => {
            h784
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h785, _, _, _, _, _, _) => {
            h785
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h786, _, _, _, _) => {
            h786
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h787, _, _, _) => {
            h787
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h788, _, _) => {
            h788
        }
    })
}

pub fn f30(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a795 = {
        let a789 = v0;
        let a790 = v2;
        nat_mul(a789, a790)?
    };
    let a796 = {
        let a791 = v1;
        let a792 = 0u64;
        if nat_eq(a791, a792) {
            0u64
        } else {
            let a793 = v0;
            let a794 = 2u64;
            nat_mul(a793, a794)?
        }
    };
    nat_add(a795, a796)
}

pub fn f31(v0: Adt1) -> R<u64> {
    let a805 = {
        let a799 = match v0.clone() {
            Adt1::C0(_, h797, _, _, _, _, _, _, _, _, _) => {
                h797
            }
        };
        let a800 = match v0.clone() {
            Adt1::C0(_, _, h798, _, _, _, _, _, _, _, _) => {
                h798
            }
        };
        nat_mul(a799, a800)?
    };
    let a806 = {
        let a803 = match v0.clone() {
            Adt1::C0(_, _, _, h801, _, _, _, _, _, _, _) => {
                h801
            }
        };
        let a804 = match v0 {
            Adt1::C0(_, _, _, _, h802, _, _, _, _, _, _) => {
                h802
            }
        };
        nat_mul(a803, a804)?
    };
    nat_mul(a805, a806)
}

pub fn f32(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a817 = {
        let a813 = {
            let a809 = 1u64;
            let a810 = {
                let a807 = v0;
                let a808 = 0u64;
                if nat_eq(a807, a808) {
                    0u64
                } else {
                    1u64
                }
            };
            nat_add(a809, a810)?
        };
        let a814 = {
            let a811 = v1;
            let a812 = 0u64;
            if nat_eq(a811, a812) {
                0u64
            } else {
                1u64
            }
        };
        nat_add(a813, a814)?
    };
    let a818 = {
        let a815 = v2;
        let a816 = 0u64;
        if nat_eq(a815, a816) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a817, a818)
}

pub fn f33(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f19(v0.clone(), v1.clone()) {
        let a821 = match v0.clone() {
            Adt1::C0(h819, _, _, _, _, _, _, _, _, _, _) => {
                h819
            }
        };
        let a822 = match v1.clone() {
            Adt1::C0(h820, _, _, _, _, _, _, _, _, _, _) => {
                h820
            }
        };
        nat_eq(a821, a822)
    } else {
        false
    } {
        let a825 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h823, _, _, _, _, _) => {
                h823
            }
        };
        let a826 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h824, _, _, _, _, _) => {
                h824
            }
        };
        nat_eq(a825, a826)
    } else {
        false
    } {
        let a829 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h827, _, _, _, _) => {
                h827
            }
        };
        let a830 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h828, _, _, _, _) => {
                h828
            }
        };
        nat_eq(a829, a830)
    } else {
        false
    } {
        let a833 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h831, _, _, _) => {
                h831
            }
        };
        let a834 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h832, _, _, _) => {
                h832
            }
        };
        nat_eq(a833, a834)
    } else {
        false
    } {
        let a837 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h835, _, _) => {
                h835
            }
        };
        let a838 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h836, _, _) => {
                h836
            }
        };
        nat_eq(a837, a838)
    } else {
        false
    } {
        let a840 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h839) => {
                h839
            }
        };
        let a841 = 0u64;
        if if nat_eq(a840, a841) {
            true
        } else {
            let a843 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h842) => {
                    h842
                }
            };
            let a844 = 0u64;
            nat_eq(a843, a844)
        } {
            true
        } else {
            let a847 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h845) => {
                    h845
                }
            };
            let a848 = match v1 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h846) => {
                    h846
                }
            };
            nat_eq(a847, a848)
        }
    } else {
        false
    }
}

pub fn f34(v0: u64) -> u64 {
    let a849 = v0;
    let a850 = 26u64;
    if nat_eq(a849, a850) {
        4u64
    } else {
        let a851 = v0;
        let a852 = 27u64;
        if nat_eq(a851, a852) {
            8u64
        } else {
            0u64
        }
    }
}

pub fn f35(v0: Adt1) -> bool {
    let a854 = match v0.clone() {
        Adt1::C0(_, h853, _, _, _, _, _, _, _, _, _) => {
            h853
        }
    };
    let a855 = 1u64;
    if nat_eq(a854, a855) {
        true
    } else {
        let a858 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h856, _, _, _, _, _) => {
                h856
            }
        };
        let a859 = f26(match v0 {
            Adt1::C0(h857, _, _, _, _, _, _, _, _, _, _) => {
                h857
            }
        });
        nat_eq(a858, a859)
    }
}

pub fn f36(v0: Adt1, v1: u64) -> R<u64> {
    let a865 = {
        let a861 = match v0.clone() {
            Adt1::C0(_, h860, _, _, _, _, _, _, _, _, _) => {
                h860
            }
        };
        let a862 = f21(v1);
        let a863 = 0u64;
        nat_quot(a861, a862, a863)
    };
    let a866 = f58(match v0 {
        Adt1::C0(h864, _, _, _, _, _, _, _, _, _, _) => {
            h864
        }
    }, v1);
    nat_mul(a865, a866)
}

pub fn f37(v0: u64) -> Option<u64> {
    let a867 = v0;
    let a868 = 16u64;
    if if if nat_eq(a867, a868) {
        true
    } else {
        let a869 = v0;
        let a870 = 17u64;
        nat_eq(a869, a870)
    } {
        true
    } else {
        let a871 = v0;
        let a872 = 19u64;
        nat_eq(a871, a872)
    } {
        Some(1065353216u64)
    } else {
        None::<u64>
    }
}

pub fn f38(v0: u64) -> u64 {
    let a873 = v0;
    let a874 = 0u64;
    if nat_eq(a873, a874) {
        4u64
    } else {
        let a875 = v0;
        let a876 = 1u64;
        if if nat_eq(a875, a876) {
            true
        } else {
            let a877 = v0;
            let a878 = 30u64;
            nat_eq(a877, a878)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f39(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a880 = match v0.clone() {
            Adt1::C0(_, h879, _, _, _, _, _, _, _, _, _) => {
                h879
            }
        };
        let a881 = 4u64;
        nat_mul(a880, a881)?
    };
    let v2: u64 = {
        let a884 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h882, _, _, _, _) => {
                h882
            }
        };
        let a885 = match v0.clone() {
            Adt1::C0(_, _, h883, _, _, _, _, _, _, _, _) => {
                h883
            }
        };
        nat_mul(a884, a885)?
    };
    let v3: u64 = {
        let a887 = v2;
        let a888 = match v0.clone() {
            Adt1::C0(_, _, _, h886, _, _, _, _, _, _, _) => {
                h886
            }
        };
        nat_mul(a887, a888)?
    };
    let a890 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h889, _, _, _, _, _) => {
            h889
        }
    };
    let a891 = 4u64;
    if if if if nat_eq(a890, a891) {
        let a896 = {
            let a893 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h892) => {
                    h892
                }
            };
            let a894 = 4u64;
            let a895 = 1u64;
            nat_rem(a893, a894, a895)
        };
        let a897 = 0u64;
        nat_eq(a896, a897)
    } else {
        false
    } {
        let a905 = {
            let a903 = match v0.clone() {
                Adt1::C0(_, _, h898, _, _, _, _, _, _, _, _) => {
                    h898
                }
            };
            let a904 = {
                let a901 = match v0.clone() {
                    Adt1::C0(_, _, _, h899, _, _, _, _, _, _, _) => {
                        h899
                    }
                };
                let a902 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h900, _, _, _, _, _, _) => {
                        h900
                    }
                };
                nat_mul(a901, a902)?
            };
            nat_mul(a903, a904)?
        };
        let a906 = 1u64;
        if nat_le(a905, a906) {
            true
        } else {
            let a908 = v1;
            let a909 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h907, _, _, _, _) => {
                    h907
                }
            };
            if nat_le(a908, a909) {
                let a914 = {
                    let a911 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, h910, _, _, _, _) => {
                            h910
                        }
                    };
                    let a912 = 4u64;
                    let a913 = 1u64;
                    nat_rem(a911, a912, a913)
                };
                let a915 = 0u64;
                nat_eq(a914, a915)
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a917 = match v0.clone() {
            Adt1::C0(_, _, _, h916, _, _, _, _, _, _, _) => {
                h916
            }
        };
        let a918 = 1u64;
        if nat_le(a917, a918) {
            true
        } else {
            let a920 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h919, _, _, _) => {
                    h919
                }
            };
            let a921 = v2;
            nat_eq(a920, a921)
        }
    } else {
        false
    } {
        let a923 = match v0.clone() {
            Adt1::C0(_, _, _, _, h922, _, _, _, _, _, _) => {
                h922
            }
        };
        let a924 = 1u64;
        if nat_le(a923, a924) {
            Ok(true)
        } else {
            let a926 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, h925, _, _) => {
                    h925
                }
            };
            let a927 = v3;
            Ok(nat_eq(a926, a927))
        }
    } else {
        Ok(false)
    }
}

pub fn f40(v0: Adt1, v1: Adt1) -> R<bool> {
    let a929 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h928, _) => {
            h928
        }
    };
    let a930 = 0u64;
    if nat_eq(a929, a930) {
        Ok(true)
    } else {
        let a932 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h931, _) => {
                h931
            }
        };
        let a933 = 0u64;
        if nat_eq(a932, a933) {
            Ok(true)
        } else {
            let a935 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h934) => {
                    h934
                }
            };
            let a936 = 0u64;
            if nat_eq(a935, a936) {
                Ok(true)
            } else {
                let a938 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h937) => {
                        h937
                    }
                };
                let a939 = 0u64;
                if nat_eq(a938, a939) {
                    Ok(true)
                } else {
                    let a945 = {
                        let a942 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h940) => {
                                h940
                            }
                        };
                        let a943 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h941, _) => {
                                h941
                            }
                        };
                        nat_add(a942, a943)?
                    };
                    let a946 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h944) => {
                            h944
                        }
                    };
                    if nat_le(a945, a946) {
                        Ok(true)
                    } else {
                        let a952 = {
                            let a949 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h947) => {
                                    h947
                                }
                            };
                            let a950 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h948, _) => {
                                    h948
                                }
                            };
                            nat_add(a949, a950)?
                        };
                        let a953 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h951) => {
                                h951
                            }
                        };
                        Ok(nat_le(a952, a953))
                    }
                }
            }
        }
    }
}

pub fn f41(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a956 = match v0.clone() {
            Adt1::C0(_, _, h954, _, _, _, _, _, _, _, _) => {
                h954
            }
        };
        let a957 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h955, _, _, _, _) => {
                h955
            }
        };
        nat_mul(a956, a957)?
    };
    if if f35(v0.clone()) {
        let a959 = match v0.clone() {
            Adt1::C0(_, _, _, h958, _, _, _, _, _, _, _) => {
                h958
            }
        };
        let a960 = 1u64;
        if nat_eq(a959, a960) {
            true
        } else {
            let a962 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h961, _, _, _) => {
                    h961
                }
            };
            let a963 = v1;
            nat_eq(a962, a963)
        }
    } else {
        false
    } {
        let a965 = match v0.clone() {
            Adt1::C0(_, _, _, _, h964, _, _, _, _, _, _) => {
                h964
            }
        };
        let a966 = 1u64;
        if nat_eq(a965, a966) {
            Ok(true)
        } else {
            let a971 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h967, _, _) => {
                    h967
                }
            };
            let a972 = {
                let a969 = v1;
                let a970 = match v0 {
                    Adt1::C0(_, _, _, h968, _, _, _, _, _, _, _) => {
                        h968
                    }
                };
                nat_mul(a969, a970)?
            };
            Ok(nat_eq(a971, a972))
        }
    } else {
        Ok(false)
    }
}

pub fn f42(v0: u64) -> u64 {
    let a973 = v0;
    let a974 = 3u64;
    if nat_eq(a973, a974) {
        16u64
    } else {
        10u64
    }
}

pub fn f43(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a976 = v0;
        let a977 = match v1.clone() {
            Adt1::C0(_, h975, _, _, _, _, _, _, _, _, _) => {
                h975
            }
        };
        let a978 = 0u64;
        nat_quot(a976, a977, a978)
    };
    let v3: u64 = {
        let a980 = v2;
        let a981 = match v1.clone() {
            Adt1::C0(_, _, h979, _, _, _, _, _, _, _, _) => {
                h979
            }
        };
        let a982 = 0u64;
        nat_quot(a980, a981, a982)
    };
    f45({
        let a984 = v0;
        let a985 = match v1.clone() {
            Adt1::C0(_, h983, _, _, _, _, _, _, _, _, _) => {
                h983
            }
        };
        let a986 = 0u64;
        nat_rem(a984, a985, a986)
    }, {
        let a988 = v2;
        let a989 = match v1.clone() {
            Adt1::C0(_, _, h987, _, _, _, _, _, _, _, _) => {
                h987
            }
        };
        let a990 = 0u64;
        nat_rem(a988, a989, a990)
    }, {
        let a992 = v3;
        let a993 = match v1.clone() {
            Adt1::C0(_, _, _, h991, _, _, _, _, _, _, _) => {
                h991
            }
        };
        let a994 = 0u64;
        nat_rem(a992, a993, a994)
    }, {
        let a996 = v3;
        let a997 = match v1.clone() {
            Adt1::C0(_, _, _, h995, _, _, _, _, _, _, _) => {
                h995
            }
        };
        let a998 = 0u64;
        nat_quot(a996, a997, a998)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h999, _, _, _, _, _) => {
            h999
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h1000, _, _, _, _) => {
            h1000
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1001, _, _, _) => {
            h1001
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1002, _, _) => {
            h1002
        }
    })
}

pub fn f44(v0: u64, v1: Adt1, v2: Adt1) -> R<u64> {
    let v3: u64 = {
        let a1004 = v0;
        let a1005 = match v1.clone() {
            Adt1::C0(_, h1003, _, _, _, _, _, _, _, _, _) => {
                h1003
            }
        };
        let a1006 = 0u64;
        nat_quot(a1004, a1005, a1006)
    };
    let v4: u64 = {
        let a1008 = v3;
        let a1009 = match v1.clone() {
            Adt1::C0(_, _, h1007, _, _, _, _, _, _, _, _) => {
                h1007
            }
        };
        let a1010 = 0u64;
        nat_quot(a1008, a1009, a1010)
    };
    f45({
        let a1016 = {
            let a1012 = v0;
            let a1013 = match v1.clone() {
                Adt1::C0(_, h1011, _, _, _, _, _, _, _, _, _) => {
                    h1011
                }
            };
            let a1014 = 0u64;
            nat_rem(a1012, a1013, a1014)
        };
        let a1017 = match v2.clone() {
            Adt1::C0(_, h1015, _, _, _, _, _, _, _, _, _) => {
                h1015
            }
        };
        let a1018 = 0u64;
        nat_rem(a1016, a1017, a1018)
    }, {
        let a1024 = {
            let a1020 = v3;
            let a1021 = match v1.clone() {
                Adt1::C0(_, _, h1019, _, _, _, _, _, _, _, _) => {
                    h1019
                }
            };
            let a1022 = 0u64;
            nat_rem(a1020, a1021, a1022)
        };
        let a1025 = match v2.clone() {
            Adt1::C0(_, _, h1023, _, _, _, _, _, _, _, _) => {
                h1023
            }
        };
        let a1026 = 0u64;
        nat_rem(a1024, a1025, a1026)
    }, {
        let a1032 = {
            let a1028 = v4;
            let a1029 = match v1.clone() {
                Adt1::C0(_, _, _, h1027, _, _, _, _, _, _, _) => {
                    h1027
                }
            };
            let a1030 = 0u64;
            nat_rem(a1028, a1029, a1030)
        };
        let a1033 = match v2.clone() {
            Adt1::C0(_, _, _, h1031, _, _, _, _, _, _, _) => {
                h1031
            }
        };
        let a1034 = 0u64;
        nat_rem(a1032, a1033, a1034)
    }, {
        let a1040 = {
            let a1036 = v4;
            let a1037 = match v1 {
                Adt1::C0(_, _, _, h1035, _, _, _, _, _, _, _) => {
                    h1035
                }
            };
            let a1038 = 0u64;
            nat_quot(a1036, a1037, a1038)
        };
        let a1041 = match v2.clone() {
            Adt1::C0(_, _, _, _, h1039, _, _, _, _, _, _) => {
                h1039
            }
        };
        let a1042 = 0u64;
        nat_rem(a1040, a1041, a1042)
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h1043, _, _, _, _, _) => {
            h1043
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h1044, _, _, _, _) => {
            h1044
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1045, _, _, _) => {
            h1045
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1046, _, _) => {
            h1046
        }
    })
}

pub fn f45(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a1059 = {
        let a1055 = {
            let a1051 = {
                let a1047 = v0;
                let a1048 = v4;
                nat_mul(a1047, a1048)?
            };
            let a1052 = {
                let a1049 = v1;
                let a1050 = v5;
                nat_mul(a1049, a1050)?
            };
            nat_add(a1051, a1052)?
        };
        let a1056 = {
            let a1053 = v2;
            let a1054 = v6;
            nat_mul(a1053, a1054)?
        };
        nat_add(a1055, a1056)?
    };
    let a1060 = {
        let a1057 = v3;
        let a1058 = v7;
        nat_mul(a1057, a1058)?
    };
    nat_add(a1059, a1060)
}

pub fn f46(v0: u64, v1: Adt1, v2: u64, v3: u64) -> R<Adt3> {
    let v4: u64 = {
        let a1061 = v0;
        let a1062 = 2u64;
        let a1063 = 0u64;
        nat_quot(a1061, a1062, a1063)
    };
    let v5: u64 = f34(match v1.clone() {
        Adt1::C0(h1064, _, _, _, _, _, _, _, _, _, _) => {
            h1064
        }
    });
    let a1068 = {
        let a1065 = v0;
        let a1066 = 2u64;
        let a1067 = 0u64;
        nat_rem(a1065, a1066, a1067)
    };
    let a1069 = 0u64;
    if nat_eq(a1068, a1069) {
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 4u64, f43(v4, v1)?, 2u64, 0u64, 0u64, 2u64, v5, 8u64))
    } else {
        let a1070 = v3;
        let a1071 = v2;
        if if nat_lt(a1070, a1071) {
            let a1074 = v3;
            let a1075 = {
                let a1072 = v5;
                let a1073 = 4u64;
                if nat_eq(a1072, a1073) {
                    2147483648u64
                } else {
                    9223372036854775808u64
                }
            };
            nat_lt(a1074, a1075)
        } else {
            false
        } {
            Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, {
                let a1076 = 3u64;
                let a1077 = v4;
                nat_add(a1076, a1077)?
            }, 8u64, 8u64))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f47(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64) -> R<Adt3> {
    let v5: u64 = {
        let a1079 = 1u64;
        let a1080 = f36(v1.clone(), match v3.clone() {
            Adt1::C0(h1078, _, _, _, _, _, _, _, _, _, _) => {
                h1078
            }
        })?;
        nat_add(a1079, a1080)?
    };
    let v6: u64 = {
        let a1081 = v0;
        let a1082 = v5;
        let a1083 = 0u64;
        nat_rem(a1081, a1082, a1083)
    };
    let v7: u64 = {
        let a1084 = v0;
        let a1085 = v5;
        let a1086 = 0u64;
        nat_quot(a1084, a1085, a1086)
    };
    let v8: u64 = {
        let a1088 = v7;
        let a1089 = match v1.clone() {
            Adt1::C0(_, _, h1087, _, _, _, _, _, _, _, _) => {
                h1087
            }
        };
        let a1090 = 0u64;
        nat_rem(a1088, a1089, a1090)
    };
    let v9: u64 = {
        let a1092 = v7;
        let a1093 = match v1.clone() {
            Adt1::C0(_, _, h1091, _, _, _, _, _, _, _, _) => {
                h1091
            }
        };
        let a1094 = 0u64;
        nat_quot(a1092, a1093, a1094)
    };
    let v10: u64 = {
        let a1096 = v9;
        let a1097 = match v1.clone() {
            Adt1::C0(_, _, _, h1095, _, _, _, _, _, _, _) => {
                h1095
            }
        };
        let a1098 = 0u64;
        nat_rem(a1096, a1097, a1098)
    };
    let v11: u64 = {
        let a1100 = v9;
        let a1101 = match v1.clone() {
            Adt1::C0(_, _, _, h1099, _, _, _, _, _, _, _) => {
                h1099
            }
        };
        let a1102 = 0u64;
        nat_quot(a1100, a1101, a1102)
    };
    let a1103 = v6;
    let a1104 = 0u64;
    if nat_eq(a1103, a1104) {
        let v12: u64 = {
            let a1121 = v8;
            let a1122 = {
                let a1119 = match v2.clone() {
                    Adt1::C0(_, h1105, _, _, _, _, _, _, _, _, _) => {
                        h1105
                    }
                };
                let a1120 = {
                    let a1117 = {
                        let a1107 = v10;
                        let a1108 = match v2.clone() {
                            Adt1::C0(_, _, h1106, _, _, _, _, _, _, _, _) => {
                                h1106
                            }
                        };
                        let a1109 = 0u64;
                        nat_rem(a1107, a1108, a1109)
                    };
                    let a1118 = {
                        let a1115 = match v2.clone() {
                            Adt1::C0(_, _, h1110, _, _, _, _, _, _, _, _) => {
                                h1110
                            }
                        };
                        let a1116 = {
                            let a1112 = v11;
                            let a1113 = match v2 {
                                Adt1::C0(_, _, _, h1111, _, _, _, _, _, _, _) => {
                                    h1111
                                }
                            };
                            let a1114 = 0u64;
                            nat_rem(a1112, a1113, a1114)
                        };
                        nat_mul(a1115, a1116)?
                    };
                    nat_add(a1117, a1118)?
                };
                nat_mul(a1119, a1120)?
            };
            nat_add(a1121, a1122)?
        };
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, {
            let a1123 = 3u64;
            let a1124 = v12;
            nat_add(a1123, a1124)?
        }, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
    } else {
        let a1126 = v4;
        let a1127 = match v3.clone() {
            Adt1::C0(_, _, h1125, _, _, _, _, _, _, _, _) => {
                h1125
            }
        };
        if nat_lt(a1126, a1127) {
            let v13: u64 = {
                let a1132 = {
                    let a1128 = v6;
                    let a1129 = 1u64;
                    nat_sub(a1128, a1129)
                };
                let a1133 = f58(match v1.clone() {
                    Adt1::C0(h1130, _, _, _, _, _, _, _, _, _, _) => {
                        h1130
                    }
                }, match v3.clone() {
                    Adt1::C0(h1131, _, _, _, _, _, _, _, _, _, _) => {
                        h1131
                    }
                });
                let a1134 = 0u64;
                nat_quot(a1132, a1133, a1134)
            };
            let v14: u64 = f45({
                let a1136 = v13;
                let a1137 = f21(match v3.clone() {
                    Adt1::C0(h1135, _, _, _, _, _, _, _, _, _, _) => {
                        h1135
                    }
                });
                nat_mul(a1136, a1137)?
            }, v8, v10, v11, match v1.clone() {
                Adt1::C0(_, _, _, _, _, h1138, _, _, _, _, _) => {
                    h1138
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, h1139, _, _, _, _) => {
                    h1139
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1140, _, _, _) => {
                    h1140
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1141, _, _) => {
                    h1141
                }
            })?;
            let v15: u64 = f45(v13, v4, v10, v11, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h1142, _, _, _, _, _) => {
                    h1142
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h1143, _, _, _, _) => {
                    h1143
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1144, _, _, _) => {
                    h1144
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1145, _, _) => {
                    h1145
                }
            })?;
            if if f2(match v3.clone() {
                Adt1::C0(h1146, _, _, _, _, _, _, _, _, _, _) => {
                    h1146
                }
            }) {
                true
            } else {
                let a1148 = match v3.clone() {
                    Adt1::C0(h1147, _, _, _, _, _, _, _, _, _, _) => {
                        h1147
                    }
                };
                let a1149 = 26u64;
                nat_eq(a1148, a1149)
            } {
                Ok(Adt3::C6(v14, v15))
            } else {
                Ok(f59(match v1 {
                    Adt1::C0(h1150, _, _, _, _, _, _, _, _, _, _) => {
                        h1150
                    }
                }, match v3 {
                    Adt1::C0(h1151, _, _, _, _, _, _, _, _, _, _) => {
                        h1151
                    }
                }, {
                    let a1152 = v6;
                    let a1153 = 1u64;
                    nat_sub(a1152, a1153)
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
    let a1154 = v0;
    let a1155 = 8u64;
    if nat_eq(a1154, a1155) {
        Adt3::C3(1u64, 6u64, 0u64, 0u64, 1u64, 0u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
    } else {
        let a1156 = v0;
        let a1157 = 9u64;
        if nat_eq(a1156, a1157) {
            Adt3::C3(1u64, 2u64, 0u64, 0u64, 2u64, v1, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
        } else {
            let a1158 = v0;
            let a1159 = 10u64;
            if nat_eq(a1158, a1159) {
                Adt3::C3(5u64, 16u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
            } else {
                let a1160 = v0;
                let a1161 = 11u64;
                if nat_eq(a1160, a1161) {
                    Adt3::C3(1u64, 0u64, 0u64, 0u64, 2u64, 1065353216u64, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
                } else {
                    let a1162 = v0;
                    let a1163 = 12u64;
                    if nat_eq(a1162, a1163) {
                        Adt3::C3(1u64, 3u64, 0u64, 0u64, 1u64, 0u64, 1u64, 2u64, 0u64, 0u64, 4u64, 4u64)
                    } else {
                        let a1164 = v0;
                        let a1165 = 13u64;
                        if nat_eq(a1164, a1165) {
                            Adt3::C3(1u64, 0u64, 0u64, 0u64, 1u64, 1u64, 2u64, 1065353216u64, 0u64, 1u64, 4u64, 4u64)
                        } else {
                            let a1166 = v0;
                            let a1167 = 14u64;
                            if nat_eq(a1166, a1167) {
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
    let a1168 = v0;
    let a1169 = 0u64;
    if nat_eq(a1168, a1169) {
        Adt4::C1(Adt5::C0(0u64, 1u64, 4u64))
    } else {
        let a1170 = v0;
        let a1171 = 1u64;
        if nat_eq(a1170, a1171) {
            Adt4::C1(Adt5::C0(1u64, 1u64, 2u64))
        } else {
            let a1172 = v0;
            let a1173 = 2u64;
            if nat_eq(a1172, a1173) {
                Adt4::C1(Adt5::C0(2u64, 32u64, 18u64))
            } else {
                let a1174 = v0;
                let a1175 = 3u64;
                if nat_eq(a1174, a1175) {
                    Adt4::C1(Adt5::C0(3u64, 32u64, 20u64))
                } else {
                    let a1176 = v0;
                    let a1177 = 6u64;
                    if nat_eq(a1176, a1177) {
                        Adt4::C1(Adt5::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1178 = v0;
                        let a1179 = 7u64;
                        if nat_eq(a1178, a1179) {
                            Adt4::C1(Adt5::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1180 = v0;
                            let a1181 = 8u64;
                            if nat_eq(a1180, a1181) {
                                Adt4::C1(Adt5::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1182 = v0;
                                let a1183 = 9u64;
                                if nat_eq(a1182, a1183) {
                                    Adt4::C1(Adt5::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1184 = v0;
                                    let a1185 = 10u64;
                                    if nat_eq(a1184, a1185) {
                                        Adt4::C1(Adt5::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1186 = v0;
                                        let a1187 = 11u64;
                                        if nat_eq(a1186, a1187) {
                                            Adt4::C1(Adt5::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1188 = v0;
                                            let a1189 = 12u64;
                                            if nat_eq(a1188, a1189) {
                                                Adt4::C1(Adt5::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1190 = v0;
                                                let a1191 = 13u64;
                                                if nat_eq(a1190, a1191) {
                                                    Adt4::C1(Adt5::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1192 = v0;
                                                    let a1193 = 14u64;
                                                    if nat_eq(a1192, a1193) {
                                                        Adt4::C1(Adt5::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1194 = v0;
                                                        let a1195 = 15u64;
                                                        if nat_eq(a1194, a1195) {
                                                            Adt4::C1(Adt5::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1196 = v0;
                                                            let a1197 = 16u64;
                                                            if nat_eq(a1196, a1197) {
                                                                Adt4::C1(Adt5::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1198 = v0;
                                                                let a1199 = 17u64;
                                                                if nat_eq(a1198, a1199) {
                                                                    Adt4::C1(Adt5::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1200 = v0;
                                                                    let a1201 = 18u64;
                                                                    if nat_eq(a1200, a1201) {
                                                                        Adt4::C1(Adt5::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1202 = v0;
                                                                        let a1203 = 19u64;
                                                                        if nat_eq(a1202, a1203) {
                                                                            Adt4::C1(Adt5::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1204 = v0;
                                                                            let a1205 = 20u64;
                                                                            if nat_eq(a1204, a1205) {
                                                                                Adt4::C1(Adt5::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1206 = v0;
                                                                                let a1207 = 21u64;
                                                                                if nat_eq(a1206, a1207) {
                                                                                    Adt4::C1(Adt5::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1208 = v0;
                                                                                    let a1209 = 22u64;
                                                                                    if nat_eq(a1208, a1209) {
                                                                                        Adt4::C1(Adt5::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1210 = v0;
                                                                                        let a1211 = 23u64;
                                                                                        if nat_eq(a1210, a1211) {
                                                                                            Adt4::C1(Adt5::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1212 = v0;
                                                                                            let a1213 = 24u64;
                                                                                            if nat_eq(a1212, a1213) {
                                                                                                Adt4::C1(Adt5::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1214 = v0;
                                                                                                let a1215 = 25u64;
                                                                                                if nat_eq(a1214, a1215) {
                                                                                                    Adt4::C1(Adt5::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1216 = v0;
                                                                                                    let a1217 = 26u64;
                                                                                                    if nat_eq(a1216, a1217) {
                                                                                                        Adt4::C1(Adt5::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1218 = v0;
                                                                                                        let a1219 = 27u64;
                                                                                                        if nat_eq(a1218, a1219) {
                                                                                                            Adt4::C1(Adt5::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1220 = v0;
                                                                                                            let a1221 = 28u64;
                                                                                                            if nat_eq(a1220, a1221) {
                                                                                                                Adt4::C1(Adt5::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1222 = v0;
                                                                                                                let a1223 = 29u64;
                                                                                                                if nat_eq(a1222, a1223) {
                                                                                                                    Adt4::C1(Adt5::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1224 = v0;
                                                                                                                    let a1225 = 30u64;
                                                                                                                    if nat_eq(a1224, a1225) {
                                                                                                                        Adt4::C1(Adt5::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1226 = v0;
                                                                                                                        let a1227 = 34u64;
                                                                                                                        if nat_eq(a1226, a1227) {
                                                                                                                            Adt4::C1(Adt5::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1228 = v0;
                                                                                                                            let a1229 = 35u64;
                                                                                                                            if nat_eq(a1228, a1229) {
                                                                                                                                Adt4::C1(Adt5::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1230 = v0;
                                                                                                                                let a1231 = 39u64;
                                                                                                                                if nat_eq(a1230, a1231) {
                                                                                                                                    Adt4::C1(Adt5::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1232 = v0;
                                                                                                                                    let a1233 = 40u64;
                                                                                                                                    if nat_eq(a1232, a1233) {
                                                                                                                                        Adt4::C1(Adt5::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1234 = v0;
                                                                                                                                        let a1235 = 41u64;
                                                                                                                                        if nat_eq(a1234, a1235) {
                                                                                                                                            Adt4::C1(Adt5::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1236 = v0;
                                                                                                                                            let a1237 = 42u64;
                                                                                                                                            if nat_eq(a1236, a1237) {
                                                                                                                                                Adt4::C1(Adt5::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1238 = v0;
                                                                                                                                                let a1239 = 43u64;
                                                                                                                                                if nat_eq(a1238, a1239) {
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
        let a1242 = match v0.clone() {
            Adt1::C0(h1240, _, _, _, _, _, _, _, _, _, _) => {
                h1240
            }
        };
        let a1243 = match v1.clone() {
            Adt1::C0(h1241, _, _, _, _, _, _, _, _, _, _) => {
                h1241
            }
        };
        nat_eq(a1242, a1243)
    } else {
        false
    } {
        let a1246 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1244) => {
                h1244
            }
        };
        let a1247 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1245) => {
                h1245
            }
        };
        nat_eq(a1246, a1247)
    } else {
        false
    } {
        let a1250 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1248, _, _, _, _, _) => {
                h1248
            }
        };
        let a1251 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1249, _, _, _, _, _) => {
                h1249
            }
        };
        nat_eq(a1250, a1251)
    } else {
        false
    } {
        let a1254 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1252, _, _, _, _) => {
                h1252
            }
        };
        let a1255 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1253, _, _, _, _) => {
                h1253
            }
        };
        nat_eq(a1254, a1255)
    } else {
        false
    } {
        let a1258 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1256, _, _, _) => {
                h1256
            }
        };
        let a1259 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1257, _, _, _) => {
                h1257
            }
        };
        nat_eq(a1258, a1259)
    } else {
        false
    } {
        let a1262 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1260, _, _) => {
                h1260
            }
        };
        let a1263 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1261, _, _) => {
                h1261
            }
        };
        nat_eq(a1262, a1263)
    } else {
        false
    }
}

pub fn f53(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a1264 = v1;
    let a1265 = 1u64;
    if if if nat_le(a1264, a1265) {
        true
    } else {
        let a1266 = v0;
        let a1267 = v2;
        nat_le(a1266, a1267)
    } {
        let a1268 = v3;
        let a1269 = 1u64;
        if nat_le(a1268, a1269) {
            true
        } else {
            let a1276 = {
                let a1274 = v0;
                let a1275 = {
                    let a1272 = {
                        let a1270 = v1;
                        let a1271 = 1u64;
                        nat_sub(a1270, a1271)
                    };
                    let a1273 = v2;
                    nat_mul(a1272, a1273)?
                };
                nat_add(a1274, a1275)?
            };
            let a1277 = v4;
            nat_le(a1276, a1277)
        }
    } else {
        false
    } {
        let a1278 = v5;
        let a1279 = 1u64;
        if nat_le(a1278, a1279) {
            Ok(true)
        } else {
            let a1292 = {
                let a1290 = {
                    let a1284 = v0;
                    let a1285 = {
                        let a1282 = {
                            let a1280 = v1;
                            let a1281 = 1u64;
                            nat_sub(a1280, a1281)
                        };
                        let a1283 = v2;
                        nat_mul(a1282, a1283)?
                    };
                    nat_add(a1284, a1285)?
                };
                let a1291 = {
                    let a1288 = {
                        let a1286 = v3;
                        let a1287 = 1u64;
                        nat_sub(a1286, a1287)
                    };
                    let a1289 = v4;
                    nat_mul(a1288, a1289)?
                };
                nat_add(a1290, a1291)?
            };
            let a1293 = v6;
            Ok(nat_le(a1292, a1293))
        }
    } else {
        Ok(false)
    }
}

pub fn f54(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1294 = v0;
    let a1295 = 0u64;
    if if if nat_eq(a1294, a1295) {
        true
    } else {
        let a1296 = v1;
        let a1297 = 0u64;
        nat_eq(a1296, a1297)
    } {
        true
    } else {
        let a1298 = v2;
        let a1299 = 0u64;
        nat_eq(a1298, a1299)
    } {
        true
    } else {
        let a1300 = v3;
        let a1301 = 0u64;
        nat_eq(a1300, a1301)
    }
}

pub fn f55(v0: u64, v1: u64, v2: u64) -> bool {
    let a1302 = v0;
    let a1303 = 0u64;
    if if if nat_eq(a1302, a1303) {
        let a1304 = v1;
        let a1305 = 0u64;
        nat_eq(a1304, a1305)
    } else {
        false
    } {
        let a1306 = v2;
        let a1307 = 0u64;
        nat_eq(a1306, a1307)
    } else {
        false
    } {
        true
    } else {
        let a1308 = v0;
        let a1309 = 1u64;
        if if nat_eq(a1308, a1309) {
            true
        } else {
            let a1310 = v0;
            let a1311 = 30u64;
            nat_eq(a1310, a1311)
        } {
            let a1312 = v1;
            let a1313 = v0;
            if if nat_eq(a1312, a1313) {
                let a1314 = v2;
                let a1315 = v0;
                nat_eq(a1314, a1315)
            } else {
                false
            } {
                true
            } else {
                let a1316 = v1;
                let a1317 = 0u64;
                if nat_eq(a1316, a1317) {
                    let a1318 = v2;
                    let a1319 = v0;
                    if nat_eq(a1318, a1319) {
                        true
                    } else {
                        let a1320 = v2;
                        let a1321 = 0u64;
                        nat_eq(a1320, a1321)
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
    let a1322 = v0;
    let a1323 = 1u64;
    if nat_le(a1322, a1323) {
        true
    } else {
        let a1327 = {
            let a1324 = v1;
            let a1325 = v2;
            let a1326 = 1u64;
            nat_rem(a1324, a1325, a1326)
        };
        let a1328 = 0u64;
        nat_eq(a1327, a1328)
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
    let a1329 = v0;
    let a1330 = 1u64;
    if if nat_eq(a1329, a1330) {
        let a1331 = v1;
        let a1332 = 43u64;
        nat_eq(a1331, a1332)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f59(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt3 {
    let a1333 = f58(v0, v1);
    let a1334 = 2u64;
    if nat_eq(a1333, a1334) {
        let a1338 = {
            let a1335 = v2;
            let a1336 = 2u64;
            let a1337 = 0u64;
            nat_rem(a1335, a1336, a1337)
        };
        let a1339 = 0u64;
        if nat_eq(a1338, a1339) {
            Adt3::C3(0u64, 0u64, v0, 0u64, 0u64, v3, 2u64, 0u64, 1u64, 0u64, f38(v0), 8u64)
        } else {
            Adt3::C3(0u64, 0u64, 0u64, v1, 1u64, 0u64, 2u64, 0u64, 3u64, v4, 8u64, f20(v1))
        }
    } else {
        Adt3::C3(0u64, 0u64, v0, v1, 0u64, v3, 2u64, 0u64, 3u64, v4, f38(v0), f20(v1))
    }
}
