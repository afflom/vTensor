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
            f19(v1.clone(), v2.clone())
        } else {
            false
        } {
            let a718 = match v2 {
                Adt1::C0(_, _, _, _, _, h717, _, _, _, _, _) => {
                    h717
                }
            };
            let a719 = 4u64;
            nat_eq(a718, a719)
        } else {
            false
        } {
            let a722 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h720, _, _, _, _, _) => {
                    h720
                }
            };
            let a723 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h721, _, _, _, _) => {
                    h721
                }
            };
            if nat_le(a722, a723) {
                let a726 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, h724, _, _, _, _) => {
                        h724
                    }
                };
                let a727 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h725, _, _, _) => {
                        h725
                    }
                };
                if nat_le(a726, a727) {
                    let a730 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, h728, _, _, _) => {
                            h728
                        }
                    };
                    let a731 = match v3 {
                        Adt1::C0(_, _, _, _, _, _, _, _, h729, _, _) => {
                            h729
                        }
                    };
                    nat_le(a730, a731)
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        } {
            let a739 = {
                let a737 = match v1.clone() {
                    Adt1::C0(_, _, h732, _, _, _, _, _, _, _, _) => {
                        h732
                    }
                };
                let a738 = {
                    let a735 = match v1.clone() {
                        Adt1::C0(_, _, _, h733, _, _, _, _, _, _, _) => {
                            h733
                        }
                    };
                    let a736 = match v1 {
                        Adt1::C0(_, _, _, _, h734, _, _, _, _, _, _) => {
                            h734
                        }
                    };
                    nat_mul(a735, a736)?
                };
                nat_mul(a737, a738)?
            };
            let a740 = 2147483647u64;
            Ok(nat_le(a739, a740))
        } else {
            Ok(false)
        }
    } else {
        if f18(v1.clone(), v2.clone()) {
            Ok(f55(match v1 {
                Adt1::C0(h741, _, _, _, _, _, _, _, _, _, _) => {
                    h741
                }
            }, match v2 {
                Adt1::C0(h742, _, _, _, _, _, _, _, _, _, _) => {
                    h742
                }
            }, match v3 {
                Adt1::C0(h743, _, _, _, _, _, _, _, _, _, _) => {
                    h743
                }
            }))
        } else {
            Ok(false)
        }
    }
}

pub fn f26(v0: u64) -> u64 {
    let a744 = v0;
    let a745 = 43u64;
    if nat_eq(a744, a745) {
        1u64
    } else {
        f38(v0)
    }
}

pub fn f27(v0: Adt1) -> bool {
    let v1: u64 = f26(match v0.clone() {
        Adt1::C0(h746, _, _, _, _, _, _, _, _, _, _) => {
            h746
        }
    });
    let a751 = {
        let a748 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h747) => {
                h747
            }
        };
        let a749 = v1;
        let a750 = 1u64;
        nat_rem(a748, a749, a750)
    };
    let a752 = 0u64;
    if if nat_eq(a751, a752) {
        f56(match v0.clone() {
            Adt1::C0(_, h753, _, _, _, _, _, _, _, _, _) => {
                h753
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h754, _, _, _, _, _) => {
                h754
            }
        }, v1)
    } else {
        false
    } {
        f57(match v0.clone() {
            Adt1::C0(_, _, h755, _, _, _, _, _, _, _, _) => {
                h755
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h756, _, _, _, _, _, _, _) => {
                h756
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h757, _, _, _, _, _, _) => {
                h757
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h758, _, _, _, _) => {
                h758
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h759, _, _, _) => {
                h759
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h760, _, _) => {
                h760
            }
        }, v1)
    } else {
        false
    }
}

pub fn f28(v0: Adt1, v1: Adt1) -> R<bool> {
    let a768 = {
        let a766 = match v0.clone() {
            Adt1::C0(_, _, h761, _, _, _, _, _, _, _, _) => {
                h761
            }
        };
        let a767 = {
            let a764 = match v0.clone() {
                Adt1::C0(_, _, _, h762, _, _, _, _, _, _, _) => {
                    h762
                }
            };
            let a765 = match v0.clone() {
                Adt1::C0(_, _, _, _, h763, _, _, _, _, _, _) => {
                    h763
                }
            };
            nat_mul(a764, a765)?
        };
        nat_mul(a766, a767)?
    };
    let a769 = 9223372036854775807u64;
    if nat_le(a768, a769) {
        if f35(v1.clone()) {
            let a771 = match v1 {
                Adt1::C0(_, h770, _, _, _, _, _, _, _, _, _) => {
                    h770
                }
            };
            let a772 = 2147483647u64;
            Ok(nat_le(a771, a772))
        } else {
            let a774 = match v0 {
                Adt1::C0(_, h773, _, _, _, _, _, _, _, _, _) => {
                    h773
                }
            };
            let a775 = 2147483647u64;
            Ok(nat_le(a774, a775))
        }
    } else {
        Ok(false)
    }
}

pub fn f29(v0: Adt1) -> R<bool> {
    f23({
        let a778 = match v0.clone() {
            Adt1::C0(_, h776, _, _, _, _, _, _, _, _, _) => {
                h776
            }
        };
        let a779 = f26(match v0.clone() {
            Adt1::C0(h777, _, _, _, _, _, _, _, _, _, _) => {
                h777
            }
        });
        nat_mul(a778, a779)?
    }, match v0.clone() {
        Adt1::C0(_, _, h780, _, _, _, _, _, _, _, _) => {
            h780
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h781, _, _, _, _, _, _, _) => {
            h781
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h782, _, _, _, _, _, _) => {
            h782
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h783, _, _, _, _) => {
            h783
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h784, _, _, _) => {
            h784
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h785, _, _) => {
            h785
        }
    })
}

pub fn f30(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a792 = {
        let a786 = v0;
        let a787 = v2;
        nat_mul(a786, a787)?
    };
    let a793 = {
        let a788 = v1;
        let a789 = 0u64;
        if nat_eq(a788, a789) {
            0u64
        } else {
            let a790 = v0;
            let a791 = 2u64;
            nat_mul(a790, a791)?
        }
    };
    nat_add(a792, a793)
}

pub fn f31(v0: Adt1) -> R<u64> {
    let a802 = {
        let a796 = match v0.clone() {
            Adt1::C0(_, h794, _, _, _, _, _, _, _, _, _) => {
                h794
            }
        };
        let a797 = match v0.clone() {
            Adt1::C0(_, _, h795, _, _, _, _, _, _, _, _) => {
                h795
            }
        };
        nat_mul(a796, a797)?
    };
    let a803 = {
        let a800 = match v0.clone() {
            Adt1::C0(_, _, _, h798, _, _, _, _, _, _, _) => {
                h798
            }
        };
        let a801 = match v0 {
            Adt1::C0(_, _, _, _, h799, _, _, _, _, _, _) => {
                h799
            }
        };
        nat_mul(a800, a801)?
    };
    nat_mul(a802, a803)
}

pub fn f32(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a814 = {
        let a810 = {
            let a806 = 1u64;
            let a807 = {
                let a804 = v0;
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
            let a808 = v1;
            let a809 = 0u64;
            if nat_eq(a808, a809) {
                0u64
            } else {
                1u64
            }
        };
        nat_add(a810, a811)?
    };
    let a815 = {
        let a812 = v2;
        let a813 = 0u64;
        if nat_eq(a812, a813) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a814, a815)
}

pub fn f33(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f19(v0.clone(), v1.clone()) {
        let a818 = match v0.clone() {
            Adt1::C0(h816, _, _, _, _, _, _, _, _, _, _) => {
                h816
            }
        };
        let a819 = match v1.clone() {
            Adt1::C0(h817, _, _, _, _, _, _, _, _, _, _) => {
                h817
            }
        };
        nat_eq(a818, a819)
    } else {
        false
    } {
        let a822 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h820, _, _, _, _, _) => {
                h820
            }
        };
        let a823 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h821, _, _, _, _, _) => {
                h821
            }
        };
        nat_eq(a822, a823)
    } else {
        false
    } {
        let a826 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h824, _, _, _, _) => {
                h824
            }
        };
        let a827 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h825, _, _, _, _) => {
                h825
            }
        };
        nat_eq(a826, a827)
    } else {
        false
    } {
        let a830 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h828, _, _, _) => {
                h828
            }
        };
        let a831 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h829, _, _, _) => {
                h829
            }
        };
        nat_eq(a830, a831)
    } else {
        false
    } {
        let a834 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h832, _, _) => {
                h832
            }
        };
        let a835 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h833, _, _) => {
                h833
            }
        };
        nat_eq(a834, a835)
    } else {
        false
    } {
        let a837 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h836) => {
                h836
            }
        };
        let a838 = 0u64;
        if if nat_eq(a837, a838) {
            true
        } else {
            let a840 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h839) => {
                    h839
                }
            };
            let a841 = 0u64;
            nat_eq(a840, a841)
        } {
            true
        } else {
            let a844 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h842) => {
                    h842
                }
            };
            let a845 = match v1 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h843) => {
                    h843
                }
            };
            nat_eq(a844, a845)
        }
    } else {
        false
    }
}

pub fn f34(v0: u64) -> u64 {
    let a846 = v0;
    let a847 = 26u64;
    if nat_eq(a846, a847) {
        4u64
    } else {
        let a848 = v0;
        let a849 = 27u64;
        if nat_eq(a848, a849) {
            8u64
        } else {
            0u64
        }
    }
}

pub fn f35(v0: Adt1) -> bool {
    let a851 = match v0.clone() {
        Adt1::C0(_, h850, _, _, _, _, _, _, _, _, _) => {
            h850
        }
    };
    let a852 = 1u64;
    if nat_eq(a851, a852) {
        true
    } else {
        let a855 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h853, _, _, _, _, _) => {
                h853
            }
        };
        let a856 = f26(match v0 {
            Adt1::C0(h854, _, _, _, _, _, _, _, _, _, _) => {
                h854
            }
        });
        nat_eq(a855, a856)
    }
}

pub fn f36(v0: Adt1, v1: u64) -> R<u64> {
    let a862 = {
        let a858 = match v0.clone() {
            Adt1::C0(_, h857, _, _, _, _, _, _, _, _, _) => {
                h857
            }
        };
        let a859 = f21(v1);
        let a860 = 0u64;
        nat_quot(a858, a859, a860)
    };
    let a863 = f58(match v0 {
        Adt1::C0(h861, _, _, _, _, _, _, _, _, _, _) => {
            h861
        }
    }, v1);
    nat_mul(a862, a863)
}

pub fn f37(v0: u64) -> Option<u64> {
    let a864 = v0;
    let a865 = 16u64;
    if if if nat_eq(a864, a865) {
        true
    } else {
        let a866 = v0;
        let a867 = 17u64;
        nat_eq(a866, a867)
    } {
        true
    } else {
        let a868 = v0;
        let a869 = 19u64;
        nat_eq(a868, a869)
    } {
        Some(1065353216u64)
    } else {
        None::<u64>
    }
}

pub fn f38(v0: u64) -> u64 {
    let a870 = v0;
    let a871 = 0u64;
    if nat_eq(a870, a871) {
        4u64
    } else {
        let a872 = v0;
        let a873 = 1u64;
        if if nat_eq(a872, a873) {
            true
        } else {
            let a874 = v0;
            let a875 = 30u64;
            nat_eq(a874, a875)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f39(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a877 = match v0.clone() {
            Adt1::C0(_, h876, _, _, _, _, _, _, _, _, _) => {
                h876
            }
        };
        let a878 = 4u64;
        nat_mul(a877, a878)?
    };
    let v2: u64 = {
        let a881 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h879, _, _, _, _) => {
                h879
            }
        };
        let a882 = match v0.clone() {
            Adt1::C0(_, _, h880, _, _, _, _, _, _, _, _) => {
                h880
            }
        };
        nat_mul(a881, a882)?
    };
    let v3: u64 = {
        let a884 = v2;
        let a885 = match v0.clone() {
            Adt1::C0(_, _, _, h883, _, _, _, _, _, _, _) => {
                h883
            }
        };
        nat_mul(a884, a885)?
    };
    let a887 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h886, _, _, _, _, _) => {
            h886
        }
    };
    let a888 = 4u64;
    if if if if nat_eq(a887, a888) {
        let a893 = {
            let a890 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h889) => {
                    h889
                }
            };
            let a891 = 4u64;
            let a892 = 1u64;
            nat_rem(a890, a891, a892)
        };
        let a894 = 0u64;
        nat_eq(a893, a894)
    } else {
        false
    } {
        let a902 = {
            let a900 = match v0.clone() {
                Adt1::C0(_, _, h895, _, _, _, _, _, _, _, _) => {
                    h895
                }
            };
            let a901 = {
                let a898 = match v0.clone() {
                    Adt1::C0(_, _, _, h896, _, _, _, _, _, _, _) => {
                        h896
                    }
                };
                let a899 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h897, _, _, _, _, _, _) => {
                        h897
                    }
                };
                nat_mul(a898, a899)?
            };
            nat_mul(a900, a901)?
        };
        let a903 = 1u64;
        if nat_le(a902, a903) {
            true
        } else {
            let a905 = v1;
            let a906 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h904, _, _, _, _) => {
                    h904
                }
            };
            if nat_le(a905, a906) {
                let a911 = {
                    let a908 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, h907, _, _, _, _) => {
                            h907
                        }
                    };
                    let a909 = 4u64;
                    let a910 = 1u64;
                    nat_rem(a908, a909, a910)
                };
                let a912 = 0u64;
                nat_eq(a911, a912)
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a914 = match v0.clone() {
            Adt1::C0(_, _, _, h913, _, _, _, _, _, _, _) => {
                h913
            }
        };
        let a915 = 1u64;
        if nat_le(a914, a915) {
            true
        } else {
            let a917 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h916, _, _, _) => {
                    h916
                }
            };
            let a918 = v2;
            nat_eq(a917, a918)
        }
    } else {
        false
    } {
        let a920 = match v0.clone() {
            Adt1::C0(_, _, _, _, h919, _, _, _, _, _, _) => {
                h919
            }
        };
        let a921 = 1u64;
        if nat_le(a920, a921) {
            Ok(true)
        } else {
            let a923 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, h922, _, _) => {
                    h922
                }
            };
            let a924 = v3;
            Ok(nat_eq(a923, a924))
        }
    } else {
        Ok(false)
    }
}

pub fn f40(v0: Adt1, v1: Adt1) -> R<bool> {
    let a926 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h925, _) => {
            h925
        }
    };
    let a927 = 0u64;
    if nat_eq(a926, a927) {
        Ok(true)
    } else {
        let a929 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h928, _) => {
                h928
            }
        };
        let a930 = 0u64;
        if nat_eq(a929, a930) {
            Ok(true)
        } else {
            let a932 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h931) => {
                    h931
                }
            };
            let a933 = 0u64;
            if nat_eq(a932, a933) {
                Ok(true)
            } else {
                let a935 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h934) => {
                        h934
                    }
                };
                let a936 = 0u64;
                if nat_eq(a935, a936) {
                    Ok(true)
                } else {
                    let a942 = {
                        let a939 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h937) => {
                                h937
                            }
                        };
                        let a940 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h938, _) => {
                                h938
                            }
                        };
                        nat_add(a939, a940)?
                    };
                    let a943 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h941) => {
                            h941
                        }
                    };
                    if nat_le(a942, a943) {
                        Ok(true)
                    } else {
                        let a949 = {
                            let a946 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h944) => {
                                    h944
                                }
                            };
                            let a947 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h945, _) => {
                                    h945
                                }
                            };
                            nat_add(a946, a947)?
                        };
                        let a950 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h948) => {
                                h948
                            }
                        };
                        Ok(nat_le(a949, a950))
                    }
                }
            }
        }
    }
}

pub fn f41(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a953 = match v0.clone() {
            Adt1::C0(_, _, h951, _, _, _, _, _, _, _, _) => {
                h951
            }
        };
        let a954 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h952, _, _, _, _) => {
                h952
            }
        };
        nat_mul(a953, a954)?
    };
    if if f35(v0.clone()) {
        let a956 = match v0.clone() {
            Adt1::C0(_, _, _, h955, _, _, _, _, _, _, _) => {
                h955
            }
        };
        let a957 = 1u64;
        if nat_eq(a956, a957) {
            true
        } else {
            let a959 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h958, _, _, _) => {
                    h958
                }
            };
            let a960 = v1;
            nat_eq(a959, a960)
        }
    } else {
        false
    } {
        let a962 = match v0.clone() {
            Adt1::C0(_, _, _, _, h961, _, _, _, _, _, _) => {
                h961
            }
        };
        let a963 = 1u64;
        if nat_eq(a962, a963) {
            Ok(true)
        } else {
            let a968 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h964, _, _) => {
                    h964
                }
            };
            let a969 = {
                let a966 = v1;
                let a967 = match v0 {
                    Adt1::C0(_, _, _, h965, _, _, _, _, _, _, _) => {
                        h965
                    }
                };
                nat_mul(a966, a967)?
            };
            Ok(nat_eq(a968, a969))
        }
    } else {
        Ok(false)
    }
}

pub fn f42(v0: u64) -> u64 {
    let a970 = v0;
    let a971 = 3u64;
    if nat_eq(a970, a971) {
        16u64
    } else {
        10u64
    }
}

pub fn f43(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a973 = v0;
        let a974 = match v1.clone() {
            Adt1::C0(_, h972, _, _, _, _, _, _, _, _, _) => {
                h972
            }
        };
        let a975 = 0u64;
        nat_quot(a973, a974, a975)
    };
    let v3: u64 = {
        let a977 = v2;
        let a978 = match v1.clone() {
            Adt1::C0(_, _, h976, _, _, _, _, _, _, _, _) => {
                h976
            }
        };
        let a979 = 0u64;
        nat_quot(a977, a978, a979)
    };
    f45({
        let a981 = v0;
        let a982 = match v1.clone() {
            Adt1::C0(_, h980, _, _, _, _, _, _, _, _, _) => {
                h980
            }
        };
        let a983 = 0u64;
        nat_rem(a981, a982, a983)
    }, {
        let a985 = v2;
        let a986 = match v1.clone() {
            Adt1::C0(_, _, h984, _, _, _, _, _, _, _, _) => {
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
        nat_rem(a989, a990, a991)
    }, {
        let a993 = v3;
        let a994 = match v1.clone() {
            Adt1::C0(_, _, _, h992, _, _, _, _, _, _, _) => {
                h992
            }
        };
        let a995 = 0u64;
        nat_quot(a993, a994, a995)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h996, _, _, _, _, _) => {
            h996
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h997, _, _, _, _) => {
            h997
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h998, _, _, _) => {
            h998
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h999, _, _) => {
            h999
        }
    })
}

pub fn f44(v0: u64, v1: Adt1, v2: Adt1) -> R<u64> {
    let v3: u64 = {
        let a1001 = v0;
        let a1002 = match v1.clone() {
            Adt1::C0(_, h1000, _, _, _, _, _, _, _, _, _) => {
                h1000
            }
        };
        let a1003 = 0u64;
        nat_quot(a1001, a1002, a1003)
    };
    let v4: u64 = {
        let a1005 = v3;
        let a1006 = match v1.clone() {
            Adt1::C0(_, _, h1004, _, _, _, _, _, _, _, _) => {
                h1004
            }
        };
        let a1007 = 0u64;
        nat_quot(a1005, a1006, a1007)
    };
    f45({
        let a1013 = {
            let a1009 = v0;
            let a1010 = match v1.clone() {
                Adt1::C0(_, h1008, _, _, _, _, _, _, _, _, _) => {
                    h1008
                }
            };
            let a1011 = 0u64;
            nat_rem(a1009, a1010, a1011)
        };
        let a1014 = match v2.clone() {
            Adt1::C0(_, h1012, _, _, _, _, _, _, _, _, _) => {
                h1012
            }
        };
        let a1015 = 0u64;
        nat_rem(a1013, a1014, a1015)
    }, {
        let a1021 = {
            let a1017 = v3;
            let a1018 = match v1.clone() {
                Adt1::C0(_, _, h1016, _, _, _, _, _, _, _, _) => {
                    h1016
                }
            };
            let a1019 = 0u64;
            nat_rem(a1017, a1018, a1019)
        };
        let a1022 = match v2.clone() {
            Adt1::C0(_, _, h1020, _, _, _, _, _, _, _, _) => {
                h1020
            }
        };
        let a1023 = 0u64;
        nat_rem(a1021, a1022, a1023)
    }, {
        let a1029 = {
            let a1025 = v4;
            let a1026 = match v1.clone() {
                Adt1::C0(_, _, _, h1024, _, _, _, _, _, _, _) => {
                    h1024
                }
            };
            let a1027 = 0u64;
            nat_rem(a1025, a1026, a1027)
        };
        let a1030 = match v2.clone() {
            Adt1::C0(_, _, _, h1028, _, _, _, _, _, _, _) => {
                h1028
            }
        };
        let a1031 = 0u64;
        nat_rem(a1029, a1030, a1031)
    }, {
        let a1037 = {
            let a1033 = v4;
            let a1034 = match v1 {
                Adt1::C0(_, _, _, h1032, _, _, _, _, _, _, _) => {
                    h1032
                }
            };
            let a1035 = 0u64;
            nat_quot(a1033, a1034, a1035)
        };
        let a1038 = match v2.clone() {
            Adt1::C0(_, _, _, _, h1036, _, _, _, _, _, _) => {
                h1036
            }
        };
        let a1039 = 0u64;
        nat_rem(a1037, a1038, a1039)
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h1040, _, _, _, _, _) => {
            h1040
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h1041, _, _, _, _) => {
            h1041
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1042, _, _, _) => {
            h1042
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1043, _, _) => {
            h1043
        }
    })
}

pub fn f45(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a1056 = {
        let a1052 = {
            let a1048 = {
                let a1044 = v0;
                let a1045 = v4;
                nat_mul(a1044, a1045)?
            };
            let a1049 = {
                let a1046 = v1;
                let a1047 = v5;
                nat_mul(a1046, a1047)?
            };
            nat_add(a1048, a1049)?
        };
        let a1053 = {
            let a1050 = v2;
            let a1051 = v6;
            nat_mul(a1050, a1051)?
        };
        nat_add(a1052, a1053)?
    };
    let a1057 = {
        let a1054 = v3;
        let a1055 = v7;
        nat_mul(a1054, a1055)?
    };
    nat_add(a1056, a1057)
}

pub fn f46(v0: u64, v1: Adt1, v2: u64, v3: u64) -> R<Adt3> {
    let v4: u64 = {
        let a1058 = v0;
        let a1059 = 2u64;
        let a1060 = 0u64;
        nat_quot(a1058, a1059, a1060)
    };
    let v5: u64 = f34(match v1.clone() {
        Adt1::C0(h1061, _, _, _, _, _, _, _, _, _, _) => {
            h1061
        }
    });
    let a1065 = {
        let a1062 = v0;
        let a1063 = 2u64;
        let a1064 = 0u64;
        nat_rem(a1062, a1063, a1064)
    };
    let a1066 = 0u64;
    if nat_eq(a1065, a1066) {
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 4u64, f43(v4, v1)?, 2u64, 0u64, 0u64, 2u64, v5, 8u64))
    } else {
        let a1067 = v3;
        let a1068 = v2;
        if if nat_lt(a1067, a1068) {
            let a1071 = v3;
            let a1072 = {
                let a1069 = v5;
                let a1070 = 4u64;
                if nat_eq(a1069, a1070) {
                    2147483648u64
                } else {
                    9223372036854775808u64
                }
            };
            nat_lt(a1071, a1072)
        } else {
            false
        } {
            Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, {
                let a1073 = 3u64;
                let a1074 = v4;
                nat_add(a1073, a1074)?
            }, 8u64, 8u64))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f47(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64) -> R<Adt3> {
    let v5: u64 = {
        let a1076 = 1u64;
        let a1077 = f36(v1.clone(), match v3.clone() {
            Adt1::C0(h1075, _, _, _, _, _, _, _, _, _, _) => {
                h1075
            }
        })?;
        nat_add(a1076, a1077)?
    };
    let v6: u64 = {
        let a1078 = v0;
        let a1079 = v5;
        let a1080 = 0u64;
        nat_rem(a1078, a1079, a1080)
    };
    let v7: u64 = {
        let a1081 = v0;
        let a1082 = v5;
        let a1083 = 0u64;
        nat_quot(a1081, a1082, a1083)
    };
    let v8: u64 = {
        let a1085 = v7;
        let a1086 = match v1.clone() {
            Adt1::C0(_, _, h1084, _, _, _, _, _, _, _, _) => {
                h1084
            }
        };
        let a1087 = 0u64;
        nat_rem(a1085, a1086, a1087)
    };
    let v9: u64 = {
        let a1089 = v7;
        let a1090 = match v1.clone() {
            Adt1::C0(_, _, h1088, _, _, _, _, _, _, _, _) => {
                h1088
            }
        };
        let a1091 = 0u64;
        nat_quot(a1089, a1090, a1091)
    };
    let v10: u64 = {
        let a1093 = v9;
        let a1094 = match v1.clone() {
            Adt1::C0(_, _, _, h1092, _, _, _, _, _, _, _) => {
                h1092
            }
        };
        let a1095 = 0u64;
        nat_rem(a1093, a1094, a1095)
    };
    let v11: u64 = {
        let a1097 = v9;
        let a1098 = match v1.clone() {
            Adt1::C0(_, _, _, h1096, _, _, _, _, _, _, _) => {
                h1096
            }
        };
        let a1099 = 0u64;
        nat_quot(a1097, a1098, a1099)
    };
    let a1100 = v6;
    let a1101 = 0u64;
    if nat_eq(a1100, a1101) {
        let v12: u64 = {
            let a1118 = v8;
            let a1119 = {
                let a1116 = match v2.clone() {
                    Adt1::C0(_, h1102, _, _, _, _, _, _, _, _, _) => {
                        h1102
                    }
                };
                let a1117 = {
                    let a1114 = {
                        let a1104 = v10;
                        let a1105 = match v2.clone() {
                            Adt1::C0(_, _, h1103, _, _, _, _, _, _, _, _) => {
                                h1103
                            }
                        };
                        let a1106 = 0u64;
                        nat_rem(a1104, a1105, a1106)
                    };
                    let a1115 = {
                        let a1112 = match v2.clone() {
                            Adt1::C0(_, _, h1107, _, _, _, _, _, _, _, _) => {
                                h1107
                            }
                        };
                        let a1113 = {
                            let a1109 = v11;
                            let a1110 = match v2 {
                                Adt1::C0(_, _, _, h1108, _, _, _, _, _, _, _) => {
                                    h1108
                                }
                            };
                            let a1111 = 0u64;
                            nat_rem(a1109, a1110, a1111)
                        };
                        nat_mul(a1112, a1113)?
                    };
                    nat_add(a1114, a1115)?
                };
                nat_mul(a1116, a1117)?
            };
            nat_add(a1118, a1119)?
        };
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, {
            let a1120 = 3u64;
            let a1121 = v12;
            nat_add(a1120, a1121)?
        }, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
    } else {
        let a1123 = v4;
        let a1124 = match v3.clone() {
            Adt1::C0(_, _, h1122, _, _, _, _, _, _, _, _) => {
                h1122
            }
        };
        if nat_lt(a1123, a1124) {
            let v13: u64 = {
                let a1129 = {
                    let a1125 = v6;
                    let a1126 = 1u64;
                    nat_sub(a1125, a1126)
                };
                let a1130 = f58(match v1.clone() {
                    Adt1::C0(h1127, _, _, _, _, _, _, _, _, _, _) => {
                        h1127
                    }
                }, match v3.clone() {
                    Adt1::C0(h1128, _, _, _, _, _, _, _, _, _, _) => {
                        h1128
                    }
                });
                let a1131 = 0u64;
                nat_quot(a1129, a1130, a1131)
            };
            let v14: u64 = f45({
                let a1133 = v13;
                let a1134 = f21(match v3.clone() {
                    Adt1::C0(h1132, _, _, _, _, _, _, _, _, _, _) => {
                        h1132
                    }
                });
                nat_mul(a1133, a1134)?
            }, v8, v10, v11, match v1.clone() {
                Adt1::C0(_, _, _, _, _, h1135, _, _, _, _, _) => {
                    h1135
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, h1136, _, _, _, _) => {
                    h1136
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1137, _, _, _) => {
                    h1137
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1138, _, _) => {
                    h1138
                }
            })?;
            let v15: u64 = f45(v13, v4, v10, v11, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h1139, _, _, _, _, _) => {
                    h1139
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h1140, _, _, _, _) => {
                    h1140
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1141, _, _, _) => {
                    h1141
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1142, _, _) => {
                    h1142
                }
            })?;
            if if f2(match v3.clone() {
                Adt1::C0(h1143, _, _, _, _, _, _, _, _, _, _) => {
                    h1143
                }
            }) {
                true
            } else {
                let a1145 = match v3.clone() {
                    Adt1::C0(h1144, _, _, _, _, _, _, _, _, _, _) => {
                        h1144
                    }
                };
                let a1146 = 26u64;
                nat_eq(a1145, a1146)
            } {
                Ok(Adt3::C6(v14, v15))
            } else {
                Ok(f59(match v1 {
                    Adt1::C0(h1147, _, _, _, _, _, _, _, _, _, _) => {
                        h1147
                    }
                }, match v3 {
                    Adt1::C0(h1148, _, _, _, _, _, _, _, _, _, _) => {
                        h1148
                    }
                }, {
                    let a1149 = v6;
                    let a1150 = 1u64;
                    nat_sub(a1149, a1150)
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
    let a1151 = v0;
    let a1152 = 8u64;
    if nat_eq(a1151, a1152) {
        Adt3::C3(1u64, 6u64, 0u64, 0u64, 1u64, 0u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
    } else {
        let a1153 = v0;
        let a1154 = 9u64;
        if nat_eq(a1153, a1154) {
            Adt3::C3(1u64, 2u64, 0u64, 0u64, 2u64, v1, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
        } else {
            let a1155 = v0;
            let a1156 = 10u64;
            if nat_eq(a1155, a1156) {
                Adt3::C3(5u64, 16u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
            } else {
                let a1157 = v0;
                let a1158 = 11u64;
                if nat_eq(a1157, a1158) {
                    Adt3::C3(1u64, 0u64, 0u64, 0u64, 2u64, 1065353216u64, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
                } else {
                    let a1159 = v0;
                    let a1160 = 12u64;
                    if nat_eq(a1159, a1160) {
                        Adt3::C3(1u64, 3u64, 0u64, 0u64, 1u64, 0u64, 1u64, 2u64, 0u64, 0u64, 4u64, 4u64)
                    } else {
                        let a1161 = v0;
                        let a1162 = 13u64;
                        if nat_eq(a1161, a1162) {
                            Adt3::C3(1u64, 0u64, 0u64, 0u64, 1u64, 1u64, 2u64, 1065353216u64, 0u64, 1u64, 4u64, 4u64)
                        } else {
                            let a1163 = v0;
                            let a1164 = 14u64;
                            if nat_eq(a1163, a1164) {
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
    let a1165 = v0;
    let a1166 = 0u64;
    if nat_eq(a1165, a1166) {
        Adt4::C1(Adt5::C0(0u64, 1u64, 4u64))
    } else {
        let a1167 = v0;
        let a1168 = 1u64;
        if nat_eq(a1167, a1168) {
            Adt4::C1(Adt5::C0(1u64, 1u64, 2u64))
        } else {
            let a1169 = v0;
            let a1170 = 2u64;
            if nat_eq(a1169, a1170) {
                Adt4::C1(Adt5::C0(2u64, 32u64, 18u64))
            } else {
                let a1171 = v0;
                let a1172 = 3u64;
                if nat_eq(a1171, a1172) {
                    Adt4::C1(Adt5::C0(3u64, 32u64, 20u64))
                } else {
                    let a1173 = v0;
                    let a1174 = 6u64;
                    if nat_eq(a1173, a1174) {
                        Adt4::C1(Adt5::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1175 = v0;
                        let a1176 = 7u64;
                        if nat_eq(a1175, a1176) {
                            Adt4::C1(Adt5::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1177 = v0;
                            let a1178 = 8u64;
                            if nat_eq(a1177, a1178) {
                                Adt4::C1(Adt5::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1179 = v0;
                                let a1180 = 9u64;
                                if nat_eq(a1179, a1180) {
                                    Adt4::C1(Adt5::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1181 = v0;
                                    let a1182 = 10u64;
                                    if nat_eq(a1181, a1182) {
                                        Adt4::C1(Adt5::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1183 = v0;
                                        let a1184 = 11u64;
                                        if nat_eq(a1183, a1184) {
                                            Adt4::C1(Adt5::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1185 = v0;
                                            let a1186 = 12u64;
                                            if nat_eq(a1185, a1186) {
                                                Adt4::C1(Adt5::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1187 = v0;
                                                let a1188 = 13u64;
                                                if nat_eq(a1187, a1188) {
                                                    Adt4::C1(Adt5::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1189 = v0;
                                                    let a1190 = 14u64;
                                                    if nat_eq(a1189, a1190) {
                                                        Adt4::C1(Adt5::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1191 = v0;
                                                        let a1192 = 15u64;
                                                        if nat_eq(a1191, a1192) {
                                                            Adt4::C1(Adt5::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1193 = v0;
                                                            let a1194 = 16u64;
                                                            if nat_eq(a1193, a1194) {
                                                                Adt4::C1(Adt5::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1195 = v0;
                                                                let a1196 = 17u64;
                                                                if nat_eq(a1195, a1196) {
                                                                    Adt4::C1(Adt5::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1197 = v0;
                                                                    let a1198 = 18u64;
                                                                    if nat_eq(a1197, a1198) {
                                                                        Adt4::C1(Adt5::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1199 = v0;
                                                                        let a1200 = 19u64;
                                                                        if nat_eq(a1199, a1200) {
                                                                            Adt4::C1(Adt5::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1201 = v0;
                                                                            let a1202 = 20u64;
                                                                            if nat_eq(a1201, a1202) {
                                                                                Adt4::C1(Adt5::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1203 = v0;
                                                                                let a1204 = 21u64;
                                                                                if nat_eq(a1203, a1204) {
                                                                                    Adt4::C1(Adt5::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1205 = v0;
                                                                                    let a1206 = 22u64;
                                                                                    if nat_eq(a1205, a1206) {
                                                                                        Adt4::C1(Adt5::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1207 = v0;
                                                                                        let a1208 = 23u64;
                                                                                        if nat_eq(a1207, a1208) {
                                                                                            Adt4::C1(Adt5::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1209 = v0;
                                                                                            let a1210 = 24u64;
                                                                                            if nat_eq(a1209, a1210) {
                                                                                                Adt4::C1(Adt5::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1211 = v0;
                                                                                                let a1212 = 25u64;
                                                                                                if nat_eq(a1211, a1212) {
                                                                                                    Adt4::C1(Adt5::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1213 = v0;
                                                                                                    let a1214 = 26u64;
                                                                                                    if nat_eq(a1213, a1214) {
                                                                                                        Adt4::C1(Adt5::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1215 = v0;
                                                                                                        let a1216 = 27u64;
                                                                                                        if nat_eq(a1215, a1216) {
                                                                                                            Adt4::C1(Adt5::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1217 = v0;
                                                                                                            let a1218 = 28u64;
                                                                                                            if nat_eq(a1217, a1218) {
                                                                                                                Adt4::C1(Adt5::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1219 = v0;
                                                                                                                let a1220 = 29u64;
                                                                                                                if nat_eq(a1219, a1220) {
                                                                                                                    Adt4::C1(Adt5::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1221 = v0;
                                                                                                                    let a1222 = 30u64;
                                                                                                                    if nat_eq(a1221, a1222) {
                                                                                                                        Adt4::C1(Adt5::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1223 = v0;
                                                                                                                        let a1224 = 34u64;
                                                                                                                        if nat_eq(a1223, a1224) {
                                                                                                                            Adt4::C1(Adt5::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1225 = v0;
                                                                                                                            let a1226 = 35u64;
                                                                                                                            if nat_eq(a1225, a1226) {
                                                                                                                                Adt4::C1(Adt5::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1227 = v0;
                                                                                                                                let a1228 = 39u64;
                                                                                                                                if nat_eq(a1227, a1228) {
                                                                                                                                    Adt4::C1(Adt5::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1229 = v0;
                                                                                                                                    let a1230 = 40u64;
                                                                                                                                    if nat_eq(a1229, a1230) {
                                                                                                                                        Adt4::C1(Adt5::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1231 = v0;
                                                                                                                                        let a1232 = 41u64;
                                                                                                                                        if nat_eq(a1231, a1232) {
                                                                                                                                            Adt4::C1(Adt5::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1233 = v0;
                                                                                                                                            let a1234 = 42u64;
                                                                                                                                            if nat_eq(a1233, a1234) {
                                                                                                                                                Adt4::C1(Adt5::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1235 = v0;
                                                                                                                                                let a1236 = 43u64;
                                                                                                                                                if nat_eq(a1235, a1236) {
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
        let a1239 = match v0.clone() {
            Adt1::C0(h1237, _, _, _, _, _, _, _, _, _, _) => {
                h1237
            }
        };
        let a1240 = match v1.clone() {
            Adt1::C0(h1238, _, _, _, _, _, _, _, _, _, _) => {
                h1238
            }
        };
        nat_eq(a1239, a1240)
    } else {
        false
    } {
        let a1243 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1241) => {
                h1241
            }
        };
        let a1244 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1242) => {
                h1242
            }
        };
        nat_eq(a1243, a1244)
    } else {
        false
    } {
        let a1247 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1245, _, _, _, _, _) => {
                h1245
            }
        };
        let a1248 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1246, _, _, _, _, _) => {
                h1246
            }
        };
        nat_eq(a1247, a1248)
    } else {
        false
    } {
        let a1251 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1249, _, _, _, _) => {
                h1249
            }
        };
        let a1252 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1250, _, _, _, _) => {
                h1250
            }
        };
        nat_eq(a1251, a1252)
    } else {
        false
    } {
        let a1255 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1253, _, _, _) => {
                h1253
            }
        };
        let a1256 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1254, _, _, _) => {
                h1254
            }
        };
        nat_eq(a1255, a1256)
    } else {
        false
    } {
        let a1259 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1257, _, _) => {
                h1257
            }
        };
        let a1260 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1258, _, _) => {
                h1258
            }
        };
        nat_eq(a1259, a1260)
    } else {
        false
    }
}

pub fn f53(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a1261 = v1;
    let a1262 = 1u64;
    if if if nat_le(a1261, a1262) {
        true
    } else {
        let a1263 = v0;
        let a1264 = v2;
        nat_le(a1263, a1264)
    } {
        let a1265 = v3;
        let a1266 = 1u64;
        if nat_le(a1265, a1266) {
            true
        } else {
            let a1273 = {
                let a1271 = v0;
                let a1272 = {
                    let a1269 = {
                        let a1267 = v1;
                        let a1268 = 1u64;
                        nat_sub(a1267, a1268)
                    };
                    let a1270 = v2;
                    nat_mul(a1269, a1270)?
                };
                nat_add(a1271, a1272)?
            };
            let a1274 = v4;
            nat_le(a1273, a1274)
        }
    } else {
        false
    } {
        let a1275 = v5;
        let a1276 = 1u64;
        if nat_le(a1275, a1276) {
            Ok(true)
        } else {
            let a1289 = {
                let a1287 = {
                    let a1281 = v0;
                    let a1282 = {
                        let a1279 = {
                            let a1277 = v1;
                            let a1278 = 1u64;
                            nat_sub(a1277, a1278)
                        };
                        let a1280 = v2;
                        nat_mul(a1279, a1280)?
                    };
                    nat_add(a1281, a1282)?
                };
                let a1288 = {
                    let a1285 = {
                        let a1283 = v3;
                        let a1284 = 1u64;
                        nat_sub(a1283, a1284)
                    };
                    let a1286 = v4;
                    nat_mul(a1285, a1286)?
                };
                nat_add(a1287, a1288)?
            };
            let a1290 = v6;
            Ok(nat_le(a1289, a1290))
        }
    } else {
        Ok(false)
    }
}

pub fn f54(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1291 = v0;
    let a1292 = 0u64;
    if if if nat_eq(a1291, a1292) {
        true
    } else {
        let a1293 = v1;
        let a1294 = 0u64;
        nat_eq(a1293, a1294)
    } {
        true
    } else {
        let a1295 = v2;
        let a1296 = 0u64;
        nat_eq(a1295, a1296)
    } {
        true
    } else {
        let a1297 = v3;
        let a1298 = 0u64;
        nat_eq(a1297, a1298)
    }
}

pub fn f55(v0: u64, v1: u64, v2: u64) -> bool {
    let a1299 = v0;
    let a1300 = 0u64;
    if if if nat_eq(a1299, a1300) {
        let a1301 = v1;
        let a1302 = 0u64;
        nat_eq(a1301, a1302)
    } else {
        false
    } {
        let a1303 = v2;
        let a1304 = 0u64;
        nat_eq(a1303, a1304)
    } else {
        false
    } {
        true
    } else {
        let a1305 = v0;
        let a1306 = 1u64;
        if if nat_eq(a1305, a1306) {
            true
        } else {
            let a1307 = v0;
            let a1308 = 30u64;
            nat_eq(a1307, a1308)
        } {
            let a1309 = v1;
            let a1310 = v0;
            if if nat_eq(a1309, a1310) {
                let a1311 = v2;
                let a1312 = v0;
                nat_eq(a1311, a1312)
            } else {
                false
            } {
                true
            } else {
                let a1313 = v1;
                let a1314 = 0u64;
                if nat_eq(a1313, a1314) {
                    let a1315 = v2;
                    let a1316 = v0;
                    if nat_eq(a1315, a1316) {
                        true
                    } else {
                        let a1317 = v2;
                        let a1318 = 0u64;
                        nat_eq(a1317, a1318)
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
    let a1319 = v0;
    let a1320 = 1u64;
    if nat_le(a1319, a1320) {
        true
    } else {
        let a1324 = {
            let a1321 = v1;
            let a1322 = v2;
            let a1323 = 1u64;
            nat_rem(a1321, a1322, a1323)
        };
        let a1325 = 0u64;
        nat_eq(a1324, a1325)
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
    let a1326 = v0;
    let a1327 = 1u64;
    if if nat_eq(a1326, a1327) {
        let a1328 = v1;
        let a1329 = 43u64;
        nat_eq(a1328, a1329)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f59(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt3 {
    let a1330 = f58(v0, v1);
    let a1331 = 2u64;
    if nat_eq(a1330, a1331) {
        let a1335 = {
            let a1332 = v2;
            let a1333 = 2u64;
            let a1334 = 0u64;
            nat_rem(a1332, a1333, a1334)
        };
        let a1336 = 0u64;
        if nat_eq(a1335, a1336) {
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
