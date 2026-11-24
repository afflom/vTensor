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
        f18(v0.clone(), v1.clone())
    } else {
        false
    } {
        f19(v0.clone(), v2.clone())
    } else {
        false
    } {
        let a69 = match v1.clone() {
            Adt1::C0(h68, _, _, _, _, _, _, _, _, _, _) => {
                h68
            }
        };
        let a70 = 0u64;
        nat_eq(a69, a70)
    } else {
        false
    } {
        let a72 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h71, _, _, _, _, _) => {
                h71
            }
        };
        let a73 = 4u64;
        nat_eq(a72, a73)
    } else {
        false
    } {
        f2(match v0.clone() {
            Adt1::C0(h74, _, _, _, _, _, _, _, _, _, _) => {
                h74
            }
        })
    } else {
        false
    } {
        let a76 = match v0.clone() {
            Adt1::C0(h75, _, _, _, _, _, _, _, _, _, _) => {
                h75
            }
        };
        let a77 = 9u64;
        !if nat_eq(a76, a77) {
            true
        } else {
            let a79 = match v0.clone() {
                Adt1::C0(h78, _, _, _, _, _, _, _, _, _, _) => {
                    h78
                }
            };
            let a80 = 15u64;
            nat_eq(a79, a80)
        }
    } else {
        false
    } {
        let a82 = 0u64;
        let a83 = f20(match v2.clone() {
            Adt1::C0(h81, _, _, _, _, _, _, _, _, _, _) => {
                h81
            }
        });
        nat_lt(a82, a83)
    } else {
        false
    } {
        let a85 = 16u64;
        let a86 = match v2.clone() {
            Adt1::C0(h84, _, _, _, _, _, _, _, _, _, _) => {
                h84
            }
        };
        !if if if nat_le(a85, a86) {
            let a88 = match v2.clone() {
                Adt1::C0(h87, _, _, _, _, _, _, _, _, _, _) => {
                    h87
                }
            };
            let a89 = 19u64;
            nat_le(a88, a89)
        } else {
            false
        } {
            true
        } else {
            let a91 = match v2.clone() {
                Adt1::C0(h90, _, _, _, _, _, _, _, _, _, _) => {
                    h90
                }
            };
            let a92 = 21u64;
            if nat_eq(a91, a92) {
                true
            } else {
                let a94 = match v2.clone() {
                    Adt1::C0(h93, _, _, _, _, _, _, _, _, _, _) => {
                        h93
                    }
                };
                let a95 = 22u64;
                nat_eq(a94, a95)
            }
        } {
            true
        } else {
            let a97 = match v2.clone() {
                Adt1::C0(h96, _, _, _, _, _, _, _, _, _, _) => {
                    h96
                }
            };
            let a98 = 29u64;
            nat_eq(a97, a98)
        }
    } else {
        false
    } {
        let a101 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h99, _, _, _, _, _) => {
                h99
            }
        };
        let a102 = f20(match v0.clone() {
            Adt1::C0(h100, _, _, _, _, _, _, _, _, _, _) => {
                h100
            }
        });
        nat_eq(a101, a102)
    } else {
        false
    } {
        let a105 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h103, _, _, _, _, _) => {
                h103
            }
        };
        let a106 = f20(match v2.clone() {
            Adt1::C0(h104, _, _, _, _, _, _, _, _, _, _) => {
                h104
            }
        });
        nat_eq(a105, a106)
    } else {
        false
    } {
        let a112 = {
            let a109 = match v0.clone() {
                Adt1::C0(_, h107, _, _, _, _, _, _, _, _, _) => {
                    h107
                }
            };
            let a110 = f21(match v0.clone() {
                Adt1::C0(h108, _, _, _, _, _, _, _, _, _, _) => {
                    h108
                }
            });
            let a111 = 1u64;
            nat_rem(a109, a110, a111)
        };
        let a113 = 0u64;
        nat_eq(a112, a113)
    } else {
        false
    } {
        let a119 = {
            let a116 = match v2.clone() {
                Adt1::C0(_, h114, _, _, _, _, _, _, _, _, _) => {
                    h114
                }
            };
            let a117 = f21(match v2.clone() {
                Adt1::C0(h115, _, _, _, _, _, _, _, _, _, _) => {
                    h115
                }
            });
            let a118 = 1u64;
            nat_rem(a116, a117, a118)
        };
        let a120 = 0u64;
        nat_eq(a119, a120)
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
            let a127 = {
                let a123 = match v2.clone() {
                    Adt1::C0(_, h121, _, _, _, _, _, _, _, _, _) => {
                        h121
                    }
                };
                let a124 = f21(match v2.clone() {
                    Adt1::C0(h122, _, _, _, _, _, _, _, _, _, _) => {
                        h122
                    }
                });
                let a125 = 0u64;
                nat_quot(a123, a124, a125)
            };
            let a128 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, h126, _, _, _, _, _) => {
                    h126
                }
            };
            nat_mul(a127, a128)?
        }, match v2.clone() {
            Adt1::C0(_, _, h129, _, _, _, _, _, _, _, _) => {
                h129
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h130, _, _, _, _, _, _, _) => {
                h130
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h131, _, _, _, _, _, _) => {
                h131
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h132, _, _, _, _) => {
                h132
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h133, _, _, _) => {
                h133
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h134, _, _) => {
                h134
            }
        })?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0(if f24(v0.clone()) {
            0u64
        } else {
            let a140 = match v0.clone() {
                Adt1::C0(_, _, h135, _, _, _, _, _, _, _, _) => {
                    h135
                }
            };
            let a141 = {
                let a138 = match v0.clone() {
                    Adt1::C0(_, _, _, h136, _, _, _, _, _, _, _) => {
                        h136
                    }
                };
                let a139 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h137, _, _, _, _, _, _) => {
                        h137
                    }
                };
                nat_mul(a138, a139)?
            };
            nat_mul(a140, a141)?
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
            let a144 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, h142, _, _, _, _, _) => {
                    h142
                }
            };
            let a145 = f26(match v1.clone() {
                Adt1::C0(h143, _, _, _, _, _, _, _, _, _, _) => {
                    h143
                }
            });
            if if if if if if if nat_eq(a144, a145) {
                let a148 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h146, _, _, _, _, _) => {
                        h146
                    }
                };
                let a149 = f26(match v3.clone() {
                    Adt1::C0(h147, _, _, _, _, _, _, _, _, _, _) => {
                        h147
                    }
                });
                nat_eq(a148, a149)
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
            let a152 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h150, _) => {
                    h150
                }
            };
            let a153 = f26(match v3.clone() {
                Adt1::C0(h151, _, _, _, _, _, _, _, _, _, _) => {
                    h151
                }
            });
            let a154 = 0u64;
            nat_quot(a152, a153, a154)
        };
        Ok(Adt3::C2(Adt2::C0(f30(f31(v1.clone())?, v4, f32(match v1.clone() {
            Adt1::C0(h155, _, _, _, _, _, _, _, _, _, _) => {
                h155
            }
        }, match v2.clone() {
            Adt1::C0(h156, _, _, _, _, _, _, _, _, _, _) => {
                h156
            }
        }, match v3.clone() {
            Adt1::C0(h157, _, _, _, _, _, _, _, _, _, _) => {
                h157
            }
        })?)?, {
            let a158 = 3u64;
            let a159 = v4;
            nat_add(a158, a159)?
        }, None::<u64>, v0, 0u64, 0u64, 0u64, v1, v2, v3, false)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f5(v0: Adt1, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if if if if if if if if if if if if if f33(v2, v3.clone()) {
        let a161 = match v0.clone() {
            Adt1::C0(h160, _, _, _, _, _, _, _, _, _, _) => {
                h160
            }
        };
        let a162 = 0u64;
        if nat_eq(a161, a162) {
            true
        } else {
            let a164 = match v0.clone() {
                Adt1::C0(h163, _, _, _, _, _, _, _, _, _, _) => {
                    h163
                }
            };
            let a165 = 1u64;
            nat_eq(a164, a165)
        }
    } else {
        false
    } {
        let a167 = 0u64;
        let a168 = f20(match v3.clone() {
            Adt1::C0(h166, _, _, _, _, _, _, _, _, _, _) => {
                h166
            }
        });
        nat_lt(a167, a168)
    } else {
        false
    } {
        let a170 = 0u64;
        let a171 = f34(match v1.clone() {
            Adt1::C0(h169, _, _, _, _, _, _, _, _, _, _) => {
                h169
            }
        });
        nat_lt(a170, a171)
    } else {
        false
    } {
        let a174 = match v0.clone() {
            Adt1::C0(_, h172, _, _, _, _, _, _, _, _, _) => {
                h172
            }
        };
        let a175 = match v3.clone() {
            Adt1::C0(_, h173, _, _, _, _, _, _, _, _, _) => {
                h173
            }
        };
        if nat_eq(a174, a175) {
            let a181 = {
                let a178 = match v0.clone() {
                    Adt1::C0(_, h176, _, _, _, _, _, _, _, _, _) => {
                        h176
                    }
                };
                let a179 = f21(match v3.clone() {
                    Adt1::C0(h177, _, _, _, _, _, _, _, _, _, _) => {
                        h177
                    }
                });
                let a180 = 1u64;
                nat_rem(a178, a179, a180)
            };
            let a182 = 0u64;
            nat_eq(a181, a182)
        } else {
            false
        }
    } else {
        false
    } {
        let a185 = match v0.clone() {
            Adt1::C0(_, _, _, h183, _, _, _, _, _, _, _) => {
                h183
            }
        };
        let a186 = match v3.clone() {
            Adt1::C0(_, _, _, h184, _, _, _, _, _, _, _) => {
                h184
            }
        };
        nat_eq(a185, a186)
    } else {
        false
    } {
        let a189 = match v0.clone() {
            Adt1::C0(_, _, _, _, h187, _, _, _, _, _, _) => {
                h187
            }
        };
        let a190 = match v3.clone() {
            Adt1::C0(_, _, _, _, h188, _, _, _, _, _, _) => {
                h188
            }
        };
        nat_eq(a189, a190)
    } else {
        false
    } {
        let a193 = match v0.clone() {
            Adt1::C0(_, _, h191, _, _, _, _, _, _, _, _) => {
                h191
            }
        };
        let a194 = match v1.clone() {
            Adt1::C0(_, h192, _, _, _, _, _, _, _, _, _) => {
                h192
            }
        };
        nat_eq(a193, a194)
    } else {
        false
    } {
        let a196 = match v1.clone() {
            Adt1::C0(_, _, _, _, h195, _, _, _, _, _, _) => {
                h195
            }
        };
        let a197 = 1u64;
        nat_eq(a196, a197)
    } else {
        false
    } {
        let a199 = 0u64;
        let a200 = match v1.clone() {
            Adt1::C0(_, _, h198, _, _, _, _, _, _, _, _) => {
                h198
            }
        };
        nat_lt(a199, a200)
    } else {
        false
    } {
        let a202 = 0u64;
        let a203 = match v1.clone() {
            Adt1::C0(_, _, _, h201, _, _, _, _, _, _, _) => {
                h201
            }
        };
        nat_lt(a202, a203)
    } else {
        false
    } {
        let a209 = {
            let a206 = match v0.clone() {
                Adt1::C0(_, _, _, h204, _, _, _, _, _, _, _) => {
                    h204
                }
            };
            let a207 = match v1.clone() {
                Adt1::C0(_, _, h205, _, _, _, _, _, _, _, _) => {
                    h205
                }
            };
            let a208 = 0u64;
            nat_rem(a206, a207, a208)
        };
        let a210 = 0u64;
        nat_eq(a209, a210)
    } else {
        false
    } {
        let a216 = {
            let a213 = match v0.clone() {
                Adt1::C0(_, _, _, _, h211, _, _, _, _, _, _) => {
                    h211
                }
            };
            let a214 = match v1.clone() {
                Adt1::C0(_, _, _, h212, _, _, _, _, _, _, _) => {
                    h212
                }
            };
            let a215 = 0u64;
            nat_rem(a213, a214, a215)
        };
        let a217 = 0u64;
        nat_eq(a216, a217)
    } else {
        false
    } {
        f35(v0.clone())
    } else {
        false
    } {
        let a219 = match v3.clone() {
            Adt1::C0(_, h218, _, _, _, _, _, _, _, _, _) => {
                h218
            }
        };
        let a220 = 1u64;
        if nat_eq(a219, a220) {
            true
        } else {
            let a223 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h221, _, _, _, _, _) => {
                    h221
                }
            };
            let a224 = f20(match v3.clone() {
                Adt1::C0(h222, _, _, _, _, _, _, _, _, _, _) => {
                    h222
                }
            });
            nat_eq(a223, a224)
        }
    } else {
        false
    } {
        let v4: u64 = f31(v1.clone())?;
        let v5: u64 = {
            let a230 = match v0.clone() {
                Adt1::C0(_, _, h225, _, _, _, _, _, _, _, _) => {
                    h225
                }
            };
            let a231 = {
                let a228 = match v0.clone() {
                    Adt1::C0(_, _, _, h226, _, _, _, _, _, _, _) => {
                        h226
                    }
                };
                let a229 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h227, _, _, _, _, _, _) => {
                        h227
                    }
                };
                nat_mul(a228, a229)?
            };
            nat_mul(a230, a231)?
        };
        let v6: u64 = if f24(v0.clone()) {
            0u64
        } else {
            let a239 = {
                let a232 = 2u64;
                let a233 = v4;
                nat_mul(a232, a233)?
            };
            let a240 = {
                let a237 = v5;
                let a238 = {
                    let a235 = 1u64;
                    let a236 = f36(v0.clone(), match v3.clone() {
                        Adt1::C0(h234, _, _, _, _, _, _, _, _, _, _) => {
                            h234
                        }
                    })?;
                    nat_add(a235, a236)?
                };
                nat_mul(a237, a238)?
            };
            nat_add(a239, a240)?
        };
        Ok(Adt3::C2(Adt2::C0(v6, if f24(v0.clone()) {
            3u64
        } else {
            let a241 = 3u64;
            let a242 = v4;
            nat_add(a241, a242)?
        }, f37(match v3.clone() {
            Adt1::C0(h243, _, _, _, _, _, _, _, _, _, _) => {
                h243
            }
        }), 41u64, 0u64, 0u64, 0u64, v0, v1, v3, false)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f6(v0: Adt1, v1: Adt1, v2: Adt1) -> R<Adt3> {
    let v3: u64 = f21(match v2.clone() {
        Adt1::C0(h244, _, _, _, _, _, _, _, _, _, _) => {
            h244
        }
    });
    if if if if if if if f2(match v2.clone() {
        Adt1::C0(h245, _, _, _, _, _, _, _, _, _, _) => {
            h245
        }
    }) {
        f33(v1.clone(), v2.clone())
    } else {
        false
    } {
        let a247 = 0u64;
        let a248 = f38(match v0.clone() {
            Adt1::C0(h246, _, _, _, _, _, _, _, _, _, _) => {
                h246
            }
        });
        nat_lt(a247, a248)
    } else {
        false
    } {
        let a251 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h249, _, _, _, _, _) => {
                h249
            }
        };
        let a252 = f38(match v0.clone() {
            Adt1::C0(h250, _, _, _, _, _, _, _, _, _, _) => {
                h250
            }
        });
        nat_eq(a251, a252)
    } else {
        false
    } {
        let a257 = {
            let a254 = match v0.clone() {
                Adt1::C0(_, h253, _, _, _, _, _, _, _, _, _) => {
                    h253
                }
            };
            let a255 = v3;
            let a256 = 1u64;
            nat_rem(a254, a255, a256)
        };
        let a258 = 0u64;
        if nat_eq(a257, a258) {
            let a263 = {
                let a260 = match v2.clone() {
                    Adt1::C0(_, h259, _, _, _, _, _, _, _, _, _) => {
                        h259
                    }
                };
                let a261 = v3;
                let a262 = 1u64;
                nat_rem(a260, a261, a262)
            };
            let a264 = 0u64;
            nat_eq(a263, a264)
        } else {
            false
        }
    } else {
        false
    } {
        let a267 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h265, _, _, _, _, _) => {
                h265
            }
        };
        let a268 = f20(match v2.clone() {
            Adt1::C0(h266, _, _, _, _, _, _, _, _, _, _) => {
                h266
            }
        });
        nat_eq(a267, a268)
    } else {
        false
    } {
        let a269 = f31(v0.clone())?;
        let a270 = f31(v2.clone())?;
        nat_eq(a269, a270)
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a271 = f31(v0.clone())?;
            let a272 = v3;
            let a273 = 0u64;
            nat_quot(a271, a272, a273)
        }, 3u64, f37(match v2.clone() {
            Adt1::C0(h274, _, _, _, _, _, _, _, _, _, _) => {
                h274
            }
        }), 33u64, 0u64, 0u64, 0u64, v0, v1, v2, false)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f7(v0: u64, v1: u64, v2: Adt1, v3: Adt1) -> R<Adt3> {
    let a276 = match v2.clone() {
        Adt1::C0(h275, _, _, _, _, _, _, _, _, _, _) => {
            h275
        }
    };
    let a277 = 0u64;
    if if if if if if if if if if if nat_eq(a276, a277) {
        let a279 = match v3.clone() {
            Adt1::C0(h278, _, _, _, _, _, _, _, _, _, _) => {
                h278
            }
        };
        let a280 = 26u64;
        nat_eq(a279, a280)
    } else {
        false
    } {
        let a282 = match v2.clone() {
            Adt1::C0(_, h281, _, _, _, _, _, _, _, _, _) => {
                h281
            }
        };
        let a283 = 2147483647u64;
        nat_le(a282, a283)
    } else {
        false
    } {
        let a286 = match v3.clone() {
            Adt1::C0(_, h284, _, _, _, _, _, _, _, _, _) => {
                h284
            }
        };
        let a287 = match v2.clone() {
            Adt1::C0(_, h285, _, _, _, _, _, _, _, _, _) => {
                h285
            }
        };
        nat_le(a286, a287)
    } else {
        false
    } {
        let a290 = match v2.clone() {
            Adt1::C0(_, _, h288, _, _, _, _, _, _, _, _) => {
                h288
            }
        };
        let a291 = match v3.clone() {
            Adt1::C0(_, _, h289, _, _, _, _, _, _, _, _) => {
                h289
            }
        };
        nat_eq(a290, a291)
    } else {
        false
    } {
        let a294 = match v2.clone() {
            Adt1::C0(_, _, _, h292, _, _, _, _, _, _, _) => {
                h292
            }
        };
        let a295 = match v3.clone() {
            Adt1::C0(_, _, _, h293, _, _, _, _, _, _, _) => {
                h293
            }
        };
        nat_eq(a294, a295)
    } else {
        false
    } {
        let a298 = match v2.clone() {
            Adt1::C0(_, _, _, _, h296, _, _, _, _, _, _) => {
                h296
            }
        };
        let a299 = match v3.clone() {
            Adt1::C0(_, _, _, _, h297, _, _, _, _, _, _) => {
                h297
            }
        };
        nat_eq(a298, a299)
    } else {
        false
    } {
        let a300 = v0;
        let a301 = 67u64;
        if nat_eq(a300, a301) {
            true
        } else {
            let a302 = v1;
            let a303 = 1u64;
            if nat_le(a302, a303) {
                let a306 = match v3.clone() {
                    Adt1::C0(_, h304, _, _, _, _, _, _, _, _, _) => {
                        h304
                    }
                };
                let a307 = match v2.clone() {
                    Adt1::C0(_, h305, _, _, _, _, _, _, _, _, _) => {
                        h305
                    }
                };
                nat_eq(a306, a307)
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
            let a313 = match v3.clone() {
                Adt1::C0(_, _, h308, _, _, _, _, _, _, _, _) => {
                    h308
                }
            };
            let a314 = {
                let a311 = match v3.clone() {
                    Adt1::C0(_, _, _, h309, _, _, _, _, _, _, _) => {
                        h309
                    }
                };
                let a312 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h310, _, _, _, _, _, _) => {
                        h310
                    }
                };
                nat_mul(a311, a312)?
            };
            nat_mul(a313, a314)?
        }, 3u64, None::<u64>, v0, {
            let a315 = v0;
            let a316 = 67u64;
            if nat_eq(a315, a316) {
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
    let a317 = v0;
    let a318 = 6u64;
    nat_le(a317, a318)
}

pub fn f9(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1) -> R<Adt3> {
    let a319 = v2;
    let a320 = 1u64;
    if if if if if if if if if if if if if if if nat_le(a319, a320) {
        let a322 = 0u64;
        let a323 = f38(match v5.clone() {
            Adt1::C0(h321, _, _, _, _, _, _, _, _, _, _) => {
                h321
            }
        });
        nat_lt(a322, a323)
    } else {
        false
    } {
        let a326 = match v5.clone() {
            Adt1::C0(h324, _, _, _, _, _, _, _, _, _, _) => {
                h324
            }
        };
        let a327 = match v6.clone() {
            Adt1::C0(h325, _, _, _, _, _, _, _, _, _, _) => {
                h325
            }
        };
        nat_eq(a326, a327)
    } else {
        false
    } {
        let a330 = match v5.clone() {
            Adt1::C0(h328, _, _, _, _, _, _, _, _, _, _) => {
                h328
            }
        };
        let a331 = match v7.clone() {
            Adt1::C0(h329, _, _, _, _, _, _, _, _, _, _) => {
                h329
            }
        };
        nat_eq(a330, a331)
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
        let a334 = match v5.clone() {
            Adt1::C0(_, _, h332, _, _, _, _, _, _, _, _) => {
                h332
            }
        };
        let a335 = match v7.clone() {
            Adt1::C0(_, _, h333, _, _, _, _, _, _, _, _) => {
                h333
            }
        };
        nat_eq(a334, a335)
    } else {
        false
    } {
        let a338 = match v5.clone() {
            Adt1::C0(_, _, _, h336, _, _, _, _, _, _, _) => {
                h336
            }
        };
        let a339 = match v7.clone() {
            Adt1::C0(_, _, _, h337, _, _, _, _, _, _, _) => {
                h337
            }
        };
        nat_eq(a338, a339)
    } else {
        false
    } {
        let a342 = match v5.clone() {
            Adt1::C0(_, _, _, _, h340, _, _, _, _, _, _) => {
                h340
            }
        };
        let a343 = match v7.clone() {
            Adt1::C0(_, _, _, _, h341, _, _, _, _, _, _) => {
                h341
            }
        };
        nat_eq(a342, a343)
    } else {
        false
    } {
        let a350 = match v7.clone() {
            Adt1::C0(_, h344, _, _, _, _, _, _, _, _, _) => {
                h344
            }
        };
        let a351 = if v1 {
            match v5.clone() {
                Adt1::C0(_, h345, _, _, _, _, _, _, _, _, _) => {
                    h345
                }
            }
        } else {
            let a347 = match v5.clone() {
                Adt1::C0(_, h346, _, _, _, _, _, _, _, _, _) => {
                    h346
                }
            };
            let a348 = 2u64;
            let a349 = 0u64;
            nat_quot(a347, a348, a349)
        };
        nat_eq(a350, a351)
    } else {
        false
    } {
        let a353 = match v7.clone() {
            Adt1::C0(_, h352, _, _, _, _, _, _, _, _, _) => {
                h352
            }
        };
        let a354 = 2147483647u64;
        nat_le(a353, a354)
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
            let a357 = 1u64;
            let a358 = {
                let a355 = f42(v0);
                let a356 = f31(v7.clone())?;
                nat_mul(a355, a356)?
            };
            nat_add(a357, a358)?
        } else {
            let a359 = 4u64;
            let a360 = f31(v7.clone())?;
            nat_mul(a359, a360)?
        }, if f14(v0) {
            4u64
        } else {
            3u64
        }, None::<u64>, if v1 {
            98u64
        } else {
            let a361 = 99u64;
            let a362 = v2;
            nat_add(a361, a362)?
        }, v0, v3, v4, v5, v6, v7, false)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f10(v0: u64, v1: Adt2) -> R<Adt3> {
    let a364 = v0;
    let a365 = match v1.clone() {
        Adt2::C0(h363, _, _, _, _, _, _, _, _, _, _) => {
            h363
        }
    };
    if nat_lt(a364, a365) {
        let v2: u64 = {
            let a369 = v0;
            let a370 = {
                let m368 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h367, _, _, _) => {
                        h367
                    }
                };
                match m368 {
                    Adt1::C0(_, h366, _, _, _, _, _, _, _, _, _) => {
                        h366
                    }
                }
            };
            nat_mul(a369, a370)?
        };
        Ok(Adt3::C7(f43(v2, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h371, _, _, _) => {
                h371
            }
        })?, f44(v2, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h372, _, _, _) => {
                h372
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h373, _, _) => {
                h373
            }
        })?, f43(v2, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h374, _) => {
                h374
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f11(v0: u64, v1: Adt2) -> R<Adt3> {
    let a376 = v0;
    let a377 = match v1.clone() {
        Adt2::C0(h375, _, _, _, _, _, _, _, _, _, _) => {
            h375
        }
    };
    if nat_lt(a376, a377) {
        let v2: Adt1 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h378, _) => {
                h378
            }
        };
        let v3: u64 = f21(match v2.clone() {
            Adt1::C0(h379, _, _, _, _, _, _, _, _, _, _) => {
                h379
            }
        });
        let v4: u64 = {
            let a381 = match v2.clone() {
                Adt1::C0(_, h380, _, _, _, _, _, _, _, _, _) => {
                    h380
                }
            };
            let a382 = v3;
            let a383 = 0u64;
            nat_quot(a381, a382, a383)
        };
        let v5: u64 = {
            let a384 = v0;
            let a385 = v4;
            let a386 = 0u64;
            nat_quot(a384, a385, a386)
        };
        let v6: u64 = {
            let a388 = v5;
            let a389 = match v2.clone() {
                Adt1::C0(_, _, h387, _, _, _, _, _, _, _, _) => {
                    h387
                }
            };
            let a390 = 0u64;
            nat_quot(a388, a389, a390)
        };
        Ok(Adt3::C6(f43({
            let a391 = v0;
            let a392 = v3;
            nat_mul(a391, a392)?
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, h393, _, _, _) => {
                h393
            }
        })?, f45({
            let a394 = v0;
            let a395 = v4;
            let a396 = 0u64;
            nat_rem(a394, a395, a396)
        }, {
            let a398 = v5;
            let a399 = match v2.clone() {
                Adt1::C0(_, _, h397, _, _, _, _, _, _, _, _) => {
                    h397
                }
            };
            let a400 = 0u64;
            nat_rem(a398, a399, a400)
        }, {
            let a402 = v6;
            let a403 = match v2.clone() {
                Adt1::C0(_, _, _, h401, _, _, _, _, _, _, _) => {
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
            nat_quot(a406, a407, a408)
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, h409, _, _, _, _, _) => {
                h409
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h410, _, _, _, _) => {
                h410
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h411, _, _, _) => {
                h411
            }
        }, match v2 {
            Adt1::C0(_, _, _, _, _, _, _, _, h412, _, _) => {
                h412
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f12(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h413, _, _, _) => {
            h413
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h414, _) => {
            h414
        }
    };
    let a416 = v0;
    let a417 = match v1.clone() {
        Adt2::C0(h415, _, _, _, _, _, _, _, _, _, _) => {
            h415
        }
    };
    if nat_lt(a416, a417) {
        Ok(Adt3::C5({
            let a419 = v0;
            let a420 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, _, h418, _, _, _, _) => {
                    h418
                }
            };
            nat_mul(a419, a420)?
        }, {
            let a422 = v0;
            let a423 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h421, _, _, _, _) => {
                    h421
                }
            };
            nat_mul(a422, a423)?
        }, match v2 {
            Adt1::C0(_, h424, _, _, _, _, _, _, _, _, _) => {
                h424
            }
        }, match v3 {
            Adt1::C0(_, h425, _, _, _, _, _, _, _, _, _) => {
                h425
            }
        }, {
            let a427 = match v1 {
                Adt2::C0(_, _, _, _, h426, _, _, _, _, _, _) => {
                    h426
                }
            };
            let a428 = 1u64;
            nat_eq(a427, a428)
        }))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f13(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let a430 = v0;
    let a431 = match v1.clone() {
        Adt2::C0(h429, _, _, _, _, _, _, _, _, _, _) => {
            h429
        }
    };
    if nat_lt(a430, a431) {
        let v3: u64 = {
            let a433 = 2u64;
            let a434 = f31(match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h432, _, _) => {
                    h432
                }
            })?;
            nat_mul(a433, a434)?
        };
        let a435 = v0;
        let a436 = v3;
        if nat_lt(a435, a436) {
            f46(v0, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h437, _, _) => {
                    h437
                }
            }, {
                let m440 = match v1 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h439, _) => {
                        h439
                    }
                };
                match m440 {
                    Adt1::C0(_, _, h438, _, _, _, _, _, _, _, _) => {
                        h438
                    }
                }
            }, v2)
        } else {
            f47({
                let a441 = v0;
                let a442 = v3;
                nat_sub(a441, a442)
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h443, _, _, _) => {
                    h443
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h444, _, _) => {
                    h444
                }
            }, match v1 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h445, _) => {
                    h445
                }
            }, v2)
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f14(v0: u64) -> bool {
    let a446 = v0;
    let a447 = 3u64;
    if nat_eq(a446, a447) {
        true
    } else {
        let a448 = v0;
        let a449 = 6u64;
        nat_eq(a448, a449)
    }
}

pub fn f15(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h450, _, _, _, _) => {
            h450
        }
    };
    let a452 = v0;
    let a453 = match v1.clone() {
        Adt2::C0(h451, _, _, _, _, _, _, _, _, _, _) => {
            h451
        }
    };
    if nat_lt(a452, a453) {
        let a454 = v0;
        let a455 = 0u64;
        if nat_eq(a454, a455) {
            Ok(Adt3::C3(1u64, 6u64, 0u64, 0u64, 2u64, v3, 2u64, 0u64, 0u64, 3u64, 4u64, 4u64))
        } else {
            let v4: u64 = {
                let a456 = v0;
                let a457 = 1u64;
                nat_sub(a456, a457)
            };
            let v5: u64 = {
                let a459 = v4;
                let a460 = f42(match v1.clone() {
                    Adt2::C0(_, _, _, _, h458, _, _, _, _, _, _) => {
                        h458
                    }
                });
                let a461 = 0u64;
                nat_rem(a459, a460, a461)
            };
            let v6: u64 = {
                let a466 = 4u64;
                let a467 = {
                    let a463 = v4;
                    let a464 = f42(match v1.clone() {
                        Adt2::C0(_, _, _, _, h462, _, _, _, _, _, _) => {
                            h462
                        }
                    });
                    let a465 = 0u64;
                    nat_quot(a463, a464, a465)
                };
                nat_mul(a466, a467)?
            };
            let a468 = v5;
            let a469 = 2u64;
            if nat_lt(a468, a469) {
                f16({
                    let a470 = v6;
                    let a471 = v5;
                    nat_add(a470, a471)?
                }, v1)
            } else {
                let a472 = v5;
                let a473 = 2u64;
                if nat_eq(a472, a473) {
                    Ok(f48(0u64, 2u64, v3))
                } else {
                    let a474 = v5;
                    let a475 = 3u64;
                    if nat_eq(a474, a475) {
                        Ok(f49(0u64, 2u64, v3, {
                            let a476 = v2;
                            let a477 = 2u64;
                            nat_eq(a476, a477)
                        }))
                    } else {
                        let a478 = v5;
                        let a479 = 4u64;
                        if nat_eq(a478, a479) {
                            Ok(f48(1u64, 1u64, 3u64))
                        } else {
                            let a480 = v5;
                            let a481 = 5u64;
                            if nat_eq(a480, a481) {
                                Ok(f49(1u64, 1u64, 3u64, {
                                    let a482 = v2;
                                    let a483 = 0u64;
                                    nat_eq(a482, a483)
                                }))
                            } else {
                                let a484 = v5;
                                let a485 = 6u64;
                                if nat_eq(a484, a485) {
                                    Ok(f48(1u64, 2u64, v3))
                                } else {
                                    let a486 = v5;
                                    let a487 = 7u64;
                                    if nat_eq(a486, a487) {
                                        Ok(f49(1u64, 2u64, v3, {
                                            let a488 = v2;
                                            let a489 = 2u64;
                                            nat_eq(a488, a489)
                                        }))
                                    } else {
                                        let a491 = match v1.clone() {
                                            Adt2::C0(_, _, _, _, h490, _, _, _, _, _, _) => {
                                                h490
                                            }
                                        };
                                        let a492 = 3u64;
                                        if nat_eq(a491, a492) {
                                            let a493 = v5;
                                            let a494 = 15u64;
                                            if nat_eq(a493, a494) {
                                                f16({
                                                    let a495 = v6;
                                                    let a496 = 3u64;
                                                    nat_add(a495, a496)?
                                                }, v1)
                                            } else {
                                                Ok(f50(v5, match v1 {
                                                    Adt2::C0(_, _, _, _, _, h497, _, _, _, _, _) => {
                                                        h497
                                                    }
                                                }))
                                            }
                                        } else {
                                            let a498 = v5;
                                            let a499 = 8u64;
                                            if nat_eq(a498, a499) {
                                                Ok(Adt3::C4(2u64, {
                                                    let m502 = match v1 {
                                                        Adt2::C0(_, _, _, _, _, _, _, _, _, h501, _) => {
                                                            h501
                                                        }
                                                    };
                                                    match m502 {
                                                        Adt1::C0(h500, _, _, _, _, _, _, _, _, _, _) => {
                                                            h500
                                                        }
                                                    }
                                                }, 0u64, 1u64, 0u64))
                                            } else {
                                                f16({
                                                    let a503 = v6;
                                                    let a504 = 3u64;
                                                    nat_add(a503, a504)?
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
        Adt2::C0(_, _, _, _, _, _, _, h505, _, _, _) => {
            h505
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h506, _, _) => {
            h506
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h507, _) => {
            h507
        }
    };
    let v5: u64 = match v1.clone() {
        Adt2::C0(_, _, _, h508, _, _, _, _, _, _, _) => {
            h508
        }
    };
    let v6: u64 = {
        let a509 = v0;
        let a510 = 4u64;
        let a511 = 0u64;
        nat_rem(a509, a510, a511)
    };
    let v7: u64 = {
        let a512 = v0;
        let a513 = 4u64;
        let a514 = 0u64;
        nat_quot(a512, a513, a514)
    };
    let v8: u64 = {
        let a516 = v7;
        let a517 = match v4.clone() {
            Adt1::C0(_, h515, _, _, _, _, _, _, _, _, _) => {
                h515
            }
        };
        let a518 = 0u64;
        nat_quot(a516, a517, a518)
    };
    let v9: u64 = f38(match v4.clone() {
        Adt1::C0(h519, _, _, _, _, _, _, _, _, _, _) => {
            h519
        }
    });
    let v10: u64 = {
        let a524 = {
            let a521 = v7;
            let a522 = match v4.clone() {
                Adt1::C0(_, h520, _, _, _, _, _, _, _, _, _) => {
                    h520
                }
            };
            let a523 = 0u64;
            nat_rem(a521, a522, a523)
        };
        let a525 = v9;
        nat_mul(a524, a525)?
    };
    let v11: u64 = {
        let a527 = match v4.clone() {
            Adt1::C0(_, h526, _, _, _, _, _, _, _, _, _) => {
                h526
            }
        };
        let a528 = v9;
        nat_mul(a527, a528)?
    };
    let a530 = v0;
    let a531 = match v1.clone() {
        Adt2::C0(h529, _, _, _, _, _, _, _, _, _, _) => {
            h529
        }
    };
    if nat_lt(a530, a531) {
        let a532 = v6;
        let a533 = 0u64;
        if nat_eq(a532, a533) {
            Ok(Adt3::C3(0u64, 0u64, match v2.clone() {
                Adt1::C0(h534, _, _, _, _, _, _, _, _, _, _) => {
                    h534
                }
            }, 0u64, 0u64, {
                let a542 = {
                    let a538 = {
                        let a536 = v8;
                        let a537 = match v2 {
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
                    let a541 = 100u64;
                    if nat_eq(a540, a541) {
                        v11
                    } else {
                        0u64
                    }
                };
                nat_add(a542, a543)?
            }, 2u64, 0u64, 0u64, 0u64, v9, 4u64))
        } else {
            let a544 = v6;
            let a545 = 1u64;
            if nat_eq(a544, a545) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h546, _, _, _, _, _, _, _, _, _, _) => {
                        h546
                    }
                }, 0u64, {
                    let a547 = v5;
                    let a548 = 98u64;
                    if nat_eq(a547, a548) {
                        4u64
                    } else {
                        0u64
                    }
                }, {
                    let a556 = {
                        let a552 = {
                            let a550 = v8;
                            let a551 = match v3 {
                                Adt1::C0(_, _, _, _, _, _, h549, _, _, _, _) => {
                                    h549
                                }
                            };
                            nat_mul(a550, a551)?
                        };
                        let a553 = v10;
                        nat_add(a552, a553)?
                    };
                    let a557 = {
                        let a554 = v5;
                        let a555 = 99u64;
                        if nat_eq(a554, a555) {
                            v11
                        } else {
                            0u64
                        }
                    };
                    nat_add(a556, a557)?
                }, 2u64, 0u64, 0u64, 1u64, v9, 4u64))
            } else {
                let a558 = v6;
                let a559 = 2u64;
                if nat_eq(a558, a559) {
                    Ok(Adt3::C4(match v1 {
                        Adt2::C0(_, _, _, _, h560, _, _, _, _, _, _) => {
                            h560
                        }
                    }, match v4 {
                        Adt1::C0(h561, _, _, _, _, _, _, _, _, _, _) => {
                            h561
                        }
                    }, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt3::C3(0u64, 0u64, 0u64, match v4.clone() {
                        Adt1::C0(h562, _, _, _, _, _, _, _, _, _, _) => {
                            h562
                        }
                    }, 1u64, 0u64, 2u64, 0u64, 3u64, {
                        let a566 = {
                            let a564 = v8;
                            let a565 = match v4 {
                                Adt1::C0(_, _, _, _, _, _, h563, _, _, _, _) => {
                                    h563
                                }
                            };
                            nat_mul(a564, a565)?
                        };
                        let a567 = v10;
                        nat_add(a566, a567)?
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
        Adt2::C0(_, _, _, _, _, _, _, h568, _, _, _) => {
            h568
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h569, _, _) => {
            h569
        }
    };
    let v5: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h570, _) => {
            h570
        }
    };
    let v6: u64 = {
        let a572 = match v1.clone() {
            Adt2::C0(_, h571, _, _, _, _, _, _, _, _, _) => {
                h571
            }
        };
        let a573 = 3u64;
        if nat_eq(a572, a573) {
            0u64
        } else {
            let a574 = f31(v3.clone())?;
            let a575 = 2u64;
            nat_mul(a574, a575)?
        }
    };
    let a577 = match v1.clone() {
        Adt2::C0(h576, _, _, _, _, _, _, _, _, _, _) => {
            h576
        }
    };
    let a578 = v0;
    if nat_le(a577, a578) {
        Ok(Adt3::C1)
    } else {
        let a579 = v0;
        let a580 = v6;
        if nat_lt(a579, a580) {
            let v7: u64 = {
                let a581 = v0;
                let a582 = 2u64;
                let a583 = 0u64;
                nat_quot(a581, a582, a583)
            };
            let v8: u64 = {
                let a588 = 3u64;
                let a589 = {
                    let a585 = f43(v7, v5.clone())?;
                    let a586 = f26(match v5 {
                        Adt1::C0(h584, _, _, _, _, _, _, _, _, _, _) => {
                            h584
                        }
                    });
                    let a587 = 0u64;
                    nat_quot(a585, a586, a587)
                };
                nat_add(a588, a589)?
            };
            let a593 = {
                let a590 = v0;
                let a591 = 2u64;
                let a592 = 0u64;
                nat_rem(a590, a591, a592)
            };
            let a594 = 0u64;
            if nat_eq(a593, a594) {
                Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, v8, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
            } else {
                let a595 = v2;
                let a596 = 0u64;
                if nat_eq(a595, a596) {
                    Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 2u64, 1u64, 2u64, 0u64, 0u64, v8, 8u64, 8u64))
                } else {
                    Ok(Adt3::C1)
                }
            }
        } else {
            let v9: u64 = {
                let a597 = v0;
                let a598 = v6;
                nat_sub(a597, a598)
            };
            let v10: u64 = f32(match v3.clone() {
                Adt1::C0(h599, _, _, _, _, _, _, _, _, _, _) => {
                    h599
                }
            }, match v4.clone() {
                Adt1::C0(h600, _, _, _, _, _, _, _, _, _, _) => {
                    h600
                }
            }, match v5.clone() {
                Adt1::C0(h601, _, _, _, _, _, _, _, _, _, _) => {
                    h601
                }
            })?;
            let v11: u64 = {
                let a602 = v9;
                let a603 = v10;
                let a604 = 0u64;
                nat_quot(a602, a603, a604)
            };
            let v12: u64 = {
                let a605 = v9;
                let a606 = v10;
                let a607 = 0u64;
                nat_rem(a605, a606, a607)
            };
            let v13: u64 = {
                let a609 = match v3.clone() {
                    Adt1::C0(h608, _, _, _, _, _, _, _, _, _, _) => {
                        h608
                    }
                };
                let a610 = 0u64;
                if nat_eq(a609, a610) {
                    0u64
                } else {
                    1u64
                }
            };
            let v14: u64 = {
                let a612 = match v4.clone() {
                    Adt1::C0(h611, _, _, _, _, _, _, _, _, _, _) => {
                        h611
                    }
                };
                let a613 = 0u64;
                if nat_eq(a612, a613) {
                    0u64
                } else {
                    1u64
                }
            };
            let a614 = v12;
            let a615 = v13;
            if nat_lt(a614, a615) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h616, _, _, _, _, _, _, _, _, _, _) => {
                        h616
                    }
                }, 0u64, 0u64, f43(v11, v3.clone())?, 2u64, 0u64, 0u64, 0u64, f26(match v3 {
                    Adt1::C0(h617, _, _, _, _, _, _, _, _, _, _) => {
                        h617
                    }
                }), 4u64))
            } else {
                let a620 = v12;
                let a621 = {
                    let a618 = v13;
                    let a619 = v14;
                    nat_add(a618, a619)?
                };
                if nat_lt(a620, a621) {
                    Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                        Adt1::C0(h622, _, _, _, _, _, _, _, _, _, _) => {
                            h622
                        }
                    }, 0u64, 4u64, f44(v11, v3, v4.clone())?, 2u64, 0u64, 0u64, 1u64, f26(match v4 {
                        Adt1::C0(h623, _, _, _, _, _, _, _, _, _, _) => {
                            h623
                        }
                    }), 4u64))
                } else {
                    let a626 = v12;
                    let a627 = {
                        let a624 = v13;
                        let a625 = v14;
                        nat_add(a624, a625)?
                    };
                    if nat_eq(a626, a627) {
                        Ok(Adt3::C3(1u64, match v1 {
                            Adt2::C0(_, _, _, h628, _, _, _, _, _, _, _) => {
                                h628
                            }
                        }, 0u64, 0u64, {
                            let a629 = v13;
                            let a630 = 0u64;
                            if nat_eq(a629, a630) {
                                0u64
                            } else {
                                1u64
                            }
                        }, {
                            let a631 = v13;
                            let a632 = 0u64;
                            if nat_eq(a631, a632) {
                                f43(v11, v3.clone())?
                            } else {
                                0u64
                            }
                        }, {
                            let a633 = v14;
                            let a634 = 0u64;
                            if nat_eq(a633, a634) {
                                4u64
                            } else {
                                1u64
                            }
                        }, {
                            let a635 = v14;
                            let a636 = 0u64;
                            if nat_eq(a635, a636) {
                                f44(v11, v3, v4)?
                            } else {
                                1u64
                            }
                        }, {
                            let a638 = match v5.clone() {
                                Adt1::C0(h637, _, _, _, _, _, _, _, _, _, _) => {
                                    h637
                                }
                            };
                            let a639 = 0u64;
                            if nat_eq(a638, a639) {
                                3u64
                            } else {
                                0u64
                            }
                        }, {
                            let a641 = match v5.clone() {
                                Adt1::C0(h640, _, _, _, _, _, _, _, _, _, _) => {
                                    h640
                                }
                            };
                            let a642 = 0u64;
                            if nat_eq(a641, a642) {
                                f43(v11, v5)?
                            } else {
                                0u64
                            }
                        }, 4u64, 4u64))
                    } else {
                        Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                            Adt1::C0(h643, _, _, _, _, _, _, _, _, _, _) => {
                                h643
                            }
                        }, 1u64, 0u64, 2u64, 0u64, 3u64, f43(v11, v5.clone())?, 4u64, f26(match v5 {
                            Adt1::C0(h644, _, _, _, _, _, _, _, _, _, _) => {
                                h644
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
        let a650 = {
            let a647 = match v0.clone() {
                Adt1::C0(_, h645, _, _, _, _, _, _, _, _, _) => {
                    h645
                }
            };
            let a648 = match v1.clone() {
                Adt1::C0(_, h646, _, _, _, _, _, _, _, _, _) => {
                    h646
                }
            };
            let a649 = 1u64;
            nat_rem(a647, a648, a649)
        };
        let a651 = 0u64;
        if if if nat_eq(a650, a651) {
            let a657 = {
                let a654 = match v0.clone() {
                    Adt1::C0(_, _, h652, _, _, _, _, _, _, _, _) => {
                        h652
                    }
                };
                let a655 = match v1.clone() {
                    Adt1::C0(_, _, h653, _, _, _, _, _, _, _, _) => {
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
        } {
            let a664 = {
                let a661 = match v0.clone() {
                    Adt1::C0(_, _, _, h659, _, _, _, _, _, _, _) => {
                        h659
                    }
                };
                let a662 = match v1.clone() {
                    Adt1::C0(_, _, _, h660, _, _, _, _, _, _, _) => {
                        h660
                    }
                };
                let a663 = 1u64;
                nat_rem(a661, a662, a663)
            };
            let a665 = 0u64;
            nat_eq(a664, a665)
        } else {
            false
        } {
            let a671 = {
                let a668 = match v0 {
                    Adt1::C0(_, _, _, _, h666, _, _, _, _, _, _) => {
                        h666
                    }
                };
                let a669 = match v1 {
                    Adt1::C0(_, _, _, _, h667, _, _, _, _, _, _) => {
                        h667
                    }
                };
                let a670 = 1u64;
                nat_rem(a668, a669, a670)
            };
            let a672 = 0u64;
            nat_eq(a671, a672)
        } else {
            false
        }
    }
}

pub fn f19(v0: Adt1, v1: Adt1) -> bool {
    let a675 = match v0.clone() {
        Adt1::C0(_, h673, _, _, _, _, _, _, _, _, _) => {
            h673
        }
    };
    let a676 = match v1.clone() {
        Adt1::C0(_, h674, _, _, _, _, _, _, _, _, _) => {
            h674
        }
    };
    if if if nat_eq(a675, a676) {
        let a679 = match v0.clone() {
            Adt1::C0(_, _, h677, _, _, _, _, _, _, _, _) => {
                h677
            }
        };
        let a680 = match v1.clone() {
            Adt1::C0(_, _, h678, _, _, _, _, _, _, _, _) => {
                h678
            }
        };
        nat_eq(a679, a680)
    } else {
        false
    } {
        let a683 = match v0.clone() {
            Adt1::C0(_, _, _, h681, _, _, _, _, _, _, _) => {
                h681
            }
        };
        let a684 = match v1.clone() {
            Adt1::C0(_, _, _, h682, _, _, _, _, _, _, _) => {
                h682
            }
        };
        nat_eq(a683, a684)
    } else {
        false
    } {
        let a687 = match v0 {
            Adt1::C0(_, _, _, _, h685, _, _, _, _, _, _) => {
                h685
            }
        };
        let a688 = match v1 {
            Adt1::C0(_, _, _, _, h686, _, _, _, _, _, _) => {
                h686
            }
        };
        nat_eq(a687, a688)
    } else {
        false
    }
}

pub fn f20(v0: u64) -> u64 {
    if f2(v0) {
        let m689 = f51(v0);
        match m689 {
            Adt4::C0 => {
                0u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, _, h690) => {
                        h690
                    }
                }
            }
        }
    } else {
        let a691 = v0;
        let a692 = 26u64;
        if nat_eq(a691, a692) {
            4u64
        } else {
            f26(v0)
        }
    }
}

pub fn f21(v0: u64) -> u64 {
    if f2(v0) {
        let m693 = f51(v0);
        match m693 {
            Adt4::C0 => {
                1u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, h694, _) => {
                        h694
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
        Adt1::C0(_, h695, _, _, _, _, _, _, _, _, _) => {
            h695
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h696, _, _, _, _, _, _, _, _) => {
            h696
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h697, _, _, _, _, _, _, _) => {
            h697
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, h698, _, _, _, _, _, _) => {
            h698
        }
    })
}

pub fn f25(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<bool> {
    let a700 = match v1.clone() {
        Adt1::C0(h699, _, _, _, _, _, _, _, _, _, _) => {
            h699
        }
    };
    let a701 = 43u64;
    if nat_eq(a700, a701) {
        let a702 = v0;
        let a703 = 0u64;
        if if if if if if nat_eq(a702, a703) {
            let a705 = match v2.clone() {
                Adt1::C0(h704, _, _, _, _, _, _, _, _, _, _) => {
                    h704
                }
            };
            let a706 = 0u64;
            nat_eq(a705, a706)
        } else {
            false
        } {
            let a708 = match v3.clone() {
                Adt1::C0(h707, _, _, _, _, _, _, _, _, _, _) => {
                    h707
                }
            };
            let a709 = 0u64;
            if nat_eq(a708, a709) {
                true
            } else {
                let a711 = match v3.clone() {
                    Adt1::C0(h710, _, _, _, _, _, _, _, _, _, _) => {
                        h710
                    }
                };
                let a712 = 43u64;
                nat_eq(a711, a712)
            }
        } else {
            false
        } {
            f19(v1.clone(), v2.clone())
        } else {
            false
        } {
            let a714 = match v2 {
                Adt1::C0(_, _, _, _, _, h713, _, _, _, _, _) => {
                    h713
                }
            };
            let a715 = 4u64;
            nat_eq(a714, a715)
        } else {
            false
        } {
            let a718 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h716, _, _, _, _, _) => {
                    h716
                }
            };
            let a719 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h717, _, _, _, _) => {
                    h717
                }
            };
            if nat_le(a718, a719) {
                let a722 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, h720, _, _, _, _) => {
                        h720
                    }
                };
                let a723 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h721, _, _, _) => {
                        h721
                    }
                };
                if nat_le(a722, a723) {
                    let a726 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, h724, _, _, _) => {
                            h724
                        }
                    };
                    let a727 = match v3 {
                        Adt1::C0(_, _, _, _, _, _, _, _, h725, _, _) => {
                            h725
                        }
                    };
                    nat_le(a726, a727)
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        } {
            let a735 = {
                let a733 = match v1.clone() {
                    Adt1::C0(_, _, h728, _, _, _, _, _, _, _, _) => {
                        h728
                    }
                };
                let a734 = {
                    let a731 = match v1.clone() {
                        Adt1::C0(_, _, _, h729, _, _, _, _, _, _, _) => {
                            h729
                        }
                    };
                    let a732 = match v1 {
                        Adt1::C0(_, _, _, _, h730, _, _, _, _, _, _) => {
                            h730
                        }
                    };
                    nat_mul(a731, a732)?
                };
                nat_mul(a733, a734)?
            };
            let a736 = 2147483647u64;
            Ok(nat_le(a735, a736))
        } else {
            Ok(false)
        }
    } else {
        if f18(v1.clone(), v2.clone()) {
            Ok(f55(match v1 {
                Adt1::C0(h737, _, _, _, _, _, _, _, _, _, _) => {
                    h737
                }
            }, match v2 {
                Adt1::C0(h738, _, _, _, _, _, _, _, _, _, _) => {
                    h738
                }
            }, match v3 {
                Adt1::C0(h739, _, _, _, _, _, _, _, _, _, _) => {
                    h739
                }
            }))
        } else {
            Ok(false)
        }
    }
}

pub fn f26(v0: u64) -> u64 {
    let a740 = v0;
    let a741 = 43u64;
    if nat_eq(a740, a741) {
        1u64
    } else {
        f38(v0)
    }
}

pub fn f27(v0: Adt1) -> bool {
    let v1: u64 = f26(match v0.clone() {
        Adt1::C0(h742, _, _, _, _, _, _, _, _, _, _) => {
            h742
        }
    });
    let a747 = {
        let a744 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h743) => {
                h743
            }
        };
        let a745 = v1;
        let a746 = 1u64;
        nat_rem(a744, a745, a746)
    };
    let a748 = 0u64;
    if if nat_eq(a747, a748) {
        f56(match v0.clone() {
            Adt1::C0(_, h749, _, _, _, _, _, _, _, _, _) => {
                h749
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h750, _, _, _, _, _) => {
                h750
            }
        }, v1)
    } else {
        false
    } {
        f57(match v0.clone() {
            Adt1::C0(_, _, h751, _, _, _, _, _, _, _, _) => {
                h751
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h752, _, _, _, _, _, _, _) => {
                h752
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h753, _, _, _, _, _, _) => {
                h753
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h754, _, _, _, _) => {
                h754
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h755, _, _, _) => {
                h755
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h756, _, _) => {
                h756
            }
        }, v1)
    } else {
        false
    }
}

pub fn f28(v0: Adt1, v1: Adt1) -> R<bool> {
    let a764 = {
        let a762 = match v0.clone() {
            Adt1::C0(_, _, h757, _, _, _, _, _, _, _, _) => {
                h757
            }
        };
        let a763 = {
            let a760 = match v0.clone() {
                Adt1::C0(_, _, _, h758, _, _, _, _, _, _, _) => {
                    h758
                }
            };
            let a761 = match v0.clone() {
                Adt1::C0(_, _, _, _, h759, _, _, _, _, _, _) => {
                    h759
                }
            };
            nat_mul(a760, a761)?
        };
        nat_mul(a762, a763)?
    };
    let a765 = 9223372036854775807u64;
    if nat_le(a764, a765) {
        if f35(v1.clone()) {
            let a767 = match v1 {
                Adt1::C0(_, h766, _, _, _, _, _, _, _, _, _) => {
                    h766
                }
            };
            let a768 = 2147483647u64;
            Ok(nat_le(a767, a768))
        } else {
            let a770 = match v0 {
                Adt1::C0(_, h769, _, _, _, _, _, _, _, _, _) => {
                    h769
                }
            };
            let a771 = 2147483647u64;
            Ok(nat_le(a770, a771))
        }
    } else {
        Ok(false)
    }
}

pub fn f29(v0: Adt1) -> R<bool> {
    f23({
        let a774 = match v0.clone() {
            Adt1::C0(_, h772, _, _, _, _, _, _, _, _, _) => {
                h772
            }
        };
        let a775 = f26(match v0.clone() {
            Adt1::C0(h773, _, _, _, _, _, _, _, _, _, _) => {
                h773
            }
        });
        nat_mul(a774, a775)?
    }, match v0.clone() {
        Adt1::C0(_, _, h776, _, _, _, _, _, _, _, _) => {
            h776
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h777, _, _, _, _, _, _, _) => {
            h777
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h778, _, _, _, _, _, _) => {
            h778
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h779, _, _, _, _) => {
            h779
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h780, _, _, _) => {
            h780
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h781, _, _) => {
            h781
        }
    })
}

pub fn f30(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a788 = {
        let a782 = v0;
        let a783 = v2;
        nat_mul(a782, a783)?
    };
    let a789 = {
        let a784 = v1;
        let a785 = 0u64;
        if nat_eq(a784, a785) {
            0u64
        } else {
            let a786 = v0;
            let a787 = 2u64;
            nat_mul(a786, a787)?
        }
    };
    nat_add(a788, a789)
}

pub fn f31(v0: Adt1) -> R<u64> {
    let a798 = {
        let a792 = match v0.clone() {
            Adt1::C0(_, h790, _, _, _, _, _, _, _, _, _) => {
                h790
            }
        };
        let a793 = match v0.clone() {
            Adt1::C0(_, _, h791, _, _, _, _, _, _, _, _) => {
                h791
            }
        };
        nat_mul(a792, a793)?
    };
    let a799 = {
        let a796 = match v0.clone() {
            Adt1::C0(_, _, _, h794, _, _, _, _, _, _, _) => {
                h794
            }
        };
        let a797 = match v0 {
            Adt1::C0(_, _, _, _, h795, _, _, _, _, _, _) => {
                h795
            }
        };
        nat_mul(a796, a797)?
    };
    nat_mul(a798, a799)
}

pub fn f32(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a810 = {
        let a806 = {
            let a802 = 1u64;
            let a803 = {
                let a800 = v0;
                let a801 = 0u64;
                if nat_eq(a800, a801) {
                    0u64
                } else {
                    1u64
                }
            };
            nat_add(a802, a803)?
        };
        let a807 = {
            let a804 = v1;
            let a805 = 0u64;
            if nat_eq(a804, a805) {
                0u64
            } else {
                1u64
            }
        };
        nat_add(a806, a807)?
    };
    let a811 = {
        let a808 = v2;
        let a809 = 0u64;
        if nat_eq(a808, a809) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a810, a811)
}

pub fn f33(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f19(v0.clone(), v1.clone()) {
        let a814 = match v0.clone() {
            Adt1::C0(h812, _, _, _, _, _, _, _, _, _, _) => {
                h812
            }
        };
        let a815 = match v1.clone() {
            Adt1::C0(h813, _, _, _, _, _, _, _, _, _, _) => {
                h813
            }
        };
        nat_eq(a814, a815)
    } else {
        false
    } {
        let a818 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h816, _, _, _, _, _) => {
                h816
            }
        };
        let a819 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h817, _, _, _, _, _) => {
                h817
            }
        };
        nat_eq(a818, a819)
    } else {
        false
    } {
        let a822 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h820, _, _, _, _) => {
                h820
            }
        };
        let a823 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h821, _, _, _, _) => {
                h821
            }
        };
        nat_eq(a822, a823)
    } else {
        false
    } {
        let a826 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h824, _, _, _) => {
                h824
            }
        };
        let a827 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h825, _, _, _) => {
                h825
            }
        };
        nat_eq(a826, a827)
    } else {
        false
    } {
        let a830 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h828, _, _) => {
                h828
            }
        };
        let a831 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h829, _, _) => {
                h829
            }
        };
        nat_eq(a830, a831)
    } else {
        false
    } {
        let a833 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h832) => {
                h832
            }
        };
        let a834 = 0u64;
        if if nat_eq(a833, a834) {
            true
        } else {
            let a836 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h835) => {
                    h835
                }
            };
            let a837 = 0u64;
            nat_eq(a836, a837)
        } {
            true
        } else {
            let a840 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h838) => {
                    h838
                }
            };
            let a841 = match v1 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h839) => {
                    h839
                }
            };
            nat_eq(a840, a841)
        }
    } else {
        false
    }
}

pub fn f34(v0: u64) -> u64 {
    let a842 = v0;
    let a843 = 26u64;
    if nat_eq(a842, a843) {
        4u64
    } else {
        let a844 = v0;
        let a845 = 27u64;
        if nat_eq(a844, a845) {
            8u64
        } else {
            0u64
        }
    }
}

pub fn f35(v0: Adt1) -> bool {
    let a847 = match v0.clone() {
        Adt1::C0(_, h846, _, _, _, _, _, _, _, _, _) => {
            h846
        }
    };
    let a848 = 1u64;
    if nat_eq(a847, a848) {
        true
    } else {
        let a851 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h849, _, _, _, _, _) => {
                h849
            }
        };
        let a852 = f26(match v0 {
            Adt1::C0(h850, _, _, _, _, _, _, _, _, _, _) => {
                h850
            }
        });
        nat_eq(a851, a852)
    }
}

pub fn f36(v0: Adt1, v1: u64) -> R<u64> {
    let a858 = {
        let a854 = match v0.clone() {
            Adt1::C0(_, h853, _, _, _, _, _, _, _, _, _) => {
                h853
            }
        };
        let a855 = f21(v1);
        let a856 = 0u64;
        nat_quot(a854, a855, a856)
    };
    let a859 = f58(match v0 {
        Adt1::C0(h857, _, _, _, _, _, _, _, _, _, _) => {
            h857
        }
    }, v1);
    nat_mul(a858, a859)
}

pub fn f37(v0: u64) -> Option<u64> {
    let a860 = v0;
    let a861 = 16u64;
    if if if nat_eq(a860, a861) {
        true
    } else {
        let a862 = v0;
        let a863 = 17u64;
        nat_eq(a862, a863)
    } {
        true
    } else {
        let a864 = v0;
        let a865 = 19u64;
        nat_eq(a864, a865)
    } {
        Some(1065353216u64)
    } else {
        None::<u64>
    }
}

pub fn f38(v0: u64) -> u64 {
    let a866 = v0;
    let a867 = 0u64;
    if nat_eq(a866, a867) {
        4u64
    } else {
        let a868 = v0;
        let a869 = 1u64;
        if if nat_eq(a868, a869) {
            true
        } else {
            let a870 = v0;
            let a871 = 30u64;
            nat_eq(a870, a871)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f39(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a873 = match v0.clone() {
            Adt1::C0(_, h872, _, _, _, _, _, _, _, _, _) => {
                h872
            }
        };
        let a874 = 4u64;
        nat_mul(a873, a874)?
    };
    let v2: u64 = {
        let a877 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h875, _, _, _, _) => {
                h875
            }
        };
        let a878 = match v0.clone() {
            Adt1::C0(_, _, h876, _, _, _, _, _, _, _, _) => {
                h876
            }
        };
        nat_mul(a877, a878)?
    };
    let v3: u64 = {
        let a880 = v2;
        let a881 = match v0.clone() {
            Adt1::C0(_, _, _, h879, _, _, _, _, _, _, _) => {
                h879
            }
        };
        nat_mul(a880, a881)?
    };
    let a883 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h882, _, _, _, _, _) => {
            h882
        }
    };
    let a884 = 4u64;
    if if if if nat_eq(a883, a884) {
        let a889 = {
            let a886 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h885) => {
                    h885
                }
            };
            let a887 = 4u64;
            let a888 = 1u64;
            nat_rem(a886, a887, a888)
        };
        let a890 = 0u64;
        nat_eq(a889, a890)
    } else {
        false
    } {
        let a898 = {
            let a896 = match v0.clone() {
                Adt1::C0(_, _, h891, _, _, _, _, _, _, _, _) => {
                    h891
                }
            };
            let a897 = {
                let a894 = match v0.clone() {
                    Adt1::C0(_, _, _, h892, _, _, _, _, _, _, _) => {
                        h892
                    }
                };
                let a895 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h893, _, _, _, _, _, _) => {
                        h893
                    }
                };
                nat_mul(a894, a895)?
            };
            nat_mul(a896, a897)?
        };
        let a899 = 1u64;
        if nat_le(a898, a899) {
            true
        } else {
            let a901 = v1;
            let a902 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h900, _, _, _, _) => {
                    h900
                }
            };
            if nat_le(a901, a902) {
                let a907 = {
                    let a904 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, h903, _, _, _, _) => {
                            h903
                        }
                    };
                    let a905 = 4u64;
                    let a906 = 1u64;
                    nat_rem(a904, a905, a906)
                };
                let a908 = 0u64;
                nat_eq(a907, a908)
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a910 = match v0.clone() {
            Adt1::C0(_, _, _, h909, _, _, _, _, _, _, _) => {
                h909
            }
        };
        let a911 = 1u64;
        if nat_le(a910, a911) {
            true
        } else {
            let a913 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h912, _, _, _) => {
                    h912
                }
            };
            let a914 = v2;
            nat_eq(a913, a914)
        }
    } else {
        false
    } {
        let a916 = match v0.clone() {
            Adt1::C0(_, _, _, _, h915, _, _, _, _, _, _) => {
                h915
            }
        };
        let a917 = 1u64;
        if nat_le(a916, a917) {
            Ok(true)
        } else {
            let a919 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, h918, _, _) => {
                    h918
                }
            };
            let a920 = v3;
            Ok(nat_eq(a919, a920))
        }
    } else {
        Ok(false)
    }
}

pub fn f40(v0: Adt1, v1: Adt1) -> R<bool> {
    let a922 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h921, _) => {
            h921
        }
    };
    let a923 = 0u64;
    if nat_eq(a922, a923) {
        Ok(true)
    } else {
        let a925 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h924, _) => {
                h924
            }
        };
        let a926 = 0u64;
        if nat_eq(a925, a926) {
            Ok(true)
        } else {
            let a928 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h927) => {
                    h927
                }
            };
            let a929 = 0u64;
            if nat_eq(a928, a929) {
                Ok(true)
            } else {
                let a931 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h930) => {
                        h930
                    }
                };
                let a932 = 0u64;
                if nat_eq(a931, a932) {
                    Ok(true)
                } else {
                    let a938 = {
                        let a935 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h933) => {
                                h933
                            }
                        };
                        let a936 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h934, _) => {
                                h934
                            }
                        };
                        nat_add(a935, a936)?
                    };
                    let a939 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h937) => {
                            h937
                        }
                    };
                    if nat_le(a938, a939) {
                        Ok(true)
                    } else {
                        let a945 = {
                            let a942 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h940) => {
                                    h940
                                }
                            };
                            let a943 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h941, _) => {
                                    h941
                                }
                            };
                            nat_add(a942, a943)?
                        };
                        let a946 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h944) => {
                                h944
                            }
                        };
                        Ok(nat_le(a945, a946))
                    }
                }
            }
        }
    }
}

pub fn f41(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a949 = match v0.clone() {
            Adt1::C0(_, _, h947, _, _, _, _, _, _, _, _) => {
                h947
            }
        };
        let a950 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h948, _, _, _, _) => {
                h948
            }
        };
        nat_mul(a949, a950)?
    };
    if if f35(v0.clone()) {
        let a952 = match v0.clone() {
            Adt1::C0(_, _, _, h951, _, _, _, _, _, _, _) => {
                h951
            }
        };
        let a953 = 1u64;
        if nat_eq(a952, a953) {
            true
        } else {
            let a955 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h954, _, _, _) => {
                    h954
                }
            };
            let a956 = v1;
            nat_eq(a955, a956)
        }
    } else {
        false
    } {
        let a958 = match v0.clone() {
            Adt1::C0(_, _, _, _, h957, _, _, _, _, _, _) => {
                h957
            }
        };
        let a959 = 1u64;
        if nat_eq(a958, a959) {
            Ok(true)
        } else {
            let a964 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h960, _, _) => {
                    h960
                }
            };
            let a965 = {
                let a962 = v1;
                let a963 = match v0 {
                    Adt1::C0(_, _, _, h961, _, _, _, _, _, _, _) => {
                        h961
                    }
                };
                nat_mul(a962, a963)?
            };
            Ok(nat_eq(a964, a965))
        }
    } else {
        Ok(false)
    }
}

pub fn f42(v0: u64) -> u64 {
    let a966 = v0;
    let a967 = 3u64;
    if nat_eq(a966, a967) {
        16u64
    } else {
        10u64
    }
}

pub fn f43(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a969 = v0;
        let a970 = match v1.clone() {
            Adt1::C0(_, h968, _, _, _, _, _, _, _, _, _) => {
                h968
            }
        };
        let a971 = 0u64;
        nat_quot(a969, a970, a971)
    };
    let v3: u64 = {
        let a973 = v2;
        let a974 = match v1.clone() {
            Adt1::C0(_, _, h972, _, _, _, _, _, _, _, _) => {
                h972
            }
        };
        let a975 = 0u64;
        nat_quot(a973, a974, a975)
    };
    f45({
        let a977 = v0;
        let a978 = match v1.clone() {
            Adt1::C0(_, h976, _, _, _, _, _, _, _, _, _) => {
                h976
            }
        };
        let a979 = 0u64;
        nat_rem(a977, a978, a979)
    }, {
        let a981 = v2;
        let a982 = match v1.clone() {
            Adt1::C0(_, _, h980, _, _, _, _, _, _, _, _) => {
                h980
            }
        };
        let a983 = 0u64;
        nat_rem(a981, a982, a983)
    }, {
        let a985 = v3;
        let a986 = match v1.clone() {
            Adt1::C0(_, _, _, h984, _, _, _, _, _, _, _) => {
                h984
            }
        };
        let a987 = 0u64;
        nat_rem(a985, a986, a987)
    }, {
        let a989 = v3;
        let a990 = match v1.clone() {
            Adt1::C0(_, _, _, h988, _, _, _, _, _, _, _) => {
                h988
            }
        };
        let a991 = 0u64;
        nat_quot(a989, a990, a991)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h992, _, _, _, _, _) => {
            h992
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h993, _, _, _, _) => {
            h993
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h994, _, _, _) => {
            h994
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h995, _, _) => {
            h995
        }
    })
}

pub fn f44(v0: u64, v1: Adt1, v2: Adt1) -> R<u64> {
    let v3: u64 = {
        let a997 = v0;
        let a998 = match v1.clone() {
            Adt1::C0(_, h996, _, _, _, _, _, _, _, _, _) => {
                h996
            }
        };
        let a999 = 0u64;
        nat_quot(a997, a998, a999)
    };
    let v4: u64 = {
        let a1001 = v3;
        let a1002 = match v1.clone() {
            Adt1::C0(_, _, h1000, _, _, _, _, _, _, _, _) => {
                h1000
            }
        };
        let a1003 = 0u64;
        nat_quot(a1001, a1002, a1003)
    };
    f45({
        let a1009 = {
            let a1005 = v0;
            let a1006 = match v1.clone() {
                Adt1::C0(_, h1004, _, _, _, _, _, _, _, _, _) => {
                    h1004
                }
            };
            let a1007 = 0u64;
            nat_rem(a1005, a1006, a1007)
        };
        let a1010 = match v2.clone() {
            Adt1::C0(_, h1008, _, _, _, _, _, _, _, _, _) => {
                h1008
            }
        };
        let a1011 = 0u64;
        nat_rem(a1009, a1010, a1011)
    }, {
        let a1017 = {
            let a1013 = v3;
            let a1014 = match v1.clone() {
                Adt1::C0(_, _, h1012, _, _, _, _, _, _, _, _) => {
                    h1012
                }
            };
            let a1015 = 0u64;
            nat_rem(a1013, a1014, a1015)
        };
        let a1018 = match v2.clone() {
            Adt1::C0(_, _, h1016, _, _, _, _, _, _, _, _) => {
                h1016
            }
        };
        let a1019 = 0u64;
        nat_rem(a1017, a1018, a1019)
    }, {
        let a1025 = {
            let a1021 = v4;
            let a1022 = match v1.clone() {
                Adt1::C0(_, _, _, h1020, _, _, _, _, _, _, _) => {
                    h1020
                }
            };
            let a1023 = 0u64;
            nat_rem(a1021, a1022, a1023)
        };
        let a1026 = match v2.clone() {
            Adt1::C0(_, _, _, h1024, _, _, _, _, _, _, _) => {
                h1024
            }
        };
        let a1027 = 0u64;
        nat_rem(a1025, a1026, a1027)
    }, {
        let a1033 = {
            let a1029 = v4;
            let a1030 = match v1 {
                Adt1::C0(_, _, _, h1028, _, _, _, _, _, _, _) => {
                    h1028
                }
            };
            let a1031 = 0u64;
            nat_quot(a1029, a1030, a1031)
        };
        let a1034 = match v2.clone() {
            Adt1::C0(_, _, _, _, h1032, _, _, _, _, _, _) => {
                h1032
            }
        };
        let a1035 = 0u64;
        nat_rem(a1033, a1034, a1035)
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h1036, _, _, _, _, _) => {
            h1036
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h1037, _, _, _, _) => {
            h1037
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1038, _, _, _) => {
            h1038
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1039, _, _) => {
            h1039
        }
    })
}

pub fn f45(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a1052 = {
        let a1048 = {
            let a1044 = {
                let a1040 = v0;
                let a1041 = v4;
                nat_mul(a1040, a1041)?
            };
            let a1045 = {
                let a1042 = v1;
                let a1043 = v5;
                nat_mul(a1042, a1043)?
            };
            nat_add(a1044, a1045)?
        };
        let a1049 = {
            let a1046 = v2;
            let a1047 = v6;
            nat_mul(a1046, a1047)?
        };
        nat_add(a1048, a1049)?
    };
    let a1053 = {
        let a1050 = v3;
        let a1051 = v7;
        nat_mul(a1050, a1051)?
    };
    nat_add(a1052, a1053)
}

pub fn f46(v0: u64, v1: Adt1, v2: u64, v3: u64) -> R<Adt3> {
    let v4: u64 = {
        let a1054 = v0;
        let a1055 = 2u64;
        let a1056 = 0u64;
        nat_quot(a1054, a1055, a1056)
    };
    let v5: u64 = f34(match v1.clone() {
        Adt1::C0(h1057, _, _, _, _, _, _, _, _, _, _) => {
            h1057
        }
    });
    let a1061 = {
        let a1058 = v0;
        let a1059 = 2u64;
        let a1060 = 0u64;
        nat_rem(a1058, a1059, a1060)
    };
    let a1062 = 0u64;
    if nat_eq(a1061, a1062) {
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 4u64, f43(v4, v1)?, 2u64, 0u64, 0u64, 2u64, v5, 8u64))
    } else {
        let a1063 = v3;
        let a1064 = v2;
        if if nat_lt(a1063, a1064) {
            let a1067 = v3;
            let a1068 = {
                let a1065 = v5;
                let a1066 = 4u64;
                if nat_eq(a1065, a1066) {
                    2147483648u64
                } else {
                    9223372036854775808u64
                }
            };
            nat_lt(a1067, a1068)
        } else {
            false
        } {
            Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, {
                let a1069 = 3u64;
                let a1070 = v4;
                nat_add(a1069, a1070)?
            }, 8u64, 8u64))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f47(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64) -> R<Adt3> {
    let v5: u64 = {
        let a1072 = 1u64;
        let a1073 = f36(v1.clone(), match v3.clone() {
            Adt1::C0(h1071, _, _, _, _, _, _, _, _, _, _) => {
                h1071
            }
        })?;
        nat_add(a1072, a1073)?
    };
    let v6: u64 = {
        let a1074 = v0;
        let a1075 = v5;
        let a1076 = 0u64;
        nat_rem(a1074, a1075, a1076)
    };
    let v7: u64 = {
        let a1077 = v0;
        let a1078 = v5;
        let a1079 = 0u64;
        nat_quot(a1077, a1078, a1079)
    };
    let v8: u64 = {
        let a1081 = v7;
        let a1082 = match v1.clone() {
            Adt1::C0(_, _, h1080, _, _, _, _, _, _, _, _) => {
                h1080
            }
        };
        let a1083 = 0u64;
        nat_rem(a1081, a1082, a1083)
    };
    let v9: u64 = {
        let a1085 = v7;
        let a1086 = match v1.clone() {
            Adt1::C0(_, _, h1084, _, _, _, _, _, _, _, _) => {
                h1084
            }
        };
        let a1087 = 0u64;
        nat_quot(a1085, a1086, a1087)
    };
    let v10: u64 = {
        let a1089 = v9;
        let a1090 = match v1.clone() {
            Adt1::C0(_, _, _, h1088, _, _, _, _, _, _, _) => {
                h1088
            }
        };
        let a1091 = 0u64;
        nat_rem(a1089, a1090, a1091)
    };
    let v11: u64 = {
        let a1093 = v9;
        let a1094 = match v1.clone() {
            Adt1::C0(_, _, _, h1092, _, _, _, _, _, _, _) => {
                h1092
            }
        };
        let a1095 = 0u64;
        nat_quot(a1093, a1094, a1095)
    };
    let a1096 = v6;
    let a1097 = 0u64;
    if nat_eq(a1096, a1097) {
        let v12: u64 = {
            let a1114 = v8;
            let a1115 = {
                let a1112 = match v2.clone() {
                    Adt1::C0(_, h1098, _, _, _, _, _, _, _, _, _) => {
                        h1098
                    }
                };
                let a1113 = {
                    let a1110 = {
                        let a1100 = v10;
                        let a1101 = match v2.clone() {
                            Adt1::C0(_, _, h1099, _, _, _, _, _, _, _, _) => {
                                h1099
                            }
                        };
                        let a1102 = 0u64;
                        nat_rem(a1100, a1101, a1102)
                    };
                    let a1111 = {
                        let a1108 = match v2.clone() {
                            Adt1::C0(_, _, h1103, _, _, _, _, _, _, _, _) => {
                                h1103
                            }
                        };
                        let a1109 = {
                            let a1105 = v11;
                            let a1106 = match v2 {
                                Adt1::C0(_, _, _, h1104, _, _, _, _, _, _, _) => {
                                    h1104
                                }
                            };
                            let a1107 = 0u64;
                            nat_rem(a1105, a1106, a1107)
                        };
                        nat_mul(a1108, a1109)?
                    };
                    nat_add(a1110, a1111)?
                };
                nat_mul(a1112, a1113)?
            };
            nat_add(a1114, a1115)?
        };
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, {
            let a1116 = 3u64;
            let a1117 = v12;
            nat_add(a1116, a1117)?
        }, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
    } else {
        let a1119 = v4;
        let a1120 = match v3.clone() {
            Adt1::C0(_, _, h1118, _, _, _, _, _, _, _, _) => {
                h1118
            }
        };
        if nat_lt(a1119, a1120) {
            let v13: u64 = {
                let a1125 = {
                    let a1121 = v6;
                    let a1122 = 1u64;
                    nat_sub(a1121, a1122)
                };
                let a1126 = f58(match v1.clone() {
                    Adt1::C0(h1123, _, _, _, _, _, _, _, _, _, _) => {
                        h1123
                    }
                }, match v3.clone() {
                    Adt1::C0(h1124, _, _, _, _, _, _, _, _, _, _) => {
                        h1124
                    }
                });
                let a1127 = 0u64;
                nat_quot(a1125, a1126, a1127)
            };
            let v14: u64 = f45({
                let a1129 = v13;
                let a1130 = f21(match v3.clone() {
                    Adt1::C0(h1128, _, _, _, _, _, _, _, _, _, _) => {
                        h1128
                    }
                });
                nat_mul(a1129, a1130)?
            }, v8, v10, v11, match v1.clone() {
                Adt1::C0(_, _, _, _, _, h1131, _, _, _, _, _) => {
                    h1131
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, h1132, _, _, _, _) => {
                    h1132
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1133, _, _, _) => {
                    h1133
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1134, _, _) => {
                    h1134
                }
            })?;
            let v15: u64 = f45(v13, v4, v10, v11, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h1135, _, _, _, _, _) => {
                    h1135
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h1136, _, _, _, _) => {
                    h1136
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1137, _, _, _) => {
                    h1137
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1138, _, _) => {
                    h1138
                }
            })?;
            if if f2(match v3.clone() {
                Adt1::C0(h1139, _, _, _, _, _, _, _, _, _, _) => {
                    h1139
                }
            }) {
                true
            } else {
                let a1141 = match v3.clone() {
                    Adt1::C0(h1140, _, _, _, _, _, _, _, _, _, _) => {
                        h1140
                    }
                };
                let a1142 = 26u64;
                nat_eq(a1141, a1142)
            } {
                Ok(Adt3::C6(v14, v15))
            } else {
                Ok(f59(match v1 {
                    Adt1::C0(h1143, _, _, _, _, _, _, _, _, _, _) => {
                        h1143
                    }
                }, match v3 {
                    Adt1::C0(h1144, _, _, _, _, _, _, _, _, _, _) => {
                        h1144
                    }
                }, {
                    let a1145 = v6;
                    let a1146 = 1u64;
                    nat_sub(a1145, a1146)
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
    let a1147 = v0;
    let a1148 = 8u64;
    if nat_eq(a1147, a1148) {
        Adt3::C3(1u64, 6u64, 0u64, 0u64, 1u64, 0u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
    } else {
        let a1149 = v0;
        let a1150 = 9u64;
        if nat_eq(a1149, a1150) {
            Adt3::C3(1u64, 2u64, 0u64, 0u64, 2u64, v1, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
        } else {
            let a1151 = v0;
            let a1152 = 10u64;
            if nat_eq(a1151, a1152) {
                Adt3::C3(5u64, 16u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
            } else {
                let a1153 = v0;
                let a1154 = 11u64;
                if nat_eq(a1153, a1154) {
                    Adt3::C3(1u64, 0u64, 0u64, 0u64, 2u64, 1065353216u64, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
                } else {
                    let a1155 = v0;
                    let a1156 = 12u64;
                    if nat_eq(a1155, a1156) {
                        Adt3::C3(1u64, 3u64, 0u64, 0u64, 1u64, 0u64, 1u64, 2u64, 0u64, 0u64, 4u64, 4u64)
                    } else {
                        let a1157 = v0;
                        let a1158 = 13u64;
                        if nat_eq(a1157, a1158) {
                            Adt3::C3(1u64, 0u64, 0u64, 0u64, 1u64, 1u64, 2u64, 1065353216u64, 0u64, 1u64, 4u64, 4u64)
                        } else {
                            let a1159 = v0;
                            let a1160 = 14u64;
                            if nat_eq(a1159, a1160) {
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
    let a1161 = v0;
    let a1162 = 0u64;
    if nat_eq(a1161, a1162) {
        Adt4::C1(Adt5::C0(0u64, 1u64, 4u64))
    } else {
        let a1163 = v0;
        let a1164 = 1u64;
        if nat_eq(a1163, a1164) {
            Adt4::C1(Adt5::C0(1u64, 1u64, 2u64))
        } else {
            let a1165 = v0;
            let a1166 = 2u64;
            if nat_eq(a1165, a1166) {
                Adt4::C1(Adt5::C0(2u64, 32u64, 18u64))
            } else {
                let a1167 = v0;
                let a1168 = 3u64;
                if nat_eq(a1167, a1168) {
                    Adt4::C1(Adt5::C0(3u64, 32u64, 20u64))
                } else {
                    let a1169 = v0;
                    let a1170 = 6u64;
                    if nat_eq(a1169, a1170) {
                        Adt4::C1(Adt5::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1171 = v0;
                        let a1172 = 7u64;
                        if nat_eq(a1171, a1172) {
                            Adt4::C1(Adt5::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1173 = v0;
                            let a1174 = 8u64;
                            if nat_eq(a1173, a1174) {
                                Adt4::C1(Adt5::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1175 = v0;
                                let a1176 = 9u64;
                                if nat_eq(a1175, a1176) {
                                    Adt4::C1(Adt5::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1177 = v0;
                                    let a1178 = 10u64;
                                    if nat_eq(a1177, a1178) {
                                        Adt4::C1(Adt5::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1179 = v0;
                                        let a1180 = 11u64;
                                        if nat_eq(a1179, a1180) {
                                            Adt4::C1(Adt5::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1181 = v0;
                                            let a1182 = 12u64;
                                            if nat_eq(a1181, a1182) {
                                                Adt4::C1(Adt5::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1183 = v0;
                                                let a1184 = 13u64;
                                                if nat_eq(a1183, a1184) {
                                                    Adt4::C1(Adt5::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1185 = v0;
                                                    let a1186 = 14u64;
                                                    if nat_eq(a1185, a1186) {
                                                        Adt4::C1(Adt5::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1187 = v0;
                                                        let a1188 = 15u64;
                                                        if nat_eq(a1187, a1188) {
                                                            Adt4::C1(Adt5::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1189 = v0;
                                                            let a1190 = 16u64;
                                                            if nat_eq(a1189, a1190) {
                                                                Adt4::C1(Adt5::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1191 = v0;
                                                                let a1192 = 17u64;
                                                                if nat_eq(a1191, a1192) {
                                                                    Adt4::C1(Adt5::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1193 = v0;
                                                                    let a1194 = 18u64;
                                                                    if nat_eq(a1193, a1194) {
                                                                        Adt4::C1(Adt5::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1195 = v0;
                                                                        let a1196 = 19u64;
                                                                        if nat_eq(a1195, a1196) {
                                                                            Adt4::C1(Adt5::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1197 = v0;
                                                                            let a1198 = 20u64;
                                                                            if nat_eq(a1197, a1198) {
                                                                                Adt4::C1(Adt5::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1199 = v0;
                                                                                let a1200 = 21u64;
                                                                                if nat_eq(a1199, a1200) {
                                                                                    Adt4::C1(Adt5::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1201 = v0;
                                                                                    let a1202 = 22u64;
                                                                                    if nat_eq(a1201, a1202) {
                                                                                        Adt4::C1(Adt5::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1203 = v0;
                                                                                        let a1204 = 23u64;
                                                                                        if nat_eq(a1203, a1204) {
                                                                                            Adt4::C1(Adt5::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1205 = v0;
                                                                                            let a1206 = 24u64;
                                                                                            if nat_eq(a1205, a1206) {
                                                                                                Adt4::C1(Adt5::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1207 = v0;
                                                                                                let a1208 = 25u64;
                                                                                                if nat_eq(a1207, a1208) {
                                                                                                    Adt4::C1(Adt5::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1209 = v0;
                                                                                                    let a1210 = 26u64;
                                                                                                    if nat_eq(a1209, a1210) {
                                                                                                        Adt4::C1(Adt5::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1211 = v0;
                                                                                                        let a1212 = 27u64;
                                                                                                        if nat_eq(a1211, a1212) {
                                                                                                            Adt4::C1(Adt5::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1213 = v0;
                                                                                                            let a1214 = 28u64;
                                                                                                            if nat_eq(a1213, a1214) {
                                                                                                                Adt4::C1(Adt5::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1215 = v0;
                                                                                                                let a1216 = 29u64;
                                                                                                                if nat_eq(a1215, a1216) {
                                                                                                                    Adt4::C1(Adt5::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1217 = v0;
                                                                                                                    let a1218 = 30u64;
                                                                                                                    if nat_eq(a1217, a1218) {
                                                                                                                        Adt4::C1(Adt5::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1219 = v0;
                                                                                                                        let a1220 = 34u64;
                                                                                                                        if nat_eq(a1219, a1220) {
                                                                                                                            Adt4::C1(Adt5::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1221 = v0;
                                                                                                                            let a1222 = 35u64;
                                                                                                                            if nat_eq(a1221, a1222) {
                                                                                                                                Adt4::C1(Adt5::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1223 = v0;
                                                                                                                                let a1224 = 39u64;
                                                                                                                                if nat_eq(a1223, a1224) {
                                                                                                                                    Adt4::C1(Adt5::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1225 = v0;
                                                                                                                                    let a1226 = 40u64;
                                                                                                                                    if nat_eq(a1225, a1226) {
                                                                                                                                        Adt4::C1(Adt5::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1227 = v0;
                                                                                                                                        let a1228 = 41u64;
                                                                                                                                        if nat_eq(a1227, a1228) {
                                                                                                                                            Adt4::C1(Adt5::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1229 = v0;
                                                                                                                                            let a1230 = 42u64;
                                                                                                                                            if nat_eq(a1229, a1230) {
                                                                                                                                                Adt4::C1(Adt5::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1231 = v0;
                                                                                                                                                let a1232 = 43u64;
                                                                                                                                                if nat_eq(a1231, a1232) {
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
        let a1235 = match v0.clone() {
            Adt1::C0(h1233, _, _, _, _, _, _, _, _, _, _) => {
                h1233
            }
        };
        let a1236 = match v1.clone() {
            Adt1::C0(h1234, _, _, _, _, _, _, _, _, _, _) => {
                h1234
            }
        };
        nat_eq(a1235, a1236)
    } else {
        false
    } {
        let a1239 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1237) => {
                h1237
            }
        };
        let a1240 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1238) => {
                h1238
            }
        };
        nat_eq(a1239, a1240)
    } else {
        false
    } {
        let a1243 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1241, _, _, _, _, _) => {
                h1241
            }
        };
        let a1244 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1242, _, _, _, _, _) => {
                h1242
            }
        };
        nat_eq(a1243, a1244)
    } else {
        false
    } {
        let a1247 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1245, _, _, _, _) => {
                h1245
            }
        };
        let a1248 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1246, _, _, _, _) => {
                h1246
            }
        };
        nat_eq(a1247, a1248)
    } else {
        false
    } {
        let a1251 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1249, _, _, _) => {
                h1249
            }
        };
        let a1252 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1250, _, _, _) => {
                h1250
            }
        };
        nat_eq(a1251, a1252)
    } else {
        false
    } {
        let a1255 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1253, _, _) => {
                h1253
            }
        };
        let a1256 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1254, _, _) => {
                h1254
            }
        };
        nat_eq(a1255, a1256)
    } else {
        false
    }
}

pub fn f53(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a1257 = v1;
    let a1258 = 1u64;
    if if if nat_le(a1257, a1258) {
        true
    } else {
        let a1259 = v0;
        let a1260 = v2;
        nat_le(a1259, a1260)
    } {
        let a1261 = v3;
        let a1262 = 1u64;
        if nat_le(a1261, a1262) {
            true
        } else {
            let a1269 = {
                let a1267 = v0;
                let a1268 = {
                    let a1265 = {
                        let a1263 = v1;
                        let a1264 = 1u64;
                        nat_sub(a1263, a1264)
                    };
                    let a1266 = v2;
                    nat_mul(a1265, a1266)?
                };
                nat_add(a1267, a1268)?
            };
            let a1270 = v4;
            nat_le(a1269, a1270)
        }
    } else {
        false
    } {
        let a1271 = v5;
        let a1272 = 1u64;
        if nat_le(a1271, a1272) {
            Ok(true)
        } else {
            let a1285 = {
                let a1283 = {
                    let a1277 = v0;
                    let a1278 = {
                        let a1275 = {
                            let a1273 = v1;
                            let a1274 = 1u64;
                            nat_sub(a1273, a1274)
                        };
                        let a1276 = v2;
                        nat_mul(a1275, a1276)?
                    };
                    nat_add(a1277, a1278)?
                };
                let a1284 = {
                    let a1281 = {
                        let a1279 = v3;
                        let a1280 = 1u64;
                        nat_sub(a1279, a1280)
                    };
                    let a1282 = v4;
                    nat_mul(a1281, a1282)?
                };
                nat_add(a1283, a1284)?
            };
            let a1286 = v6;
            Ok(nat_le(a1285, a1286))
        }
    } else {
        Ok(false)
    }
}

pub fn f54(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1287 = v0;
    let a1288 = 0u64;
    if if if nat_eq(a1287, a1288) {
        true
    } else {
        let a1289 = v1;
        let a1290 = 0u64;
        nat_eq(a1289, a1290)
    } {
        true
    } else {
        let a1291 = v2;
        let a1292 = 0u64;
        nat_eq(a1291, a1292)
    } {
        true
    } else {
        let a1293 = v3;
        let a1294 = 0u64;
        nat_eq(a1293, a1294)
    }
}

pub fn f55(v0: u64, v1: u64, v2: u64) -> bool {
    let a1295 = v0;
    let a1296 = 0u64;
    if if if nat_eq(a1295, a1296) {
        let a1297 = v1;
        let a1298 = 0u64;
        nat_eq(a1297, a1298)
    } else {
        false
    } {
        let a1299 = v2;
        let a1300 = 0u64;
        nat_eq(a1299, a1300)
    } else {
        false
    } {
        true
    } else {
        let a1301 = v0;
        let a1302 = 1u64;
        if if nat_eq(a1301, a1302) {
            true
        } else {
            let a1303 = v0;
            let a1304 = 30u64;
            nat_eq(a1303, a1304)
        } {
            let a1305 = v1;
            let a1306 = v0;
            if if nat_eq(a1305, a1306) {
                let a1307 = v2;
                let a1308 = v0;
                nat_eq(a1307, a1308)
            } else {
                false
            } {
                true
            } else {
                let a1309 = v1;
                let a1310 = 0u64;
                if nat_eq(a1309, a1310) {
                    let a1311 = v2;
                    let a1312 = v0;
                    if nat_eq(a1311, a1312) {
                        true
                    } else {
                        let a1313 = v2;
                        let a1314 = 0u64;
                        nat_eq(a1313, a1314)
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
    let a1315 = v0;
    let a1316 = 1u64;
    if nat_le(a1315, a1316) {
        true
    } else {
        let a1320 = {
            let a1317 = v1;
            let a1318 = v2;
            let a1319 = 1u64;
            nat_rem(a1317, a1318, a1319)
        };
        let a1321 = 0u64;
        nat_eq(a1320, a1321)
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
    let a1322 = v0;
    let a1323 = 1u64;
    if if nat_eq(a1322, a1323) {
        let a1324 = v1;
        let a1325 = 43u64;
        nat_eq(a1324, a1325)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f59(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt3 {
    let a1326 = f58(v0, v1);
    let a1327 = 2u64;
    if nat_eq(a1326, a1327) {
        let a1331 = {
            let a1328 = v2;
            let a1329 = 2u64;
            let a1330 = 0u64;
            nat_rem(a1328, a1329, a1330)
        };
        let a1332 = 0u64;
        if nat_eq(a1331, a1332) {
            Adt3::C3(0u64, 0u64, v0, 0u64, 0u64, v3, 2u64, 0u64, 1u64, 0u64, f38(v0), 8u64)
        } else {
            Adt3::C3(0u64, 0u64, 0u64, v1, 1u64, 0u64, 2u64, 0u64, 3u64, v4, 8u64, f20(v1))
        }
    } else {
        Adt3::C3(0u64, 0u64, v0, v1, 0u64, v3, 2u64, 0u64, 3u64, v4, f38(v0), f20(v1))
    }
}

pub fn run(p0: Adt0) -> R<Adt3> {
    f0(p0)
}
