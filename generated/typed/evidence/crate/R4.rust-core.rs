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
    C0(u64, u64, Option<u64>, u64, u64, u64, u64, Adt1, Adt1, Adt1, bool, u64),
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
    C8(u64, u64, u64),
    C9(u64, u64, u64),
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
                                    if f2(v5, match v6.clone() {
                                        Adt1::C0(h5, _, _, _, _, _, _, _, _, _, _) => {
                                            h5
                                        }
                                    }, match v7.clone() {
                                        Adt1::C0(h6, _, _, _, _, _, _, _, _, _, _) => {
                                            h6
                                        }
                                    }, match v4.clone() {
                                        Adt1::C0(h7, _, _, _, _, _, _, _, _, _, _) => {
                                            h7
                                        }
                                    }) {
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
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h21, _) => {
                    h21
                }
            } {
                f10(v26, v27)
            } else {
                let a23 = match v27.clone() {
                    Adt2::C0(_, _, _, h22, _, _, _, _, _, _, _, _) => {
                        h22
                    }
                };
                let a24 = 33u64;
                if nat_eq(a23, a24) {
                    f11(v26, v27)
                } else {
                    let a26 = match v27.clone() {
                        Adt2::C0(_, _, _, h25, _, _, _, _, _, _, _, _) => {
                            h25
                        }
                    };
                    let a27 = 66u64;
                    if if nat_eq(a26, a27) {
                        true
                    } else {
                        let a29 = match v27.clone() {
                            Adt2::C0(_, _, _, h28, _, _, _, _, _, _, _, _) => {
                                h28
                            }
                        };
                        let a30 = 67u64;
                        nat_eq(a29, a30)
                    } {
                        f12(v26, v27)
                    } else {
                        let a32 = match v27.clone() {
                            Adt2::C0(_, _, _, h31, _, _, _, _, _, _, _, _) => {
                                h31
                            }
                        };
                        let a33 = 41u64;
                        if nat_eq(a32, a33) {
                            f13(v26, v27, v28)
                        } else {
                            let a35 = 98u64;
                            let a36 = match v27.clone() {
                                Adt2::C0(_, _, _, h34, _, _, _, _, _, _, _, _) => {
                                    h34
                                }
                            };
                            if nat_le(a35, a36) {
                                if f14(match v27.clone() {
                                    Adt2::C0(_, _, _, _, h37, _, _, _, _, _, _, _) => {
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

pub fn f2(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    if if f18(v1) {
        true
    } else {
        f18(v3)
    } {
        true
    } else {
        if if f19(v1, v2, v3) {
            true
        } else {
            let a46 = v1;
            let a47 = 43u64;
            if if nat_eq(a46, a47) {
                let a48 = v0;
                let a49 = 0u64;
                nat_eq(a48, a49)
            } else {
                false
            } {
                let a50 = v2;
                let a51 = 0u64;
                nat_eq(a50, a51)
            } else {
                false
            }
        } {
            false
        } else {
            let a52 = v1;
            let a53 = v3;
            if if nat_eq(a52, a53) {
                f20(v0, v1, v2)
            } else {
                false
            } {
                true
            } else {
                let a54 = v0;
                let a55 = 0u64;
                if if nat_eq(a54, a55) {
                    let a56 = f21(v1);
                    let a57 = 2u64;
                    nat_eq(a56, a57)
                } else {
                    false
                } {
                    f22(v3)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f3(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    let a60 = match v1.clone() {
        Adt1::C0(_, h58, _, _, _, _, _, _, _, _, _) => {
            h58
        }
    };
    let a61 = match v2.clone() {
        Adt1::C0(_, h59, _, _, _, _, _, _, _, _, _) => {
            h59
        }
    };
    if if if if if if if if if if if if if if nat_eq(a60, a61) {
        true
    } else {
        let a64 = match v1.clone() {
            Adt1::C0(h62, _, _, _, _, _, _, _, _, _, _) => {
                h62
            }
        };
        let a65 = match v3.clone() {
            Adt1::C0(h63, _, _, _, _, _, _, _, _, _, _) => {
                h63
            }
        };
        nat_eq(a64, a65)
    } {
        f23(v1.clone(), v2.clone())
    } else {
        false
    } {
        f24(v1.clone(), v3.clone())
    } else {
        false
    } {
        f25(v0, match v1.clone() {
            Adt1::C0(h66, _, _, _, _, _, _, _, _, _, _) => {
                h66
            }
        }, v2.clone(), match v3.clone() {
            Adt1::C0(h67, _, _, _, _, _, _, _, _, _, _) => {
                h67
            }
        })
    } else {
        false
    } {
        f26(match v1.clone() {
            Adt1::C0(h68, _, _, _, _, _, _, _, _, _, _) => {
                h68
            }
        }, match v3.clone() {
            Adt1::C0(h69, _, _, _, _, _, _, _, _, _, _) => {
                h69
            }
        })
    } else {
        false
    } {
        let a71 = 0u64;
        let a72 = f27(match v3.clone() {
            Adt1::C0(h70, _, _, _, _, _, _, _, _, _, _) => {
                h70
            }
        });
        nat_lt(a71, a72)
    } else {
        false
    } {
        let a75 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h73, _, _, _, _, _) => {
                h73
            }
        };
        let a76 = f27(match v1.clone() {
            Adt1::C0(h74, _, _, _, _, _, _, _, _, _, _) => {
                h74
            }
        });
        nat_eq(a75, a76)
    } else {
        false
    } {
        let a79 = match v3.clone() {
            Adt1::C0(_, _, _, _, _, h77, _, _, _, _, _) => {
                h77
            }
        };
        let a80 = f27(match v3.clone() {
            Adt1::C0(h78, _, _, _, _, _, _, _, _, _, _) => {
                h78
            }
        });
        nat_eq(a79, a80)
    } else {
        false
    } {
        let a86 = {
            let a83 = match v1.clone() {
                Adt1::C0(_, h81, _, _, _, _, _, _, _, _, _) => {
                    h81
                }
            };
            let a84 = f28(match v1.clone() {
                Adt1::C0(h82, _, _, _, _, _, _, _, _, _, _) => {
                    h82
                }
            });
            let a85 = 1u64;
            nat_rem(a83, a84, a85)
        };
        let a87 = 0u64;
        nat_eq(a86, a87)
    } else {
        false
    } {
        let a93 = {
            let a90 = match v3.clone() {
                Adt1::C0(_, h88, _, _, _, _, _, _, _, _, _) => {
                    h88
                }
            };
            let a91 = f28(match v3.clone() {
                Adt1::C0(h89, _, _, _, _, _, _, _, _, _, _) => {
                    h89
                }
            });
            let a92 = 1u64;
            nat_rem(a90, a91, a92)
        };
        let a94 = 0u64;
        nat_eq(a93, a94)
    } else {
        false
    } {
        f29(v1.clone(), v3.clone())?
    } else {
        false
    } {
        f29(v2.clone(), v3.clone())?
    } else {
        false
    } {
        f30({
            let a101 = {
                let a97 = match v3.clone() {
                    Adt1::C0(_, h95, _, _, _, _, _, _, _, _, _) => {
                        h95
                    }
                };
                let a98 = f28(match v3.clone() {
                    Adt1::C0(h96, _, _, _, _, _, _, _, _, _, _) => {
                        h96
                    }
                });
                let a99 = 0u64;
                nat_quot(a97, a98, a99)
            };
            let a102 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h100, _, _, _, _, _) => {
                    h100
                }
            };
            nat_mul(a101, a102)?
        }, match v3.clone() {
            Adt1::C0(_, _, h103, _, _, _, _, _, _, _, _) => {
                h103
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, h104, _, _, _, _, _, _, _) => {
                h104
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, h105, _, _, _, _, _, _) => {
                h105
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, _, _, h106, _, _, _, _) => {
                h106
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h107, _, _, _) => {
                h107
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h108, _, _) => {
                h108
            }
        })?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0(if f31(v1.clone()) {
            0u64
        } else {
            let a114 = match v1.clone() {
                Adt1::C0(_, _, h109, _, _, _, _, _, _, _, _) => {
                    h109
                }
            };
            let a115 = {
                let a112 = match v1.clone() {
                    Adt1::C0(_, _, _, h110, _, _, _, _, _, _, _) => {
                        h110
                    }
                };
                let a113 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h111, _, _, _, _, _, _) => {
                        h111
                    }
                };
                nat_mul(a112, a113)?
            };
            nat_mul(a114, a115)?
        }, 3u64, f32(match v3.clone() {
            Adt1::C0(h116, _, _, _, _, _, _, _, _, _, _) => {
                h116
            }
        }), v0, 0u64, 0u64, 0u64, v1.clone(), v2, v3, true, match v1 {
            Adt1::C0(_, h117, _, _, _, _, _, _, _, _, _) => {
                h117
            }
        })))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f4(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if f24(v1.clone(), v3.clone()) {
        f33(v0, v1.clone(), v2.clone(), v3.clone())?
    } else {
        false
    } {
        if f31(v1.clone()) {
            true
        } else {
            let a120 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, h118, _, _, _, _, _) => {
                    h118
                }
            };
            let a121 = f34(match v1.clone() {
                Adt1::C0(h119, _, _, _, _, _, _, _, _, _, _) => {
                    h119
                }
            });
            if if if if if if if nat_eq(a120, a121) {
                let a124 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h122, _, _, _, _, _) => {
                        h122
                    }
                };
                let a125 = f34(match v3.clone() {
                    Adt1::C0(h123, _, _, _, _, _, _, _, _, _, _) => {
                        h123
                    }
                });
                nat_eq(a124, a125)
            } else {
                false
            } {
                f35(v1.clone())
            } else {
                false
            } {
                f35(v2.clone())
            } else {
                false
            } {
                f35(v3.clone())
            } else {
                false
            } {
                f36(v1.clone(), v2.clone())?
            } else {
                false
            } {
                f29(v1.clone(), v3.clone())?
            } else {
                false
            } {
                f29(v2.clone(), v3.clone())?
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v4: u64 = if if f31(v1.clone()) {
            true
        } else {
            f37(v3.clone())?
        } {
            0u64
        } else {
            let a128 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h126, _) => {
                    h126
                }
            };
            let a129 = f34(match v3.clone() {
                Adt1::C0(h127, _, _, _, _, _, _, _, _, _, _) => {
                    h127
                }
            });
            let a130 = 0u64;
            nat_quot(a128, a129, a130)
        };
        Ok(Adt3::C2(Adt2::C0(f38(f39(v1.clone())?, v4, f40(v0, match v1.clone() {
            Adt1::C0(h131, _, _, _, _, _, _, _, _, _, _) => {
                h131
            }
        }, match v2.clone() {
            Adt1::C0(h132, _, _, _, _, _, _, _, _, _, _) => {
                h132
            }
        }, match v3.clone() {
            Adt1::C0(h133, _, _, _, _, _, _, _, _, _, _) => {
                h133
            }
        })?)?, {
            let a134 = 3u64;
            let a135 = v4;
            nat_add(a134, a135)?
        }, None::<u64>, v0, 0u64, 0u64, 0u64, v1, v2, v3, false, 0u64)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f5(v0: Adt1, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if if if if if if if if if if if if if f41(v2, v3.clone()) {
        let a137 = match v0.clone() {
            Adt1::C0(h136, _, _, _, _, _, _, _, _, _, _) => {
                h136
            }
        };
        let a138 = 0u64;
        if nat_eq(a137, a138) {
            true
        } else {
            let a140 = match v0.clone() {
                Adt1::C0(h139, _, _, _, _, _, _, _, _, _, _) => {
                    h139
                }
            };
            let a141 = 1u64;
            nat_eq(a140, a141)
        }
    } else {
        false
    } {
        let a143 = 0u64;
        let a144 = f27(match v3.clone() {
            Adt1::C0(h142, _, _, _, _, _, _, _, _, _, _) => {
                h142
            }
        });
        nat_lt(a143, a144)
    } else {
        false
    } {
        let a146 = 0u64;
        let a147 = f42(match v1.clone() {
            Adt1::C0(h145, _, _, _, _, _, _, _, _, _, _) => {
                h145
            }
        });
        nat_lt(a146, a147)
    } else {
        false
    } {
        let a150 = match v0.clone() {
            Adt1::C0(_, h148, _, _, _, _, _, _, _, _, _) => {
                h148
            }
        };
        let a151 = match v3.clone() {
            Adt1::C0(_, h149, _, _, _, _, _, _, _, _, _) => {
                h149
            }
        };
        if nat_eq(a150, a151) {
            let a157 = {
                let a154 = match v0.clone() {
                    Adt1::C0(_, h152, _, _, _, _, _, _, _, _, _) => {
                        h152
                    }
                };
                let a155 = f28(match v3.clone() {
                    Adt1::C0(h153, _, _, _, _, _, _, _, _, _, _) => {
                        h153
                    }
                });
                let a156 = 1u64;
                nat_rem(a154, a155, a156)
            };
            let a158 = 0u64;
            nat_eq(a157, a158)
        } else {
            false
        }
    } else {
        false
    } {
        let a161 = match v0.clone() {
            Adt1::C0(_, _, _, h159, _, _, _, _, _, _, _) => {
                h159
            }
        };
        let a162 = match v3.clone() {
            Adt1::C0(_, _, _, h160, _, _, _, _, _, _, _) => {
                h160
            }
        };
        nat_eq(a161, a162)
    } else {
        false
    } {
        let a165 = match v0.clone() {
            Adt1::C0(_, _, _, _, h163, _, _, _, _, _, _) => {
                h163
            }
        };
        let a166 = match v3.clone() {
            Adt1::C0(_, _, _, _, h164, _, _, _, _, _, _) => {
                h164
            }
        };
        nat_eq(a165, a166)
    } else {
        false
    } {
        let a169 = match v0.clone() {
            Adt1::C0(_, _, h167, _, _, _, _, _, _, _, _) => {
                h167
            }
        };
        let a170 = match v1.clone() {
            Adt1::C0(_, h168, _, _, _, _, _, _, _, _, _) => {
                h168
            }
        };
        nat_eq(a169, a170)
    } else {
        false
    } {
        let a172 = match v1.clone() {
            Adt1::C0(_, _, _, _, h171, _, _, _, _, _, _) => {
                h171
            }
        };
        let a173 = 1u64;
        nat_eq(a172, a173)
    } else {
        false
    } {
        let a175 = 0u64;
        let a176 = match v1.clone() {
            Adt1::C0(_, _, h174, _, _, _, _, _, _, _, _) => {
                h174
            }
        };
        nat_lt(a175, a176)
    } else {
        false
    } {
        let a178 = 0u64;
        let a179 = match v1.clone() {
            Adt1::C0(_, _, _, h177, _, _, _, _, _, _, _) => {
                h177
            }
        };
        nat_lt(a178, a179)
    } else {
        false
    } {
        let a185 = {
            let a182 = match v0.clone() {
                Adt1::C0(_, _, _, h180, _, _, _, _, _, _, _) => {
                    h180
                }
            };
            let a183 = match v1.clone() {
                Adt1::C0(_, _, h181, _, _, _, _, _, _, _, _) => {
                    h181
                }
            };
            let a184 = 0u64;
            nat_rem(a182, a183, a184)
        };
        let a186 = 0u64;
        nat_eq(a185, a186)
    } else {
        false
    } {
        let a192 = {
            let a189 = match v0.clone() {
                Adt1::C0(_, _, _, _, h187, _, _, _, _, _, _) => {
                    h187
                }
            };
            let a190 = match v1.clone() {
                Adt1::C0(_, _, _, h188, _, _, _, _, _, _, _) => {
                    h188
                }
            };
            let a191 = 0u64;
            nat_rem(a189, a190, a191)
        };
        let a193 = 0u64;
        nat_eq(a192, a193)
    } else {
        false
    } {
        f43(v0.clone())
    } else {
        false
    } {
        let a195 = match v3.clone() {
            Adt1::C0(_, h194, _, _, _, _, _, _, _, _, _) => {
                h194
            }
        };
        let a196 = 1u64;
        if nat_eq(a195, a196) {
            true
        } else {
            let a199 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h197, _, _, _, _, _) => {
                    h197
                }
            };
            let a200 = f27(match v3.clone() {
                Adt1::C0(h198, _, _, _, _, _, _, _, _, _, _) => {
                    h198
                }
            });
            nat_eq(a199, a200)
        }
    } else {
        false
    } {
        let v4: u64 = f39(v1.clone())?;
        let v5: u64 = {
            let a206 = match v0.clone() {
                Adt1::C0(_, _, h201, _, _, _, _, _, _, _, _) => {
                    h201
                }
            };
            let a207 = {
                let a204 = match v0.clone() {
                    Adt1::C0(_, _, _, h202, _, _, _, _, _, _, _) => {
                        h202
                    }
                };
                let a205 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h203, _, _, _, _, _, _) => {
                        h203
                    }
                };
                nat_mul(a204, a205)?
            };
            nat_mul(a206, a207)?
        };
        let v6: u64 = if f31(v0.clone()) {
            0u64
        } else {
            let a215 = {
                let a208 = 2u64;
                let a209 = v4;
                nat_mul(a208, a209)?
            };
            let a216 = {
                let a213 = v5;
                let a214 = {
                    let a211 = 1u64;
                    let a212 = f44(v0.clone(), match v3.clone() {
                        Adt1::C0(h210, _, _, _, _, _, _, _, _, _, _) => {
                            h210
                        }
                    })?;
                    nat_add(a211, a212)?
                };
                nat_mul(a213, a214)?
            };
            nat_add(a215, a216)?
        };
        Ok(Adt3::C2(Adt2::C0(v6, if f31(v0.clone()) {
            3u64
        } else {
            let a217 = 3u64;
            let a218 = v4;
            nat_add(a217, a218)?
        }, f32(match v3.clone() {
            Adt1::C0(h219, _, _, _, _, _, _, _, _, _, _) => {
                h219
            }
        }), 41u64, 0u64, 0u64, 0u64, v0, v1, v3.clone(), false, f45(match v3 {
            Adt1::C0(h220, _, _, _, _, _, _, _, _, _, _) => {
                h220
            }
        }))))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f6(v0: Adt1, v1: Adt1, v2: Adt1) -> R<Adt3> {
    let v3: u64 = f46(v0.clone(), v2.clone());
    let v4: u64 = f39(v0.clone())?;
    if if if if if f41(v1.clone(), v2.clone()) {
        let a221 = v4;
        let a222 = f39(v2.clone())?;
        nat_eq(a221, a222)
    } else {
        false
    } {
        f47(v0.clone())
    } else {
        false
    } {
        f47(v2.clone())
    } else {
        false
    } {
        if f18(match v0.clone() {
            Adt1::C0(h223, _, _, _, _, _, _, _, _, _, _) => {
                h223
            }
        }) {
            true
        } else {
            f18(match v2.clone() {
                Adt1::C0(h224, _, _, _, _, _, _, _, _, _, _) => {
                    h224
                }
            })
        }
    } else {
        false
    } {
        let v5: u64 = {
            let a225 = 0u64;
            let a226 = v3;
            if nat_lt(a225, a226) {
                let a227 = v4;
                let a228 = v3;
                let a229 = 0u64;
                nat_quot(a227, a228, a229)
            } else {
                let a237 = {
                    let a231 = v4;
                    let a232 = match v0.clone() {
                        Adt1::C0(_, h230, _, _, _, _, _, _, _, _, _) => {
                            h230
                        }
                    };
                    let a233 = 0u64;
                    nat_quot(a231, a232, a233)
                };
                let a238 = {
                    let a234 = v4;
                    let a235 = f48(v2.clone());
                    let a236 = 0u64;
                    nat_quot(a234, a235, a236)
                };
                nat_add(a237, a238)?
            }
        };
        Ok(Adt3::C2(Adt2::C0(v5, 3u64, f32(match v2.clone() {
            Adt1::C0(h239, _, _, _, _, _, _, _, _, _, _) => {
                h239
            }
        }), 33u64, 0u64, 0u64, 0u64, v0, v1, v2, false, {
            let a240 = 0u64;
            let a241 = v3;
            if nat_lt(a240, a241) {
                v3
            } else {
                v4
            }
        })))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f7(v0: u64, v1: u64, v2: Adt1, v3: Adt1) -> R<Adt3> {
    let a243 = match v2.clone() {
        Adt1::C0(h242, _, _, _, _, _, _, _, _, _, _) => {
            h242
        }
    };
    let a244 = 0u64;
    if if if if if if if if if if if nat_eq(a243, a244) {
        let a246 = match v3.clone() {
            Adt1::C0(h245, _, _, _, _, _, _, _, _, _, _) => {
                h245
            }
        };
        let a247 = 26u64;
        nat_eq(a246, a247)
    } else {
        false
    } {
        let a249 = match v2.clone() {
            Adt1::C0(_, h248, _, _, _, _, _, _, _, _, _) => {
                h248
            }
        };
        let a250 = 2147483647u64;
        nat_le(a249, a250)
    } else {
        false
    } {
        let a253 = match v3.clone() {
            Adt1::C0(_, h251, _, _, _, _, _, _, _, _, _) => {
                h251
            }
        };
        let a254 = match v2.clone() {
            Adt1::C0(_, h252, _, _, _, _, _, _, _, _, _) => {
                h252
            }
        };
        nat_le(a253, a254)
    } else {
        false
    } {
        let a257 = match v2.clone() {
            Adt1::C0(_, _, h255, _, _, _, _, _, _, _, _) => {
                h255
            }
        };
        let a258 = match v3.clone() {
            Adt1::C0(_, _, h256, _, _, _, _, _, _, _, _) => {
                h256
            }
        };
        nat_eq(a257, a258)
    } else {
        false
    } {
        let a261 = match v2.clone() {
            Adt1::C0(_, _, _, h259, _, _, _, _, _, _, _) => {
                h259
            }
        };
        let a262 = match v3.clone() {
            Adt1::C0(_, _, _, h260, _, _, _, _, _, _, _) => {
                h260
            }
        };
        nat_eq(a261, a262)
    } else {
        false
    } {
        let a265 = match v2.clone() {
            Adt1::C0(_, _, _, _, h263, _, _, _, _, _, _) => {
                h263
            }
        };
        let a266 = match v3.clone() {
            Adt1::C0(_, _, _, _, h264, _, _, _, _, _, _) => {
                h264
            }
        };
        nat_eq(a265, a266)
    } else {
        false
    } {
        let a267 = v0;
        let a268 = 67u64;
        if nat_eq(a267, a268) {
            true
        } else {
            let a269 = v1;
            let a270 = 1u64;
            if nat_le(a269, a270) {
                let a273 = match v3.clone() {
                    Adt1::C0(_, h271, _, _, _, _, _, _, _, _, _) => {
                        h271
                    }
                };
                let a274 = match v2.clone() {
                    Adt1::C0(_, h272, _, _, _, _, _, _, _, _, _) => {
                        h272
                    }
                };
                nat_eq(a273, a274)
            } else {
                false
            }
        }
    } else {
        false
    } {
        f49(v2.clone())?
    } else {
        false
    } {
        f49(v3.clone())?
    } else {
        false
    } {
        f50(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a280 = match v3.clone() {
                Adt1::C0(_, _, h275, _, _, _, _, _, _, _, _) => {
                    h275
                }
            };
            let a281 = {
                let a278 = match v3.clone() {
                    Adt1::C0(_, _, _, h276, _, _, _, _, _, _, _) => {
                        h276
                    }
                };
                let a279 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h277, _, _, _, _, _, _) => {
                        h277
                    }
                };
                nat_mul(a278, a279)?
            };
            nat_mul(a280, a281)?
        }, 3u64, None::<u64>, v0, {
            let a282 = v0;
            let a283 = 67u64;
            if nat_eq(a282, a283) {
                1u64
            } else {
                v1
            }
        }, 0u64, 0u64, v2.clone(), v2, v3, false, 0u64)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f8(v0: u64) -> bool {
    let a284 = v0;
    let a285 = 6u64;
    nat_le(a284, a285)
}

pub fn f9(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1) -> R<Adt3> {
    let a286 = v2;
    let a287 = 1u64;
    if if if if if if if if if if if if if if if nat_le(a286, a287) {
        let a289 = 0u64;
        let a290 = f21(match v5.clone() {
            Adt1::C0(h288, _, _, _, _, _, _, _, _, _, _) => {
                h288
            }
        });
        nat_lt(a289, a290)
    } else {
        false
    } {
        let a293 = match v5.clone() {
            Adt1::C0(h291, _, _, _, _, _, _, _, _, _, _) => {
                h291
            }
        };
        let a294 = match v6.clone() {
            Adt1::C0(h292, _, _, _, _, _, _, _, _, _, _) => {
                h292
            }
        };
        nat_eq(a293, a294)
    } else {
        false
    } {
        let a297 = match v5.clone() {
            Adt1::C0(h295, _, _, _, _, _, _, _, _, _, _) => {
                h295
            }
        };
        let a298 = match v7.clone() {
            Adt1::C0(h296, _, _, _, _, _, _, _, _, _, _) => {
                h296
            }
        };
        nat_eq(a297, a298)
    } else {
        false
    } {
        f24(v5.clone(), v6.clone())
    } else {
        false
    } {
        f51(v5.clone())?
    } else {
        false
    } {
        f51(v6.clone())?
    } else {
        false
    } {
        f51(v7.clone())?
    } else {
        false
    } {
        let a301 = match v5.clone() {
            Adt1::C0(_, _, h299, _, _, _, _, _, _, _, _) => {
                h299
            }
        };
        let a302 = match v7.clone() {
            Adt1::C0(_, _, h300, _, _, _, _, _, _, _, _) => {
                h300
            }
        };
        nat_eq(a301, a302)
    } else {
        false
    } {
        let a305 = match v5.clone() {
            Adt1::C0(_, _, _, h303, _, _, _, _, _, _, _) => {
                h303
            }
        };
        let a306 = match v7.clone() {
            Adt1::C0(_, _, _, h304, _, _, _, _, _, _, _) => {
                h304
            }
        };
        nat_eq(a305, a306)
    } else {
        false
    } {
        let a309 = match v5.clone() {
            Adt1::C0(_, _, _, _, h307, _, _, _, _, _, _) => {
                h307
            }
        };
        let a310 = match v7.clone() {
            Adt1::C0(_, _, _, _, h308, _, _, _, _, _, _) => {
                h308
            }
        };
        nat_eq(a309, a310)
    } else {
        false
    } {
        let a317 = match v7.clone() {
            Adt1::C0(_, h311, _, _, _, _, _, _, _, _, _) => {
                h311
            }
        };
        let a318 = if v1 {
            match v5.clone() {
                Adt1::C0(_, h312, _, _, _, _, _, _, _, _, _) => {
                    h312
                }
            }
        } else {
            let a314 = match v5.clone() {
                Adt1::C0(_, h313, _, _, _, _, _, _, _, _, _) => {
                    h313
                }
            };
            let a315 = 2u64;
            let a316 = 0u64;
            nat_quot(a314, a315, a316)
        };
        nat_eq(a317, a318)
    } else {
        false
    } {
        let a320 = match v7.clone() {
            Adt1::C0(_, h319, _, _, _, _, _, _, _, _, _) => {
                h319
            }
        };
        let a321 = 2147483647u64;
        nat_le(a320, a321)
    } else {
        false
    } {
        f29(v5.clone(), v7.clone())?
    } else {
        false
    } {
        f29(v6.clone(), v7.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0(if f14(v0) {
            let a324 = 1u64;
            let a325 = {
                let a322 = f52(v0);
                let a323 = f39(v7.clone())?;
                nat_mul(a322, a323)?
            };
            nat_add(a324, a325)?
        } else {
            let a326 = 4u64;
            let a327 = f39(v7.clone())?;
            nat_mul(a326, a327)?
        }, if f14(v0) {
            4u64
        } else {
            3u64
        }, None::<u64>, if v1 {
            98u64
        } else {
            let a328 = 99u64;
            let a329 = v2;
            nat_add(a328, a329)?
        }, v0, v3, v4, v5, v6, v7, false, 0u64)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f10(v0: u64, v1: Adt2) -> R<Adt3> {
    let a331 = v0;
    let a332 = match v1.clone() {
        Adt2::C0(h330, _, _, _, _, _, _, _, _, _, _, _) => {
            h330
        }
    };
    if nat_lt(a331, a332) {
        let v2: u64 = {
            let a336 = v0;
            let a337 = {
                let m335 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h334, _, _, _, _) => {
                        h334
                    }
                };
                match m335 {
                    Adt1::C0(_, h333, _, _, _, _, _, _, _, _, _) => {
                        h333
                    }
                }
            };
            nat_mul(a336, a337)?
        };
        Ok(Adt3::C7(f53(v2, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h338, _, _, _, _) => {
                h338
            }
        })?, f54(v2, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h339, _, _, _, _) => {
                h339
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h340, _, _, _) => {
                h340
            }
        })?, f53(v2, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h341, _, _) => {
                h341
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f11(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h342, _, _, _, _) => {
            h342
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h343, _, _) => {
            h343
        }
    };
    let a345 = v0;
    let a346 = match v1.clone() {
        Adt2::C0(h344, _, _, _, _, _, _, _, _, _, _, _) => {
            h344
        }
    };
    if nat_lt(a345, a346) {
        let v4: u64 = f46(v2.clone(), v3.clone());
        let a347 = 0u64;
        let a348 = v4;
        if nat_lt(a347, a348) {
            let v5: u64 = {
                let a349 = v0;
                let a350 = v4;
                nat_mul(a349, a350)?
            };
            Ok(Adt3::C6(f55(v5, v2)?, f55(v5, v3)?))
        } else {
            let v6: u64 = {
                let a353 = match v1 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h351) => {
                        h351
                    }
                };
                let a354 = match v2.clone() {
                    Adt1::C0(_, h352, _, _, _, _, _, _, _, _, _) => {
                        h352
                    }
                };
                let a355 = 0u64;
                nat_quot(a353, a354, a355)
            };
            let a356 = v0;
            let a357 = v6;
            if nat_lt(a356, a357) {
                let v7: u64 = {
                    let a359 = v0;
                    let a360 = match v2.clone() {
                        Adt1::C0(_, h358, _, _, _, _, _, _, _, _, _) => {
                            h358
                        }
                    };
                    nat_mul(a359, a360)?
                };
                Ok(Adt3::C8(f55(v7, v2.clone())?, v7, match v2 {
                    Adt1::C0(_, h361, _, _, _, _, _, _, _, _, _) => {
                        h361
                    }
                }))
            } else {
                let v8: u64 = f48(v3.clone());
                let v9: u64 = {
                    let a364 = {
                        let a362 = v0;
                        let a363 = v6;
                        nat_sub(a362, a363)
                    };
                    let a365 = v8;
                    nat_mul(a364, a365)?
                };
                Ok(Adt3::C9(v9, f55(v9, v3)?, v8))
            }
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f12(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h366, _, _, _, _) => {
            h366
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h367, _, _) => {
            h367
        }
    };
    let a369 = v0;
    let a370 = match v1.clone() {
        Adt2::C0(h368, _, _, _, _, _, _, _, _, _, _, _) => {
            h368
        }
    };
    if nat_lt(a369, a370) {
        Ok(Adt3::C5({
            let a372 = v0;
            let a373 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, _, h371, _, _, _, _) => {
                    h371
                }
            };
            nat_mul(a372, a373)?
        }, {
            let a375 = v0;
            let a376 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h374, _, _, _, _) => {
                    h374
                }
            };
            nat_mul(a375, a376)?
        }, match v2 {
            Adt1::C0(_, h377, _, _, _, _, _, _, _, _, _) => {
                h377
            }
        }, match v3 {
            Adt1::C0(_, h378, _, _, _, _, _, _, _, _, _) => {
                h378
            }
        }, {
            let a380 = match v1 {
                Adt2::C0(_, _, _, _, h379, _, _, _, _, _, _, _) => {
                    h379
                }
            };
            let a381 = 1u64;
            nat_eq(a380, a381)
        }))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f13(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let a383 = v0;
    let a384 = match v1.clone() {
        Adt2::C0(h382, _, _, _, _, _, _, _, _, _, _, _) => {
            h382
        }
    };
    if nat_lt(a383, a384) {
        let v3: u64 = {
            let a386 = 2u64;
            let a387 = f39(match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h385, _, _, _) => {
                    h385
                }
            })?;
            nat_mul(a386, a387)?
        };
        let a388 = v0;
        let a389 = v3;
        if nat_lt(a388, a389) {
            f56(v0, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h390, _, _, _) => {
                    h390
                }
            }, {
                let m393 = match v1 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h392, _, _) => {
                        h392
                    }
                };
                match m393 {
                    Adt1::C0(_, _, h391, _, _, _, _, _, _, _, _) => {
                        h391
                    }
                }
            }, v2)
        } else {
            f57({
                let a394 = v0;
                let a395 = v3;
                nat_sub(a394, a395)
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h396, _, _, _, _) => {
                    h396
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h397, _, _, _) => {
                    h397
                }
            }, match v1 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h398, _, _) => {
                    h398
                }
            }, v2)
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f14(v0: u64) -> bool {
    let a399 = v0;
    let a400 = 3u64;
    if nat_eq(a399, a400) {
        true
    } else {
        let a401 = v0;
        let a402 = 6u64;
        nat_eq(a401, a402)
    }
}

pub fn f15(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h403, _, _, _, _, _) => {
            h403
        }
    };
    let a405 = v0;
    let a406 = match v1.clone() {
        Adt2::C0(h404, _, _, _, _, _, _, _, _, _, _, _) => {
            h404
        }
    };
    if nat_lt(a405, a406) {
        let a407 = v0;
        let a408 = 0u64;
        if nat_eq(a407, a408) {
            Ok(Adt3::C3(1u64, 6u64, 0u64, 0u64, 2u64, v3, 2u64, 0u64, 0u64, 3u64, 4u64, 4u64))
        } else {
            let v4: u64 = {
                let a409 = v0;
                let a410 = 1u64;
                nat_sub(a409, a410)
            };
            let v5: u64 = {
                let a412 = v4;
                let a413 = f52(match v1.clone() {
                    Adt2::C0(_, _, _, _, h411, _, _, _, _, _, _, _) => {
                        h411
                    }
                });
                let a414 = 0u64;
                nat_rem(a412, a413, a414)
            };
            let v6: u64 = {
                let a419 = 4u64;
                let a420 = {
                    let a416 = v4;
                    let a417 = f52(match v1.clone() {
                        Adt2::C0(_, _, _, _, h415, _, _, _, _, _, _, _) => {
                            h415
                        }
                    });
                    let a418 = 0u64;
                    nat_quot(a416, a417, a418)
                };
                nat_mul(a419, a420)?
            };
            let a421 = v5;
            let a422 = 2u64;
            if nat_lt(a421, a422) {
                f16({
                    let a423 = v6;
                    let a424 = v5;
                    nat_add(a423, a424)?
                }, v1)
            } else {
                let a425 = v5;
                let a426 = 2u64;
                if nat_eq(a425, a426) {
                    Ok(f58(0u64, 2u64, v3))
                } else {
                    let a427 = v5;
                    let a428 = 3u64;
                    if nat_eq(a427, a428) {
                        Ok(f59(0u64, 2u64, v3, {
                            let a429 = v2;
                            let a430 = 2u64;
                            nat_eq(a429, a430)
                        }))
                    } else {
                        let a431 = v5;
                        let a432 = 4u64;
                        if nat_eq(a431, a432) {
                            Ok(f58(1u64, 1u64, 3u64))
                        } else {
                            let a433 = v5;
                            let a434 = 5u64;
                            if nat_eq(a433, a434) {
                                Ok(f59(1u64, 1u64, 3u64, {
                                    let a435 = v2;
                                    let a436 = 0u64;
                                    nat_eq(a435, a436)
                                }))
                            } else {
                                let a437 = v5;
                                let a438 = 6u64;
                                if nat_eq(a437, a438) {
                                    Ok(f58(1u64, 2u64, v3))
                                } else {
                                    let a439 = v5;
                                    let a440 = 7u64;
                                    if nat_eq(a439, a440) {
                                        Ok(f59(1u64, 2u64, v3, {
                                            let a441 = v2;
                                            let a442 = 2u64;
                                            nat_eq(a441, a442)
                                        }))
                                    } else {
                                        let a444 = match v1.clone() {
                                            Adt2::C0(_, _, _, _, h443, _, _, _, _, _, _, _) => {
                                                h443
                                            }
                                        };
                                        let a445 = 3u64;
                                        if nat_eq(a444, a445) {
                                            let a446 = v5;
                                            let a447 = 15u64;
                                            if nat_eq(a446, a447) {
                                                f16({
                                                    let a448 = v6;
                                                    let a449 = 3u64;
                                                    nat_add(a448, a449)?
                                                }, v1)
                                            } else {
                                                Ok(f60(v5, match v1 {
                                                    Adt2::C0(_, _, _, _, _, h450, _, _, _, _, _, _) => {
                                                        h450
                                                    }
                                                }))
                                            }
                                        } else {
                                            let a451 = v5;
                                            let a452 = 8u64;
                                            if nat_eq(a451, a452) {
                                                Ok(Adt3::C4(2u64, {
                                                    let m455 = match v1 {
                                                        Adt2::C0(_, _, _, _, _, _, _, _, _, h454, _, _) => {
                                                            h454
                                                        }
                                                    };
                                                    match m455 {
                                                        Adt1::C0(h453, _, _, _, _, _, _, _, _, _, _) => {
                                                            h453
                                                        }
                                                    }
                                                }, 0u64, 1u64, 0u64))
                                            } else {
                                                f16({
                                                    let a456 = v6;
                                                    let a457 = 3u64;
                                                    nat_add(a456, a457)?
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
    let a459 = v0;
    let a460 = match v1.clone() {
        Adt2::C0(h458, _, _, _, _, _, _, _, _, _, _, _) => {
            h458
        }
    };
    if nat_lt(a459, a460) {
        f61({
            let a461 = v0;
            let a462 = 4u64;
            let a463 = 0u64;
            nat_rem(a461, a462, a463)
        }, {
            let a464 = v0;
            let a465 = 4u64;
            let a466 = 0u64;
            nat_quot(a464, a465, a466)
        }, v1)
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f17(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h467, _, _, _, _) => {
            h467
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h468, _, _, _) => {
            h468
        }
    };
    let v5: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h469, _, _) => {
            h469
        }
    };
    let v6: u64 = {
        let a471 = match v1.clone() {
            Adt2::C0(_, h470, _, _, _, _, _, _, _, _, _, _) => {
                h470
            }
        };
        let a472 = 3u64;
        if nat_eq(a471, a472) {
            0u64
        } else {
            let a473 = f39(v3.clone())?;
            let a474 = 2u64;
            nat_mul(a473, a474)?
        }
    };
    let a476 = match v1.clone() {
        Adt2::C0(h475, _, _, _, _, _, _, _, _, _, _, _) => {
            h475
        }
    };
    let a477 = v0;
    if nat_le(a476, a477) {
        Ok(Adt3::C1)
    } else {
        let a478 = v0;
        let a479 = v6;
        if nat_lt(a478, a479) {
            let v7: u64 = {
                let a480 = v0;
                let a481 = 2u64;
                let a482 = 0u64;
                nat_quot(a480, a481, a482)
            };
            let v8: u64 = {
                let a487 = 3u64;
                let a488 = {
                    let a484 = f53(v7, v5.clone())?;
                    let a485 = f34(match v5 {
                        Adt1::C0(h483, _, _, _, _, _, _, _, _, _, _) => {
                            h483
                        }
                    });
                    let a486 = 0u64;
                    nat_quot(a484, a485, a486)
                };
                nat_add(a487, a488)?
            };
            let a492 = {
                let a489 = v0;
                let a490 = 2u64;
                let a491 = 0u64;
                nat_rem(a489, a490, a491)
            };
            let a493 = 0u64;
            if nat_eq(a492, a493) {
                Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, v8, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
            } else {
                let a494 = v2;
                let a495 = 0u64;
                if nat_eq(a494, a495) {
                    Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 2u64, 1u64, 2u64, 0u64, 0u64, v8, 8u64, 8u64))
                } else {
                    Ok(Adt3::C1)
                }
            }
        } else {
            let v9: u64 = {
                let a496 = v0;
                let a497 = v6;
                nat_sub(a496, a497)
            };
            if f62(match v3.clone() {
                Adt1::C0(h498, _, _, _, _, _, _, _, _, _, _) => {
                    h498
                }
            }) {
                f63(v9, v1)
            } else {
                let v10: u64 = f64(match v3.clone() {
                    Adt1::C0(h499, _, _, _, _, _, _, _, _, _, _) => {
                        h499
                    }
                }, match v4.clone() {
                    Adt1::C0(h500, _, _, _, _, _, _, _, _, _, _) => {
                        h500
                    }
                }, match v5.clone() {
                    Adt1::C0(h501, _, _, _, _, _, _, _, _, _, _) => {
                        h501
                    }
                })?;
                let v11: u64 = {
                    let a502 = v9;
                    let a503 = v10;
                    let a504 = 0u64;
                    nat_quot(a502, a503, a504)
                };
                let v12: u64 = {
                    let a505 = v9;
                    let a506 = v10;
                    let a507 = 0u64;
                    nat_rem(a505, a506, a507)
                };
                let v13: u64 = {
                    let a509 = match v3.clone() {
                        Adt1::C0(h508, _, _, _, _, _, _, _, _, _, _) => {
                            h508
                        }
                    };
                    let a510 = 0u64;
                    if nat_eq(a509, a510) {
                        0u64
                    } else {
                        1u64
                    }
                };
                let v14: u64 = {
                    let a512 = match v4.clone() {
                        Adt1::C0(h511, _, _, _, _, _, _, _, _, _, _) => {
                            h511
                        }
                    };
                    let a513 = 0u64;
                    if nat_eq(a512, a513) {
                        0u64
                    } else {
                        1u64
                    }
                };
                let a514 = v12;
                let a515 = v13;
                if nat_lt(a514, a515) {
                    Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                        Adt1::C0(h516, _, _, _, _, _, _, _, _, _, _) => {
                            h516
                        }
                    }, 0u64, 0u64, f53(v11, v3.clone())?, 2u64, 0u64, 0u64, 0u64, f34(match v3 {
                        Adt1::C0(h517, _, _, _, _, _, _, _, _, _, _) => {
                            h517
                        }
                    }), 4u64))
                } else {
                    let a520 = v12;
                    let a521 = {
                        let a518 = v13;
                        let a519 = v14;
                        nat_add(a518, a519)?
                    };
                    if nat_lt(a520, a521) {
                        Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                            Adt1::C0(h522, _, _, _, _, _, _, _, _, _, _) => {
                                h522
                            }
                        }, 0u64, 4u64, f54(v11, v3, v4.clone())?, 2u64, 0u64, 0u64, 1u64, f34(match v4 {
                            Adt1::C0(h523, _, _, _, _, _, _, _, _, _, _) => {
                                h523
                            }
                        }), 4u64))
                    } else {
                        let a526 = v12;
                        let a527 = {
                            let a524 = v13;
                            let a525 = v14;
                            nat_add(a524, a525)?
                        };
                        if nat_eq(a526, a527) {
                            Ok(Adt3::C3(1u64, match v1 {
                                Adt2::C0(_, _, _, h528, _, _, _, _, _, _, _, _) => {
                                    h528
                                }
                            }, 0u64, 0u64, {
                                let a529 = v13;
                                let a530 = 0u64;
                                if nat_eq(a529, a530) {
                                    0u64
                                } else {
                                    1u64
                                }
                            }, {
                                let a531 = v13;
                                let a532 = 0u64;
                                if nat_eq(a531, a532) {
                                    f53(v11, v3.clone())?
                                } else {
                                    0u64
                                }
                            }, {
                                let a533 = v14;
                                let a534 = 0u64;
                                if nat_eq(a533, a534) {
                                    4u64
                                } else {
                                    1u64
                                }
                            }, {
                                let a535 = v14;
                                let a536 = 0u64;
                                if nat_eq(a535, a536) {
                                    f54(v11, v3, v4)?
                                } else {
                                    1u64
                                }
                            }, {
                                let a538 = match v5.clone() {
                                    Adt1::C0(h537, _, _, _, _, _, _, _, _, _, _) => {
                                        h537
                                    }
                                };
                                let a539 = 0u64;
                                if nat_eq(a538, a539) {
                                    3u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a541 = match v5.clone() {
                                    Adt1::C0(h540, _, _, _, _, _, _, _, _, _, _) => {
                                        h540
                                    }
                                };
                                let a542 = 0u64;
                                if nat_eq(a541, a542) {
                                    f53(v11, v5)?
                                } else {
                                    0u64
                                }
                            }, 4u64, 4u64))
                        } else {
                            Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                                Adt1::C0(h543, _, _, _, _, _, _, _, _, _, _) => {
                                    h543
                                }
                            }, 1u64, 0u64, 2u64, 0u64, 3u64, f53(v11, v5.clone())?, 4u64, f34(match v5 {
                                Adt1::C0(h544, _, _, _, _, _, _, _, _, _, _) => {
                                    h544
                                }
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f18(v0: u64) -> bool {
    let a545 = 6u64;
    let a546 = v0;
    if if if if nat_le(a545, a546) {
        let a547 = v0;
        let a548 = 23u64;
        nat_le(a547, a548)
    } else {
        false
    } {
        true
    } else {
        let a549 = v0;
        let a550 = 2u64;
        if nat_eq(a549, a550) {
            true
        } else {
            let a551 = v0;
            let a552 = 3u64;
            nat_eq(a551, a552)
        }
    } {
        true
    } else {
        let a553 = v0;
        let a554 = 29u64;
        nat_eq(a553, a554)
    } {
        true
    } else {
        let a555 = 34u64;
        let a556 = v0;
        if if nat_le(a555, a556) {
            let a557 = v0;
            let a558 = 35u64;
            nat_le(a557, a558)
        } else {
            false
        } {
            true
        } else {
            let a559 = 39u64;
            let a560 = v0;
            if nat_le(a559, a560) {
                let a561 = v0;
                let a562 = 42u64;
                nat_le(a561, a562)
            } else {
                false
            }
        }
    }
}

pub fn f19(v0: u64, v1: u64, v2: u64) -> bool {
    let a563 = v0;
    let a564 = 0u64;
    if if if nat_eq(a563, a564) {
        let a565 = v1;
        let a566 = 0u64;
        nat_eq(a565, a566)
    } else {
        false
    } {
        let a567 = v2;
        let a568 = 0u64;
        nat_eq(a567, a568)
    } else {
        false
    } {
        true
    } else {
        let a569 = v0;
        let a570 = 1u64;
        if if nat_eq(a569, a570) {
            true
        } else {
            let a571 = v0;
            let a572 = 30u64;
            nat_eq(a571, a572)
        } {
            let a573 = v1;
            let a574 = v0;
            if if nat_eq(a573, a574) {
                let a575 = v2;
                let a576 = v0;
                nat_eq(a575, a576)
            } else {
                false
            } {
                true
            } else {
                let a577 = v1;
                let a578 = 0u64;
                if nat_eq(a577, a578) {
                    let a579 = v2;
                    let a580 = v0;
                    if nat_eq(a579, a580) {
                        true
                    } else {
                        let a581 = v2;
                        let a582 = 0u64;
                        nat_eq(a581, a582)
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

pub fn f20(v0: u64, v1: u64, v2: u64) -> bool {
    let a583 = v1;
    let a584 = 27u64;
    if if if nat_eq(a583, a584) {
        true
    } else {
        let a585 = v1;
        let a586 = 28u64;
        nat_eq(a585, a586)
    } {
        let a587 = v1;
        let a588 = v2;
        nat_eq(a587, a588)
    } else {
        false
    } {
        let a589 = v0;
        let a590 = 3u64;
        nat_eq(a589, a590)
    } else {
        f22(v1)
    }
}

pub fn f21(v0: u64) -> u64 {
    let a591 = v0;
    let a592 = 0u64;
    if nat_eq(a591, a592) {
        4u64
    } else {
        let a593 = v0;
        let a594 = 1u64;
        if if nat_eq(a593, a594) {
            true
        } else {
            let a595 = v0;
            let a596 = 30u64;
            nat_eq(a595, a596)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f22(v0: u64) -> bool {
    let a597 = 0u64;
    let a598 = f65(v0);
    if nat_lt(a597, a598) {
        true
    } else {
        let a599 = 24u64;
        let a600 = v0;
        if nat_le(a599, a600) {
            let a601 = v0;
            let a602 = 28u64;
            nat_le(a601, a602)
        } else {
            false
        }
    }
}

pub fn f23(v0: Adt1, v1: Adt1) -> bool {
    if f31(v1.clone()) {
        f31(v0)
    } else {
        let a608 = {
            let a605 = match v0.clone() {
                Adt1::C0(_, h603, _, _, _, _, _, _, _, _, _) => {
                    h603
                }
            };
            let a606 = match v1.clone() {
                Adt1::C0(_, h604, _, _, _, _, _, _, _, _, _) => {
                    h604
                }
            };
            let a607 = 1u64;
            nat_rem(a605, a606, a607)
        };
        let a609 = 0u64;
        if if if nat_eq(a608, a609) {
            let a615 = {
                let a612 = match v0.clone() {
                    Adt1::C0(_, _, h610, _, _, _, _, _, _, _, _) => {
                        h610
                    }
                };
                let a613 = match v1.clone() {
                    Adt1::C0(_, _, h611, _, _, _, _, _, _, _, _) => {
                        h611
                    }
                };
                let a614 = 1u64;
                nat_rem(a612, a613, a614)
            };
            let a616 = 0u64;
            nat_eq(a615, a616)
        } else {
            false
        } {
            let a622 = {
                let a619 = match v0.clone() {
                    Adt1::C0(_, _, _, h617, _, _, _, _, _, _, _) => {
                        h617
                    }
                };
                let a620 = match v1.clone() {
                    Adt1::C0(_, _, _, h618, _, _, _, _, _, _, _) => {
                        h618
                    }
                };
                let a621 = 1u64;
                nat_rem(a619, a620, a621)
            };
            let a623 = 0u64;
            nat_eq(a622, a623)
        } else {
            false
        } {
            let a629 = {
                let a626 = match v0 {
                    Adt1::C0(_, _, _, _, h624, _, _, _, _, _, _) => {
                        h624
                    }
                };
                let a627 = match v1 {
                    Adt1::C0(_, _, _, _, h625, _, _, _, _, _, _) => {
                        h625
                    }
                };
                let a628 = 1u64;
                nat_rem(a626, a627, a628)
            };
            let a630 = 0u64;
            nat_eq(a629, a630)
        } else {
            false
        }
    }
}

pub fn f24(v0: Adt1, v1: Adt1) -> bool {
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
    if if if nat_eq(a633, a634) {
        let a637 = match v0.clone() {
            Adt1::C0(_, _, h635, _, _, _, _, _, _, _, _) => {
                h635
            }
        };
        let a638 = match v1.clone() {
            Adt1::C0(_, _, h636, _, _, _, _, _, _, _, _) => {
                h636
            }
        };
        nat_eq(a637, a638)
    } else {
        false
    } {
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
        nat_eq(a641, a642)
    } else {
        false
    } {
        let a645 = match v0 {
            Adt1::C0(_, _, _, _, h643, _, _, _, _, _, _) => {
                h643
            }
        };
        let a646 = match v1 {
            Adt1::C0(_, _, _, _, h644, _, _, _, _, _, _) => {
                h644
            }
        };
        nat_eq(a645, a646)
    } else {
        false
    }
}

pub fn f25(v0: u64, v1: u64, v2: Adt1, v3: u64) -> bool {
    let v4: u64 = match v2.clone() {
        Adt1::C0(h647, _, _, _, _, _, _, _, _, _, _) => {
            h647
        }
    };
    let a648 = v1;
    let a649 = v3;
    if if nat_eq(a648, a649) {
        true
    } else {
        let a650 = v0;
        let a651 = 0u64;
        if nat_eq(a650, a651) {
            let a652 = v4;
            let a653 = 0u64;
            if if nat_eq(a652, a653) {
                true
            } else {
                f18(v3)
            } {
                true
            } else {
                f22(v3)
            }
        } else {
            false
        }
    } {
        if if if f22(v4) {
            true
        } else {
            f18(v4)
        } {
            let a655 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, h654, _, _, _, _, _) => {
                    h654
                }
            };
            let a656 = f27(v4);
            nat_eq(a655, a656)
        } else {
            false
        } {
            let a661 = {
                let a658 = match v2 {
                    Adt1::C0(_, h657, _, _, _, _, _, _, _, _, _) => {
                        h657
                    }
                };
                let a659 = f28(v4);
                let a660 = 1u64;
                nat_rem(a658, a659, a660)
            };
            let a662 = 0u64;
            nat_eq(a661, a662)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f26(v0: u64, v1: u64) -> bool {
    if f18(v0) {
        true
    } else {
        let a663 = v0;
        let a664 = v1;
        if if nat_eq(a663, a664) {
            f22(v0)
        } else {
            false
        } {
            true
        } else {
            let a665 = f21(v0);
            let a666 = 2u64;
            if nat_eq(a665, a666) {
                if f18(v1) {
                    true
                } else {
                    f22(v1)
                }
            } else {
                false
            }
        }
    }
}

pub fn f27(v0: u64) -> u64 {
    let m667 = f66(v0);
    match m667 {
        Adt4::C0 => {
            0u64
        }
        Adt4::C1(v1) => {
            match v1 {
                Adt5::C0(_, _, h668) => {
                    h668
                }
            }
        }
    }
}

pub fn f28(v0: u64) -> u64 {
    if f18(v0) {
        let m669 = f66(v0);
        match m669 {
            Adt4::C0 => {
                1u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, h670, _) => {
                        h670
                    }
                }
            }
        }
    } else {
        1u64
    }
}

pub fn f29(v0: Adt1, v1: Adt1) -> R<bool> {
    if f50(v0.clone(), v1.clone())? {
        Ok(true)
    } else {
        Ok(f67(v0, v1))
    }
}

pub fn f30(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f68(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f68(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f68(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f68(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f68(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f68(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f31(v0: Adt1) -> bool {
    f69(match v0.clone() {
        Adt1::C0(_, h671, _, _, _, _, _, _, _, _, _) => {
            h671
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h672, _, _, _, _, _, _, _, _) => {
            h672
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h673, _, _, _, _, _, _, _) => {
            h673
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, h674, _, _, _, _, _, _) => {
            h674
        }
    })
}

pub fn f32(v0: u64) -> Option<u64> {
    let a675 = v0;
    let a676 = 16u64;
    if if if nat_eq(a675, a676) {
        true
    } else {
        let a677 = v0;
        let a678 = 17u64;
        nat_eq(a677, a678)
    } {
        true
    } else {
        let a679 = v0;
        let a680 = 19u64;
        nat_eq(a679, a680)
    } {
        Some(1065353216u64)
    } else {
        None::<u64>
    }
}

pub fn f33(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<bool> {
    let a682 = match v1.clone() {
        Adt1::C0(h681, _, _, _, _, _, _, _, _, _, _) => {
            h681
        }
    };
    let a683 = 43u64;
    if nat_eq(a682, a683) {
        let a684 = v0;
        let a685 = 0u64;
        if if if if if if nat_eq(a684, a685) {
            let a687 = match v2.clone() {
                Adt1::C0(h686, _, _, _, _, _, _, _, _, _, _) => {
                    h686
                }
            };
            let a688 = 0u64;
            nat_eq(a687, a688)
        } else {
            false
        } {
            let a690 = match v3.clone() {
                Adt1::C0(h689, _, _, _, _, _, _, _, _, _, _) => {
                    h689
                }
            };
            let a691 = 0u64;
            if nat_eq(a690, a691) {
                true
            } else {
                let a693 = match v3.clone() {
                    Adt1::C0(h692, _, _, _, _, _, _, _, _, _, _) => {
                        h692
                    }
                };
                let a694 = 43u64;
                nat_eq(a693, a694)
            }
        } else {
            false
        } {
            let a696 = match v3.clone() {
                Adt1::C0(h695, _, _, _, _, _, _, _, _, _, _) => {
                    h695
                }
            };
            let a697 = 43u64;
            if nat_eq(a696, a697) {
                f23(v1.clone(), v2.clone())
            } else {
                f24(v1.clone(), v2.clone())
            }
        } else {
            false
        } {
            let a699 = match v2 {
                Adt1::C0(_, _, _, _, _, h698, _, _, _, _, _) => {
                    h698
                }
            };
            let a700 = 4u64;
            nat_eq(a699, a700)
        } else {
            false
        } {
            let a703 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h701, _, _, _, _, _) => {
                    h701
                }
            };
            let a704 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h702, _, _, _, _) => {
                    h702
                }
            };
            if nat_le(a703, a704) {
                let a707 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, h705, _, _, _, _) => {
                        h705
                    }
                };
                let a708 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h706, _, _, _) => {
                        h706
                    }
                };
                if nat_le(a707, a708) {
                    let a711 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, h709, _, _, _) => {
                            h709
                        }
                    };
                    let a712 = match v3 {
                        Adt1::C0(_, _, _, _, _, _, _, _, h710, _, _) => {
                            h710
                        }
                    };
                    nat_le(a711, a712)
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        } {
            let a720 = {
                let a718 = match v1.clone() {
                    Adt1::C0(_, _, h713, _, _, _, _, _, _, _, _) => {
                        h713
                    }
                };
                let a719 = {
                    let a716 = match v1.clone() {
                        Adt1::C0(_, _, _, h714, _, _, _, _, _, _, _) => {
                            h714
                        }
                    };
                    let a717 = match v1 {
                        Adt1::C0(_, _, _, _, h715, _, _, _, _, _, _) => {
                            h715
                        }
                    };
                    nat_mul(a716, a717)?
                };
                nat_mul(a718, a719)?
            };
            let a721 = 2147483647u64;
            Ok(nat_le(a720, a721))
        } else {
            Ok(false)
        }
    } else {
        if f23(v1.clone(), v2.clone()) {
            if f19(match v1.clone() {
                Adt1::C0(h722, _, _, _, _, _, _, _, _, _, _) => {
                    h722
                }
            }, match v2.clone() {
                Adt1::C0(h723, _, _, _, _, _, _, _, _, _, _) => {
                    h723
                }
            }, match v3.clone() {
                Adt1::C0(h724, _, _, _, _, _, _, _, _, _, _) => {
                    h724
                }
            }) {
                Ok(true)
            } else {
                let a725 = v0;
                let a726 = 3u64;
                if if if if nat_lt(a725, a726) {
                    f62(match v1.clone() {
                        Adt1::C0(h727, _, _, _, _, _, _, _, _, _, _) => {
                            h727
                        }
                    })
                } else {
                    false
                } {
                    let a730 = match v1.clone() {
                        Adt1::C0(h728, _, _, _, _, _, _, _, _, _, _) => {
                            h728
                        }
                    };
                    let a731 = match v2.clone() {
                        Adt1::C0(h729, _, _, _, _, _, _, _, _, _, _) => {
                            h729
                        }
                    };
                    nat_eq(a730, a731)
                } else {
                    false
                } {
                    let a734 = match v1 {
                        Adt1::C0(h732, _, _, _, _, _, _, _, _, _, _) => {
                            h732
                        }
                    };
                    let a735 = match v3 {
                        Adt1::C0(h733, _, _, _, _, _, _, _, _, _, _) => {
                            h733
                        }
                    };
                    nat_eq(a734, a735)
                } else {
                    false
                } {
                    let a737 = match v2 {
                        Adt1::C0(_, _, _, _, _, h736, _, _, _, _, _) => {
                            h736
                        }
                    };
                    let a738 = 8u64;
                    Ok(nat_eq(a737, a738))
                } else {
                    Ok(false)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f34(v0: u64) -> u64 {
    if f62(v0) {
        8u64
    } else {
        f65(v0)
    }
}

pub fn f35(v0: Adt1) -> bool {
    let v1: u64 = f34(match v0.clone() {
        Adt1::C0(h739, _, _, _, _, _, _, _, _, _, _) => {
            h739
        }
    });
    let a744 = {
        let a741 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h740) => {
                h740
            }
        };
        let a742 = v1;
        let a743 = 1u64;
        nat_rem(a741, a742, a743)
    };
    let a745 = 0u64;
    if if nat_eq(a744, a745) {
        f70(match v0.clone() {
            Adt1::C0(_, h746, _, _, _, _, _, _, _, _, _) => {
                h746
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h747, _, _, _, _, _) => {
                h747
            }
        }, v1)
    } else {
        false
    } {
        f71(match v0.clone() {
            Adt1::C0(_, _, h748, _, _, _, _, _, _, _, _) => {
                h748
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h749, _, _, _, _, _, _, _) => {
                h749
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h750, _, _, _, _, _, _) => {
                h750
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h751, _, _, _, _) => {
                h751
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h752, _, _, _) => {
                h752
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h753, _, _) => {
                h753
            }
        }, v1)
    } else {
        false
    }
}

pub fn f36(v0: Adt1, v1: Adt1) -> R<bool> {
    let a761 = {
        let a759 = match v0.clone() {
            Adt1::C0(_, _, h754, _, _, _, _, _, _, _, _) => {
                h754
            }
        };
        let a760 = {
            let a757 = match v0.clone() {
                Adt1::C0(_, _, _, h755, _, _, _, _, _, _, _) => {
                    h755
                }
            };
            let a758 = match v0.clone() {
                Adt1::C0(_, _, _, _, h756, _, _, _, _, _, _) => {
                    h756
                }
            };
            nat_mul(a757, a758)?
        };
        nat_mul(a759, a760)?
    };
    let a762 = 9223372036854775807u64;
    if nat_le(a761, a762) {
        if f43(v1.clone()) {
            let a764 = match v1 {
                Adt1::C0(_, h763, _, _, _, _, _, _, _, _, _) => {
                    h763
                }
            };
            let a765 = 2147483647u64;
            Ok(nat_le(a764, a765))
        } else {
            let a767 = match v0 {
                Adt1::C0(_, h766, _, _, _, _, _, _, _, _, _) => {
                    h766
                }
            };
            let a768 = 2147483647u64;
            Ok(nat_le(a767, a768))
        }
    } else {
        Ok(false)
    }
}

pub fn f37(v0: Adt1) -> R<bool> {
    f30({
        let a771 = match v0.clone() {
            Adt1::C0(_, h769, _, _, _, _, _, _, _, _, _) => {
                h769
            }
        };
        let a772 = f34(match v0.clone() {
            Adt1::C0(h770, _, _, _, _, _, _, _, _, _, _) => {
                h770
            }
        });
        nat_mul(a771, a772)?
    }, match v0.clone() {
        Adt1::C0(_, _, h773, _, _, _, _, _, _, _, _) => {
            h773
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h774, _, _, _, _, _, _, _) => {
            h774
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h775, _, _, _, _, _, _) => {
            h775
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h776, _, _, _, _) => {
            h776
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h777, _, _, _) => {
            h777
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h778, _, _) => {
            h778
        }
    })
}

pub fn f38(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a785 = {
        let a779 = v0;
        let a780 = v2;
        nat_mul(a779, a780)?
    };
    let a786 = {
        let a781 = v1;
        let a782 = 0u64;
        if nat_eq(a781, a782) {
            0u64
        } else {
            let a783 = v0;
            let a784 = 2u64;
            nat_mul(a783, a784)?
        }
    };
    nat_add(a785, a786)
}

pub fn f39(v0: Adt1) -> R<u64> {
    let a795 = {
        let a789 = match v0.clone() {
            Adt1::C0(_, h787, _, _, _, _, _, _, _, _, _) => {
                h787
            }
        };
        let a790 = match v0.clone() {
            Adt1::C0(_, _, h788, _, _, _, _, _, _, _, _) => {
                h788
            }
        };
        nat_mul(a789, a790)?
    };
    let a796 = {
        let a793 = match v0.clone() {
            Adt1::C0(_, _, _, h791, _, _, _, _, _, _, _) => {
                h791
            }
        };
        let a794 = match v0 {
            Adt1::C0(_, _, _, _, h792, _, _, _, _, _, _) => {
                h792
            }
        };
        nat_mul(a793, a794)?
    };
    nat_mul(a795, a796)
}

pub fn f40(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    if f62(v1) {
        Ok(f72(v0, v1))
    } else {
        f64(v1, v2, v3)
    }
}

pub fn f41(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f24(v0.clone(), v1.clone()) {
        let a799 = match v0.clone() {
            Adt1::C0(h797, _, _, _, _, _, _, _, _, _, _) => {
                h797
            }
        };
        let a800 = match v1.clone() {
            Adt1::C0(h798, _, _, _, _, _, _, _, _, _, _) => {
                h798
            }
        };
        nat_eq(a799, a800)
    } else {
        false
    } {
        let a803 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h801, _, _, _, _, _) => {
                h801
            }
        };
        let a804 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h802, _, _, _, _, _) => {
                h802
            }
        };
        nat_eq(a803, a804)
    } else {
        false
    } {
        let a807 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h805, _, _, _, _) => {
                h805
            }
        };
        let a808 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h806, _, _, _, _) => {
                h806
            }
        };
        nat_eq(a807, a808)
    } else {
        false
    } {
        let a811 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h809, _, _, _) => {
                h809
            }
        };
        let a812 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h810, _, _, _) => {
                h810
            }
        };
        nat_eq(a811, a812)
    } else {
        false
    } {
        let a815 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h813, _, _) => {
                h813
            }
        };
        let a816 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h814, _, _) => {
                h814
            }
        };
        nat_eq(a815, a816)
    } else {
        false
    } {
        let a818 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h817) => {
                h817
            }
        };
        let a819 = 0u64;
        if if nat_eq(a818, a819) {
            true
        } else {
            let a821 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h820) => {
                    h820
                }
            };
            let a822 = 0u64;
            nat_eq(a821, a822)
        } {
            true
        } else {
            let a825 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h823) => {
                    h823
                }
            };
            let a826 = match v1 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h824) => {
                    h824
                }
            };
            nat_eq(a825, a826)
        }
    } else {
        false
    }
}

pub fn f42(v0: u64) -> u64 {
    let a827 = v0;
    let a828 = 26u64;
    if nat_eq(a827, a828) {
        4u64
    } else {
        let a829 = v0;
        let a830 = 27u64;
        if nat_eq(a829, a830) {
            8u64
        } else {
            0u64
        }
    }
}

pub fn f43(v0: Adt1) -> bool {
    let a832 = match v0.clone() {
        Adt1::C0(_, h831, _, _, _, _, _, _, _, _, _) => {
            h831
        }
    };
    let a833 = 1u64;
    if nat_eq(a832, a833) {
        true
    } else {
        let a836 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h834, _, _, _, _, _) => {
                h834
            }
        };
        let a837 = f65(match v0 {
            Adt1::C0(h835, _, _, _, _, _, _, _, _, _, _) => {
                h835
            }
        });
        nat_eq(a836, a837)
    }
}

pub fn f44(v0: Adt1, v1: u64) -> R<u64> {
    let a843 = {
        let a839 = match v0.clone() {
            Adt1::C0(_, h838, _, _, _, _, _, _, _, _, _) => {
                h838
            }
        };
        let a840 = f28(v1);
        let a841 = 0u64;
        nat_quot(a839, a840, a841)
    };
    let a844 = f73(match v0 {
        Adt1::C0(h842, _, _, _, _, _, _, _, _, _, _) => {
            h842
        }
    }, v1);
    nat_mul(a843, a844)
}

pub fn f45(v0: u64) -> u64 {
    if if f18(v0) {
        true
    } else {
        let a845 = 24u64;
        let a846 = v0;
        if nat_le(a845, a846) {
            let a847 = v0;
            let a848 = 28u64;
            nat_le(a847, a848)
        } else {
            false
        }
    } {
        f28(v0)
    } else {
        0u64
    }
}

pub fn f46(v0: Adt1, v1: Adt1) -> u64 {
    let a851 = match v1.clone() {
        Adt1::C0(_, _, _, _, _, h849, _, _, _, _, _) => {
            h849
        }
    };
    let a852 = f27(match v1.clone() {
        Adt1::C0(h850, _, _, _, _, _, _, _, _, _, _) => {
            h850
        }
    });
    if nat_eq(a851, a852) {
        f74(v0, v1)
    } else {
        0u64
    }
}

pub fn f47(v0: Adt1) -> bool {
    let v1: u64 = match v0.clone() {
        Adt1::C0(h853, _, _, _, _, _, _, _, _, _, _) => {
            h853
        }
    };
    let a854 = 0u64;
    let a855 = f27(v1);
    if if nat_lt(a854, a855) {
        let a857 = f27(v1);
        let a858 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h856, _, _, _, _, _) => {
                h856
            }
        };
        nat_le(a857, a858)
    } else {
        false
    } {
        let a863 = {
            let a860 = match v0 {
                Adt1::C0(_, h859, _, _, _, _, _, _, _, _, _) => {
                    h859
                }
            };
            let a861 = f28(v1);
            let a862 = 1u64;
            nat_rem(a860, a861, a862)
        };
        let a864 = 0u64;
        nat_eq(a863, a864)
    } else {
        false
    }
}

pub fn f48(v0: Adt1) -> u64 {
    let a867 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h865, _, _, _, _, _) => {
            h865
        }
    };
    let a868 = f27(match v0.clone() {
        Adt1::C0(h866, _, _, _, _, _, _, _, _, _, _) => {
            h866
        }
    });
    if nat_eq(a867, a868) {
        match v0 {
            Adt1::C0(_, h869, _, _, _, _, _, _, _, _, _) => {
                h869
            }
        }
    } else {
        f28(match v0 {
            Adt1::C0(h870, _, _, _, _, _, _, _, _, _, _) => {
                h870
            }
        })
    }
}

pub fn f49(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a872 = match v0.clone() {
            Adt1::C0(_, h871, _, _, _, _, _, _, _, _, _) => {
                h871
            }
        };
        let a873 = 4u64;
        nat_mul(a872, a873)?
    };
    let v2: u64 = {
        let a876 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h874, _, _, _, _) => {
                h874
            }
        };
        let a877 = match v0.clone() {
            Adt1::C0(_, _, h875, _, _, _, _, _, _, _, _) => {
                h875
            }
        };
        nat_mul(a876, a877)?
    };
    let v3: u64 = {
        let a879 = v2;
        let a880 = match v0.clone() {
            Adt1::C0(_, _, _, h878, _, _, _, _, _, _, _) => {
                h878
            }
        };
        nat_mul(a879, a880)?
    };
    let a882 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h881, _, _, _, _, _) => {
            h881
        }
    };
    let a883 = 4u64;
    if if if if nat_eq(a882, a883) {
        let a888 = {
            let a885 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h884) => {
                    h884
                }
            };
            let a886 = 4u64;
            let a887 = 1u64;
            nat_rem(a885, a886, a887)
        };
        let a889 = 0u64;
        nat_eq(a888, a889)
    } else {
        false
    } {
        let a897 = {
            let a895 = match v0.clone() {
                Adt1::C0(_, _, h890, _, _, _, _, _, _, _, _) => {
                    h890
                }
            };
            let a896 = {
                let a893 = match v0.clone() {
                    Adt1::C0(_, _, _, h891, _, _, _, _, _, _, _) => {
                        h891
                    }
                };
                let a894 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h892, _, _, _, _, _, _) => {
                        h892
                    }
                };
                nat_mul(a893, a894)?
            };
            nat_mul(a895, a896)?
        };
        let a898 = 1u64;
        if nat_le(a897, a898) {
            true
        } else {
            let a900 = v1;
            let a901 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h899, _, _, _, _) => {
                    h899
                }
            };
            if nat_le(a900, a901) {
                let a906 = {
                    let a903 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, h902, _, _, _, _) => {
                            h902
                        }
                    };
                    let a904 = 4u64;
                    let a905 = 1u64;
                    nat_rem(a903, a904, a905)
                };
                let a907 = 0u64;
                nat_eq(a906, a907)
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a909 = match v0.clone() {
            Adt1::C0(_, _, _, h908, _, _, _, _, _, _, _) => {
                h908
            }
        };
        let a910 = 1u64;
        if nat_le(a909, a910) {
            true
        } else {
            let a912 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h911, _, _, _) => {
                    h911
                }
            };
            let a913 = v2;
            nat_eq(a912, a913)
        }
    } else {
        false
    } {
        let a915 = match v0.clone() {
            Adt1::C0(_, _, _, _, h914, _, _, _, _, _, _) => {
                h914
            }
        };
        let a916 = 1u64;
        if nat_le(a915, a916) {
            Ok(true)
        } else {
            let a918 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, h917, _, _) => {
                    h917
                }
            };
            let a919 = v3;
            Ok(nat_eq(a918, a919))
        }
    } else {
        Ok(false)
    }
}

pub fn f50(v0: Adt1, v1: Adt1) -> R<bool> {
    let a921 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h920, _) => {
            h920
        }
    };
    let a922 = 0u64;
    if nat_eq(a921, a922) {
        Ok(true)
    } else {
        let a924 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h923, _) => {
                h923
            }
        };
        let a925 = 0u64;
        if nat_eq(a924, a925) {
            Ok(true)
        } else {
            let a927 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h926) => {
                    h926
                }
            };
            let a928 = 0u64;
            if nat_eq(a927, a928) {
                Ok(true)
            } else {
                let a930 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h929) => {
                        h929
                    }
                };
                let a931 = 0u64;
                if nat_eq(a930, a931) {
                    Ok(true)
                } else {
                    let a937 = {
                        let a934 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h932) => {
                                h932
                            }
                        };
                        let a935 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h933, _) => {
                                h933
                            }
                        };
                        nat_add(a934, a935)?
                    };
                    let a938 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h936) => {
                            h936
                        }
                    };
                    if nat_le(a937, a938) {
                        Ok(true)
                    } else {
                        let a944 = {
                            let a941 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h939) => {
                                    h939
                                }
                            };
                            let a942 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h940, _) => {
                                    h940
                                }
                            };
                            nat_add(a941, a942)?
                        };
                        let a945 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h943) => {
                                h943
                            }
                        };
                        Ok(nat_le(a944, a945))
                    }
                }
            }
        }
    }
}

pub fn f51(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a948 = match v0.clone() {
            Adt1::C0(_, _, h946, _, _, _, _, _, _, _, _) => {
                h946
            }
        };
        let a949 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h947, _, _, _, _) => {
                h947
            }
        };
        nat_mul(a948, a949)?
    };
    if if f43(v0.clone()) {
        let a951 = match v0.clone() {
            Adt1::C0(_, _, _, h950, _, _, _, _, _, _, _) => {
                h950
            }
        };
        let a952 = 1u64;
        if nat_eq(a951, a952) {
            true
        } else {
            let a954 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h953, _, _, _) => {
                    h953
                }
            };
            let a955 = v1;
            nat_eq(a954, a955)
        }
    } else {
        false
    } {
        let a957 = match v0.clone() {
            Adt1::C0(_, _, _, _, h956, _, _, _, _, _, _) => {
                h956
            }
        };
        let a958 = 1u64;
        if nat_eq(a957, a958) {
            Ok(true)
        } else {
            let a963 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h959, _, _) => {
                    h959
                }
            };
            let a964 = {
                let a961 = v1;
                let a962 = match v0 {
                    Adt1::C0(_, _, _, h960, _, _, _, _, _, _, _) => {
                        h960
                    }
                };
                nat_mul(a961, a962)?
            };
            Ok(nat_eq(a963, a964))
        }
    } else {
        Ok(false)
    }
}

pub fn f52(v0: u64) -> u64 {
    let a965 = v0;
    let a966 = 3u64;
    if nat_eq(a965, a966) {
        16u64
    } else {
        10u64
    }
}

pub fn f53(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a968 = v0;
        let a969 = match v1.clone() {
            Adt1::C0(_, h967, _, _, _, _, _, _, _, _, _) => {
                h967
            }
        };
        let a970 = 0u64;
        nat_quot(a968, a969, a970)
    };
    let v3: u64 = {
        let a972 = v2;
        let a973 = match v1.clone() {
            Adt1::C0(_, _, h971, _, _, _, _, _, _, _, _) => {
                h971
            }
        };
        let a974 = 0u64;
        nat_quot(a972, a973, a974)
    };
    f75({
        let a976 = v0;
        let a977 = match v1.clone() {
            Adt1::C0(_, h975, _, _, _, _, _, _, _, _, _) => {
                h975
            }
        };
        let a978 = 0u64;
        nat_rem(a976, a977, a978)
    }, {
        let a980 = v2;
        let a981 = match v1.clone() {
            Adt1::C0(_, _, h979, _, _, _, _, _, _, _, _) => {
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
        nat_rem(a984, a985, a986)
    }, {
        let a988 = v3;
        let a989 = match v1.clone() {
            Adt1::C0(_, _, _, h987, _, _, _, _, _, _, _) => {
                h987
            }
        };
        let a990 = 0u64;
        nat_quot(a988, a989, a990)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h991, _, _, _, _, _) => {
            h991
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h992, _, _, _, _) => {
            h992
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h993, _, _, _) => {
            h993
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h994, _, _) => {
            h994
        }
    })
}

pub fn f54(v0: u64, v1: Adt1, v2: Adt1) -> R<u64> {
    let v3: u64 = {
        let a996 = v0;
        let a997 = match v1.clone() {
            Adt1::C0(_, h995, _, _, _, _, _, _, _, _, _) => {
                h995
            }
        };
        let a998 = 0u64;
        nat_quot(a996, a997, a998)
    };
    let v4: u64 = {
        let a1000 = v3;
        let a1001 = match v1.clone() {
            Adt1::C0(_, _, h999, _, _, _, _, _, _, _, _) => {
                h999
            }
        };
        let a1002 = 0u64;
        nat_quot(a1000, a1001, a1002)
    };
    f75({
        let a1008 = {
            let a1004 = v0;
            let a1005 = match v1.clone() {
                Adt1::C0(_, h1003, _, _, _, _, _, _, _, _, _) => {
                    h1003
                }
            };
            let a1006 = 0u64;
            nat_rem(a1004, a1005, a1006)
        };
        let a1009 = match v2.clone() {
            Adt1::C0(_, h1007, _, _, _, _, _, _, _, _, _) => {
                h1007
            }
        };
        let a1010 = 0u64;
        nat_rem(a1008, a1009, a1010)
    }, {
        let a1016 = {
            let a1012 = v3;
            let a1013 = match v1.clone() {
                Adt1::C0(_, _, h1011, _, _, _, _, _, _, _, _) => {
                    h1011
                }
            };
            let a1014 = 0u64;
            nat_rem(a1012, a1013, a1014)
        };
        let a1017 = match v2.clone() {
            Adt1::C0(_, _, h1015, _, _, _, _, _, _, _, _) => {
                h1015
            }
        };
        let a1018 = 0u64;
        nat_rem(a1016, a1017, a1018)
    }, {
        let a1024 = {
            let a1020 = v4;
            let a1021 = match v1.clone() {
                Adt1::C0(_, _, _, h1019, _, _, _, _, _, _, _) => {
                    h1019
                }
            };
            let a1022 = 0u64;
            nat_rem(a1020, a1021, a1022)
        };
        let a1025 = match v2.clone() {
            Adt1::C0(_, _, _, h1023, _, _, _, _, _, _, _) => {
                h1023
            }
        };
        let a1026 = 0u64;
        nat_rem(a1024, a1025, a1026)
    }, {
        let a1032 = {
            let a1028 = v4;
            let a1029 = match v1 {
                Adt1::C0(_, _, _, h1027, _, _, _, _, _, _, _) => {
                    h1027
                }
            };
            let a1030 = 0u64;
            nat_quot(a1028, a1029, a1030)
        };
        let a1033 = match v2.clone() {
            Adt1::C0(_, _, _, _, h1031, _, _, _, _, _, _) => {
                h1031
            }
        };
        let a1034 = 0u64;
        nat_rem(a1032, a1033, a1034)
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h1035, _, _, _, _, _) => {
            h1035
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h1036, _, _, _, _) => {
            h1036
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1037, _, _, _) => {
            h1037
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1038, _, _) => {
            h1038
        }
    })
}

pub fn f55(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a1040 = v0;
        let a1041 = match v1.clone() {
            Adt1::C0(_, h1039, _, _, _, _, _, _, _, _, _) => {
                h1039
            }
        };
        let a1042 = 0u64;
        nat_quot(a1040, a1041, a1042)
    };
    let v3: u64 = {
        let a1044 = v2;
        let a1045 = match v1.clone() {
            Adt1::C0(_, _, h1043, _, _, _, _, _, _, _, _) => {
                h1043
            }
        };
        let a1046 = 0u64;
        nat_quot(a1044, a1045, a1046)
    };
    let v4: u64 = {
        let a1052 = {
            let a1048 = v0;
            let a1049 = match v1.clone() {
                Adt1::C0(_, h1047, _, _, _, _, _, _, _, _, _) => {
                    h1047
                }
            };
            let a1050 = 0u64;
            nat_rem(a1048, a1049, a1050)
        };
        let a1053 = f28(match v1.clone() {
            Adt1::C0(h1051, _, _, _, _, _, _, _, _, _, _) => {
                h1051
            }
        });
        let a1054 = 0u64;
        nat_quot(a1052, a1053, a1054)
    };
    f75(v4, {
        let a1056 = v2;
        let a1057 = match v1.clone() {
            Adt1::C0(_, _, h1055, _, _, _, _, _, _, _, _) => {
                h1055
            }
        };
        let a1058 = 0u64;
        nat_rem(a1056, a1057, a1058)
    }, {
        let a1060 = v3;
        let a1061 = match v1.clone() {
            Adt1::C0(_, _, _, h1059, _, _, _, _, _, _, _) => {
                h1059
            }
        };
        let a1062 = 0u64;
        nat_rem(a1060, a1061, a1062)
    }, {
        let a1064 = v3;
        let a1065 = match v1.clone() {
            Adt1::C0(_, _, _, h1063, _, _, _, _, _, _, _) => {
                h1063
            }
        };
        let a1066 = 0u64;
        nat_quot(a1064, a1065, a1066)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h1067, _, _, _, _, _) => {
            h1067
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h1068, _, _, _, _) => {
            h1068
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1069, _, _, _) => {
            h1069
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1070, _, _) => {
            h1070
        }
    })
}

pub fn f56(v0: u64, v1: Adt1, v2: u64, v3: u64) -> R<Adt3> {
    let v4: u64 = {
        let a1071 = v0;
        let a1072 = 2u64;
        let a1073 = 0u64;
        nat_quot(a1071, a1072, a1073)
    };
    let v5: u64 = f42(match v1.clone() {
        Adt1::C0(h1074, _, _, _, _, _, _, _, _, _, _) => {
            h1074
        }
    });
    let a1078 = {
        let a1075 = v0;
        let a1076 = 2u64;
        let a1077 = 0u64;
        nat_rem(a1075, a1076, a1077)
    };
    let a1079 = 0u64;
    if nat_eq(a1078, a1079) {
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 4u64, f53(v4, v1)?, 2u64, 0u64, 0u64, 2u64, v5, 8u64))
    } else {
        let a1080 = v3;
        let a1081 = v2;
        if if nat_lt(a1080, a1081) {
            let a1084 = v3;
            let a1085 = {
                let a1082 = v5;
                let a1083 = 4u64;
                if nat_eq(a1082, a1083) {
                    2147483648u64
                } else {
                    9223372036854775808u64
                }
            };
            nat_lt(a1084, a1085)
        } else {
            false
        } {
            Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, {
                let a1086 = 3u64;
                let a1087 = v4;
                nat_add(a1086, a1087)?
            }, 8u64, 8u64))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f57(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64) -> R<Adt3> {
    let v5: u64 = {
        let a1089 = 1u64;
        let a1090 = f44(v1.clone(), match v3.clone() {
            Adt1::C0(h1088, _, _, _, _, _, _, _, _, _, _) => {
                h1088
            }
        })?;
        nat_add(a1089, a1090)?
    };
    let v6: u64 = {
        let a1091 = v0;
        let a1092 = v5;
        let a1093 = 0u64;
        nat_rem(a1091, a1092, a1093)
    };
    let v7: u64 = {
        let a1094 = v0;
        let a1095 = v5;
        let a1096 = 0u64;
        nat_quot(a1094, a1095, a1096)
    };
    let v8: u64 = {
        let a1098 = v7;
        let a1099 = match v1.clone() {
            Adt1::C0(_, _, h1097, _, _, _, _, _, _, _, _) => {
                h1097
            }
        };
        let a1100 = 0u64;
        nat_rem(a1098, a1099, a1100)
    };
    let v9: u64 = {
        let a1102 = v7;
        let a1103 = match v1.clone() {
            Adt1::C0(_, _, h1101, _, _, _, _, _, _, _, _) => {
                h1101
            }
        };
        let a1104 = 0u64;
        nat_quot(a1102, a1103, a1104)
    };
    let v10: u64 = {
        let a1106 = v9;
        let a1107 = match v1.clone() {
            Adt1::C0(_, _, _, h1105, _, _, _, _, _, _, _) => {
                h1105
            }
        };
        let a1108 = 0u64;
        nat_rem(a1106, a1107, a1108)
    };
    let v11: u64 = {
        let a1110 = v9;
        let a1111 = match v1.clone() {
            Adt1::C0(_, _, _, h1109, _, _, _, _, _, _, _) => {
                h1109
            }
        };
        let a1112 = 0u64;
        nat_quot(a1110, a1111, a1112)
    };
    let a1113 = v6;
    let a1114 = 0u64;
    if nat_eq(a1113, a1114) {
        let v12: u64 = {
            let a1131 = v8;
            let a1132 = {
                let a1129 = match v2.clone() {
                    Adt1::C0(_, h1115, _, _, _, _, _, _, _, _, _) => {
                        h1115
                    }
                };
                let a1130 = {
                    let a1127 = {
                        let a1117 = v10;
                        let a1118 = match v2.clone() {
                            Adt1::C0(_, _, h1116, _, _, _, _, _, _, _, _) => {
                                h1116
                            }
                        };
                        let a1119 = 0u64;
                        nat_rem(a1117, a1118, a1119)
                    };
                    let a1128 = {
                        let a1125 = match v2.clone() {
                            Adt1::C0(_, _, h1120, _, _, _, _, _, _, _, _) => {
                                h1120
                            }
                        };
                        let a1126 = {
                            let a1122 = v11;
                            let a1123 = match v2 {
                                Adt1::C0(_, _, _, h1121, _, _, _, _, _, _, _) => {
                                    h1121
                                }
                            };
                            let a1124 = 0u64;
                            nat_rem(a1122, a1123, a1124)
                        };
                        nat_mul(a1125, a1126)?
                    };
                    nat_add(a1127, a1128)?
                };
                nat_mul(a1129, a1130)?
            };
            nat_add(a1131, a1132)?
        };
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, {
            let a1133 = 3u64;
            let a1134 = v12;
            nat_add(a1133, a1134)?
        }, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
    } else {
        let a1136 = v4;
        let a1137 = match v3.clone() {
            Adt1::C0(_, _, h1135, _, _, _, _, _, _, _, _) => {
                h1135
            }
        };
        if nat_lt(a1136, a1137) {
            let v13: u64 = {
                let a1142 = {
                    let a1138 = v6;
                    let a1139 = 1u64;
                    nat_sub(a1138, a1139)
                };
                let a1143 = f73(match v1.clone() {
                    Adt1::C0(h1140, _, _, _, _, _, _, _, _, _, _) => {
                        h1140
                    }
                }, match v3.clone() {
                    Adt1::C0(h1141, _, _, _, _, _, _, _, _, _, _) => {
                        h1141
                    }
                });
                let a1144 = 0u64;
                nat_quot(a1142, a1143, a1144)
            };
            let v14: u64 = f75({
                let a1146 = v13;
                let a1147 = f28(match v3.clone() {
                    Adt1::C0(h1145, _, _, _, _, _, _, _, _, _, _) => {
                        h1145
                    }
                });
                nat_mul(a1146, a1147)?
            }, v8, v10, v11, match v1.clone() {
                Adt1::C0(_, _, _, _, _, h1148, _, _, _, _, _) => {
                    h1148
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, h1149, _, _, _, _) => {
                    h1149
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1150, _, _, _) => {
                    h1150
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1151, _, _) => {
                    h1151
                }
            })?;
            let v15: u64 = f75(v13, v4, v10, v11, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h1152, _, _, _, _, _) => {
                    h1152
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h1153, _, _, _, _) => {
                    h1153
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1154, _, _, _) => {
                    h1154
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1155, _, _) => {
                    h1155
                }
            })?;
            let a1157 = 0u64;
            let a1158 = f45(match v3.clone() {
                Adt1::C0(h1156, _, _, _, _, _, _, _, _, _, _) => {
                    h1156
                }
            });
            if nat_lt(a1157, a1158) {
                Ok(Adt3::C6(v14, v15))
            } else {
                Ok(f76(match v1 {
                    Adt1::C0(h1159, _, _, _, _, _, _, _, _, _, _) => {
                        h1159
                    }
                }, match v3 {
                    Adt1::C0(h1160, _, _, _, _, _, _, _, _, _, _) => {
                        h1160
                    }
                }, {
                    let a1161 = v6;
                    let a1162 = 1u64;
                    nat_sub(a1161, a1162)
                }, v14, v15))
            }
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f58(v0: u64, v1: u64, v2: u64) -> Adt3 {
    Adt3::C3(2u64, 0u64, 0u64, 0u64, 1u64, v0, v1, v2, 0u64, 2u64, 4u64, 4u64)
}

pub fn f59(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt3 {
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

pub fn f60(v0: u64, v1: u64) -> Adt3 {
    let a1163 = v0;
    let a1164 = 8u64;
    if nat_eq(a1163, a1164) {
        Adt3::C3(1u64, 6u64, 0u64, 0u64, 1u64, 0u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
    } else {
        let a1165 = v0;
        let a1166 = 9u64;
        if nat_eq(a1165, a1166) {
            Adt3::C3(1u64, 2u64, 0u64, 0u64, 2u64, v1, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
        } else {
            let a1167 = v0;
            let a1168 = 10u64;
            if nat_eq(a1167, a1168) {
                Adt3::C3(5u64, 16u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
            } else {
                let a1169 = v0;
                let a1170 = 11u64;
                if nat_eq(a1169, a1170) {
                    Adt3::C3(1u64, 0u64, 0u64, 0u64, 2u64, 1065353216u64, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
                } else {
                    let a1171 = v0;
                    let a1172 = 12u64;
                    if nat_eq(a1171, a1172) {
                        Adt3::C3(1u64, 3u64, 0u64, 0u64, 1u64, 0u64, 1u64, 2u64, 0u64, 0u64, 4u64, 4u64)
                    } else {
                        let a1173 = v0;
                        let a1174 = 13u64;
                        if nat_eq(a1173, a1174) {
                            Adt3::C3(1u64, 0u64, 0u64, 0u64, 1u64, 1u64, 2u64, 1065353216u64, 0u64, 1u64, 4u64, 4u64)
                        } else {
                            let a1175 = v0;
                            let a1176 = 14u64;
                            if nat_eq(a1175, a1176) {
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

pub fn f61(v0: u64, v1: u64, v2: Adt2) -> R<Adt3> {
    let v3: Adt1 = match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1177, _, _, _, _) => {
            h1177
        }
    };
    let v4: Adt1 = match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h1178, _, _, _) => {
            h1178
        }
    };
    let v5: Adt1 = match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1179, _, _) => {
            h1179
        }
    };
    let v6: u64 = match v2.clone() {
        Adt2::C0(_, _, _, h1180, _, _, _, _, _, _, _, _) => {
            h1180
        }
    };
    let v7: u64 = {
        let a1182 = v1;
        let a1183 = match v5.clone() {
            Adt1::C0(_, h1181, _, _, _, _, _, _, _, _, _) => {
                h1181
            }
        };
        let a1184 = 0u64;
        nat_quot(a1182, a1183, a1184)
    };
    let v8: u64 = f21(match v5.clone() {
        Adt1::C0(h1185, _, _, _, _, _, _, _, _, _, _) => {
            h1185
        }
    });
    let v9: u64 = {
        let a1190 = {
            let a1187 = v1;
            let a1188 = match v5.clone() {
                Adt1::C0(_, h1186, _, _, _, _, _, _, _, _, _) => {
                    h1186
                }
            };
            let a1189 = 0u64;
            nat_rem(a1187, a1188, a1189)
        };
        let a1191 = v8;
        nat_mul(a1190, a1191)?
    };
    let v10: u64 = {
        let a1193 = match v5.clone() {
            Adt1::C0(_, h1192, _, _, _, _, _, _, _, _, _) => {
                h1192
            }
        };
        let a1194 = v8;
        nat_mul(a1193, a1194)?
    };
    let a1195 = v0;
    let a1196 = 0u64;
    if nat_eq(a1195, a1196) {
        Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
            Adt1::C0(h1197, _, _, _, _, _, _, _, _, _, _) => {
                h1197
            }
        }, 0u64, 0u64, {
            let a1205 = {
                let a1201 = {
                    let a1199 = v7;
                    let a1200 = match v3 {
                        Adt1::C0(_, _, _, _, _, _, h1198, _, _, _, _) => {
                            h1198
                        }
                    };
                    nat_mul(a1199, a1200)?
                };
                let a1202 = v9;
                nat_add(a1201, a1202)?
            };
            let a1206 = {
                let a1203 = v6;
                let a1204 = 100u64;
                if nat_eq(a1203, a1204) {
                    v10
                } else {
                    0u64
                }
            };
            nat_add(a1205, a1206)?
        }, 2u64, 0u64, 0u64, 0u64, v8, 4u64))
    } else {
        let a1207 = v0;
        let a1208 = 1u64;
        if nat_eq(a1207, a1208) {
            Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                Adt1::C0(h1209, _, _, _, _, _, _, _, _, _, _) => {
                    h1209
                }
            }, 0u64, {
                let a1210 = v6;
                let a1211 = 98u64;
                if nat_eq(a1210, a1211) {
                    4u64
                } else {
                    0u64
                }
            }, {
                let a1219 = {
                    let a1215 = {
                        let a1213 = v7;
                        let a1214 = match v4 {
                            Adt1::C0(_, _, _, _, _, _, h1212, _, _, _, _) => {
                                h1212
                            }
                        };
                        nat_mul(a1213, a1214)?
                    };
                    let a1216 = v9;
                    nat_add(a1215, a1216)?
                };
                let a1220 = {
                    let a1217 = v6;
                    let a1218 = 99u64;
                    if nat_eq(a1217, a1218) {
                        v10
                    } else {
                        0u64
                    }
                };
                nat_add(a1219, a1220)?
            }, 2u64, 0u64, 0u64, 1u64, v8, 4u64))
        } else {
            let a1221 = v0;
            let a1222 = 2u64;
            if nat_eq(a1221, a1222) {
                Ok(Adt3::C4(f77(match v2 {
                    Adt2::C0(_, _, _, _, h1223, _, _, _, _, _, _, _) => {
                        h1223
                    }
                }, v5.clone(), v1), match v5 {
                    Adt1::C0(h1224, _, _, _, _, _, _, _, _, _, _) => {
                        h1224
                    }
                }, 0u64, 1u64, 0u64))
            } else {
                Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                    Adt1::C0(h1225, _, _, _, _, _, _, _, _, _, _) => {
                        h1225
                    }
                }, 1u64, 0u64, 2u64, 0u64, 3u64, {
                    let a1229 = {
                        let a1227 = v7;
                        let a1228 = match v5 {
                            Adt1::C0(_, _, _, _, _, _, h1226, _, _, _, _) => {
                                h1226
                            }
                        };
                        nat_mul(a1227, a1228)?
                    };
                    let a1230 = v9;
                    nat_add(a1229, a1230)?
                }, 4u64, v8))
            }
        }
    }
}

pub fn f62(v0: u64) -> bool {
    let a1231 = v0;
    let a1232 = 27u64;
    if nat_eq(a1231, a1232) {
        true
    } else {
        let a1233 = v0;
        let a1234 = 28u64;
        nat_eq(a1233, a1234)
    }
}

pub fn f63(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1235, _, _, _, _) => {
            h1235
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h1236, _, _, _) => {
            h1236
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1237, _, _) => {
            h1237
        }
    };
    let v5: u64 = match v1 {
        Adt2::C0(_, _, _, h1238, _, _, _, _, _, _, _, _) => {
            h1238
        }
    };
    let v6: u64 = f72(v5, match v2.clone() {
        Adt1::C0(h1239, _, _, _, _, _, _, _, _, _, _) => {
            h1239
        }
    });
    let v7: u64 = {
        let a1240 = v0;
        let a1241 = v6;
        let a1242 = 0u64;
        nat_quot(a1240, a1241, a1242)
    };
    let v8: u64 = f54(v7, v2.clone(), v3)?;
    let a1243 = v6;
    let a1244 = 2u64;
    if if nat_eq(a1243, a1244) {
        let a1248 = {
            let a1245 = v0;
            let a1246 = v6;
            let a1247 = 0u64;
            nat_rem(a1245, a1246, a1247)
        };
        let a1249 = 0u64;
        nat_eq(a1248, a1249)
    } else {
        false
    } {
        Ok(Adt3::C3(6u64, 15u64, 0u64, 0u64, 4u64, v8, 2u64, 9223372036854775808u64, 0u64, 0u64, 8u64, 8u64))
    } else {
        let v9: u64 = {
            let a1251 = match v2.clone() {
                Adt1::C0(h1250, _, _, _, _, _, _, _, _, _, _) => {
                    h1250
                }
            };
            let a1252 = 27u64;
            if nat_eq(a1251, a1252) {
                let a1253 = 15u64;
                let a1254 = v5;
                nat_add(a1253, a1254)?
            } else {
                let a1255 = v5;
                let a1256 = 2u64;
                if nat_eq(a1255, a1256) {
                    14u64
                } else {
                    0u64
                }
            }
        };
        Ok(Adt3::C3(6u64, v9, 0u64, 0u64, 0u64, f53(v7, v2)?, {
            let a1257 = v6;
            let a1258 = 2u64;
            if nat_eq(a1257, a1258) {
                1u64
            } else {
                4u64
            }
        }, {
            let a1259 = v6;
            let a1260 = 2u64;
            if nat_eq(a1259, a1260) {
                0u64
            } else {
                v8
            }
        }, 3u64, f53(v7, v4)?, 8u64, 8u64))
    }
}

pub fn f64(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1271 = {
        let a1267 = {
            let a1263 = 1u64;
            let a1264 = {
                let a1261 = v0;
                let a1262 = 0u64;
                if nat_eq(a1261, a1262) {
                    0u64
                } else {
                    1u64
                }
            };
            nat_add(a1263, a1264)?
        };
        let a1268 = {
            let a1265 = v1;
            let a1266 = 0u64;
            if nat_eq(a1265, a1266) {
                0u64
            } else {
                1u64
            }
        };
        nat_add(a1267, a1268)?
    };
    let a1272 = {
        let a1269 = v2;
        let a1270 = 0u64;
        if nat_eq(a1269, a1270) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a1271, a1272)
}

pub fn f65(v0: u64) -> u64 {
    let a1273 = v0;
    let a1274 = 43u64;
    if nat_eq(a1273, a1274) {
        1u64
    } else {
        f21(v0)
    }
}

pub fn f66(v0: u64) -> Adt4 {
    let a1275 = v0;
    let a1276 = 0u64;
    if nat_eq(a1275, a1276) {
        Adt4::C1(Adt5::C0(0u64, 1u64, 4u64))
    } else {
        let a1277 = v0;
        let a1278 = 1u64;
        if nat_eq(a1277, a1278) {
            Adt4::C1(Adt5::C0(1u64, 1u64, 2u64))
        } else {
            let a1279 = v0;
            let a1280 = 2u64;
            if nat_eq(a1279, a1280) {
                Adt4::C1(Adt5::C0(2u64, 32u64, 18u64))
            } else {
                let a1281 = v0;
                let a1282 = 3u64;
                if nat_eq(a1281, a1282) {
                    Adt4::C1(Adt5::C0(3u64, 32u64, 20u64))
                } else {
                    let a1283 = v0;
                    let a1284 = 6u64;
                    if nat_eq(a1283, a1284) {
                        Adt4::C1(Adt5::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1285 = v0;
                        let a1286 = 7u64;
                        if nat_eq(a1285, a1286) {
                            Adt4::C1(Adt5::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1287 = v0;
                            let a1288 = 8u64;
                            if nat_eq(a1287, a1288) {
                                Adt4::C1(Adt5::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1289 = v0;
                                let a1290 = 9u64;
                                if nat_eq(a1289, a1290) {
                                    Adt4::C1(Adt5::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1291 = v0;
                                    let a1292 = 10u64;
                                    if nat_eq(a1291, a1292) {
                                        Adt4::C1(Adt5::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1293 = v0;
                                        let a1294 = 11u64;
                                        if nat_eq(a1293, a1294) {
                                            Adt4::C1(Adt5::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1295 = v0;
                                            let a1296 = 12u64;
                                            if nat_eq(a1295, a1296) {
                                                Adt4::C1(Adt5::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1297 = v0;
                                                let a1298 = 13u64;
                                                if nat_eq(a1297, a1298) {
                                                    Adt4::C1(Adt5::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1299 = v0;
                                                    let a1300 = 14u64;
                                                    if nat_eq(a1299, a1300) {
                                                        Adt4::C1(Adt5::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1301 = v0;
                                                        let a1302 = 15u64;
                                                        if nat_eq(a1301, a1302) {
                                                            Adt4::C1(Adt5::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1303 = v0;
                                                            let a1304 = 16u64;
                                                            if nat_eq(a1303, a1304) {
                                                                Adt4::C1(Adt5::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1305 = v0;
                                                                let a1306 = 17u64;
                                                                if nat_eq(a1305, a1306) {
                                                                    Adt4::C1(Adt5::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1307 = v0;
                                                                    let a1308 = 18u64;
                                                                    if nat_eq(a1307, a1308) {
                                                                        Adt4::C1(Adt5::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1309 = v0;
                                                                        let a1310 = 19u64;
                                                                        if nat_eq(a1309, a1310) {
                                                                            Adt4::C1(Adt5::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1311 = v0;
                                                                            let a1312 = 20u64;
                                                                            if nat_eq(a1311, a1312) {
                                                                                Adt4::C1(Adt5::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1313 = v0;
                                                                                let a1314 = 21u64;
                                                                                if nat_eq(a1313, a1314) {
                                                                                    Adt4::C1(Adt5::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1315 = v0;
                                                                                    let a1316 = 22u64;
                                                                                    if nat_eq(a1315, a1316) {
                                                                                        Adt4::C1(Adt5::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1317 = v0;
                                                                                        let a1318 = 23u64;
                                                                                        if nat_eq(a1317, a1318) {
                                                                                            Adt4::C1(Adt5::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1319 = v0;
                                                                                            let a1320 = 24u64;
                                                                                            if nat_eq(a1319, a1320) {
                                                                                                Adt4::C1(Adt5::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1321 = v0;
                                                                                                let a1322 = 25u64;
                                                                                                if nat_eq(a1321, a1322) {
                                                                                                    Adt4::C1(Adt5::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1323 = v0;
                                                                                                    let a1324 = 26u64;
                                                                                                    if nat_eq(a1323, a1324) {
                                                                                                        Adt4::C1(Adt5::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1325 = v0;
                                                                                                        let a1326 = 27u64;
                                                                                                        if nat_eq(a1325, a1326) {
                                                                                                            Adt4::C1(Adt5::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1327 = v0;
                                                                                                            let a1328 = 28u64;
                                                                                                            if nat_eq(a1327, a1328) {
                                                                                                                Adt4::C1(Adt5::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1329 = v0;
                                                                                                                let a1330 = 29u64;
                                                                                                                if nat_eq(a1329, a1330) {
                                                                                                                    Adt4::C1(Adt5::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1331 = v0;
                                                                                                                    let a1332 = 30u64;
                                                                                                                    if nat_eq(a1331, a1332) {
                                                                                                                        Adt4::C1(Adt5::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1333 = v0;
                                                                                                                        let a1334 = 34u64;
                                                                                                                        if nat_eq(a1333, a1334) {
                                                                                                                            Adt4::C1(Adt5::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1335 = v0;
                                                                                                                            let a1336 = 35u64;
                                                                                                                            if nat_eq(a1335, a1336) {
                                                                                                                                Adt4::C1(Adt5::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1337 = v0;
                                                                                                                                let a1338 = 39u64;
                                                                                                                                if nat_eq(a1337, a1338) {
                                                                                                                                    Adt4::C1(Adt5::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1339 = v0;
                                                                                                                                    let a1340 = 40u64;
                                                                                                                                    if nat_eq(a1339, a1340) {
                                                                                                                                        Adt4::C1(Adt5::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1341 = v0;
                                                                                                                                        let a1342 = 41u64;
                                                                                                                                        if nat_eq(a1341, a1342) {
                                                                                                                                            Adt4::C1(Adt5::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1343 = v0;
                                                                                                                                            let a1344 = 42u64;
                                                                                                                                            if nat_eq(a1343, a1344) {
                                                                                                                                                Adt4::C1(Adt5::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1345 = v0;
                                                                                                                                                let a1346 = 43u64;
                                                                                                                                                if nat_eq(a1345, a1346) {
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

pub fn f67(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f24(v0.clone(), v1.clone()) {
        let a1349 = match v0.clone() {
            Adt1::C0(h1347, _, _, _, _, _, _, _, _, _, _) => {
                h1347
            }
        };
        let a1350 = match v1.clone() {
            Adt1::C0(h1348, _, _, _, _, _, _, _, _, _, _) => {
                h1348
            }
        };
        nat_eq(a1349, a1350)
    } else {
        false
    } {
        let a1353 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1351) => {
                h1351
            }
        };
        let a1354 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1352) => {
                h1352
            }
        };
        nat_eq(a1353, a1354)
    } else {
        false
    } {
        let a1357 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1355, _, _, _, _, _) => {
                h1355
            }
        };
        let a1358 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1356, _, _, _, _, _) => {
                h1356
            }
        };
        nat_eq(a1357, a1358)
    } else {
        false
    } {
        let a1361 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1359, _, _, _, _) => {
                h1359
            }
        };
        let a1362 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1360, _, _, _, _) => {
                h1360
            }
        };
        nat_eq(a1361, a1362)
    } else {
        false
    } {
        let a1365 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1363, _, _, _) => {
                h1363
            }
        };
        let a1366 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1364, _, _, _) => {
                h1364
            }
        };
        nat_eq(a1365, a1366)
    } else {
        false
    } {
        let a1369 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1367, _, _) => {
                h1367
            }
        };
        let a1370 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1368, _, _) => {
                h1368
            }
        };
        nat_eq(a1369, a1370)
    } else {
        false
    }
}

pub fn f68(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a1371 = v1;
    let a1372 = 1u64;
    if if if nat_le(a1371, a1372) {
        true
    } else {
        let a1373 = v0;
        let a1374 = v2;
        nat_le(a1373, a1374)
    } {
        let a1375 = v3;
        let a1376 = 1u64;
        if nat_le(a1375, a1376) {
            true
        } else {
            let a1383 = {
                let a1381 = v0;
                let a1382 = {
                    let a1379 = {
                        let a1377 = v1;
                        let a1378 = 1u64;
                        nat_sub(a1377, a1378)
                    };
                    let a1380 = v2;
                    nat_mul(a1379, a1380)?
                };
                nat_add(a1381, a1382)?
            };
            let a1384 = v4;
            nat_le(a1383, a1384)
        }
    } else {
        false
    } {
        let a1385 = v5;
        let a1386 = 1u64;
        if nat_le(a1385, a1386) {
            Ok(true)
        } else {
            let a1399 = {
                let a1397 = {
                    let a1391 = v0;
                    let a1392 = {
                        let a1389 = {
                            let a1387 = v1;
                            let a1388 = 1u64;
                            nat_sub(a1387, a1388)
                        };
                        let a1390 = v2;
                        nat_mul(a1389, a1390)?
                    };
                    nat_add(a1391, a1392)?
                };
                let a1398 = {
                    let a1395 = {
                        let a1393 = v3;
                        let a1394 = 1u64;
                        nat_sub(a1393, a1394)
                    };
                    let a1396 = v4;
                    nat_mul(a1395, a1396)?
                };
                nat_add(a1397, a1398)?
            };
            let a1400 = v6;
            Ok(nat_le(a1399, a1400))
        }
    } else {
        Ok(false)
    }
}

pub fn f69(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1401 = v0;
    let a1402 = 0u64;
    if if if nat_eq(a1401, a1402) {
        true
    } else {
        let a1403 = v1;
        let a1404 = 0u64;
        nat_eq(a1403, a1404)
    } {
        true
    } else {
        let a1405 = v2;
        let a1406 = 0u64;
        nat_eq(a1405, a1406)
    } {
        true
    } else {
        let a1407 = v3;
        let a1408 = 0u64;
        nat_eq(a1407, a1408)
    }
}

pub fn f70(v0: u64, v1: u64, v2: u64) -> bool {
    let a1409 = v0;
    let a1410 = 1u64;
    if nat_le(a1409, a1410) {
        true
    } else {
        let a1414 = {
            let a1411 = v1;
            let a1412 = v2;
            let a1413 = 1u64;
            nat_rem(a1411, a1412, a1413)
        };
        let a1415 = 0u64;
        nat_eq(a1414, a1415)
    }
}

pub fn f71(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f70(v0, v3, v6) {
        f70(v1, v4, v6)
    } else {
        false
    } {
        f70(v2, v5, v6)
    } else {
        false
    }
}

pub fn f72(v0: u64, v1: u64) -> u64 {
    let a1416 = v1;
    let a1417 = 28u64;
    if if nat_eq(a1416, a1417) {
        let a1418 = v0;
        let a1419 = 1u64;
        nat_eq(a1418, a1419)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f73(v0: u64, v1: u64) -> u64 {
    let a1420 = v0;
    let a1421 = 1u64;
    if if nat_eq(a1420, a1421) {
        let a1422 = v1;
        let a1423 = 43u64;
        nat_eq(a1422, a1423)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f74(v0: Adt1, v1: Adt1) -> u64 {
    let v2: u64 = match v0.clone() {
        Adt1::C0(h1424, _, _, _, _, _, _, _, _, _, _) => {
            h1424
        }
    };
    let v3: u64 = match v1.clone() {
        Adt1::C0(h1425, _, _, _, _, _, _, _, _, _, _) => {
            h1425
        }
    };
    if if if f24(v0.clone(), v1.clone()) {
        if f18(v2) {
            true
        } else {
            f18(v3)
        }
    } else {
        false
    } {
        if if if f18(v2) {
            true
        } else {
            f22(v2)
        } {
            if f18(v3) {
                true
            } else {
                f22(v3)
            }
        } else {
            false
        } {
            let a1430 = {
                let a1427 = match v0.clone() {
                    Adt1::C0(_, h1426, _, _, _, _, _, _, _, _, _) => {
                        h1426
                    }
                };
                let a1428 = f28(v2);
                let a1429 = 1u64;
                nat_rem(a1427, a1428, a1429)
            };
            let a1431 = 0u64;
            if nat_eq(a1430, a1431) {
                let a1436 = {
                    let a1433 = match v1.clone() {
                        Adt1::C0(_, h1432, _, _, _, _, _, _, _, _, _) => {
                            h1432
                        }
                    };
                    let a1434 = f28(v3);
                    let a1435 = 1u64;
                    nat_rem(a1433, a1434, a1435)
                };
                let a1437 = 0u64;
                nat_eq(a1436, a1437)
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    } {
        let a1439 = match v0.clone() {
            Adt1::C0(_, h1438, _, _, _, _, _, _, _, _, _) => {
                h1438
            }
        };
        let a1440 = 0u64;
        if nat_eq(a1439, a1440) {
            f28(v3)
        } else {
            match v0 {
                Adt1::C0(_, h1441, _, _, _, _, _, _, _, _, _) => {
                    h1441
                }
            }
        }
    } else {
        if if if f18(v3) {
            let a1442 = 0u64;
            let a1443 = f21(v2);
            nat_lt(a1442, a1443)
        } else {
            false
        } {
            let a1448 = {
                let a1445 = match v0 {
                    Adt1::C0(_, h1444, _, _, _, _, _, _, _, _, _) => {
                        h1444
                    }
                };
                let a1446 = f28(v3);
                let a1447 = 1u64;
                nat_rem(a1445, a1446, a1447)
            };
            let a1449 = 0u64;
            if nat_eq(a1448, a1449) {
                let a1454 = {
                    let a1451 = match v1 {
                        Adt1::C0(_, h1450, _, _, _, _, _, _, _, _, _) => {
                            h1450
                        }
                    };
                    let a1452 = f28(v3);
                    let a1453 = 1u64;
                    nat_rem(a1451, a1452, a1453)
                };
                let a1455 = 0u64;
                nat_eq(a1454, a1455)
            } else {
                false
            }
        } else {
            false
        } {
            f28(v3)
        } else {
            0u64
        }
    }
}

pub fn f75(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a1468 = {
        let a1464 = {
            let a1460 = {
                let a1456 = v0;
                let a1457 = v4;
                nat_mul(a1456, a1457)?
            };
            let a1461 = {
                let a1458 = v1;
                let a1459 = v5;
                nat_mul(a1458, a1459)?
            };
            nat_add(a1460, a1461)?
        };
        let a1465 = {
            let a1462 = v2;
            let a1463 = v6;
            nat_mul(a1462, a1463)?
        };
        nat_add(a1464, a1465)?
    };
    let a1469 = {
        let a1466 = v3;
        let a1467 = v7;
        nat_mul(a1466, a1467)?
    };
    nat_add(a1468, a1469)
}

pub fn f76(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt3 {
    let a1470 = f73(v0, v1);
    let a1471 = 2u64;
    if nat_eq(a1470, a1471) {
        let a1475 = {
            let a1472 = v2;
            let a1473 = 2u64;
            let a1474 = 0u64;
            nat_rem(a1472, a1473, a1474)
        };
        let a1476 = 0u64;
        if nat_eq(a1475, a1476) {
            Adt3::C3(0u64, 0u64, v0, 0u64, 0u64, v3, 2u64, 0u64, 1u64, 0u64, f21(v0), 8u64)
        } else {
            Adt3::C3(0u64, 0u64, 0u64, v1, 1u64, 0u64, 2u64, 0u64, 3u64, v4, 8u64, f27(v1))
        }
    } else {
        Adt3::C3(0u64, 0u64, v0, v1, 0u64, v3, 2u64, 0u64, 3u64, v4, f21(v0), f27(v1))
    }
}

pub fn f77(v0: u64, v1: Adt1, v2: u64) -> u64 {
    let a1477 = v0;
    let a1478 = 2u64;
    if if nat_eq(a1477, a1478) {
        let a1480 = match v1.clone() {
            Adt1::C0(h1479, _, _, _, _, _, _, _, _, _, _) => {
                h1479
            }
        };
        let a1481 = 0u64;
        if nat_eq(a1480, a1481) {
            let a1493 = {
                let a1483 = v2;
                let a1484 = match v1.clone() {
                    Adt1::C0(_, h1482, _, _, _, _, _, _, _, _, _) => {
                        h1482
                    }
                };
                let a1485 = 0u64;
                nat_rem(a1483, a1484, a1485)
            };
            let a1494 = {
                let a1491 = match v1.clone() {
                    Adt1::C0(_, h1486, _, _, _, _, _, _, _, _, _) => {
                        h1486
                    }
                };
                let a1492 = {
                    let a1488 = match v1 {
                        Adt1::C0(_, h1487, _, _, _, _, _, _, _, _, _) => {
                            h1487
                        }
                    };
                    let a1489 = 4u64;
                    let a1490 = 0u64;
                    nat_rem(a1488, a1489, a1490)
                };
                nat_sub(a1491, a1492)
            };
            nat_lt(a1493, a1494)
        } else {
            false
        }
    } else {
        false
    } {
        7u64
    } else {
        v0
    }
}
