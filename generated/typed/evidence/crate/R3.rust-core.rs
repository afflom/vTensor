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
                let a54 = 0u64;
                let a55 = f20(v1);
                nat_lt(a54, a55)
            } else {
                false
            } {
                true
            } else {
                let a56 = v0;
                let a57 = 0u64;
                if if nat_eq(a56, a57) {
                    let a58 = f21(v1);
                    let a59 = 2u64;
                    nat_eq(a58, a59)
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
    let a62 = match v1.clone() {
        Adt1::C0(_, h60, _, _, _, _, _, _, _, _, _) => {
            h60
        }
    };
    let a63 = match v2.clone() {
        Adt1::C0(_, h61, _, _, _, _, _, _, _, _, _) => {
            h61
        }
    };
    if if if if if if if if if if if if if if nat_eq(a62, a63) {
        true
    } else {
        let a66 = match v1.clone() {
            Adt1::C0(h64, _, _, _, _, _, _, _, _, _, _) => {
                h64
            }
        };
        let a67 = match v3.clone() {
            Adt1::C0(h65, _, _, _, _, _, _, _, _, _, _) => {
                h65
            }
        };
        nat_eq(a66, a67)
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
            Adt1::C0(h68, _, _, _, _, _, _, _, _, _, _) => {
                h68
            }
        }, v2.clone(), match v3.clone() {
            Adt1::C0(h69, _, _, _, _, _, _, _, _, _, _) => {
                h69
            }
        })
    } else {
        false
    } {
        f26(match v1.clone() {
            Adt1::C0(h70, _, _, _, _, _, _, _, _, _, _) => {
                h70
            }
        }, match v3.clone() {
            Adt1::C0(h71, _, _, _, _, _, _, _, _, _, _) => {
                h71
            }
        })
    } else {
        false
    } {
        let a73 = 0u64;
        let a74 = f27(match v3.clone() {
            Adt1::C0(h72, _, _, _, _, _, _, _, _, _, _) => {
                h72
            }
        });
        nat_lt(a73, a74)
    } else {
        false
    } {
        let a77 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h75, _, _, _, _, _) => {
                h75
            }
        };
        let a78 = f28(match v1.clone() {
            Adt1::C0(h76, _, _, _, _, _, _, _, _, _, _) => {
                h76
            }
        });
        nat_eq(a77, a78)
    } else {
        false
    } {
        let a81 = match v3.clone() {
            Adt1::C0(_, _, _, _, _, h79, _, _, _, _, _) => {
                h79
            }
        };
        let a82 = f27(match v3.clone() {
            Adt1::C0(h80, _, _, _, _, _, _, _, _, _, _) => {
                h80
            }
        });
        nat_eq(a81, a82)
    } else {
        false
    } {
        let a88 = {
            let a85 = match v1.clone() {
                Adt1::C0(_, h83, _, _, _, _, _, _, _, _, _) => {
                    h83
                }
            };
            let a86 = f29(match v1.clone() {
                Adt1::C0(h84, _, _, _, _, _, _, _, _, _, _) => {
                    h84
                }
            });
            let a87 = 1u64;
            nat_rem(a85, a86, a87)
        };
        let a89 = 0u64;
        nat_eq(a88, a89)
    } else {
        false
    } {
        let a95 = {
            let a92 = match v3.clone() {
                Adt1::C0(_, h90, _, _, _, _, _, _, _, _, _) => {
                    h90
                }
            };
            let a93 = f29(match v3.clone() {
                Adt1::C0(h91, _, _, _, _, _, _, _, _, _, _) => {
                    h91
                }
            });
            let a94 = 1u64;
            nat_rem(a92, a93, a94)
        };
        let a96 = 0u64;
        nat_eq(a95, a96)
    } else {
        false
    } {
        f30(v1.clone(), v3.clone())?
    } else {
        false
    } {
        f30(v2.clone(), v3.clone())?
    } else {
        false
    } {
        f31({
            let a103 = {
                let a99 = match v3.clone() {
                    Adt1::C0(_, h97, _, _, _, _, _, _, _, _, _) => {
                        h97
                    }
                };
                let a100 = f29(match v3.clone() {
                    Adt1::C0(h98, _, _, _, _, _, _, _, _, _, _) => {
                        h98
                    }
                });
                let a101 = 0u64;
                nat_quot(a99, a100, a101)
            };
            let a104 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h102, _, _, _, _, _) => {
                    h102
                }
            };
            nat_mul(a103, a104)?
        }, match v3.clone() {
            Adt1::C0(_, _, h105, _, _, _, _, _, _, _, _) => {
                h105
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, h106, _, _, _, _, _, _, _) => {
                h106
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, h107, _, _, _, _, _, _) => {
                h107
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, _, _, h108, _, _, _, _) => {
                h108
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h109, _, _, _) => {
                h109
            }
        }, match v3.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h110, _, _) => {
                h110
            }
        })?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0(if f32(v1.clone()) {
            0u64
        } else {
            let a116 = match v1.clone() {
                Adt1::C0(_, _, h111, _, _, _, _, _, _, _, _) => {
                    h111
                }
            };
            let a117 = {
                let a114 = match v1.clone() {
                    Adt1::C0(_, _, _, h112, _, _, _, _, _, _, _) => {
                        h112
                    }
                };
                let a115 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h113, _, _, _, _, _, _) => {
                        h113
                    }
                };
                nat_mul(a114, a115)?
            };
            nat_mul(a116, a117)?
        }, 3u64, f33(match v3.clone() {
            Adt1::C0(h118, _, _, _, _, _, _, _, _, _, _) => {
                h118
            }
        }), v0, 0u64, 0u64, 0u64, v1.clone(), v2, v3, true, match v1 {
            Adt1::C0(_, h119, _, _, _, _, _, _, _, _, _) => {
                h119
            }
        })))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f4(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if f24(v1.clone(), v3.clone()) {
        f34(v0, v1.clone(), v2.clone(), v3.clone())?
    } else {
        false
    } {
        if f32(v1.clone()) {
            true
        } else {
            let a122 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, h120, _, _, _, _, _) => {
                    h120
                }
            };
            let a123 = f20(match v1.clone() {
                Adt1::C0(h121, _, _, _, _, _, _, _, _, _, _) => {
                    h121
                }
            });
            if if if if if if if nat_eq(a122, a123) {
                let a126 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, h124, _, _, _, _, _) => {
                        h124
                    }
                };
                let a127 = f20(match v3.clone() {
                    Adt1::C0(h125, _, _, _, _, _, _, _, _, _, _) => {
                        h125
                    }
                });
                nat_eq(a126, a127)
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
                f30(v1.clone(), v3.clone())?
            } else {
                false
            } {
                f30(v2.clone(), v3.clone())?
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v4: u64 = if if f32(v1.clone()) {
            true
        } else {
            f37(v3.clone())?
        } {
            0u64
        } else {
            let a130 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h128, _) => {
                    h128
                }
            };
            let a131 = f20(match v3.clone() {
                Adt1::C0(h129, _, _, _, _, _, _, _, _, _, _) => {
                    h129
                }
            });
            let a132 = 0u64;
            nat_quot(a130, a131, a132)
        };
        Ok(Adt3::C2(Adt2::C0(f38(f39(v1.clone())?, v4, f40(match v1.clone() {
            Adt1::C0(h133, _, _, _, _, _, _, _, _, _, _) => {
                h133
            }
        }, match v2.clone() {
            Adt1::C0(h134, _, _, _, _, _, _, _, _, _, _) => {
                h134
            }
        }, match v3.clone() {
            Adt1::C0(h135, _, _, _, _, _, _, _, _, _, _) => {
                h135
            }
        })?)?, {
            let a136 = 3u64;
            let a137 = v4;
            nat_add(a136, a137)?
        }, None::<u64>, v0, 0u64, 0u64, 0u64, v1, v2, v3, false, 0u64)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f5(v0: Adt1, v1: Adt1, v2: Adt1, v3: Adt1) -> R<Adt3> {
    if if if if if if if if if if if if if if if f41(v2, v3.clone()) {
        let a139 = match v0.clone() {
            Adt1::C0(h138, _, _, _, _, _, _, _, _, _, _) => {
                h138
            }
        };
        let a140 = 0u64;
        if nat_eq(a139, a140) {
            true
        } else {
            let a142 = match v0.clone() {
                Adt1::C0(h141, _, _, _, _, _, _, _, _, _, _) => {
                    h141
                }
            };
            let a143 = 1u64;
            nat_eq(a142, a143)
        }
    } else {
        false
    } {
        let a145 = 0u64;
        let a146 = f28(match v3.clone() {
            Adt1::C0(h144, _, _, _, _, _, _, _, _, _, _) => {
                h144
            }
        });
        nat_lt(a145, a146)
    } else {
        false
    } {
        let a148 = 0u64;
        let a149 = f42(match v1.clone() {
            Adt1::C0(h147, _, _, _, _, _, _, _, _, _, _) => {
                h147
            }
        });
        nat_lt(a148, a149)
    } else {
        false
    } {
        let a152 = match v0.clone() {
            Adt1::C0(_, h150, _, _, _, _, _, _, _, _, _) => {
                h150
            }
        };
        let a153 = match v3.clone() {
            Adt1::C0(_, h151, _, _, _, _, _, _, _, _, _) => {
                h151
            }
        };
        if nat_eq(a152, a153) {
            let a159 = {
                let a156 = match v0.clone() {
                    Adt1::C0(_, h154, _, _, _, _, _, _, _, _, _) => {
                        h154
                    }
                };
                let a157 = f29(match v3.clone() {
                    Adt1::C0(h155, _, _, _, _, _, _, _, _, _, _) => {
                        h155
                    }
                });
                let a158 = 1u64;
                nat_rem(a156, a157, a158)
            };
            let a160 = 0u64;
            nat_eq(a159, a160)
        } else {
            false
        }
    } else {
        false
    } {
        let a163 = match v0.clone() {
            Adt1::C0(_, _, _, h161, _, _, _, _, _, _, _) => {
                h161
            }
        };
        let a164 = match v3.clone() {
            Adt1::C0(_, _, _, h162, _, _, _, _, _, _, _) => {
                h162
            }
        };
        nat_eq(a163, a164)
    } else {
        false
    } {
        let a167 = match v0.clone() {
            Adt1::C0(_, _, _, _, h165, _, _, _, _, _, _) => {
                h165
            }
        };
        let a168 = match v3.clone() {
            Adt1::C0(_, _, _, _, h166, _, _, _, _, _, _) => {
                h166
            }
        };
        nat_eq(a167, a168)
    } else {
        false
    } {
        let a171 = match v0.clone() {
            Adt1::C0(_, _, h169, _, _, _, _, _, _, _, _) => {
                h169
            }
        };
        let a172 = match v1.clone() {
            Adt1::C0(_, h170, _, _, _, _, _, _, _, _, _) => {
                h170
            }
        };
        nat_eq(a171, a172)
    } else {
        false
    } {
        let a174 = match v1.clone() {
            Adt1::C0(_, _, _, _, h173, _, _, _, _, _, _) => {
                h173
            }
        };
        let a175 = 1u64;
        nat_eq(a174, a175)
    } else {
        false
    } {
        let a177 = 0u64;
        let a178 = match v1.clone() {
            Adt1::C0(_, _, h176, _, _, _, _, _, _, _, _) => {
                h176
            }
        };
        nat_lt(a177, a178)
    } else {
        false
    } {
        let a180 = 0u64;
        let a181 = match v1.clone() {
            Adt1::C0(_, _, _, h179, _, _, _, _, _, _, _) => {
                h179
            }
        };
        nat_lt(a180, a181)
    } else {
        false
    } {
        let a187 = {
            let a184 = match v0.clone() {
                Adt1::C0(_, _, _, h182, _, _, _, _, _, _, _) => {
                    h182
                }
            };
            let a185 = match v1.clone() {
                Adt1::C0(_, _, h183, _, _, _, _, _, _, _, _) => {
                    h183
                }
            };
            let a186 = 0u64;
            nat_rem(a184, a185, a186)
        };
        let a188 = 0u64;
        nat_eq(a187, a188)
    } else {
        false
    } {
        let a194 = {
            let a191 = match v0.clone() {
                Adt1::C0(_, _, _, _, h189, _, _, _, _, _, _) => {
                    h189
                }
            };
            let a192 = match v1.clone() {
                Adt1::C0(_, _, _, h190, _, _, _, _, _, _, _) => {
                    h190
                }
            };
            let a193 = 0u64;
            nat_rem(a191, a192, a193)
        };
        let a195 = 0u64;
        nat_eq(a194, a195)
    } else {
        false
    } {
        f43(v0.clone())
    } else {
        false
    } {
        let a197 = match v3.clone() {
            Adt1::C0(_, h196, _, _, _, _, _, _, _, _, _) => {
                h196
            }
        };
        let a198 = 1u64;
        if nat_eq(a197, a198) {
            true
        } else {
            let a201 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h199, _, _, _, _, _) => {
                    h199
                }
            };
            let a202 = f28(match v3.clone() {
                Adt1::C0(h200, _, _, _, _, _, _, _, _, _, _) => {
                    h200
                }
            });
            nat_eq(a201, a202)
        }
    } else {
        false
    } {
        let v4: u64 = f39(v1.clone())?;
        let v5: u64 = {
            let a208 = match v0.clone() {
                Adt1::C0(_, _, h203, _, _, _, _, _, _, _, _) => {
                    h203
                }
            };
            let a209 = {
                let a206 = match v0.clone() {
                    Adt1::C0(_, _, _, h204, _, _, _, _, _, _, _) => {
                        h204
                    }
                };
                let a207 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h205, _, _, _, _, _, _) => {
                        h205
                    }
                };
                nat_mul(a206, a207)?
            };
            nat_mul(a208, a209)?
        };
        let v6: u64 = if f32(v0.clone()) {
            0u64
        } else {
            let a217 = {
                let a210 = 2u64;
                let a211 = v4;
                nat_mul(a210, a211)?
            };
            let a218 = {
                let a215 = v5;
                let a216 = {
                    let a213 = 1u64;
                    let a214 = f44(v0.clone(), match v3.clone() {
                        Adt1::C0(h212, _, _, _, _, _, _, _, _, _, _) => {
                            h212
                        }
                    })?;
                    nat_add(a213, a214)?
                };
                nat_mul(a215, a216)?
            };
            nat_add(a217, a218)?
        };
        Ok(Adt3::C2(Adt2::C0(v6, if f32(v0.clone()) {
            3u64
        } else {
            let a219 = 3u64;
            let a220 = v4;
            nat_add(a219, a220)?
        }, f33(match v3.clone() {
            Adt1::C0(h221, _, _, _, _, _, _, _, _, _, _) => {
                h221
            }
        }), 41u64, 0u64, 0u64, 0u64, v0, v1, v3.clone(), false, f45(match v3 {
            Adt1::C0(h222, _, _, _, _, _, _, _, _, _, _) => {
                h222
            }
        }))))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f6(v0: Adt1, v1: Adt1, v2: Adt1) -> R<Adt3> {
    let v3: u64 = f29(match v2.clone() {
        Adt1::C0(h223, _, _, _, _, _, _, _, _, _, _) => {
            h223
        }
    });
    if if if if if if if f18(match v2.clone() {
        Adt1::C0(h224, _, _, _, _, _, _, _, _, _, _) => {
            h224
        }
    }) {
        f41(v1.clone(), v2.clone())
    } else {
        false
    } {
        let a226 = 0u64;
        let a227 = f21(match v0.clone() {
            Adt1::C0(h225, _, _, _, _, _, _, _, _, _, _) => {
                h225
            }
        });
        nat_lt(a226, a227)
    } else {
        false
    } {
        let a230 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h228, _, _, _, _, _) => {
                h228
            }
        };
        let a231 = f21(match v0.clone() {
            Adt1::C0(h229, _, _, _, _, _, _, _, _, _, _) => {
                h229
            }
        });
        nat_eq(a230, a231)
    } else {
        false
    } {
        let a236 = {
            let a233 = match v0.clone() {
                Adt1::C0(_, h232, _, _, _, _, _, _, _, _, _) => {
                    h232
                }
            };
            let a234 = v3;
            let a235 = 1u64;
            nat_rem(a233, a234, a235)
        };
        let a237 = 0u64;
        if nat_eq(a236, a237) {
            let a242 = {
                let a239 = match v2.clone() {
                    Adt1::C0(_, h238, _, _, _, _, _, _, _, _, _) => {
                        h238
                    }
                };
                let a240 = v3;
                let a241 = 1u64;
                nat_rem(a239, a240, a241)
            };
            let a243 = 0u64;
            nat_eq(a242, a243)
        } else {
            false
        }
    } else {
        false
    } {
        let a246 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h244, _, _, _, _, _) => {
                h244
            }
        };
        let a247 = f28(match v2.clone() {
            Adt1::C0(h245, _, _, _, _, _, _, _, _, _, _) => {
                h245
            }
        });
        nat_eq(a246, a247)
    } else {
        false
    } {
        let a248 = f39(v0.clone())?;
        let a249 = f39(v2.clone())?;
        nat_eq(a248, a249)
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a250 = f39(v0.clone())?;
            let a251 = v3;
            let a252 = 0u64;
            nat_quot(a250, a251, a252)
        }, 3u64, f33(match v2.clone() {
            Adt1::C0(h253, _, _, _, _, _, _, _, _, _, _) => {
                h253
            }
        }), 33u64, 0u64, 0u64, 0u64, v0, v1, v2, false, v3)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f7(v0: u64, v1: u64, v2: Adt1, v3: Adt1) -> R<Adt3> {
    let a255 = match v2.clone() {
        Adt1::C0(h254, _, _, _, _, _, _, _, _, _, _) => {
            h254
        }
    };
    let a256 = 0u64;
    if if if if if if if if if if if nat_eq(a255, a256) {
        let a258 = match v3.clone() {
            Adt1::C0(h257, _, _, _, _, _, _, _, _, _, _) => {
                h257
            }
        };
        let a259 = 26u64;
        nat_eq(a258, a259)
    } else {
        false
    } {
        let a261 = match v2.clone() {
            Adt1::C0(_, h260, _, _, _, _, _, _, _, _, _) => {
                h260
            }
        };
        let a262 = 2147483647u64;
        nat_le(a261, a262)
    } else {
        false
    } {
        let a265 = match v3.clone() {
            Adt1::C0(_, h263, _, _, _, _, _, _, _, _, _) => {
                h263
            }
        };
        let a266 = match v2.clone() {
            Adt1::C0(_, h264, _, _, _, _, _, _, _, _, _) => {
                h264
            }
        };
        nat_le(a265, a266)
    } else {
        false
    } {
        let a269 = match v2.clone() {
            Adt1::C0(_, _, h267, _, _, _, _, _, _, _, _) => {
                h267
            }
        };
        let a270 = match v3.clone() {
            Adt1::C0(_, _, h268, _, _, _, _, _, _, _, _) => {
                h268
            }
        };
        nat_eq(a269, a270)
    } else {
        false
    } {
        let a273 = match v2.clone() {
            Adt1::C0(_, _, _, h271, _, _, _, _, _, _, _) => {
                h271
            }
        };
        let a274 = match v3.clone() {
            Adt1::C0(_, _, _, h272, _, _, _, _, _, _, _) => {
                h272
            }
        };
        nat_eq(a273, a274)
    } else {
        false
    } {
        let a277 = match v2.clone() {
            Adt1::C0(_, _, _, _, h275, _, _, _, _, _, _) => {
                h275
            }
        };
        let a278 = match v3.clone() {
            Adt1::C0(_, _, _, _, h276, _, _, _, _, _, _) => {
                h276
            }
        };
        nat_eq(a277, a278)
    } else {
        false
    } {
        let a279 = v0;
        let a280 = 67u64;
        if nat_eq(a279, a280) {
            true
        } else {
            let a281 = v1;
            let a282 = 1u64;
            if nat_le(a281, a282) {
                let a285 = match v3.clone() {
                    Adt1::C0(_, h283, _, _, _, _, _, _, _, _, _) => {
                        h283
                    }
                };
                let a286 = match v2.clone() {
                    Adt1::C0(_, h284, _, _, _, _, _, _, _, _, _) => {
                        h284
                    }
                };
                nat_eq(a285, a286)
            } else {
                false
            }
        }
    } else {
        false
    } {
        f46(v2.clone())?
    } else {
        false
    } {
        f46(v3.clone())?
    } else {
        false
    } {
        f47(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a292 = match v3.clone() {
                Adt1::C0(_, _, h287, _, _, _, _, _, _, _, _) => {
                    h287
                }
            };
            let a293 = {
                let a290 = match v3.clone() {
                    Adt1::C0(_, _, _, h288, _, _, _, _, _, _, _) => {
                        h288
                    }
                };
                let a291 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h289, _, _, _, _, _, _) => {
                        h289
                    }
                };
                nat_mul(a290, a291)?
            };
            nat_mul(a292, a293)?
        }, 3u64, None::<u64>, v0, {
            let a294 = v0;
            let a295 = 67u64;
            if nat_eq(a294, a295) {
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
    let a296 = v0;
    let a297 = 6u64;
    nat_le(a296, a297)
}

pub fn f9(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1) -> R<Adt3> {
    let a298 = v2;
    let a299 = 1u64;
    if if if if if if if if if if if if if if if nat_le(a298, a299) {
        let a301 = 0u64;
        let a302 = f21(match v5.clone() {
            Adt1::C0(h300, _, _, _, _, _, _, _, _, _, _) => {
                h300
            }
        });
        nat_lt(a301, a302)
    } else {
        false
    } {
        let a305 = match v5.clone() {
            Adt1::C0(h303, _, _, _, _, _, _, _, _, _, _) => {
                h303
            }
        };
        let a306 = match v6.clone() {
            Adt1::C0(h304, _, _, _, _, _, _, _, _, _, _) => {
                h304
            }
        };
        nat_eq(a305, a306)
    } else {
        false
    } {
        let a309 = match v5.clone() {
            Adt1::C0(h307, _, _, _, _, _, _, _, _, _, _) => {
                h307
            }
        };
        let a310 = match v7.clone() {
            Adt1::C0(h308, _, _, _, _, _, _, _, _, _, _) => {
                h308
            }
        };
        nat_eq(a309, a310)
    } else {
        false
    } {
        f24(v5.clone(), v6.clone())
    } else {
        false
    } {
        f48(v5.clone())?
    } else {
        false
    } {
        f48(v6.clone())?
    } else {
        false
    } {
        f48(v7.clone())?
    } else {
        false
    } {
        let a313 = match v5.clone() {
            Adt1::C0(_, _, h311, _, _, _, _, _, _, _, _) => {
                h311
            }
        };
        let a314 = match v7.clone() {
            Adt1::C0(_, _, h312, _, _, _, _, _, _, _, _) => {
                h312
            }
        };
        nat_eq(a313, a314)
    } else {
        false
    } {
        let a317 = match v5.clone() {
            Adt1::C0(_, _, _, h315, _, _, _, _, _, _, _) => {
                h315
            }
        };
        let a318 = match v7.clone() {
            Adt1::C0(_, _, _, h316, _, _, _, _, _, _, _) => {
                h316
            }
        };
        nat_eq(a317, a318)
    } else {
        false
    } {
        let a321 = match v5.clone() {
            Adt1::C0(_, _, _, _, h319, _, _, _, _, _, _) => {
                h319
            }
        };
        let a322 = match v7.clone() {
            Adt1::C0(_, _, _, _, h320, _, _, _, _, _, _) => {
                h320
            }
        };
        nat_eq(a321, a322)
    } else {
        false
    } {
        let a329 = match v7.clone() {
            Adt1::C0(_, h323, _, _, _, _, _, _, _, _, _) => {
                h323
            }
        };
        let a330 = if v1 {
            match v5.clone() {
                Adt1::C0(_, h324, _, _, _, _, _, _, _, _, _) => {
                    h324
                }
            }
        } else {
            let a326 = match v5.clone() {
                Adt1::C0(_, h325, _, _, _, _, _, _, _, _, _) => {
                    h325
                }
            };
            let a327 = 2u64;
            let a328 = 0u64;
            nat_quot(a326, a327, a328)
        };
        nat_eq(a329, a330)
    } else {
        false
    } {
        let a332 = match v7.clone() {
            Adt1::C0(_, h331, _, _, _, _, _, _, _, _, _) => {
                h331
            }
        };
        let a333 = 2147483647u64;
        nat_le(a332, a333)
    } else {
        false
    } {
        f30(v5.clone(), v7.clone())?
    } else {
        false
    } {
        f30(v6.clone(), v7.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0(if f14(v0) {
            let a336 = 1u64;
            let a337 = {
                let a334 = f49(v0);
                let a335 = f39(v7.clone())?;
                nat_mul(a334, a335)?
            };
            nat_add(a336, a337)?
        } else {
            let a338 = 4u64;
            let a339 = f39(v7.clone())?;
            nat_mul(a338, a339)?
        }, if f14(v0) {
            4u64
        } else {
            3u64
        }, None::<u64>, if v1 {
            98u64
        } else {
            let a340 = 99u64;
            let a341 = v2;
            nat_add(a340, a341)?
        }, v0, v3, v4, v5, v6, v7, false, 0u64)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f10(v0: u64, v1: Adt2) -> R<Adt3> {
    let a343 = v0;
    let a344 = match v1.clone() {
        Adt2::C0(h342, _, _, _, _, _, _, _, _, _, _, _) => {
            h342
        }
    };
    if nat_lt(a343, a344) {
        let v2: u64 = {
            let a348 = v0;
            let a349 = {
                let m347 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h346, _, _, _, _) => {
                        h346
                    }
                };
                match m347 {
                    Adt1::C0(_, h345, _, _, _, _, _, _, _, _, _) => {
                        h345
                    }
                }
            };
            nat_mul(a348, a349)?
        };
        Ok(Adt3::C7(f50(v2, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h350, _, _, _, _) => {
                h350
            }
        })?, f51(v2, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h351, _, _, _, _) => {
                h351
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h352, _, _, _) => {
                h352
            }
        })?, f50(v2, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h353, _, _) => {
                h353
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f11(v0: u64, v1: Adt2) -> R<Adt3> {
    let a355 = v0;
    let a356 = match v1.clone() {
        Adt2::C0(h354, _, _, _, _, _, _, _, _, _, _, _) => {
            h354
        }
    };
    if nat_lt(a355, a356) {
        let v2: Adt1 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h357, _, _) => {
                h357
            }
        };
        let v3: u64 = f29(match v2.clone() {
            Adt1::C0(h358, _, _, _, _, _, _, _, _, _, _) => {
                h358
            }
        });
        let v4: u64 = {
            let a360 = match v2.clone() {
                Adt1::C0(_, h359, _, _, _, _, _, _, _, _, _) => {
                    h359
                }
            };
            let a361 = v3;
            let a362 = 0u64;
            nat_quot(a360, a361, a362)
        };
        let v5: u64 = {
            let a363 = v0;
            let a364 = v4;
            let a365 = 0u64;
            nat_quot(a363, a364, a365)
        };
        let v6: u64 = {
            let a367 = v5;
            let a368 = match v2.clone() {
                Adt1::C0(_, _, h366, _, _, _, _, _, _, _, _) => {
                    h366
                }
            };
            let a369 = 0u64;
            nat_quot(a367, a368, a369)
        };
        Ok(Adt3::C6(f50({
            let a370 = v0;
            let a371 = v3;
            nat_mul(a370, a371)?
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, h372, _, _, _, _) => {
                h372
            }
        })?, f52({
            let a373 = v0;
            let a374 = v4;
            let a375 = 0u64;
            nat_rem(a373, a374, a375)
        }, {
            let a377 = v5;
            let a378 = match v2.clone() {
                Adt1::C0(_, _, h376, _, _, _, _, _, _, _, _) => {
                    h376
                }
            };
            let a379 = 0u64;
            nat_rem(a377, a378, a379)
        }, {
            let a381 = v6;
            let a382 = match v2.clone() {
                Adt1::C0(_, _, _, h380, _, _, _, _, _, _, _) => {
                    h380
                }
            };
            let a383 = 0u64;
            nat_rem(a381, a382, a383)
        }, {
            let a385 = v6;
            let a386 = match v2.clone() {
                Adt1::C0(_, _, _, h384, _, _, _, _, _, _, _) => {
                    h384
                }
            };
            let a387 = 0u64;
            nat_quot(a385, a386, a387)
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, h388, _, _, _, _, _) => {
                h388
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h389, _, _, _, _) => {
                h389
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h390, _, _, _) => {
                h390
            }
        }, match v2 {
            Adt1::C0(_, _, _, _, _, _, _, _, h391, _, _) => {
                h391
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f12(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h392, _, _, _, _) => {
            h392
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h393, _, _) => {
            h393
        }
    };
    let a395 = v0;
    let a396 = match v1.clone() {
        Adt2::C0(h394, _, _, _, _, _, _, _, _, _, _, _) => {
            h394
        }
    };
    if nat_lt(a395, a396) {
        Ok(Adt3::C5({
            let a398 = v0;
            let a399 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, _, h397, _, _, _, _) => {
                    h397
                }
            };
            nat_mul(a398, a399)?
        }, {
            let a401 = v0;
            let a402 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h400, _, _, _, _) => {
                    h400
                }
            };
            nat_mul(a401, a402)?
        }, match v2 {
            Adt1::C0(_, h403, _, _, _, _, _, _, _, _, _) => {
                h403
            }
        }, match v3 {
            Adt1::C0(_, h404, _, _, _, _, _, _, _, _, _) => {
                h404
            }
        }, {
            let a406 = match v1 {
                Adt2::C0(_, _, _, _, h405, _, _, _, _, _, _, _) => {
                    h405
                }
            };
            let a407 = 1u64;
            nat_eq(a406, a407)
        }))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f13(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let a409 = v0;
    let a410 = match v1.clone() {
        Adt2::C0(h408, _, _, _, _, _, _, _, _, _, _, _) => {
            h408
        }
    };
    if nat_lt(a409, a410) {
        let v3: u64 = {
            let a412 = 2u64;
            let a413 = f39(match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h411, _, _, _) => {
                    h411
                }
            })?;
            nat_mul(a412, a413)?
        };
        let a414 = v0;
        let a415 = v3;
        if nat_lt(a414, a415) {
            f53(v0, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h416, _, _, _) => {
                    h416
                }
            }, {
                let m419 = match v1 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h418, _, _) => {
                        h418
                    }
                };
                match m419 {
                    Adt1::C0(_, _, h417, _, _, _, _, _, _, _, _) => {
                        h417
                    }
                }
            }, v2)
        } else {
            f54({
                let a420 = v0;
                let a421 = v3;
                nat_sub(a420, a421)
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h422, _, _, _, _) => {
                    h422
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h423, _, _, _) => {
                    h423
                }
            }, match v1 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h424, _, _) => {
                    h424
                }
            }, v2)
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f14(v0: u64) -> bool {
    let a425 = v0;
    let a426 = 3u64;
    if nat_eq(a425, a426) {
        true
    } else {
        let a427 = v0;
        let a428 = 6u64;
        nat_eq(a427, a428)
    }
}

pub fn f15(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h429, _, _, _, _, _) => {
            h429
        }
    };
    let a431 = v0;
    let a432 = match v1.clone() {
        Adt2::C0(h430, _, _, _, _, _, _, _, _, _, _, _) => {
            h430
        }
    };
    if nat_lt(a431, a432) {
        let a433 = v0;
        let a434 = 0u64;
        if nat_eq(a433, a434) {
            Ok(Adt3::C3(1u64, 6u64, 0u64, 0u64, 2u64, v3, 2u64, 0u64, 0u64, 3u64, 4u64, 4u64))
        } else {
            let v4: u64 = {
                let a435 = v0;
                let a436 = 1u64;
                nat_sub(a435, a436)
            };
            let v5: u64 = {
                let a438 = v4;
                let a439 = f49(match v1.clone() {
                    Adt2::C0(_, _, _, _, h437, _, _, _, _, _, _, _) => {
                        h437
                    }
                });
                let a440 = 0u64;
                nat_rem(a438, a439, a440)
            };
            let v6: u64 = {
                let a445 = 4u64;
                let a446 = {
                    let a442 = v4;
                    let a443 = f49(match v1.clone() {
                        Adt2::C0(_, _, _, _, h441, _, _, _, _, _, _, _) => {
                            h441
                        }
                    });
                    let a444 = 0u64;
                    nat_quot(a442, a443, a444)
                };
                nat_mul(a445, a446)?
            };
            let a447 = v5;
            let a448 = 2u64;
            if nat_lt(a447, a448) {
                f16({
                    let a449 = v6;
                    let a450 = v5;
                    nat_add(a449, a450)?
                }, v1)
            } else {
                let a451 = v5;
                let a452 = 2u64;
                if nat_eq(a451, a452) {
                    Ok(f55(0u64, 2u64, v3))
                } else {
                    let a453 = v5;
                    let a454 = 3u64;
                    if nat_eq(a453, a454) {
                        Ok(f56(0u64, 2u64, v3, {
                            let a455 = v2;
                            let a456 = 2u64;
                            nat_eq(a455, a456)
                        }))
                    } else {
                        let a457 = v5;
                        let a458 = 4u64;
                        if nat_eq(a457, a458) {
                            Ok(f55(1u64, 1u64, 3u64))
                        } else {
                            let a459 = v5;
                            let a460 = 5u64;
                            if nat_eq(a459, a460) {
                                Ok(f56(1u64, 1u64, 3u64, {
                                    let a461 = v2;
                                    let a462 = 0u64;
                                    nat_eq(a461, a462)
                                }))
                            } else {
                                let a463 = v5;
                                let a464 = 6u64;
                                if nat_eq(a463, a464) {
                                    Ok(f55(1u64, 2u64, v3))
                                } else {
                                    let a465 = v5;
                                    let a466 = 7u64;
                                    if nat_eq(a465, a466) {
                                        Ok(f56(1u64, 2u64, v3, {
                                            let a467 = v2;
                                            let a468 = 2u64;
                                            nat_eq(a467, a468)
                                        }))
                                    } else {
                                        let a470 = match v1.clone() {
                                            Adt2::C0(_, _, _, _, h469, _, _, _, _, _, _, _) => {
                                                h469
                                            }
                                        };
                                        let a471 = 3u64;
                                        if nat_eq(a470, a471) {
                                            let a472 = v5;
                                            let a473 = 15u64;
                                            if nat_eq(a472, a473) {
                                                f16({
                                                    let a474 = v6;
                                                    let a475 = 3u64;
                                                    nat_add(a474, a475)?
                                                }, v1)
                                            } else {
                                                Ok(f57(v5, match v1 {
                                                    Adt2::C0(_, _, _, _, _, h476, _, _, _, _, _, _) => {
                                                        h476
                                                    }
                                                }))
                                            }
                                        } else {
                                            let a477 = v5;
                                            let a478 = 8u64;
                                            if nat_eq(a477, a478) {
                                                Ok(Adt3::C4(2u64, {
                                                    let m481 = match v1 {
                                                        Adt2::C0(_, _, _, _, _, _, _, _, _, h480, _, _) => {
                                                            h480
                                                        }
                                                    };
                                                    match m481 {
                                                        Adt1::C0(h479, _, _, _, _, _, _, _, _, _, _) => {
                                                            h479
                                                        }
                                                    }
                                                }, 0u64, 1u64, 0u64))
                                            } else {
                                                f16({
                                                    let a482 = v6;
                                                    let a483 = 3u64;
                                                    nat_add(a482, a483)?
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
        Adt2::C0(_, _, _, _, _, _, _, h484, _, _, _, _) => {
            h484
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h485, _, _, _) => {
            h485
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h486, _, _) => {
            h486
        }
    };
    let v5: u64 = match v1.clone() {
        Adt2::C0(_, _, _, h487, _, _, _, _, _, _, _, _) => {
            h487
        }
    };
    let v6: u64 = {
        let a488 = v0;
        let a489 = 4u64;
        let a490 = 0u64;
        nat_rem(a488, a489, a490)
    };
    let v7: u64 = {
        let a491 = v0;
        let a492 = 4u64;
        let a493 = 0u64;
        nat_quot(a491, a492, a493)
    };
    let v8: u64 = {
        let a495 = v7;
        let a496 = match v4.clone() {
            Adt1::C0(_, h494, _, _, _, _, _, _, _, _, _) => {
                h494
            }
        };
        let a497 = 0u64;
        nat_quot(a495, a496, a497)
    };
    let v9: u64 = f21(match v4.clone() {
        Adt1::C0(h498, _, _, _, _, _, _, _, _, _, _) => {
            h498
        }
    });
    let v10: u64 = {
        let a503 = {
            let a500 = v7;
            let a501 = match v4.clone() {
                Adt1::C0(_, h499, _, _, _, _, _, _, _, _, _) => {
                    h499
                }
            };
            let a502 = 0u64;
            nat_rem(a500, a501, a502)
        };
        let a504 = v9;
        nat_mul(a503, a504)?
    };
    let v11: u64 = {
        let a506 = match v4.clone() {
            Adt1::C0(_, h505, _, _, _, _, _, _, _, _, _) => {
                h505
            }
        };
        let a507 = v9;
        nat_mul(a506, a507)?
    };
    let a509 = v0;
    let a510 = match v1.clone() {
        Adt2::C0(h508, _, _, _, _, _, _, _, _, _, _, _) => {
            h508
        }
    };
    if nat_lt(a509, a510) {
        let a511 = v6;
        let a512 = 0u64;
        if nat_eq(a511, a512) {
            Ok(Adt3::C3(0u64, 0u64, match v2.clone() {
                Adt1::C0(h513, _, _, _, _, _, _, _, _, _, _) => {
                    h513
                }
            }, 0u64, 0u64, {
                let a521 = {
                    let a517 = {
                        let a515 = v8;
                        let a516 = match v2 {
                            Adt1::C0(_, _, _, _, _, _, h514, _, _, _, _) => {
                                h514
                            }
                        };
                        nat_mul(a515, a516)?
                    };
                    let a518 = v10;
                    nat_add(a517, a518)?
                };
                let a522 = {
                    let a519 = v5;
                    let a520 = 100u64;
                    if nat_eq(a519, a520) {
                        v11
                    } else {
                        0u64
                    }
                };
                nat_add(a521, a522)?
            }, 2u64, 0u64, 0u64, 0u64, v9, 4u64))
        } else {
            let a523 = v6;
            let a524 = 1u64;
            if nat_eq(a523, a524) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h525, _, _, _, _, _, _, _, _, _, _) => {
                        h525
                    }
                }, 0u64, {
                    let a526 = v5;
                    let a527 = 98u64;
                    if nat_eq(a526, a527) {
                        4u64
                    } else {
                        0u64
                    }
                }, {
                    let a535 = {
                        let a531 = {
                            let a529 = v8;
                            let a530 = match v3 {
                                Adt1::C0(_, _, _, _, _, _, h528, _, _, _, _) => {
                                    h528
                                }
                            };
                            nat_mul(a529, a530)?
                        };
                        let a532 = v10;
                        nat_add(a531, a532)?
                    };
                    let a536 = {
                        let a533 = v5;
                        let a534 = 99u64;
                        if nat_eq(a533, a534) {
                            v11
                        } else {
                            0u64
                        }
                    };
                    nat_add(a535, a536)?
                }, 2u64, 0u64, 0u64, 1u64, v9, 4u64))
            } else {
                let a537 = v6;
                let a538 = 2u64;
                if nat_eq(a537, a538) {
                    Ok(Adt3::C4(match v1 {
                        Adt2::C0(_, _, _, _, h539, _, _, _, _, _, _, _) => {
                            h539
                        }
                    }, match v4 {
                        Adt1::C0(h540, _, _, _, _, _, _, _, _, _, _) => {
                            h540
                        }
                    }, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt3::C3(0u64, 0u64, 0u64, match v4.clone() {
                        Adt1::C0(h541, _, _, _, _, _, _, _, _, _, _) => {
                            h541
                        }
                    }, 1u64, 0u64, 2u64, 0u64, 3u64, {
                        let a545 = {
                            let a543 = v8;
                            let a544 = match v4 {
                                Adt1::C0(_, _, _, _, _, _, h542, _, _, _, _) => {
                                    h542
                                }
                            };
                            nat_mul(a543, a544)?
                        };
                        let a546 = v10;
                        nat_add(a545, a546)?
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
        Adt2::C0(_, _, _, _, _, _, _, h547, _, _, _, _) => {
            h547
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h548, _, _, _) => {
            h548
        }
    };
    let v5: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h549, _, _) => {
            h549
        }
    };
    let v6: u64 = {
        let a551 = match v1.clone() {
            Adt2::C0(_, h550, _, _, _, _, _, _, _, _, _, _) => {
                h550
            }
        };
        let a552 = 3u64;
        if nat_eq(a551, a552) {
            0u64
        } else {
            let a553 = f39(v3.clone())?;
            let a554 = 2u64;
            nat_mul(a553, a554)?
        }
    };
    let a556 = match v1.clone() {
        Adt2::C0(h555, _, _, _, _, _, _, _, _, _, _, _) => {
            h555
        }
    };
    let a557 = v0;
    if nat_le(a556, a557) {
        Ok(Adt3::C1)
    } else {
        let a558 = v0;
        let a559 = v6;
        if nat_lt(a558, a559) {
            let v7: u64 = {
                let a560 = v0;
                let a561 = 2u64;
                let a562 = 0u64;
                nat_quot(a560, a561, a562)
            };
            let v8: u64 = {
                let a567 = 3u64;
                let a568 = {
                    let a564 = f50(v7, v5.clone())?;
                    let a565 = f20(match v5 {
                        Adt1::C0(h563, _, _, _, _, _, _, _, _, _, _) => {
                            h563
                        }
                    });
                    let a566 = 0u64;
                    nat_quot(a564, a565, a566)
                };
                nat_add(a567, a568)?
            };
            let a572 = {
                let a569 = v0;
                let a570 = 2u64;
                let a571 = 0u64;
                nat_rem(a569, a570, a571)
            };
            let a573 = 0u64;
            if nat_eq(a572, a573) {
                Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, v8, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
            } else {
                let a574 = v2;
                let a575 = 0u64;
                if nat_eq(a574, a575) {
                    Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 2u64, 1u64, 2u64, 0u64, 0u64, v8, 8u64, 8u64))
                } else {
                    Ok(Adt3::C1)
                }
            }
        } else {
            let v9: u64 = {
                let a576 = v0;
                let a577 = v6;
                nat_sub(a576, a577)
            };
            let v10: u64 = f40(match v3.clone() {
                Adt1::C0(h578, _, _, _, _, _, _, _, _, _, _) => {
                    h578
                }
            }, match v4.clone() {
                Adt1::C0(h579, _, _, _, _, _, _, _, _, _, _) => {
                    h579
                }
            }, match v5.clone() {
                Adt1::C0(h580, _, _, _, _, _, _, _, _, _, _) => {
                    h580
                }
            })?;
            let v11: u64 = {
                let a581 = v9;
                let a582 = v10;
                let a583 = 0u64;
                nat_quot(a581, a582, a583)
            };
            let v12: u64 = {
                let a584 = v9;
                let a585 = v10;
                let a586 = 0u64;
                nat_rem(a584, a585, a586)
            };
            let v13: u64 = {
                let a588 = match v3.clone() {
                    Adt1::C0(h587, _, _, _, _, _, _, _, _, _, _) => {
                        h587
                    }
                };
                let a589 = 0u64;
                if nat_eq(a588, a589) {
                    0u64
                } else {
                    1u64
                }
            };
            let v14: u64 = {
                let a591 = match v4.clone() {
                    Adt1::C0(h590, _, _, _, _, _, _, _, _, _, _) => {
                        h590
                    }
                };
                let a592 = 0u64;
                if nat_eq(a591, a592) {
                    0u64
                } else {
                    1u64
                }
            };
            let a593 = v12;
            let a594 = v13;
            if nat_lt(a593, a594) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h595, _, _, _, _, _, _, _, _, _, _) => {
                        h595
                    }
                }, 0u64, 0u64, f50(v11, v3.clone())?, 2u64, 0u64, 0u64, 0u64, f20(match v3 {
                    Adt1::C0(h596, _, _, _, _, _, _, _, _, _, _) => {
                        h596
                    }
                }), 4u64))
            } else {
                let a599 = v12;
                let a600 = {
                    let a597 = v13;
                    let a598 = v14;
                    nat_add(a597, a598)?
                };
                if nat_lt(a599, a600) {
                    Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                        Adt1::C0(h601, _, _, _, _, _, _, _, _, _, _) => {
                            h601
                        }
                    }, 0u64, 4u64, f51(v11, v3, v4.clone())?, 2u64, 0u64, 0u64, 1u64, f20(match v4 {
                        Adt1::C0(h602, _, _, _, _, _, _, _, _, _, _) => {
                            h602
                        }
                    }), 4u64))
                } else {
                    let a605 = v12;
                    let a606 = {
                        let a603 = v13;
                        let a604 = v14;
                        nat_add(a603, a604)?
                    };
                    if nat_eq(a605, a606) {
                        Ok(Adt3::C3(1u64, match v1 {
                            Adt2::C0(_, _, _, h607, _, _, _, _, _, _, _, _) => {
                                h607
                            }
                        }, 0u64, 0u64, {
                            let a608 = v13;
                            let a609 = 0u64;
                            if nat_eq(a608, a609) {
                                0u64
                            } else {
                                1u64
                            }
                        }, {
                            let a610 = v13;
                            let a611 = 0u64;
                            if nat_eq(a610, a611) {
                                f50(v11, v3.clone())?
                            } else {
                                0u64
                            }
                        }, {
                            let a612 = v14;
                            let a613 = 0u64;
                            if nat_eq(a612, a613) {
                                4u64
                            } else {
                                1u64
                            }
                        }, {
                            let a614 = v14;
                            let a615 = 0u64;
                            if nat_eq(a614, a615) {
                                f51(v11, v3, v4)?
                            } else {
                                1u64
                            }
                        }, {
                            let a617 = match v5.clone() {
                                Adt1::C0(h616, _, _, _, _, _, _, _, _, _, _) => {
                                    h616
                                }
                            };
                            let a618 = 0u64;
                            if nat_eq(a617, a618) {
                                3u64
                            } else {
                                0u64
                            }
                        }, {
                            let a620 = match v5.clone() {
                                Adt1::C0(h619, _, _, _, _, _, _, _, _, _, _) => {
                                    h619
                                }
                            };
                            let a621 = 0u64;
                            if nat_eq(a620, a621) {
                                f50(v11, v5)?
                            } else {
                                0u64
                            }
                        }, 4u64, 4u64))
                    } else {
                        Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                            Adt1::C0(h622, _, _, _, _, _, _, _, _, _, _) => {
                                h622
                            }
                        }, 1u64, 0u64, 2u64, 0u64, 3u64, f50(v11, v5.clone())?, 4u64, f20(match v5 {
                            Adt1::C0(h623, _, _, _, _, _, _, _, _, _, _) => {
                                h623
                            }
                        })))
                    }
                }
            }
        }
    }
}

pub fn f18(v0: u64) -> bool {
    let a624 = 6u64;
    let a625 = v0;
    if if if if nat_le(a624, a625) {
        let a626 = v0;
        let a627 = 23u64;
        nat_le(a626, a627)
    } else {
        false
    } {
        true
    } else {
        let a628 = v0;
        let a629 = 2u64;
        if nat_eq(a628, a629) {
            true
        } else {
            let a630 = v0;
            let a631 = 3u64;
            nat_eq(a630, a631)
        }
    } {
        true
    } else {
        let a632 = v0;
        let a633 = 29u64;
        nat_eq(a632, a633)
    } {
        true
    } else {
        let a634 = 34u64;
        let a635 = v0;
        if if nat_le(a634, a635) {
            let a636 = v0;
            let a637 = 35u64;
            nat_le(a636, a637)
        } else {
            false
        } {
            true
        } else {
            let a638 = 39u64;
            let a639 = v0;
            if nat_le(a638, a639) {
                let a640 = v0;
                let a641 = 42u64;
                nat_le(a640, a641)
            } else {
                false
            }
        }
    }
}

pub fn f19(v0: u64, v1: u64, v2: u64) -> bool {
    let a642 = v0;
    let a643 = 0u64;
    if if if nat_eq(a642, a643) {
        let a644 = v1;
        let a645 = 0u64;
        nat_eq(a644, a645)
    } else {
        false
    } {
        let a646 = v2;
        let a647 = 0u64;
        nat_eq(a646, a647)
    } else {
        false
    } {
        true
    } else {
        let a648 = v0;
        let a649 = 1u64;
        if if nat_eq(a648, a649) {
            true
        } else {
            let a650 = v0;
            let a651 = 30u64;
            nat_eq(a650, a651)
        } {
            let a652 = v1;
            let a653 = v0;
            if if nat_eq(a652, a653) {
                let a654 = v2;
                let a655 = v0;
                nat_eq(a654, a655)
            } else {
                false
            } {
                true
            } else {
                let a656 = v1;
                let a657 = 0u64;
                if nat_eq(a656, a657) {
                    let a658 = v2;
                    let a659 = v0;
                    if nat_eq(a658, a659) {
                        true
                    } else {
                        let a660 = v2;
                        let a661 = 0u64;
                        nat_eq(a660, a661)
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

pub fn f20(v0: u64) -> u64 {
    let a662 = v0;
    let a663 = 43u64;
    if nat_eq(a662, a663) {
        1u64
    } else {
        f21(v0)
    }
}

pub fn f21(v0: u64) -> u64 {
    let a664 = v0;
    let a665 = 0u64;
    if nat_eq(a664, a665) {
        4u64
    } else {
        let a666 = v0;
        let a667 = 1u64;
        if if nat_eq(a666, a667) {
            true
        } else {
            let a668 = v0;
            let a669 = 30u64;
            nat_eq(a668, a669)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f22(v0: u64) -> bool {
    let a670 = 0u64;
    let a671 = f20(v0);
    if nat_lt(a670, a671) {
        true
    } else {
        let a672 = 24u64;
        let a673 = v0;
        if nat_le(a672, a673) {
            let a674 = v0;
            let a675 = 28u64;
            nat_le(a674, a675)
        } else {
            false
        }
    }
}

pub fn f23(v0: Adt1, v1: Adt1) -> bool {
    if f32(v1.clone()) {
        f32(v0)
    } else {
        let a681 = {
            let a678 = match v0.clone() {
                Adt1::C0(_, h676, _, _, _, _, _, _, _, _, _) => {
                    h676
                }
            };
            let a679 = match v1.clone() {
                Adt1::C0(_, h677, _, _, _, _, _, _, _, _, _) => {
                    h677
                }
            };
            let a680 = 1u64;
            nat_rem(a678, a679, a680)
        };
        let a682 = 0u64;
        if if if nat_eq(a681, a682) {
            let a688 = {
                let a685 = match v0.clone() {
                    Adt1::C0(_, _, h683, _, _, _, _, _, _, _, _) => {
                        h683
                    }
                };
                let a686 = match v1.clone() {
                    Adt1::C0(_, _, h684, _, _, _, _, _, _, _, _) => {
                        h684
                    }
                };
                let a687 = 1u64;
                nat_rem(a685, a686, a687)
            };
            let a689 = 0u64;
            nat_eq(a688, a689)
        } else {
            false
        } {
            let a695 = {
                let a692 = match v0.clone() {
                    Adt1::C0(_, _, _, h690, _, _, _, _, _, _, _) => {
                        h690
                    }
                };
                let a693 = match v1.clone() {
                    Adt1::C0(_, _, _, h691, _, _, _, _, _, _, _) => {
                        h691
                    }
                };
                let a694 = 1u64;
                nat_rem(a692, a693, a694)
            };
            let a696 = 0u64;
            nat_eq(a695, a696)
        } else {
            false
        } {
            let a702 = {
                let a699 = match v0 {
                    Adt1::C0(_, _, _, _, h697, _, _, _, _, _, _) => {
                        h697
                    }
                };
                let a700 = match v1 {
                    Adt1::C0(_, _, _, _, h698, _, _, _, _, _, _) => {
                        h698
                    }
                };
                let a701 = 1u64;
                nat_rem(a699, a700, a701)
            };
            let a703 = 0u64;
            nat_eq(a702, a703)
        } else {
            false
        }
    }
}

pub fn f24(v0: Adt1, v1: Adt1) -> bool {
    let a706 = match v0.clone() {
        Adt1::C0(_, h704, _, _, _, _, _, _, _, _, _) => {
            h704
        }
    };
    let a707 = match v1.clone() {
        Adt1::C0(_, h705, _, _, _, _, _, _, _, _, _) => {
            h705
        }
    };
    if if if nat_eq(a706, a707) {
        let a710 = match v0.clone() {
            Adt1::C0(_, _, h708, _, _, _, _, _, _, _, _) => {
                h708
            }
        };
        let a711 = match v1.clone() {
            Adt1::C0(_, _, h709, _, _, _, _, _, _, _, _) => {
                h709
            }
        };
        nat_eq(a710, a711)
    } else {
        false
    } {
        let a714 = match v0.clone() {
            Adt1::C0(_, _, _, h712, _, _, _, _, _, _, _) => {
                h712
            }
        };
        let a715 = match v1.clone() {
            Adt1::C0(_, _, _, h713, _, _, _, _, _, _, _) => {
                h713
            }
        };
        nat_eq(a714, a715)
    } else {
        false
    } {
        let a718 = match v0 {
            Adt1::C0(_, _, _, _, h716, _, _, _, _, _, _) => {
                h716
            }
        };
        let a719 = match v1 {
            Adt1::C0(_, _, _, _, h717, _, _, _, _, _, _) => {
                h717
            }
        };
        nat_eq(a718, a719)
    } else {
        false
    }
}

pub fn f25(v0: u64, v1: u64, v2: Adt1, v3: u64) -> bool {
    let v4: u64 = match v2.clone() {
        Adt1::C0(h720, _, _, _, _, _, _, _, _, _, _) => {
            h720
        }
    };
    let a721 = v1;
    let a722 = v3;
    if if nat_eq(a721, a722) {
        true
    } else {
        let a723 = v0;
        let a724 = 0u64;
        if nat_eq(a723, a724) {
            let a725 = v4;
            let a726 = 0u64;
            if if nat_eq(a725, a726) {
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
            let a728 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, h727, _, _, _, _, _) => {
                    h727
                }
            };
            let a729 = f27(v4);
            nat_eq(a728, a729)
        } else {
            false
        } {
            let a734 = {
                let a731 = match v2 {
                    Adt1::C0(_, h730, _, _, _, _, _, _, _, _, _) => {
                        h730
                    }
                };
                let a732 = f29(v4);
                let a733 = 1u64;
                nat_rem(a731, a732, a733)
            };
            let a735 = 0u64;
            nat_eq(a734, a735)
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
        let a736 = v0;
        let a737 = v1;
        if if nat_eq(a736, a737) {
            let a738 = 0u64;
            let a739 = f20(v0);
            nat_lt(a738, a739)
        } else {
            false
        } {
            true
        } else {
            let a740 = f21(v0);
            let a741 = 2u64;
            if nat_eq(a740, a741) {
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
    let a742 = 24u64;
    let a743 = v0;
    if if nat_le(a742, a743) {
        let a744 = v0;
        let a745 = 28u64;
        nat_le(a744, a745)
    } else {
        false
    } {
        let m746 = f58(v0);
        match m746 {
            Adt4::C0 => {
                0u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, _, h747) => {
                        h747
                    }
                }
            }
        }
    } else {
        f28(v0)
    }
}

pub fn f28(v0: u64) -> u64 {
    if f18(v0) {
        let m748 = f58(v0);
        match m748 {
            Adt4::C0 => {
                0u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, _, h749) => {
                        h749
                    }
                }
            }
        }
    } else {
        let a750 = v0;
        let a751 = 26u64;
        if nat_eq(a750, a751) {
            4u64
        } else {
            f20(v0)
        }
    }
}

pub fn f29(v0: u64) -> u64 {
    if f18(v0) {
        let m752 = f58(v0);
        match m752 {
            Adt4::C0 => {
                1u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, h753, _) => {
                        h753
                    }
                }
            }
        }
    } else {
        1u64
    }
}

pub fn f30(v0: Adt1, v1: Adt1) -> R<bool> {
    if f47(v0.clone(), v1.clone())? {
        Ok(true)
    } else {
        Ok(f59(v0, v1))
    }
}

pub fn f31(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f60(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f60(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f60(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f60(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f60(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f60(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f32(v0: Adt1) -> bool {
    f61(match v0.clone() {
        Adt1::C0(_, h754, _, _, _, _, _, _, _, _, _) => {
            h754
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h755, _, _, _, _, _, _, _, _) => {
            h755
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h756, _, _, _, _, _, _, _) => {
            h756
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, h757, _, _, _, _, _, _) => {
            h757
        }
    })
}

pub fn f33(v0: u64) -> Option<u64> {
    let a758 = v0;
    let a759 = 16u64;
    if if if nat_eq(a758, a759) {
        true
    } else {
        let a760 = v0;
        let a761 = 17u64;
        nat_eq(a760, a761)
    } {
        true
    } else {
        let a762 = v0;
        let a763 = 19u64;
        nat_eq(a762, a763)
    } {
        Some(1065353216u64)
    } else {
        None::<u64>
    }
}

pub fn f34(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<bool> {
    let a765 = match v1.clone() {
        Adt1::C0(h764, _, _, _, _, _, _, _, _, _, _) => {
            h764
        }
    };
    let a766 = 43u64;
    if nat_eq(a765, a766) {
        let a767 = v0;
        let a768 = 0u64;
        if if if if if if nat_eq(a767, a768) {
            let a770 = match v2.clone() {
                Adt1::C0(h769, _, _, _, _, _, _, _, _, _, _) => {
                    h769
                }
            };
            let a771 = 0u64;
            nat_eq(a770, a771)
        } else {
            false
        } {
            let a773 = match v3.clone() {
                Adt1::C0(h772, _, _, _, _, _, _, _, _, _, _) => {
                    h772
                }
            };
            let a774 = 0u64;
            if nat_eq(a773, a774) {
                true
            } else {
                let a776 = match v3.clone() {
                    Adt1::C0(h775, _, _, _, _, _, _, _, _, _, _) => {
                        h775
                    }
                };
                let a777 = 43u64;
                nat_eq(a776, a777)
            }
        } else {
            false
        } {
            let a779 = match v3.clone() {
                Adt1::C0(h778, _, _, _, _, _, _, _, _, _, _) => {
                    h778
                }
            };
            let a780 = 43u64;
            if nat_eq(a779, a780) {
                f23(v1.clone(), v2.clone())
            } else {
                f24(v1.clone(), v2.clone())
            }
        } else {
            false
        } {
            let a782 = match v2 {
                Adt1::C0(_, _, _, _, _, h781, _, _, _, _, _) => {
                    h781
                }
            };
            let a783 = 4u64;
            nat_eq(a782, a783)
        } else {
            false
        } {
            let a786 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h784, _, _, _, _, _) => {
                    h784
                }
            };
            let a787 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h785, _, _, _, _) => {
                    h785
                }
            };
            if nat_le(a786, a787) {
                let a790 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, h788, _, _, _, _) => {
                        h788
                    }
                };
                let a791 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h789, _, _, _) => {
                        h789
                    }
                };
                if nat_le(a790, a791) {
                    let a794 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, h792, _, _, _) => {
                            h792
                        }
                    };
                    let a795 = match v3 {
                        Adt1::C0(_, _, _, _, _, _, _, _, h793, _, _) => {
                            h793
                        }
                    };
                    nat_le(a794, a795)
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        } {
            let a803 = {
                let a801 = match v1.clone() {
                    Adt1::C0(_, _, h796, _, _, _, _, _, _, _, _) => {
                        h796
                    }
                };
                let a802 = {
                    let a799 = match v1.clone() {
                        Adt1::C0(_, _, _, h797, _, _, _, _, _, _, _) => {
                            h797
                        }
                    };
                    let a800 = match v1 {
                        Adt1::C0(_, _, _, _, h798, _, _, _, _, _, _) => {
                            h798
                        }
                    };
                    nat_mul(a799, a800)?
                };
                nat_mul(a801, a802)?
            };
            let a804 = 2147483647u64;
            Ok(nat_le(a803, a804))
        } else {
            Ok(false)
        }
    } else {
        if f23(v1.clone(), v2.clone()) {
            Ok(f19(match v1 {
                Adt1::C0(h805, _, _, _, _, _, _, _, _, _, _) => {
                    h805
                }
            }, match v2 {
                Adt1::C0(h806, _, _, _, _, _, _, _, _, _, _) => {
                    h806
                }
            }, match v3 {
                Adt1::C0(h807, _, _, _, _, _, _, _, _, _, _) => {
                    h807
                }
            }))
        } else {
            Ok(false)
        }
    }
}

pub fn f35(v0: Adt1) -> bool {
    let v1: u64 = f20(match v0.clone() {
        Adt1::C0(h808, _, _, _, _, _, _, _, _, _, _) => {
            h808
        }
    });
    let a813 = {
        let a810 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h809) => {
                h809
            }
        };
        let a811 = v1;
        let a812 = 1u64;
        nat_rem(a810, a811, a812)
    };
    let a814 = 0u64;
    if if nat_eq(a813, a814) {
        f62(match v0.clone() {
            Adt1::C0(_, h815, _, _, _, _, _, _, _, _, _) => {
                h815
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h816, _, _, _, _, _) => {
                h816
            }
        }, v1)
    } else {
        false
    } {
        f63(match v0.clone() {
            Adt1::C0(_, _, h817, _, _, _, _, _, _, _, _) => {
                h817
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h818, _, _, _, _, _, _, _) => {
                h818
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h819, _, _, _, _, _, _) => {
                h819
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h820, _, _, _, _) => {
                h820
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h821, _, _, _) => {
                h821
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h822, _, _) => {
                h822
            }
        }, v1)
    } else {
        false
    }
}

pub fn f36(v0: Adt1, v1: Adt1) -> R<bool> {
    let a830 = {
        let a828 = match v0.clone() {
            Adt1::C0(_, _, h823, _, _, _, _, _, _, _, _) => {
                h823
            }
        };
        let a829 = {
            let a826 = match v0.clone() {
                Adt1::C0(_, _, _, h824, _, _, _, _, _, _, _) => {
                    h824
                }
            };
            let a827 = match v0.clone() {
                Adt1::C0(_, _, _, _, h825, _, _, _, _, _, _) => {
                    h825
                }
            };
            nat_mul(a826, a827)?
        };
        nat_mul(a828, a829)?
    };
    let a831 = 9223372036854775807u64;
    if nat_le(a830, a831) {
        if f43(v1.clone()) {
            let a833 = match v1 {
                Adt1::C0(_, h832, _, _, _, _, _, _, _, _, _) => {
                    h832
                }
            };
            let a834 = 2147483647u64;
            Ok(nat_le(a833, a834))
        } else {
            let a836 = match v0 {
                Adt1::C0(_, h835, _, _, _, _, _, _, _, _, _) => {
                    h835
                }
            };
            let a837 = 2147483647u64;
            Ok(nat_le(a836, a837))
        }
    } else {
        Ok(false)
    }
}

pub fn f37(v0: Adt1) -> R<bool> {
    f31({
        let a840 = match v0.clone() {
            Adt1::C0(_, h838, _, _, _, _, _, _, _, _, _) => {
                h838
            }
        };
        let a841 = f20(match v0.clone() {
            Adt1::C0(h839, _, _, _, _, _, _, _, _, _, _) => {
                h839
            }
        });
        nat_mul(a840, a841)?
    }, match v0.clone() {
        Adt1::C0(_, _, h842, _, _, _, _, _, _, _, _) => {
            h842
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h843, _, _, _, _, _, _, _) => {
            h843
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h844, _, _, _, _, _, _) => {
            h844
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h845, _, _, _, _) => {
            h845
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h846, _, _, _) => {
            h846
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h847, _, _) => {
            h847
        }
    })
}

pub fn f38(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a854 = {
        let a848 = v0;
        let a849 = v2;
        nat_mul(a848, a849)?
    };
    let a855 = {
        let a850 = v1;
        let a851 = 0u64;
        if nat_eq(a850, a851) {
            0u64
        } else {
            let a852 = v0;
            let a853 = 2u64;
            nat_mul(a852, a853)?
        }
    };
    nat_add(a854, a855)
}

pub fn f39(v0: Adt1) -> R<u64> {
    let a864 = {
        let a858 = match v0.clone() {
            Adt1::C0(_, h856, _, _, _, _, _, _, _, _, _) => {
                h856
            }
        };
        let a859 = match v0.clone() {
            Adt1::C0(_, _, h857, _, _, _, _, _, _, _, _) => {
                h857
            }
        };
        nat_mul(a858, a859)?
    };
    let a865 = {
        let a862 = match v0.clone() {
            Adt1::C0(_, _, _, h860, _, _, _, _, _, _, _) => {
                h860
            }
        };
        let a863 = match v0 {
            Adt1::C0(_, _, _, _, h861, _, _, _, _, _, _) => {
                h861
            }
        };
        nat_mul(a862, a863)?
    };
    nat_mul(a864, a865)
}

pub fn f40(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a876 = {
        let a872 = {
            let a868 = 1u64;
            let a869 = {
                let a866 = v0;
                let a867 = 0u64;
                if nat_eq(a866, a867) {
                    0u64
                } else {
                    1u64
                }
            };
            nat_add(a868, a869)?
        };
        let a873 = {
            let a870 = v1;
            let a871 = 0u64;
            if nat_eq(a870, a871) {
                0u64
            } else {
                1u64
            }
        };
        nat_add(a872, a873)?
    };
    let a877 = {
        let a874 = v2;
        let a875 = 0u64;
        if nat_eq(a874, a875) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a876, a877)
}

pub fn f41(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f24(v0.clone(), v1.clone()) {
        let a880 = match v0.clone() {
            Adt1::C0(h878, _, _, _, _, _, _, _, _, _, _) => {
                h878
            }
        };
        let a881 = match v1.clone() {
            Adt1::C0(h879, _, _, _, _, _, _, _, _, _, _) => {
                h879
            }
        };
        nat_eq(a880, a881)
    } else {
        false
    } {
        let a884 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h882, _, _, _, _, _) => {
                h882
            }
        };
        let a885 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h883, _, _, _, _, _) => {
                h883
            }
        };
        nat_eq(a884, a885)
    } else {
        false
    } {
        let a888 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h886, _, _, _, _) => {
                h886
            }
        };
        let a889 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h887, _, _, _, _) => {
                h887
            }
        };
        nat_eq(a888, a889)
    } else {
        false
    } {
        let a892 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h890, _, _, _) => {
                h890
            }
        };
        let a893 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h891, _, _, _) => {
                h891
            }
        };
        nat_eq(a892, a893)
    } else {
        false
    } {
        let a896 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h894, _, _) => {
                h894
            }
        };
        let a897 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h895, _, _) => {
                h895
            }
        };
        nat_eq(a896, a897)
    } else {
        false
    } {
        let a899 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h898) => {
                h898
            }
        };
        let a900 = 0u64;
        if if nat_eq(a899, a900) {
            true
        } else {
            let a902 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h901) => {
                    h901
                }
            };
            let a903 = 0u64;
            nat_eq(a902, a903)
        } {
            true
        } else {
            let a906 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h904) => {
                    h904
                }
            };
            let a907 = match v1 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h905) => {
                    h905
                }
            };
            nat_eq(a906, a907)
        }
    } else {
        false
    }
}

pub fn f42(v0: u64) -> u64 {
    let a908 = v0;
    let a909 = 26u64;
    if nat_eq(a908, a909) {
        4u64
    } else {
        let a910 = v0;
        let a911 = 27u64;
        if nat_eq(a910, a911) {
            8u64
        } else {
            0u64
        }
    }
}

pub fn f43(v0: Adt1) -> bool {
    let a913 = match v0.clone() {
        Adt1::C0(_, h912, _, _, _, _, _, _, _, _, _) => {
            h912
        }
    };
    let a914 = 1u64;
    if nat_eq(a913, a914) {
        true
    } else {
        let a917 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h915, _, _, _, _, _) => {
                h915
            }
        };
        let a918 = f20(match v0 {
            Adt1::C0(h916, _, _, _, _, _, _, _, _, _, _) => {
                h916
            }
        });
        nat_eq(a917, a918)
    }
}

pub fn f44(v0: Adt1, v1: u64) -> R<u64> {
    let a924 = {
        let a920 = match v0.clone() {
            Adt1::C0(_, h919, _, _, _, _, _, _, _, _, _) => {
                h919
            }
        };
        let a921 = f29(v1);
        let a922 = 0u64;
        nat_quot(a920, a921, a922)
    };
    let a925 = f64(match v0 {
        Adt1::C0(h923, _, _, _, _, _, _, _, _, _, _) => {
            h923
        }
    }, v1);
    nat_mul(a924, a925)
}

pub fn f45(v0: u64) -> u64 {
    if if f18(v0) {
        true
    } else {
        let a926 = v0;
        let a927 = 26u64;
        nat_eq(a926, a927)
    } {
        f29(v0)
    } else {
        0u64
    }
}

pub fn f46(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a929 = match v0.clone() {
            Adt1::C0(_, h928, _, _, _, _, _, _, _, _, _) => {
                h928
            }
        };
        let a930 = 4u64;
        nat_mul(a929, a930)?
    };
    let v2: u64 = {
        let a933 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h931, _, _, _, _) => {
                h931
            }
        };
        let a934 = match v0.clone() {
            Adt1::C0(_, _, h932, _, _, _, _, _, _, _, _) => {
                h932
            }
        };
        nat_mul(a933, a934)?
    };
    let v3: u64 = {
        let a936 = v2;
        let a937 = match v0.clone() {
            Adt1::C0(_, _, _, h935, _, _, _, _, _, _, _) => {
                h935
            }
        };
        nat_mul(a936, a937)?
    };
    let a939 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h938, _, _, _, _, _) => {
            h938
        }
    };
    let a940 = 4u64;
    if if if if nat_eq(a939, a940) {
        let a945 = {
            let a942 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h941) => {
                    h941
                }
            };
            let a943 = 4u64;
            let a944 = 1u64;
            nat_rem(a942, a943, a944)
        };
        let a946 = 0u64;
        nat_eq(a945, a946)
    } else {
        false
    } {
        let a954 = {
            let a952 = match v0.clone() {
                Adt1::C0(_, _, h947, _, _, _, _, _, _, _, _) => {
                    h947
                }
            };
            let a953 = {
                let a950 = match v0.clone() {
                    Adt1::C0(_, _, _, h948, _, _, _, _, _, _, _) => {
                        h948
                    }
                };
                let a951 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h949, _, _, _, _, _, _) => {
                        h949
                    }
                };
                nat_mul(a950, a951)?
            };
            nat_mul(a952, a953)?
        };
        let a955 = 1u64;
        if nat_le(a954, a955) {
            true
        } else {
            let a957 = v1;
            let a958 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h956, _, _, _, _) => {
                    h956
                }
            };
            if nat_le(a957, a958) {
                let a963 = {
                    let a960 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, h959, _, _, _, _) => {
                            h959
                        }
                    };
                    let a961 = 4u64;
                    let a962 = 1u64;
                    nat_rem(a960, a961, a962)
                };
                let a964 = 0u64;
                nat_eq(a963, a964)
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a966 = match v0.clone() {
            Adt1::C0(_, _, _, h965, _, _, _, _, _, _, _) => {
                h965
            }
        };
        let a967 = 1u64;
        if nat_le(a966, a967) {
            true
        } else {
            let a969 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h968, _, _, _) => {
                    h968
                }
            };
            let a970 = v2;
            nat_eq(a969, a970)
        }
    } else {
        false
    } {
        let a972 = match v0.clone() {
            Adt1::C0(_, _, _, _, h971, _, _, _, _, _, _) => {
                h971
            }
        };
        let a973 = 1u64;
        if nat_le(a972, a973) {
            Ok(true)
        } else {
            let a975 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, h974, _, _) => {
                    h974
                }
            };
            let a976 = v3;
            Ok(nat_eq(a975, a976))
        }
    } else {
        Ok(false)
    }
}

pub fn f47(v0: Adt1, v1: Adt1) -> R<bool> {
    let a978 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h977, _) => {
            h977
        }
    };
    let a979 = 0u64;
    if nat_eq(a978, a979) {
        Ok(true)
    } else {
        let a981 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h980, _) => {
                h980
            }
        };
        let a982 = 0u64;
        if nat_eq(a981, a982) {
            Ok(true)
        } else {
            let a984 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h983) => {
                    h983
                }
            };
            let a985 = 0u64;
            if nat_eq(a984, a985) {
                Ok(true)
            } else {
                let a987 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h986) => {
                        h986
                    }
                };
                let a988 = 0u64;
                if nat_eq(a987, a988) {
                    Ok(true)
                } else {
                    let a994 = {
                        let a991 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h989) => {
                                h989
                            }
                        };
                        let a992 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h990, _) => {
                                h990
                            }
                        };
                        nat_add(a991, a992)?
                    };
                    let a995 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h993) => {
                            h993
                        }
                    };
                    if nat_le(a994, a995) {
                        Ok(true)
                    } else {
                        let a1001 = {
                            let a998 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h996) => {
                                    h996
                                }
                            };
                            let a999 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h997, _) => {
                                    h997
                                }
                            };
                            nat_add(a998, a999)?
                        };
                        let a1002 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1000) => {
                                h1000
                            }
                        };
                        Ok(nat_le(a1001, a1002))
                    }
                }
            }
        }
    }
}

pub fn f48(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a1005 = match v0.clone() {
            Adt1::C0(_, _, h1003, _, _, _, _, _, _, _, _) => {
                h1003
            }
        };
        let a1006 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1004, _, _, _, _) => {
                h1004
            }
        };
        nat_mul(a1005, a1006)?
    };
    if if f43(v0.clone()) {
        let a1008 = match v0.clone() {
            Adt1::C0(_, _, _, h1007, _, _, _, _, _, _, _) => {
                h1007
            }
        };
        let a1009 = 1u64;
        if nat_eq(a1008, a1009) {
            true
        } else {
            let a1011 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1010, _, _, _) => {
                    h1010
                }
            };
            let a1012 = v1;
            nat_eq(a1011, a1012)
        }
    } else {
        false
    } {
        let a1014 = match v0.clone() {
            Adt1::C0(_, _, _, _, h1013, _, _, _, _, _, _) => {
                h1013
            }
        };
        let a1015 = 1u64;
        if nat_eq(a1014, a1015) {
            Ok(true)
        } else {
            let a1020 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1016, _, _) => {
                    h1016
                }
            };
            let a1021 = {
                let a1018 = v1;
                let a1019 = match v0 {
                    Adt1::C0(_, _, _, h1017, _, _, _, _, _, _, _) => {
                        h1017
                    }
                };
                nat_mul(a1018, a1019)?
            };
            Ok(nat_eq(a1020, a1021))
        }
    } else {
        Ok(false)
    }
}

pub fn f49(v0: u64) -> u64 {
    let a1022 = v0;
    let a1023 = 3u64;
    if nat_eq(a1022, a1023) {
        16u64
    } else {
        10u64
    }
}

pub fn f50(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a1025 = v0;
        let a1026 = match v1.clone() {
            Adt1::C0(_, h1024, _, _, _, _, _, _, _, _, _) => {
                h1024
            }
        };
        let a1027 = 0u64;
        nat_quot(a1025, a1026, a1027)
    };
    let v3: u64 = {
        let a1029 = v2;
        let a1030 = match v1.clone() {
            Adt1::C0(_, _, h1028, _, _, _, _, _, _, _, _) => {
                h1028
            }
        };
        let a1031 = 0u64;
        nat_quot(a1029, a1030, a1031)
    };
    f52({
        let a1033 = v0;
        let a1034 = match v1.clone() {
            Adt1::C0(_, h1032, _, _, _, _, _, _, _, _, _) => {
                h1032
            }
        };
        let a1035 = 0u64;
        nat_rem(a1033, a1034, a1035)
    }, {
        let a1037 = v2;
        let a1038 = match v1.clone() {
            Adt1::C0(_, _, h1036, _, _, _, _, _, _, _, _) => {
                h1036
            }
        };
        let a1039 = 0u64;
        nat_rem(a1037, a1038, a1039)
    }, {
        let a1041 = v3;
        let a1042 = match v1.clone() {
            Adt1::C0(_, _, _, h1040, _, _, _, _, _, _, _) => {
                h1040
            }
        };
        let a1043 = 0u64;
        nat_rem(a1041, a1042, a1043)
    }, {
        let a1045 = v3;
        let a1046 = match v1.clone() {
            Adt1::C0(_, _, _, h1044, _, _, _, _, _, _, _) => {
                h1044
            }
        };
        let a1047 = 0u64;
        nat_quot(a1045, a1046, a1047)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h1048, _, _, _, _, _) => {
            h1048
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h1049, _, _, _, _) => {
            h1049
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1050, _, _, _) => {
            h1050
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1051, _, _) => {
            h1051
        }
    })
}

pub fn f51(v0: u64, v1: Adt1, v2: Adt1) -> R<u64> {
    let v3: u64 = {
        let a1053 = v0;
        let a1054 = match v1.clone() {
            Adt1::C0(_, h1052, _, _, _, _, _, _, _, _, _) => {
                h1052
            }
        };
        let a1055 = 0u64;
        nat_quot(a1053, a1054, a1055)
    };
    let v4: u64 = {
        let a1057 = v3;
        let a1058 = match v1.clone() {
            Adt1::C0(_, _, h1056, _, _, _, _, _, _, _, _) => {
                h1056
            }
        };
        let a1059 = 0u64;
        nat_quot(a1057, a1058, a1059)
    };
    f52({
        let a1065 = {
            let a1061 = v0;
            let a1062 = match v1.clone() {
                Adt1::C0(_, h1060, _, _, _, _, _, _, _, _, _) => {
                    h1060
                }
            };
            let a1063 = 0u64;
            nat_rem(a1061, a1062, a1063)
        };
        let a1066 = match v2.clone() {
            Adt1::C0(_, h1064, _, _, _, _, _, _, _, _, _) => {
                h1064
            }
        };
        let a1067 = 0u64;
        nat_rem(a1065, a1066, a1067)
    }, {
        let a1073 = {
            let a1069 = v3;
            let a1070 = match v1.clone() {
                Adt1::C0(_, _, h1068, _, _, _, _, _, _, _, _) => {
                    h1068
                }
            };
            let a1071 = 0u64;
            nat_rem(a1069, a1070, a1071)
        };
        let a1074 = match v2.clone() {
            Adt1::C0(_, _, h1072, _, _, _, _, _, _, _, _) => {
                h1072
            }
        };
        let a1075 = 0u64;
        nat_rem(a1073, a1074, a1075)
    }, {
        let a1081 = {
            let a1077 = v4;
            let a1078 = match v1.clone() {
                Adt1::C0(_, _, _, h1076, _, _, _, _, _, _, _) => {
                    h1076
                }
            };
            let a1079 = 0u64;
            nat_rem(a1077, a1078, a1079)
        };
        let a1082 = match v2.clone() {
            Adt1::C0(_, _, _, h1080, _, _, _, _, _, _, _) => {
                h1080
            }
        };
        let a1083 = 0u64;
        nat_rem(a1081, a1082, a1083)
    }, {
        let a1089 = {
            let a1085 = v4;
            let a1086 = match v1 {
                Adt1::C0(_, _, _, h1084, _, _, _, _, _, _, _) => {
                    h1084
                }
            };
            let a1087 = 0u64;
            nat_quot(a1085, a1086, a1087)
        };
        let a1090 = match v2.clone() {
            Adt1::C0(_, _, _, _, h1088, _, _, _, _, _, _) => {
                h1088
            }
        };
        let a1091 = 0u64;
        nat_rem(a1089, a1090, a1091)
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h1092, _, _, _, _, _) => {
            h1092
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h1093, _, _, _, _) => {
            h1093
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1094, _, _, _) => {
            h1094
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1095, _, _) => {
            h1095
        }
    })
}

pub fn f52(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a1108 = {
        let a1104 = {
            let a1100 = {
                let a1096 = v0;
                let a1097 = v4;
                nat_mul(a1096, a1097)?
            };
            let a1101 = {
                let a1098 = v1;
                let a1099 = v5;
                nat_mul(a1098, a1099)?
            };
            nat_add(a1100, a1101)?
        };
        let a1105 = {
            let a1102 = v2;
            let a1103 = v6;
            nat_mul(a1102, a1103)?
        };
        nat_add(a1104, a1105)?
    };
    let a1109 = {
        let a1106 = v3;
        let a1107 = v7;
        nat_mul(a1106, a1107)?
    };
    nat_add(a1108, a1109)
}

pub fn f53(v0: u64, v1: Adt1, v2: u64, v3: u64) -> R<Adt3> {
    let v4: u64 = {
        let a1110 = v0;
        let a1111 = 2u64;
        let a1112 = 0u64;
        nat_quot(a1110, a1111, a1112)
    };
    let v5: u64 = f42(match v1.clone() {
        Adt1::C0(h1113, _, _, _, _, _, _, _, _, _, _) => {
            h1113
        }
    });
    let a1117 = {
        let a1114 = v0;
        let a1115 = 2u64;
        let a1116 = 0u64;
        nat_rem(a1114, a1115, a1116)
    };
    let a1118 = 0u64;
    if nat_eq(a1117, a1118) {
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 4u64, f50(v4, v1)?, 2u64, 0u64, 0u64, 2u64, v5, 8u64))
    } else {
        let a1119 = v3;
        let a1120 = v2;
        if if nat_lt(a1119, a1120) {
            let a1123 = v3;
            let a1124 = {
                let a1121 = v5;
                let a1122 = 4u64;
                if nat_eq(a1121, a1122) {
                    2147483648u64
                } else {
                    9223372036854775808u64
                }
            };
            nat_lt(a1123, a1124)
        } else {
            false
        } {
            Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, {
                let a1125 = 3u64;
                let a1126 = v4;
                nat_add(a1125, a1126)?
            }, 8u64, 8u64))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f54(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64) -> R<Adt3> {
    let v5: u64 = {
        let a1128 = 1u64;
        let a1129 = f44(v1.clone(), match v3.clone() {
            Adt1::C0(h1127, _, _, _, _, _, _, _, _, _, _) => {
                h1127
            }
        })?;
        nat_add(a1128, a1129)?
    };
    let v6: u64 = {
        let a1130 = v0;
        let a1131 = v5;
        let a1132 = 0u64;
        nat_rem(a1130, a1131, a1132)
    };
    let v7: u64 = {
        let a1133 = v0;
        let a1134 = v5;
        let a1135 = 0u64;
        nat_quot(a1133, a1134, a1135)
    };
    let v8: u64 = {
        let a1137 = v7;
        let a1138 = match v1.clone() {
            Adt1::C0(_, _, h1136, _, _, _, _, _, _, _, _) => {
                h1136
            }
        };
        let a1139 = 0u64;
        nat_rem(a1137, a1138, a1139)
    };
    let v9: u64 = {
        let a1141 = v7;
        let a1142 = match v1.clone() {
            Adt1::C0(_, _, h1140, _, _, _, _, _, _, _, _) => {
                h1140
            }
        };
        let a1143 = 0u64;
        nat_quot(a1141, a1142, a1143)
    };
    let v10: u64 = {
        let a1145 = v9;
        let a1146 = match v1.clone() {
            Adt1::C0(_, _, _, h1144, _, _, _, _, _, _, _) => {
                h1144
            }
        };
        let a1147 = 0u64;
        nat_rem(a1145, a1146, a1147)
    };
    let v11: u64 = {
        let a1149 = v9;
        let a1150 = match v1.clone() {
            Adt1::C0(_, _, _, h1148, _, _, _, _, _, _, _) => {
                h1148
            }
        };
        let a1151 = 0u64;
        nat_quot(a1149, a1150, a1151)
    };
    let a1152 = v6;
    let a1153 = 0u64;
    if nat_eq(a1152, a1153) {
        let v12: u64 = {
            let a1170 = v8;
            let a1171 = {
                let a1168 = match v2.clone() {
                    Adt1::C0(_, h1154, _, _, _, _, _, _, _, _, _) => {
                        h1154
                    }
                };
                let a1169 = {
                    let a1166 = {
                        let a1156 = v10;
                        let a1157 = match v2.clone() {
                            Adt1::C0(_, _, h1155, _, _, _, _, _, _, _, _) => {
                                h1155
                            }
                        };
                        let a1158 = 0u64;
                        nat_rem(a1156, a1157, a1158)
                    };
                    let a1167 = {
                        let a1164 = match v2.clone() {
                            Adt1::C0(_, _, h1159, _, _, _, _, _, _, _, _) => {
                                h1159
                            }
                        };
                        let a1165 = {
                            let a1161 = v11;
                            let a1162 = match v2 {
                                Adt1::C0(_, _, _, h1160, _, _, _, _, _, _, _) => {
                                    h1160
                                }
                            };
                            let a1163 = 0u64;
                            nat_rem(a1161, a1162, a1163)
                        };
                        nat_mul(a1164, a1165)?
                    };
                    nat_add(a1166, a1167)?
                };
                nat_mul(a1168, a1169)?
            };
            nat_add(a1170, a1171)?
        };
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, {
            let a1172 = 3u64;
            let a1173 = v12;
            nat_add(a1172, a1173)?
        }, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
    } else {
        let a1175 = v4;
        let a1176 = match v3.clone() {
            Adt1::C0(_, _, h1174, _, _, _, _, _, _, _, _) => {
                h1174
            }
        };
        if nat_lt(a1175, a1176) {
            let v13: u64 = {
                let a1181 = {
                    let a1177 = v6;
                    let a1178 = 1u64;
                    nat_sub(a1177, a1178)
                };
                let a1182 = f64(match v1.clone() {
                    Adt1::C0(h1179, _, _, _, _, _, _, _, _, _, _) => {
                        h1179
                    }
                }, match v3.clone() {
                    Adt1::C0(h1180, _, _, _, _, _, _, _, _, _, _) => {
                        h1180
                    }
                });
                let a1183 = 0u64;
                nat_quot(a1181, a1182, a1183)
            };
            let v14: u64 = f52({
                let a1185 = v13;
                let a1186 = f29(match v3.clone() {
                    Adt1::C0(h1184, _, _, _, _, _, _, _, _, _, _) => {
                        h1184
                    }
                });
                nat_mul(a1185, a1186)?
            }, v8, v10, v11, match v1.clone() {
                Adt1::C0(_, _, _, _, _, h1187, _, _, _, _, _) => {
                    h1187
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, h1188, _, _, _, _) => {
                    h1188
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1189, _, _, _) => {
                    h1189
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1190, _, _) => {
                    h1190
                }
            })?;
            let v15: u64 = f52(v13, v4, v10, v11, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h1191, _, _, _, _, _) => {
                    h1191
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h1192, _, _, _, _) => {
                    h1192
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1193, _, _, _) => {
                    h1193
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1194, _, _) => {
                    h1194
                }
            })?;
            let a1196 = 0u64;
            let a1197 = f45(match v3.clone() {
                Adt1::C0(h1195, _, _, _, _, _, _, _, _, _, _) => {
                    h1195
                }
            });
            if nat_lt(a1196, a1197) {
                Ok(Adt3::C6(v14, v15))
            } else {
                Ok(f65(match v1 {
                    Adt1::C0(h1198, _, _, _, _, _, _, _, _, _, _) => {
                        h1198
                    }
                }, match v3 {
                    Adt1::C0(h1199, _, _, _, _, _, _, _, _, _, _) => {
                        h1199
                    }
                }, {
                    let a1200 = v6;
                    let a1201 = 1u64;
                    nat_sub(a1200, a1201)
                }, v14, v15))
            }
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f55(v0: u64, v1: u64, v2: u64) -> Adt3 {
    Adt3::C3(2u64, 0u64, 0u64, 0u64, 1u64, v0, v1, v2, 0u64, 2u64, 4u64, 4u64)
}

pub fn f56(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt3 {
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

pub fn f57(v0: u64, v1: u64) -> Adt3 {
    let a1202 = v0;
    let a1203 = 8u64;
    if nat_eq(a1202, a1203) {
        Adt3::C3(1u64, 6u64, 0u64, 0u64, 1u64, 0u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
    } else {
        let a1204 = v0;
        let a1205 = 9u64;
        if nat_eq(a1204, a1205) {
            Adt3::C3(1u64, 2u64, 0u64, 0u64, 2u64, v1, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
        } else {
            let a1206 = v0;
            let a1207 = 10u64;
            if nat_eq(a1206, a1207) {
                Adt3::C3(5u64, 16u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
            } else {
                let a1208 = v0;
                let a1209 = 11u64;
                if nat_eq(a1208, a1209) {
                    Adt3::C3(1u64, 0u64, 0u64, 0u64, 2u64, 1065353216u64, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
                } else {
                    let a1210 = v0;
                    let a1211 = 12u64;
                    if nat_eq(a1210, a1211) {
                        Adt3::C3(1u64, 3u64, 0u64, 0u64, 1u64, 0u64, 1u64, 2u64, 0u64, 0u64, 4u64, 4u64)
                    } else {
                        let a1212 = v0;
                        let a1213 = 13u64;
                        if nat_eq(a1212, a1213) {
                            Adt3::C3(1u64, 0u64, 0u64, 0u64, 1u64, 1u64, 2u64, 1065353216u64, 0u64, 1u64, 4u64, 4u64)
                        } else {
                            let a1214 = v0;
                            let a1215 = 14u64;
                            if nat_eq(a1214, a1215) {
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

pub fn f58(v0: u64) -> Adt4 {
    let a1216 = v0;
    let a1217 = 0u64;
    if nat_eq(a1216, a1217) {
        Adt4::C1(Adt5::C0(0u64, 1u64, 4u64))
    } else {
        let a1218 = v0;
        let a1219 = 1u64;
        if nat_eq(a1218, a1219) {
            Adt4::C1(Adt5::C0(1u64, 1u64, 2u64))
        } else {
            let a1220 = v0;
            let a1221 = 2u64;
            if nat_eq(a1220, a1221) {
                Adt4::C1(Adt5::C0(2u64, 32u64, 18u64))
            } else {
                let a1222 = v0;
                let a1223 = 3u64;
                if nat_eq(a1222, a1223) {
                    Adt4::C1(Adt5::C0(3u64, 32u64, 20u64))
                } else {
                    let a1224 = v0;
                    let a1225 = 6u64;
                    if nat_eq(a1224, a1225) {
                        Adt4::C1(Adt5::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1226 = v0;
                        let a1227 = 7u64;
                        if nat_eq(a1226, a1227) {
                            Adt4::C1(Adt5::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1228 = v0;
                            let a1229 = 8u64;
                            if nat_eq(a1228, a1229) {
                                Adt4::C1(Adt5::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1230 = v0;
                                let a1231 = 9u64;
                                if nat_eq(a1230, a1231) {
                                    Adt4::C1(Adt5::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1232 = v0;
                                    let a1233 = 10u64;
                                    if nat_eq(a1232, a1233) {
                                        Adt4::C1(Adt5::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1234 = v0;
                                        let a1235 = 11u64;
                                        if nat_eq(a1234, a1235) {
                                            Adt4::C1(Adt5::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1236 = v0;
                                            let a1237 = 12u64;
                                            if nat_eq(a1236, a1237) {
                                                Adt4::C1(Adt5::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1238 = v0;
                                                let a1239 = 13u64;
                                                if nat_eq(a1238, a1239) {
                                                    Adt4::C1(Adt5::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1240 = v0;
                                                    let a1241 = 14u64;
                                                    if nat_eq(a1240, a1241) {
                                                        Adt4::C1(Adt5::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1242 = v0;
                                                        let a1243 = 15u64;
                                                        if nat_eq(a1242, a1243) {
                                                            Adt4::C1(Adt5::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1244 = v0;
                                                            let a1245 = 16u64;
                                                            if nat_eq(a1244, a1245) {
                                                                Adt4::C1(Adt5::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1246 = v0;
                                                                let a1247 = 17u64;
                                                                if nat_eq(a1246, a1247) {
                                                                    Adt4::C1(Adt5::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1248 = v0;
                                                                    let a1249 = 18u64;
                                                                    if nat_eq(a1248, a1249) {
                                                                        Adt4::C1(Adt5::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1250 = v0;
                                                                        let a1251 = 19u64;
                                                                        if nat_eq(a1250, a1251) {
                                                                            Adt4::C1(Adt5::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1252 = v0;
                                                                            let a1253 = 20u64;
                                                                            if nat_eq(a1252, a1253) {
                                                                                Adt4::C1(Adt5::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1254 = v0;
                                                                                let a1255 = 21u64;
                                                                                if nat_eq(a1254, a1255) {
                                                                                    Adt4::C1(Adt5::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1256 = v0;
                                                                                    let a1257 = 22u64;
                                                                                    if nat_eq(a1256, a1257) {
                                                                                        Adt4::C1(Adt5::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1258 = v0;
                                                                                        let a1259 = 23u64;
                                                                                        if nat_eq(a1258, a1259) {
                                                                                            Adt4::C1(Adt5::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1260 = v0;
                                                                                            let a1261 = 24u64;
                                                                                            if nat_eq(a1260, a1261) {
                                                                                                Adt4::C1(Adt5::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1262 = v0;
                                                                                                let a1263 = 25u64;
                                                                                                if nat_eq(a1262, a1263) {
                                                                                                    Adt4::C1(Adt5::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1264 = v0;
                                                                                                    let a1265 = 26u64;
                                                                                                    if nat_eq(a1264, a1265) {
                                                                                                        Adt4::C1(Adt5::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1266 = v0;
                                                                                                        let a1267 = 27u64;
                                                                                                        if nat_eq(a1266, a1267) {
                                                                                                            Adt4::C1(Adt5::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1268 = v0;
                                                                                                            let a1269 = 28u64;
                                                                                                            if nat_eq(a1268, a1269) {
                                                                                                                Adt4::C1(Adt5::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1270 = v0;
                                                                                                                let a1271 = 29u64;
                                                                                                                if nat_eq(a1270, a1271) {
                                                                                                                    Adt4::C1(Adt5::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1272 = v0;
                                                                                                                    let a1273 = 30u64;
                                                                                                                    if nat_eq(a1272, a1273) {
                                                                                                                        Adt4::C1(Adt5::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1274 = v0;
                                                                                                                        let a1275 = 34u64;
                                                                                                                        if nat_eq(a1274, a1275) {
                                                                                                                            Adt4::C1(Adt5::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1276 = v0;
                                                                                                                            let a1277 = 35u64;
                                                                                                                            if nat_eq(a1276, a1277) {
                                                                                                                                Adt4::C1(Adt5::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1278 = v0;
                                                                                                                                let a1279 = 39u64;
                                                                                                                                if nat_eq(a1278, a1279) {
                                                                                                                                    Adt4::C1(Adt5::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1280 = v0;
                                                                                                                                    let a1281 = 40u64;
                                                                                                                                    if nat_eq(a1280, a1281) {
                                                                                                                                        Adt4::C1(Adt5::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1282 = v0;
                                                                                                                                        let a1283 = 41u64;
                                                                                                                                        if nat_eq(a1282, a1283) {
                                                                                                                                            Adt4::C1(Adt5::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1284 = v0;
                                                                                                                                            let a1285 = 42u64;
                                                                                                                                            if nat_eq(a1284, a1285) {
                                                                                                                                                Adt4::C1(Adt5::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1286 = v0;
                                                                                                                                                let a1287 = 43u64;
                                                                                                                                                if nat_eq(a1286, a1287) {
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

pub fn f59(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f24(v0.clone(), v1.clone()) {
        let a1290 = match v0.clone() {
            Adt1::C0(h1288, _, _, _, _, _, _, _, _, _, _) => {
                h1288
            }
        };
        let a1291 = match v1.clone() {
            Adt1::C0(h1289, _, _, _, _, _, _, _, _, _, _) => {
                h1289
            }
        };
        nat_eq(a1290, a1291)
    } else {
        false
    } {
        let a1294 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1292) => {
                h1292
            }
        };
        let a1295 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1293) => {
                h1293
            }
        };
        nat_eq(a1294, a1295)
    } else {
        false
    } {
        let a1298 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1296, _, _, _, _, _) => {
                h1296
            }
        };
        let a1299 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1297, _, _, _, _, _) => {
                h1297
            }
        };
        nat_eq(a1298, a1299)
    } else {
        false
    } {
        let a1302 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1300, _, _, _, _) => {
                h1300
            }
        };
        let a1303 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1301, _, _, _, _) => {
                h1301
            }
        };
        nat_eq(a1302, a1303)
    } else {
        false
    } {
        let a1306 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1304, _, _, _) => {
                h1304
            }
        };
        let a1307 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1305, _, _, _) => {
                h1305
            }
        };
        nat_eq(a1306, a1307)
    } else {
        false
    } {
        let a1310 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1308, _, _) => {
                h1308
            }
        };
        let a1311 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1309, _, _) => {
                h1309
            }
        };
        nat_eq(a1310, a1311)
    } else {
        false
    }
}

pub fn f60(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a1312 = v1;
    let a1313 = 1u64;
    if if if nat_le(a1312, a1313) {
        true
    } else {
        let a1314 = v0;
        let a1315 = v2;
        nat_le(a1314, a1315)
    } {
        let a1316 = v3;
        let a1317 = 1u64;
        if nat_le(a1316, a1317) {
            true
        } else {
            let a1324 = {
                let a1322 = v0;
                let a1323 = {
                    let a1320 = {
                        let a1318 = v1;
                        let a1319 = 1u64;
                        nat_sub(a1318, a1319)
                    };
                    let a1321 = v2;
                    nat_mul(a1320, a1321)?
                };
                nat_add(a1322, a1323)?
            };
            let a1325 = v4;
            nat_le(a1324, a1325)
        }
    } else {
        false
    } {
        let a1326 = v5;
        let a1327 = 1u64;
        if nat_le(a1326, a1327) {
            Ok(true)
        } else {
            let a1340 = {
                let a1338 = {
                    let a1332 = v0;
                    let a1333 = {
                        let a1330 = {
                            let a1328 = v1;
                            let a1329 = 1u64;
                            nat_sub(a1328, a1329)
                        };
                        let a1331 = v2;
                        nat_mul(a1330, a1331)?
                    };
                    nat_add(a1332, a1333)?
                };
                let a1339 = {
                    let a1336 = {
                        let a1334 = v3;
                        let a1335 = 1u64;
                        nat_sub(a1334, a1335)
                    };
                    let a1337 = v4;
                    nat_mul(a1336, a1337)?
                };
                nat_add(a1338, a1339)?
            };
            let a1341 = v6;
            Ok(nat_le(a1340, a1341))
        }
    } else {
        Ok(false)
    }
}

pub fn f61(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1342 = v0;
    let a1343 = 0u64;
    if if if nat_eq(a1342, a1343) {
        true
    } else {
        let a1344 = v1;
        let a1345 = 0u64;
        nat_eq(a1344, a1345)
    } {
        true
    } else {
        let a1346 = v2;
        let a1347 = 0u64;
        nat_eq(a1346, a1347)
    } {
        true
    } else {
        let a1348 = v3;
        let a1349 = 0u64;
        nat_eq(a1348, a1349)
    }
}

pub fn f62(v0: u64, v1: u64, v2: u64) -> bool {
    let a1350 = v0;
    let a1351 = 1u64;
    if nat_le(a1350, a1351) {
        true
    } else {
        let a1355 = {
            let a1352 = v1;
            let a1353 = v2;
            let a1354 = 1u64;
            nat_rem(a1352, a1353, a1354)
        };
        let a1356 = 0u64;
        nat_eq(a1355, a1356)
    }
}

pub fn f63(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f62(v0, v3, v6) {
        f62(v1, v4, v6)
    } else {
        false
    } {
        f62(v2, v5, v6)
    } else {
        false
    }
}

pub fn f64(v0: u64, v1: u64) -> u64 {
    let a1357 = v0;
    let a1358 = 1u64;
    if if nat_eq(a1357, a1358) {
        let a1359 = v1;
        let a1360 = 43u64;
        nat_eq(a1359, a1360)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f65(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt3 {
    let a1361 = f64(v0, v1);
    let a1362 = 2u64;
    if nat_eq(a1361, a1362) {
        let a1366 = {
            let a1363 = v2;
            let a1364 = 2u64;
            let a1365 = 0u64;
            nat_rem(a1363, a1364, a1365)
        };
        let a1367 = 0u64;
        if nat_eq(a1366, a1367) {
            Adt3::C3(0u64, 0u64, v0, 0u64, 0u64, v3, 2u64, 0u64, 1u64, 0u64, f21(v0), 8u64)
        } else {
            Adt3::C3(0u64, 0u64, 0u64, v1, 1u64, 0u64, 2u64, 0u64, 3u64, v4, 8u64, f28(v1))
        }
    } else {
        Adt3::C3(0u64, 0u64, v0, v1, 0u64, v3, 2u64, 0u64, 3u64, v4, f21(v0), f28(v1))
    }
}
