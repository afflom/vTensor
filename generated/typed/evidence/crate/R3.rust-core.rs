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
        let a727 = 0u64;
        let a728 = f20(v4);
        if if if nat_lt(a727, a728) {
            true
        } else {
            if f18(v4) {
                let a729 = v4;
                let a730 = 9u64;
                !if nat_eq(a729, a730) {
                    true
                } else {
                    let a731 = v4;
                    let a732 = 15u64;
                    nat_eq(a731, a732)
                }
            } else {
                false
            }
        } {
            let a734 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, h733, _, _, _, _, _) => {
                    h733
                }
            };
            let a735 = f28(v4);
            nat_eq(a734, a735)
        } else {
            false
        } {
            let a740 = {
                let a737 = match v2 {
                    Adt1::C0(_, h736, _, _, _, _, _, _, _, _, _) => {
                        h736
                    }
                };
                let a738 = f29(v4);
                let a739 = 1u64;
                nat_rem(a737, a738, a739)
            };
            let a741 = 0u64;
            nat_eq(a740, a741)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f26(v0: u64, v1: u64) -> bool {
    if f18(v0) {
        let a742 = v0;
        let a743 = 9u64;
        !if nat_eq(a742, a743) {
            true
        } else {
            let a744 = v0;
            let a745 = 15u64;
            nat_eq(a744, a745)
        }
    } else {
        let a746 = v0;
        let a747 = v1;
        if if nat_eq(a746, a747) {
            let a748 = 0u64;
            let a749 = f20(v0);
            nat_lt(a748, a749)
        } else {
            false
        } {
            true
        } else {
            let a750 = f21(v0);
            let a751 = 2u64;
            if nat_eq(a750, a751) {
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
    let a752 = 24u64;
    let a753 = v0;
    if if nat_le(a752, a753) {
        let a754 = v0;
        let a755 = 28u64;
        nat_le(a754, a755)
    } else {
        false
    } {
        let m756 = f58(v0);
        match m756 {
            Adt4::C0 => {
                0u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, _, h757) => {
                        h757
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
        let m758 = f58(v0);
        match m758 {
            Adt4::C0 => {
                0u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, _, h759) => {
                        h759
                    }
                }
            }
        }
    } else {
        let a760 = v0;
        let a761 = 26u64;
        if nat_eq(a760, a761) {
            4u64
        } else {
            f20(v0)
        }
    }
}

pub fn f29(v0: u64) -> u64 {
    if f18(v0) {
        let m762 = f58(v0);
        match m762 {
            Adt4::C0 => {
                1u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, h763, _) => {
                        h763
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
        Adt1::C0(_, h764, _, _, _, _, _, _, _, _, _) => {
            h764
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h765, _, _, _, _, _, _, _, _) => {
            h765
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h766, _, _, _, _, _, _, _) => {
            h766
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, h767, _, _, _, _, _, _) => {
            h767
        }
    })
}

pub fn f33(v0: u64) -> Option<u64> {
    let a768 = v0;
    let a769 = 16u64;
    if if if nat_eq(a768, a769) {
        true
    } else {
        let a770 = v0;
        let a771 = 17u64;
        nat_eq(a770, a771)
    } {
        true
    } else {
        let a772 = v0;
        let a773 = 19u64;
        nat_eq(a772, a773)
    } {
        Some(1065353216u64)
    } else {
        None::<u64>
    }
}

pub fn f34(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<bool> {
    let a775 = match v1.clone() {
        Adt1::C0(h774, _, _, _, _, _, _, _, _, _, _) => {
            h774
        }
    };
    let a776 = 43u64;
    if nat_eq(a775, a776) {
        let a777 = v0;
        let a778 = 0u64;
        if if if if if if nat_eq(a777, a778) {
            let a780 = match v2.clone() {
                Adt1::C0(h779, _, _, _, _, _, _, _, _, _, _) => {
                    h779
                }
            };
            let a781 = 0u64;
            nat_eq(a780, a781)
        } else {
            false
        } {
            let a783 = match v3.clone() {
                Adt1::C0(h782, _, _, _, _, _, _, _, _, _, _) => {
                    h782
                }
            };
            let a784 = 0u64;
            if nat_eq(a783, a784) {
                true
            } else {
                let a786 = match v3.clone() {
                    Adt1::C0(h785, _, _, _, _, _, _, _, _, _, _) => {
                        h785
                    }
                };
                let a787 = 43u64;
                nat_eq(a786, a787)
            }
        } else {
            false
        } {
            let a789 = match v3.clone() {
                Adt1::C0(h788, _, _, _, _, _, _, _, _, _, _) => {
                    h788
                }
            };
            let a790 = 43u64;
            if nat_eq(a789, a790) {
                f23(v1.clone(), v2.clone())
            } else {
                f24(v1.clone(), v2.clone())
            }
        } else {
            false
        } {
            let a792 = match v2 {
                Adt1::C0(_, _, _, _, _, h791, _, _, _, _, _) => {
                    h791
                }
            };
            let a793 = 4u64;
            nat_eq(a792, a793)
        } else {
            false
        } {
            let a796 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h794, _, _, _, _, _) => {
                    h794
                }
            };
            let a797 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h795, _, _, _, _) => {
                    h795
                }
            };
            if nat_le(a796, a797) {
                let a800 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, h798, _, _, _, _) => {
                        h798
                    }
                };
                let a801 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h799, _, _, _) => {
                        h799
                    }
                };
                if nat_le(a800, a801) {
                    let a804 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, h802, _, _, _) => {
                            h802
                        }
                    };
                    let a805 = match v3 {
                        Adt1::C0(_, _, _, _, _, _, _, _, h803, _, _) => {
                            h803
                        }
                    };
                    nat_le(a804, a805)
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        } {
            let a813 = {
                let a811 = match v1.clone() {
                    Adt1::C0(_, _, h806, _, _, _, _, _, _, _, _) => {
                        h806
                    }
                };
                let a812 = {
                    let a809 = match v1.clone() {
                        Adt1::C0(_, _, _, h807, _, _, _, _, _, _, _) => {
                            h807
                        }
                    };
                    let a810 = match v1 {
                        Adt1::C0(_, _, _, _, h808, _, _, _, _, _, _) => {
                            h808
                        }
                    };
                    nat_mul(a809, a810)?
                };
                nat_mul(a811, a812)?
            };
            let a814 = 2147483647u64;
            Ok(nat_le(a813, a814))
        } else {
            Ok(false)
        }
    } else {
        if f23(v1.clone(), v2.clone()) {
            Ok(f19(match v1 {
                Adt1::C0(h815, _, _, _, _, _, _, _, _, _, _) => {
                    h815
                }
            }, match v2 {
                Adt1::C0(h816, _, _, _, _, _, _, _, _, _, _) => {
                    h816
                }
            }, match v3 {
                Adt1::C0(h817, _, _, _, _, _, _, _, _, _, _) => {
                    h817
                }
            }))
        } else {
            Ok(false)
        }
    }
}

pub fn f35(v0: Adt1) -> bool {
    let v1: u64 = f20(match v0.clone() {
        Adt1::C0(h818, _, _, _, _, _, _, _, _, _, _) => {
            h818
        }
    });
    let a823 = {
        let a820 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h819) => {
                h819
            }
        };
        let a821 = v1;
        let a822 = 1u64;
        nat_rem(a820, a821, a822)
    };
    let a824 = 0u64;
    if if nat_eq(a823, a824) {
        f62(match v0.clone() {
            Adt1::C0(_, h825, _, _, _, _, _, _, _, _, _) => {
                h825
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h826, _, _, _, _, _) => {
                h826
            }
        }, v1)
    } else {
        false
    } {
        f63(match v0.clone() {
            Adt1::C0(_, _, h827, _, _, _, _, _, _, _, _) => {
                h827
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h828, _, _, _, _, _, _, _) => {
                h828
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h829, _, _, _, _, _, _) => {
                h829
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h830, _, _, _, _) => {
                h830
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h831, _, _, _) => {
                h831
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h832, _, _) => {
                h832
            }
        }, v1)
    } else {
        false
    }
}

pub fn f36(v0: Adt1, v1: Adt1) -> R<bool> {
    let a840 = {
        let a838 = match v0.clone() {
            Adt1::C0(_, _, h833, _, _, _, _, _, _, _, _) => {
                h833
            }
        };
        let a839 = {
            let a836 = match v0.clone() {
                Adt1::C0(_, _, _, h834, _, _, _, _, _, _, _) => {
                    h834
                }
            };
            let a837 = match v0.clone() {
                Adt1::C0(_, _, _, _, h835, _, _, _, _, _, _) => {
                    h835
                }
            };
            nat_mul(a836, a837)?
        };
        nat_mul(a838, a839)?
    };
    let a841 = 9223372036854775807u64;
    if nat_le(a840, a841) {
        if f43(v1.clone()) {
            let a843 = match v1 {
                Adt1::C0(_, h842, _, _, _, _, _, _, _, _, _) => {
                    h842
                }
            };
            let a844 = 2147483647u64;
            Ok(nat_le(a843, a844))
        } else {
            let a846 = match v0 {
                Adt1::C0(_, h845, _, _, _, _, _, _, _, _, _) => {
                    h845
                }
            };
            let a847 = 2147483647u64;
            Ok(nat_le(a846, a847))
        }
    } else {
        Ok(false)
    }
}

pub fn f37(v0: Adt1) -> R<bool> {
    f31({
        let a850 = match v0.clone() {
            Adt1::C0(_, h848, _, _, _, _, _, _, _, _, _) => {
                h848
            }
        };
        let a851 = f20(match v0.clone() {
            Adt1::C0(h849, _, _, _, _, _, _, _, _, _, _) => {
                h849
            }
        });
        nat_mul(a850, a851)?
    }, match v0.clone() {
        Adt1::C0(_, _, h852, _, _, _, _, _, _, _, _) => {
            h852
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h853, _, _, _, _, _, _, _) => {
            h853
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h854, _, _, _, _, _, _) => {
            h854
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h855, _, _, _, _) => {
            h855
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h856, _, _, _) => {
            h856
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h857, _, _) => {
            h857
        }
    })
}

pub fn f38(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a864 = {
        let a858 = v0;
        let a859 = v2;
        nat_mul(a858, a859)?
    };
    let a865 = {
        let a860 = v1;
        let a861 = 0u64;
        if nat_eq(a860, a861) {
            0u64
        } else {
            let a862 = v0;
            let a863 = 2u64;
            nat_mul(a862, a863)?
        }
    };
    nat_add(a864, a865)
}

pub fn f39(v0: Adt1) -> R<u64> {
    let a874 = {
        let a868 = match v0.clone() {
            Adt1::C0(_, h866, _, _, _, _, _, _, _, _, _) => {
                h866
            }
        };
        let a869 = match v0.clone() {
            Adt1::C0(_, _, h867, _, _, _, _, _, _, _, _) => {
                h867
            }
        };
        nat_mul(a868, a869)?
    };
    let a875 = {
        let a872 = match v0.clone() {
            Adt1::C0(_, _, _, h870, _, _, _, _, _, _, _) => {
                h870
            }
        };
        let a873 = match v0 {
            Adt1::C0(_, _, _, _, h871, _, _, _, _, _, _) => {
                h871
            }
        };
        nat_mul(a872, a873)?
    };
    nat_mul(a874, a875)
}

pub fn f40(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a886 = {
        let a882 = {
            let a878 = 1u64;
            let a879 = {
                let a876 = v0;
                let a877 = 0u64;
                if nat_eq(a876, a877) {
                    0u64
                } else {
                    1u64
                }
            };
            nat_add(a878, a879)?
        };
        let a883 = {
            let a880 = v1;
            let a881 = 0u64;
            if nat_eq(a880, a881) {
                0u64
            } else {
                1u64
            }
        };
        nat_add(a882, a883)?
    };
    let a887 = {
        let a884 = v2;
        let a885 = 0u64;
        if nat_eq(a884, a885) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a886, a887)
}

pub fn f41(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f24(v0.clone(), v1.clone()) {
        let a890 = match v0.clone() {
            Adt1::C0(h888, _, _, _, _, _, _, _, _, _, _) => {
                h888
            }
        };
        let a891 = match v1.clone() {
            Adt1::C0(h889, _, _, _, _, _, _, _, _, _, _) => {
                h889
            }
        };
        nat_eq(a890, a891)
    } else {
        false
    } {
        let a894 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h892, _, _, _, _, _) => {
                h892
            }
        };
        let a895 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h893, _, _, _, _, _) => {
                h893
            }
        };
        nat_eq(a894, a895)
    } else {
        false
    } {
        let a898 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h896, _, _, _, _) => {
                h896
            }
        };
        let a899 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h897, _, _, _, _) => {
                h897
            }
        };
        nat_eq(a898, a899)
    } else {
        false
    } {
        let a902 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h900, _, _, _) => {
                h900
            }
        };
        let a903 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h901, _, _, _) => {
                h901
            }
        };
        nat_eq(a902, a903)
    } else {
        false
    } {
        let a906 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h904, _, _) => {
                h904
            }
        };
        let a907 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h905, _, _) => {
                h905
            }
        };
        nat_eq(a906, a907)
    } else {
        false
    } {
        let a909 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h908) => {
                h908
            }
        };
        let a910 = 0u64;
        if if nat_eq(a909, a910) {
            true
        } else {
            let a912 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h911) => {
                    h911
                }
            };
            let a913 = 0u64;
            nat_eq(a912, a913)
        } {
            true
        } else {
            let a916 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h914) => {
                    h914
                }
            };
            let a917 = match v1 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h915) => {
                    h915
                }
            };
            nat_eq(a916, a917)
        }
    } else {
        false
    }
}

pub fn f42(v0: u64) -> u64 {
    let a918 = v0;
    let a919 = 26u64;
    if nat_eq(a918, a919) {
        4u64
    } else {
        let a920 = v0;
        let a921 = 27u64;
        if nat_eq(a920, a921) {
            8u64
        } else {
            0u64
        }
    }
}

pub fn f43(v0: Adt1) -> bool {
    let a923 = match v0.clone() {
        Adt1::C0(_, h922, _, _, _, _, _, _, _, _, _) => {
            h922
        }
    };
    let a924 = 1u64;
    if nat_eq(a923, a924) {
        true
    } else {
        let a927 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h925, _, _, _, _, _) => {
                h925
            }
        };
        let a928 = f20(match v0 {
            Adt1::C0(h926, _, _, _, _, _, _, _, _, _, _) => {
                h926
            }
        });
        nat_eq(a927, a928)
    }
}

pub fn f44(v0: Adt1, v1: u64) -> R<u64> {
    let a934 = {
        let a930 = match v0.clone() {
            Adt1::C0(_, h929, _, _, _, _, _, _, _, _, _) => {
                h929
            }
        };
        let a931 = f29(v1);
        let a932 = 0u64;
        nat_quot(a930, a931, a932)
    };
    let a935 = f64(match v0 {
        Adt1::C0(h933, _, _, _, _, _, _, _, _, _, _) => {
            h933
        }
    }, v1);
    nat_mul(a934, a935)
}

pub fn f45(v0: u64) -> u64 {
    if if f18(v0) {
        true
    } else {
        let a936 = v0;
        let a937 = 26u64;
        nat_eq(a936, a937)
    } {
        f29(v0)
    } else {
        0u64
    }
}

pub fn f46(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a939 = match v0.clone() {
            Adt1::C0(_, h938, _, _, _, _, _, _, _, _, _) => {
                h938
            }
        };
        let a940 = 4u64;
        nat_mul(a939, a940)?
    };
    let v2: u64 = {
        let a943 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h941, _, _, _, _) => {
                h941
            }
        };
        let a944 = match v0.clone() {
            Adt1::C0(_, _, h942, _, _, _, _, _, _, _, _) => {
                h942
            }
        };
        nat_mul(a943, a944)?
    };
    let v3: u64 = {
        let a946 = v2;
        let a947 = match v0.clone() {
            Adt1::C0(_, _, _, h945, _, _, _, _, _, _, _) => {
                h945
            }
        };
        nat_mul(a946, a947)?
    };
    let a949 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h948, _, _, _, _, _) => {
            h948
        }
    };
    let a950 = 4u64;
    if if if if nat_eq(a949, a950) {
        let a955 = {
            let a952 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h951) => {
                    h951
                }
            };
            let a953 = 4u64;
            let a954 = 1u64;
            nat_rem(a952, a953, a954)
        };
        let a956 = 0u64;
        nat_eq(a955, a956)
    } else {
        false
    } {
        let a964 = {
            let a962 = match v0.clone() {
                Adt1::C0(_, _, h957, _, _, _, _, _, _, _, _) => {
                    h957
                }
            };
            let a963 = {
                let a960 = match v0.clone() {
                    Adt1::C0(_, _, _, h958, _, _, _, _, _, _, _) => {
                        h958
                    }
                };
                let a961 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h959, _, _, _, _, _, _) => {
                        h959
                    }
                };
                nat_mul(a960, a961)?
            };
            nat_mul(a962, a963)?
        };
        let a965 = 1u64;
        if nat_le(a964, a965) {
            true
        } else {
            let a967 = v1;
            let a968 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h966, _, _, _, _) => {
                    h966
                }
            };
            if nat_le(a967, a968) {
                let a973 = {
                    let a970 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, h969, _, _, _, _) => {
                            h969
                        }
                    };
                    let a971 = 4u64;
                    let a972 = 1u64;
                    nat_rem(a970, a971, a972)
                };
                let a974 = 0u64;
                nat_eq(a973, a974)
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a976 = match v0.clone() {
            Adt1::C0(_, _, _, h975, _, _, _, _, _, _, _) => {
                h975
            }
        };
        let a977 = 1u64;
        if nat_le(a976, a977) {
            true
        } else {
            let a979 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h978, _, _, _) => {
                    h978
                }
            };
            let a980 = v2;
            nat_eq(a979, a980)
        }
    } else {
        false
    } {
        let a982 = match v0.clone() {
            Adt1::C0(_, _, _, _, h981, _, _, _, _, _, _) => {
                h981
            }
        };
        let a983 = 1u64;
        if nat_le(a982, a983) {
            Ok(true)
        } else {
            let a985 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, h984, _, _) => {
                    h984
                }
            };
            let a986 = v3;
            Ok(nat_eq(a985, a986))
        }
    } else {
        Ok(false)
    }
}

pub fn f47(v0: Adt1, v1: Adt1) -> R<bool> {
    let a988 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h987, _) => {
            h987
        }
    };
    let a989 = 0u64;
    if nat_eq(a988, a989) {
        Ok(true)
    } else {
        let a991 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h990, _) => {
                h990
            }
        };
        let a992 = 0u64;
        if nat_eq(a991, a992) {
            Ok(true)
        } else {
            let a994 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h993) => {
                    h993
                }
            };
            let a995 = 0u64;
            if nat_eq(a994, a995) {
                Ok(true)
            } else {
                let a997 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h996) => {
                        h996
                    }
                };
                let a998 = 0u64;
                if nat_eq(a997, a998) {
                    Ok(true)
                } else {
                    let a1004 = {
                        let a1001 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h999) => {
                                h999
                            }
                        };
                        let a1002 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h1000, _) => {
                                h1000
                            }
                        };
                        nat_add(a1001, a1002)?
                    };
                    let a1005 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1003) => {
                            h1003
                        }
                    };
                    if nat_le(a1004, a1005) {
                        Ok(true)
                    } else {
                        let a1011 = {
                            let a1008 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1006) => {
                                    h1006
                                }
                            };
                            let a1009 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h1007, _) => {
                                    h1007
                                }
                            };
                            nat_add(a1008, a1009)?
                        };
                        let a1012 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1010) => {
                                h1010
                            }
                        };
                        Ok(nat_le(a1011, a1012))
                    }
                }
            }
        }
    }
}

pub fn f48(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a1015 = match v0.clone() {
            Adt1::C0(_, _, h1013, _, _, _, _, _, _, _, _) => {
                h1013
            }
        };
        let a1016 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1014, _, _, _, _) => {
                h1014
            }
        };
        nat_mul(a1015, a1016)?
    };
    if if f43(v0.clone()) {
        let a1018 = match v0.clone() {
            Adt1::C0(_, _, _, h1017, _, _, _, _, _, _, _) => {
                h1017
            }
        };
        let a1019 = 1u64;
        if nat_eq(a1018, a1019) {
            true
        } else {
            let a1021 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1020, _, _, _) => {
                    h1020
                }
            };
            let a1022 = v1;
            nat_eq(a1021, a1022)
        }
    } else {
        false
    } {
        let a1024 = match v0.clone() {
            Adt1::C0(_, _, _, _, h1023, _, _, _, _, _, _) => {
                h1023
            }
        };
        let a1025 = 1u64;
        if nat_eq(a1024, a1025) {
            Ok(true)
        } else {
            let a1030 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1026, _, _) => {
                    h1026
                }
            };
            let a1031 = {
                let a1028 = v1;
                let a1029 = match v0 {
                    Adt1::C0(_, _, _, h1027, _, _, _, _, _, _, _) => {
                        h1027
                    }
                };
                nat_mul(a1028, a1029)?
            };
            Ok(nat_eq(a1030, a1031))
        }
    } else {
        Ok(false)
    }
}

pub fn f49(v0: u64) -> u64 {
    let a1032 = v0;
    let a1033 = 3u64;
    if nat_eq(a1032, a1033) {
        16u64
    } else {
        10u64
    }
}

pub fn f50(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a1035 = v0;
        let a1036 = match v1.clone() {
            Adt1::C0(_, h1034, _, _, _, _, _, _, _, _, _) => {
                h1034
            }
        };
        let a1037 = 0u64;
        nat_quot(a1035, a1036, a1037)
    };
    let v3: u64 = {
        let a1039 = v2;
        let a1040 = match v1.clone() {
            Adt1::C0(_, _, h1038, _, _, _, _, _, _, _, _) => {
                h1038
            }
        };
        let a1041 = 0u64;
        nat_quot(a1039, a1040, a1041)
    };
    f52({
        let a1043 = v0;
        let a1044 = match v1.clone() {
            Adt1::C0(_, h1042, _, _, _, _, _, _, _, _, _) => {
                h1042
            }
        };
        let a1045 = 0u64;
        nat_rem(a1043, a1044, a1045)
    }, {
        let a1047 = v2;
        let a1048 = match v1.clone() {
            Adt1::C0(_, _, h1046, _, _, _, _, _, _, _, _) => {
                h1046
            }
        };
        let a1049 = 0u64;
        nat_rem(a1047, a1048, a1049)
    }, {
        let a1051 = v3;
        let a1052 = match v1.clone() {
            Adt1::C0(_, _, _, h1050, _, _, _, _, _, _, _) => {
                h1050
            }
        };
        let a1053 = 0u64;
        nat_rem(a1051, a1052, a1053)
    }, {
        let a1055 = v3;
        let a1056 = match v1.clone() {
            Adt1::C0(_, _, _, h1054, _, _, _, _, _, _, _) => {
                h1054
            }
        };
        let a1057 = 0u64;
        nat_quot(a1055, a1056, a1057)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h1058, _, _, _, _, _) => {
            h1058
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h1059, _, _, _, _) => {
            h1059
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1060, _, _, _) => {
            h1060
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1061, _, _) => {
            h1061
        }
    })
}

pub fn f51(v0: u64, v1: Adt1, v2: Adt1) -> R<u64> {
    let v3: u64 = {
        let a1063 = v0;
        let a1064 = match v1.clone() {
            Adt1::C0(_, h1062, _, _, _, _, _, _, _, _, _) => {
                h1062
            }
        };
        let a1065 = 0u64;
        nat_quot(a1063, a1064, a1065)
    };
    let v4: u64 = {
        let a1067 = v3;
        let a1068 = match v1.clone() {
            Adt1::C0(_, _, h1066, _, _, _, _, _, _, _, _) => {
                h1066
            }
        };
        let a1069 = 0u64;
        nat_quot(a1067, a1068, a1069)
    };
    f52({
        let a1075 = {
            let a1071 = v0;
            let a1072 = match v1.clone() {
                Adt1::C0(_, h1070, _, _, _, _, _, _, _, _, _) => {
                    h1070
                }
            };
            let a1073 = 0u64;
            nat_rem(a1071, a1072, a1073)
        };
        let a1076 = match v2.clone() {
            Adt1::C0(_, h1074, _, _, _, _, _, _, _, _, _) => {
                h1074
            }
        };
        let a1077 = 0u64;
        nat_rem(a1075, a1076, a1077)
    }, {
        let a1083 = {
            let a1079 = v3;
            let a1080 = match v1.clone() {
                Adt1::C0(_, _, h1078, _, _, _, _, _, _, _, _) => {
                    h1078
                }
            };
            let a1081 = 0u64;
            nat_rem(a1079, a1080, a1081)
        };
        let a1084 = match v2.clone() {
            Adt1::C0(_, _, h1082, _, _, _, _, _, _, _, _) => {
                h1082
            }
        };
        let a1085 = 0u64;
        nat_rem(a1083, a1084, a1085)
    }, {
        let a1091 = {
            let a1087 = v4;
            let a1088 = match v1.clone() {
                Adt1::C0(_, _, _, h1086, _, _, _, _, _, _, _) => {
                    h1086
                }
            };
            let a1089 = 0u64;
            nat_rem(a1087, a1088, a1089)
        };
        let a1092 = match v2.clone() {
            Adt1::C0(_, _, _, h1090, _, _, _, _, _, _, _) => {
                h1090
            }
        };
        let a1093 = 0u64;
        nat_rem(a1091, a1092, a1093)
    }, {
        let a1099 = {
            let a1095 = v4;
            let a1096 = match v1 {
                Adt1::C0(_, _, _, h1094, _, _, _, _, _, _, _) => {
                    h1094
                }
            };
            let a1097 = 0u64;
            nat_quot(a1095, a1096, a1097)
        };
        let a1100 = match v2.clone() {
            Adt1::C0(_, _, _, _, h1098, _, _, _, _, _, _) => {
                h1098
            }
        };
        let a1101 = 0u64;
        nat_rem(a1099, a1100, a1101)
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h1102, _, _, _, _, _) => {
            h1102
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h1103, _, _, _, _) => {
            h1103
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1104, _, _, _) => {
            h1104
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1105, _, _) => {
            h1105
        }
    })
}

pub fn f52(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a1118 = {
        let a1114 = {
            let a1110 = {
                let a1106 = v0;
                let a1107 = v4;
                nat_mul(a1106, a1107)?
            };
            let a1111 = {
                let a1108 = v1;
                let a1109 = v5;
                nat_mul(a1108, a1109)?
            };
            nat_add(a1110, a1111)?
        };
        let a1115 = {
            let a1112 = v2;
            let a1113 = v6;
            nat_mul(a1112, a1113)?
        };
        nat_add(a1114, a1115)?
    };
    let a1119 = {
        let a1116 = v3;
        let a1117 = v7;
        nat_mul(a1116, a1117)?
    };
    nat_add(a1118, a1119)
}

pub fn f53(v0: u64, v1: Adt1, v2: u64, v3: u64) -> R<Adt3> {
    let v4: u64 = {
        let a1120 = v0;
        let a1121 = 2u64;
        let a1122 = 0u64;
        nat_quot(a1120, a1121, a1122)
    };
    let v5: u64 = f42(match v1.clone() {
        Adt1::C0(h1123, _, _, _, _, _, _, _, _, _, _) => {
            h1123
        }
    });
    let a1127 = {
        let a1124 = v0;
        let a1125 = 2u64;
        let a1126 = 0u64;
        nat_rem(a1124, a1125, a1126)
    };
    let a1128 = 0u64;
    if nat_eq(a1127, a1128) {
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 4u64, f50(v4, v1)?, 2u64, 0u64, 0u64, 2u64, v5, 8u64))
    } else {
        let a1129 = v3;
        let a1130 = v2;
        if if nat_lt(a1129, a1130) {
            let a1133 = v3;
            let a1134 = {
                let a1131 = v5;
                let a1132 = 4u64;
                if nat_eq(a1131, a1132) {
                    2147483648u64
                } else {
                    9223372036854775808u64
                }
            };
            nat_lt(a1133, a1134)
        } else {
            false
        } {
            Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, {
                let a1135 = 3u64;
                let a1136 = v4;
                nat_add(a1135, a1136)?
            }, 8u64, 8u64))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f54(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64) -> R<Adt3> {
    let v5: u64 = {
        let a1138 = 1u64;
        let a1139 = f44(v1.clone(), match v3.clone() {
            Adt1::C0(h1137, _, _, _, _, _, _, _, _, _, _) => {
                h1137
            }
        })?;
        nat_add(a1138, a1139)?
    };
    let v6: u64 = {
        let a1140 = v0;
        let a1141 = v5;
        let a1142 = 0u64;
        nat_rem(a1140, a1141, a1142)
    };
    let v7: u64 = {
        let a1143 = v0;
        let a1144 = v5;
        let a1145 = 0u64;
        nat_quot(a1143, a1144, a1145)
    };
    let v8: u64 = {
        let a1147 = v7;
        let a1148 = match v1.clone() {
            Adt1::C0(_, _, h1146, _, _, _, _, _, _, _, _) => {
                h1146
            }
        };
        let a1149 = 0u64;
        nat_rem(a1147, a1148, a1149)
    };
    let v9: u64 = {
        let a1151 = v7;
        let a1152 = match v1.clone() {
            Adt1::C0(_, _, h1150, _, _, _, _, _, _, _, _) => {
                h1150
            }
        };
        let a1153 = 0u64;
        nat_quot(a1151, a1152, a1153)
    };
    let v10: u64 = {
        let a1155 = v9;
        let a1156 = match v1.clone() {
            Adt1::C0(_, _, _, h1154, _, _, _, _, _, _, _) => {
                h1154
            }
        };
        let a1157 = 0u64;
        nat_rem(a1155, a1156, a1157)
    };
    let v11: u64 = {
        let a1159 = v9;
        let a1160 = match v1.clone() {
            Adt1::C0(_, _, _, h1158, _, _, _, _, _, _, _) => {
                h1158
            }
        };
        let a1161 = 0u64;
        nat_quot(a1159, a1160, a1161)
    };
    let a1162 = v6;
    let a1163 = 0u64;
    if nat_eq(a1162, a1163) {
        let v12: u64 = {
            let a1180 = v8;
            let a1181 = {
                let a1178 = match v2.clone() {
                    Adt1::C0(_, h1164, _, _, _, _, _, _, _, _, _) => {
                        h1164
                    }
                };
                let a1179 = {
                    let a1176 = {
                        let a1166 = v10;
                        let a1167 = match v2.clone() {
                            Adt1::C0(_, _, h1165, _, _, _, _, _, _, _, _) => {
                                h1165
                            }
                        };
                        let a1168 = 0u64;
                        nat_rem(a1166, a1167, a1168)
                    };
                    let a1177 = {
                        let a1174 = match v2.clone() {
                            Adt1::C0(_, _, h1169, _, _, _, _, _, _, _, _) => {
                                h1169
                            }
                        };
                        let a1175 = {
                            let a1171 = v11;
                            let a1172 = match v2 {
                                Adt1::C0(_, _, _, h1170, _, _, _, _, _, _, _) => {
                                    h1170
                                }
                            };
                            let a1173 = 0u64;
                            nat_rem(a1171, a1172, a1173)
                        };
                        nat_mul(a1174, a1175)?
                    };
                    nat_add(a1176, a1177)?
                };
                nat_mul(a1178, a1179)?
            };
            nat_add(a1180, a1181)?
        };
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, {
            let a1182 = 3u64;
            let a1183 = v12;
            nat_add(a1182, a1183)?
        }, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
    } else {
        let a1185 = v4;
        let a1186 = match v3.clone() {
            Adt1::C0(_, _, h1184, _, _, _, _, _, _, _, _) => {
                h1184
            }
        };
        if nat_lt(a1185, a1186) {
            let v13: u64 = {
                let a1191 = {
                    let a1187 = v6;
                    let a1188 = 1u64;
                    nat_sub(a1187, a1188)
                };
                let a1192 = f64(match v1.clone() {
                    Adt1::C0(h1189, _, _, _, _, _, _, _, _, _, _) => {
                        h1189
                    }
                }, match v3.clone() {
                    Adt1::C0(h1190, _, _, _, _, _, _, _, _, _, _) => {
                        h1190
                    }
                });
                let a1193 = 0u64;
                nat_quot(a1191, a1192, a1193)
            };
            let v14: u64 = f52({
                let a1195 = v13;
                let a1196 = f29(match v3.clone() {
                    Adt1::C0(h1194, _, _, _, _, _, _, _, _, _, _) => {
                        h1194
                    }
                });
                nat_mul(a1195, a1196)?
            }, v8, v10, v11, match v1.clone() {
                Adt1::C0(_, _, _, _, _, h1197, _, _, _, _, _) => {
                    h1197
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, h1198, _, _, _, _) => {
                    h1198
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1199, _, _, _) => {
                    h1199
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1200, _, _) => {
                    h1200
                }
            })?;
            let v15: u64 = f52(v13, v4, v10, v11, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h1201, _, _, _, _, _) => {
                    h1201
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h1202, _, _, _, _) => {
                    h1202
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1203, _, _, _) => {
                    h1203
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1204, _, _) => {
                    h1204
                }
            })?;
            let a1206 = 0u64;
            let a1207 = f45(match v3.clone() {
                Adt1::C0(h1205, _, _, _, _, _, _, _, _, _, _) => {
                    h1205
                }
            });
            if nat_lt(a1206, a1207) {
                Ok(Adt3::C6(v14, v15))
            } else {
                Ok(f65(match v1 {
                    Adt1::C0(h1208, _, _, _, _, _, _, _, _, _, _) => {
                        h1208
                    }
                }, match v3 {
                    Adt1::C0(h1209, _, _, _, _, _, _, _, _, _, _) => {
                        h1209
                    }
                }, {
                    let a1210 = v6;
                    let a1211 = 1u64;
                    nat_sub(a1210, a1211)
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
    let a1212 = v0;
    let a1213 = 8u64;
    if nat_eq(a1212, a1213) {
        Adt3::C3(1u64, 6u64, 0u64, 0u64, 1u64, 0u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
    } else {
        let a1214 = v0;
        let a1215 = 9u64;
        if nat_eq(a1214, a1215) {
            Adt3::C3(1u64, 2u64, 0u64, 0u64, 2u64, v1, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
        } else {
            let a1216 = v0;
            let a1217 = 10u64;
            if nat_eq(a1216, a1217) {
                Adt3::C3(5u64, 16u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
            } else {
                let a1218 = v0;
                let a1219 = 11u64;
                if nat_eq(a1218, a1219) {
                    Adt3::C3(1u64, 0u64, 0u64, 0u64, 2u64, 1065353216u64, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
                } else {
                    let a1220 = v0;
                    let a1221 = 12u64;
                    if nat_eq(a1220, a1221) {
                        Adt3::C3(1u64, 3u64, 0u64, 0u64, 1u64, 0u64, 1u64, 2u64, 0u64, 0u64, 4u64, 4u64)
                    } else {
                        let a1222 = v0;
                        let a1223 = 13u64;
                        if nat_eq(a1222, a1223) {
                            Adt3::C3(1u64, 0u64, 0u64, 0u64, 1u64, 1u64, 2u64, 1065353216u64, 0u64, 1u64, 4u64, 4u64)
                        } else {
                            let a1224 = v0;
                            let a1225 = 14u64;
                            if nat_eq(a1224, a1225) {
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
    let a1226 = v0;
    let a1227 = 0u64;
    if nat_eq(a1226, a1227) {
        Adt4::C1(Adt5::C0(0u64, 1u64, 4u64))
    } else {
        let a1228 = v0;
        let a1229 = 1u64;
        if nat_eq(a1228, a1229) {
            Adt4::C1(Adt5::C0(1u64, 1u64, 2u64))
        } else {
            let a1230 = v0;
            let a1231 = 2u64;
            if nat_eq(a1230, a1231) {
                Adt4::C1(Adt5::C0(2u64, 32u64, 18u64))
            } else {
                let a1232 = v0;
                let a1233 = 3u64;
                if nat_eq(a1232, a1233) {
                    Adt4::C1(Adt5::C0(3u64, 32u64, 20u64))
                } else {
                    let a1234 = v0;
                    let a1235 = 6u64;
                    if nat_eq(a1234, a1235) {
                        Adt4::C1(Adt5::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1236 = v0;
                        let a1237 = 7u64;
                        if nat_eq(a1236, a1237) {
                            Adt4::C1(Adt5::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1238 = v0;
                            let a1239 = 8u64;
                            if nat_eq(a1238, a1239) {
                                Adt4::C1(Adt5::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1240 = v0;
                                let a1241 = 9u64;
                                if nat_eq(a1240, a1241) {
                                    Adt4::C1(Adt5::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1242 = v0;
                                    let a1243 = 10u64;
                                    if nat_eq(a1242, a1243) {
                                        Adt4::C1(Adt5::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1244 = v0;
                                        let a1245 = 11u64;
                                        if nat_eq(a1244, a1245) {
                                            Adt4::C1(Adt5::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1246 = v0;
                                            let a1247 = 12u64;
                                            if nat_eq(a1246, a1247) {
                                                Adt4::C1(Adt5::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1248 = v0;
                                                let a1249 = 13u64;
                                                if nat_eq(a1248, a1249) {
                                                    Adt4::C1(Adt5::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1250 = v0;
                                                    let a1251 = 14u64;
                                                    if nat_eq(a1250, a1251) {
                                                        Adt4::C1(Adt5::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1252 = v0;
                                                        let a1253 = 15u64;
                                                        if nat_eq(a1252, a1253) {
                                                            Adt4::C1(Adt5::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1254 = v0;
                                                            let a1255 = 16u64;
                                                            if nat_eq(a1254, a1255) {
                                                                Adt4::C1(Adt5::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1256 = v0;
                                                                let a1257 = 17u64;
                                                                if nat_eq(a1256, a1257) {
                                                                    Adt4::C1(Adt5::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1258 = v0;
                                                                    let a1259 = 18u64;
                                                                    if nat_eq(a1258, a1259) {
                                                                        Adt4::C1(Adt5::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1260 = v0;
                                                                        let a1261 = 19u64;
                                                                        if nat_eq(a1260, a1261) {
                                                                            Adt4::C1(Adt5::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1262 = v0;
                                                                            let a1263 = 20u64;
                                                                            if nat_eq(a1262, a1263) {
                                                                                Adt4::C1(Adt5::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1264 = v0;
                                                                                let a1265 = 21u64;
                                                                                if nat_eq(a1264, a1265) {
                                                                                    Adt4::C1(Adt5::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1266 = v0;
                                                                                    let a1267 = 22u64;
                                                                                    if nat_eq(a1266, a1267) {
                                                                                        Adt4::C1(Adt5::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1268 = v0;
                                                                                        let a1269 = 23u64;
                                                                                        if nat_eq(a1268, a1269) {
                                                                                            Adt4::C1(Adt5::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1270 = v0;
                                                                                            let a1271 = 24u64;
                                                                                            if nat_eq(a1270, a1271) {
                                                                                                Adt4::C1(Adt5::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1272 = v0;
                                                                                                let a1273 = 25u64;
                                                                                                if nat_eq(a1272, a1273) {
                                                                                                    Adt4::C1(Adt5::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1274 = v0;
                                                                                                    let a1275 = 26u64;
                                                                                                    if nat_eq(a1274, a1275) {
                                                                                                        Adt4::C1(Adt5::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1276 = v0;
                                                                                                        let a1277 = 27u64;
                                                                                                        if nat_eq(a1276, a1277) {
                                                                                                            Adt4::C1(Adt5::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1278 = v0;
                                                                                                            let a1279 = 28u64;
                                                                                                            if nat_eq(a1278, a1279) {
                                                                                                                Adt4::C1(Adt5::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1280 = v0;
                                                                                                                let a1281 = 29u64;
                                                                                                                if nat_eq(a1280, a1281) {
                                                                                                                    Adt4::C1(Adt5::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1282 = v0;
                                                                                                                    let a1283 = 30u64;
                                                                                                                    if nat_eq(a1282, a1283) {
                                                                                                                        Adt4::C1(Adt5::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1284 = v0;
                                                                                                                        let a1285 = 34u64;
                                                                                                                        if nat_eq(a1284, a1285) {
                                                                                                                            Adt4::C1(Adt5::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1286 = v0;
                                                                                                                            let a1287 = 35u64;
                                                                                                                            if nat_eq(a1286, a1287) {
                                                                                                                                Adt4::C1(Adt5::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1288 = v0;
                                                                                                                                let a1289 = 39u64;
                                                                                                                                if nat_eq(a1288, a1289) {
                                                                                                                                    Adt4::C1(Adt5::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1290 = v0;
                                                                                                                                    let a1291 = 40u64;
                                                                                                                                    if nat_eq(a1290, a1291) {
                                                                                                                                        Adt4::C1(Adt5::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1292 = v0;
                                                                                                                                        let a1293 = 41u64;
                                                                                                                                        if nat_eq(a1292, a1293) {
                                                                                                                                            Adt4::C1(Adt5::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1294 = v0;
                                                                                                                                            let a1295 = 42u64;
                                                                                                                                            if nat_eq(a1294, a1295) {
                                                                                                                                                Adt4::C1(Adt5::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1296 = v0;
                                                                                                                                                let a1297 = 43u64;
                                                                                                                                                if nat_eq(a1296, a1297) {
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
        let a1300 = match v0.clone() {
            Adt1::C0(h1298, _, _, _, _, _, _, _, _, _, _) => {
                h1298
            }
        };
        let a1301 = match v1.clone() {
            Adt1::C0(h1299, _, _, _, _, _, _, _, _, _, _) => {
                h1299
            }
        };
        nat_eq(a1300, a1301)
    } else {
        false
    } {
        let a1304 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1302) => {
                h1302
            }
        };
        let a1305 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1303) => {
                h1303
            }
        };
        nat_eq(a1304, a1305)
    } else {
        false
    } {
        let a1308 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1306, _, _, _, _, _) => {
                h1306
            }
        };
        let a1309 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1307, _, _, _, _, _) => {
                h1307
            }
        };
        nat_eq(a1308, a1309)
    } else {
        false
    } {
        let a1312 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1310, _, _, _, _) => {
                h1310
            }
        };
        let a1313 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1311, _, _, _, _) => {
                h1311
            }
        };
        nat_eq(a1312, a1313)
    } else {
        false
    } {
        let a1316 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1314, _, _, _) => {
                h1314
            }
        };
        let a1317 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1315, _, _, _) => {
                h1315
            }
        };
        nat_eq(a1316, a1317)
    } else {
        false
    } {
        let a1320 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1318, _, _) => {
                h1318
            }
        };
        let a1321 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1319, _, _) => {
                h1319
            }
        };
        nat_eq(a1320, a1321)
    } else {
        false
    }
}

pub fn f60(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a1322 = v1;
    let a1323 = 1u64;
    if if if nat_le(a1322, a1323) {
        true
    } else {
        let a1324 = v0;
        let a1325 = v2;
        nat_le(a1324, a1325)
    } {
        let a1326 = v3;
        let a1327 = 1u64;
        if nat_le(a1326, a1327) {
            true
        } else {
            let a1334 = {
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
            let a1335 = v4;
            nat_le(a1334, a1335)
        }
    } else {
        false
    } {
        let a1336 = v5;
        let a1337 = 1u64;
        if nat_le(a1336, a1337) {
            Ok(true)
        } else {
            let a1350 = {
                let a1348 = {
                    let a1342 = v0;
                    let a1343 = {
                        let a1340 = {
                            let a1338 = v1;
                            let a1339 = 1u64;
                            nat_sub(a1338, a1339)
                        };
                        let a1341 = v2;
                        nat_mul(a1340, a1341)?
                    };
                    nat_add(a1342, a1343)?
                };
                let a1349 = {
                    let a1346 = {
                        let a1344 = v3;
                        let a1345 = 1u64;
                        nat_sub(a1344, a1345)
                    };
                    let a1347 = v4;
                    nat_mul(a1346, a1347)?
                };
                nat_add(a1348, a1349)?
            };
            let a1351 = v6;
            Ok(nat_le(a1350, a1351))
        }
    } else {
        Ok(false)
    }
}

pub fn f61(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1352 = v0;
    let a1353 = 0u64;
    if if if nat_eq(a1352, a1353) {
        true
    } else {
        let a1354 = v1;
        let a1355 = 0u64;
        nat_eq(a1354, a1355)
    } {
        true
    } else {
        let a1356 = v2;
        let a1357 = 0u64;
        nat_eq(a1356, a1357)
    } {
        true
    } else {
        let a1358 = v3;
        let a1359 = 0u64;
        nat_eq(a1358, a1359)
    }
}

pub fn f62(v0: u64, v1: u64, v2: u64) -> bool {
    let a1360 = v0;
    let a1361 = 1u64;
    if nat_le(a1360, a1361) {
        true
    } else {
        let a1365 = {
            let a1362 = v1;
            let a1363 = v2;
            let a1364 = 1u64;
            nat_rem(a1362, a1363, a1364)
        };
        let a1366 = 0u64;
        nat_eq(a1365, a1366)
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
    let a1367 = v0;
    let a1368 = 1u64;
    if if nat_eq(a1367, a1368) {
        let a1369 = v1;
        let a1370 = 43u64;
        nat_eq(a1369, a1370)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f65(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt3 {
    let a1371 = f64(v0, v1);
    let a1372 = 2u64;
    if nat_eq(a1371, a1372) {
        let a1376 = {
            let a1373 = v2;
            let a1374 = 2u64;
            let a1375 = 0u64;
            nat_rem(a1373, a1374, a1375)
        };
        let a1377 = 0u64;
        if nat_eq(a1376, a1377) {
            Adt3::C3(0u64, 0u64, v0, 0u64, 0u64, v3, 2u64, 0u64, 1u64, 0u64, f21(v0), 8u64)
        } else {
            Adt3::C3(0u64, 0u64, 0u64, v1, 1u64, 0u64, 2u64, 0u64, 3u64, v4, 8u64, f28(v1))
        }
    } else {
        Adt3::C3(0u64, 0u64, v0, v1, 0u64, v3, 2u64, 0u64, 3u64, v4, f21(v0), f28(v1))
    }
}
