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
                                    if if f2(match v6.clone() {
                                        Adt1::C0(h5, _, _, _, _, _, _, _, _, _, _) => {
                                            h5
                                        }
                                    }) {
                                        true
                                    } else {
                                        f2(match v4.clone() {
                                            Adt1::C0(h6, _, _, _, _, _, _, _, _, _, _) => {
                                                h6
                                            }
                                        })
                                    } {
                                        f3(v5, v6, v7, v4)
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
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h20) => {
                    h20
                }
            } {
                f10(v26, v27)
            } else {
                let a22 = match v27.clone() {
                    Adt2::C0(_, _, _, h21, _, _, _, _, _, _, _) => {
                        h21
                    }
                };
                let a23 = 33u64;
                if nat_eq(a22, a23) {
                    f11(v26, v27)
                } else {
                    let a25 = match v27.clone() {
                        Adt2::C0(_, _, _, h24, _, _, _, _, _, _, _) => {
                            h24
                        }
                    };
                    let a26 = 66u64;
                    if if nat_eq(a25, a26) {
                        true
                    } else {
                        let a28 = match v27.clone() {
                            Adt2::C0(_, _, _, h27, _, _, _, _, _, _, _) => {
                                h27
                            }
                        };
                        let a29 = 67u64;
                        nat_eq(a28, a29)
                    } {
                        f12(v26, v27)
                    } else {
                        let a31 = match v27.clone() {
                            Adt2::C0(_, _, _, h30, _, _, _, _, _, _, _) => {
                                h30
                            }
                        };
                        let a32 = 41u64;
                        if nat_eq(a31, a32) {
                            f13(v26, v27, v28)
                        } else {
                            let a34 = 98u64;
                            let a35 = match v27.clone() {
                                Adt2::C0(_, _, _, h33, _, _, _, _, _, _, _) => {
                                    h33
                                }
                            };
                            if nat_le(a34, a35) {
                                if f14(match v27.clone() {
                                    Adt2::C0(_, _, _, _, h36, _, _, _, _, _, _) => {
                                        h36
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
            let a7 = v8;
            let a8 = 41u64;
            if nat_eq(a7, a8) {
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
            let a9 = v14;
            let a10 = 33u64;
            if nat_eq(a9, a10) {
                if v19 {
                    Ok(Adt3::C1)
                } else {
                    let m11 = v21;
                    match m11 {
                        None => {
                            Ok(Adt3::C1)
                        }
                        Some(v23) => {
                            f6(v20, v23, v22)
                        }
                    }
                }
            } else {
                let a12 = v14;
                let a13 = 66u64;
                if if nat_eq(a12, a13) {
                    true
                } else {
                    let a14 = v14;
                    let a15 = 67u64;
                    nat_eq(a14, a15)
                } {
                    if v19 {
                        Ok(Adt3::C1)
                    } else {
                        let m16 = v21;
                        match m16 {
                            None => {
                                f7(v14, v15, v20, v22)
                            }
                            Some(_) => {
                                Ok(Adt3::C1)
                            }
                        }
                    }
                } else {
                    let a17 = v14;
                    let a18 = 98u64;
                    if nat_eq(a17, a18) {
                        if if f8(v15) {
                            !v19
                        } else {
                            false
                        } {
                            let m19 = v21;
                            match m19 {
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
    let a37 = v0;
    let a38 = 2u64;
    if nat_eq(a37, a38) {
        Some(0u64)
    } else {
        let a39 = v0;
        let a40 = 5u64;
        if nat_eq(a39, a40) {
            Some(1u64)
        } else {
            let a41 = v0;
            let a42 = 6u64;
            if nat_eq(a41, a42) {
                Some(2u64)
            } else {
                let a43 = v0;
                let a44 = 7u64;
                if nat_eq(a43, a44) {
                    Some(3u64)
                } else {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f2(v0: u64) -> bool {
    let a45 = 6u64;
    let a46 = v0;
    if if if if nat_le(a45, a46) {
        let a47 = v0;
        let a48 = 23u64;
        nat_le(a47, a48)
    } else {
        false
    } {
        true
    } else {
        let a49 = v0;
        let a50 = 2u64;
        if nat_eq(a49, a50) {
            true
        } else {
            let a51 = v0;
            let a52 = 3u64;
            nat_eq(a51, a52)
        }
    } {
        true
    } else {
        let a53 = v0;
        let a54 = 29u64;
        nat_eq(a53, a54)
    } {
        true
    } else {
        let a55 = 34u64;
        let a56 = v0;
        if if nat_le(a55, a56) {
            let a57 = v0;
            let a58 = 35u64;
            nat_le(a57, a58)
        } else {
            false
        } {
            true
        } else {
            let a59 = 39u64;
            let a60 = v0;
            if nat_le(a59, a60) {
                let a61 = v0;
                let a62 = 42u64;
                nat_le(a61, a62)
            } else {
                false
            }
        }
    }
}

pub fn f3(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    let a65 = match v1.clone() {
        Adt1::C0(_, h63, _, _, _, _, _, _, _, _, _) => {
            h63
        }
    };
    let a66 = match v2.clone() {
        Adt1::C0(_, h64, _, _, _, _, _, _, _, _, _) => {
            h64
        }
    };
    if if if if if if if if if if if if if if nat_eq(a65, a66) {
        true
    } else {
        let a69 = match v1.clone() {
            Adt1::C0(h67, _, _, _, _, _, _, _, _, _, _) => {
                h67
            }
        };
        let a70 = match v3.clone() {
            Adt1::C0(h68, _, _, _, _, _, _, _, _, _, _) => {
                h68
            }
        };
        nat_eq(a69, a70)
    } {
        f18(v1.clone(), v2.clone())
    } else {
        false
    } {
        f19(v1.clone(), v3.clone())
    } else {
        false
    } {
        f20(v0, match v1.clone() {
            Adt1::C0(h71, _, _, _, _, _, _, _, _, _, _) => {
                h71
            }
        }, v2.clone(), match v3.clone() {
            Adt1::C0(h72, _, _, _, _, _, _, _, _, _, _) => {
                h72
            }
        })
    } else {
        false
    } {
        f21(match v1.clone() {
            Adt1::C0(h73, _, _, _, _, _, _, _, _, _, _) => {
                h73
            }
        }, match v3.clone() {
            Adt1::C0(h74, _, _, _, _, _, _, _, _, _, _) => {
                h74
            }
        })
    } else {
        false
    } {
        let a76 = 0u64;
        let a77 = f22(match v3.clone() {
            Adt1::C0(h75, _, _, _, _, _, _, _, _, _, _) => {
                h75
            }
        });
        nat_lt(a76, a77)
    } else {
        false
    } {
        let a80 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h78, _, _, _, _, _) => {
                h78
            }
        };
        let a81 = f22(match v1.clone() {
            Adt1::C0(h79, _, _, _, _, _, _, _, _, _, _) => {
                h79
            }
        });
        nat_eq(a80, a81)
    } else {
        false
    } {
        let a84 = match v3.clone() {
            Adt1::C0(_, _, _, _, _, h82, _, _, _, _, _) => {
                h82
            }
        };
        let a85 = f22(match v3.clone() {
            Adt1::C0(h83, _, _, _, _, _, _, _, _, _, _) => {
                h83
            }
        });
        nat_eq(a84, a85)
    } else {
        false
    } {
        let a91 = {
            let a88 = match v1.clone() {
                Adt1::C0(_, h86, _, _, _, _, _, _, _, _, _) => {
                    h86
                }
            };
            let a89 = f23(match v1.clone() {
                Adt1::C0(h87, _, _, _, _, _, _, _, _, _, _) => {
                    h87
                }
            });
            let a90 = 1u64;
            nat_rem(a88, a89, a90)
        };
        let a92 = 0u64;
        nat_eq(a91, a92)
    } else {
        false
    } {
        let a98 = {
            let a95 = match v3.clone() {
                Adt1::C0(_, h93, _, _, _, _, _, _, _, _, _) => {
                    h93
                }
            };
            let a96 = f23(match v3.clone() {
                Adt1::C0(h94, _, _, _, _, _, _, _, _, _, _) => {
                    h94
                }
            });
            let a97 = 1u64;
            nat_rem(a95, a96, a97)
        };
        let a99 = 0u64;
        nat_eq(a98, a99)
    } else {
        false
    } {
        f24(v1.clone(), v3.clone())?
    } else {
        false
    } {
        f24(v2.clone(), v3.clone())?
    } else {
        false
    } {
        f25({
            let a106 = {
                let a102 = match v3.clone() {
                    Adt1::C0(_, h100, _, _, _, _, _, _, _, _, _) => {
                        h100
                    }
                };
                let a103 = f23(match v3.clone() {
                    Adt1::C0(h101, _, _, _, _, _, _, _, _, _, _) => {
                        h101
                    }
                });
                let a104 = 0u64;
                nat_quot(a102, a103, a104)
            };
            let a107 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h105, _, _, _, _, _) => {
                    h105
                }
            };
            nat_mul(a106, a107)?
        }, match v3.clone() {
            Adt1::C0(_, _, h108, _, _, _, _, _, _, _, _) => {
                h108
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, h109, _, _, _, _, _, _, _) => {
                h109
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, h110, _, _, _, _, _, _) => {
                h110
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, _, _, h111, _, _, _, _) => {
                h111
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h112, _, _, _) => {
                h112
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h113, _, _) => {
                h113
            }
        })?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0(if f26(v1.clone()) {
            0u64
        } else {
            let a119 = match v1.clone() {
                Adt1::C0(_, _, h114, _, _, _, _, _, _, _, _) => {
                    h114
                }
            };
            let a120 = {
                let a117 = match v1.clone() {
                    Adt1::C0(_, _, _, h115, _, _, _, _, _, _, _) => {
                        h115
                    }
                };
                let a118 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h116, _, _, _, _, _, _) => {
                        h116
                    }
                };
                nat_mul(a117, a118)?
            };
            nat_mul(a119, a120)?
        }, 3u64, f27(match v3.clone() {
            Adt1::C0(h121, _, _, _, _, _, _, _, _, _, _) => {
                h121
            }
        }), v0, 0u64, 0u64, 0u64, v1, v2, v3, true)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f4(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if f19(v1.clone(), v3.clone()) {
        f28(v0, v1.clone(), v2.clone(), v3.clone())?
    } else {
        false
    } {
        if f26(v1.clone()) {
            true
        } else {
            let a124 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, h122, _, _, _, _, _) => {
                    h122
                }
            };
            let a125 = f29(match v1.clone() {
                Adt1::C0(h123, _, _, _, _, _, _, _, _, _, _) => {
                    h123
                }
            });
            if if if if if if if nat_eq(a124, a125) {
                let a128 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h126, _, _, _, _, _) => {
                        h126
                    }
                };
                let a129 = f29(match v3.clone() {
                    Adt1::C0(h127, _, _, _, _, _, _, _, _, _, _) => {
                        h127
                    }
                });
                nat_eq(a128, a129)
            } else {
                false
            } {
                f30(v1.clone())
            } else {
                false
            } {
                f30(v2.clone())
            } else {
                false
            } {
                f30(v3.clone())
            } else {
                false
            } {
                f31(v1.clone(), v2.clone())?
            } else {
                false
            } {
                f24(v1.clone(), v3.clone())?
            } else {
                false
            } {
                f24(v2.clone(), v3.clone())?
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v4: u64 = if if f26(v1.clone()) {
            true
        } else {
            f32(v3.clone())?
        } {
            0u64
        } else {
            let a132 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h130, _) => {
                    h130
                }
            };
            let a133 = f29(match v3.clone() {
                Adt1::C0(h131, _, _, _, _, _, _, _, _, _, _) => {
                    h131
                }
            });
            let a134 = 0u64;
            nat_quot(a132, a133, a134)
        };
        Ok(Adt3::C2(Adt2::C0(f33(f34(v1.clone())?, v4, f35(match v1.clone() {
            Adt1::C0(h135, _, _, _, _, _, _, _, _, _, _) => {
                h135
            }
        }, match v2.clone() {
            Adt1::C0(h136, _, _, _, _, _, _, _, _, _, _) => {
                h136
            }
        }, match v3.clone() {
            Adt1::C0(h137, _, _, _, _, _, _, _, _, _, _) => {
                h137
            }
        })?)?, {
            let a138 = 3u64;
            let a139 = v4;
            nat_add(a138, a139)?
        }, None::<u64>, v0, 0u64, 0u64, 0u64, v1, v2, v3, false)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f5(v0: Adt1, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if if if if if if if if if if if if if f36(v2, v3.clone()) {
        let a141 = match v0.clone() {
            Adt1::C0(h140, _, _, _, _, _, _, _, _, _, _) => {
                h140
            }
        };
        let a142 = 0u64;
        if nat_eq(a141, a142) {
            true
        } else {
            let a144 = match v0.clone() {
                Adt1::C0(h143, _, _, _, _, _, _, _, _, _, _) => {
                    h143
                }
            };
            let a145 = 1u64;
            nat_eq(a144, a145)
        }
    } else {
        false
    } {
        let a147 = 0u64;
        let a148 = f22(match v3.clone() {
            Adt1::C0(h146, _, _, _, _, _, _, _, _, _, _) => {
                h146
            }
        });
        nat_lt(a147, a148)
    } else {
        false
    } {
        let a150 = 0u64;
        let a151 = f37(match v1.clone() {
            Adt1::C0(h149, _, _, _, _, _, _, _, _, _, _) => {
                h149
            }
        });
        nat_lt(a150, a151)
    } else {
        false
    } {
        let a154 = match v0.clone() {
            Adt1::C0(_, h152, _, _, _, _, _, _, _, _, _) => {
                h152
            }
        };
        let a155 = match v3.clone() {
            Adt1::C0(_, h153, _, _, _, _, _, _, _, _, _) => {
                h153
            }
        };
        if nat_eq(a154, a155) {
            let a161 = {
                let a158 = match v0.clone() {
                    Adt1::C0(_, h156, _, _, _, _, _, _, _, _, _) => {
                        h156
                    }
                };
                let a159 = f23(match v3.clone() {
                    Adt1::C0(h157, _, _, _, _, _, _, _, _, _, _) => {
                        h157
                    }
                });
                let a160 = 1u64;
                nat_rem(a158, a159, a160)
            };
            let a162 = 0u64;
            nat_eq(a161, a162)
        } else {
            false
        }
    } else {
        false
    } {
        let a165 = match v0.clone() {
            Adt1::C0(_, _, _, h163, _, _, _, _, _, _, _) => {
                h163
            }
        };
        let a166 = match v3.clone() {
            Adt1::C0(_, _, _, h164, _, _, _, _, _, _, _) => {
                h164
            }
        };
        nat_eq(a165, a166)
    } else {
        false
    } {
        let a169 = match v0.clone() {
            Adt1::C0(_, _, _, _, h167, _, _, _, _, _, _) => {
                h167
            }
        };
        let a170 = match v3.clone() {
            Adt1::C0(_, _, _, _, h168, _, _, _, _, _, _) => {
                h168
            }
        };
        nat_eq(a169, a170)
    } else {
        false
    } {
        let a173 = match v0.clone() {
            Adt1::C0(_, _, h171, _, _, _, _, _, _, _, _) => {
                h171
            }
        };
        let a174 = match v1.clone() {
            Adt1::C0(_, h172, _, _, _, _, _, _, _, _, _) => {
                h172
            }
        };
        nat_eq(a173, a174)
    } else {
        false
    } {
        let a176 = match v1.clone() {
            Adt1::C0(_, _, _, _, h175, _, _, _, _, _, _) => {
                h175
            }
        };
        let a177 = 1u64;
        nat_eq(a176, a177)
    } else {
        false
    } {
        let a179 = 0u64;
        let a180 = match v1.clone() {
            Adt1::C0(_, _, h178, _, _, _, _, _, _, _, _) => {
                h178
            }
        };
        nat_lt(a179, a180)
    } else {
        false
    } {
        let a182 = 0u64;
        let a183 = match v1.clone() {
            Adt1::C0(_, _, _, h181, _, _, _, _, _, _, _) => {
                h181
            }
        };
        nat_lt(a182, a183)
    } else {
        false
    } {
        let a189 = {
            let a186 = match v0.clone() {
                Adt1::C0(_, _, _, h184, _, _, _, _, _, _, _) => {
                    h184
                }
            };
            let a187 = match v1.clone() {
                Adt1::C0(_, _, h185, _, _, _, _, _, _, _, _) => {
                    h185
                }
            };
            let a188 = 0u64;
            nat_rem(a186, a187, a188)
        };
        let a190 = 0u64;
        nat_eq(a189, a190)
    } else {
        false
    } {
        let a196 = {
            let a193 = match v0.clone() {
                Adt1::C0(_, _, _, _, h191, _, _, _, _, _, _) => {
                    h191
                }
            };
            let a194 = match v1.clone() {
                Adt1::C0(_, _, _, h192, _, _, _, _, _, _, _) => {
                    h192
                }
            };
            let a195 = 0u64;
            nat_rem(a193, a194, a195)
        };
        let a197 = 0u64;
        nat_eq(a196, a197)
    } else {
        false
    } {
        f38(v0.clone())
    } else {
        false
    } {
        let a199 = match v3.clone() {
            Adt1::C0(_, h198, _, _, _, _, _, _, _, _, _) => {
                h198
            }
        };
        let a200 = 1u64;
        if nat_eq(a199, a200) {
            true
        } else {
            let a203 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h201, _, _, _, _, _) => {
                    h201
                }
            };
            let a204 = f22(match v3.clone() {
                Adt1::C0(h202, _, _, _, _, _, _, _, _, _, _) => {
                    h202
                }
            });
            nat_eq(a203, a204)
        }
    } else {
        false
    } {
        let v4: u64 = f34(v1.clone())?;
        let v5: u64 = {
            let a210 = match v0.clone() {
                Adt1::C0(_, _, h205, _, _, _, _, _, _, _, _) => {
                    h205
                }
            };
            let a211 = {
                let a208 = match v0.clone() {
                    Adt1::C0(_, _, _, h206, _, _, _, _, _, _, _) => {
                        h206
                    }
                };
                let a209 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h207, _, _, _, _, _, _) => {
                        h207
                    }
                };
                nat_mul(a208, a209)?
            };
            nat_mul(a210, a211)?
        };
        let v6: u64 = if f26(v0.clone()) {
            0u64
        } else {
            let a219 = {
                let a212 = 2u64;
                let a213 = v4;
                nat_mul(a212, a213)?
            };
            let a220 = {
                let a217 = v5;
                let a218 = {
                    let a215 = 1u64;
                    let a216 = f39(v0.clone(), match v3.clone() {
                        Adt1::C0(h214, _, _, _, _, _, _, _, _, _, _) => {
                            h214
                        }
                    })?;
                    nat_add(a215, a216)?
                };
                nat_mul(a217, a218)?
            };
            nat_add(a219, a220)?
        };
        Ok(Adt3::C2(Adt2::C0(v6, if f26(v0.clone()) {
            3u64
        } else {
            let a221 = 3u64;
            let a222 = v4;
            nat_add(a221, a222)?
        }, f27(match v3.clone() {
            Adt1::C0(h223, _, _, _, _, _, _, _, _, _, _) => {
                h223
            }
        }), 41u64, 0u64, 0u64, 0u64, v0, v1, v3, false)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f6(v0: Adt1, v1: Adt1, v2: Adt1) -> R<Adt3> {
    let v3: u64 = f23(match v2.clone() {
        Adt1::C0(h224, _, _, _, _, _, _, _, _, _, _) => {
            h224
        }
    });
    if if if if if if if f2(match v2.clone() {
        Adt1::C0(h225, _, _, _, _, _, _, _, _, _, _) => {
            h225
        }
    }) {
        f36(v1.clone(), v2.clone())
    } else {
        false
    } {
        let a227 = 0u64;
        let a228 = f40(match v0.clone() {
            Adt1::C0(h226, _, _, _, _, _, _, _, _, _, _) => {
                h226
            }
        });
        nat_lt(a227, a228)
    } else {
        false
    } {
        let a231 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h229, _, _, _, _, _) => {
                h229
            }
        };
        let a232 = f40(match v0.clone() {
            Adt1::C0(h230, _, _, _, _, _, _, _, _, _, _) => {
                h230
            }
        });
        nat_eq(a231, a232)
    } else {
        false
    } {
        let a237 = {
            let a234 = match v0.clone() {
                Adt1::C0(_, h233, _, _, _, _, _, _, _, _, _) => {
                    h233
                }
            };
            let a235 = v3;
            let a236 = 1u64;
            nat_rem(a234, a235, a236)
        };
        let a238 = 0u64;
        if nat_eq(a237, a238) {
            let a243 = {
                let a240 = match v2.clone() {
                    Adt1::C0(_, h239, _, _, _, _, _, _, _, _, _) => {
                        h239
                    }
                };
                let a241 = v3;
                let a242 = 1u64;
                nat_rem(a240, a241, a242)
            };
            let a244 = 0u64;
            nat_eq(a243, a244)
        } else {
            false
        }
    } else {
        false
    } {
        let a247 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h245, _, _, _, _, _) => {
                h245
            }
        };
        let a248 = f22(match v2.clone() {
            Adt1::C0(h246, _, _, _, _, _, _, _, _, _, _) => {
                h246
            }
        });
        nat_eq(a247, a248)
    } else {
        false
    } {
        let a249 = f34(v0.clone())?;
        let a250 = f34(v2.clone())?;
        nat_eq(a249, a250)
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a251 = f34(v0.clone())?;
            let a252 = v3;
            let a253 = 0u64;
            nat_quot(a251, a252, a253)
        }, 3u64, f27(match v2.clone() {
            Adt1::C0(h254, _, _, _, _, _, _, _, _, _, _) => {
                h254
            }
        }), 33u64, 0u64, 0u64, 0u64, v0, v1, v2, false)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f7(v0: u64, v1: u64, v2: Adt1, v3: Adt1) -> R<Adt3> {
    let a256 = match v2.clone() {
        Adt1::C0(h255, _, _, _, _, _, _, _, _, _, _) => {
            h255
        }
    };
    let a257 = 0u64;
    if if if if if if if if if if if nat_eq(a256, a257) {
        let a259 = match v3.clone() {
            Adt1::C0(h258, _, _, _, _, _, _, _, _, _, _) => {
                h258
            }
        };
        let a260 = 26u64;
        nat_eq(a259, a260)
    } else {
        false
    } {
        let a262 = match v2.clone() {
            Adt1::C0(_, h261, _, _, _, _, _, _, _, _, _) => {
                h261
            }
        };
        let a263 = 2147483647u64;
        nat_le(a262, a263)
    } else {
        false
    } {
        let a266 = match v3.clone() {
            Adt1::C0(_, h264, _, _, _, _, _, _, _, _, _) => {
                h264
            }
        };
        let a267 = match v2.clone() {
            Adt1::C0(_, h265, _, _, _, _, _, _, _, _, _) => {
                h265
            }
        };
        nat_le(a266, a267)
    } else {
        false
    } {
        let a270 = match v2.clone() {
            Adt1::C0(_, _, h268, _, _, _, _, _, _, _, _) => {
                h268
            }
        };
        let a271 = match v3.clone() {
            Adt1::C0(_, _, h269, _, _, _, _, _, _, _, _) => {
                h269
            }
        };
        nat_eq(a270, a271)
    } else {
        false
    } {
        let a274 = match v2.clone() {
            Adt1::C0(_, _, _, h272, _, _, _, _, _, _, _) => {
                h272
            }
        };
        let a275 = match v3.clone() {
            Adt1::C0(_, _, _, h273, _, _, _, _, _, _, _) => {
                h273
            }
        };
        nat_eq(a274, a275)
    } else {
        false
    } {
        let a278 = match v2.clone() {
            Adt1::C0(_, _, _, _, h276, _, _, _, _, _, _) => {
                h276
            }
        };
        let a279 = match v3.clone() {
            Adt1::C0(_, _, _, _, h277, _, _, _, _, _, _) => {
                h277
            }
        };
        nat_eq(a278, a279)
    } else {
        false
    } {
        let a280 = v0;
        let a281 = 67u64;
        if nat_eq(a280, a281) {
            true
        } else {
            let a282 = v1;
            let a283 = 1u64;
            if nat_le(a282, a283) {
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
                nat_eq(a286, a287)
            } else {
                false
            }
        }
    } else {
        false
    } {
        f41(v2.clone())?
    } else {
        false
    } {
        f41(v3.clone())?
    } else {
        false
    } {
        f42(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a293 = match v3.clone() {
                Adt1::C0(_, _, h288, _, _, _, _, _, _, _, _) => {
                    h288
                }
            };
            let a294 = {
                let a291 = match v3.clone() {
                    Adt1::C0(_, _, _, h289, _, _, _, _, _, _, _) => {
                        h289
                    }
                };
                let a292 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h290, _, _, _, _, _, _) => {
                        h290
                    }
                };
                nat_mul(a291, a292)?
            };
            nat_mul(a293, a294)?
        }, 3u64, None::<u64>, v0, {
            let a295 = v0;
            let a296 = 67u64;
            if nat_eq(a295, a296) {
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
    let a297 = v0;
    let a298 = 6u64;
    nat_le(a297, a298)
}

pub fn f9(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1) -> R<Adt3> {
    let a299 = v2;
    let a300 = 1u64;
    if if if if if if if if if if if if if if if nat_le(a299, a300) {
        let a302 = 0u64;
        let a303 = f40(match v5.clone() {
            Adt1::C0(h301, _, _, _, _, _, _, _, _, _, _) => {
                h301
            }
        });
        nat_lt(a302, a303)
    } else {
        false
    } {
        let a306 = match v5.clone() {
            Adt1::C0(h304, _, _, _, _, _, _, _, _, _, _) => {
                h304
            }
        };
        let a307 = match v6.clone() {
            Adt1::C0(h305, _, _, _, _, _, _, _, _, _, _) => {
                h305
            }
        };
        nat_eq(a306, a307)
    } else {
        false
    } {
        let a310 = match v5.clone() {
            Adt1::C0(h308, _, _, _, _, _, _, _, _, _, _) => {
                h308
            }
        };
        let a311 = match v7.clone() {
            Adt1::C0(h309, _, _, _, _, _, _, _, _, _, _) => {
                h309
            }
        };
        nat_eq(a310, a311)
    } else {
        false
    } {
        f19(v5.clone(), v6.clone())
    } else {
        false
    } {
        f43(v5.clone())?
    } else {
        false
    } {
        f43(v6.clone())?
    } else {
        false
    } {
        f43(v7.clone())?
    } else {
        false
    } {
        let a314 = match v5.clone() {
            Adt1::C0(_, _, h312, _, _, _, _, _, _, _, _) => {
                h312
            }
        };
        let a315 = match v7.clone() {
            Adt1::C0(_, _, h313, _, _, _, _, _, _, _, _) => {
                h313
            }
        };
        nat_eq(a314, a315)
    } else {
        false
    } {
        let a318 = match v5.clone() {
            Adt1::C0(_, _, _, h316, _, _, _, _, _, _, _) => {
                h316
            }
        };
        let a319 = match v7.clone() {
            Adt1::C0(_, _, _, h317, _, _, _, _, _, _, _) => {
                h317
            }
        };
        nat_eq(a318, a319)
    } else {
        false
    } {
        let a322 = match v5.clone() {
            Adt1::C0(_, _, _, _, h320, _, _, _, _, _, _) => {
                h320
            }
        };
        let a323 = match v7.clone() {
            Adt1::C0(_, _, _, _, h321, _, _, _, _, _, _) => {
                h321
            }
        };
        nat_eq(a322, a323)
    } else {
        false
    } {
        let a330 = match v7.clone() {
            Adt1::C0(_, h324, _, _, _, _, _, _, _, _, _) => {
                h324
            }
        };
        let a331 = if v1 {
            match v5.clone() {
                Adt1::C0(_, h325, _, _, _, _, _, _, _, _, _) => {
                    h325
                }
            }
        } else {
            let a327 = match v5.clone() {
                Adt1::C0(_, h326, _, _, _, _, _, _, _, _, _) => {
                    h326
                }
            };
            let a328 = 2u64;
            let a329 = 0u64;
            nat_quot(a327, a328, a329)
        };
        nat_eq(a330, a331)
    } else {
        false
    } {
        let a333 = match v7.clone() {
            Adt1::C0(_, h332, _, _, _, _, _, _, _, _, _) => {
                h332
            }
        };
        let a334 = 2147483647u64;
        nat_le(a333, a334)
    } else {
        false
    } {
        f24(v5.clone(), v7.clone())?
    } else {
        false
    } {
        f24(v6.clone(), v7.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0(if f14(v0) {
            let a337 = 1u64;
            let a338 = {
                let a335 = f44(v0);
                let a336 = f34(v7.clone())?;
                nat_mul(a335, a336)?
            };
            nat_add(a337, a338)?
        } else {
            let a339 = 4u64;
            let a340 = f34(v7.clone())?;
            nat_mul(a339, a340)?
        }, if f14(v0) {
            4u64
        } else {
            3u64
        }, None::<u64>, if v1 {
            98u64
        } else {
            let a341 = 99u64;
            let a342 = v2;
            nat_add(a341, a342)?
        }, v0, v3, v4, v5, v6, v7, false)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f10(v0: u64, v1: Adt2) -> R<Adt3> {
    let a344 = v0;
    let a345 = match v1.clone() {
        Adt2::C0(h343, _, _, _, _, _, _, _, _, _, _) => {
            h343
        }
    };
    if nat_lt(a344, a345) {
        let v2: u64 = {
            let a349 = v0;
            let a350 = {
                let m348 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h347, _, _, _) => {
                        h347
                    }
                };
                match m348 {
                    Adt1::C0(_, h346, _, _, _, _, _, _, _, _, _) => {
                        h346
                    }
                }
            };
            nat_mul(a349, a350)?
        };
        Ok(Adt3::C7(f45(v2, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h351, _, _, _) => {
                h351
            }
        })?, f46(v2, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h352, _, _, _) => {
                h352
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h353, _, _) => {
                h353
            }
        })?, f45(v2, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h354, _) => {
                h354
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f11(v0: u64, v1: Adt2) -> R<Adt3> {
    let a356 = v0;
    let a357 = match v1.clone() {
        Adt2::C0(h355, _, _, _, _, _, _, _, _, _, _) => {
            h355
        }
    };
    if nat_lt(a356, a357) {
        let v2: Adt1 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h358, _) => {
                h358
            }
        };
        let v3: u64 = f23(match v2.clone() {
            Adt1::C0(h359, _, _, _, _, _, _, _, _, _, _) => {
                h359
            }
        });
        let v4: u64 = {
            let a361 = match v2.clone() {
                Adt1::C0(_, h360, _, _, _, _, _, _, _, _, _) => {
                    h360
                }
            };
            let a362 = v3;
            let a363 = 0u64;
            nat_quot(a361, a362, a363)
        };
        let v5: u64 = {
            let a364 = v0;
            let a365 = v4;
            let a366 = 0u64;
            nat_quot(a364, a365, a366)
        };
        let v6: u64 = {
            let a368 = v5;
            let a369 = match v2.clone() {
                Adt1::C0(_, _, h367, _, _, _, _, _, _, _, _) => {
                    h367
                }
            };
            let a370 = 0u64;
            nat_quot(a368, a369, a370)
        };
        Ok(Adt3::C6(f45({
            let a371 = v0;
            let a372 = v3;
            nat_mul(a371, a372)?
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, h373, _, _, _) => {
                h373
            }
        })?, f47({
            let a374 = v0;
            let a375 = v4;
            let a376 = 0u64;
            nat_rem(a374, a375, a376)
        }, {
            let a378 = v5;
            let a379 = match v2.clone() {
                Adt1::C0(_, _, h377, _, _, _, _, _, _, _, _) => {
                    h377
                }
            };
            let a380 = 0u64;
            nat_rem(a378, a379, a380)
        }, {
            let a382 = v6;
            let a383 = match v2.clone() {
                Adt1::C0(_, _, _, h381, _, _, _, _, _, _, _) => {
                    h381
                }
            };
            let a384 = 0u64;
            nat_rem(a382, a383, a384)
        }, {
            let a386 = v6;
            let a387 = match v2.clone() {
                Adt1::C0(_, _, _, h385, _, _, _, _, _, _, _) => {
                    h385
                }
            };
            let a388 = 0u64;
            nat_quot(a386, a387, a388)
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, h389, _, _, _, _, _) => {
                h389
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h390, _, _, _, _) => {
                h390
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h391, _, _, _) => {
                h391
            }
        }, match v2 {
            Adt1::C0(_, _, _, _, _, _, _, _, h392, _, _) => {
                h392
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f12(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h393, _, _, _) => {
            h393
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h394, _) => {
            h394
        }
    };
    let a396 = v0;
    let a397 = match v1.clone() {
        Adt2::C0(h395, _, _, _, _, _, _, _, _, _, _) => {
            h395
        }
    };
    if nat_lt(a396, a397) {
        Ok(Adt3::C5({
            let a399 = v0;
            let a400 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, _, h398, _, _, _, _) => {
                    h398
                }
            };
            nat_mul(a399, a400)?
        }, {
            let a402 = v0;
            let a403 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h401, _, _, _, _) => {
                    h401
                }
            };
            nat_mul(a402, a403)?
        }, match v2 {
            Adt1::C0(_, h404, _, _, _, _, _, _, _, _, _) => {
                h404
            }
        }, match v3 {
            Adt1::C0(_, h405, _, _, _, _, _, _, _, _, _) => {
                h405
            }
        }, {
            let a407 = match v1 {
                Adt2::C0(_, _, _, _, h406, _, _, _, _, _, _) => {
                    h406
                }
            };
            let a408 = 1u64;
            nat_eq(a407, a408)
        }))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f13(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let a410 = v0;
    let a411 = match v1.clone() {
        Adt2::C0(h409, _, _, _, _, _, _, _, _, _, _) => {
            h409
        }
    };
    if nat_lt(a410, a411) {
        let v3: u64 = {
            let a413 = 2u64;
            let a414 = f34(match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h412, _, _) => {
                    h412
                }
            })?;
            nat_mul(a413, a414)?
        };
        let a415 = v0;
        let a416 = v3;
        if nat_lt(a415, a416) {
            f48(v0, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h417, _, _) => {
                    h417
                }
            }, {
                let m420 = match v1 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h419, _) => {
                        h419
                    }
                };
                match m420 {
                    Adt1::C0(_, _, h418, _, _, _, _, _, _, _, _) => {
                        h418
                    }
                }
            }, v2)
        } else {
            f49({
                let a421 = v0;
                let a422 = v3;
                nat_sub(a421, a422)
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h423, _, _, _) => {
                    h423
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h424, _, _) => {
                    h424
                }
            }, match v1 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h425, _) => {
                    h425
                }
            }, v2)
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f14(v0: u64) -> bool {
    let a426 = v0;
    let a427 = 3u64;
    if nat_eq(a426, a427) {
        true
    } else {
        let a428 = v0;
        let a429 = 6u64;
        nat_eq(a428, a429)
    }
}

pub fn f15(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h430, _, _, _, _) => {
            h430
        }
    };
    let a432 = v0;
    let a433 = match v1.clone() {
        Adt2::C0(h431, _, _, _, _, _, _, _, _, _, _) => {
            h431
        }
    };
    if nat_lt(a432, a433) {
        let a434 = v0;
        let a435 = 0u64;
        if nat_eq(a434, a435) {
            Ok(Adt3::C3(1u64, 6u64, 0u64, 0u64, 2u64, v3, 2u64, 0u64, 0u64, 3u64, 4u64, 4u64))
        } else {
            let v4: u64 = {
                let a436 = v0;
                let a437 = 1u64;
                nat_sub(a436, a437)
            };
            let v5: u64 = {
                let a439 = v4;
                let a440 = f44(match v1.clone() {
                    Adt2::C0(_, _, _, _, h438, _, _, _, _, _, _) => {
                        h438
                    }
                });
                let a441 = 0u64;
                nat_rem(a439, a440, a441)
            };
            let v6: u64 = {
                let a446 = 4u64;
                let a447 = {
                    let a443 = v4;
                    let a444 = f44(match v1.clone() {
                        Adt2::C0(_, _, _, _, h442, _, _, _, _, _, _) => {
                            h442
                        }
                    });
                    let a445 = 0u64;
                    nat_quot(a443, a444, a445)
                };
                nat_mul(a446, a447)?
            };
            let a448 = v5;
            let a449 = 2u64;
            if nat_lt(a448, a449) {
                f16({
                    let a450 = v6;
                    let a451 = v5;
                    nat_add(a450, a451)?
                }, v1)
            } else {
                let a452 = v5;
                let a453 = 2u64;
                if nat_eq(a452, a453) {
                    Ok(f50(0u64, 2u64, v3))
                } else {
                    let a454 = v5;
                    let a455 = 3u64;
                    if nat_eq(a454, a455) {
                        Ok(f51(0u64, 2u64, v3, {
                            let a456 = v2;
                            let a457 = 2u64;
                            nat_eq(a456, a457)
                        }))
                    } else {
                        let a458 = v5;
                        let a459 = 4u64;
                        if nat_eq(a458, a459) {
                            Ok(f50(1u64, 1u64, 3u64))
                        } else {
                            let a460 = v5;
                            let a461 = 5u64;
                            if nat_eq(a460, a461) {
                                Ok(f51(1u64, 1u64, 3u64, {
                                    let a462 = v2;
                                    let a463 = 0u64;
                                    nat_eq(a462, a463)
                                }))
                            } else {
                                let a464 = v5;
                                let a465 = 6u64;
                                if nat_eq(a464, a465) {
                                    Ok(f50(1u64, 2u64, v3))
                                } else {
                                    let a466 = v5;
                                    let a467 = 7u64;
                                    if nat_eq(a466, a467) {
                                        Ok(f51(1u64, 2u64, v3, {
                                            let a468 = v2;
                                            let a469 = 2u64;
                                            nat_eq(a468, a469)
                                        }))
                                    } else {
                                        let a471 = match v1.clone() {
                                            Adt2::C0(_, _, _, _, h470, _, _, _, _, _, _) => {
                                                h470
                                            }
                                        };
                                        let a472 = 3u64;
                                        if nat_eq(a471, a472) {
                                            let a473 = v5;
                                            let a474 = 15u64;
                                            if nat_eq(a473, a474) {
                                                f16({
                                                    let a475 = v6;
                                                    let a476 = 3u64;
                                                    nat_add(a475, a476)?
                                                }, v1)
                                            } else {
                                                Ok(f52(v5, match v1 {
                                                    Adt2::C0(_, _, _, _, _, h477, _, _, _, _, _) => {
                                                        h477
                                                    }
                                                }))
                                            }
                                        } else {
                                            let a478 = v5;
                                            let a479 = 8u64;
                                            if nat_eq(a478, a479) {
                                                Ok(Adt3::C4(2u64, {
                                                    let m482 = match v1 {
                                                        Adt2::C0(_, _, _, _, _, _, _, _, _, h481, _) => {
                                                            h481
                                                        }
                                                    };
                                                    match m482 {
                                                        Adt1::C0(h480, _, _, _, _, _, _, _, _, _, _) => {
                                                            h480
                                                        }
                                                    }
                                                }, 0u64, 1u64, 0u64))
                                            } else {
                                                f16({
                                                    let a483 = v6;
                                                    let a484 = 3u64;
                                                    nat_add(a483, a484)?
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
        Adt2::C0(_, _, _, _, _, _, _, h485, _, _, _) => {
            h485
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h486, _, _) => {
            h486
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h487, _) => {
            h487
        }
    };
    let v5: u64 = match v1.clone() {
        Adt2::C0(_, _, _, h488, _, _, _, _, _, _, _) => {
            h488
        }
    };
    let v6: u64 = {
        let a489 = v0;
        let a490 = 4u64;
        let a491 = 0u64;
        nat_rem(a489, a490, a491)
    };
    let v7: u64 = {
        let a492 = v0;
        let a493 = 4u64;
        let a494 = 0u64;
        nat_quot(a492, a493, a494)
    };
    let v8: u64 = {
        let a496 = v7;
        let a497 = match v4.clone() {
            Adt1::C0(_, h495, _, _, _, _, _, _, _, _, _) => {
                h495
            }
        };
        let a498 = 0u64;
        nat_quot(a496, a497, a498)
    };
    let v9: u64 = f40(match v4.clone() {
        Adt1::C0(h499, _, _, _, _, _, _, _, _, _, _) => {
            h499
        }
    });
    let v10: u64 = {
        let a504 = {
            let a501 = v7;
            let a502 = match v4.clone() {
                Adt1::C0(_, h500, _, _, _, _, _, _, _, _, _) => {
                    h500
                }
            };
            let a503 = 0u64;
            nat_rem(a501, a502, a503)
        };
        let a505 = v9;
        nat_mul(a504, a505)?
    };
    let v11: u64 = {
        let a507 = match v4.clone() {
            Adt1::C0(_, h506, _, _, _, _, _, _, _, _, _) => {
                h506
            }
        };
        let a508 = v9;
        nat_mul(a507, a508)?
    };
    let a510 = v0;
    let a511 = match v1.clone() {
        Adt2::C0(h509, _, _, _, _, _, _, _, _, _, _) => {
            h509
        }
    };
    if nat_lt(a510, a511) {
        let a512 = v6;
        let a513 = 0u64;
        if nat_eq(a512, a513) {
            Ok(Adt3::C3(0u64, 0u64, match v2.clone() {
                Adt1::C0(h514, _, _, _, _, _, _, _, _, _, _) => {
                    h514
                }
            }, 0u64, 0u64, {
                let a522 = {
                    let a518 = {
                        let a516 = v8;
                        let a517 = match v2 {
                            Adt1::C0(_, _, _, _, _, _, h515, _, _, _, _) => {
                                h515
                            }
                        };
                        nat_mul(a516, a517)?
                    };
                    let a519 = v10;
                    nat_add(a518, a519)?
                };
                let a523 = {
                    let a520 = v5;
                    let a521 = 100u64;
                    if nat_eq(a520, a521) {
                        v11
                    } else {
                        0u64
                    }
                };
                nat_add(a522, a523)?
            }, 2u64, 0u64, 0u64, 0u64, v9, 4u64))
        } else {
            let a524 = v6;
            let a525 = 1u64;
            if nat_eq(a524, a525) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h526, _, _, _, _, _, _, _, _, _, _) => {
                        h526
                    }
                }, 0u64, {
                    let a527 = v5;
                    let a528 = 98u64;
                    if nat_eq(a527, a528) {
                        4u64
                    } else {
                        0u64
                    }
                }, {
                    let a536 = {
                        let a532 = {
                            let a530 = v8;
                            let a531 = match v3 {
                                Adt1::C0(_, _, _, _, _, _, h529, _, _, _, _) => {
                                    h529
                                }
                            };
                            nat_mul(a530, a531)?
                        };
                        let a533 = v10;
                        nat_add(a532, a533)?
                    };
                    let a537 = {
                        let a534 = v5;
                        let a535 = 99u64;
                        if nat_eq(a534, a535) {
                            v11
                        } else {
                            0u64
                        }
                    };
                    nat_add(a536, a537)?
                }, 2u64, 0u64, 0u64, 1u64, v9, 4u64))
            } else {
                let a538 = v6;
                let a539 = 2u64;
                if nat_eq(a538, a539) {
                    Ok(Adt3::C4(match v1 {
                        Adt2::C0(_, _, _, _, h540, _, _, _, _, _, _) => {
                            h540
                        }
                    }, match v4 {
                        Adt1::C0(h541, _, _, _, _, _, _, _, _, _, _) => {
                            h541
                        }
                    }, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt3::C3(0u64, 0u64, 0u64, match v4.clone() {
                        Adt1::C0(h542, _, _, _, _, _, _, _, _, _, _) => {
                            h542
                        }
                    }, 1u64, 0u64, 2u64, 0u64, 3u64, {
                        let a546 = {
                            let a544 = v8;
                            let a545 = match v4 {
                                Adt1::C0(_, _, _, _, _, _, h543, _, _, _, _) => {
                                    h543
                                }
                            };
                            nat_mul(a544, a545)?
                        };
                        let a547 = v10;
                        nat_add(a546, a547)?
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
        Adt2::C0(_, _, _, _, _, _, _, h548, _, _, _) => {
            h548
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h549, _, _) => {
            h549
        }
    };
    let v5: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h550, _) => {
            h550
        }
    };
    let v6: u64 = {
        let a552 = match v1.clone() {
            Adt2::C0(_, h551, _, _, _, _, _, _, _, _, _) => {
                h551
            }
        };
        let a553 = 3u64;
        if nat_eq(a552, a553) {
            0u64
        } else {
            let a554 = f34(v3.clone())?;
            let a555 = 2u64;
            nat_mul(a554, a555)?
        }
    };
    let a557 = match v1.clone() {
        Adt2::C0(h556, _, _, _, _, _, _, _, _, _, _) => {
            h556
        }
    };
    let a558 = v0;
    if nat_le(a557, a558) {
        Ok(Adt3::C1)
    } else {
        let a559 = v0;
        let a560 = v6;
        if nat_lt(a559, a560) {
            let v7: u64 = {
                let a561 = v0;
                let a562 = 2u64;
                let a563 = 0u64;
                nat_quot(a561, a562, a563)
            };
            let v8: u64 = {
                let a568 = 3u64;
                let a569 = {
                    let a565 = f45(v7, v5.clone())?;
                    let a566 = f29(match v5 {
                        Adt1::C0(h564, _, _, _, _, _, _, _, _, _, _) => {
                            h564
                        }
                    });
                    let a567 = 0u64;
                    nat_quot(a565, a566, a567)
                };
                nat_add(a568, a569)?
            };
            let a573 = {
                let a570 = v0;
                let a571 = 2u64;
                let a572 = 0u64;
                nat_rem(a570, a571, a572)
            };
            let a574 = 0u64;
            if nat_eq(a573, a574) {
                Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, v8, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
            } else {
                let a575 = v2;
                let a576 = 0u64;
                if nat_eq(a575, a576) {
                    Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 2u64, 1u64, 2u64, 0u64, 0u64, v8, 8u64, 8u64))
                } else {
                    Ok(Adt3::C1)
                }
            }
        } else {
            let v9: u64 = {
                let a577 = v0;
                let a578 = v6;
                nat_sub(a577, a578)
            };
            let v10: u64 = f35(match v3.clone() {
                Adt1::C0(h579, _, _, _, _, _, _, _, _, _, _) => {
                    h579
                }
            }, match v4.clone() {
                Adt1::C0(h580, _, _, _, _, _, _, _, _, _, _) => {
                    h580
                }
            }, match v5.clone() {
                Adt1::C0(h581, _, _, _, _, _, _, _, _, _, _) => {
                    h581
                }
            })?;
            let v11: u64 = {
                let a582 = v9;
                let a583 = v10;
                let a584 = 0u64;
                nat_quot(a582, a583, a584)
            };
            let v12: u64 = {
                let a585 = v9;
                let a586 = v10;
                let a587 = 0u64;
                nat_rem(a585, a586, a587)
            };
            let v13: u64 = {
                let a589 = match v3.clone() {
                    Adt1::C0(h588, _, _, _, _, _, _, _, _, _, _) => {
                        h588
                    }
                };
                let a590 = 0u64;
                if nat_eq(a589, a590) {
                    0u64
                } else {
                    1u64
                }
            };
            let v14: u64 = {
                let a592 = match v4.clone() {
                    Adt1::C0(h591, _, _, _, _, _, _, _, _, _, _) => {
                        h591
                    }
                };
                let a593 = 0u64;
                if nat_eq(a592, a593) {
                    0u64
                } else {
                    1u64
                }
            };
            let a594 = v12;
            let a595 = v13;
            if nat_lt(a594, a595) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h596, _, _, _, _, _, _, _, _, _, _) => {
                        h596
                    }
                }, 0u64, 0u64, f45(v11, v3.clone())?, 2u64, 0u64, 0u64, 0u64, f29(match v3 {
                    Adt1::C0(h597, _, _, _, _, _, _, _, _, _, _) => {
                        h597
                    }
                }), 4u64))
            } else {
                let a600 = v12;
                let a601 = {
                    let a598 = v13;
                    let a599 = v14;
                    nat_add(a598, a599)?
                };
                if nat_lt(a600, a601) {
                    Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                        Adt1::C0(h602, _, _, _, _, _, _, _, _, _, _) => {
                            h602
                        }
                    }, 0u64, 4u64, f46(v11, v3, v4.clone())?, 2u64, 0u64, 0u64, 1u64, f29(match v4 {
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
                    if nat_eq(a606, a607) {
                        Ok(Adt3::C3(1u64, match v1 {
                            Adt2::C0(_, _, _, h608, _, _, _, _, _, _, _) => {
                                h608
                            }
                        }, 0u64, 0u64, {
                            let a609 = v13;
                            let a610 = 0u64;
                            if nat_eq(a609, a610) {
                                0u64
                            } else {
                                1u64
                            }
                        }, {
                            let a611 = v13;
                            let a612 = 0u64;
                            if nat_eq(a611, a612) {
                                f45(v11, v3.clone())?
                            } else {
                                0u64
                            }
                        }, {
                            let a613 = v14;
                            let a614 = 0u64;
                            if nat_eq(a613, a614) {
                                4u64
                            } else {
                                1u64
                            }
                        }, {
                            let a615 = v14;
                            let a616 = 0u64;
                            if nat_eq(a615, a616) {
                                f46(v11, v3, v4)?
                            } else {
                                1u64
                            }
                        }, {
                            let a618 = match v5.clone() {
                                Adt1::C0(h617, _, _, _, _, _, _, _, _, _, _) => {
                                    h617
                                }
                            };
                            let a619 = 0u64;
                            if nat_eq(a618, a619) {
                                3u64
                            } else {
                                0u64
                            }
                        }, {
                            let a621 = match v5.clone() {
                                Adt1::C0(h620, _, _, _, _, _, _, _, _, _, _) => {
                                    h620
                                }
                            };
                            let a622 = 0u64;
                            if nat_eq(a621, a622) {
                                f45(v11, v5)?
                            } else {
                                0u64
                            }
                        }, 4u64, 4u64))
                    } else {
                        Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                            Adt1::C0(h623, _, _, _, _, _, _, _, _, _, _) => {
                                h623
                            }
                        }, 1u64, 0u64, 2u64, 0u64, 3u64, f45(v11, v5.clone())?, 4u64, f29(match v5 {
                            Adt1::C0(h624, _, _, _, _, _, _, _, _, _, _) => {
                                h624
                            }
                        })))
                    }
                }
            }
        }
    }
}

pub fn f18(v0: Adt1, v1: Adt1) -> bool {
    if f26(v1.clone()) {
        f26(v0)
    } else {
        let a630 = {
            let a627 = match v0.clone() {
                Adt1::C0(_, h625, _, _, _, _, _, _, _, _, _) => {
                    h625
                }
            };
            let a628 = match v1.clone() {
                Adt1::C0(_, h626, _, _, _, _, _, _, _, _, _) => {
                    h626
                }
            };
            let a629 = 1u64;
            nat_rem(a627, a628, a629)
        };
        let a631 = 0u64;
        if if if nat_eq(a630, a631) {
            let a637 = {
                let a634 = match v0.clone() {
                    Adt1::C0(_, _, h632, _, _, _, _, _, _, _, _) => {
                        h632
                    }
                };
                let a635 = match v1.clone() {
                    Adt1::C0(_, _, h633, _, _, _, _, _, _, _, _) => {
                        h633
                    }
                };
                let a636 = 1u64;
                nat_rem(a634, a635, a636)
            };
            let a638 = 0u64;
            nat_eq(a637, a638)
        } else {
            false
        } {
            let a644 = {
                let a641 = match v0.clone() {
                    Adt1::C0(_, _, _, h639, _, _, _, _, _, _, _) => {
                        h639
                    }
                };
                let a642 = match v1.clone() {
                    Adt1::C0(_, _, _, h640, _, _, _, _, _, _, _) => {
                        h640
                    }
                };
                let a643 = 1u64;
                nat_rem(a641, a642, a643)
            };
            let a645 = 0u64;
            nat_eq(a644, a645)
        } else {
            false
        } {
            let a651 = {
                let a648 = match v0 {
                    Adt1::C0(_, _, _, _, h646, _, _, _, _, _, _) => {
                        h646
                    }
                };
                let a649 = match v1 {
                    Adt1::C0(_, _, _, _, h647, _, _, _, _, _, _) => {
                        h647
                    }
                };
                let a650 = 1u64;
                nat_rem(a648, a649, a650)
            };
            let a652 = 0u64;
            nat_eq(a651, a652)
        } else {
            false
        }
    }
}

pub fn f19(v0: Adt1, v1: Adt1) -> bool {
    let a655 = match v0.clone() {
        Adt1::C0(_, h653, _, _, _, _, _, _, _, _, _) => {
            h653
        }
    };
    let a656 = match v1.clone() {
        Adt1::C0(_, h654, _, _, _, _, _, _, _, _, _) => {
            h654
        }
    };
    if if if nat_eq(a655, a656) {
        let a659 = match v0.clone() {
            Adt1::C0(_, _, h657, _, _, _, _, _, _, _, _) => {
                h657
            }
        };
        let a660 = match v1.clone() {
            Adt1::C0(_, _, h658, _, _, _, _, _, _, _, _) => {
                h658
            }
        };
        nat_eq(a659, a660)
    } else {
        false
    } {
        let a663 = match v0.clone() {
            Adt1::C0(_, _, _, h661, _, _, _, _, _, _, _) => {
                h661
            }
        };
        let a664 = match v1.clone() {
            Adt1::C0(_, _, _, h662, _, _, _, _, _, _, _) => {
                h662
            }
        };
        nat_eq(a663, a664)
    } else {
        false
    } {
        let a667 = match v0 {
            Adt1::C0(_, _, _, _, h665, _, _, _, _, _, _) => {
                h665
            }
        };
        let a668 = match v1 {
            Adt1::C0(_, _, _, _, h666, _, _, _, _, _, _) => {
                h666
            }
        };
        nat_eq(a667, a668)
    } else {
        false
    }
}

pub fn f20(v0: u64, v1: u64, v2: Adt1, v3: u64) -> bool {
    let v4: u64 = match v2.clone() {
        Adt1::C0(h669, _, _, _, _, _, _, _, _, _, _) => {
            h669
        }
    };
    let a670 = v1;
    let a671 = v3;
    if if if nat_eq(a670, a671) {
        f2(v1)
    } else {
        false
    } {
        true
    } else {
        let a672 = v0;
        let a673 = 0u64;
        if nat_eq(a672, a673) {
            let a674 = v4;
            let a675 = 0u64;
            nat_eq(a674, a675)
        } else {
            false
        }
    } {
        let a676 = 0u64;
        let a677 = f29(v4);
        if if if nat_lt(a676, a677) {
            true
        } else {
            if f2(v4) {
                let a678 = v4;
                let a679 = 9u64;
                !if nat_eq(a678, a679) {
                    true
                } else {
                    let a680 = v4;
                    let a681 = 15u64;
                    nat_eq(a680, a681)
                }
            } else {
                false
            }
        } {
            let a683 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, h682, _, _, _, _, _) => {
                    h682
                }
            };
            let a684 = f22(v4);
            nat_eq(a683, a684)
        } else {
            false
        } {
            let a689 = {
                let a686 = match v2 {
                    Adt1::C0(_, h685, _, _, _, _, _, _, _, _, _) => {
                        h685
                    }
                };
                let a687 = f23(v4);
                let a688 = 1u64;
                nat_rem(a686, a687, a688)
            };
            let a690 = 0u64;
            nat_eq(a689, a690)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f21(v0: u64, v1: u64) -> bool {
    if f2(v0) {
        let a691 = v0;
        let a692 = 9u64;
        !if nat_eq(a691, a692) {
            true
        } else {
            let a693 = v0;
            let a694 = 15u64;
            nat_eq(a693, a694)
        }
    } else {
        let a695 = f40(v0);
        let a696 = 2u64;
        if nat_eq(a695, a696) {
            f2(v1)
        } else {
            false
        }
    }
}

pub fn f22(v0: u64) -> u64 {
    if f2(v0) {
        let m697 = f53(v0);
        match m697 {
            Adt4::C0 => {
                0u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, _, h698) => {
                        h698
                    }
                }
            }
        }
    } else {
        let a699 = v0;
        let a700 = 26u64;
        if nat_eq(a699, a700) {
            4u64
        } else {
            f29(v0)
        }
    }
}

pub fn f23(v0: u64) -> u64 {
    if f2(v0) {
        let m701 = f53(v0);
        match m701 {
            Adt4::C0 => {
                1u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, h702, _) => {
                        h702
                    }
                }
            }
        }
    } else {
        1u64
    }
}

pub fn f24(v0: Adt1, v1: Adt1) -> R<bool> {
    if f42(v0.clone(), v1.clone())? {
        Ok(true)
    } else {
        Ok(f54(v0, v1))
    }
}

pub fn f25(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f55(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f55(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f55(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f55(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f55(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f55(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f26(v0: Adt1) -> bool {
    f56(match v0.clone() {
        Adt1::C0(_, h703, _, _, _, _, _, _, _, _, _) => {
            h703
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h704, _, _, _, _, _, _, _, _) => {
            h704
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h705, _, _, _, _, _, _, _) => {
            h705
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, h706, _, _, _, _, _, _) => {
            h706
        }
    })
}

pub fn f27(v0: u64) -> Option<u64> {
    let a707 = v0;
    let a708 = 16u64;
    if if if nat_eq(a707, a708) {
        true
    } else {
        let a709 = v0;
        let a710 = 17u64;
        nat_eq(a709, a710)
    } {
        true
    } else {
        let a711 = v0;
        let a712 = 19u64;
        nat_eq(a711, a712)
    } {
        Some(1065353216u64)
    } else {
        None::<u64>
    }
}

pub fn f28(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<bool> {
    let a714 = match v1.clone() {
        Adt1::C0(h713, _, _, _, _, _, _, _, _, _, _) => {
            h713
        }
    };
    let a715 = 43u64;
    if nat_eq(a714, a715) {
        let a716 = v0;
        let a717 = 0u64;
        if if if if if if nat_eq(a716, a717) {
            let a719 = match v2.clone() {
                Adt1::C0(h718, _, _, _, _, _, _, _, _, _, _) => {
                    h718
                }
            };
            let a720 = 0u64;
            nat_eq(a719, a720)
        } else {
            false
        } {
            let a722 = match v3.clone() {
                Adt1::C0(h721, _, _, _, _, _, _, _, _, _, _) => {
                    h721
                }
            };
            let a723 = 0u64;
            if nat_eq(a722, a723) {
                true
            } else {
                let a725 = match v3.clone() {
                    Adt1::C0(h724, _, _, _, _, _, _, _, _, _, _) => {
                        h724
                    }
                };
                let a726 = 43u64;
                nat_eq(a725, a726)
            }
        } else {
            false
        } {
            let a728 = match v3.clone() {
                Adt1::C0(h727, _, _, _, _, _, _, _, _, _, _) => {
                    h727
                }
            };
            let a729 = 43u64;
            if nat_eq(a728, a729) {
                f18(v1.clone(), v2.clone())
            } else {
                f19(v1.clone(), v2.clone())
            }
        } else {
            false
        } {
            let a731 = match v2 {
                Adt1::C0(_, _, _, _, _, h730, _, _, _, _, _) => {
                    h730
                }
            };
            let a732 = 4u64;
            nat_eq(a731, a732)
        } else {
            false
        } {
            let a735 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h733, _, _, _, _, _) => {
                    h733
                }
            };
            let a736 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h734, _, _, _, _) => {
                    h734
                }
            };
            if nat_le(a735, a736) {
                let a739 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, h737, _, _, _, _) => {
                        h737
                    }
                };
                let a740 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h738, _, _, _) => {
                        h738
                    }
                };
                if nat_le(a739, a740) {
                    let a743 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, h741, _, _, _) => {
                            h741
                        }
                    };
                    let a744 = match v3 {
                        Adt1::C0(_, _, _, _, _, _, _, _, h742, _, _) => {
                            h742
                        }
                    };
                    nat_le(a743, a744)
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        } {
            let a752 = {
                let a750 = match v1.clone() {
                    Adt1::C0(_, _, h745, _, _, _, _, _, _, _, _) => {
                        h745
                    }
                };
                let a751 = {
                    let a748 = match v1.clone() {
                        Adt1::C0(_, _, _, h746, _, _, _, _, _, _, _) => {
                            h746
                        }
                    };
                    let a749 = match v1 {
                        Adt1::C0(_, _, _, _, h747, _, _, _, _, _, _) => {
                            h747
                        }
                    };
                    nat_mul(a748, a749)?
                };
                nat_mul(a750, a751)?
            };
            let a753 = 2147483647u64;
            Ok(nat_le(a752, a753))
        } else {
            Ok(false)
        }
    } else {
        if f18(v1.clone(), v2.clone()) {
            Ok(f57(match v1 {
                Adt1::C0(h754, _, _, _, _, _, _, _, _, _, _) => {
                    h754
                }
            }, match v2 {
                Adt1::C0(h755, _, _, _, _, _, _, _, _, _, _) => {
                    h755
                }
            }, match v3 {
                Adt1::C0(h756, _, _, _, _, _, _, _, _, _, _) => {
                    h756
                }
            }))
        } else {
            Ok(false)
        }
    }
}

pub fn f29(v0: u64) -> u64 {
    let a757 = v0;
    let a758 = 43u64;
    if nat_eq(a757, a758) {
        1u64
    } else {
        f40(v0)
    }
}

pub fn f30(v0: Adt1) -> bool {
    let v1: u64 = f29(match v0.clone() {
        Adt1::C0(h759, _, _, _, _, _, _, _, _, _, _) => {
            h759
        }
    });
    let a764 = {
        let a761 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h760) => {
                h760
            }
        };
        let a762 = v1;
        let a763 = 1u64;
        nat_rem(a761, a762, a763)
    };
    let a765 = 0u64;
    if if nat_eq(a764, a765) {
        f58(match v0.clone() {
            Adt1::C0(_, h766, _, _, _, _, _, _, _, _, _) => {
                h766
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h767, _, _, _, _, _) => {
                h767
            }
        }, v1)
    } else {
        false
    } {
        f59(match v0.clone() {
            Adt1::C0(_, _, h768, _, _, _, _, _, _, _, _) => {
                h768
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h769, _, _, _, _, _, _, _) => {
                h769
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h770, _, _, _, _, _, _) => {
                h770
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h771, _, _, _, _) => {
                h771
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h772, _, _, _) => {
                h772
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h773, _, _) => {
                h773
            }
        }, v1)
    } else {
        false
    }
}

pub fn f31(v0: Adt1, v1: Adt1) -> R<bool> {
    let a781 = {
        let a779 = match v0.clone() {
            Adt1::C0(_, _, h774, _, _, _, _, _, _, _, _) => {
                h774
            }
        };
        let a780 = {
            let a777 = match v0.clone() {
                Adt1::C0(_, _, _, h775, _, _, _, _, _, _, _) => {
                    h775
                }
            };
            let a778 = match v0.clone() {
                Adt1::C0(_, _, _, _, h776, _, _, _, _, _, _) => {
                    h776
                }
            };
            nat_mul(a777, a778)?
        };
        nat_mul(a779, a780)?
    };
    let a782 = 9223372036854775807u64;
    if nat_le(a781, a782) {
        if f38(v1.clone()) {
            let a784 = match v1 {
                Adt1::C0(_, h783, _, _, _, _, _, _, _, _, _) => {
                    h783
                }
            };
            let a785 = 2147483647u64;
            Ok(nat_le(a784, a785))
        } else {
            let a787 = match v0 {
                Adt1::C0(_, h786, _, _, _, _, _, _, _, _, _) => {
                    h786
                }
            };
            let a788 = 2147483647u64;
            Ok(nat_le(a787, a788))
        }
    } else {
        Ok(false)
    }
}

pub fn f32(v0: Adt1) -> R<bool> {
    f25({
        let a791 = match v0.clone() {
            Adt1::C0(_, h789, _, _, _, _, _, _, _, _, _) => {
                h789
            }
        };
        let a792 = f29(match v0.clone() {
            Adt1::C0(h790, _, _, _, _, _, _, _, _, _, _) => {
                h790
            }
        });
        nat_mul(a791, a792)?
    }, match v0.clone() {
        Adt1::C0(_, _, h793, _, _, _, _, _, _, _, _) => {
            h793
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h794, _, _, _, _, _, _, _) => {
            h794
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h795, _, _, _, _, _, _) => {
            h795
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h796, _, _, _, _) => {
            h796
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h797, _, _, _) => {
            h797
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h798, _, _) => {
            h798
        }
    })
}

pub fn f33(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a805 = {
        let a799 = v0;
        let a800 = v2;
        nat_mul(a799, a800)?
    };
    let a806 = {
        let a801 = v1;
        let a802 = 0u64;
        if nat_eq(a801, a802) {
            0u64
        } else {
            let a803 = v0;
            let a804 = 2u64;
            nat_mul(a803, a804)?
        }
    };
    nat_add(a805, a806)
}

pub fn f34(v0: Adt1) -> R<u64> {
    let a815 = {
        let a809 = match v0.clone() {
            Adt1::C0(_, h807, _, _, _, _, _, _, _, _, _) => {
                h807
            }
        };
        let a810 = match v0.clone() {
            Adt1::C0(_, _, h808, _, _, _, _, _, _, _, _) => {
                h808
            }
        };
        nat_mul(a809, a810)?
    };
    let a816 = {
        let a813 = match v0.clone() {
            Adt1::C0(_, _, _, h811, _, _, _, _, _, _, _) => {
                h811
            }
        };
        let a814 = match v0 {
            Adt1::C0(_, _, _, _, h812, _, _, _, _, _, _) => {
                h812
            }
        };
        nat_mul(a813, a814)?
    };
    nat_mul(a815, a816)
}

pub fn f35(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a827 = {
        let a823 = {
            let a819 = 1u64;
            let a820 = {
                let a817 = v0;
                let a818 = 0u64;
                if nat_eq(a817, a818) {
                    0u64
                } else {
                    1u64
                }
            };
            nat_add(a819, a820)?
        };
        let a824 = {
            let a821 = v1;
            let a822 = 0u64;
            if nat_eq(a821, a822) {
                0u64
            } else {
                1u64
            }
        };
        nat_add(a823, a824)?
    };
    let a828 = {
        let a825 = v2;
        let a826 = 0u64;
        if nat_eq(a825, a826) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a827, a828)
}

pub fn f36(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f19(v0.clone(), v1.clone()) {
        let a831 = match v0.clone() {
            Adt1::C0(h829, _, _, _, _, _, _, _, _, _, _) => {
                h829
            }
        };
        let a832 = match v1.clone() {
            Adt1::C0(h830, _, _, _, _, _, _, _, _, _, _) => {
                h830
            }
        };
        nat_eq(a831, a832)
    } else {
        false
    } {
        let a835 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h833, _, _, _, _, _) => {
                h833
            }
        };
        let a836 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h834, _, _, _, _, _) => {
                h834
            }
        };
        nat_eq(a835, a836)
    } else {
        false
    } {
        let a839 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h837, _, _, _, _) => {
                h837
            }
        };
        let a840 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h838, _, _, _, _) => {
                h838
            }
        };
        nat_eq(a839, a840)
    } else {
        false
    } {
        let a843 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h841, _, _, _) => {
                h841
            }
        };
        let a844 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h842, _, _, _) => {
                h842
            }
        };
        nat_eq(a843, a844)
    } else {
        false
    } {
        let a847 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h845, _, _) => {
                h845
            }
        };
        let a848 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h846, _, _) => {
                h846
            }
        };
        nat_eq(a847, a848)
    } else {
        false
    } {
        let a850 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h849) => {
                h849
            }
        };
        let a851 = 0u64;
        if if nat_eq(a850, a851) {
            true
        } else {
            let a853 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h852) => {
                    h852
                }
            };
            let a854 = 0u64;
            nat_eq(a853, a854)
        } {
            true
        } else {
            let a857 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h855) => {
                    h855
                }
            };
            let a858 = match v1 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h856) => {
                    h856
                }
            };
            nat_eq(a857, a858)
        }
    } else {
        false
    }
}

pub fn f37(v0: u64) -> u64 {
    let a859 = v0;
    let a860 = 26u64;
    if nat_eq(a859, a860) {
        4u64
    } else {
        let a861 = v0;
        let a862 = 27u64;
        if nat_eq(a861, a862) {
            8u64
        } else {
            0u64
        }
    }
}

pub fn f38(v0: Adt1) -> bool {
    let a864 = match v0.clone() {
        Adt1::C0(_, h863, _, _, _, _, _, _, _, _, _) => {
            h863
        }
    };
    let a865 = 1u64;
    if nat_eq(a864, a865) {
        true
    } else {
        let a868 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h866, _, _, _, _, _) => {
                h866
            }
        };
        let a869 = f29(match v0 {
            Adt1::C0(h867, _, _, _, _, _, _, _, _, _, _) => {
                h867
            }
        });
        nat_eq(a868, a869)
    }
}

pub fn f39(v0: Adt1, v1: u64) -> R<u64> {
    let a875 = {
        let a871 = match v0.clone() {
            Adt1::C0(_, h870, _, _, _, _, _, _, _, _, _) => {
                h870
            }
        };
        let a872 = f23(v1);
        let a873 = 0u64;
        nat_quot(a871, a872, a873)
    };
    let a876 = f60(match v0 {
        Adt1::C0(h874, _, _, _, _, _, _, _, _, _, _) => {
            h874
        }
    }, v1);
    nat_mul(a875, a876)
}

pub fn f40(v0: u64) -> u64 {
    let a877 = v0;
    let a878 = 0u64;
    if nat_eq(a877, a878) {
        4u64
    } else {
        let a879 = v0;
        let a880 = 1u64;
        if if nat_eq(a879, a880) {
            true
        } else {
            let a881 = v0;
            let a882 = 30u64;
            nat_eq(a881, a882)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f41(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a884 = match v0.clone() {
            Adt1::C0(_, h883, _, _, _, _, _, _, _, _, _) => {
                h883
            }
        };
        let a885 = 4u64;
        nat_mul(a884, a885)?
    };
    let v2: u64 = {
        let a888 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h886, _, _, _, _) => {
                h886
            }
        };
        let a889 = match v0.clone() {
            Adt1::C0(_, _, h887, _, _, _, _, _, _, _, _) => {
                h887
            }
        };
        nat_mul(a888, a889)?
    };
    let v3: u64 = {
        let a891 = v2;
        let a892 = match v0.clone() {
            Adt1::C0(_, _, _, h890, _, _, _, _, _, _, _) => {
                h890
            }
        };
        nat_mul(a891, a892)?
    };
    let a894 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h893, _, _, _, _, _) => {
            h893
        }
    };
    let a895 = 4u64;
    if if if if nat_eq(a894, a895) {
        let a900 = {
            let a897 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h896) => {
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
    } {
        let a909 = {
            let a907 = match v0.clone() {
                Adt1::C0(_, _, h902, _, _, _, _, _, _, _, _) => {
                    h902
                }
            };
            let a908 = {
                let a905 = match v0.clone() {
                    Adt1::C0(_, _, _, h903, _, _, _, _, _, _, _) => {
                        h903
                    }
                };
                let a906 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h904, _, _, _, _, _, _) => {
                        h904
                    }
                };
                nat_mul(a905, a906)?
            };
            nat_mul(a907, a908)?
        };
        let a910 = 1u64;
        if nat_le(a909, a910) {
            true
        } else {
            let a912 = v1;
            let a913 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h911, _, _, _, _) => {
                    h911
                }
            };
            if nat_le(a912, a913) {
                let a918 = {
                    let a915 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, h914, _, _, _, _) => {
                            h914
                        }
                    };
                    let a916 = 4u64;
                    let a917 = 1u64;
                    nat_rem(a915, a916, a917)
                };
                let a919 = 0u64;
                nat_eq(a918, a919)
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a921 = match v0.clone() {
            Adt1::C0(_, _, _, h920, _, _, _, _, _, _, _) => {
                h920
            }
        };
        let a922 = 1u64;
        if nat_le(a921, a922) {
            true
        } else {
            let a924 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h923, _, _, _) => {
                    h923
                }
            };
            let a925 = v2;
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
        if nat_le(a927, a928) {
            Ok(true)
        } else {
            let a930 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, h929, _, _) => {
                    h929
                }
            };
            let a931 = v3;
            Ok(nat_eq(a930, a931))
        }
    } else {
        Ok(false)
    }
}

pub fn f42(v0: Adt1, v1: Adt1) -> R<bool> {
    let a933 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h932, _) => {
            h932
        }
    };
    let a934 = 0u64;
    if nat_eq(a933, a934) {
        Ok(true)
    } else {
        let a936 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h935, _) => {
                h935
            }
        };
        let a937 = 0u64;
        if nat_eq(a936, a937) {
            Ok(true)
        } else {
            let a939 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h938) => {
                    h938
                }
            };
            let a940 = 0u64;
            if nat_eq(a939, a940) {
                Ok(true)
            } else {
                let a942 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h941) => {
                        h941
                    }
                };
                let a943 = 0u64;
                if nat_eq(a942, a943) {
                    Ok(true)
                } else {
                    let a949 = {
                        let a946 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h944) => {
                                h944
                            }
                        };
                        let a947 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h945, _) => {
                                h945
                            }
                        };
                        nat_add(a946, a947)?
                    };
                    let a950 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h948) => {
                            h948
                        }
                    };
                    if nat_le(a949, a950) {
                        Ok(true)
                    } else {
                        let a956 = {
                            let a953 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h951) => {
                                    h951
                                }
                            };
                            let a954 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h952, _) => {
                                    h952
                                }
                            };
                            nat_add(a953, a954)?
                        };
                        let a957 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h955) => {
                                h955
                            }
                        };
                        Ok(nat_le(a956, a957))
                    }
                }
            }
        }
    }
}

pub fn f43(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a960 = match v0.clone() {
            Adt1::C0(_, _, h958, _, _, _, _, _, _, _, _) => {
                h958
            }
        };
        let a961 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h959, _, _, _, _) => {
                h959
            }
        };
        nat_mul(a960, a961)?
    };
    if if f38(v0.clone()) {
        let a963 = match v0.clone() {
            Adt1::C0(_, _, _, h962, _, _, _, _, _, _, _) => {
                h962
            }
        };
        let a964 = 1u64;
        if nat_eq(a963, a964) {
            true
        } else {
            let a966 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h965, _, _, _) => {
                    h965
                }
            };
            let a967 = v1;
            nat_eq(a966, a967)
        }
    } else {
        false
    } {
        let a969 = match v0.clone() {
            Adt1::C0(_, _, _, _, h968, _, _, _, _, _, _) => {
                h968
            }
        };
        let a970 = 1u64;
        if nat_eq(a969, a970) {
            Ok(true)
        } else {
            let a975 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h971, _, _) => {
                    h971
                }
            };
            let a976 = {
                let a973 = v1;
                let a974 = match v0 {
                    Adt1::C0(_, _, _, h972, _, _, _, _, _, _, _) => {
                        h972
                    }
                };
                nat_mul(a973, a974)?
            };
            Ok(nat_eq(a975, a976))
        }
    } else {
        Ok(false)
    }
}

pub fn f44(v0: u64) -> u64 {
    let a977 = v0;
    let a978 = 3u64;
    if nat_eq(a977, a978) {
        16u64
    } else {
        10u64
    }
}

pub fn f45(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a980 = v0;
        let a981 = match v1.clone() {
            Adt1::C0(_, h979, _, _, _, _, _, _, _, _, _) => {
                h979
            }
        };
        let a982 = 0u64;
        nat_quot(a980, a981, a982)
    };
    let v3: u64 = {
        let a984 = v2;
        let a985 = match v1.clone() {
            Adt1::C0(_, _, h983, _, _, _, _, _, _, _, _) => {
                h983
            }
        };
        let a986 = 0u64;
        nat_quot(a984, a985, a986)
    };
    f47({
        let a988 = v0;
        let a989 = match v1.clone() {
            Adt1::C0(_, h987, _, _, _, _, _, _, _, _, _) => {
                h987
            }
        };
        let a990 = 0u64;
        nat_rem(a988, a989, a990)
    }, {
        let a992 = v2;
        let a993 = match v1.clone() {
            Adt1::C0(_, _, h991, _, _, _, _, _, _, _, _) => {
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
        nat_rem(a996, a997, a998)
    }, {
        let a1000 = v3;
        let a1001 = match v1.clone() {
            Adt1::C0(_, _, _, h999, _, _, _, _, _, _, _) => {
                h999
            }
        };
        let a1002 = 0u64;
        nat_quot(a1000, a1001, a1002)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h1003, _, _, _, _, _) => {
            h1003
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h1004, _, _, _, _) => {
            h1004
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1005, _, _, _) => {
            h1005
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1006, _, _) => {
            h1006
        }
    })
}

pub fn f46(v0: u64, v1: Adt1, v2: Adt1) -> R<u64> {
    let v3: u64 = {
        let a1008 = v0;
        let a1009 = match v1.clone() {
            Adt1::C0(_, h1007, _, _, _, _, _, _, _, _, _) => {
                h1007
            }
        };
        let a1010 = 0u64;
        nat_quot(a1008, a1009, a1010)
    };
    let v4: u64 = {
        let a1012 = v3;
        let a1013 = match v1.clone() {
            Adt1::C0(_, _, h1011, _, _, _, _, _, _, _, _) => {
                h1011
            }
        };
        let a1014 = 0u64;
        nat_quot(a1012, a1013, a1014)
    };
    f47({
        let a1020 = {
            let a1016 = v0;
            let a1017 = match v1.clone() {
                Adt1::C0(_, h1015, _, _, _, _, _, _, _, _, _) => {
                    h1015
                }
            };
            let a1018 = 0u64;
            nat_rem(a1016, a1017, a1018)
        };
        let a1021 = match v2.clone() {
            Adt1::C0(_, h1019, _, _, _, _, _, _, _, _, _) => {
                h1019
            }
        };
        let a1022 = 0u64;
        nat_rem(a1020, a1021, a1022)
    }, {
        let a1028 = {
            let a1024 = v3;
            let a1025 = match v1.clone() {
                Adt1::C0(_, _, h1023, _, _, _, _, _, _, _, _) => {
                    h1023
                }
            };
            let a1026 = 0u64;
            nat_rem(a1024, a1025, a1026)
        };
        let a1029 = match v2.clone() {
            Adt1::C0(_, _, h1027, _, _, _, _, _, _, _, _) => {
                h1027
            }
        };
        let a1030 = 0u64;
        nat_rem(a1028, a1029, a1030)
    }, {
        let a1036 = {
            let a1032 = v4;
            let a1033 = match v1.clone() {
                Adt1::C0(_, _, _, h1031, _, _, _, _, _, _, _) => {
                    h1031
                }
            };
            let a1034 = 0u64;
            nat_rem(a1032, a1033, a1034)
        };
        let a1037 = match v2.clone() {
            Adt1::C0(_, _, _, h1035, _, _, _, _, _, _, _) => {
                h1035
            }
        };
        let a1038 = 0u64;
        nat_rem(a1036, a1037, a1038)
    }, {
        let a1044 = {
            let a1040 = v4;
            let a1041 = match v1 {
                Adt1::C0(_, _, _, h1039, _, _, _, _, _, _, _) => {
                    h1039
                }
            };
            let a1042 = 0u64;
            nat_quot(a1040, a1041, a1042)
        };
        let a1045 = match v2.clone() {
            Adt1::C0(_, _, _, _, h1043, _, _, _, _, _, _) => {
                h1043
            }
        };
        let a1046 = 0u64;
        nat_rem(a1044, a1045, a1046)
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h1047, _, _, _, _, _) => {
            h1047
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h1048, _, _, _, _) => {
            h1048
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1049, _, _, _) => {
            h1049
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1050, _, _) => {
            h1050
        }
    })
}

pub fn f47(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a1063 = {
        let a1059 = {
            let a1055 = {
                let a1051 = v0;
                let a1052 = v4;
                nat_mul(a1051, a1052)?
            };
            let a1056 = {
                let a1053 = v1;
                let a1054 = v5;
                nat_mul(a1053, a1054)?
            };
            nat_add(a1055, a1056)?
        };
        let a1060 = {
            let a1057 = v2;
            let a1058 = v6;
            nat_mul(a1057, a1058)?
        };
        nat_add(a1059, a1060)?
    };
    let a1064 = {
        let a1061 = v3;
        let a1062 = v7;
        nat_mul(a1061, a1062)?
    };
    nat_add(a1063, a1064)
}

pub fn f48(v0: u64, v1: Adt1, v2: u64, v3: u64) -> R<Adt3> {
    let v4: u64 = {
        let a1065 = v0;
        let a1066 = 2u64;
        let a1067 = 0u64;
        nat_quot(a1065, a1066, a1067)
    };
    let v5: u64 = f37(match v1.clone() {
        Adt1::C0(h1068, _, _, _, _, _, _, _, _, _, _) => {
            h1068
        }
    });
    let a1072 = {
        let a1069 = v0;
        let a1070 = 2u64;
        let a1071 = 0u64;
        nat_rem(a1069, a1070, a1071)
    };
    let a1073 = 0u64;
    if nat_eq(a1072, a1073) {
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 4u64, f45(v4, v1)?, 2u64, 0u64, 0u64, 2u64, v5, 8u64))
    } else {
        let a1074 = v3;
        let a1075 = v2;
        if if nat_lt(a1074, a1075) {
            let a1078 = v3;
            let a1079 = {
                let a1076 = v5;
                let a1077 = 4u64;
                if nat_eq(a1076, a1077) {
                    2147483648u64
                } else {
                    9223372036854775808u64
                }
            };
            nat_lt(a1078, a1079)
        } else {
            false
        } {
            Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, {
                let a1080 = 3u64;
                let a1081 = v4;
                nat_add(a1080, a1081)?
            }, 8u64, 8u64))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f49(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64) -> R<Adt3> {
    let v5: u64 = {
        let a1083 = 1u64;
        let a1084 = f39(v1.clone(), match v3.clone() {
            Adt1::C0(h1082, _, _, _, _, _, _, _, _, _, _) => {
                h1082
            }
        })?;
        nat_add(a1083, a1084)?
    };
    let v6: u64 = {
        let a1085 = v0;
        let a1086 = v5;
        let a1087 = 0u64;
        nat_rem(a1085, a1086, a1087)
    };
    let v7: u64 = {
        let a1088 = v0;
        let a1089 = v5;
        let a1090 = 0u64;
        nat_quot(a1088, a1089, a1090)
    };
    let v8: u64 = {
        let a1092 = v7;
        let a1093 = match v1.clone() {
            Adt1::C0(_, _, h1091, _, _, _, _, _, _, _, _) => {
                h1091
            }
        };
        let a1094 = 0u64;
        nat_rem(a1092, a1093, a1094)
    };
    let v9: u64 = {
        let a1096 = v7;
        let a1097 = match v1.clone() {
            Adt1::C0(_, _, h1095, _, _, _, _, _, _, _, _) => {
                h1095
            }
        };
        let a1098 = 0u64;
        nat_quot(a1096, a1097, a1098)
    };
    let v10: u64 = {
        let a1100 = v9;
        let a1101 = match v1.clone() {
            Adt1::C0(_, _, _, h1099, _, _, _, _, _, _, _) => {
                h1099
            }
        };
        let a1102 = 0u64;
        nat_rem(a1100, a1101, a1102)
    };
    let v11: u64 = {
        let a1104 = v9;
        let a1105 = match v1.clone() {
            Adt1::C0(_, _, _, h1103, _, _, _, _, _, _, _) => {
                h1103
            }
        };
        let a1106 = 0u64;
        nat_quot(a1104, a1105, a1106)
    };
    let a1107 = v6;
    let a1108 = 0u64;
    if nat_eq(a1107, a1108) {
        let v12: u64 = {
            let a1125 = v8;
            let a1126 = {
                let a1123 = match v2.clone() {
                    Adt1::C0(_, h1109, _, _, _, _, _, _, _, _, _) => {
                        h1109
                    }
                };
                let a1124 = {
                    let a1121 = {
                        let a1111 = v10;
                        let a1112 = match v2.clone() {
                            Adt1::C0(_, _, h1110, _, _, _, _, _, _, _, _) => {
                                h1110
                            }
                        };
                        let a1113 = 0u64;
                        nat_rem(a1111, a1112, a1113)
                    };
                    let a1122 = {
                        let a1119 = match v2.clone() {
                            Adt1::C0(_, _, h1114, _, _, _, _, _, _, _, _) => {
                                h1114
                            }
                        };
                        let a1120 = {
                            let a1116 = v11;
                            let a1117 = match v2 {
                                Adt1::C0(_, _, _, h1115, _, _, _, _, _, _, _) => {
                                    h1115
                                }
                            };
                            let a1118 = 0u64;
                            nat_rem(a1116, a1117, a1118)
                        };
                        nat_mul(a1119, a1120)?
                    };
                    nat_add(a1121, a1122)?
                };
                nat_mul(a1123, a1124)?
            };
            nat_add(a1125, a1126)?
        };
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, {
            let a1127 = 3u64;
            let a1128 = v12;
            nat_add(a1127, a1128)?
        }, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
    } else {
        let a1130 = v4;
        let a1131 = match v3.clone() {
            Adt1::C0(_, _, h1129, _, _, _, _, _, _, _, _) => {
                h1129
            }
        };
        if nat_lt(a1130, a1131) {
            let v13: u64 = {
                let a1136 = {
                    let a1132 = v6;
                    let a1133 = 1u64;
                    nat_sub(a1132, a1133)
                };
                let a1137 = f60(match v1.clone() {
                    Adt1::C0(h1134, _, _, _, _, _, _, _, _, _, _) => {
                        h1134
                    }
                }, match v3.clone() {
                    Adt1::C0(h1135, _, _, _, _, _, _, _, _, _, _) => {
                        h1135
                    }
                });
                let a1138 = 0u64;
                nat_quot(a1136, a1137, a1138)
            };
            let v14: u64 = f47({
                let a1140 = v13;
                let a1141 = f23(match v3.clone() {
                    Adt1::C0(h1139, _, _, _, _, _, _, _, _, _, _) => {
                        h1139
                    }
                });
                nat_mul(a1140, a1141)?
            }, v8, v10, v11, match v1.clone() {
                Adt1::C0(_, _, _, _, _, h1142, _, _, _, _, _) => {
                    h1142
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, h1143, _, _, _, _) => {
                    h1143
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1144, _, _, _) => {
                    h1144
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1145, _, _) => {
                    h1145
                }
            })?;
            let v15: u64 = f47(v13, v4, v10, v11, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h1146, _, _, _, _, _) => {
                    h1146
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h1147, _, _, _, _) => {
                    h1147
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1148, _, _, _) => {
                    h1148
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1149, _, _) => {
                    h1149
                }
            })?;
            if if f2(match v3.clone() {
                Adt1::C0(h1150, _, _, _, _, _, _, _, _, _, _) => {
                    h1150
                }
            }) {
                true
            } else {
                let a1152 = match v3.clone() {
                    Adt1::C0(h1151, _, _, _, _, _, _, _, _, _, _) => {
                        h1151
                    }
                };
                let a1153 = 26u64;
                nat_eq(a1152, a1153)
            } {
                Ok(Adt3::C6(v14, v15))
            } else {
                Ok(f61(match v1 {
                    Adt1::C0(h1154, _, _, _, _, _, _, _, _, _, _) => {
                        h1154
                    }
                }, match v3 {
                    Adt1::C0(h1155, _, _, _, _, _, _, _, _, _, _) => {
                        h1155
                    }
                }, {
                    let a1156 = v6;
                    let a1157 = 1u64;
                    nat_sub(a1156, a1157)
                }, v14, v15))
            }
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f50(v0: u64, v1: u64, v2: u64) -> Adt3 {
    Adt3::C3(2u64, 0u64, 0u64, 0u64, 1u64, v0, v1, v2, 0u64, 2u64, 4u64, 4u64)
}

pub fn f51(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt3 {
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

pub fn f52(v0: u64, v1: u64) -> Adt3 {
    let a1158 = v0;
    let a1159 = 8u64;
    if nat_eq(a1158, a1159) {
        Adt3::C3(1u64, 6u64, 0u64, 0u64, 1u64, 0u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
    } else {
        let a1160 = v0;
        let a1161 = 9u64;
        if nat_eq(a1160, a1161) {
            Adt3::C3(1u64, 2u64, 0u64, 0u64, 2u64, v1, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
        } else {
            let a1162 = v0;
            let a1163 = 10u64;
            if nat_eq(a1162, a1163) {
                Adt3::C3(5u64, 16u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
            } else {
                let a1164 = v0;
                let a1165 = 11u64;
                if nat_eq(a1164, a1165) {
                    Adt3::C3(1u64, 0u64, 0u64, 0u64, 2u64, 1065353216u64, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
                } else {
                    let a1166 = v0;
                    let a1167 = 12u64;
                    if nat_eq(a1166, a1167) {
                        Adt3::C3(1u64, 3u64, 0u64, 0u64, 1u64, 0u64, 1u64, 2u64, 0u64, 0u64, 4u64, 4u64)
                    } else {
                        let a1168 = v0;
                        let a1169 = 13u64;
                        if nat_eq(a1168, a1169) {
                            Adt3::C3(1u64, 0u64, 0u64, 0u64, 1u64, 1u64, 2u64, 1065353216u64, 0u64, 1u64, 4u64, 4u64)
                        } else {
                            let a1170 = v0;
                            let a1171 = 14u64;
                            if nat_eq(a1170, a1171) {
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

pub fn f53(v0: u64) -> Adt4 {
    let a1172 = v0;
    let a1173 = 0u64;
    if nat_eq(a1172, a1173) {
        Adt4::C1(Adt5::C0(0u64, 1u64, 4u64))
    } else {
        let a1174 = v0;
        let a1175 = 1u64;
        if nat_eq(a1174, a1175) {
            Adt4::C1(Adt5::C0(1u64, 1u64, 2u64))
        } else {
            let a1176 = v0;
            let a1177 = 2u64;
            if nat_eq(a1176, a1177) {
                Adt4::C1(Adt5::C0(2u64, 32u64, 18u64))
            } else {
                let a1178 = v0;
                let a1179 = 3u64;
                if nat_eq(a1178, a1179) {
                    Adt4::C1(Adt5::C0(3u64, 32u64, 20u64))
                } else {
                    let a1180 = v0;
                    let a1181 = 6u64;
                    if nat_eq(a1180, a1181) {
                        Adt4::C1(Adt5::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1182 = v0;
                        let a1183 = 7u64;
                        if nat_eq(a1182, a1183) {
                            Adt4::C1(Adt5::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1184 = v0;
                            let a1185 = 8u64;
                            if nat_eq(a1184, a1185) {
                                Adt4::C1(Adt5::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1186 = v0;
                                let a1187 = 9u64;
                                if nat_eq(a1186, a1187) {
                                    Adt4::C1(Adt5::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1188 = v0;
                                    let a1189 = 10u64;
                                    if nat_eq(a1188, a1189) {
                                        Adt4::C1(Adt5::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1190 = v0;
                                        let a1191 = 11u64;
                                        if nat_eq(a1190, a1191) {
                                            Adt4::C1(Adt5::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1192 = v0;
                                            let a1193 = 12u64;
                                            if nat_eq(a1192, a1193) {
                                                Adt4::C1(Adt5::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1194 = v0;
                                                let a1195 = 13u64;
                                                if nat_eq(a1194, a1195) {
                                                    Adt4::C1(Adt5::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1196 = v0;
                                                    let a1197 = 14u64;
                                                    if nat_eq(a1196, a1197) {
                                                        Adt4::C1(Adt5::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1198 = v0;
                                                        let a1199 = 15u64;
                                                        if nat_eq(a1198, a1199) {
                                                            Adt4::C1(Adt5::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1200 = v0;
                                                            let a1201 = 16u64;
                                                            if nat_eq(a1200, a1201) {
                                                                Adt4::C1(Adt5::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1202 = v0;
                                                                let a1203 = 17u64;
                                                                if nat_eq(a1202, a1203) {
                                                                    Adt4::C1(Adt5::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1204 = v0;
                                                                    let a1205 = 18u64;
                                                                    if nat_eq(a1204, a1205) {
                                                                        Adt4::C1(Adt5::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1206 = v0;
                                                                        let a1207 = 19u64;
                                                                        if nat_eq(a1206, a1207) {
                                                                            Adt4::C1(Adt5::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1208 = v0;
                                                                            let a1209 = 20u64;
                                                                            if nat_eq(a1208, a1209) {
                                                                                Adt4::C1(Adt5::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1210 = v0;
                                                                                let a1211 = 21u64;
                                                                                if nat_eq(a1210, a1211) {
                                                                                    Adt4::C1(Adt5::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1212 = v0;
                                                                                    let a1213 = 22u64;
                                                                                    if nat_eq(a1212, a1213) {
                                                                                        Adt4::C1(Adt5::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1214 = v0;
                                                                                        let a1215 = 23u64;
                                                                                        if nat_eq(a1214, a1215) {
                                                                                            Adt4::C1(Adt5::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1216 = v0;
                                                                                            let a1217 = 24u64;
                                                                                            if nat_eq(a1216, a1217) {
                                                                                                Adt4::C1(Adt5::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1218 = v0;
                                                                                                let a1219 = 25u64;
                                                                                                if nat_eq(a1218, a1219) {
                                                                                                    Adt4::C1(Adt5::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1220 = v0;
                                                                                                    let a1221 = 26u64;
                                                                                                    if nat_eq(a1220, a1221) {
                                                                                                        Adt4::C1(Adt5::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1222 = v0;
                                                                                                        let a1223 = 27u64;
                                                                                                        if nat_eq(a1222, a1223) {
                                                                                                            Adt4::C1(Adt5::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1224 = v0;
                                                                                                            let a1225 = 28u64;
                                                                                                            if nat_eq(a1224, a1225) {
                                                                                                                Adt4::C1(Adt5::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1226 = v0;
                                                                                                                let a1227 = 29u64;
                                                                                                                if nat_eq(a1226, a1227) {
                                                                                                                    Adt4::C1(Adt5::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1228 = v0;
                                                                                                                    let a1229 = 30u64;
                                                                                                                    if nat_eq(a1228, a1229) {
                                                                                                                        Adt4::C1(Adt5::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1230 = v0;
                                                                                                                        let a1231 = 34u64;
                                                                                                                        if nat_eq(a1230, a1231) {
                                                                                                                            Adt4::C1(Adt5::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1232 = v0;
                                                                                                                            let a1233 = 35u64;
                                                                                                                            if nat_eq(a1232, a1233) {
                                                                                                                                Adt4::C1(Adt5::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1234 = v0;
                                                                                                                                let a1235 = 39u64;
                                                                                                                                if nat_eq(a1234, a1235) {
                                                                                                                                    Adt4::C1(Adt5::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1236 = v0;
                                                                                                                                    let a1237 = 40u64;
                                                                                                                                    if nat_eq(a1236, a1237) {
                                                                                                                                        Adt4::C1(Adt5::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1238 = v0;
                                                                                                                                        let a1239 = 41u64;
                                                                                                                                        if nat_eq(a1238, a1239) {
                                                                                                                                            Adt4::C1(Adt5::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1240 = v0;
                                                                                                                                            let a1241 = 42u64;
                                                                                                                                            if nat_eq(a1240, a1241) {
                                                                                                                                                Adt4::C1(Adt5::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1242 = v0;
                                                                                                                                                let a1243 = 43u64;
                                                                                                                                                if nat_eq(a1242, a1243) {
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

pub fn f54(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f19(v0.clone(), v1.clone()) {
        let a1246 = match v0.clone() {
            Adt1::C0(h1244, _, _, _, _, _, _, _, _, _, _) => {
                h1244
            }
        };
        let a1247 = match v1.clone() {
            Adt1::C0(h1245, _, _, _, _, _, _, _, _, _, _) => {
                h1245
            }
        };
        nat_eq(a1246, a1247)
    } else {
        false
    } {
        let a1250 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1248) => {
                h1248
            }
        };
        let a1251 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1249) => {
                h1249
            }
        };
        nat_eq(a1250, a1251)
    } else {
        false
    } {
        let a1254 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1252, _, _, _, _, _) => {
                h1252
            }
        };
        let a1255 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1253, _, _, _, _, _) => {
                h1253
            }
        };
        nat_eq(a1254, a1255)
    } else {
        false
    } {
        let a1258 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1256, _, _, _, _) => {
                h1256
            }
        };
        let a1259 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1257, _, _, _, _) => {
                h1257
            }
        };
        nat_eq(a1258, a1259)
    } else {
        false
    } {
        let a1262 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1260, _, _, _) => {
                h1260
            }
        };
        let a1263 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1261, _, _, _) => {
                h1261
            }
        };
        nat_eq(a1262, a1263)
    } else {
        false
    } {
        let a1266 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1264, _, _) => {
                h1264
            }
        };
        let a1267 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1265, _, _) => {
                h1265
            }
        };
        nat_eq(a1266, a1267)
    } else {
        false
    }
}

pub fn f55(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a1268 = v1;
    let a1269 = 1u64;
    if if if nat_le(a1268, a1269) {
        true
    } else {
        let a1270 = v0;
        let a1271 = v2;
        nat_le(a1270, a1271)
    } {
        let a1272 = v3;
        let a1273 = 1u64;
        if nat_le(a1272, a1273) {
            true
        } else {
            let a1280 = {
                let a1278 = v0;
                let a1279 = {
                    let a1276 = {
                        let a1274 = v1;
                        let a1275 = 1u64;
                        nat_sub(a1274, a1275)
                    };
                    let a1277 = v2;
                    nat_mul(a1276, a1277)?
                };
                nat_add(a1278, a1279)?
            };
            let a1281 = v4;
            nat_le(a1280, a1281)
        }
    } else {
        false
    } {
        let a1282 = v5;
        let a1283 = 1u64;
        if nat_le(a1282, a1283) {
            Ok(true)
        } else {
            let a1296 = {
                let a1294 = {
                    let a1288 = v0;
                    let a1289 = {
                        let a1286 = {
                            let a1284 = v1;
                            let a1285 = 1u64;
                            nat_sub(a1284, a1285)
                        };
                        let a1287 = v2;
                        nat_mul(a1286, a1287)?
                    };
                    nat_add(a1288, a1289)?
                };
                let a1295 = {
                    let a1292 = {
                        let a1290 = v3;
                        let a1291 = 1u64;
                        nat_sub(a1290, a1291)
                    };
                    let a1293 = v4;
                    nat_mul(a1292, a1293)?
                };
                nat_add(a1294, a1295)?
            };
            let a1297 = v6;
            Ok(nat_le(a1296, a1297))
        }
    } else {
        Ok(false)
    }
}

pub fn f56(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1298 = v0;
    let a1299 = 0u64;
    if if if nat_eq(a1298, a1299) {
        true
    } else {
        let a1300 = v1;
        let a1301 = 0u64;
        nat_eq(a1300, a1301)
    } {
        true
    } else {
        let a1302 = v2;
        let a1303 = 0u64;
        nat_eq(a1302, a1303)
    } {
        true
    } else {
        let a1304 = v3;
        let a1305 = 0u64;
        nat_eq(a1304, a1305)
    }
}

pub fn f57(v0: u64, v1: u64, v2: u64) -> bool {
    let a1306 = v0;
    let a1307 = 0u64;
    if if if nat_eq(a1306, a1307) {
        let a1308 = v1;
        let a1309 = 0u64;
        nat_eq(a1308, a1309)
    } else {
        false
    } {
        let a1310 = v2;
        let a1311 = 0u64;
        nat_eq(a1310, a1311)
    } else {
        false
    } {
        true
    } else {
        let a1312 = v0;
        let a1313 = 1u64;
        if if nat_eq(a1312, a1313) {
            true
        } else {
            let a1314 = v0;
            let a1315 = 30u64;
            nat_eq(a1314, a1315)
        } {
            let a1316 = v1;
            let a1317 = v0;
            if if nat_eq(a1316, a1317) {
                let a1318 = v2;
                let a1319 = v0;
                nat_eq(a1318, a1319)
            } else {
                false
            } {
                true
            } else {
                let a1320 = v1;
                let a1321 = 0u64;
                if nat_eq(a1320, a1321) {
                    let a1322 = v2;
                    let a1323 = v0;
                    if nat_eq(a1322, a1323) {
                        true
                    } else {
                        let a1324 = v2;
                        let a1325 = 0u64;
                        nat_eq(a1324, a1325)
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

pub fn f58(v0: u64, v1: u64, v2: u64) -> bool {
    let a1326 = v0;
    let a1327 = 1u64;
    if nat_le(a1326, a1327) {
        true
    } else {
        let a1331 = {
            let a1328 = v1;
            let a1329 = v2;
            let a1330 = 1u64;
            nat_rem(a1328, a1329, a1330)
        };
        let a1332 = 0u64;
        nat_eq(a1331, a1332)
    }
}

pub fn f59(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f58(v0, v3, v6) {
        f58(v1, v4, v6)
    } else {
        false
    } {
        f58(v2, v5, v6)
    } else {
        false
    }
}

pub fn f60(v0: u64, v1: u64) -> u64 {
    let a1333 = v0;
    let a1334 = 1u64;
    if if nat_eq(a1333, a1334) {
        let a1335 = v1;
        let a1336 = 43u64;
        nat_eq(a1335, a1336)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f61(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt3 {
    let a1337 = f60(v0, v1);
    let a1338 = 2u64;
    if nat_eq(a1337, a1338) {
        let a1342 = {
            let a1339 = v2;
            let a1340 = 2u64;
            let a1341 = 0u64;
            nat_rem(a1339, a1340, a1341)
        };
        let a1343 = 0u64;
        if nat_eq(a1342, a1343) {
            Adt3::C3(0u64, 0u64, v0, 0u64, 0u64, v3, 2u64, 0u64, 1u64, 0u64, f40(v0), 8u64)
        } else {
            Adt3::C3(0u64, 0u64, 0u64, v1, 1u64, 0u64, 2u64, 0u64, 3u64, v4, 8u64, f22(v1))
        }
    } else {
        Adt3::C3(0u64, 0u64, v0, v1, 0u64, v3, 2u64, 0u64, 3u64, v4, f40(v0), f22(v1))
    }
}
