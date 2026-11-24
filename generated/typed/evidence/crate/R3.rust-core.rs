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
    C0(u64, u64, Option<u64>, u64, u64, u64, u64, Adt1, Adt1, Adt1),
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
            let a22 = match v27.clone() {
                Adt2::C0(_, _, _, h21, _, _, _, _, _, _) => {
                    h21
                }
            };
            let a23 = 100u64;
            if nat_eq(a22, a23) {
                f10(v26, v27)
            } else {
                let a25 = match v27.clone() {
                    Adt2::C0(_, _, _, h24, _, _, _, _, _, _) => {
                        h24
                    }
                };
                let a26 = 33u64;
                if nat_eq(a25, a26) {
                    f11(v26, v27)
                } else {
                    let a28 = match v27.clone() {
                        Adt2::C0(_, _, _, h27, _, _, _, _, _, _) => {
                            h27
                        }
                    };
                    let a29 = 66u64;
                    if if nat_eq(a28, a29) {
                        true
                    } else {
                        let a31 = match v27.clone() {
                            Adt2::C0(_, _, _, h30, _, _, _, _, _, _) => {
                                h30
                            }
                        };
                        let a32 = 67u64;
                        nat_eq(a31, a32)
                    } {
                        f12(v26, v27)
                    } else {
                        let a34 = match v27.clone() {
                            Adt2::C0(_, _, _, h33, _, _, _, _, _, _) => {
                                h33
                            }
                        };
                        let a35 = 41u64;
                        if nat_eq(a34, a35) {
                            f13(v26, v27, v28)
                        } else {
                            let a37 = 98u64;
                            let a38 = match v27.clone() {
                                Adt2::C0(_, _, _, h36, _, _, _, _, _, _) => {
                                    h36
                                }
                            };
                            if nat_le(a37, a38) {
                                if f14(match v27.clone() {
                                    Adt2::C0(_, _, _, _, h39, _, _, _, _, _) => {
                                        h39
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
    let a40 = v0;
    let a41 = 2u64;
    if nat_eq(a40, a41) {
        Some(0u64)
    } else {
        let a42 = v0;
        let a43 = 5u64;
        if nat_eq(a42, a43) {
            Some(1u64)
        } else {
            let a44 = v0;
            let a45 = 6u64;
            if nat_eq(a44, a45) {
                Some(2u64)
            } else {
                let a46 = v0;
                let a47 = 7u64;
                if nat_eq(a46, a47) {
                    Some(3u64)
                } else {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f2(v0: u64) -> bool {
    let a48 = 6u64;
    let a49 = v0;
    if if if if nat_le(a48, a49) {
        let a50 = v0;
        let a51 = 23u64;
        nat_le(a50, a51)
    } else {
        false
    } {
        true
    } else {
        let a52 = v0;
        let a53 = 2u64;
        if nat_eq(a52, a53) {
            true
        } else {
            let a54 = v0;
            let a55 = 3u64;
            nat_eq(a54, a55)
        }
    } {
        true
    } else {
        let a56 = v0;
        let a57 = 29u64;
        nat_eq(a56, a57)
    } {
        true
    } else {
        let a58 = 34u64;
        let a59 = v0;
        if if nat_le(a58, a59) {
            let a60 = v0;
            let a61 = 35u64;
            nat_le(a60, a61)
        } else {
            false
        } {
            true
        } else {
            let a62 = 39u64;
            let a63 = v0;
            if nat_le(a62, a63) {
                let a64 = v0;
                let a65 = 42u64;
                nat_le(a64, a65)
            } else {
                false
            }
        }
    }
}

pub fn f3(v0: Adt1, v1: Adt1, v2: Adt1) -> R<Adt3> {
    if if if if if if if if if if if if if if if f18(v0.clone(), v1.clone()) {
        f18(v0.clone(), v2.clone())
    } else {
        false
    } {
        let a67 = match v1.clone() {
            Adt1::C0(h66, _, _, _, _, _, _, _, _, _, _) => {
                h66
            }
        };
        let a68 = 0u64;
        nat_eq(a67, a68)
    } else {
        false
    } {
        let a70 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h69, _, _, _, _, _) => {
                h69
            }
        };
        let a71 = 4u64;
        nat_eq(a70, a71)
    } else {
        false
    } {
        f2(match v0.clone() {
            Adt1::C0(h72, _, _, _, _, _, _, _, _, _, _) => {
                h72
            }
        })
    } else {
        false
    } {
        let a74 = match v0.clone() {
            Adt1::C0(h73, _, _, _, _, _, _, _, _, _, _) => {
                h73
            }
        };
        let a75 = 9u64;
        !if nat_eq(a74, a75) {
            true
        } else {
            let a77 = match v0.clone() {
                Adt1::C0(h76, _, _, _, _, _, _, _, _, _, _) => {
                    h76
                }
            };
            let a78 = 15u64;
            nat_eq(a77, a78)
        }
    } else {
        false
    } {
        let a80 = 0u64;
        let a81 = f19(match v2.clone() {
            Adt1::C0(h79, _, _, _, _, _, _, _, _, _, _) => {
                h79
            }
        });
        nat_lt(a80, a81)
    } else {
        false
    } {
        let a83 = 16u64;
        let a84 = match v2.clone() {
            Adt1::C0(h82, _, _, _, _, _, _, _, _, _, _) => {
                h82
            }
        };
        !if if if nat_le(a83, a84) {
            let a86 = match v2.clone() {
                Adt1::C0(h85, _, _, _, _, _, _, _, _, _, _) => {
                    h85
                }
            };
            let a87 = 19u64;
            nat_le(a86, a87)
        } else {
            false
        } {
            true
        } else {
            let a89 = match v2.clone() {
                Adt1::C0(h88, _, _, _, _, _, _, _, _, _, _) => {
                    h88
                }
            };
            let a90 = 21u64;
            if nat_eq(a89, a90) {
                true
            } else {
                let a92 = match v2.clone() {
                    Adt1::C0(h91, _, _, _, _, _, _, _, _, _, _) => {
                        h91
                    }
                };
                let a93 = 22u64;
                nat_eq(a92, a93)
            }
        } {
            true
        } else {
            let a95 = match v2.clone() {
                Adt1::C0(h94, _, _, _, _, _, _, _, _, _, _) => {
                    h94
                }
            };
            let a96 = 29u64;
            nat_eq(a95, a96)
        }
    } else {
        false
    } {
        let a99 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h97, _, _, _, _, _) => {
                h97
            }
        };
        let a100 = f19(match v0.clone() {
            Adt1::C0(h98, _, _, _, _, _, _, _, _, _, _) => {
                h98
            }
        });
        nat_eq(a99, a100)
    } else {
        false
    } {
        let a103 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h101, _, _, _, _, _) => {
                h101
            }
        };
        let a104 = f19(match v2.clone() {
            Adt1::C0(h102, _, _, _, _, _, _, _, _, _, _) => {
                h102
            }
        });
        nat_eq(a103, a104)
    } else {
        false
    } {
        let a110 = {
            let a107 = match v0.clone() {
                Adt1::C0(_, h105, _, _, _, _, _, _, _, _, _) => {
                    h105
                }
            };
            let a108 = f20(match v0.clone() {
                Adt1::C0(h106, _, _, _, _, _, _, _, _, _, _) => {
                    h106
                }
            });
            let a109 = 1u64;
            nat_rem(a107, a108, a109)
        };
        let a111 = 0u64;
        nat_eq(a110, a111)
    } else {
        false
    } {
        let a117 = {
            let a114 = match v2.clone() {
                Adt1::C0(_, h112, _, _, _, _, _, _, _, _, _) => {
                    h112
                }
            };
            let a115 = f20(match v2.clone() {
                Adt1::C0(h113, _, _, _, _, _, _, _, _, _, _) => {
                    h113
                }
            });
            let a116 = 1u64;
            nat_rem(a114, a115, a116)
        };
        let a118 = 0u64;
        nat_eq(a117, a118)
    } else {
        false
    } {
        f21(v0.clone(), v2.clone())?
    } else {
        false
    } {
        f21(v1.clone(), v2.clone())?
    } else {
        false
    } {
        f22({
            let a125 = {
                let a121 = match v2.clone() {
                    Adt1::C0(_, h119, _, _, _, _, _, _, _, _, _) => {
                        h119
                    }
                };
                let a122 = f20(match v2.clone() {
                    Adt1::C0(h120, _, _, _, _, _, _, _, _, _, _) => {
                        h120
                    }
                });
                let a123 = 0u64;
                nat_quot(a121, a122, a123)
            };
            let a126 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, h124, _, _, _, _, _) => {
                    h124
                }
            };
            nat_mul(a125, a126)?
        }, match v2.clone() {
            Adt1::C0(_, _, h127, _, _, _, _, _, _, _, _) => {
                h127
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, h128, _, _, _, _, _, _, _) => {
                h128
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h129, _, _, _, _, _, _) => {
                h129
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h130, _, _, _, _) => {
                h130
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h131, _, _, _) => {
                h131
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h132, _, _) => {
                h132
            }
        })?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0(if f23(v0.clone()) {
            0u64
        } else {
            let a138 = match v0.clone() {
                Adt1::C0(_, _, h133, _, _, _, _, _, _, _, _) => {
                    h133
                }
            };
            let a139 = {
                let a136 = match v0.clone() {
                    Adt1::C0(_, _, _, h134, _, _, _, _, _, _, _) => {
                        h134
                    }
                };
                let a137 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h135, _, _, _, _, _, _) => {
                        h135
                    }
                };
                nat_mul(a136, a137)?
            };
            nat_mul(a138, a139)?
        }, 3u64, None::<u64>, 100u64, 0u64, 0u64, 0u64, v0, v1, v2)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f4(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if f18(v1.clone(), v3.clone()) {
        f24(v0, v1.clone(), v2.clone(), v3.clone())?
    } else {
        false
    } {
        if f23(v1.clone()) {
            true
        } else {
            let a142 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, h140, _, _, _, _, _) => {
                    h140
                }
            };
            let a143 = f25(match v1.clone() {
                Adt1::C0(h141, _, _, _, _, _, _, _, _, _, _) => {
                    h141
                }
            });
            if if if if if if if nat_eq(a142, a143) {
                let a146 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h144, _, _, _, _, _) => {
                        h144
                    }
                };
                let a147 = f25(match v3.clone() {
                    Adt1::C0(h145, _, _, _, _, _, _, _, _, _, _) => {
                        h145
                    }
                });
                nat_eq(a146, a147)
            } else {
                false
            } {
                f26(v1.clone())
            } else {
                false
            } {
                f26(v2.clone())
            } else {
                false
            } {
                f26(v3.clone())
            } else {
                false
            } {
                f27(v1.clone(), v2.clone())?
            } else {
                false
            } {
                f21(v1.clone(), v3.clone())?
            } else {
                false
            } {
                f21(v2.clone(), v3.clone())?
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v4: u64 = if if f23(v1.clone()) {
            true
        } else {
            f28(v3.clone())?
        } {
            0u64
        } else {
            let a150 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h148, _) => {
                    h148
                }
            };
            let a151 = f25(match v3.clone() {
                Adt1::C0(h149, _, _, _, _, _, _, _, _, _, _) => {
                    h149
                }
            });
            let a152 = 0u64;
            nat_quot(a150, a151, a152)
        };
        Ok(Adt3::C2(Adt2::C0(f29(f30(v1.clone())?, v4, f31(match v1.clone() {
            Adt1::C0(h153, _, _, _, _, _, _, _, _, _, _) => {
                h153
            }
        }, match v2.clone() {
            Adt1::C0(h154, _, _, _, _, _, _, _, _, _, _) => {
                h154
            }
        }, match v3.clone() {
            Adt1::C0(h155, _, _, _, _, _, _, _, _, _, _) => {
                h155
            }
        })?)?, {
            let a156 = 3u64;
            let a157 = v4;
            nat_add(a156, a157)?
        }, None::<u64>, v0, 0u64, 0u64, 0u64, v1, v2, v3)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f5(v0: Adt1, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if if if if if if if if if if if if if f32(v2, v3.clone()) {
        let a159 = match v0.clone() {
            Adt1::C0(h158, _, _, _, _, _, _, _, _, _, _) => {
                h158
            }
        };
        let a160 = 0u64;
        if nat_eq(a159, a160) {
            true
        } else {
            let a162 = match v0.clone() {
                Adt1::C0(h161, _, _, _, _, _, _, _, _, _, _) => {
                    h161
                }
            };
            let a163 = 1u64;
            nat_eq(a162, a163)
        }
    } else {
        false
    } {
        let a165 = 0u64;
        let a166 = f19(match v3.clone() {
            Adt1::C0(h164, _, _, _, _, _, _, _, _, _, _) => {
                h164
            }
        });
        nat_lt(a165, a166)
    } else {
        false
    } {
        let a168 = 0u64;
        let a169 = f33(match v1.clone() {
            Adt1::C0(h167, _, _, _, _, _, _, _, _, _, _) => {
                h167
            }
        });
        nat_lt(a168, a169)
    } else {
        false
    } {
        let a172 = match v0.clone() {
            Adt1::C0(_, h170, _, _, _, _, _, _, _, _, _) => {
                h170
            }
        };
        let a173 = match v3.clone() {
            Adt1::C0(_, h171, _, _, _, _, _, _, _, _, _) => {
                h171
            }
        };
        if nat_eq(a172, a173) {
            let a179 = {
                let a176 = match v0.clone() {
                    Adt1::C0(_, h174, _, _, _, _, _, _, _, _, _) => {
                        h174
                    }
                };
                let a177 = f20(match v3.clone() {
                    Adt1::C0(h175, _, _, _, _, _, _, _, _, _, _) => {
                        h175
                    }
                });
                let a178 = 1u64;
                nat_rem(a176, a177, a178)
            };
            let a180 = 0u64;
            nat_eq(a179, a180)
        } else {
            false
        }
    } else {
        false
    } {
        let a183 = match v0.clone() {
            Adt1::C0(_, _, _, h181, _, _, _, _, _, _, _) => {
                h181
            }
        };
        let a184 = match v3.clone() {
            Adt1::C0(_, _, _, h182, _, _, _, _, _, _, _) => {
                h182
            }
        };
        nat_eq(a183, a184)
    } else {
        false
    } {
        let a187 = match v0.clone() {
            Adt1::C0(_, _, _, _, h185, _, _, _, _, _, _) => {
                h185
            }
        };
        let a188 = match v3.clone() {
            Adt1::C0(_, _, _, _, h186, _, _, _, _, _, _) => {
                h186
            }
        };
        nat_eq(a187, a188)
    } else {
        false
    } {
        let a191 = match v0.clone() {
            Adt1::C0(_, _, h189, _, _, _, _, _, _, _, _) => {
                h189
            }
        };
        let a192 = match v1.clone() {
            Adt1::C0(_, h190, _, _, _, _, _, _, _, _, _) => {
                h190
            }
        };
        nat_eq(a191, a192)
    } else {
        false
    } {
        let a194 = match v1.clone() {
            Adt1::C0(_, _, _, _, h193, _, _, _, _, _, _) => {
                h193
            }
        };
        let a195 = 1u64;
        nat_eq(a194, a195)
    } else {
        false
    } {
        let a197 = 0u64;
        let a198 = match v1.clone() {
            Adt1::C0(_, _, h196, _, _, _, _, _, _, _, _) => {
                h196
            }
        };
        nat_lt(a197, a198)
    } else {
        false
    } {
        let a200 = 0u64;
        let a201 = match v1.clone() {
            Adt1::C0(_, _, _, h199, _, _, _, _, _, _, _) => {
                h199
            }
        };
        nat_lt(a200, a201)
    } else {
        false
    } {
        let a207 = {
            let a204 = match v0.clone() {
                Adt1::C0(_, _, _, h202, _, _, _, _, _, _, _) => {
                    h202
                }
            };
            let a205 = match v1.clone() {
                Adt1::C0(_, _, h203, _, _, _, _, _, _, _, _) => {
                    h203
                }
            };
            let a206 = 0u64;
            nat_rem(a204, a205, a206)
        };
        let a208 = 0u64;
        nat_eq(a207, a208)
    } else {
        false
    } {
        let a214 = {
            let a211 = match v0.clone() {
                Adt1::C0(_, _, _, _, h209, _, _, _, _, _, _) => {
                    h209
                }
            };
            let a212 = match v1.clone() {
                Adt1::C0(_, _, _, h210, _, _, _, _, _, _, _) => {
                    h210
                }
            };
            let a213 = 0u64;
            nat_rem(a211, a212, a213)
        };
        let a215 = 0u64;
        nat_eq(a214, a215)
    } else {
        false
    } {
        f34(v0.clone())
    } else {
        false
    } {
        let a217 = match v3.clone() {
            Adt1::C0(_, h216, _, _, _, _, _, _, _, _, _) => {
                h216
            }
        };
        let a218 = 1u64;
        if nat_eq(a217, a218) {
            true
        } else {
            let a221 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h219, _, _, _, _, _) => {
                    h219
                }
            };
            let a222 = f19(match v3.clone() {
                Adt1::C0(h220, _, _, _, _, _, _, _, _, _, _) => {
                    h220
                }
            });
            nat_eq(a221, a222)
        }
    } else {
        false
    } {
        let v4: u64 = f30(v1.clone())?;
        let v5: u64 = {
            let a228 = match v0.clone() {
                Adt1::C0(_, _, h223, _, _, _, _, _, _, _, _) => {
                    h223
                }
            };
            let a229 = {
                let a226 = match v0.clone() {
                    Adt1::C0(_, _, _, h224, _, _, _, _, _, _, _) => {
                        h224
                    }
                };
                let a227 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h225, _, _, _, _, _, _) => {
                        h225
                    }
                };
                nat_mul(a226, a227)?
            };
            nat_mul(a228, a229)?
        };
        let v6: u64 = if f23(v0.clone()) {
            0u64
        } else {
            let a237 = {
                let a230 = 2u64;
                let a231 = v4;
                nat_mul(a230, a231)?
            };
            let a238 = {
                let a235 = v5;
                let a236 = {
                    let a233 = 1u64;
                    let a234 = f35(v0.clone(), match v3.clone() {
                        Adt1::C0(h232, _, _, _, _, _, _, _, _, _, _) => {
                            h232
                        }
                    })?;
                    nat_add(a233, a234)?
                };
                nat_mul(a235, a236)?
            };
            nat_add(a237, a238)?
        };
        Ok(Adt3::C2(Adt2::C0(v6, if f23(v0.clone()) {
            3u64
        } else {
            let a239 = 3u64;
            let a240 = v4;
            nat_add(a239, a240)?
        }, f36(match v3.clone() {
            Adt1::C0(h241, _, _, _, _, _, _, _, _, _, _) => {
                h241
            }
        }), 41u64, 0u64, 0u64, 0u64, v0, v1, v3)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f6(v0: Adt1, v1: Adt1, v2: Adt1) -> R<Adt3> {
    let v3: u64 = f20(match v2.clone() {
        Adt1::C0(h242, _, _, _, _, _, _, _, _, _, _) => {
            h242
        }
    });
    if if if if if if if f2(match v2.clone() {
        Adt1::C0(h243, _, _, _, _, _, _, _, _, _, _) => {
            h243
        }
    }) {
        f32(v1.clone(), v2.clone())
    } else {
        false
    } {
        let a245 = 0u64;
        let a246 = f37(match v0.clone() {
            Adt1::C0(h244, _, _, _, _, _, _, _, _, _, _) => {
                h244
            }
        });
        nat_lt(a245, a246)
    } else {
        false
    } {
        let a249 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h247, _, _, _, _, _) => {
                h247
            }
        };
        let a250 = f37(match v0.clone() {
            Adt1::C0(h248, _, _, _, _, _, _, _, _, _, _) => {
                h248
            }
        });
        nat_eq(a249, a250)
    } else {
        false
    } {
        let a255 = {
            let a252 = match v0.clone() {
                Adt1::C0(_, h251, _, _, _, _, _, _, _, _, _) => {
                    h251
                }
            };
            let a253 = v3;
            let a254 = 1u64;
            nat_rem(a252, a253, a254)
        };
        let a256 = 0u64;
        if nat_eq(a255, a256) {
            let a261 = {
                let a258 = match v2.clone() {
                    Adt1::C0(_, h257, _, _, _, _, _, _, _, _, _) => {
                        h257
                    }
                };
                let a259 = v3;
                let a260 = 1u64;
                nat_rem(a258, a259, a260)
            };
            let a262 = 0u64;
            nat_eq(a261, a262)
        } else {
            false
        }
    } else {
        false
    } {
        let a265 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h263, _, _, _, _, _) => {
                h263
            }
        };
        let a266 = f19(match v2.clone() {
            Adt1::C0(h264, _, _, _, _, _, _, _, _, _, _) => {
                h264
            }
        });
        nat_eq(a265, a266)
    } else {
        false
    } {
        let a267 = f30(v0.clone())?;
        let a268 = f30(v2.clone())?;
        nat_eq(a267, a268)
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a269 = f30(v0.clone())?;
            let a270 = v3;
            let a271 = 0u64;
            nat_quot(a269, a270, a271)
        }, 3u64, f36(match v2.clone() {
            Adt1::C0(h272, _, _, _, _, _, _, _, _, _, _) => {
                h272
            }
        }), 33u64, 0u64, 0u64, 0u64, v0, v1, v2)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f7(v0: u64, v1: u64, v2: Adt1, v3: Adt1) -> R<Adt3> {
    let a274 = match v2.clone() {
        Adt1::C0(h273, _, _, _, _, _, _, _, _, _, _) => {
            h273
        }
    };
    let a275 = 0u64;
    if if if if if if if if if if if nat_eq(a274, a275) {
        let a277 = match v3.clone() {
            Adt1::C0(h276, _, _, _, _, _, _, _, _, _, _) => {
                h276
            }
        };
        let a278 = 26u64;
        nat_eq(a277, a278)
    } else {
        false
    } {
        let a280 = match v2.clone() {
            Adt1::C0(_, h279, _, _, _, _, _, _, _, _, _) => {
                h279
            }
        };
        let a281 = 2147483647u64;
        nat_le(a280, a281)
    } else {
        false
    } {
        let a284 = match v3.clone() {
            Adt1::C0(_, h282, _, _, _, _, _, _, _, _, _) => {
                h282
            }
        };
        let a285 = match v2.clone() {
            Adt1::C0(_, h283, _, _, _, _, _, _, _, _, _) => {
                h283
            }
        };
        nat_le(a284, a285)
    } else {
        false
    } {
        let a288 = match v2.clone() {
            Adt1::C0(_, _, h286, _, _, _, _, _, _, _, _) => {
                h286
            }
        };
        let a289 = match v3.clone() {
            Adt1::C0(_, _, h287, _, _, _, _, _, _, _, _) => {
                h287
            }
        };
        nat_eq(a288, a289)
    } else {
        false
    } {
        let a292 = match v2.clone() {
            Adt1::C0(_, _, _, h290, _, _, _, _, _, _, _) => {
                h290
            }
        };
        let a293 = match v3.clone() {
            Adt1::C0(_, _, _, h291, _, _, _, _, _, _, _) => {
                h291
            }
        };
        nat_eq(a292, a293)
    } else {
        false
    } {
        let a296 = match v2.clone() {
            Adt1::C0(_, _, _, _, h294, _, _, _, _, _, _) => {
                h294
            }
        };
        let a297 = match v3.clone() {
            Adt1::C0(_, _, _, _, h295, _, _, _, _, _, _) => {
                h295
            }
        };
        nat_eq(a296, a297)
    } else {
        false
    } {
        let a298 = v0;
        let a299 = 67u64;
        if nat_eq(a298, a299) {
            true
        } else {
            let a300 = v1;
            let a301 = 1u64;
            if nat_le(a300, a301) {
                let a304 = match v3.clone() {
                    Adt1::C0(_, h302, _, _, _, _, _, _, _, _, _) => {
                        h302
                    }
                };
                let a305 = match v2.clone() {
                    Adt1::C0(_, h303, _, _, _, _, _, _, _, _, _) => {
                        h303
                    }
                };
                nat_eq(a304, a305)
            } else {
                false
            }
        }
    } else {
        false
    } {
        f38(v2.clone())?
    } else {
        false
    } {
        f38(v3.clone())?
    } else {
        false
    } {
        f39(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a311 = match v3.clone() {
                Adt1::C0(_, _, h306, _, _, _, _, _, _, _, _) => {
                    h306
                }
            };
            let a312 = {
                let a309 = match v3.clone() {
                    Adt1::C0(_, _, _, h307, _, _, _, _, _, _, _) => {
                        h307
                    }
                };
                let a310 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h308, _, _, _, _, _, _) => {
                        h308
                    }
                };
                nat_mul(a309, a310)?
            };
            nat_mul(a311, a312)?
        }, 3u64, None::<u64>, v0, {
            let a313 = v0;
            let a314 = 67u64;
            if nat_eq(a313, a314) {
                1u64
            } else {
                v1
            }
        }, 0u64, 0u64, v2.clone(), v2, v3)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f8(v0: u64) -> bool {
    let a315 = v0;
    let a316 = 6u64;
    nat_le(a315, a316)
}

pub fn f9(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1) -> R<Adt3> {
    let a317 = v2;
    let a318 = 1u64;
    if if if if if if if if if if if if if if if nat_le(a317, a318) {
        let a320 = 0u64;
        let a321 = f37(match v5.clone() {
            Adt1::C0(h319, _, _, _, _, _, _, _, _, _, _) => {
                h319
            }
        });
        nat_lt(a320, a321)
    } else {
        false
    } {
        let a324 = match v5.clone() {
            Adt1::C0(h322, _, _, _, _, _, _, _, _, _, _) => {
                h322
            }
        };
        let a325 = match v6.clone() {
            Adt1::C0(h323, _, _, _, _, _, _, _, _, _, _) => {
                h323
            }
        };
        nat_eq(a324, a325)
    } else {
        false
    } {
        let a328 = match v5.clone() {
            Adt1::C0(h326, _, _, _, _, _, _, _, _, _, _) => {
                h326
            }
        };
        let a329 = match v7.clone() {
            Adt1::C0(h327, _, _, _, _, _, _, _, _, _, _) => {
                h327
            }
        };
        nat_eq(a328, a329)
    } else {
        false
    } {
        f18(v5.clone(), v6.clone())
    } else {
        false
    } {
        f40(v5.clone())?
    } else {
        false
    } {
        f40(v6.clone())?
    } else {
        false
    } {
        f40(v7.clone())?
    } else {
        false
    } {
        let a332 = match v5.clone() {
            Adt1::C0(_, _, h330, _, _, _, _, _, _, _, _) => {
                h330
            }
        };
        let a333 = match v7.clone() {
            Adt1::C0(_, _, h331, _, _, _, _, _, _, _, _) => {
                h331
            }
        };
        nat_eq(a332, a333)
    } else {
        false
    } {
        let a336 = match v5.clone() {
            Adt1::C0(_, _, _, h334, _, _, _, _, _, _, _) => {
                h334
            }
        };
        let a337 = match v7.clone() {
            Adt1::C0(_, _, _, h335, _, _, _, _, _, _, _) => {
                h335
            }
        };
        nat_eq(a336, a337)
    } else {
        false
    } {
        let a340 = match v5.clone() {
            Adt1::C0(_, _, _, _, h338, _, _, _, _, _, _) => {
                h338
            }
        };
        let a341 = match v7.clone() {
            Adt1::C0(_, _, _, _, h339, _, _, _, _, _, _) => {
                h339
            }
        };
        nat_eq(a340, a341)
    } else {
        false
    } {
        let a348 = match v7.clone() {
            Adt1::C0(_, h342, _, _, _, _, _, _, _, _, _) => {
                h342
            }
        };
        let a349 = if v1 {
            match v5.clone() {
                Adt1::C0(_, h343, _, _, _, _, _, _, _, _, _) => {
                    h343
                }
            }
        } else {
            let a345 = match v5.clone() {
                Adt1::C0(_, h344, _, _, _, _, _, _, _, _, _) => {
                    h344
                }
            };
            let a346 = 2u64;
            let a347 = 0u64;
            nat_quot(a345, a346, a347)
        };
        nat_eq(a348, a349)
    } else {
        false
    } {
        let a351 = match v7.clone() {
            Adt1::C0(_, h350, _, _, _, _, _, _, _, _, _) => {
                h350
            }
        };
        let a352 = 2147483647u64;
        nat_le(a351, a352)
    } else {
        false
    } {
        f21(v5.clone(), v7.clone())?
    } else {
        false
    } {
        f21(v6.clone(), v7.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0(if f14(v0) {
            let a355 = 1u64;
            let a356 = {
                let a353 = f41(v0);
                let a354 = f30(v7.clone())?;
                nat_mul(a353, a354)?
            };
            nat_add(a355, a356)?
        } else {
            let a357 = 4u64;
            let a358 = f30(v7.clone())?;
            nat_mul(a357, a358)?
        }, if f14(v0) {
            4u64
        } else {
            3u64
        }, None::<u64>, if v1 {
            98u64
        } else {
            let a359 = 99u64;
            let a360 = v2;
            nat_add(a359, a360)?
        }, v0, v3, v4, v5, v6, v7)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f10(v0: u64, v1: Adt2) -> R<Adt3> {
    let a362 = v0;
    let a363 = match v1.clone() {
        Adt2::C0(h361, _, _, _, _, _, _, _, _, _) => {
            h361
        }
    };
    if nat_lt(a362, a363) {
        let v2: u64 = {
            let a367 = v0;
            let a368 = {
                let m366 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h365, _, _) => {
                        h365
                    }
                };
                match m366 {
                    Adt1::C0(_, h364, _, _, _, _, _, _, _, _, _) => {
                        h364
                    }
                }
            };
            nat_mul(a367, a368)?
        };
        Ok(Adt3::C7(f42(v2, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h369, _, _) => {
                h369
            }
        })?, f42(v2, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h370, _) => {
                h370
            }
        })?, f42(v2, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h371) => {
                h371
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f11(v0: u64, v1: Adt2) -> R<Adt3> {
    let a373 = v0;
    let a374 = match v1.clone() {
        Adt2::C0(h372, _, _, _, _, _, _, _, _, _) => {
            h372
        }
    };
    if nat_lt(a373, a374) {
        let v2: Adt1 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h375) => {
                h375
            }
        };
        let v3: u64 = f20(match v2.clone() {
            Adt1::C0(h376, _, _, _, _, _, _, _, _, _, _) => {
                h376
            }
        });
        let v4: u64 = {
            let a378 = match v2.clone() {
                Adt1::C0(_, h377, _, _, _, _, _, _, _, _, _) => {
                    h377
                }
            };
            let a379 = v3;
            let a380 = 0u64;
            nat_quot(a378, a379, a380)
        };
        let v5: u64 = {
            let a381 = v0;
            let a382 = v4;
            let a383 = 0u64;
            nat_quot(a381, a382, a383)
        };
        let v6: u64 = {
            let a385 = v5;
            let a386 = match v2.clone() {
                Adt1::C0(_, _, h384, _, _, _, _, _, _, _, _) => {
                    h384
                }
            };
            let a387 = 0u64;
            nat_quot(a385, a386, a387)
        };
        Ok(Adt3::C6(f42({
            let a388 = v0;
            let a389 = v3;
            nat_mul(a388, a389)?
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, h390, _, _) => {
                h390
            }
        })?, f43({
            let a391 = v0;
            let a392 = v4;
            let a393 = 0u64;
            nat_rem(a391, a392, a393)
        }, {
            let a395 = v5;
            let a396 = match v2.clone() {
                Adt1::C0(_, _, h394, _, _, _, _, _, _, _, _) => {
                    h394
                }
            };
            let a397 = 0u64;
            nat_rem(a395, a396, a397)
        }, {
            let a399 = v6;
            let a400 = match v2.clone() {
                Adt1::C0(_, _, _, h398, _, _, _, _, _, _, _) => {
                    h398
                }
            };
            let a401 = 0u64;
            nat_rem(a399, a400, a401)
        }, {
            let a403 = v6;
            let a404 = match v2.clone() {
                Adt1::C0(_, _, _, h402, _, _, _, _, _, _, _) => {
                    h402
                }
            };
            let a405 = 0u64;
            nat_quot(a403, a404, a405)
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, h406, _, _, _, _, _) => {
                h406
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h407, _, _, _, _) => {
                h407
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h408, _, _, _) => {
                h408
            }
        }, match v2 {
            Adt1::C0(_, _, _, _, _, _, _, _, h409, _, _) => {
                h409
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f12(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h410, _, _) => {
            h410
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h411) => {
            h411
        }
    };
    let a413 = v0;
    let a414 = match v1.clone() {
        Adt2::C0(h412, _, _, _, _, _, _, _, _, _) => {
            h412
        }
    };
    if nat_lt(a413, a414) {
        Ok(Adt3::C5({
            let a416 = v0;
            let a417 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, _, h415, _, _, _, _) => {
                    h415
                }
            };
            nat_mul(a416, a417)?
        }, {
            let a419 = v0;
            let a420 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h418, _, _, _, _) => {
                    h418
                }
            };
            nat_mul(a419, a420)?
        }, match v2 {
            Adt1::C0(_, h421, _, _, _, _, _, _, _, _, _) => {
                h421
            }
        }, match v3 {
            Adt1::C0(_, h422, _, _, _, _, _, _, _, _, _) => {
                h422
            }
        }, {
            let a424 = match v1 {
                Adt2::C0(_, _, _, _, h423, _, _, _, _, _) => {
                    h423
                }
            };
            let a425 = 1u64;
            nat_eq(a424, a425)
        }))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f13(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let a427 = v0;
    let a428 = match v1.clone() {
        Adt2::C0(h426, _, _, _, _, _, _, _, _, _) => {
            h426
        }
    };
    if nat_lt(a427, a428) {
        let v3: u64 = {
            let a430 = 2u64;
            let a431 = f30(match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h429, _) => {
                    h429
                }
            })?;
            nat_mul(a430, a431)?
        };
        let a432 = v0;
        let a433 = v3;
        if nat_lt(a432, a433) {
            f44(v0, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h434, _) => {
                    h434
                }
            }, {
                let m437 = match v1 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h436) => {
                        h436
                    }
                };
                match m437 {
                    Adt1::C0(_, _, h435, _, _, _, _, _, _, _, _) => {
                        h435
                    }
                }
            }, v2)
        } else {
            f45({
                let a438 = v0;
                let a439 = v3;
                nat_sub(a438, a439)
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h440, _, _) => {
                    h440
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h441, _) => {
                    h441
                }
            }, match v1 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h442) => {
                    h442
                }
            }, v2)
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f14(v0: u64) -> bool {
    let a443 = v0;
    let a444 = 3u64;
    if nat_eq(a443, a444) {
        true
    } else {
        let a445 = v0;
        let a446 = 6u64;
        nat_eq(a445, a446)
    }
}

pub fn f15(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h447, _, _, _) => {
            h447
        }
    };
    let a449 = v0;
    let a450 = match v1.clone() {
        Adt2::C0(h448, _, _, _, _, _, _, _, _, _) => {
            h448
        }
    };
    if nat_lt(a449, a450) {
        let a451 = v0;
        let a452 = 0u64;
        if nat_eq(a451, a452) {
            Ok(Adt3::C3(1u64, 6u64, 0u64, 0u64, 2u64, v3, 2u64, 0u64, 0u64, 3u64, 4u64, 4u64))
        } else {
            let v4: u64 = {
                let a453 = v0;
                let a454 = 1u64;
                nat_sub(a453, a454)
            };
            let v5: u64 = {
                let a456 = v4;
                let a457 = f41(match v1.clone() {
                    Adt2::C0(_, _, _, _, h455, _, _, _, _, _) => {
                        h455
                    }
                });
                let a458 = 0u64;
                nat_rem(a456, a457, a458)
            };
            let v6: u64 = {
                let a463 = 4u64;
                let a464 = {
                    let a460 = v4;
                    let a461 = f41(match v1.clone() {
                        Adt2::C0(_, _, _, _, h459, _, _, _, _, _) => {
                            h459
                        }
                    });
                    let a462 = 0u64;
                    nat_quot(a460, a461, a462)
                };
                nat_mul(a463, a464)?
            };
            let a465 = v5;
            let a466 = 2u64;
            if nat_lt(a465, a466) {
                f16({
                    let a467 = v6;
                    let a468 = v5;
                    nat_add(a467, a468)?
                }, v1)
            } else {
                let a469 = v5;
                let a470 = 2u64;
                if nat_eq(a469, a470) {
                    Ok(f46(0u64, 2u64, v3))
                } else {
                    let a471 = v5;
                    let a472 = 3u64;
                    if nat_eq(a471, a472) {
                        Ok(f47(0u64, 2u64, v3, {
                            let a473 = v2;
                            let a474 = 2u64;
                            nat_eq(a473, a474)
                        }))
                    } else {
                        let a475 = v5;
                        let a476 = 4u64;
                        if nat_eq(a475, a476) {
                            Ok(f46(1u64, 1u64, 3u64))
                        } else {
                            let a477 = v5;
                            let a478 = 5u64;
                            if nat_eq(a477, a478) {
                                Ok(f47(1u64, 1u64, 3u64, {
                                    let a479 = v2;
                                    let a480 = 0u64;
                                    nat_eq(a479, a480)
                                }))
                            } else {
                                let a481 = v5;
                                let a482 = 6u64;
                                if nat_eq(a481, a482) {
                                    Ok(f46(1u64, 2u64, v3))
                                } else {
                                    let a483 = v5;
                                    let a484 = 7u64;
                                    if nat_eq(a483, a484) {
                                        Ok(f47(1u64, 2u64, v3, {
                                            let a485 = v2;
                                            let a486 = 2u64;
                                            nat_eq(a485, a486)
                                        }))
                                    } else {
                                        let a488 = match v1.clone() {
                                            Adt2::C0(_, _, _, _, h487, _, _, _, _, _) => {
                                                h487
                                            }
                                        };
                                        let a489 = 3u64;
                                        if nat_eq(a488, a489) {
                                            let a490 = v5;
                                            let a491 = 15u64;
                                            if nat_eq(a490, a491) {
                                                f16({
                                                    let a492 = v6;
                                                    let a493 = 3u64;
                                                    nat_add(a492, a493)?
                                                }, v1)
                                            } else {
                                                Ok(f48(v5, match v1 {
                                                    Adt2::C0(_, _, _, _, _, h494, _, _, _, _) => {
                                                        h494
                                                    }
                                                }))
                                            }
                                        } else {
                                            let a495 = v5;
                                            let a496 = 8u64;
                                            if nat_eq(a495, a496) {
                                                Ok(Adt3::C4(2u64, {
                                                    let m499 = match v1 {
                                                        Adt2::C0(_, _, _, _, _, _, _, _, _, h498) => {
                                                            h498
                                                        }
                                                    };
                                                    match m499 {
                                                        Adt1::C0(h497, _, _, _, _, _, _, _, _, _, _) => {
                                                            h497
                                                        }
                                                    }
                                                }, 0u64, 1u64, 0u64))
                                            } else {
                                                f16({
                                                    let a500 = v6;
                                                    let a501 = 3u64;
                                                    nat_add(a500, a501)?
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
        Adt2::C0(_, _, _, _, _, _, _, h502, _, _) => {
            h502
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h503, _) => {
            h503
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h504) => {
            h504
        }
    };
    let v5: u64 = match v1.clone() {
        Adt2::C0(_, _, _, h505, _, _, _, _, _, _) => {
            h505
        }
    };
    let v6: u64 = {
        let a506 = v0;
        let a507 = 4u64;
        let a508 = 0u64;
        nat_rem(a506, a507, a508)
    };
    let v7: u64 = {
        let a509 = v0;
        let a510 = 4u64;
        let a511 = 0u64;
        nat_quot(a509, a510, a511)
    };
    let v8: u64 = {
        let a513 = v7;
        let a514 = match v4.clone() {
            Adt1::C0(_, h512, _, _, _, _, _, _, _, _, _) => {
                h512
            }
        };
        let a515 = 0u64;
        nat_quot(a513, a514, a515)
    };
    let v9: u64 = f37(match v4.clone() {
        Adt1::C0(h516, _, _, _, _, _, _, _, _, _, _) => {
            h516
        }
    });
    let v10: u64 = {
        let a521 = {
            let a518 = v7;
            let a519 = match v4.clone() {
                Adt1::C0(_, h517, _, _, _, _, _, _, _, _, _) => {
                    h517
                }
            };
            let a520 = 0u64;
            nat_rem(a518, a519, a520)
        };
        let a522 = v9;
        nat_mul(a521, a522)?
    };
    let v11: u64 = {
        let a524 = match v4.clone() {
            Adt1::C0(_, h523, _, _, _, _, _, _, _, _, _) => {
                h523
            }
        };
        let a525 = v9;
        nat_mul(a524, a525)?
    };
    let a527 = v0;
    let a528 = match v1.clone() {
        Adt2::C0(h526, _, _, _, _, _, _, _, _, _) => {
            h526
        }
    };
    if nat_lt(a527, a528) {
        let a529 = v6;
        let a530 = 0u64;
        if nat_eq(a529, a530) {
            Ok(Adt3::C3(0u64, 0u64, match v2.clone() {
                Adt1::C0(h531, _, _, _, _, _, _, _, _, _, _) => {
                    h531
                }
            }, 0u64, 0u64, {
                let a539 = {
                    let a535 = {
                        let a533 = v8;
                        let a534 = match v2 {
                            Adt1::C0(_, _, _, _, _, _, h532, _, _, _, _) => {
                                h532
                            }
                        };
                        nat_mul(a533, a534)?
                    };
                    let a536 = v10;
                    nat_add(a535, a536)?
                };
                let a540 = {
                    let a537 = v5;
                    let a538 = 100u64;
                    if nat_eq(a537, a538) {
                        v11
                    } else {
                        0u64
                    }
                };
                nat_add(a539, a540)?
            }, 2u64, 0u64, 0u64, 0u64, v9, 4u64))
        } else {
            let a541 = v6;
            let a542 = 1u64;
            if nat_eq(a541, a542) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h543, _, _, _, _, _, _, _, _, _, _) => {
                        h543
                    }
                }, 0u64, {
                    let a544 = v5;
                    let a545 = 98u64;
                    if nat_eq(a544, a545) {
                        4u64
                    } else {
                        0u64
                    }
                }, {
                    let a553 = {
                        let a549 = {
                            let a547 = v8;
                            let a548 = match v3 {
                                Adt1::C0(_, _, _, _, _, _, h546, _, _, _, _) => {
                                    h546
                                }
                            };
                            nat_mul(a547, a548)?
                        };
                        let a550 = v10;
                        nat_add(a549, a550)?
                    };
                    let a554 = {
                        let a551 = v5;
                        let a552 = 99u64;
                        if nat_eq(a551, a552) {
                            v11
                        } else {
                            0u64
                        }
                    };
                    nat_add(a553, a554)?
                }, 2u64, 0u64, 0u64, 1u64, v9, 4u64))
            } else {
                let a555 = v6;
                let a556 = 2u64;
                if nat_eq(a555, a556) {
                    Ok(Adt3::C4(match v1 {
                        Adt2::C0(_, _, _, _, h557, _, _, _, _, _) => {
                            h557
                        }
                    }, match v4 {
                        Adt1::C0(h558, _, _, _, _, _, _, _, _, _, _) => {
                            h558
                        }
                    }, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt3::C3(0u64, 0u64, 0u64, match v4.clone() {
                        Adt1::C0(h559, _, _, _, _, _, _, _, _, _, _) => {
                            h559
                        }
                    }, 1u64, 0u64, 2u64, 0u64, 3u64, {
                        let a563 = {
                            let a561 = v8;
                            let a562 = match v4 {
                                Adt1::C0(_, _, _, _, _, _, h560, _, _, _, _) => {
                                    h560
                                }
                            };
                            nat_mul(a561, a562)?
                        };
                        let a564 = v10;
                        nat_add(a563, a564)?
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
        Adt2::C0(_, _, _, _, _, _, _, h565, _, _) => {
            h565
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h566, _) => {
            h566
        }
    };
    let v5: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h567) => {
            h567
        }
    };
    let v6: u64 = {
        let a569 = match v1.clone() {
            Adt2::C0(_, h568, _, _, _, _, _, _, _, _) => {
                h568
            }
        };
        let a570 = 3u64;
        if nat_eq(a569, a570) {
            0u64
        } else {
            let a571 = f30(v3.clone())?;
            let a572 = 2u64;
            nat_mul(a571, a572)?
        }
    };
    let a574 = match v1.clone() {
        Adt2::C0(h573, _, _, _, _, _, _, _, _, _) => {
            h573
        }
    };
    let a575 = v0;
    if nat_le(a574, a575) {
        Ok(Adt3::C1)
    } else {
        let a576 = v0;
        let a577 = v6;
        if nat_lt(a576, a577) {
            let v7: u64 = {
                let a578 = v0;
                let a579 = 2u64;
                let a580 = 0u64;
                nat_quot(a578, a579, a580)
            };
            let v8: u64 = {
                let a585 = 3u64;
                let a586 = {
                    let a582 = f42(v7, v5.clone())?;
                    let a583 = f25(match v5 {
                        Adt1::C0(h581, _, _, _, _, _, _, _, _, _, _) => {
                            h581
                        }
                    });
                    let a584 = 0u64;
                    nat_quot(a582, a583, a584)
                };
                nat_add(a585, a586)?
            };
            let a590 = {
                let a587 = v0;
                let a588 = 2u64;
                let a589 = 0u64;
                nat_rem(a587, a588, a589)
            };
            let a591 = 0u64;
            if nat_eq(a590, a591) {
                Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, v8, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
            } else {
                let a592 = v2;
                let a593 = 0u64;
                if nat_eq(a592, a593) {
                    Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 2u64, 1u64, 2u64, 0u64, 0u64, v8, 8u64, 8u64))
                } else {
                    Ok(Adt3::C1)
                }
            }
        } else {
            let v9: u64 = {
                let a594 = v0;
                let a595 = v6;
                nat_sub(a594, a595)
            };
            let v10: u64 = f31(match v3.clone() {
                Adt1::C0(h596, _, _, _, _, _, _, _, _, _, _) => {
                    h596
                }
            }, match v4.clone() {
                Adt1::C0(h597, _, _, _, _, _, _, _, _, _, _) => {
                    h597
                }
            }, match v5.clone() {
                Adt1::C0(h598, _, _, _, _, _, _, _, _, _, _) => {
                    h598
                }
            })?;
            let v11: u64 = {
                let a599 = v9;
                let a600 = v10;
                let a601 = 0u64;
                nat_quot(a599, a600, a601)
            };
            let v12: u64 = {
                let a602 = v9;
                let a603 = v10;
                let a604 = 0u64;
                nat_rem(a602, a603, a604)
            };
            let v13: u64 = {
                let a606 = match v3.clone() {
                    Adt1::C0(h605, _, _, _, _, _, _, _, _, _, _) => {
                        h605
                    }
                };
                let a607 = 0u64;
                if nat_eq(a606, a607) {
                    0u64
                } else {
                    1u64
                }
            };
            let v14: u64 = {
                let a609 = match v4.clone() {
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
            let a611 = v12;
            let a612 = v13;
            if nat_lt(a611, a612) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h613, _, _, _, _, _, _, _, _, _, _) => {
                        h613
                    }
                }, 0u64, 0u64, f42(v11, v3.clone())?, 2u64, 0u64, 0u64, 0u64, f25(match v3 {
                    Adt1::C0(h614, _, _, _, _, _, _, _, _, _, _) => {
                        h614
                    }
                }), 4u64))
            } else {
                let a617 = v12;
                let a618 = {
                    let a615 = v13;
                    let a616 = v14;
                    nat_add(a615, a616)?
                };
                if nat_lt(a617, a618) {
                    Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                        Adt1::C0(h619, _, _, _, _, _, _, _, _, _, _) => {
                            h619
                        }
                    }, 0u64, 4u64, f49(v11, v3, v4.clone())?, 2u64, 0u64, 0u64, 1u64, f25(match v4 {
                        Adt1::C0(h620, _, _, _, _, _, _, _, _, _, _) => {
                            h620
                        }
                    }), 4u64))
                } else {
                    let a623 = v12;
                    let a624 = {
                        let a621 = v13;
                        let a622 = v14;
                        nat_add(a621, a622)?
                    };
                    if nat_eq(a623, a624) {
                        Ok(Adt3::C3(1u64, match v1 {
                            Adt2::C0(_, _, _, h625, _, _, _, _, _, _) => {
                                h625
                            }
                        }, 0u64, 0u64, {
                            let a626 = v13;
                            let a627 = 0u64;
                            if nat_eq(a626, a627) {
                                0u64
                            } else {
                                1u64
                            }
                        }, {
                            let a628 = v13;
                            let a629 = 0u64;
                            if nat_eq(a628, a629) {
                                f42(v11, v3.clone())?
                            } else {
                                0u64
                            }
                        }, {
                            let a630 = v14;
                            let a631 = 0u64;
                            if nat_eq(a630, a631) {
                                4u64
                            } else {
                                1u64
                            }
                        }, {
                            let a632 = v14;
                            let a633 = 0u64;
                            if nat_eq(a632, a633) {
                                f49(v11, v3, v4)?
                            } else {
                                1u64
                            }
                        }, {
                            let a635 = match v5.clone() {
                                Adt1::C0(h634, _, _, _, _, _, _, _, _, _, _) => {
                                    h634
                                }
                            };
                            let a636 = 0u64;
                            if nat_eq(a635, a636) {
                                3u64
                            } else {
                                0u64
                            }
                        }, {
                            let a638 = match v5.clone() {
                                Adt1::C0(h637, _, _, _, _, _, _, _, _, _, _) => {
                                    h637
                                }
                            };
                            let a639 = 0u64;
                            if nat_eq(a638, a639) {
                                f42(v11, v5)?
                            } else {
                                0u64
                            }
                        }, 4u64, 4u64))
                    } else {
                        Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                            Adt1::C0(h640, _, _, _, _, _, _, _, _, _, _) => {
                                h640
                            }
                        }, 1u64, 0u64, 2u64, 0u64, 3u64, f42(v11, v5.clone())?, 4u64, f25(match v5 {
                            Adt1::C0(h641, _, _, _, _, _, _, _, _, _, _) => {
                                h641
                            }
                        })))
                    }
                }
            }
        }
    }
}

pub fn f18(v0: Adt1, v1: Adt1) -> bool {
    let a644 = match v0.clone() {
        Adt1::C0(_, h642, _, _, _, _, _, _, _, _, _) => {
            h642
        }
    };
    let a645 = match v1.clone() {
        Adt1::C0(_, h643, _, _, _, _, _, _, _, _, _) => {
            h643
        }
    };
    if if if nat_eq(a644, a645) {
        let a648 = match v0.clone() {
            Adt1::C0(_, _, h646, _, _, _, _, _, _, _, _) => {
                h646
            }
        };
        let a649 = match v1.clone() {
            Adt1::C0(_, _, h647, _, _, _, _, _, _, _, _) => {
                h647
            }
        };
        nat_eq(a648, a649)
    } else {
        false
    } {
        let a652 = match v0.clone() {
            Adt1::C0(_, _, _, h650, _, _, _, _, _, _, _) => {
                h650
            }
        };
        let a653 = match v1.clone() {
            Adt1::C0(_, _, _, h651, _, _, _, _, _, _, _) => {
                h651
            }
        };
        nat_eq(a652, a653)
    } else {
        false
    } {
        let a656 = match v0 {
            Adt1::C0(_, _, _, _, h654, _, _, _, _, _, _) => {
                h654
            }
        };
        let a657 = match v1 {
            Adt1::C0(_, _, _, _, h655, _, _, _, _, _, _) => {
                h655
            }
        };
        nat_eq(a656, a657)
    } else {
        false
    }
}

pub fn f19(v0: u64) -> u64 {
    if f2(v0) {
        let m658 = f50(v0);
        match m658 {
            Adt4::C0 => {
                0u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, _, h659) => {
                        h659
                    }
                }
            }
        }
    } else {
        let a660 = v0;
        let a661 = 26u64;
        if nat_eq(a660, a661) {
            4u64
        } else {
            f25(v0)
        }
    }
}

pub fn f20(v0: u64) -> u64 {
    if f2(v0) {
        let m662 = f50(v0);
        match m662 {
            Adt4::C0 => {
                1u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, h663, _) => {
                        h663
                    }
                }
            }
        }
    } else {
        1u64
    }
}

pub fn f21(v0: Adt1, v1: Adt1) -> R<bool> {
    if f39(v0.clone(), v1.clone())? {
        Ok(true)
    } else {
        Ok(f51(v0, v1))
    }
}

pub fn f22(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f52(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f52(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f52(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f52(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f52(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f52(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f23(v0: Adt1) -> bool {
    f53(match v0.clone() {
        Adt1::C0(_, h664, _, _, _, _, _, _, _, _, _) => {
            h664
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h665, _, _, _, _, _, _, _, _) => {
            h665
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h666, _, _, _, _, _, _, _) => {
            h666
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, h667, _, _, _, _, _, _) => {
            h667
        }
    })
}

pub fn f24(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<bool> {
    let a669 = match v1.clone() {
        Adt1::C0(h668, _, _, _, _, _, _, _, _, _, _) => {
            h668
        }
    };
    let a670 = 43u64;
    if nat_eq(a669, a670) {
        let a671 = v0;
        let a672 = 0u64;
        if if if if if if nat_eq(a671, a672) {
            let a674 = match v2.clone() {
                Adt1::C0(h673, _, _, _, _, _, _, _, _, _, _) => {
                    h673
                }
            };
            let a675 = 0u64;
            nat_eq(a674, a675)
        } else {
            false
        } {
            let a677 = match v3.clone() {
                Adt1::C0(h676, _, _, _, _, _, _, _, _, _, _) => {
                    h676
                }
            };
            let a678 = 0u64;
            if nat_eq(a677, a678) {
                true
            } else {
                let a680 = match v3.clone() {
                    Adt1::C0(h679, _, _, _, _, _, _, _, _, _, _) => {
                        h679
                    }
                };
                let a681 = 43u64;
                nat_eq(a680, a681)
            }
        } else {
            false
        } {
            f18(v1.clone(), v2.clone())
        } else {
            false
        } {
            let a683 = match v2 {
                Adt1::C0(_, _, _, _, _, h682, _, _, _, _, _) => {
                    h682
                }
            };
            let a684 = 4u64;
            nat_eq(a683, a684)
        } else {
            false
        } {
            let a687 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h685, _, _, _, _, _) => {
                    h685
                }
            };
            let a688 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h686, _, _, _, _) => {
                    h686
                }
            };
            if nat_le(a687, a688) {
                let a691 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, h689, _, _, _, _) => {
                        h689
                    }
                };
                let a692 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h690, _, _, _) => {
                        h690
                    }
                };
                if nat_le(a691, a692) {
                    let a695 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, h693, _, _, _) => {
                            h693
                        }
                    };
                    let a696 = match v3 {
                        Adt1::C0(_, _, _, _, _, _, _, _, h694, _, _) => {
                            h694
                        }
                    };
                    nat_le(a695, a696)
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        } {
            let a704 = {
                let a702 = match v1.clone() {
                    Adt1::C0(_, _, h697, _, _, _, _, _, _, _, _) => {
                        h697
                    }
                };
                let a703 = {
                    let a700 = match v1.clone() {
                        Adt1::C0(_, _, _, h698, _, _, _, _, _, _, _) => {
                            h698
                        }
                    };
                    let a701 = match v1 {
                        Adt1::C0(_, _, _, _, h699, _, _, _, _, _, _) => {
                            h699
                        }
                    };
                    nat_mul(a700, a701)?
                };
                nat_mul(a702, a703)?
            };
            let a705 = 2147483647u64;
            Ok(nat_le(a704, a705))
        } else {
            Ok(false)
        }
    } else {
        if f54(v1.clone(), v2.clone()) {
            Ok(f55(match v1 {
                Adt1::C0(h706, _, _, _, _, _, _, _, _, _, _) => {
                    h706
                }
            }, match v2 {
                Adt1::C0(h707, _, _, _, _, _, _, _, _, _, _) => {
                    h707
                }
            }, match v3 {
                Adt1::C0(h708, _, _, _, _, _, _, _, _, _, _) => {
                    h708
                }
            }))
        } else {
            Ok(false)
        }
    }
}

pub fn f25(v0: u64) -> u64 {
    let a709 = v0;
    let a710 = 43u64;
    if nat_eq(a709, a710) {
        1u64
    } else {
        f37(v0)
    }
}

pub fn f26(v0: Adt1) -> bool {
    let v1: u64 = f25(match v0.clone() {
        Adt1::C0(h711, _, _, _, _, _, _, _, _, _, _) => {
            h711
        }
    });
    let a716 = {
        let a713 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h712) => {
                h712
            }
        };
        let a714 = v1;
        let a715 = 1u64;
        nat_rem(a713, a714, a715)
    };
    let a717 = 0u64;
    if if nat_eq(a716, a717) {
        f56(match v0.clone() {
            Adt1::C0(_, h718, _, _, _, _, _, _, _, _, _) => {
                h718
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h719, _, _, _, _, _) => {
                h719
            }
        }, v1)
    } else {
        false
    } {
        f57(match v0.clone() {
            Adt1::C0(_, _, h720, _, _, _, _, _, _, _, _) => {
                h720
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h721, _, _, _, _, _, _, _) => {
                h721
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h722, _, _, _, _, _, _) => {
                h722
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h723, _, _, _, _) => {
                h723
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h724, _, _, _) => {
                h724
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h725, _, _) => {
                h725
            }
        }, v1)
    } else {
        false
    }
}

pub fn f27(v0: Adt1, v1: Adt1) -> R<bool> {
    let a733 = {
        let a731 = match v0.clone() {
            Adt1::C0(_, _, h726, _, _, _, _, _, _, _, _) => {
                h726
            }
        };
        let a732 = {
            let a729 = match v0.clone() {
                Adt1::C0(_, _, _, h727, _, _, _, _, _, _, _) => {
                    h727
                }
            };
            let a730 = match v0.clone() {
                Adt1::C0(_, _, _, _, h728, _, _, _, _, _, _) => {
                    h728
                }
            };
            nat_mul(a729, a730)?
        };
        nat_mul(a731, a732)?
    };
    let a734 = 9223372036854775807u64;
    if nat_le(a733, a734) {
        if f34(v1.clone()) {
            let a736 = match v1 {
                Adt1::C0(_, h735, _, _, _, _, _, _, _, _, _) => {
                    h735
                }
            };
            let a737 = 2147483647u64;
            Ok(nat_le(a736, a737))
        } else {
            let a739 = match v0 {
                Adt1::C0(_, h738, _, _, _, _, _, _, _, _, _) => {
                    h738
                }
            };
            let a740 = 2147483647u64;
            Ok(nat_le(a739, a740))
        }
    } else {
        Ok(false)
    }
}

pub fn f28(v0: Adt1) -> R<bool> {
    f22({
        let a743 = match v0.clone() {
            Adt1::C0(_, h741, _, _, _, _, _, _, _, _, _) => {
                h741
            }
        };
        let a744 = f25(match v0.clone() {
            Adt1::C0(h742, _, _, _, _, _, _, _, _, _, _) => {
                h742
            }
        });
        nat_mul(a743, a744)?
    }, match v0.clone() {
        Adt1::C0(_, _, h745, _, _, _, _, _, _, _, _) => {
            h745
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h746, _, _, _, _, _, _, _) => {
            h746
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h747, _, _, _, _, _, _) => {
            h747
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h748, _, _, _, _) => {
            h748
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h749, _, _, _) => {
            h749
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h750, _, _) => {
            h750
        }
    })
}

pub fn f29(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a757 = {
        let a751 = v0;
        let a752 = v2;
        nat_mul(a751, a752)?
    };
    let a758 = {
        let a753 = v1;
        let a754 = 0u64;
        if nat_eq(a753, a754) {
            0u64
        } else {
            let a755 = v0;
            let a756 = 2u64;
            nat_mul(a755, a756)?
        }
    };
    nat_add(a757, a758)
}

pub fn f30(v0: Adt1) -> R<u64> {
    let a767 = {
        let a761 = match v0.clone() {
            Adt1::C0(_, h759, _, _, _, _, _, _, _, _, _) => {
                h759
            }
        };
        let a762 = match v0.clone() {
            Adt1::C0(_, _, h760, _, _, _, _, _, _, _, _) => {
                h760
            }
        };
        nat_mul(a761, a762)?
    };
    let a768 = {
        let a765 = match v0.clone() {
            Adt1::C0(_, _, _, h763, _, _, _, _, _, _, _) => {
                h763
            }
        };
        let a766 = match v0 {
            Adt1::C0(_, _, _, _, h764, _, _, _, _, _, _) => {
                h764
            }
        };
        nat_mul(a765, a766)?
    };
    nat_mul(a767, a768)
}

pub fn f31(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a779 = {
        let a775 = {
            let a771 = 1u64;
            let a772 = {
                let a769 = v0;
                let a770 = 0u64;
                if nat_eq(a769, a770) {
                    0u64
                } else {
                    1u64
                }
            };
            nat_add(a771, a772)?
        };
        let a776 = {
            let a773 = v1;
            let a774 = 0u64;
            if nat_eq(a773, a774) {
                0u64
            } else {
                1u64
            }
        };
        nat_add(a775, a776)?
    };
    let a780 = {
        let a777 = v2;
        let a778 = 0u64;
        if nat_eq(a777, a778) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a779, a780)
}

pub fn f32(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f18(v0.clone(), v1.clone()) {
        let a783 = match v0.clone() {
            Adt1::C0(h781, _, _, _, _, _, _, _, _, _, _) => {
                h781
            }
        };
        let a784 = match v1.clone() {
            Adt1::C0(h782, _, _, _, _, _, _, _, _, _, _) => {
                h782
            }
        };
        nat_eq(a783, a784)
    } else {
        false
    } {
        let a787 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h785, _, _, _, _, _) => {
                h785
            }
        };
        let a788 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h786, _, _, _, _, _) => {
                h786
            }
        };
        nat_eq(a787, a788)
    } else {
        false
    } {
        let a791 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h789, _, _, _, _) => {
                h789
            }
        };
        let a792 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h790, _, _, _, _) => {
                h790
            }
        };
        nat_eq(a791, a792)
    } else {
        false
    } {
        let a795 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h793, _, _, _) => {
                h793
            }
        };
        let a796 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h794, _, _, _) => {
                h794
            }
        };
        nat_eq(a795, a796)
    } else {
        false
    } {
        let a799 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h797, _, _) => {
                h797
            }
        };
        let a800 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h798, _, _) => {
                h798
            }
        };
        nat_eq(a799, a800)
    } else {
        false
    } {
        let a802 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h801) => {
                h801
            }
        };
        let a803 = 0u64;
        if if nat_eq(a802, a803) {
            true
        } else {
            let a805 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h804) => {
                    h804
                }
            };
            let a806 = 0u64;
            nat_eq(a805, a806)
        } {
            true
        } else {
            let a809 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h807) => {
                    h807
                }
            };
            let a810 = match v1 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h808) => {
                    h808
                }
            };
            nat_eq(a809, a810)
        }
    } else {
        false
    }
}

pub fn f33(v0: u64) -> u64 {
    let a811 = v0;
    let a812 = 26u64;
    if nat_eq(a811, a812) {
        4u64
    } else {
        let a813 = v0;
        let a814 = 27u64;
        if nat_eq(a813, a814) {
            8u64
        } else {
            0u64
        }
    }
}

pub fn f34(v0: Adt1) -> bool {
    let a816 = match v0.clone() {
        Adt1::C0(_, h815, _, _, _, _, _, _, _, _, _) => {
            h815
        }
    };
    let a817 = 1u64;
    if nat_eq(a816, a817) {
        true
    } else {
        let a820 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h818, _, _, _, _, _) => {
                h818
            }
        };
        let a821 = f25(match v0 {
            Adt1::C0(h819, _, _, _, _, _, _, _, _, _, _) => {
                h819
            }
        });
        nat_eq(a820, a821)
    }
}

pub fn f35(v0: Adt1, v1: u64) -> R<u64> {
    let a827 = {
        let a823 = match v0.clone() {
            Adt1::C0(_, h822, _, _, _, _, _, _, _, _, _) => {
                h822
            }
        };
        let a824 = f20(v1);
        let a825 = 0u64;
        nat_quot(a823, a824, a825)
    };
    let a828 = f58(match v0 {
        Adt1::C0(h826, _, _, _, _, _, _, _, _, _, _) => {
            h826
        }
    }, v1);
    nat_mul(a827, a828)
}

pub fn f36(v0: u64) -> Option<u64> {
    let a829 = v0;
    let a830 = 16u64;
    if if if nat_eq(a829, a830) {
        true
    } else {
        let a831 = v0;
        let a832 = 17u64;
        nat_eq(a831, a832)
    } {
        true
    } else {
        let a833 = v0;
        let a834 = 19u64;
        nat_eq(a833, a834)
    } {
        Some(1065353216u64)
    } else {
        None::<u64>
    }
}

pub fn f37(v0: u64) -> u64 {
    let a835 = v0;
    let a836 = 0u64;
    if nat_eq(a835, a836) {
        4u64
    } else {
        let a837 = v0;
        let a838 = 1u64;
        if if nat_eq(a837, a838) {
            true
        } else {
            let a839 = v0;
            let a840 = 30u64;
            nat_eq(a839, a840)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f38(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a842 = match v0.clone() {
            Adt1::C0(_, h841, _, _, _, _, _, _, _, _, _) => {
                h841
            }
        };
        let a843 = 4u64;
        nat_mul(a842, a843)?
    };
    let v2: u64 = {
        let a846 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h844, _, _, _, _) => {
                h844
            }
        };
        let a847 = match v0.clone() {
            Adt1::C0(_, _, h845, _, _, _, _, _, _, _, _) => {
                h845
            }
        };
        nat_mul(a846, a847)?
    };
    let v3: u64 = {
        let a849 = v2;
        let a850 = match v0.clone() {
            Adt1::C0(_, _, _, h848, _, _, _, _, _, _, _) => {
                h848
            }
        };
        nat_mul(a849, a850)?
    };
    let a852 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h851, _, _, _, _, _) => {
            h851
        }
    };
    let a853 = 4u64;
    if if if if nat_eq(a852, a853) {
        let a858 = {
            let a855 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h854) => {
                    h854
                }
            };
            let a856 = 4u64;
            let a857 = 1u64;
            nat_rem(a855, a856, a857)
        };
        let a859 = 0u64;
        nat_eq(a858, a859)
    } else {
        false
    } {
        let a867 = {
            let a865 = match v0.clone() {
                Adt1::C0(_, _, h860, _, _, _, _, _, _, _, _) => {
                    h860
                }
            };
            let a866 = {
                let a863 = match v0.clone() {
                    Adt1::C0(_, _, _, h861, _, _, _, _, _, _, _) => {
                        h861
                    }
                };
                let a864 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h862, _, _, _, _, _, _) => {
                        h862
                    }
                };
                nat_mul(a863, a864)?
            };
            nat_mul(a865, a866)?
        };
        let a868 = 1u64;
        if nat_le(a867, a868) {
            true
        } else {
            let a870 = v1;
            let a871 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h869, _, _, _, _) => {
                    h869
                }
            };
            if nat_le(a870, a871) {
                let a876 = {
                    let a873 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, h872, _, _, _, _) => {
                            h872
                        }
                    };
                    let a874 = 4u64;
                    let a875 = 1u64;
                    nat_rem(a873, a874, a875)
                };
                let a877 = 0u64;
                nat_eq(a876, a877)
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a879 = match v0.clone() {
            Adt1::C0(_, _, _, h878, _, _, _, _, _, _, _) => {
                h878
            }
        };
        let a880 = 1u64;
        if nat_le(a879, a880) {
            true
        } else {
            let a882 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h881, _, _, _) => {
                    h881
                }
            };
            let a883 = v2;
            nat_eq(a882, a883)
        }
    } else {
        false
    } {
        let a885 = match v0.clone() {
            Adt1::C0(_, _, _, _, h884, _, _, _, _, _, _) => {
                h884
            }
        };
        let a886 = 1u64;
        if nat_le(a885, a886) {
            Ok(true)
        } else {
            let a888 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, h887, _, _) => {
                    h887
                }
            };
            let a889 = v3;
            Ok(nat_eq(a888, a889))
        }
    } else {
        Ok(false)
    }
}

pub fn f39(v0: Adt1, v1: Adt1) -> R<bool> {
    let a891 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h890, _) => {
            h890
        }
    };
    let a892 = 0u64;
    if nat_eq(a891, a892) {
        Ok(true)
    } else {
        let a894 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h893, _) => {
                h893
            }
        };
        let a895 = 0u64;
        if nat_eq(a894, a895) {
            Ok(true)
        } else {
            let a897 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h896) => {
                    h896
                }
            };
            let a898 = 0u64;
            if nat_eq(a897, a898) {
                Ok(true)
            } else {
                let a900 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h899) => {
                        h899
                    }
                };
                let a901 = 0u64;
                if nat_eq(a900, a901) {
                    Ok(true)
                } else {
                    let a907 = {
                        let a904 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h902) => {
                                h902
                            }
                        };
                        let a905 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h903, _) => {
                                h903
                            }
                        };
                        nat_add(a904, a905)?
                    };
                    let a908 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h906) => {
                            h906
                        }
                    };
                    if nat_le(a907, a908) {
                        Ok(true)
                    } else {
                        let a914 = {
                            let a911 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h909) => {
                                    h909
                                }
                            };
                            let a912 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h910, _) => {
                                    h910
                                }
                            };
                            nat_add(a911, a912)?
                        };
                        let a915 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h913) => {
                                h913
                            }
                        };
                        Ok(nat_le(a914, a915))
                    }
                }
            }
        }
    }
}

pub fn f40(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a918 = match v0.clone() {
            Adt1::C0(_, _, h916, _, _, _, _, _, _, _, _) => {
                h916
            }
        };
        let a919 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h917, _, _, _, _) => {
                h917
            }
        };
        nat_mul(a918, a919)?
    };
    if if f34(v0.clone()) {
        let a921 = match v0.clone() {
            Adt1::C0(_, _, _, h920, _, _, _, _, _, _, _) => {
                h920
            }
        };
        let a922 = 1u64;
        if nat_eq(a921, a922) {
            true
        } else {
            let a924 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h923, _, _, _) => {
                    h923
                }
            };
            let a925 = v1;
            nat_eq(a924, a925)
        }
    } else {
        false
    } {
        let a927 = match v0.clone() {
            Adt1::C0(_, _, _, _, h926, _, _, _, _, _, _) => {
                h926
            }
        };
        let a928 = 1u64;
        if nat_eq(a927, a928) {
            Ok(true)
        } else {
            let a933 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h929, _, _) => {
                    h929
                }
            };
            let a934 = {
                let a931 = v1;
                let a932 = match v0 {
                    Adt1::C0(_, _, _, h930, _, _, _, _, _, _, _) => {
                        h930
                    }
                };
                nat_mul(a931, a932)?
            };
            Ok(nat_eq(a933, a934))
        }
    } else {
        Ok(false)
    }
}

pub fn f41(v0: u64) -> u64 {
    let a935 = v0;
    let a936 = 3u64;
    if nat_eq(a935, a936) {
        16u64
    } else {
        10u64
    }
}

pub fn f42(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a938 = v0;
        let a939 = match v1.clone() {
            Adt1::C0(_, h937, _, _, _, _, _, _, _, _, _) => {
                h937
            }
        };
        let a940 = 0u64;
        nat_quot(a938, a939, a940)
    };
    let v3: u64 = {
        let a942 = v2;
        let a943 = match v1.clone() {
            Adt1::C0(_, _, h941, _, _, _, _, _, _, _, _) => {
                h941
            }
        };
        let a944 = 0u64;
        nat_quot(a942, a943, a944)
    };
    f43({
        let a946 = v0;
        let a947 = match v1.clone() {
            Adt1::C0(_, h945, _, _, _, _, _, _, _, _, _) => {
                h945
            }
        };
        let a948 = 0u64;
        nat_rem(a946, a947, a948)
    }, {
        let a950 = v2;
        let a951 = match v1.clone() {
            Adt1::C0(_, _, h949, _, _, _, _, _, _, _, _) => {
                h949
            }
        };
        let a952 = 0u64;
        nat_rem(a950, a951, a952)
    }, {
        let a954 = v3;
        let a955 = match v1.clone() {
            Adt1::C0(_, _, _, h953, _, _, _, _, _, _, _) => {
                h953
            }
        };
        let a956 = 0u64;
        nat_rem(a954, a955, a956)
    }, {
        let a958 = v3;
        let a959 = match v1.clone() {
            Adt1::C0(_, _, _, h957, _, _, _, _, _, _, _) => {
                h957
            }
        };
        let a960 = 0u64;
        nat_quot(a958, a959, a960)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h961, _, _, _, _, _) => {
            h961
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h962, _, _, _, _) => {
            h962
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h963, _, _, _) => {
            h963
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h964, _, _) => {
            h964
        }
    })
}

pub fn f43(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a977 = {
        let a973 = {
            let a969 = {
                let a965 = v0;
                let a966 = v4;
                nat_mul(a965, a966)?
            };
            let a970 = {
                let a967 = v1;
                let a968 = v5;
                nat_mul(a967, a968)?
            };
            nat_add(a969, a970)?
        };
        let a974 = {
            let a971 = v2;
            let a972 = v6;
            nat_mul(a971, a972)?
        };
        nat_add(a973, a974)?
    };
    let a978 = {
        let a975 = v3;
        let a976 = v7;
        nat_mul(a975, a976)?
    };
    nat_add(a977, a978)
}

pub fn f44(v0: u64, v1: Adt1, v2: u64, v3: u64) -> R<Adt3> {
    let v4: u64 = {
        let a979 = v0;
        let a980 = 2u64;
        let a981 = 0u64;
        nat_quot(a979, a980, a981)
    };
    let v5: u64 = f33(match v1.clone() {
        Adt1::C0(h982, _, _, _, _, _, _, _, _, _, _) => {
            h982
        }
    });
    let a986 = {
        let a983 = v0;
        let a984 = 2u64;
        let a985 = 0u64;
        nat_rem(a983, a984, a985)
    };
    let a987 = 0u64;
    if nat_eq(a986, a987) {
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 4u64, f42(v4, v1)?, 2u64, 0u64, 0u64, 2u64, v5, 8u64))
    } else {
        let a988 = v3;
        let a989 = v2;
        if if nat_lt(a988, a989) {
            let a992 = v3;
            let a993 = {
                let a990 = v5;
                let a991 = 4u64;
                if nat_eq(a990, a991) {
                    2147483648u64
                } else {
                    9223372036854775808u64
                }
            };
            nat_lt(a992, a993)
        } else {
            false
        } {
            Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, {
                let a994 = 3u64;
                let a995 = v4;
                nat_add(a994, a995)?
            }, 8u64, 8u64))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f45(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64) -> R<Adt3> {
    let v5: u64 = {
        let a997 = 1u64;
        let a998 = f35(v1.clone(), match v3.clone() {
            Adt1::C0(h996, _, _, _, _, _, _, _, _, _, _) => {
                h996
            }
        })?;
        nat_add(a997, a998)?
    };
    let v6: u64 = {
        let a999 = v0;
        let a1000 = v5;
        let a1001 = 0u64;
        nat_rem(a999, a1000, a1001)
    };
    let v7: u64 = {
        let a1002 = v0;
        let a1003 = v5;
        let a1004 = 0u64;
        nat_quot(a1002, a1003, a1004)
    };
    let v8: u64 = {
        let a1006 = v7;
        let a1007 = match v1.clone() {
            Adt1::C0(_, _, h1005, _, _, _, _, _, _, _, _) => {
                h1005
            }
        };
        let a1008 = 0u64;
        nat_rem(a1006, a1007, a1008)
    };
    let v9: u64 = {
        let a1010 = v7;
        let a1011 = match v1.clone() {
            Adt1::C0(_, _, h1009, _, _, _, _, _, _, _, _) => {
                h1009
            }
        };
        let a1012 = 0u64;
        nat_quot(a1010, a1011, a1012)
    };
    let v10: u64 = {
        let a1014 = v9;
        let a1015 = match v1.clone() {
            Adt1::C0(_, _, _, h1013, _, _, _, _, _, _, _) => {
                h1013
            }
        };
        let a1016 = 0u64;
        nat_rem(a1014, a1015, a1016)
    };
    let v11: u64 = {
        let a1018 = v9;
        let a1019 = match v1.clone() {
            Adt1::C0(_, _, _, h1017, _, _, _, _, _, _, _) => {
                h1017
            }
        };
        let a1020 = 0u64;
        nat_quot(a1018, a1019, a1020)
    };
    let a1021 = v6;
    let a1022 = 0u64;
    if nat_eq(a1021, a1022) {
        let v12: u64 = {
            let a1039 = v8;
            let a1040 = {
                let a1037 = match v2.clone() {
                    Adt1::C0(_, h1023, _, _, _, _, _, _, _, _, _) => {
                        h1023
                    }
                };
                let a1038 = {
                    let a1035 = {
                        let a1025 = v10;
                        let a1026 = match v2.clone() {
                            Adt1::C0(_, _, h1024, _, _, _, _, _, _, _, _) => {
                                h1024
                            }
                        };
                        let a1027 = 0u64;
                        nat_rem(a1025, a1026, a1027)
                    };
                    let a1036 = {
                        let a1033 = match v2.clone() {
                            Adt1::C0(_, _, h1028, _, _, _, _, _, _, _, _) => {
                                h1028
                            }
                        };
                        let a1034 = {
                            let a1030 = v11;
                            let a1031 = match v2 {
                                Adt1::C0(_, _, _, h1029, _, _, _, _, _, _, _) => {
                                    h1029
                                }
                            };
                            let a1032 = 0u64;
                            nat_rem(a1030, a1031, a1032)
                        };
                        nat_mul(a1033, a1034)?
                    };
                    nat_add(a1035, a1036)?
                };
                nat_mul(a1037, a1038)?
            };
            nat_add(a1039, a1040)?
        };
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, {
            let a1041 = 3u64;
            let a1042 = v12;
            nat_add(a1041, a1042)?
        }, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
    } else {
        let a1044 = v4;
        let a1045 = match v3.clone() {
            Adt1::C0(_, _, h1043, _, _, _, _, _, _, _, _) => {
                h1043
            }
        };
        if nat_lt(a1044, a1045) {
            let v13: u64 = {
                let a1050 = {
                    let a1046 = v6;
                    let a1047 = 1u64;
                    nat_sub(a1046, a1047)
                };
                let a1051 = f58(match v1.clone() {
                    Adt1::C0(h1048, _, _, _, _, _, _, _, _, _, _) => {
                        h1048
                    }
                }, match v3.clone() {
                    Adt1::C0(h1049, _, _, _, _, _, _, _, _, _, _) => {
                        h1049
                    }
                });
                let a1052 = 0u64;
                nat_quot(a1050, a1051, a1052)
            };
            let v14: u64 = f43({
                let a1054 = v13;
                let a1055 = f20(match v3.clone() {
                    Adt1::C0(h1053, _, _, _, _, _, _, _, _, _, _) => {
                        h1053
                    }
                });
                nat_mul(a1054, a1055)?
            }, v8, v10, v11, match v1.clone() {
                Adt1::C0(_, _, _, _, _, h1056, _, _, _, _, _) => {
                    h1056
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, h1057, _, _, _, _) => {
                    h1057
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1058, _, _, _) => {
                    h1058
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1059, _, _) => {
                    h1059
                }
            })?;
            let v15: u64 = f43(v13, v4, v10, v11, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h1060, _, _, _, _, _) => {
                    h1060
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h1061, _, _, _, _) => {
                    h1061
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1062, _, _, _) => {
                    h1062
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1063, _, _) => {
                    h1063
                }
            })?;
            if if f2(match v3.clone() {
                Adt1::C0(h1064, _, _, _, _, _, _, _, _, _, _) => {
                    h1064
                }
            }) {
                true
            } else {
                let a1066 = match v3.clone() {
                    Adt1::C0(h1065, _, _, _, _, _, _, _, _, _, _) => {
                        h1065
                    }
                };
                let a1067 = 26u64;
                nat_eq(a1066, a1067)
            } {
                Ok(Adt3::C6(v14, v15))
            } else {
                Ok(f59(match v1 {
                    Adt1::C0(h1068, _, _, _, _, _, _, _, _, _, _) => {
                        h1068
                    }
                }, match v3 {
                    Adt1::C0(h1069, _, _, _, _, _, _, _, _, _, _) => {
                        h1069
                    }
                }, {
                    let a1070 = v6;
                    let a1071 = 1u64;
                    nat_sub(a1070, a1071)
                }, v14, v15))
            }
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f46(v0: u64, v1: u64, v2: u64) -> Adt3 {
    Adt3::C3(2u64, 0u64, 0u64, 0u64, 1u64, v0, v1, v2, 0u64, 2u64, 4u64, 4u64)
}

pub fn f47(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt3 {
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

pub fn f48(v0: u64, v1: u64) -> Adt3 {
    let a1072 = v0;
    let a1073 = 8u64;
    if nat_eq(a1072, a1073) {
        Adt3::C3(1u64, 6u64, 0u64, 0u64, 1u64, 0u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
    } else {
        let a1074 = v0;
        let a1075 = 9u64;
        if nat_eq(a1074, a1075) {
            Adt3::C3(1u64, 2u64, 0u64, 0u64, 2u64, v1, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
        } else {
            let a1076 = v0;
            let a1077 = 10u64;
            if nat_eq(a1076, a1077) {
                Adt3::C3(5u64, 16u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
            } else {
                let a1078 = v0;
                let a1079 = 11u64;
                if nat_eq(a1078, a1079) {
                    Adt3::C3(1u64, 0u64, 0u64, 0u64, 2u64, 1065353216u64, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
                } else {
                    let a1080 = v0;
                    let a1081 = 12u64;
                    if nat_eq(a1080, a1081) {
                        Adt3::C3(1u64, 3u64, 0u64, 0u64, 1u64, 0u64, 1u64, 2u64, 0u64, 0u64, 4u64, 4u64)
                    } else {
                        let a1082 = v0;
                        let a1083 = 13u64;
                        if nat_eq(a1082, a1083) {
                            Adt3::C3(1u64, 0u64, 0u64, 0u64, 1u64, 1u64, 2u64, 1065353216u64, 0u64, 1u64, 4u64, 4u64)
                        } else {
                            let a1084 = v0;
                            let a1085 = 14u64;
                            if nat_eq(a1084, a1085) {
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

pub fn f49(v0: u64, v1: Adt1, v2: Adt1) -> R<u64> {
    let v3: u64 = {
        let a1087 = v0;
        let a1088 = match v1.clone() {
            Adt1::C0(_, h1086, _, _, _, _, _, _, _, _, _) => {
                h1086
            }
        };
        let a1089 = 0u64;
        nat_quot(a1087, a1088, a1089)
    };
    let v4: u64 = {
        let a1091 = v3;
        let a1092 = match v1.clone() {
            Adt1::C0(_, _, h1090, _, _, _, _, _, _, _, _) => {
                h1090
            }
        };
        let a1093 = 0u64;
        nat_quot(a1091, a1092, a1093)
    };
    f43({
        let a1099 = {
            let a1095 = v0;
            let a1096 = match v1.clone() {
                Adt1::C0(_, h1094, _, _, _, _, _, _, _, _, _) => {
                    h1094
                }
            };
            let a1097 = 0u64;
            nat_rem(a1095, a1096, a1097)
        };
        let a1100 = match v2.clone() {
            Adt1::C0(_, h1098, _, _, _, _, _, _, _, _, _) => {
                h1098
            }
        };
        let a1101 = 0u64;
        nat_rem(a1099, a1100, a1101)
    }, {
        let a1107 = {
            let a1103 = v3;
            let a1104 = match v1.clone() {
                Adt1::C0(_, _, h1102, _, _, _, _, _, _, _, _) => {
                    h1102
                }
            };
            let a1105 = 0u64;
            nat_rem(a1103, a1104, a1105)
        };
        let a1108 = match v2.clone() {
            Adt1::C0(_, _, h1106, _, _, _, _, _, _, _, _) => {
                h1106
            }
        };
        let a1109 = 0u64;
        nat_rem(a1107, a1108, a1109)
    }, {
        let a1115 = {
            let a1111 = v4;
            let a1112 = match v1.clone() {
                Adt1::C0(_, _, _, h1110, _, _, _, _, _, _, _) => {
                    h1110
                }
            };
            let a1113 = 0u64;
            nat_rem(a1111, a1112, a1113)
        };
        let a1116 = match v2.clone() {
            Adt1::C0(_, _, _, h1114, _, _, _, _, _, _, _) => {
                h1114
            }
        };
        let a1117 = 0u64;
        nat_rem(a1115, a1116, a1117)
    }, {
        let a1123 = {
            let a1119 = v4;
            let a1120 = match v1 {
                Adt1::C0(_, _, _, h1118, _, _, _, _, _, _, _) => {
                    h1118
                }
            };
            let a1121 = 0u64;
            nat_quot(a1119, a1120, a1121)
        };
        let a1124 = match v2.clone() {
            Adt1::C0(_, _, _, _, h1122, _, _, _, _, _, _) => {
                h1122
            }
        };
        let a1125 = 0u64;
        nat_rem(a1123, a1124, a1125)
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h1126, _, _, _, _, _) => {
            h1126
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h1127, _, _, _, _) => {
            h1127
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1128, _, _, _) => {
            h1128
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1129, _, _) => {
            h1129
        }
    })
}

pub fn f50(v0: u64) -> Adt4 {
    let a1130 = v0;
    let a1131 = 0u64;
    if nat_eq(a1130, a1131) {
        Adt4::C1(Adt5::C0(0u64, 1u64, 4u64))
    } else {
        let a1132 = v0;
        let a1133 = 1u64;
        if nat_eq(a1132, a1133) {
            Adt4::C1(Adt5::C0(1u64, 1u64, 2u64))
        } else {
            let a1134 = v0;
            let a1135 = 2u64;
            if nat_eq(a1134, a1135) {
                Adt4::C1(Adt5::C0(2u64, 32u64, 18u64))
            } else {
                let a1136 = v0;
                let a1137 = 3u64;
                if nat_eq(a1136, a1137) {
                    Adt4::C1(Adt5::C0(3u64, 32u64, 20u64))
                } else {
                    let a1138 = v0;
                    let a1139 = 6u64;
                    if nat_eq(a1138, a1139) {
                        Adt4::C1(Adt5::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1140 = v0;
                        let a1141 = 7u64;
                        if nat_eq(a1140, a1141) {
                            Adt4::C1(Adt5::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1142 = v0;
                            let a1143 = 8u64;
                            if nat_eq(a1142, a1143) {
                                Adt4::C1(Adt5::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1144 = v0;
                                let a1145 = 9u64;
                                if nat_eq(a1144, a1145) {
                                    Adt4::C1(Adt5::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1146 = v0;
                                    let a1147 = 10u64;
                                    if nat_eq(a1146, a1147) {
                                        Adt4::C1(Adt5::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1148 = v0;
                                        let a1149 = 11u64;
                                        if nat_eq(a1148, a1149) {
                                            Adt4::C1(Adt5::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1150 = v0;
                                            let a1151 = 12u64;
                                            if nat_eq(a1150, a1151) {
                                                Adt4::C1(Adt5::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1152 = v0;
                                                let a1153 = 13u64;
                                                if nat_eq(a1152, a1153) {
                                                    Adt4::C1(Adt5::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1154 = v0;
                                                    let a1155 = 14u64;
                                                    if nat_eq(a1154, a1155) {
                                                        Adt4::C1(Adt5::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1156 = v0;
                                                        let a1157 = 15u64;
                                                        if nat_eq(a1156, a1157) {
                                                            Adt4::C1(Adt5::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1158 = v0;
                                                            let a1159 = 16u64;
                                                            if nat_eq(a1158, a1159) {
                                                                Adt4::C1(Adt5::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1160 = v0;
                                                                let a1161 = 17u64;
                                                                if nat_eq(a1160, a1161) {
                                                                    Adt4::C1(Adt5::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1162 = v0;
                                                                    let a1163 = 18u64;
                                                                    if nat_eq(a1162, a1163) {
                                                                        Adt4::C1(Adt5::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1164 = v0;
                                                                        let a1165 = 19u64;
                                                                        if nat_eq(a1164, a1165) {
                                                                            Adt4::C1(Adt5::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1166 = v0;
                                                                            let a1167 = 20u64;
                                                                            if nat_eq(a1166, a1167) {
                                                                                Adt4::C1(Adt5::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1168 = v0;
                                                                                let a1169 = 21u64;
                                                                                if nat_eq(a1168, a1169) {
                                                                                    Adt4::C1(Adt5::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1170 = v0;
                                                                                    let a1171 = 22u64;
                                                                                    if nat_eq(a1170, a1171) {
                                                                                        Adt4::C1(Adt5::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1172 = v0;
                                                                                        let a1173 = 23u64;
                                                                                        if nat_eq(a1172, a1173) {
                                                                                            Adt4::C1(Adt5::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1174 = v0;
                                                                                            let a1175 = 24u64;
                                                                                            if nat_eq(a1174, a1175) {
                                                                                                Adt4::C1(Adt5::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1176 = v0;
                                                                                                let a1177 = 25u64;
                                                                                                if nat_eq(a1176, a1177) {
                                                                                                    Adt4::C1(Adt5::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1178 = v0;
                                                                                                    let a1179 = 26u64;
                                                                                                    if nat_eq(a1178, a1179) {
                                                                                                        Adt4::C1(Adt5::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1180 = v0;
                                                                                                        let a1181 = 27u64;
                                                                                                        if nat_eq(a1180, a1181) {
                                                                                                            Adt4::C1(Adt5::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1182 = v0;
                                                                                                            let a1183 = 28u64;
                                                                                                            if nat_eq(a1182, a1183) {
                                                                                                                Adt4::C1(Adt5::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1184 = v0;
                                                                                                                let a1185 = 29u64;
                                                                                                                if nat_eq(a1184, a1185) {
                                                                                                                    Adt4::C1(Adt5::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1186 = v0;
                                                                                                                    let a1187 = 30u64;
                                                                                                                    if nat_eq(a1186, a1187) {
                                                                                                                        Adt4::C1(Adt5::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1188 = v0;
                                                                                                                        let a1189 = 34u64;
                                                                                                                        if nat_eq(a1188, a1189) {
                                                                                                                            Adt4::C1(Adt5::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1190 = v0;
                                                                                                                            let a1191 = 35u64;
                                                                                                                            if nat_eq(a1190, a1191) {
                                                                                                                                Adt4::C1(Adt5::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1192 = v0;
                                                                                                                                let a1193 = 39u64;
                                                                                                                                if nat_eq(a1192, a1193) {
                                                                                                                                    Adt4::C1(Adt5::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1194 = v0;
                                                                                                                                    let a1195 = 40u64;
                                                                                                                                    if nat_eq(a1194, a1195) {
                                                                                                                                        Adt4::C1(Adt5::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1196 = v0;
                                                                                                                                        let a1197 = 41u64;
                                                                                                                                        if nat_eq(a1196, a1197) {
                                                                                                                                            Adt4::C1(Adt5::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1198 = v0;
                                                                                                                                            let a1199 = 42u64;
                                                                                                                                            if nat_eq(a1198, a1199) {
                                                                                                                                                Adt4::C1(Adt5::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1200 = v0;
                                                                                                                                                let a1201 = 43u64;
                                                                                                                                                if nat_eq(a1200, a1201) {
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

pub fn f51(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f18(v0.clone(), v1.clone()) {
        let a1204 = match v0.clone() {
            Adt1::C0(h1202, _, _, _, _, _, _, _, _, _, _) => {
                h1202
            }
        };
        let a1205 = match v1.clone() {
            Adt1::C0(h1203, _, _, _, _, _, _, _, _, _, _) => {
                h1203
            }
        };
        nat_eq(a1204, a1205)
    } else {
        false
    } {
        let a1208 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1206) => {
                h1206
            }
        };
        let a1209 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1207) => {
                h1207
            }
        };
        nat_eq(a1208, a1209)
    } else {
        false
    } {
        let a1212 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1210, _, _, _, _, _) => {
                h1210
            }
        };
        let a1213 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1211, _, _, _, _, _) => {
                h1211
            }
        };
        nat_eq(a1212, a1213)
    } else {
        false
    } {
        let a1216 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1214, _, _, _, _) => {
                h1214
            }
        };
        let a1217 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1215, _, _, _, _) => {
                h1215
            }
        };
        nat_eq(a1216, a1217)
    } else {
        false
    } {
        let a1220 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1218, _, _, _) => {
                h1218
            }
        };
        let a1221 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1219, _, _, _) => {
                h1219
            }
        };
        nat_eq(a1220, a1221)
    } else {
        false
    } {
        let a1224 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1222, _, _) => {
                h1222
            }
        };
        let a1225 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1223, _, _) => {
                h1223
            }
        };
        nat_eq(a1224, a1225)
    } else {
        false
    }
}

pub fn f52(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a1226 = v1;
    let a1227 = 1u64;
    if if if nat_le(a1226, a1227) {
        true
    } else {
        let a1228 = v0;
        let a1229 = v2;
        nat_le(a1228, a1229)
    } {
        let a1230 = v3;
        let a1231 = 1u64;
        if nat_le(a1230, a1231) {
            true
        } else {
            let a1238 = {
                let a1236 = v0;
                let a1237 = {
                    let a1234 = {
                        let a1232 = v1;
                        let a1233 = 1u64;
                        nat_sub(a1232, a1233)
                    };
                    let a1235 = v2;
                    nat_mul(a1234, a1235)?
                };
                nat_add(a1236, a1237)?
            };
            let a1239 = v4;
            nat_le(a1238, a1239)
        }
    } else {
        false
    } {
        let a1240 = v5;
        let a1241 = 1u64;
        if nat_le(a1240, a1241) {
            Ok(true)
        } else {
            let a1254 = {
                let a1252 = {
                    let a1246 = v0;
                    let a1247 = {
                        let a1244 = {
                            let a1242 = v1;
                            let a1243 = 1u64;
                            nat_sub(a1242, a1243)
                        };
                        let a1245 = v2;
                        nat_mul(a1244, a1245)?
                    };
                    nat_add(a1246, a1247)?
                };
                let a1253 = {
                    let a1250 = {
                        let a1248 = v3;
                        let a1249 = 1u64;
                        nat_sub(a1248, a1249)
                    };
                    let a1251 = v4;
                    nat_mul(a1250, a1251)?
                };
                nat_add(a1252, a1253)?
            };
            let a1255 = v6;
            Ok(nat_le(a1254, a1255))
        }
    } else {
        Ok(false)
    }
}

pub fn f53(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1256 = v0;
    let a1257 = 0u64;
    if if if nat_eq(a1256, a1257) {
        true
    } else {
        let a1258 = v1;
        let a1259 = 0u64;
        nat_eq(a1258, a1259)
    } {
        true
    } else {
        let a1260 = v2;
        let a1261 = 0u64;
        nat_eq(a1260, a1261)
    } {
        true
    } else {
        let a1262 = v3;
        let a1263 = 0u64;
        nat_eq(a1262, a1263)
    }
}

pub fn f54(v0: Adt1, v1: Adt1) -> bool {
    if f23(v1.clone()) {
        f23(v0)
    } else {
        let a1269 = {
            let a1266 = match v0.clone() {
                Adt1::C0(_, h1264, _, _, _, _, _, _, _, _, _) => {
                    h1264
                }
            };
            let a1267 = match v1.clone() {
                Adt1::C0(_, h1265, _, _, _, _, _, _, _, _, _) => {
                    h1265
                }
            };
            let a1268 = 1u64;
            nat_rem(a1266, a1267, a1268)
        };
        let a1270 = 0u64;
        if if if nat_eq(a1269, a1270) {
            let a1276 = {
                let a1273 = match v0.clone() {
                    Adt1::C0(_, _, h1271, _, _, _, _, _, _, _, _) => {
                        h1271
                    }
                };
                let a1274 = match v1.clone() {
                    Adt1::C0(_, _, h1272, _, _, _, _, _, _, _, _) => {
                        h1272
                    }
                };
                let a1275 = 1u64;
                nat_rem(a1273, a1274, a1275)
            };
            let a1277 = 0u64;
            nat_eq(a1276, a1277)
        } else {
            false
        } {
            let a1283 = {
                let a1280 = match v0.clone() {
                    Adt1::C0(_, _, _, h1278, _, _, _, _, _, _, _) => {
                        h1278
                    }
                };
                let a1281 = match v1.clone() {
                    Adt1::C0(_, _, _, h1279, _, _, _, _, _, _, _) => {
                        h1279
                    }
                };
                let a1282 = 1u64;
                nat_rem(a1280, a1281, a1282)
            };
            let a1284 = 0u64;
            nat_eq(a1283, a1284)
        } else {
            false
        } {
            let a1290 = {
                let a1287 = match v0 {
                    Adt1::C0(_, _, _, _, h1285, _, _, _, _, _, _) => {
                        h1285
                    }
                };
                let a1288 = match v1 {
                    Adt1::C0(_, _, _, _, h1286, _, _, _, _, _, _) => {
                        h1286
                    }
                };
                let a1289 = 1u64;
                nat_rem(a1287, a1288, a1289)
            };
            let a1291 = 0u64;
            nat_eq(a1290, a1291)
        } else {
            false
        }
    }
}

pub fn f55(v0: u64, v1: u64, v2: u64) -> bool {
    let a1292 = v0;
    let a1293 = 0u64;
    if if if nat_eq(a1292, a1293) {
        let a1294 = v1;
        let a1295 = 0u64;
        nat_eq(a1294, a1295)
    } else {
        false
    } {
        let a1296 = v2;
        let a1297 = 0u64;
        nat_eq(a1296, a1297)
    } else {
        false
    } {
        true
    } else {
        let a1298 = v0;
        let a1299 = 1u64;
        if if nat_eq(a1298, a1299) {
            true
        } else {
            let a1300 = v0;
            let a1301 = 30u64;
            nat_eq(a1300, a1301)
        } {
            let a1302 = v1;
            let a1303 = v0;
            if if nat_eq(a1302, a1303) {
                let a1304 = v2;
                let a1305 = v0;
                nat_eq(a1304, a1305)
            } else {
                false
            } {
                true
            } else {
                let a1306 = v1;
                let a1307 = 0u64;
                if nat_eq(a1306, a1307) {
                    let a1308 = v2;
                    let a1309 = v0;
                    if nat_eq(a1308, a1309) {
                        true
                    } else {
                        let a1310 = v2;
                        let a1311 = 0u64;
                        nat_eq(a1310, a1311)
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
    let a1312 = v0;
    let a1313 = 1u64;
    if nat_le(a1312, a1313) {
        true
    } else {
        let a1317 = {
            let a1314 = v1;
            let a1315 = v2;
            let a1316 = 1u64;
            nat_rem(a1314, a1315, a1316)
        };
        let a1318 = 0u64;
        nat_eq(a1317, a1318)
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
    let a1319 = v0;
    let a1320 = 1u64;
    if if nat_eq(a1319, a1320) {
        let a1321 = v1;
        let a1322 = 43u64;
        nat_eq(a1321, a1322)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f59(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt3 {
    let a1323 = f58(v0, v1);
    let a1324 = 2u64;
    if nat_eq(a1323, a1324) {
        let a1328 = {
            let a1325 = v2;
            let a1326 = 2u64;
            let a1327 = 0u64;
            nat_rem(a1325, a1326, a1327)
        };
        let a1329 = 0u64;
        if nat_eq(a1328, a1329) {
            Adt3::C3(0u64, 0u64, v0, 0u64, 0u64, v3, 2u64, 0u64, 1u64, 0u64, f37(v0), 8u64)
        } else {
            Adt3::C3(0u64, 0u64, 0u64, v1, 1u64, 0u64, 2u64, 0u64, 3u64, v4, 8u64, f19(v1))
        }
    } else {
        Adt3::C3(0u64, 0u64, v0, v1, 0u64, v3, 2u64, 0u64, 3u64, v4, f37(v0), f19(v1))
    }
}
