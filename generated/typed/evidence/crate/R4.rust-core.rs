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
                f20(v1)
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
        Ok(Adt3::C2(Adt2::C0(f38(f39(v1.clone())?, v4, f40(match v1.clone() {
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
        let a144 = f42(match v3.clone() {
            Adt1::C0(h142, _, _, _, _, _, _, _, _, _, _) => {
                h142
            }
        });
        nat_lt(a143, a144)
    } else {
        false
    } {
        let a146 = 0u64;
        let a147 = f43(match v1.clone() {
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
        f44(v0.clone())
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
            let a200 = f42(match v3.clone() {
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
                    let a212 = f45(v0.clone(), match v3.clone() {
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
        }), 41u64, 0u64, 0u64, 0u64, v0, v1, v3.clone(), false, f46(match v3 {
            Adt1::C0(h220, _, _, _, _, _, _, _, _, _, _) => {
                h220
            }
        }))))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f6(v0: Adt1, v1: Adt1, v2: Adt1) -> R<Adt3> {
    let v3: u64 = f28(match v2.clone() {
        Adt1::C0(h221, _, _, _, _, _, _, _, _, _, _) => {
            h221
        }
    });
    if if if if if if if f18(match v2.clone() {
        Adt1::C0(h222, _, _, _, _, _, _, _, _, _, _) => {
            h222
        }
    }) {
        f41(v1.clone(), v2.clone())
    } else {
        false
    } {
        let a224 = 0u64;
        let a225 = f21(match v0.clone() {
            Adt1::C0(h223, _, _, _, _, _, _, _, _, _, _) => {
                h223
            }
        });
        nat_lt(a224, a225)
    } else {
        false
    } {
        let a228 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h226, _, _, _, _, _) => {
                h226
            }
        };
        let a229 = f21(match v0.clone() {
            Adt1::C0(h227, _, _, _, _, _, _, _, _, _, _) => {
                h227
            }
        });
        nat_eq(a228, a229)
    } else {
        false
    } {
        let a234 = {
            let a231 = match v0.clone() {
                Adt1::C0(_, h230, _, _, _, _, _, _, _, _, _) => {
                    h230
                }
            };
            let a232 = v3;
            let a233 = 1u64;
            nat_rem(a231, a232, a233)
        };
        let a235 = 0u64;
        if nat_eq(a234, a235) {
            let a240 = {
                let a237 = match v2.clone() {
                    Adt1::C0(_, h236, _, _, _, _, _, _, _, _, _) => {
                        h236
                    }
                };
                let a238 = v3;
                let a239 = 1u64;
                nat_rem(a237, a238, a239)
            };
            let a241 = 0u64;
            nat_eq(a240, a241)
        } else {
            false
        }
    } else {
        false
    } {
        let a244 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h242, _, _, _, _, _) => {
                h242
            }
        };
        let a245 = f42(match v2.clone() {
            Adt1::C0(h243, _, _, _, _, _, _, _, _, _, _) => {
                h243
            }
        });
        nat_eq(a244, a245)
    } else {
        false
    } {
        let a246 = f39(v0.clone())?;
        let a247 = f39(v2.clone())?;
        nat_eq(a246, a247)
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a248 = f39(v0.clone())?;
            let a249 = v3;
            let a250 = 0u64;
            nat_quot(a248, a249, a250)
        }, 3u64, f32(match v2.clone() {
            Adt1::C0(h251, _, _, _, _, _, _, _, _, _, _) => {
                h251
            }
        }), 33u64, 0u64, 0u64, 0u64, v0, v1, v2, false, v3)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f7(v0: u64, v1: u64, v2: Adt1, v3: Adt1) -> R<Adt3> {
    let a253 = match v2.clone() {
        Adt1::C0(h252, _, _, _, _, _, _, _, _, _, _) => {
            h252
        }
    };
    let a254 = 0u64;
    if if if if if if if if if if if nat_eq(a253, a254) {
        let a256 = match v3.clone() {
            Adt1::C0(h255, _, _, _, _, _, _, _, _, _, _) => {
                h255
            }
        };
        let a257 = 26u64;
        nat_eq(a256, a257)
    } else {
        false
    } {
        let a259 = match v2.clone() {
            Adt1::C0(_, h258, _, _, _, _, _, _, _, _, _) => {
                h258
            }
        };
        let a260 = 2147483647u64;
        nat_le(a259, a260)
    } else {
        false
    } {
        let a263 = match v3.clone() {
            Adt1::C0(_, h261, _, _, _, _, _, _, _, _, _) => {
                h261
            }
        };
        let a264 = match v2.clone() {
            Adt1::C0(_, h262, _, _, _, _, _, _, _, _, _) => {
                h262
            }
        };
        nat_le(a263, a264)
    } else {
        false
    } {
        let a267 = match v2.clone() {
            Adt1::C0(_, _, h265, _, _, _, _, _, _, _, _) => {
                h265
            }
        };
        let a268 = match v3.clone() {
            Adt1::C0(_, _, h266, _, _, _, _, _, _, _, _) => {
                h266
            }
        };
        nat_eq(a267, a268)
    } else {
        false
    } {
        let a271 = match v2.clone() {
            Adt1::C0(_, _, _, h269, _, _, _, _, _, _, _) => {
                h269
            }
        };
        let a272 = match v3.clone() {
            Adt1::C0(_, _, _, h270, _, _, _, _, _, _, _) => {
                h270
            }
        };
        nat_eq(a271, a272)
    } else {
        false
    } {
        let a275 = match v2.clone() {
            Adt1::C0(_, _, _, _, h273, _, _, _, _, _, _) => {
                h273
            }
        };
        let a276 = match v3.clone() {
            Adt1::C0(_, _, _, _, h274, _, _, _, _, _, _) => {
                h274
            }
        };
        nat_eq(a275, a276)
    } else {
        false
    } {
        let a277 = v0;
        let a278 = 67u64;
        if nat_eq(a277, a278) {
            true
        } else {
            let a279 = v1;
            let a280 = 1u64;
            if nat_le(a279, a280) {
                let a283 = match v3.clone() {
                    Adt1::C0(_, h281, _, _, _, _, _, _, _, _, _) => {
                        h281
                    }
                };
                let a284 = match v2.clone() {
                    Adt1::C0(_, h282, _, _, _, _, _, _, _, _, _) => {
                        h282
                    }
                };
                nat_eq(a283, a284)
            } else {
                false
            }
        }
    } else {
        false
    } {
        f47(v2.clone())?
    } else {
        false
    } {
        f47(v3.clone())?
    } else {
        false
    } {
        f48(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a290 = match v3.clone() {
                Adt1::C0(_, _, h285, _, _, _, _, _, _, _, _) => {
                    h285
                }
            };
            let a291 = {
                let a288 = match v3.clone() {
                    Adt1::C0(_, _, _, h286, _, _, _, _, _, _, _) => {
                        h286
                    }
                };
                let a289 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h287, _, _, _, _, _, _) => {
                        h287
                    }
                };
                nat_mul(a288, a289)?
            };
            nat_mul(a290, a291)?
        }, 3u64, None::<u64>, v0, {
            let a292 = v0;
            let a293 = 67u64;
            if nat_eq(a292, a293) {
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
    let a294 = v0;
    let a295 = 6u64;
    nat_le(a294, a295)
}

pub fn f9(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1) -> R<Adt3> {
    let a296 = v2;
    let a297 = 1u64;
    if if if if if if if if if if if if if if if nat_le(a296, a297) {
        let a299 = 0u64;
        let a300 = f21(match v5.clone() {
            Adt1::C0(h298, _, _, _, _, _, _, _, _, _, _) => {
                h298
            }
        });
        nat_lt(a299, a300)
    } else {
        false
    } {
        let a303 = match v5.clone() {
            Adt1::C0(h301, _, _, _, _, _, _, _, _, _, _) => {
                h301
            }
        };
        let a304 = match v6.clone() {
            Adt1::C0(h302, _, _, _, _, _, _, _, _, _, _) => {
                h302
            }
        };
        nat_eq(a303, a304)
    } else {
        false
    } {
        let a307 = match v5.clone() {
            Adt1::C0(h305, _, _, _, _, _, _, _, _, _, _) => {
                h305
            }
        };
        let a308 = match v7.clone() {
            Adt1::C0(h306, _, _, _, _, _, _, _, _, _, _) => {
                h306
            }
        };
        nat_eq(a307, a308)
    } else {
        false
    } {
        f24(v5.clone(), v6.clone())
    } else {
        false
    } {
        f49(v5.clone())?
    } else {
        false
    } {
        f49(v6.clone())?
    } else {
        false
    } {
        f49(v7.clone())?
    } else {
        false
    } {
        let a311 = match v5.clone() {
            Adt1::C0(_, _, h309, _, _, _, _, _, _, _, _) => {
                h309
            }
        };
        let a312 = match v7.clone() {
            Adt1::C0(_, _, h310, _, _, _, _, _, _, _, _) => {
                h310
            }
        };
        nat_eq(a311, a312)
    } else {
        false
    } {
        let a315 = match v5.clone() {
            Adt1::C0(_, _, _, h313, _, _, _, _, _, _, _) => {
                h313
            }
        };
        let a316 = match v7.clone() {
            Adt1::C0(_, _, _, h314, _, _, _, _, _, _, _) => {
                h314
            }
        };
        nat_eq(a315, a316)
    } else {
        false
    } {
        let a319 = match v5.clone() {
            Adt1::C0(_, _, _, _, h317, _, _, _, _, _, _) => {
                h317
            }
        };
        let a320 = match v7.clone() {
            Adt1::C0(_, _, _, _, h318, _, _, _, _, _, _) => {
                h318
            }
        };
        nat_eq(a319, a320)
    } else {
        false
    } {
        let a327 = match v7.clone() {
            Adt1::C0(_, h321, _, _, _, _, _, _, _, _, _) => {
                h321
            }
        };
        let a328 = if v1 {
            match v5.clone() {
                Adt1::C0(_, h322, _, _, _, _, _, _, _, _, _) => {
                    h322
                }
            }
        } else {
            let a324 = match v5.clone() {
                Adt1::C0(_, h323, _, _, _, _, _, _, _, _, _) => {
                    h323
                }
            };
            let a325 = 2u64;
            let a326 = 0u64;
            nat_quot(a324, a325, a326)
        };
        nat_eq(a327, a328)
    } else {
        false
    } {
        let a330 = match v7.clone() {
            Adt1::C0(_, h329, _, _, _, _, _, _, _, _, _) => {
                h329
            }
        };
        let a331 = 2147483647u64;
        nat_le(a330, a331)
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
            let a334 = 1u64;
            let a335 = {
                let a332 = f50(v0);
                let a333 = f39(v7.clone())?;
                nat_mul(a332, a333)?
            };
            nat_add(a334, a335)?
        } else {
            let a336 = 4u64;
            let a337 = f39(v7.clone())?;
            nat_mul(a336, a337)?
        }, if f14(v0) {
            4u64
        } else {
            3u64
        }, None::<u64>, if v1 {
            98u64
        } else {
            let a338 = 99u64;
            let a339 = v2;
            nat_add(a338, a339)?
        }, v0, v3, v4, v5, v6, v7, false, 0u64)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f10(v0: u64, v1: Adt2) -> R<Adt3> {
    let a341 = v0;
    let a342 = match v1.clone() {
        Adt2::C0(h340, _, _, _, _, _, _, _, _, _, _, _) => {
            h340
        }
    };
    if nat_lt(a341, a342) {
        let v2: u64 = {
            let a346 = v0;
            let a347 = {
                let m345 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h344, _, _, _, _) => {
                        h344
                    }
                };
                match m345 {
                    Adt1::C0(_, h343, _, _, _, _, _, _, _, _, _) => {
                        h343
                    }
                }
            };
            nat_mul(a346, a347)?
        };
        Ok(Adt3::C7(f51(v2, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h348, _, _, _, _) => {
                h348
            }
        })?, f52(v2, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h349, _, _, _, _) => {
                h349
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h350, _, _, _) => {
                h350
            }
        })?, f51(v2, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h351, _, _) => {
                h351
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f11(v0: u64, v1: Adt2) -> R<Adt3> {
    let a353 = v0;
    let a354 = match v1.clone() {
        Adt2::C0(h352, _, _, _, _, _, _, _, _, _, _, _) => {
            h352
        }
    };
    if nat_lt(a353, a354) {
        let v2: Adt1 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h355, _, _) => {
                h355
            }
        };
        let v3: u64 = f28(match v2.clone() {
            Adt1::C0(h356, _, _, _, _, _, _, _, _, _, _) => {
                h356
            }
        });
        let v4: u64 = {
            let a358 = match v2.clone() {
                Adt1::C0(_, h357, _, _, _, _, _, _, _, _, _) => {
                    h357
                }
            };
            let a359 = v3;
            let a360 = 0u64;
            nat_quot(a358, a359, a360)
        };
        let v5: u64 = {
            let a361 = v0;
            let a362 = v4;
            let a363 = 0u64;
            nat_quot(a361, a362, a363)
        };
        let v6: u64 = {
            let a365 = v5;
            let a366 = match v2.clone() {
                Adt1::C0(_, _, h364, _, _, _, _, _, _, _, _) => {
                    h364
                }
            };
            let a367 = 0u64;
            nat_quot(a365, a366, a367)
        };
        Ok(Adt3::C6(f51({
            let a368 = v0;
            let a369 = v3;
            nat_mul(a368, a369)?
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, h370, _, _, _, _) => {
                h370
            }
        })?, f53({
            let a371 = v0;
            let a372 = v4;
            let a373 = 0u64;
            nat_rem(a371, a372, a373)
        }, {
            let a375 = v5;
            let a376 = match v2.clone() {
                Adt1::C0(_, _, h374, _, _, _, _, _, _, _, _) => {
                    h374
                }
            };
            let a377 = 0u64;
            nat_rem(a375, a376, a377)
        }, {
            let a379 = v6;
            let a380 = match v2.clone() {
                Adt1::C0(_, _, _, h378, _, _, _, _, _, _, _) => {
                    h378
                }
            };
            let a381 = 0u64;
            nat_rem(a379, a380, a381)
        }, {
            let a383 = v6;
            let a384 = match v2.clone() {
                Adt1::C0(_, _, _, h382, _, _, _, _, _, _, _) => {
                    h382
                }
            };
            let a385 = 0u64;
            nat_quot(a383, a384, a385)
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, h386, _, _, _, _, _) => {
                h386
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h387, _, _, _, _) => {
                h387
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h388, _, _, _) => {
                h388
            }
        }, match v2 {
            Adt1::C0(_, _, _, _, _, _, _, _, h389, _, _) => {
                h389
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f12(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h390, _, _, _, _) => {
            h390
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h391, _, _) => {
            h391
        }
    };
    let a393 = v0;
    let a394 = match v1.clone() {
        Adt2::C0(h392, _, _, _, _, _, _, _, _, _, _, _) => {
            h392
        }
    };
    if nat_lt(a393, a394) {
        Ok(Adt3::C5({
            let a396 = v0;
            let a397 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, _, h395, _, _, _, _) => {
                    h395
                }
            };
            nat_mul(a396, a397)?
        }, {
            let a399 = v0;
            let a400 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h398, _, _, _, _) => {
                    h398
                }
            };
            nat_mul(a399, a400)?
        }, match v2 {
            Adt1::C0(_, h401, _, _, _, _, _, _, _, _, _) => {
                h401
            }
        }, match v3 {
            Adt1::C0(_, h402, _, _, _, _, _, _, _, _, _) => {
                h402
            }
        }, {
            let a404 = match v1 {
                Adt2::C0(_, _, _, _, h403, _, _, _, _, _, _, _) => {
                    h403
                }
            };
            let a405 = 1u64;
            nat_eq(a404, a405)
        }))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f13(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let a407 = v0;
    let a408 = match v1.clone() {
        Adt2::C0(h406, _, _, _, _, _, _, _, _, _, _, _) => {
            h406
        }
    };
    if nat_lt(a407, a408) {
        let v3: u64 = {
            let a410 = 2u64;
            let a411 = f39(match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h409, _, _, _) => {
                    h409
                }
            })?;
            nat_mul(a410, a411)?
        };
        let a412 = v0;
        let a413 = v3;
        if nat_lt(a412, a413) {
            f54(v0, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h414, _, _, _) => {
                    h414
                }
            }, {
                let m417 = match v1 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h416, _, _) => {
                        h416
                    }
                };
                match m417 {
                    Adt1::C0(_, _, h415, _, _, _, _, _, _, _, _) => {
                        h415
                    }
                }
            }, v2)
        } else {
            f55({
                let a418 = v0;
                let a419 = v3;
                nat_sub(a418, a419)
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h420, _, _, _, _) => {
                    h420
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h421, _, _, _) => {
                    h421
                }
            }, match v1 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h422, _, _) => {
                    h422
                }
            }, v2)
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f14(v0: u64) -> bool {
    let a423 = v0;
    let a424 = 3u64;
    if nat_eq(a423, a424) {
        true
    } else {
        let a425 = v0;
        let a426 = 6u64;
        nat_eq(a425, a426)
    }
}

pub fn f15(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h427, _, _, _, _, _) => {
            h427
        }
    };
    let a429 = v0;
    let a430 = match v1.clone() {
        Adt2::C0(h428, _, _, _, _, _, _, _, _, _, _, _) => {
            h428
        }
    };
    if nat_lt(a429, a430) {
        let a431 = v0;
        let a432 = 0u64;
        if nat_eq(a431, a432) {
            Ok(Adt3::C3(1u64, 6u64, 0u64, 0u64, 2u64, v3, 2u64, 0u64, 0u64, 3u64, 4u64, 4u64))
        } else {
            let v4: u64 = {
                let a433 = v0;
                let a434 = 1u64;
                nat_sub(a433, a434)
            };
            let v5: u64 = {
                let a436 = v4;
                let a437 = f50(match v1.clone() {
                    Adt2::C0(_, _, _, _, h435, _, _, _, _, _, _, _) => {
                        h435
                    }
                });
                let a438 = 0u64;
                nat_rem(a436, a437, a438)
            };
            let v6: u64 = {
                let a443 = 4u64;
                let a444 = {
                    let a440 = v4;
                    let a441 = f50(match v1.clone() {
                        Adt2::C0(_, _, _, _, h439, _, _, _, _, _, _, _) => {
                            h439
                        }
                    });
                    let a442 = 0u64;
                    nat_quot(a440, a441, a442)
                };
                nat_mul(a443, a444)?
            };
            let a445 = v5;
            let a446 = 2u64;
            if nat_lt(a445, a446) {
                f16({
                    let a447 = v6;
                    let a448 = v5;
                    nat_add(a447, a448)?
                }, v1)
            } else {
                let a449 = v5;
                let a450 = 2u64;
                if nat_eq(a449, a450) {
                    Ok(f56(0u64, 2u64, v3))
                } else {
                    let a451 = v5;
                    let a452 = 3u64;
                    if nat_eq(a451, a452) {
                        Ok(f57(0u64, 2u64, v3, {
                            let a453 = v2;
                            let a454 = 2u64;
                            nat_eq(a453, a454)
                        }))
                    } else {
                        let a455 = v5;
                        let a456 = 4u64;
                        if nat_eq(a455, a456) {
                            Ok(f56(1u64, 1u64, 3u64))
                        } else {
                            let a457 = v5;
                            let a458 = 5u64;
                            if nat_eq(a457, a458) {
                                Ok(f57(1u64, 1u64, 3u64, {
                                    let a459 = v2;
                                    let a460 = 0u64;
                                    nat_eq(a459, a460)
                                }))
                            } else {
                                let a461 = v5;
                                let a462 = 6u64;
                                if nat_eq(a461, a462) {
                                    Ok(f56(1u64, 2u64, v3))
                                } else {
                                    let a463 = v5;
                                    let a464 = 7u64;
                                    if nat_eq(a463, a464) {
                                        Ok(f57(1u64, 2u64, v3, {
                                            let a465 = v2;
                                            let a466 = 2u64;
                                            nat_eq(a465, a466)
                                        }))
                                    } else {
                                        let a468 = match v1.clone() {
                                            Adt2::C0(_, _, _, _, h467, _, _, _, _, _, _, _) => {
                                                h467
                                            }
                                        };
                                        let a469 = 3u64;
                                        if nat_eq(a468, a469) {
                                            let a470 = v5;
                                            let a471 = 15u64;
                                            if nat_eq(a470, a471) {
                                                f16({
                                                    let a472 = v6;
                                                    let a473 = 3u64;
                                                    nat_add(a472, a473)?
                                                }, v1)
                                            } else {
                                                Ok(f58(v5, match v1 {
                                                    Adt2::C0(_, _, _, _, _, h474, _, _, _, _, _, _) => {
                                                        h474
                                                    }
                                                }))
                                            }
                                        } else {
                                            let a475 = v5;
                                            let a476 = 8u64;
                                            if nat_eq(a475, a476) {
                                                Ok(Adt3::C4(2u64, {
                                                    let m479 = match v1 {
                                                        Adt2::C0(_, _, _, _, _, _, _, _, _, h478, _, _) => {
                                                            h478
                                                        }
                                                    };
                                                    match m479 {
                                                        Adt1::C0(h477, _, _, _, _, _, _, _, _, _, _) => {
                                                            h477
                                                        }
                                                    }
                                                }, 0u64, 1u64, 0u64))
                                            } else {
                                                f16({
                                                    let a480 = v6;
                                                    let a481 = 3u64;
                                                    nat_add(a480, a481)?
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
    let a483 = v0;
    let a484 = match v1.clone() {
        Adt2::C0(h482, _, _, _, _, _, _, _, _, _, _, _) => {
            h482
        }
    };
    if nat_lt(a483, a484) {
        f59({
            let a485 = v0;
            let a486 = 4u64;
            let a487 = 0u64;
            nat_rem(a485, a486, a487)
        }, {
            let a488 = v0;
            let a489 = 4u64;
            let a490 = 0u64;
            nat_quot(a488, a489, a490)
        }, v1)
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f17(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h491, _, _, _, _) => {
            h491
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h492, _, _, _) => {
            h492
        }
    };
    let v5: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h493, _, _) => {
            h493
        }
    };
    let v6: u64 = {
        let a495 = match v1.clone() {
            Adt2::C0(_, h494, _, _, _, _, _, _, _, _, _, _) => {
                h494
            }
        };
        let a496 = 3u64;
        if nat_eq(a495, a496) {
            0u64
        } else {
            let a497 = f39(v3.clone())?;
            let a498 = 2u64;
            nat_mul(a497, a498)?
        }
    };
    let a500 = match v1.clone() {
        Adt2::C0(h499, _, _, _, _, _, _, _, _, _, _, _) => {
            h499
        }
    };
    let a501 = v0;
    if nat_le(a500, a501) {
        Ok(Adt3::C1)
    } else {
        let a502 = v0;
        let a503 = v6;
        if nat_lt(a502, a503) {
            let v7: u64 = {
                let a504 = v0;
                let a505 = 2u64;
                let a506 = 0u64;
                nat_quot(a504, a505, a506)
            };
            let v8: u64 = {
                let a511 = 3u64;
                let a512 = {
                    let a508 = f51(v7, v5.clone())?;
                    let a509 = f34(match v5 {
                        Adt1::C0(h507, _, _, _, _, _, _, _, _, _, _) => {
                            h507
                        }
                    });
                    let a510 = 0u64;
                    nat_quot(a508, a509, a510)
                };
                nat_add(a511, a512)?
            };
            let a516 = {
                let a513 = v0;
                let a514 = 2u64;
                let a515 = 0u64;
                nat_rem(a513, a514, a515)
            };
            let a517 = 0u64;
            if nat_eq(a516, a517) {
                Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, v8, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
            } else {
                let a518 = v2;
                let a519 = 0u64;
                if nat_eq(a518, a519) {
                    Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 2u64, 1u64, 2u64, 0u64, 0u64, v8, 8u64, 8u64))
                } else {
                    Ok(Adt3::C1)
                }
            }
        } else {
            let v9: u64 = {
                let a520 = v0;
                let a521 = v6;
                nat_sub(a520, a521)
            };
            let v10: u64 = f40(match v3.clone() {
                Adt1::C0(h522, _, _, _, _, _, _, _, _, _, _) => {
                    h522
                }
            }, match v4.clone() {
                Adt1::C0(h523, _, _, _, _, _, _, _, _, _, _) => {
                    h523
                }
            }, match v5.clone() {
                Adt1::C0(h524, _, _, _, _, _, _, _, _, _, _) => {
                    h524
                }
            })?;
            let v11: u64 = {
                let a525 = v9;
                let a526 = v10;
                let a527 = 0u64;
                nat_quot(a525, a526, a527)
            };
            let v12: u64 = {
                let a528 = v9;
                let a529 = v10;
                let a530 = 0u64;
                nat_rem(a528, a529, a530)
            };
            let v13: u64 = {
                let a532 = match v3.clone() {
                    Adt1::C0(h531, _, _, _, _, _, _, _, _, _, _) => {
                        h531
                    }
                };
                let a533 = 0u64;
                if nat_eq(a532, a533) {
                    0u64
                } else {
                    1u64
                }
            };
            let v14: u64 = {
                let a535 = match v4.clone() {
                    Adt1::C0(h534, _, _, _, _, _, _, _, _, _, _) => {
                        h534
                    }
                };
                let a536 = 0u64;
                if nat_eq(a535, a536) {
                    0u64
                } else {
                    1u64
                }
            };
            let a537 = v12;
            let a538 = v13;
            if nat_lt(a537, a538) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h539, _, _, _, _, _, _, _, _, _, _) => {
                        h539
                    }
                }, 0u64, 0u64, f51(v11, v3.clone())?, 2u64, 0u64, 0u64, 0u64, f34(match v3 {
                    Adt1::C0(h540, _, _, _, _, _, _, _, _, _, _) => {
                        h540
                    }
                }), 4u64))
            } else {
                let a543 = v12;
                let a544 = {
                    let a541 = v13;
                    let a542 = v14;
                    nat_add(a541, a542)?
                };
                if nat_lt(a543, a544) {
                    Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                        Adt1::C0(h545, _, _, _, _, _, _, _, _, _, _) => {
                            h545
                        }
                    }, 0u64, 4u64, f52(v11, v3, v4.clone())?, 2u64, 0u64, 0u64, 1u64, f34(match v4 {
                        Adt1::C0(h546, _, _, _, _, _, _, _, _, _, _) => {
                            h546
                        }
                    }), 4u64))
                } else {
                    let a549 = v12;
                    let a550 = {
                        let a547 = v13;
                        let a548 = v14;
                        nat_add(a547, a548)?
                    };
                    if nat_eq(a549, a550) {
                        Ok(Adt3::C3(1u64, match v1 {
                            Adt2::C0(_, _, _, h551, _, _, _, _, _, _, _, _) => {
                                h551
                            }
                        }, 0u64, 0u64, {
                            let a552 = v13;
                            let a553 = 0u64;
                            if nat_eq(a552, a553) {
                                0u64
                            } else {
                                1u64
                            }
                        }, {
                            let a554 = v13;
                            let a555 = 0u64;
                            if nat_eq(a554, a555) {
                                f51(v11, v3.clone())?
                            } else {
                                0u64
                            }
                        }, {
                            let a556 = v14;
                            let a557 = 0u64;
                            if nat_eq(a556, a557) {
                                4u64
                            } else {
                                1u64
                            }
                        }, {
                            let a558 = v14;
                            let a559 = 0u64;
                            if nat_eq(a558, a559) {
                                f52(v11, v3, v4)?
                            } else {
                                1u64
                            }
                        }, {
                            let a561 = match v5.clone() {
                                Adt1::C0(h560, _, _, _, _, _, _, _, _, _, _) => {
                                    h560
                                }
                            };
                            let a562 = 0u64;
                            if nat_eq(a561, a562) {
                                3u64
                            } else {
                                0u64
                            }
                        }, {
                            let a564 = match v5.clone() {
                                Adt1::C0(h563, _, _, _, _, _, _, _, _, _, _) => {
                                    h563
                                }
                            };
                            let a565 = 0u64;
                            if nat_eq(a564, a565) {
                                f51(v11, v5)?
                            } else {
                                0u64
                            }
                        }, 4u64, 4u64))
                    } else {
                        Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                            Adt1::C0(h566, _, _, _, _, _, _, _, _, _, _) => {
                                h566
                            }
                        }, 1u64, 0u64, 2u64, 0u64, 3u64, f51(v11, v5.clone())?, 4u64, f34(match v5 {
                            Adt1::C0(h567, _, _, _, _, _, _, _, _, _, _) => {
                                h567
                            }
                        })))
                    }
                }
            }
        }
    }
}

pub fn f18(v0: u64) -> bool {
    let a568 = 6u64;
    let a569 = v0;
    if if if if nat_le(a568, a569) {
        let a570 = v0;
        let a571 = 23u64;
        nat_le(a570, a571)
    } else {
        false
    } {
        true
    } else {
        let a572 = v0;
        let a573 = 2u64;
        if nat_eq(a572, a573) {
            true
        } else {
            let a574 = v0;
            let a575 = 3u64;
            nat_eq(a574, a575)
        }
    } {
        true
    } else {
        let a576 = v0;
        let a577 = 29u64;
        nat_eq(a576, a577)
    } {
        true
    } else {
        let a578 = 34u64;
        let a579 = v0;
        if if nat_le(a578, a579) {
            let a580 = v0;
            let a581 = 35u64;
            nat_le(a580, a581)
        } else {
            false
        } {
            true
        } else {
            let a582 = 39u64;
            let a583 = v0;
            if nat_le(a582, a583) {
                let a584 = v0;
                let a585 = 42u64;
                nat_le(a584, a585)
            } else {
                false
            }
        }
    }
}

pub fn f19(v0: u64, v1: u64, v2: u64) -> bool {
    let a586 = v0;
    let a587 = 0u64;
    if if if nat_eq(a586, a587) {
        let a588 = v1;
        let a589 = 0u64;
        nat_eq(a588, a589)
    } else {
        false
    } {
        let a590 = v2;
        let a591 = 0u64;
        nat_eq(a590, a591)
    } else {
        false
    } {
        true
    } else {
        let a592 = v0;
        let a593 = 1u64;
        if if nat_eq(a592, a593) {
            true
        } else {
            let a594 = v0;
            let a595 = 30u64;
            nat_eq(a594, a595)
        } {
            let a596 = v1;
            let a597 = v0;
            if if nat_eq(a596, a597) {
                let a598 = v2;
                let a599 = v0;
                nat_eq(a598, a599)
            } else {
                false
            } {
                true
            } else {
                let a600 = v1;
                let a601 = 0u64;
                if nat_eq(a600, a601) {
                    let a602 = v2;
                    let a603 = v0;
                    if nat_eq(a602, a603) {
                        true
                    } else {
                        let a604 = v2;
                        let a605 = 0u64;
                        nat_eq(a604, a605)
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

pub fn f20(v0: u64) -> bool {
    let a606 = 0u64;
    let a607 = f34(v0);
    if nat_lt(a606, a607) {
        true
    } else {
        let a608 = 24u64;
        let a609 = v0;
        if nat_le(a608, a609) {
            let a610 = v0;
            let a611 = 26u64;
            nat_le(a610, a611)
        } else {
            false
        }
    }
}

pub fn f21(v0: u64) -> u64 {
    let a612 = v0;
    let a613 = 0u64;
    if nat_eq(a612, a613) {
        4u64
    } else {
        let a614 = v0;
        let a615 = 1u64;
        if if nat_eq(a614, a615) {
            true
        } else {
            let a616 = v0;
            let a617 = 30u64;
            nat_eq(a616, a617)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f22(v0: u64) -> bool {
    let a618 = 0u64;
    let a619 = f34(v0);
    if nat_lt(a618, a619) {
        true
    } else {
        let a620 = 24u64;
        let a621 = v0;
        if nat_le(a620, a621) {
            let a622 = v0;
            let a623 = 28u64;
            nat_le(a622, a623)
        } else {
            false
        }
    }
}

pub fn f23(v0: Adt1, v1: Adt1) -> bool {
    if f31(v1.clone()) {
        f31(v0)
    } else {
        let a629 = {
            let a626 = match v0.clone() {
                Adt1::C0(_, h624, _, _, _, _, _, _, _, _, _) => {
                    h624
                }
            };
            let a627 = match v1.clone() {
                Adt1::C0(_, h625, _, _, _, _, _, _, _, _, _) => {
                    h625
                }
            };
            let a628 = 1u64;
            nat_rem(a626, a627, a628)
        };
        let a630 = 0u64;
        if if if nat_eq(a629, a630) {
            let a636 = {
                let a633 = match v0.clone() {
                    Adt1::C0(_, _, h631, _, _, _, _, _, _, _, _) => {
                        h631
                    }
                };
                let a634 = match v1.clone() {
                    Adt1::C0(_, _, h632, _, _, _, _, _, _, _, _) => {
                        h632
                    }
                };
                let a635 = 1u64;
                nat_rem(a633, a634, a635)
            };
            let a637 = 0u64;
            nat_eq(a636, a637)
        } else {
            false
        } {
            let a643 = {
                let a640 = match v0.clone() {
                    Adt1::C0(_, _, _, h638, _, _, _, _, _, _, _) => {
                        h638
                    }
                };
                let a641 = match v1.clone() {
                    Adt1::C0(_, _, _, h639, _, _, _, _, _, _, _) => {
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
                let a647 = match v0 {
                    Adt1::C0(_, _, _, _, h645, _, _, _, _, _, _) => {
                        h645
                    }
                };
                let a648 = match v1 {
                    Adt1::C0(_, _, _, _, h646, _, _, _, _, _, _) => {
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
        }
    }
}

pub fn f24(v0: Adt1, v1: Adt1) -> bool {
    let a654 = match v0.clone() {
        Adt1::C0(_, h652, _, _, _, _, _, _, _, _, _) => {
            h652
        }
    };
    let a655 = match v1.clone() {
        Adt1::C0(_, h653, _, _, _, _, _, _, _, _, _) => {
            h653
        }
    };
    if if if nat_eq(a654, a655) {
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
        nat_eq(a658, a659)
    } else {
        false
    } {
        let a662 = match v0.clone() {
            Adt1::C0(_, _, _, h660, _, _, _, _, _, _, _) => {
                h660
            }
        };
        let a663 = match v1.clone() {
            Adt1::C0(_, _, _, h661, _, _, _, _, _, _, _) => {
                h661
            }
        };
        nat_eq(a662, a663)
    } else {
        false
    } {
        let a666 = match v0 {
            Adt1::C0(_, _, _, _, h664, _, _, _, _, _, _) => {
                h664
            }
        };
        let a667 = match v1 {
            Adt1::C0(_, _, _, _, h665, _, _, _, _, _, _) => {
                h665
            }
        };
        nat_eq(a666, a667)
    } else {
        false
    }
}

pub fn f25(v0: u64, v1: u64, v2: Adt1, v3: u64) -> bool {
    let v4: u64 = match v2.clone() {
        Adt1::C0(h668, _, _, _, _, _, _, _, _, _, _) => {
            h668
        }
    };
    let a669 = v1;
    let a670 = v3;
    if if nat_eq(a669, a670) {
        true
    } else {
        let a671 = v0;
        let a672 = 0u64;
        if nat_eq(a671, a672) {
            let a673 = v4;
            let a674 = 0u64;
            if if nat_eq(a673, a674) {
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
            let a676 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, h675, _, _, _, _, _) => {
                    h675
                }
            };
            let a677 = f27(v4);
            nat_eq(a676, a677)
        } else {
            false
        } {
            let a682 = {
                let a679 = match v2 {
                    Adt1::C0(_, h678, _, _, _, _, _, _, _, _, _) => {
                        h678
                    }
                };
                let a680 = f28(v4);
                let a681 = 1u64;
                nat_rem(a679, a680, a681)
            };
            let a683 = 0u64;
            nat_eq(a682, a683)
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
        let a684 = v0;
        let a685 = v1;
        if if nat_eq(a684, a685) {
            f20(v0)
        } else {
            false
        } {
            true
        } else {
            let a686 = f21(v0);
            let a687 = 2u64;
            if nat_eq(a686, a687) {
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
    let a688 = 24u64;
    let a689 = v0;
    if if nat_le(a688, a689) {
        let a690 = v0;
        let a691 = 28u64;
        nat_le(a690, a691)
    } else {
        false
    } {
        let m692 = f60(v0);
        match m692 {
            Adt4::C0 => {
                0u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, _, h693) => {
                        h693
                    }
                }
            }
        }
    } else {
        f42(v0)
    }
}

pub fn f28(v0: u64) -> u64 {
    if f18(v0) {
        let m694 = f60(v0);
        match m694 {
            Adt4::C0 => {
                1u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, h695, _) => {
                        h695
                    }
                }
            }
        }
    } else {
        1u64
    }
}

pub fn f29(v0: Adt1, v1: Adt1) -> R<bool> {
    if f48(v0.clone(), v1.clone())? {
        Ok(true)
    } else {
        Ok(f61(v0, v1))
    }
}

pub fn f30(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f62(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f62(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f62(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f62(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f62(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f62(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f31(v0: Adt1) -> bool {
    f63(match v0.clone() {
        Adt1::C0(_, h696, _, _, _, _, _, _, _, _, _) => {
            h696
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h697, _, _, _, _, _, _, _, _) => {
            h697
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h698, _, _, _, _, _, _, _) => {
            h698
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, h699, _, _, _, _, _, _) => {
            h699
        }
    })
}

pub fn f32(v0: u64) -> Option<u64> {
    let a700 = v0;
    let a701 = 16u64;
    if if if nat_eq(a700, a701) {
        true
    } else {
        let a702 = v0;
        let a703 = 17u64;
        nat_eq(a702, a703)
    } {
        true
    } else {
        let a704 = v0;
        let a705 = 19u64;
        nat_eq(a704, a705)
    } {
        Some(1065353216u64)
    } else {
        None::<u64>
    }
}

pub fn f33(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<bool> {
    let a707 = match v1.clone() {
        Adt1::C0(h706, _, _, _, _, _, _, _, _, _, _) => {
            h706
        }
    };
    let a708 = 43u64;
    if nat_eq(a707, a708) {
        let a709 = v0;
        let a710 = 0u64;
        if if if if if if nat_eq(a709, a710) {
            let a712 = match v2.clone() {
                Adt1::C0(h711, _, _, _, _, _, _, _, _, _, _) => {
                    h711
                }
            };
            let a713 = 0u64;
            nat_eq(a712, a713)
        } else {
            false
        } {
            let a715 = match v3.clone() {
                Adt1::C0(h714, _, _, _, _, _, _, _, _, _, _) => {
                    h714
                }
            };
            let a716 = 0u64;
            if nat_eq(a715, a716) {
                true
            } else {
                let a718 = match v3.clone() {
                    Adt1::C0(h717, _, _, _, _, _, _, _, _, _, _) => {
                        h717
                    }
                };
                let a719 = 43u64;
                nat_eq(a718, a719)
            }
        } else {
            false
        } {
            let a721 = match v3.clone() {
                Adt1::C0(h720, _, _, _, _, _, _, _, _, _, _) => {
                    h720
                }
            };
            let a722 = 43u64;
            if nat_eq(a721, a722) {
                f23(v1.clone(), v2.clone())
            } else {
                f24(v1.clone(), v2.clone())
            }
        } else {
            false
        } {
            let a724 = match v2 {
                Adt1::C0(_, _, _, _, _, h723, _, _, _, _, _) => {
                    h723
                }
            };
            let a725 = 4u64;
            nat_eq(a724, a725)
        } else {
            false
        } {
            let a728 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h726, _, _, _, _, _) => {
                    h726
                }
            };
            let a729 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h727, _, _, _, _) => {
                    h727
                }
            };
            if nat_le(a728, a729) {
                let a732 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, h730, _, _, _, _) => {
                        h730
                    }
                };
                let a733 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h731, _, _, _) => {
                        h731
                    }
                };
                if nat_le(a732, a733) {
                    let a736 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, h734, _, _, _) => {
                            h734
                        }
                    };
                    let a737 = match v3 {
                        Adt1::C0(_, _, _, _, _, _, _, _, h735, _, _) => {
                            h735
                        }
                    };
                    nat_le(a736, a737)
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        } {
            let a745 = {
                let a743 = match v1.clone() {
                    Adt1::C0(_, _, h738, _, _, _, _, _, _, _, _) => {
                        h738
                    }
                };
                let a744 = {
                    let a741 = match v1.clone() {
                        Adt1::C0(_, _, _, h739, _, _, _, _, _, _, _) => {
                            h739
                        }
                    };
                    let a742 = match v1 {
                        Adt1::C0(_, _, _, _, h740, _, _, _, _, _, _) => {
                            h740
                        }
                    };
                    nat_mul(a741, a742)?
                };
                nat_mul(a743, a744)?
            };
            let a746 = 2147483647u64;
            Ok(nat_le(a745, a746))
        } else {
            Ok(false)
        }
    } else {
        if f23(v1.clone(), v2.clone()) {
            Ok(f19(match v1 {
                Adt1::C0(h747, _, _, _, _, _, _, _, _, _, _) => {
                    h747
                }
            }, match v2 {
                Adt1::C0(h748, _, _, _, _, _, _, _, _, _, _) => {
                    h748
                }
            }, match v3 {
                Adt1::C0(h749, _, _, _, _, _, _, _, _, _, _) => {
                    h749
                }
            }))
        } else {
            Ok(false)
        }
    }
}

pub fn f34(v0: u64) -> u64 {
    let a750 = v0;
    let a751 = 43u64;
    if nat_eq(a750, a751) {
        1u64
    } else {
        f21(v0)
    }
}

pub fn f35(v0: Adt1) -> bool {
    let v1: u64 = f34(match v0.clone() {
        Adt1::C0(h752, _, _, _, _, _, _, _, _, _, _) => {
            h752
        }
    });
    let a757 = {
        let a754 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h753) => {
                h753
            }
        };
        let a755 = v1;
        let a756 = 1u64;
        nat_rem(a754, a755, a756)
    };
    let a758 = 0u64;
    if if nat_eq(a757, a758) {
        f64(match v0.clone() {
            Adt1::C0(_, h759, _, _, _, _, _, _, _, _, _) => {
                h759
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h760, _, _, _, _, _) => {
                h760
            }
        }, v1)
    } else {
        false
    } {
        f65(match v0.clone() {
            Adt1::C0(_, _, h761, _, _, _, _, _, _, _, _) => {
                h761
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h762, _, _, _, _, _, _, _) => {
                h762
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h763, _, _, _, _, _, _) => {
                h763
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h764, _, _, _, _) => {
                h764
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h765, _, _, _) => {
                h765
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h766, _, _) => {
                h766
            }
        }, v1)
    } else {
        false
    }
}

pub fn f36(v0: Adt1, v1: Adt1) -> R<bool> {
    let a774 = {
        let a772 = match v0.clone() {
            Adt1::C0(_, _, h767, _, _, _, _, _, _, _, _) => {
                h767
            }
        };
        let a773 = {
            let a770 = match v0.clone() {
                Adt1::C0(_, _, _, h768, _, _, _, _, _, _, _) => {
                    h768
                }
            };
            let a771 = match v0.clone() {
                Adt1::C0(_, _, _, _, h769, _, _, _, _, _, _) => {
                    h769
                }
            };
            nat_mul(a770, a771)?
        };
        nat_mul(a772, a773)?
    };
    let a775 = 9223372036854775807u64;
    if nat_le(a774, a775) {
        if f44(v1.clone()) {
            let a777 = match v1 {
                Adt1::C0(_, h776, _, _, _, _, _, _, _, _, _) => {
                    h776
                }
            };
            let a778 = 2147483647u64;
            Ok(nat_le(a777, a778))
        } else {
            let a780 = match v0 {
                Adt1::C0(_, h779, _, _, _, _, _, _, _, _, _) => {
                    h779
                }
            };
            let a781 = 2147483647u64;
            Ok(nat_le(a780, a781))
        }
    } else {
        Ok(false)
    }
}

pub fn f37(v0: Adt1) -> R<bool> {
    f30({
        let a784 = match v0.clone() {
            Adt1::C0(_, h782, _, _, _, _, _, _, _, _, _) => {
                h782
            }
        };
        let a785 = f34(match v0.clone() {
            Adt1::C0(h783, _, _, _, _, _, _, _, _, _, _) => {
                h783
            }
        });
        nat_mul(a784, a785)?
    }, match v0.clone() {
        Adt1::C0(_, _, h786, _, _, _, _, _, _, _, _) => {
            h786
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h787, _, _, _, _, _, _, _) => {
            h787
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h788, _, _, _, _, _, _) => {
            h788
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h789, _, _, _, _) => {
            h789
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h790, _, _, _) => {
            h790
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h791, _, _) => {
            h791
        }
    })
}

pub fn f38(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a798 = {
        let a792 = v0;
        let a793 = v2;
        nat_mul(a792, a793)?
    };
    let a799 = {
        let a794 = v1;
        let a795 = 0u64;
        if nat_eq(a794, a795) {
            0u64
        } else {
            let a796 = v0;
            let a797 = 2u64;
            nat_mul(a796, a797)?
        }
    };
    nat_add(a798, a799)
}

pub fn f39(v0: Adt1) -> R<u64> {
    let a808 = {
        let a802 = match v0.clone() {
            Adt1::C0(_, h800, _, _, _, _, _, _, _, _, _) => {
                h800
            }
        };
        let a803 = match v0.clone() {
            Adt1::C0(_, _, h801, _, _, _, _, _, _, _, _) => {
                h801
            }
        };
        nat_mul(a802, a803)?
    };
    let a809 = {
        let a806 = match v0.clone() {
            Adt1::C0(_, _, _, h804, _, _, _, _, _, _, _) => {
                h804
            }
        };
        let a807 = match v0 {
            Adt1::C0(_, _, _, _, h805, _, _, _, _, _, _) => {
                h805
            }
        };
        nat_mul(a806, a807)?
    };
    nat_mul(a808, a809)
}

pub fn f40(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a820 = {
        let a816 = {
            let a812 = 1u64;
            let a813 = {
                let a810 = v0;
                let a811 = 0u64;
                if nat_eq(a810, a811) {
                    0u64
                } else {
                    1u64
                }
            };
            nat_add(a812, a813)?
        };
        let a817 = {
            let a814 = v1;
            let a815 = 0u64;
            if nat_eq(a814, a815) {
                0u64
            } else {
                1u64
            }
        };
        nat_add(a816, a817)?
    };
    let a821 = {
        let a818 = v2;
        let a819 = 0u64;
        if nat_eq(a818, a819) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a820, a821)
}

pub fn f41(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f24(v0.clone(), v1.clone()) {
        let a824 = match v0.clone() {
            Adt1::C0(h822, _, _, _, _, _, _, _, _, _, _) => {
                h822
            }
        };
        let a825 = match v1.clone() {
            Adt1::C0(h823, _, _, _, _, _, _, _, _, _, _) => {
                h823
            }
        };
        nat_eq(a824, a825)
    } else {
        false
    } {
        let a828 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h826, _, _, _, _, _) => {
                h826
            }
        };
        let a829 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h827, _, _, _, _, _) => {
                h827
            }
        };
        nat_eq(a828, a829)
    } else {
        false
    } {
        let a832 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h830, _, _, _, _) => {
                h830
            }
        };
        let a833 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h831, _, _, _, _) => {
                h831
            }
        };
        nat_eq(a832, a833)
    } else {
        false
    } {
        let a836 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h834, _, _, _) => {
                h834
            }
        };
        let a837 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h835, _, _, _) => {
                h835
            }
        };
        nat_eq(a836, a837)
    } else {
        false
    } {
        let a840 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h838, _, _) => {
                h838
            }
        };
        let a841 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h839, _, _) => {
                h839
            }
        };
        nat_eq(a840, a841)
    } else {
        false
    } {
        let a843 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h842) => {
                h842
            }
        };
        let a844 = 0u64;
        if if nat_eq(a843, a844) {
            true
        } else {
            let a846 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h845) => {
                    h845
                }
            };
            let a847 = 0u64;
            nat_eq(a846, a847)
        } {
            true
        } else {
            let a850 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h848) => {
                    h848
                }
            };
            let a851 = match v1 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h849) => {
                    h849
                }
            };
            nat_eq(a850, a851)
        }
    } else {
        false
    }
}

pub fn f42(v0: u64) -> u64 {
    if f18(v0) {
        let m852 = f60(v0);
        match m852 {
            Adt4::C0 => {
                0u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, _, h853) => {
                        h853
                    }
                }
            }
        }
    } else {
        let a854 = v0;
        let a855 = 26u64;
        if nat_eq(a854, a855) {
            4u64
        } else {
            f34(v0)
        }
    }
}

pub fn f43(v0: u64) -> u64 {
    let a856 = v0;
    let a857 = 26u64;
    if nat_eq(a856, a857) {
        4u64
    } else {
        let a858 = v0;
        let a859 = 27u64;
        if nat_eq(a858, a859) {
            8u64
        } else {
            0u64
        }
    }
}

pub fn f44(v0: Adt1) -> bool {
    let a861 = match v0.clone() {
        Adt1::C0(_, h860, _, _, _, _, _, _, _, _, _) => {
            h860
        }
    };
    let a862 = 1u64;
    if nat_eq(a861, a862) {
        true
    } else {
        let a865 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h863, _, _, _, _, _) => {
                h863
            }
        };
        let a866 = f34(match v0 {
            Adt1::C0(h864, _, _, _, _, _, _, _, _, _, _) => {
                h864
            }
        });
        nat_eq(a865, a866)
    }
}

pub fn f45(v0: Adt1, v1: u64) -> R<u64> {
    let a872 = {
        let a868 = match v0.clone() {
            Adt1::C0(_, h867, _, _, _, _, _, _, _, _, _) => {
                h867
            }
        };
        let a869 = f28(v1);
        let a870 = 0u64;
        nat_quot(a868, a869, a870)
    };
    let a873 = f66(match v0 {
        Adt1::C0(h871, _, _, _, _, _, _, _, _, _, _) => {
            h871
        }
    }, v1);
    nat_mul(a872, a873)
}

pub fn f46(v0: u64) -> u64 {
    if if f18(v0) {
        true
    } else {
        let a874 = v0;
        let a875 = 26u64;
        nat_eq(a874, a875)
    } {
        f28(v0)
    } else {
        0u64
    }
}

pub fn f47(v0: Adt1) -> R<bool> {
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

pub fn f48(v0: Adt1, v1: Adt1) -> R<bool> {
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

pub fn f49(v0: Adt1) -> R<bool> {
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
    if if f44(v0.clone()) {
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

pub fn f50(v0: u64) -> u64 {
    let a970 = v0;
    let a971 = 3u64;
    if nat_eq(a970, a971) {
        16u64
    } else {
        10u64
    }
}

pub fn f51(v0: u64, v1: Adt1) -> R<u64> {
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
    f53({
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

pub fn f52(v0: u64, v1: Adt1, v2: Adt1) -> R<u64> {
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
    f53({
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

pub fn f53(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
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

pub fn f54(v0: u64, v1: Adt1, v2: u64, v3: u64) -> R<Adt3> {
    let v4: u64 = {
        let a1058 = v0;
        let a1059 = 2u64;
        let a1060 = 0u64;
        nat_quot(a1058, a1059, a1060)
    };
    let v5: u64 = f43(match v1.clone() {
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
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 4u64, f51(v4, v1)?, 2u64, 0u64, 0u64, 2u64, v5, 8u64))
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

pub fn f55(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64) -> R<Adt3> {
    let v5: u64 = {
        let a1076 = 1u64;
        let a1077 = f45(v1.clone(), match v3.clone() {
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
                let a1130 = f66(match v1.clone() {
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
            let v14: u64 = f53({
                let a1133 = v13;
                let a1134 = f28(match v3.clone() {
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
            let v15: u64 = f53(v13, v4, v10, v11, match v3.clone() {
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
            let a1144 = 0u64;
            let a1145 = f46(match v3.clone() {
                Adt1::C0(h1143, _, _, _, _, _, _, _, _, _, _) => {
                    h1143
                }
            });
            if nat_lt(a1144, a1145) {
                Ok(Adt3::C6(v14, v15))
            } else {
                Ok(f67(match v1 {
                    Adt1::C0(h1146, _, _, _, _, _, _, _, _, _, _) => {
                        h1146
                    }
                }, match v3 {
                    Adt1::C0(h1147, _, _, _, _, _, _, _, _, _, _) => {
                        h1147
                    }
                }, {
                    let a1148 = v6;
                    let a1149 = 1u64;
                    nat_sub(a1148, a1149)
                }, v14, v15))
            }
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f56(v0: u64, v1: u64, v2: u64) -> Adt3 {
    Adt3::C3(2u64, 0u64, 0u64, 0u64, 1u64, v0, v1, v2, 0u64, 2u64, 4u64, 4u64)
}

pub fn f57(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt3 {
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

pub fn f58(v0: u64, v1: u64) -> Adt3 {
    let a1150 = v0;
    let a1151 = 8u64;
    if nat_eq(a1150, a1151) {
        Adt3::C3(1u64, 6u64, 0u64, 0u64, 1u64, 0u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
    } else {
        let a1152 = v0;
        let a1153 = 9u64;
        if nat_eq(a1152, a1153) {
            Adt3::C3(1u64, 2u64, 0u64, 0u64, 2u64, v1, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
        } else {
            let a1154 = v0;
            let a1155 = 10u64;
            if nat_eq(a1154, a1155) {
                Adt3::C3(5u64, 16u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
            } else {
                let a1156 = v0;
                let a1157 = 11u64;
                if nat_eq(a1156, a1157) {
                    Adt3::C3(1u64, 0u64, 0u64, 0u64, 2u64, 1065353216u64, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
                } else {
                    let a1158 = v0;
                    let a1159 = 12u64;
                    if nat_eq(a1158, a1159) {
                        Adt3::C3(1u64, 3u64, 0u64, 0u64, 1u64, 0u64, 1u64, 2u64, 0u64, 0u64, 4u64, 4u64)
                    } else {
                        let a1160 = v0;
                        let a1161 = 13u64;
                        if nat_eq(a1160, a1161) {
                            Adt3::C3(1u64, 0u64, 0u64, 0u64, 1u64, 1u64, 2u64, 1065353216u64, 0u64, 1u64, 4u64, 4u64)
                        } else {
                            let a1162 = v0;
                            let a1163 = 14u64;
                            if nat_eq(a1162, a1163) {
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

pub fn f59(v0: u64, v1: u64, v2: Adt2) -> R<Adt3> {
    let v3: Adt1 = match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1164, _, _, _, _) => {
            h1164
        }
    };
    let v4: Adt1 = match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h1165, _, _, _) => {
            h1165
        }
    };
    let v5: Adt1 = match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1166, _, _) => {
            h1166
        }
    };
    let v6: u64 = match v2.clone() {
        Adt2::C0(_, _, _, h1167, _, _, _, _, _, _, _, _) => {
            h1167
        }
    };
    let v7: u64 = {
        let a1169 = v1;
        let a1170 = match v5.clone() {
            Adt1::C0(_, h1168, _, _, _, _, _, _, _, _, _) => {
                h1168
            }
        };
        let a1171 = 0u64;
        nat_quot(a1169, a1170, a1171)
    };
    let v8: u64 = f21(match v5.clone() {
        Adt1::C0(h1172, _, _, _, _, _, _, _, _, _, _) => {
            h1172
        }
    });
    let v9: u64 = {
        let a1177 = {
            let a1174 = v1;
            let a1175 = match v5.clone() {
                Adt1::C0(_, h1173, _, _, _, _, _, _, _, _, _) => {
                    h1173
                }
            };
            let a1176 = 0u64;
            nat_rem(a1174, a1175, a1176)
        };
        let a1178 = v8;
        nat_mul(a1177, a1178)?
    };
    let v10: u64 = {
        let a1180 = match v5.clone() {
            Adt1::C0(_, h1179, _, _, _, _, _, _, _, _, _) => {
                h1179
            }
        };
        let a1181 = v8;
        nat_mul(a1180, a1181)?
    };
    let a1182 = v0;
    let a1183 = 0u64;
    if nat_eq(a1182, a1183) {
        Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
            Adt1::C0(h1184, _, _, _, _, _, _, _, _, _, _) => {
                h1184
            }
        }, 0u64, 0u64, {
            let a1192 = {
                let a1188 = {
                    let a1186 = v7;
                    let a1187 = match v3 {
                        Adt1::C0(_, _, _, _, _, _, h1185, _, _, _, _) => {
                            h1185
                        }
                    };
                    nat_mul(a1186, a1187)?
                };
                let a1189 = v9;
                nat_add(a1188, a1189)?
            };
            let a1193 = {
                let a1190 = v6;
                let a1191 = 100u64;
                if nat_eq(a1190, a1191) {
                    v10
                } else {
                    0u64
                }
            };
            nat_add(a1192, a1193)?
        }, 2u64, 0u64, 0u64, 0u64, v8, 4u64))
    } else {
        let a1194 = v0;
        let a1195 = 1u64;
        if nat_eq(a1194, a1195) {
            Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                Adt1::C0(h1196, _, _, _, _, _, _, _, _, _, _) => {
                    h1196
                }
            }, 0u64, {
                let a1197 = v6;
                let a1198 = 98u64;
                if nat_eq(a1197, a1198) {
                    4u64
                } else {
                    0u64
                }
            }, {
                let a1206 = {
                    let a1202 = {
                        let a1200 = v7;
                        let a1201 = match v4 {
                            Adt1::C0(_, _, _, _, _, _, h1199, _, _, _, _) => {
                                h1199
                            }
                        };
                        nat_mul(a1200, a1201)?
                    };
                    let a1203 = v9;
                    nat_add(a1202, a1203)?
                };
                let a1207 = {
                    let a1204 = v6;
                    let a1205 = 99u64;
                    if nat_eq(a1204, a1205) {
                        v10
                    } else {
                        0u64
                    }
                };
                nat_add(a1206, a1207)?
            }, 2u64, 0u64, 0u64, 1u64, v8, 4u64))
        } else {
            let a1208 = v0;
            let a1209 = 2u64;
            if nat_eq(a1208, a1209) {
                Ok(Adt3::C4(f68(match v2 {
                    Adt2::C0(_, _, _, _, h1210, _, _, _, _, _, _, _) => {
                        h1210
                    }
                }, v5.clone(), v1), match v5 {
                    Adt1::C0(h1211, _, _, _, _, _, _, _, _, _, _) => {
                        h1211
                    }
                }, 0u64, 1u64, 0u64))
            } else {
                Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                    Adt1::C0(h1212, _, _, _, _, _, _, _, _, _, _) => {
                        h1212
                    }
                }, 1u64, 0u64, 2u64, 0u64, 3u64, {
                    let a1216 = {
                        let a1214 = v7;
                        let a1215 = match v5 {
                            Adt1::C0(_, _, _, _, _, _, h1213, _, _, _, _) => {
                                h1213
                            }
                        };
                        nat_mul(a1214, a1215)?
                    };
                    let a1217 = v9;
                    nat_add(a1216, a1217)?
                }, 4u64, v8))
            }
        }
    }
}

pub fn f60(v0: u64) -> Adt4 {
    let a1218 = v0;
    let a1219 = 0u64;
    if nat_eq(a1218, a1219) {
        Adt4::C1(Adt5::C0(0u64, 1u64, 4u64))
    } else {
        let a1220 = v0;
        let a1221 = 1u64;
        if nat_eq(a1220, a1221) {
            Adt4::C1(Adt5::C0(1u64, 1u64, 2u64))
        } else {
            let a1222 = v0;
            let a1223 = 2u64;
            if nat_eq(a1222, a1223) {
                Adt4::C1(Adt5::C0(2u64, 32u64, 18u64))
            } else {
                let a1224 = v0;
                let a1225 = 3u64;
                if nat_eq(a1224, a1225) {
                    Adt4::C1(Adt5::C0(3u64, 32u64, 20u64))
                } else {
                    let a1226 = v0;
                    let a1227 = 6u64;
                    if nat_eq(a1226, a1227) {
                        Adt4::C1(Adt5::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1228 = v0;
                        let a1229 = 7u64;
                        if nat_eq(a1228, a1229) {
                            Adt4::C1(Adt5::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1230 = v0;
                            let a1231 = 8u64;
                            if nat_eq(a1230, a1231) {
                                Adt4::C1(Adt5::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1232 = v0;
                                let a1233 = 9u64;
                                if nat_eq(a1232, a1233) {
                                    Adt4::C1(Adt5::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1234 = v0;
                                    let a1235 = 10u64;
                                    if nat_eq(a1234, a1235) {
                                        Adt4::C1(Adt5::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1236 = v0;
                                        let a1237 = 11u64;
                                        if nat_eq(a1236, a1237) {
                                            Adt4::C1(Adt5::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1238 = v0;
                                            let a1239 = 12u64;
                                            if nat_eq(a1238, a1239) {
                                                Adt4::C1(Adt5::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1240 = v0;
                                                let a1241 = 13u64;
                                                if nat_eq(a1240, a1241) {
                                                    Adt4::C1(Adt5::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1242 = v0;
                                                    let a1243 = 14u64;
                                                    if nat_eq(a1242, a1243) {
                                                        Adt4::C1(Adt5::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1244 = v0;
                                                        let a1245 = 15u64;
                                                        if nat_eq(a1244, a1245) {
                                                            Adt4::C1(Adt5::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1246 = v0;
                                                            let a1247 = 16u64;
                                                            if nat_eq(a1246, a1247) {
                                                                Adt4::C1(Adt5::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1248 = v0;
                                                                let a1249 = 17u64;
                                                                if nat_eq(a1248, a1249) {
                                                                    Adt4::C1(Adt5::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1250 = v0;
                                                                    let a1251 = 18u64;
                                                                    if nat_eq(a1250, a1251) {
                                                                        Adt4::C1(Adt5::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1252 = v0;
                                                                        let a1253 = 19u64;
                                                                        if nat_eq(a1252, a1253) {
                                                                            Adt4::C1(Adt5::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1254 = v0;
                                                                            let a1255 = 20u64;
                                                                            if nat_eq(a1254, a1255) {
                                                                                Adt4::C1(Adt5::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1256 = v0;
                                                                                let a1257 = 21u64;
                                                                                if nat_eq(a1256, a1257) {
                                                                                    Adt4::C1(Adt5::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1258 = v0;
                                                                                    let a1259 = 22u64;
                                                                                    if nat_eq(a1258, a1259) {
                                                                                        Adt4::C1(Adt5::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1260 = v0;
                                                                                        let a1261 = 23u64;
                                                                                        if nat_eq(a1260, a1261) {
                                                                                            Adt4::C1(Adt5::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1262 = v0;
                                                                                            let a1263 = 24u64;
                                                                                            if nat_eq(a1262, a1263) {
                                                                                                Adt4::C1(Adt5::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1264 = v0;
                                                                                                let a1265 = 25u64;
                                                                                                if nat_eq(a1264, a1265) {
                                                                                                    Adt4::C1(Adt5::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1266 = v0;
                                                                                                    let a1267 = 26u64;
                                                                                                    if nat_eq(a1266, a1267) {
                                                                                                        Adt4::C1(Adt5::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1268 = v0;
                                                                                                        let a1269 = 27u64;
                                                                                                        if nat_eq(a1268, a1269) {
                                                                                                            Adt4::C1(Adt5::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1270 = v0;
                                                                                                            let a1271 = 28u64;
                                                                                                            if nat_eq(a1270, a1271) {
                                                                                                                Adt4::C1(Adt5::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1272 = v0;
                                                                                                                let a1273 = 29u64;
                                                                                                                if nat_eq(a1272, a1273) {
                                                                                                                    Adt4::C1(Adt5::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1274 = v0;
                                                                                                                    let a1275 = 30u64;
                                                                                                                    if nat_eq(a1274, a1275) {
                                                                                                                        Adt4::C1(Adt5::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1276 = v0;
                                                                                                                        let a1277 = 34u64;
                                                                                                                        if nat_eq(a1276, a1277) {
                                                                                                                            Adt4::C1(Adt5::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1278 = v0;
                                                                                                                            let a1279 = 35u64;
                                                                                                                            if nat_eq(a1278, a1279) {
                                                                                                                                Adt4::C1(Adt5::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1280 = v0;
                                                                                                                                let a1281 = 39u64;
                                                                                                                                if nat_eq(a1280, a1281) {
                                                                                                                                    Adt4::C1(Adt5::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1282 = v0;
                                                                                                                                    let a1283 = 40u64;
                                                                                                                                    if nat_eq(a1282, a1283) {
                                                                                                                                        Adt4::C1(Adt5::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1284 = v0;
                                                                                                                                        let a1285 = 41u64;
                                                                                                                                        if nat_eq(a1284, a1285) {
                                                                                                                                            Adt4::C1(Adt5::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1286 = v0;
                                                                                                                                            let a1287 = 42u64;
                                                                                                                                            if nat_eq(a1286, a1287) {
                                                                                                                                                Adt4::C1(Adt5::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1288 = v0;
                                                                                                                                                let a1289 = 43u64;
                                                                                                                                                if nat_eq(a1288, a1289) {
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

pub fn f61(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f24(v0.clone(), v1.clone()) {
        let a1292 = match v0.clone() {
            Adt1::C0(h1290, _, _, _, _, _, _, _, _, _, _) => {
                h1290
            }
        };
        let a1293 = match v1.clone() {
            Adt1::C0(h1291, _, _, _, _, _, _, _, _, _, _) => {
                h1291
            }
        };
        nat_eq(a1292, a1293)
    } else {
        false
    } {
        let a1296 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1294) => {
                h1294
            }
        };
        let a1297 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1295) => {
                h1295
            }
        };
        nat_eq(a1296, a1297)
    } else {
        false
    } {
        let a1300 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1298, _, _, _, _, _) => {
                h1298
            }
        };
        let a1301 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1299, _, _, _, _, _) => {
                h1299
            }
        };
        nat_eq(a1300, a1301)
    } else {
        false
    } {
        let a1304 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1302, _, _, _, _) => {
                h1302
            }
        };
        let a1305 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1303, _, _, _, _) => {
                h1303
            }
        };
        nat_eq(a1304, a1305)
    } else {
        false
    } {
        let a1308 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1306, _, _, _) => {
                h1306
            }
        };
        let a1309 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1307, _, _, _) => {
                h1307
            }
        };
        nat_eq(a1308, a1309)
    } else {
        false
    } {
        let a1312 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1310, _, _) => {
                h1310
            }
        };
        let a1313 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1311, _, _) => {
                h1311
            }
        };
        nat_eq(a1312, a1313)
    } else {
        false
    }
}

pub fn f62(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a1314 = v1;
    let a1315 = 1u64;
    if if if nat_le(a1314, a1315) {
        true
    } else {
        let a1316 = v0;
        let a1317 = v2;
        nat_le(a1316, a1317)
    } {
        let a1318 = v3;
        let a1319 = 1u64;
        if nat_le(a1318, a1319) {
            true
        } else {
            let a1326 = {
                let a1324 = v0;
                let a1325 = {
                    let a1322 = {
                        let a1320 = v1;
                        let a1321 = 1u64;
                        nat_sub(a1320, a1321)
                    };
                    let a1323 = v2;
                    nat_mul(a1322, a1323)?
                };
                nat_add(a1324, a1325)?
            };
            let a1327 = v4;
            nat_le(a1326, a1327)
        }
    } else {
        false
    } {
        let a1328 = v5;
        let a1329 = 1u64;
        if nat_le(a1328, a1329) {
            Ok(true)
        } else {
            let a1342 = {
                let a1340 = {
                    let a1334 = v0;
                    let a1335 = {
                        let a1332 = {
                            let a1330 = v1;
                            let a1331 = 1u64;
                            nat_sub(a1330, a1331)
                        };
                        let a1333 = v2;
                        nat_mul(a1332, a1333)?
                    };
                    nat_add(a1334, a1335)?
                };
                let a1341 = {
                    let a1338 = {
                        let a1336 = v3;
                        let a1337 = 1u64;
                        nat_sub(a1336, a1337)
                    };
                    let a1339 = v4;
                    nat_mul(a1338, a1339)?
                };
                nat_add(a1340, a1341)?
            };
            let a1343 = v6;
            Ok(nat_le(a1342, a1343))
        }
    } else {
        Ok(false)
    }
}

pub fn f63(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1344 = v0;
    let a1345 = 0u64;
    if if if nat_eq(a1344, a1345) {
        true
    } else {
        let a1346 = v1;
        let a1347 = 0u64;
        nat_eq(a1346, a1347)
    } {
        true
    } else {
        let a1348 = v2;
        let a1349 = 0u64;
        nat_eq(a1348, a1349)
    } {
        true
    } else {
        let a1350 = v3;
        let a1351 = 0u64;
        nat_eq(a1350, a1351)
    }
}

pub fn f64(v0: u64, v1: u64, v2: u64) -> bool {
    let a1352 = v0;
    let a1353 = 1u64;
    if nat_le(a1352, a1353) {
        true
    } else {
        let a1357 = {
            let a1354 = v1;
            let a1355 = v2;
            let a1356 = 1u64;
            nat_rem(a1354, a1355, a1356)
        };
        let a1358 = 0u64;
        nat_eq(a1357, a1358)
    }
}

pub fn f65(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f64(v0, v3, v6) {
        f64(v1, v4, v6)
    } else {
        false
    } {
        f64(v2, v5, v6)
    } else {
        false
    }
}

pub fn f66(v0: u64, v1: u64) -> u64 {
    let a1359 = v0;
    let a1360 = 1u64;
    if if nat_eq(a1359, a1360) {
        let a1361 = v1;
        let a1362 = 43u64;
        nat_eq(a1361, a1362)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f67(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt3 {
    let a1363 = f66(v0, v1);
    let a1364 = 2u64;
    if nat_eq(a1363, a1364) {
        let a1368 = {
            let a1365 = v2;
            let a1366 = 2u64;
            let a1367 = 0u64;
            nat_rem(a1365, a1366, a1367)
        };
        let a1369 = 0u64;
        if nat_eq(a1368, a1369) {
            Adt3::C3(0u64, 0u64, v0, 0u64, 0u64, v3, 2u64, 0u64, 1u64, 0u64, f21(v0), 8u64)
        } else {
            Adt3::C3(0u64, 0u64, 0u64, v1, 1u64, 0u64, 2u64, 0u64, 3u64, v4, 8u64, f42(v1))
        }
    } else {
        Adt3::C3(0u64, 0u64, v0, v1, 0u64, v3, 2u64, 0u64, 3u64, v4, f21(v0), f42(v1))
    }
}

pub fn f68(v0: u64, v1: Adt1, v2: u64) -> u64 {
    let a1370 = v0;
    let a1371 = 2u64;
    if if nat_eq(a1370, a1371) {
        let a1373 = match v1.clone() {
            Adt1::C0(h1372, _, _, _, _, _, _, _, _, _, _) => {
                h1372
            }
        };
        let a1374 = 0u64;
        if nat_eq(a1373, a1374) {
            let a1386 = {
                let a1376 = v2;
                let a1377 = match v1.clone() {
                    Adt1::C0(_, h1375, _, _, _, _, _, _, _, _, _) => {
                        h1375
                    }
                };
                let a1378 = 0u64;
                nat_rem(a1376, a1377, a1378)
            };
            let a1387 = {
                let a1384 = match v1.clone() {
                    Adt1::C0(_, h1379, _, _, _, _, _, _, _, _, _) => {
                        h1379
                    }
                };
                let a1385 = {
                    let a1381 = match v1 {
                        Adt1::C0(_, h1380, _, _, _, _, _, _, _, _, _) => {
                            h1380
                        }
                    };
                    let a1382 = 4u64;
                    let a1383 = 0u64;
                    nat_rem(a1381, a1382, a1383)
                };
                nat_sub(a1384, a1385)
            };
            nat_lt(a1386, a1387)
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
