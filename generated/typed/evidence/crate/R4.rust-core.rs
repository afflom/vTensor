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
            if f60(match v3.clone() {
                Adt1::C0(h522, _, _, _, _, _, _, _, _, _, _) => {
                    h522
                }
            }) {
                f61(v9, v1)
            } else {
                let v10: u64 = f62(match v3.clone() {
                    Adt1::C0(h523, _, _, _, _, _, _, _, _, _, _) => {
                        h523
                    }
                }, match v4.clone() {
                    Adt1::C0(h524, _, _, _, _, _, _, _, _, _, _) => {
                        h524
                    }
                }, match v5.clone() {
                    Adt1::C0(h525, _, _, _, _, _, _, _, _, _, _) => {
                        h525
                    }
                })?;
                let v11: u64 = {
                    let a526 = v9;
                    let a527 = v10;
                    let a528 = 0u64;
                    nat_quot(a526, a527, a528)
                };
                let v12: u64 = {
                    let a529 = v9;
                    let a530 = v10;
                    let a531 = 0u64;
                    nat_rem(a529, a530, a531)
                };
                let v13: u64 = {
                    let a533 = match v3.clone() {
                        Adt1::C0(h532, _, _, _, _, _, _, _, _, _, _) => {
                            h532
                        }
                    };
                    let a534 = 0u64;
                    if nat_eq(a533, a534) {
                        0u64
                    } else {
                        1u64
                    }
                };
                let v14: u64 = {
                    let a536 = match v4.clone() {
                        Adt1::C0(h535, _, _, _, _, _, _, _, _, _, _) => {
                            h535
                        }
                    };
                    let a537 = 0u64;
                    if nat_eq(a536, a537) {
                        0u64
                    } else {
                        1u64
                    }
                };
                let a538 = v12;
                let a539 = v13;
                if nat_lt(a538, a539) {
                    Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                        Adt1::C0(h540, _, _, _, _, _, _, _, _, _, _) => {
                            h540
                        }
                    }, 0u64, 0u64, f51(v11, v3.clone())?, 2u64, 0u64, 0u64, 0u64, f34(match v3 {
                        Adt1::C0(h541, _, _, _, _, _, _, _, _, _, _) => {
                            h541
                        }
                    }), 4u64))
                } else {
                    let a544 = v12;
                    let a545 = {
                        let a542 = v13;
                        let a543 = v14;
                        nat_add(a542, a543)?
                    };
                    if nat_lt(a544, a545) {
                        Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                            Adt1::C0(h546, _, _, _, _, _, _, _, _, _, _) => {
                                h546
                            }
                        }, 0u64, 4u64, f52(v11, v3, v4.clone())?, 2u64, 0u64, 0u64, 1u64, f34(match v4 {
                            Adt1::C0(h547, _, _, _, _, _, _, _, _, _, _) => {
                                h547
                            }
                        }), 4u64))
                    } else {
                        let a550 = v12;
                        let a551 = {
                            let a548 = v13;
                            let a549 = v14;
                            nat_add(a548, a549)?
                        };
                        if nat_eq(a550, a551) {
                            Ok(Adt3::C3(1u64, match v1 {
                                Adt2::C0(_, _, _, h552, _, _, _, _, _, _, _, _) => {
                                    h552
                                }
                            }, 0u64, 0u64, {
                                let a553 = v13;
                                let a554 = 0u64;
                                if nat_eq(a553, a554) {
                                    0u64
                                } else {
                                    1u64
                                }
                            }, {
                                let a555 = v13;
                                let a556 = 0u64;
                                if nat_eq(a555, a556) {
                                    f51(v11, v3.clone())?
                                } else {
                                    0u64
                                }
                            }, {
                                let a557 = v14;
                                let a558 = 0u64;
                                if nat_eq(a557, a558) {
                                    4u64
                                } else {
                                    1u64
                                }
                            }, {
                                let a559 = v14;
                                let a560 = 0u64;
                                if nat_eq(a559, a560) {
                                    f52(v11, v3, v4)?
                                } else {
                                    1u64
                                }
                            }, {
                                let a562 = match v5.clone() {
                                    Adt1::C0(h561, _, _, _, _, _, _, _, _, _, _) => {
                                        h561
                                    }
                                };
                                let a563 = 0u64;
                                if nat_eq(a562, a563) {
                                    3u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a565 = match v5.clone() {
                                    Adt1::C0(h564, _, _, _, _, _, _, _, _, _, _) => {
                                        h564
                                    }
                                };
                                let a566 = 0u64;
                                if nat_eq(a565, a566) {
                                    f51(v11, v5)?
                                } else {
                                    0u64
                                }
                            }, 4u64, 4u64))
                        } else {
                            Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                                Adt1::C0(h567, _, _, _, _, _, _, _, _, _, _) => {
                                    h567
                                }
                            }, 1u64, 0u64, 2u64, 0u64, 3u64, f51(v11, v5.clone())?, 4u64, f34(match v5 {
                                Adt1::C0(h568, _, _, _, _, _, _, _, _, _, _) => {
                                    h568
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
    let a569 = 6u64;
    let a570 = v0;
    if if if if nat_le(a569, a570) {
        let a571 = v0;
        let a572 = 23u64;
        nat_le(a571, a572)
    } else {
        false
    } {
        true
    } else {
        let a573 = v0;
        let a574 = 2u64;
        if nat_eq(a573, a574) {
            true
        } else {
            let a575 = v0;
            let a576 = 3u64;
            nat_eq(a575, a576)
        }
    } {
        true
    } else {
        let a577 = v0;
        let a578 = 29u64;
        nat_eq(a577, a578)
    } {
        true
    } else {
        let a579 = 34u64;
        let a580 = v0;
        if if nat_le(a579, a580) {
            let a581 = v0;
            let a582 = 35u64;
            nat_le(a581, a582)
        } else {
            false
        } {
            true
        } else {
            let a583 = 39u64;
            let a584 = v0;
            if nat_le(a583, a584) {
                let a585 = v0;
                let a586 = 42u64;
                nat_le(a585, a586)
            } else {
                false
            }
        }
    }
}

pub fn f19(v0: u64, v1: u64, v2: u64) -> bool {
    let a587 = v0;
    let a588 = 0u64;
    if if if nat_eq(a587, a588) {
        let a589 = v1;
        let a590 = 0u64;
        nat_eq(a589, a590)
    } else {
        false
    } {
        let a591 = v2;
        let a592 = 0u64;
        nat_eq(a591, a592)
    } else {
        false
    } {
        true
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
            let a597 = v1;
            let a598 = v0;
            if if nat_eq(a597, a598) {
                let a599 = v2;
                let a600 = v0;
                nat_eq(a599, a600)
            } else {
                false
            } {
                true
            } else {
                let a601 = v1;
                let a602 = 0u64;
                if nat_eq(a601, a602) {
                    let a603 = v2;
                    let a604 = v0;
                    if nat_eq(a603, a604) {
                        true
                    } else {
                        let a605 = v2;
                        let a606 = 0u64;
                        nat_eq(a605, a606)
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
    let a607 = v1;
    let a608 = 27u64;
    if if if nat_eq(a607, a608) {
        true
    } else {
        let a609 = v1;
        let a610 = 28u64;
        nat_eq(a609, a610)
    } {
        let a611 = v1;
        let a612 = v2;
        nat_eq(a611, a612)
    } else {
        false
    } {
        let a613 = v0;
        let a614 = 3u64;
        nat_eq(a613, a614)
    } else {
        f22(v1)
    }
}

pub fn f21(v0: u64) -> u64 {
    let a615 = v0;
    let a616 = 0u64;
    if nat_eq(a615, a616) {
        4u64
    } else {
        let a617 = v0;
        let a618 = 1u64;
        if if nat_eq(a617, a618) {
            true
        } else {
            let a619 = v0;
            let a620 = 30u64;
            nat_eq(a619, a620)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f22(v0: u64) -> bool {
    let a621 = 0u64;
    let a622 = f63(v0);
    if nat_lt(a621, a622) {
        true
    } else {
        let a623 = 24u64;
        let a624 = v0;
        if nat_le(a623, a624) {
            let a625 = v0;
            let a626 = 28u64;
            nat_le(a625, a626)
        } else {
            false
        }
    }
}

pub fn f23(v0: Adt1, v1: Adt1) -> bool {
    if f31(v1.clone()) {
        f31(v0)
    } else {
        let a632 = {
            let a629 = match v0.clone() {
                Adt1::C0(_, h627, _, _, _, _, _, _, _, _, _) => {
                    h627
                }
            };
            let a630 = match v1.clone() {
                Adt1::C0(_, h628, _, _, _, _, _, _, _, _, _) => {
                    h628
                }
            };
            let a631 = 1u64;
            nat_rem(a629, a630, a631)
        };
        let a633 = 0u64;
        if if if nat_eq(a632, a633) {
            let a639 = {
                let a636 = match v0.clone() {
                    Adt1::C0(_, _, h634, _, _, _, _, _, _, _, _) => {
                        h634
                    }
                };
                let a637 = match v1.clone() {
                    Adt1::C0(_, _, h635, _, _, _, _, _, _, _, _) => {
                        h635
                    }
                };
                let a638 = 1u64;
                nat_rem(a636, a637, a638)
            };
            let a640 = 0u64;
            nat_eq(a639, a640)
        } else {
            false
        } {
            let a646 = {
                let a643 = match v0.clone() {
                    Adt1::C0(_, _, _, h641, _, _, _, _, _, _, _) => {
                        h641
                    }
                };
                let a644 = match v1.clone() {
                    Adt1::C0(_, _, _, h642, _, _, _, _, _, _, _) => {
                        h642
                    }
                };
                let a645 = 1u64;
                nat_rem(a643, a644, a645)
            };
            let a647 = 0u64;
            nat_eq(a646, a647)
        } else {
            false
        } {
            let a653 = {
                let a650 = match v0 {
                    Adt1::C0(_, _, _, _, h648, _, _, _, _, _, _) => {
                        h648
                    }
                };
                let a651 = match v1 {
                    Adt1::C0(_, _, _, _, h649, _, _, _, _, _, _) => {
                        h649
                    }
                };
                let a652 = 1u64;
                nat_rem(a650, a651, a652)
            };
            let a654 = 0u64;
            nat_eq(a653, a654)
        } else {
            false
        }
    }
}

pub fn f24(v0: Adt1, v1: Adt1) -> bool {
    let a657 = match v0.clone() {
        Adt1::C0(_, h655, _, _, _, _, _, _, _, _, _) => {
            h655
        }
    };
    let a658 = match v1.clone() {
        Adt1::C0(_, h656, _, _, _, _, _, _, _, _, _) => {
            h656
        }
    };
    if if if nat_eq(a657, a658) {
        let a661 = match v0.clone() {
            Adt1::C0(_, _, h659, _, _, _, _, _, _, _, _) => {
                h659
            }
        };
        let a662 = match v1.clone() {
            Adt1::C0(_, _, h660, _, _, _, _, _, _, _, _) => {
                h660
            }
        };
        nat_eq(a661, a662)
    } else {
        false
    } {
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
        nat_eq(a665, a666)
    } else {
        false
    } {
        let a669 = match v0 {
            Adt1::C0(_, _, _, _, h667, _, _, _, _, _, _) => {
                h667
            }
        };
        let a670 = match v1 {
            Adt1::C0(_, _, _, _, h668, _, _, _, _, _, _) => {
                h668
            }
        };
        nat_eq(a669, a670)
    } else {
        false
    }
}

pub fn f25(v0: u64, v1: u64, v2: Adt1, v3: u64) -> bool {
    let v4: u64 = match v2.clone() {
        Adt1::C0(h671, _, _, _, _, _, _, _, _, _, _) => {
            h671
        }
    };
    let a672 = v1;
    let a673 = v3;
    if if nat_eq(a672, a673) {
        true
    } else {
        let a674 = v0;
        let a675 = 0u64;
        if nat_eq(a674, a675) {
            let a676 = v4;
            let a677 = 0u64;
            if if nat_eq(a676, a677) {
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
            let a679 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, h678, _, _, _, _, _) => {
                    h678
                }
            };
            let a680 = f27(v4);
            nat_eq(a679, a680)
        } else {
            false
        } {
            let a685 = {
                let a682 = match v2 {
                    Adt1::C0(_, h681, _, _, _, _, _, _, _, _, _) => {
                        h681
                    }
                };
                let a683 = f28(v4);
                let a684 = 1u64;
                nat_rem(a682, a683, a684)
            };
            let a686 = 0u64;
            nat_eq(a685, a686)
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
        let a687 = v0;
        let a688 = v1;
        if if nat_eq(a687, a688) {
            f22(v0)
        } else {
            false
        } {
            true
        } else {
            let a689 = f21(v0);
            let a690 = 2u64;
            if nat_eq(a689, a690) {
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
    let a691 = 24u64;
    let a692 = v0;
    if if nat_le(a691, a692) {
        let a693 = v0;
        let a694 = 28u64;
        nat_le(a693, a694)
    } else {
        false
    } {
        let m695 = f64(v0);
        match m695 {
            Adt4::C0 => {
                0u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, _, h696) => {
                        h696
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
        let m697 = f64(v0);
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

pub fn f29(v0: Adt1, v1: Adt1) -> R<bool> {
    if f48(v0.clone(), v1.clone())? {
        Ok(true)
    } else {
        Ok(f65(v0, v1))
    }
}

pub fn f30(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f66(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f66(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f66(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f66(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f66(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f66(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f31(v0: Adt1) -> bool {
    f67(match v0.clone() {
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

pub fn f32(v0: u64) -> Option<u64> {
    let a703 = v0;
    let a704 = 16u64;
    if if if nat_eq(a703, a704) {
        true
    } else {
        let a705 = v0;
        let a706 = 17u64;
        nat_eq(a705, a706)
    } {
        true
    } else {
        let a707 = v0;
        let a708 = 19u64;
        nat_eq(a707, a708)
    } {
        Some(1065353216u64)
    } else {
        None::<u64>
    }
}

pub fn f33(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<bool> {
    let a710 = match v1.clone() {
        Adt1::C0(h709, _, _, _, _, _, _, _, _, _, _) => {
            h709
        }
    };
    let a711 = 43u64;
    if nat_eq(a710, a711) {
        let a712 = v0;
        let a713 = 0u64;
        if if if if if if nat_eq(a712, a713) {
            let a715 = match v2.clone() {
                Adt1::C0(h714, _, _, _, _, _, _, _, _, _, _) => {
                    h714
                }
            };
            let a716 = 0u64;
            nat_eq(a715, a716)
        } else {
            false
        } {
            let a718 = match v3.clone() {
                Adt1::C0(h717, _, _, _, _, _, _, _, _, _, _) => {
                    h717
                }
            };
            let a719 = 0u64;
            if nat_eq(a718, a719) {
                true
            } else {
                let a721 = match v3.clone() {
                    Adt1::C0(h720, _, _, _, _, _, _, _, _, _, _) => {
                        h720
                    }
                };
                let a722 = 43u64;
                nat_eq(a721, a722)
            }
        } else {
            false
        } {
            let a724 = match v3.clone() {
                Adt1::C0(h723, _, _, _, _, _, _, _, _, _, _) => {
                    h723
                }
            };
            let a725 = 43u64;
            if nat_eq(a724, a725) {
                f23(v1.clone(), v2.clone())
            } else {
                f24(v1.clone(), v2.clone())
            }
        } else {
            false
        } {
            let a727 = match v2 {
                Adt1::C0(_, _, _, _, _, h726, _, _, _, _, _) => {
                    h726
                }
            };
            let a728 = 4u64;
            nat_eq(a727, a728)
        } else {
            false
        } {
            let a731 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h729, _, _, _, _, _) => {
                    h729
                }
            };
            let a732 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h730, _, _, _, _) => {
                    h730
                }
            };
            if nat_le(a731, a732) {
                let a735 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, h733, _, _, _, _) => {
                        h733
                    }
                };
                let a736 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h734, _, _, _) => {
                        h734
                    }
                };
                if nat_le(a735, a736) {
                    let a739 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, h737, _, _, _) => {
                            h737
                        }
                    };
                    let a740 = match v3 {
                        Adt1::C0(_, _, _, _, _, _, _, _, h738, _, _) => {
                            h738
                        }
                    };
                    nat_le(a739, a740)
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        } {
            let a748 = {
                let a746 = match v1.clone() {
                    Adt1::C0(_, _, h741, _, _, _, _, _, _, _, _) => {
                        h741
                    }
                };
                let a747 = {
                    let a744 = match v1.clone() {
                        Adt1::C0(_, _, _, h742, _, _, _, _, _, _, _) => {
                            h742
                        }
                    };
                    let a745 = match v1 {
                        Adt1::C0(_, _, _, _, h743, _, _, _, _, _, _) => {
                            h743
                        }
                    };
                    nat_mul(a744, a745)?
                };
                nat_mul(a746, a747)?
            };
            let a749 = 2147483647u64;
            Ok(nat_le(a748, a749))
        } else {
            Ok(false)
        }
    } else {
        if f23(v1.clone(), v2.clone()) {
            if f19(match v1.clone() {
                Adt1::C0(h750, _, _, _, _, _, _, _, _, _, _) => {
                    h750
                }
            }, match v2.clone() {
                Adt1::C0(h751, _, _, _, _, _, _, _, _, _, _) => {
                    h751
                }
            }, match v3.clone() {
                Adt1::C0(h752, _, _, _, _, _, _, _, _, _, _) => {
                    h752
                }
            }) {
                Ok(true)
            } else {
                let a753 = v0;
                let a754 = 3u64;
                if if if if nat_lt(a753, a754) {
                    f60(match v1.clone() {
                        Adt1::C0(h755, _, _, _, _, _, _, _, _, _, _) => {
                            h755
                        }
                    })
                } else {
                    false
                } {
                    let a758 = match v1.clone() {
                        Adt1::C0(h756, _, _, _, _, _, _, _, _, _, _) => {
                            h756
                        }
                    };
                    let a759 = match v2.clone() {
                        Adt1::C0(h757, _, _, _, _, _, _, _, _, _, _) => {
                            h757
                        }
                    };
                    nat_eq(a758, a759)
                } else {
                    false
                } {
                    let a762 = match v1 {
                        Adt1::C0(h760, _, _, _, _, _, _, _, _, _, _) => {
                            h760
                        }
                    };
                    let a763 = match v3 {
                        Adt1::C0(h761, _, _, _, _, _, _, _, _, _, _) => {
                            h761
                        }
                    };
                    nat_eq(a762, a763)
                } else {
                    false
                } {
                    let a765 = match v2 {
                        Adt1::C0(_, _, _, _, _, h764, _, _, _, _, _) => {
                            h764
                        }
                    };
                    let a766 = 8u64;
                    Ok(nat_eq(a765, a766))
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
    if f60(v0) {
        8u64
    } else {
        f63(v0)
    }
}

pub fn f35(v0: Adt1) -> bool {
    let v1: u64 = f34(match v0.clone() {
        Adt1::C0(h767, _, _, _, _, _, _, _, _, _, _) => {
            h767
        }
    });
    let a772 = {
        let a769 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h768) => {
                h768
            }
        };
        let a770 = v1;
        let a771 = 1u64;
        nat_rem(a769, a770, a771)
    };
    let a773 = 0u64;
    if if nat_eq(a772, a773) {
        f68(match v0.clone() {
            Adt1::C0(_, h774, _, _, _, _, _, _, _, _, _) => {
                h774
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h775, _, _, _, _, _) => {
                h775
            }
        }, v1)
    } else {
        false
    } {
        f69(match v0.clone() {
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
        }, v1)
    } else {
        false
    }
}

pub fn f36(v0: Adt1, v1: Adt1) -> R<bool> {
    let a789 = {
        let a787 = match v0.clone() {
            Adt1::C0(_, _, h782, _, _, _, _, _, _, _, _) => {
                h782
            }
        };
        let a788 = {
            let a785 = match v0.clone() {
                Adt1::C0(_, _, _, h783, _, _, _, _, _, _, _) => {
                    h783
                }
            };
            let a786 = match v0.clone() {
                Adt1::C0(_, _, _, _, h784, _, _, _, _, _, _) => {
                    h784
                }
            };
            nat_mul(a785, a786)?
        };
        nat_mul(a787, a788)?
    };
    let a790 = 9223372036854775807u64;
    if nat_le(a789, a790) {
        if f44(v1.clone()) {
            let a792 = match v1 {
                Adt1::C0(_, h791, _, _, _, _, _, _, _, _, _) => {
                    h791
                }
            };
            let a793 = 2147483647u64;
            Ok(nat_le(a792, a793))
        } else {
            let a795 = match v0 {
                Adt1::C0(_, h794, _, _, _, _, _, _, _, _, _) => {
                    h794
                }
            };
            let a796 = 2147483647u64;
            Ok(nat_le(a795, a796))
        }
    } else {
        Ok(false)
    }
}

pub fn f37(v0: Adt1) -> R<bool> {
    f30({
        let a799 = match v0.clone() {
            Adt1::C0(_, h797, _, _, _, _, _, _, _, _, _) => {
                h797
            }
        };
        let a800 = f34(match v0.clone() {
            Adt1::C0(h798, _, _, _, _, _, _, _, _, _, _) => {
                h798
            }
        });
        nat_mul(a799, a800)?
    }, match v0.clone() {
        Adt1::C0(_, _, h801, _, _, _, _, _, _, _, _) => {
            h801
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h802, _, _, _, _, _, _, _) => {
            h802
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h803, _, _, _, _, _, _) => {
            h803
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h804, _, _, _, _) => {
            h804
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h805, _, _, _) => {
            h805
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h806, _, _) => {
            h806
        }
    })
}

pub fn f38(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a813 = {
        let a807 = v0;
        let a808 = v2;
        nat_mul(a807, a808)?
    };
    let a814 = {
        let a809 = v1;
        let a810 = 0u64;
        if nat_eq(a809, a810) {
            0u64
        } else {
            let a811 = v0;
            let a812 = 2u64;
            nat_mul(a811, a812)?
        }
    };
    nat_add(a813, a814)
}

pub fn f39(v0: Adt1) -> R<u64> {
    let a823 = {
        let a817 = match v0.clone() {
            Adt1::C0(_, h815, _, _, _, _, _, _, _, _, _) => {
                h815
            }
        };
        let a818 = match v0.clone() {
            Adt1::C0(_, _, h816, _, _, _, _, _, _, _, _) => {
                h816
            }
        };
        nat_mul(a817, a818)?
    };
    let a824 = {
        let a821 = match v0.clone() {
            Adt1::C0(_, _, _, h819, _, _, _, _, _, _, _) => {
                h819
            }
        };
        let a822 = match v0 {
            Adt1::C0(_, _, _, _, h820, _, _, _, _, _, _) => {
                h820
            }
        };
        nat_mul(a821, a822)?
    };
    nat_mul(a823, a824)
}

pub fn f40(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    if f60(v1) {
        Ok(f70(v0, v1))
    } else {
        f62(v1, v2, v3)
    }
}

pub fn f41(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f24(v0.clone(), v1.clone()) {
        let a827 = match v0.clone() {
            Adt1::C0(h825, _, _, _, _, _, _, _, _, _, _) => {
                h825
            }
        };
        let a828 = match v1.clone() {
            Adt1::C0(h826, _, _, _, _, _, _, _, _, _, _) => {
                h826
            }
        };
        nat_eq(a827, a828)
    } else {
        false
    } {
        let a831 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h829, _, _, _, _, _) => {
                h829
            }
        };
        let a832 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h830, _, _, _, _, _) => {
                h830
            }
        };
        nat_eq(a831, a832)
    } else {
        false
    } {
        let a835 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h833, _, _, _, _) => {
                h833
            }
        };
        let a836 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h834, _, _, _, _) => {
                h834
            }
        };
        nat_eq(a835, a836)
    } else {
        false
    } {
        let a839 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h837, _, _, _) => {
                h837
            }
        };
        let a840 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h838, _, _, _) => {
                h838
            }
        };
        nat_eq(a839, a840)
    } else {
        false
    } {
        let a843 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h841, _, _) => {
                h841
            }
        };
        let a844 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h842, _, _) => {
                h842
            }
        };
        nat_eq(a843, a844)
    } else {
        false
    } {
        let a846 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h845) => {
                h845
            }
        };
        let a847 = 0u64;
        if if nat_eq(a846, a847) {
            true
        } else {
            let a849 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h848) => {
                    h848
                }
            };
            let a850 = 0u64;
            nat_eq(a849, a850)
        } {
            true
        } else {
            let a853 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h851) => {
                    h851
                }
            };
            let a854 = match v1 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h852) => {
                    h852
                }
            };
            nat_eq(a853, a854)
        }
    } else {
        false
    }
}

pub fn f42(v0: u64) -> u64 {
    if f18(v0) {
        let m855 = f64(v0);
        match m855 {
            Adt4::C0 => {
                0u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, _, h856) => {
                        h856
                    }
                }
            }
        }
    } else {
        let a857 = v0;
        let a858 = 26u64;
        if nat_eq(a857, a858) {
            4u64
        } else {
            f63(v0)
        }
    }
}

pub fn f43(v0: u64) -> u64 {
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

pub fn f44(v0: Adt1) -> bool {
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
        let a869 = f63(match v0 {
            Adt1::C0(h867, _, _, _, _, _, _, _, _, _, _) => {
                h867
            }
        });
        nat_eq(a868, a869)
    }
}

pub fn f45(v0: Adt1, v1: u64) -> R<u64> {
    let a875 = {
        let a871 = match v0.clone() {
            Adt1::C0(_, h870, _, _, _, _, _, _, _, _, _) => {
                h870
            }
        };
        let a872 = f28(v1);
        let a873 = 0u64;
        nat_quot(a871, a872, a873)
    };
    let a876 = f71(match v0 {
        Adt1::C0(h874, _, _, _, _, _, _, _, _, _, _) => {
            h874
        }
    }, v1);
    nat_mul(a875, a876)
}

pub fn f46(v0: u64) -> u64 {
    if if f18(v0) {
        true
    } else {
        let a877 = v0;
        let a878 = 26u64;
        nat_eq(a877, a878)
    } {
        f28(v0)
    } else {
        0u64
    }
}

pub fn f47(v0: Adt1) -> R<bool> {
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

pub fn f48(v0: Adt1, v1: Adt1) -> R<bool> {
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

pub fn f49(v0: Adt1) -> R<bool> {
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
    if if f44(v0.clone()) {
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

pub fn f50(v0: u64) -> u64 {
    let a973 = v0;
    let a974 = 3u64;
    if nat_eq(a973, a974) {
        16u64
    } else {
        10u64
    }
}

pub fn f51(v0: u64, v1: Adt1) -> R<u64> {
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
    f53({
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

pub fn f52(v0: u64, v1: Adt1, v2: Adt1) -> R<u64> {
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
    f53({
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

pub fn f53(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
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

pub fn f54(v0: u64, v1: Adt1, v2: u64, v3: u64) -> R<Adt3> {
    let v4: u64 = {
        let a1061 = v0;
        let a1062 = 2u64;
        let a1063 = 0u64;
        nat_quot(a1061, a1062, a1063)
    };
    let v5: u64 = f43(match v1.clone() {
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
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 4u64, f51(v4, v1)?, 2u64, 0u64, 0u64, 2u64, v5, 8u64))
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

pub fn f55(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64) -> R<Adt3> {
    let v5: u64 = {
        let a1079 = 1u64;
        let a1080 = f45(v1.clone(), match v3.clone() {
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
                let a1133 = f71(match v1.clone() {
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
            let v14: u64 = f53({
                let a1136 = v13;
                let a1137 = f28(match v3.clone() {
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
            let v15: u64 = f53(v13, v4, v10, v11, match v3.clone() {
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
            let a1147 = 0u64;
            let a1148 = f46(match v3.clone() {
                Adt1::C0(h1146, _, _, _, _, _, _, _, _, _, _) => {
                    h1146
                }
            });
            if nat_lt(a1147, a1148) {
                Ok(Adt3::C6(v14, v15))
            } else {
                Ok(f72(match v1 {
                    Adt1::C0(h1149, _, _, _, _, _, _, _, _, _, _) => {
                        h1149
                    }
                }, match v3 {
                    Adt1::C0(h1150, _, _, _, _, _, _, _, _, _, _) => {
                        h1150
                    }
                }, {
                    let a1151 = v6;
                    let a1152 = 1u64;
                    nat_sub(a1151, a1152)
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
    let a1153 = v0;
    let a1154 = 8u64;
    if nat_eq(a1153, a1154) {
        Adt3::C3(1u64, 6u64, 0u64, 0u64, 1u64, 0u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
    } else {
        let a1155 = v0;
        let a1156 = 9u64;
        if nat_eq(a1155, a1156) {
            Adt3::C3(1u64, 2u64, 0u64, 0u64, 2u64, v1, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
        } else {
            let a1157 = v0;
            let a1158 = 10u64;
            if nat_eq(a1157, a1158) {
                Adt3::C3(5u64, 16u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
            } else {
                let a1159 = v0;
                let a1160 = 11u64;
                if nat_eq(a1159, a1160) {
                    Adt3::C3(1u64, 0u64, 0u64, 0u64, 2u64, 1065353216u64, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
                } else {
                    let a1161 = v0;
                    let a1162 = 12u64;
                    if nat_eq(a1161, a1162) {
                        Adt3::C3(1u64, 3u64, 0u64, 0u64, 1u64, 0u64, 1u64, 2u64, 0u64, 0u64, 4u64, 4u64)
                    } else {
                        let a1163 = v0;
                        let a1164 = 13u64;
                        if nat_eq(a1163, a1164) {
                            Adt3::C3(1u64, 0u64, 0u64, 0u64, 1u64, 1u64, 2u64, 1065353216u64, 0u64, 1u64, 4u64, 4u64)
                        } else {
                            let a1165 = v0;
                            let a1166 = 14u64;
                            if nat_eq(a1165, a1166) {
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
        Adt2::C0(_, _, _, _, _, _, _, h1167, _, _, _, _) => {
            h1167
        }
    };
    let v4: Adt1 = match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h1168, _, _, _) => {
            h1168
        }
    };
    let v5: Adt1 = match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1169, _, _) => {
            h1169
        }
    };
    let v6: u64 = match v2.clone() {
        Adt2::C0(_, _, _, h1170, _, _, _, _, _, _, _, _) => {
            h1170
        }
    };
    let v7: u64 = {
        let a1172 = v1;
        let a1173 = match v5.clone() {
            Adt1::C0(_, h1171, _, _, _, _, _, _, _, _, _) => {
                h1171
            }
        };
        let a1174 = 0u64;
        nat_quot(a1172, a1173, a1174)
    };
    let v8: u64 = f21(match v5.clone() {
        Adt1::C0(h1175, _, _, _, _, _, _, _, _, _, _) => {
            h1175
        }
    });
    let v9: u64 = {
        let a1180 = {
            let a1177 = v1;
            let a1178 = match v5.clone() {
                Adt1::C0(_, h1176, _, _, _, _, _, _, _, _, _) => {
                    h1176
                }
            };
            let a1179 = 0u64;
            nat_rem(a1177, a1178, a1179)
        };
        let a1181 = v8;
        nat_mul(a1180, a1181)?
    };
    let v10: u64 = {
        let a1183 = match v5.clone() {
            Adt1::C0(_, h1182, _, _, _, _, _, _, _, _, _) => {
                h1182
            }
        };
        let a1184 = v8;
        nat_mul(a1183, a1184)?
    };
    let a1185 = v0;
    let a1186 = 0u64;
    if nat_eq(a1185, a1186) {
        Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
            Adt1::C0(h1187, _, _, _, _, _, _, _, _, _, _) => {
                h1187
            }
        }, 0u64, 0u64, {
            let a1195 = {
                let a1191 = {
                    let a1189 = v7;
                    let a1190 = match v3 {
                        Adt1::C0(_, _, _, _, _, _, h1188, _, _, _, _) => {
                            h1188
                        }
                    };
                    nat_mul(a1189, a1190)?
                };
                let a1192 = v9;
                nat_add(a1191, a1192)?
            };
            let a1196 = {
                let a1193 = v6;
                let a1194 = 100u64;
                if nat_eq(a1193, a1194) {
                    v10
                } else {
                    0u64
                }
            };
            nat_add(a1195, a1196)?
        }, 2u64, 0u64, 0u64, 0u64, v8, 4u64))
    } else {
        let a1197 = v0;
        let a1198 = 1u64;
        if nat_eq(a1197, a1198) {
            Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                Adt1::C0(h1199, _, _, _, _, _, _, _, _, _, _) => {
                    h1199
                }
            }, 0u64, {
                let a1200 = v6;
                let a1201 = 98u64;
                if nat_eq(a1200, a1201) {
                    4u64
                } else {
                    0u64
                }
            }, {
                let a1209 = {
                    let a1205 = {
                        let a1203 = v7;
                        let a1204 = match v4 {
                            Adt1::C0(_, _, _, _, _, _, h1202, _, _, _, _) => {
                                h1202
                            }
                        };
                        nat_mul(a1203, a1204)?
                    };
                    let a1206 = v9;
                    nat_add(a1205, a1206)?
                };
                let a1210 = {
                    let a1207 = v6;
                    let a1208 = 99u64;
                    if nat_eq(a1207, a1208) {
                        v10
                    } else {
                        0u64
                    }
                };
                nat_add(a1209, a1210)?
            }, 2u64, 0u64, 0u64, 1u64, v8, 4u64))
        } else {
            let a1211 = v0;
            let a1212 = 2u64;
            if nat_eq(a1211, a1212) {
                Ok(Adt3::C4(f73(match v2 {
                    Adt2::C0(_, _, _, _, h1213, _, _, _, _, _, _, _) => {
                        h1213
                    }
                }, v5.clone(), v1), match v5 {
                    Adt1::C0(h1214, _, _, _, _, _, _, _, _, _, _) => {
                        h1214
                    }
                }, 0u64, 1u64, 0u64))
            } else {
                Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                    Adt1::C0(h1215, _, _, _, _, _, _, _, _, _, _) => {
                        h1215
                    }
                }, 1u64, 0u64, 2u64, 0u64, 3u64, {
                    let a1219 = {
                        let a1217 = v7;
                        let a1218 = match v5 {
                            Adt1::C0(_, _, _, _, _, _, h1216, _, _, _, _) => {
                                h1216
                            }
                        };
                        nat_mul(a1217, a1218)?
                    };
                    let a1220 = v9;
                    nat_add(a1219, a1220)?
                }, 4u64, v8))
            }
        }
    }
}

pub fn f60(v0: u64) -> bool {
    let a1221 = v0;
    let a1222 = 27u64;
    if nat_eq(a1221, a1222) {
        true
    } else {
        let a1223 = v0;
        let a1224 = 28u64;
        nat_eq(a1223, a1224)
    }
}

pub fn f61(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1225, _, _, _, _) => {
            h1225
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h1226, _, _, _) => {
            h1226
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1227, _, _) => {
            h1227
        }
    };
    let v5: u64 = match v1 {
        Adt2::C0(_, _, _, h1228, _, _, _, _, _, _, _, _) => {
            h1228
        }
    };
    let v6: u64 = f70(v5, match v2.clone() {
        Adt1::C0(h1229, _, _, _, _, _, _, _, _, _, _) => {
            h1229
        }
    });
    let v7: u64 = {
        let a1230 = v0;
        let a1231 = v6;
        let a1232 = 0u64;
        nat_quot(a1230, a1231, a1232)
    };
    let v8: u64 = f52(v7, v2.clone(), v3)?;
    let a1233 = v6;
    let a1234 = 2u64;
    if if nat_eq(a1233, a1234) {
        let a1238 = {
            let a1235 = v0;
            let a1236 = v6;
            let a1237 = 0u64;
            nat_rem(a1235, a1236, a1237)
        };
        let a1239 = 0u64;
        nat_eq(a1238, a1239)
    } else {
        false
    } {
        Ok(Adt3::C3(6u64, 15u64, 0u64, 0u64, 4u64, v8, 2u64, 9223372036854775808u64, 0u64, 0u64, 8u64, 8u64))
    } else {
        let v9: u64 = {
            let a1241 = match v2.clone() {
                Adt1::C0(h1240, _, _, _, _, _, _, _, _, _, _) => {
                    h1240
                }
            };
            let a1242 = 27u64;
            if nat_eq(a1241, a1242) {
                let a1243 = 15u64;
                let a1244 = v5;
                nat_add(a1243, a1244)?
            } else {
                let a1245 = v5;
                let a1246 = 2u64;
                if nat_eq(a1245, a1246) {
                    14u64
                } else {
                    0u64
                }
            }
        };
        Ok(Adt3::C3(6u64, v9, 0u64, 0u64, 0u64, f51(v7, v2)?, {
            let a1247 = v6;
            let a1248 = 2u64;
            if nat_eq(a1247, a1248) {
                1u64
            } else {
                4u64
            }
        }, {
            let a1249 = v6;
            let a1250 = 2u64;
            if nat_eq(a1249, a1250) {
                0u64
            } else {
                v8
            }
        }, 3u64, f51(v7, v4)?, 8u64, 8u64))
    }
}

pub fn f62(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1261 = {
        let a1257 = {
            let a1253 = 1u64;
            let a1254 = {
                let a1251 = v0;
                let a1252 = 0u64;
                if nat_eq(a1251, a1252) {
                    0u64
                } else {
                    1u64
                }
            };
            nat_add(a1253, a1254)?
        };
        let a1258 = {
            let a1255 = v1;
            let a1256 = 0u64;
            if nat_eq(a1255, a1256) {
                0u64
            } else {
                1u64
            }
        };
        nat_add(a1257, a1258)?
    };
    let a1262 = {
        let a1259 = v2;
        let a1260 = 0u64;
        if nat_eq(a1259, a1260) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a1261, a1262)
}

pub fn f63(v0: u64) -> u64 {
    let a1263 = v0;
    let a1264 = 43u64;
    if nat_eq(a1263, a1264) {
        1u64
    } else {
        f21(v0)
    }
}

pub fn f64(v0: u64) -> Adt4 {
    let a1265 = v0;
    let a1266 = 0u64;
    if nat_eq(a1265, a1266) {
        Adt4::C1(Adt5::C0(0u64, 1u64, 4u64))
    } else {
        let a1267 = v0;
        let a1268 = 1u64;
        if nat_eq(a1267, a1268) {
            Adt4::C1(Adt5::C0(1u64, 1u64, 2u64))
        } else {
            let a1269 = v0;
            let a1270 = 2u64;
            if nat_eq(a1269, a1270) {
                Adt4::C1(Adt5::C0(2u64, 32u64, 18u64))
            } else {
                let a1271 = v0;
                let a1272 = 3u64;
                if nat_eq(a1271, a1272) {
                    Adt4::C1(Adt5::C0(3u64, 32u64, 20u64))
                } else {
                    let a1273 = v0;
                    let a1274 = 6u64;
                    if nat_eq(a1273, a1274) {
                        Adt4::C1(Adt5::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1275 = v0;
                        let a1276 = 7u64;
                        if nat_eq(a1275, a1276) {
                            Adt4::C1(Adt5::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1277 = v0;
                            let a1278 = 8u64;
                            if nat_eq(a1277, a1278) {
                                Adt4::C1(Adt5::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1279 = v0;
                                let a1280 = 9u64;
                                if nat_eq(a1279, a1280) {
                                    Adt4::C1(Adt5::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1281 = v0;
                                    let a1282 = 10u64;
                                    if nat_eq(a1281, a1282) {
                                        Adt4::C1(Adt5::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1283 = v0;
                                        let a1284 = 11u64;
                                        if nat_eq(a1283, a1284) {
                                            Adt4::C1(Adt5::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1285 = v0;
                                            let a1286 = 12u64;
                                            if nat_eq(a1285, a1286) {
                                                Adt4::C1(Adt5::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1287 = v0;
                                                let a1288 = 13u64;
                                                if nat_eq(a1287, a1288) {
                                                    Adt4::C1(Adt5::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1289 = v0;
                                                    let a1290 = 14u64;
                                                    if nat_eq(a1289, a1290) {
                                                        Adt4::C1(Adt5::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1291 = v0;
                                                        let a1292 = 15u64;
                                                        if nat_eq(a1291, a1292) {
                                                            Adt4::C1(Adt5::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1293 = v0;
                                                            let a1294 = 16u64;
                                                            if nat_eq(a1293, a1294) {
                                                                Adt4::C1(Adt5::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1295 = v0;
                                                                let a1296 = 17u64;
                                                                if nat_eq(a1295, a1296) {
                                                                    Adt4::C1(Adt5::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1297 = v0;
                                                                    let a1298 = 18u64;
                                                                    if nat_eq(a1297, a1298) {
                                                                        Adt4::C1(Adt5::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1299 = v0;
                                                                        let a1300 = 19u64;
                                                                        if nat_eq(a1299, a1300) {
                                                                            Adt4::C1(Adt5::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1301 = v0;
                                                                            let a1302 = 20u64;
                                                                            if nat_eq(a1301, a1302) {
                                                                                Adt4::C1(Adt5::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1303 = v0;
                                                                                let a1304 = 21u64;
                                                                                if nat_eq(a1303, a1304) {
                                                                                    Adt4::C1(Adt5::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1305 = v0;
                                                                                    let a1306 = 22u64;
                                                                                    if nat_eq(a1305, a1306) {
                                                                                        Adt4::C1(Adt5::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1307 = v0;
                                                                                        let a1308 = 23u64;
                                                                                        if nat_eq(a1307, a1308) {
                                                                                            Adt4::C1(Adt5::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1309 = v0;
                                                                                            let a1310 = 24u64;
                                                                                            if nat_eq(a1309, a1310) {
                                                                                                Adt4::C1(Adt5::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1311 = v0;
                                                                                                let a1312 = 25u64;
                                                                                                if nat_eq(a1311, a1312) {
                                                                                                    Adt4::C1(Adt5::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1313 = v0;
                                                                                                    let a1314 = 26u64;
                                                                                                    if nat_eq(a1313, a1314) {
                                                                                                        Adt4::C1(Adt5::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1315 = v0;
                                                                                                        let a1316 = 27u64;
                                                                                                        if nat_eq(a1315, a1316) {
                                                                                                            Adt4::C1(Adt5::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1317 = v0;
                                                                                                            let a1318 = 28u64;
                                                                                                            if nat_eq(a1317, a1318) {
                                                                                                                Adt4::C1(Adt5::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1319 = v0;
                                                                                                                let a1320 = 29u64;
                                                                                                                if nat_eq(a1319, a1320) {
                                                                                                                    Adt4::C1(Adt5::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1321 = v0;
                                                                                                                    let a1322 = 30u64;
                                                                                                                    if nat_eq(a1321, a1322) {
                                                                                                                        Adt4::C1(Adt5::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1323 = v0;
                                                                                                                        let a1324 = 34u64;
                                                                                                                        if nat_eq(a1323, a1324) {
                                                                                                                            Adt4::C1(Adt5::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1325 = v0;
                                                                                                                            let a1326 = 35u64;
                                                                                                                            if nat_eq(a1325, a1326) {
                                                                                                                                Adt4::C1(Adt5::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1327 = v0;
                                                                                                                                let a1328 = 39u64;
                                                                                                                                if nat_eq(a1327, a1328) {
                                                                                                                                    Adt4::C1(Adt5::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1329 = v0;
                                                                                                                                    let a1330 = 40u64;
                                                                                                                                    if nat_eq(a1329, a1330) {
                                                                                                                                        Adt4::C1(Adt5::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1331 = v0;
                                                                                                                                        let a1332 = 41u64;
                                                                                                                                        if nat_eq(a1331, a1332) {
                                                                                                                                            Adt4::C1(Adt5::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1333 = v0;
                                                                                                                                            let a1334 = 42u64;
                                                                                                                                            if nat_eq(a1333, a1334) {
                                                                                                                                                Adt4::C1(Adt5::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1335 = v0;
                                                                                                                                                let a1336 = 43u64;
                                                                                                                                                if nat_eq(a1335, a1336) {
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

pub fn f65(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f24(v0.clone(), v1.clone()) {
        let a1339 = match v0.clone() {
            Adt1::C0(h1337, _, _, _, _, _, _, _, _, _, _) => {
                h1337
            }
        };
        let a1340 = match v1.clone() {
            Adt1::C0(h1338, _, _, _, _, _, _, _, _, _, _) => {
                h1338
            }
        };
        nat_eq(a1339, a1340)
    } else {
        false
    } {
        let a1343 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1341) => {
                h1341
            }
        };
        let a1344 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1342) => {
                h1342
            }
        };
        nat_eq(a1343, a1344)
    } else {
        false
    } {
        let a1347 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1345, _, _, _, _, _) => {
                h1345
            }
        };
        let a1348 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1346, _, _, _, _, _) => {
                h1346
            }
        };
        nat_eq(a1347, a1348)
    } else {
        false
    } {
        let a1351 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1349, _, _, _, _) => {
                h1349
            }
        };
        let a1352 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1350, _, _, _, _) => {
                h1350
            }
        };
        nat_eq(a1351, a1352)
    } else {
        false
    } {
        let a1355 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1353, _, _, _) => {
                h1353
            }
        };
        let a1356 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1354, _, _, _) => {
                h1354
            }
        };
        nat_eq(a1355, a1356)
    } else {
        false
    } {
        let a1359 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1357, _, _) => {
                h1357
            }
        };
        let a1360 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1358, _, _) => {
                h1358
            }
        };
        nat_eq(a1359, a1360)
    } else {
        false
    }
}

pub fn f66(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a1361 = v1;
    let a1362 = 1u64;
    if if if nat_le(a1361, a1362) {
        true
    } else {
        let a1363 = v0;
        let a1364 = v2;
        nat_le(a1363, a1364)
    } {
        let a1365 = v3;
        let a1366 = 1u64;
        if nat_le(a1365, a1366) {
            true
        } else {
            let a1373 = {
                let a1371 = v0;
                let a1372 = {
                    let a1369 = {
                        let a1367 = v1;
                        let a1368 = 1u64;
                        nat_sub(a1367, a1368)
                    };
                    let a1370 = v2;
                    nat_mul(a1369, a1370)?
                };
                nat_add(a1371, a1372)?
            };
            let a1374 = v4;
            nat_le(a1373, a1374)
        }
    } else {
        false
    } {
        let a1375 = v5;
        let a1376 = 1u64;
        if nat_le(a1375, a1376) {
            Ok(true)
        } else {
            let a1389 = {
                let a1387 = {
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
                let a1388 = {
                    let a1385 = {
                        let a1383 = v3;
                        let a1384 = 1u64;
                        nat_sub(a1383, a1384)
                    };
                    let a1386 = v4;
                    nat_mul(a1385, a1386)?
                };
                nat_add(a1387, a1388)?
            };
            let a1390 = v6;
            Ok(nat_le(a1389, a1390))
        }
    } else {
        Ok(false)
    }
}

pub fn f67(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1391 = v0;
    let a1392 = 0u64;
    if if if nat_eq(a1391, a1392) {
        true
    } else {
        let a1393 = v1;
        let a1394 = 0u64;
        nat_eq(a1393, a1394)
    } {
        true
    } else {
        let a1395 = v2;
        let a1396 = 0u64;
        nat_eq(a1395, a1396)
    } {
        true
    } else {
        let a1397 = v3;
        let a1398 = 0u64;
        nat_eq(a1397, a1398)
    }
}

pub fn f68(v0: u64, v1: u64, v2: u64) -> bool {
    let a1399 = v0;
    let a1400 = 1u64;
    if nat_le(a1399, a1400) {
        true
    } else {
        let a1404 = {
            let a1401 = v1;
            let a1402 = v2;
            let a1403 = 1u64;
            nat_rem(a1401, a1402, a1403)
        };
        let a1405 = 0u64;
        nat_eq(a1404, a1405)
    }
}

pub fn f69(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f68(v0, v3, v6) {
        f68(v1, v4, v6)
    } else {
        false
    } {
        f68(v2, v5, v6)
    } else {
        false
    }
}

pub fn f70(v0: u64, v1: u64) -> u64 {
    let a1406 = v1;
    let a1407 = 28u64;
    if if nat_eq(a1406, a1407) {
        let a1408 = v0;
        let a1409 = 1u64;
        nat_eq(a1408, a1409)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f71(v0: u64, v1: u64) -> u64 {
    let a1410 = v0;
    let a1411 = 1u64;
    if if nat_eq(a1410, a1411) {
        let a1412 = v1;
        let a1413 = 43u64;
        nat_eq(a1412, a1413)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f72(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt3 {
    let a1414 = f71(v0, v1);
    let a1415 = 2u64;
    if nat_eq(a1414, a1415) {
        let a1419 = {
            let a1416 = v2;
            let a1417 = 2u64;
            let a1418 = 0u64;
            nat_rem(a1416, a1417, a1418)
        };
        let a1420 = 0u64;
        if nat_eq(a1419, a1420) {
            Adt3::C3(0u64, 0u64, v0, 0u64, 0u64, v3, 2u64, 0u64, 1u64, 0u64, f21(v0), 8u64)
        } else {
            Adt3::C3(0u64, 0u64, 0u64, v1, 1u64, 0u64, 2u64, 0u64, 3u64, v4, 8u64, f42(v1))
        }
    } else {
        Adt3::C3(0u64, 0u64, v0, v1, 0u64, v3, 2u64, 0u64, 3u64, v4, f21(v0), f42(v1))
    }
}

pub fn f73(v0: u64, v1: Adt1, v2: u64) -> u64 {
    let a1421 = v0;
    let a1422 = 2u64;
    if if nat_eq(a1421, a1422) {
        let a1424 = match v1.clone() {
            Adt1::C0(h1423, _, _, _, _, _, _, _, _, _, _) => {
                h1423
            }
        };
        let a1425 = 0u64;
        if nat_eq(a1424, a1425) {
            let a1437 = {
                let a1427 = v2;
                let a1428 = match v1.clone() {
                    Adt1::C0(_, h1426, _, _, _, _, _, _, _, _, _) => {
                        h1426
                    }
                };
                let a1429 = 0u64;
                nat_rem(a1427, a1428, a1429)
            };
            let a1438 = {
                let a1435 = match v1.clone() {
                    Adt1::C0(_, h1430, _, _, _, _, _, _, _, _, _) => {
                        h1430
                    }
                };
                let a1436 = {
                    let a1432 = match v1 {
                        Adt1::C0(_, h1431, _, _, _, _, _, _, _, _, _) => {
                            h1431
                        }
                    };
                    let a1433 = 4u64;
                    let a1434 = 0u64;
                    nat_rem(a1432, a1433, a1434)
                };
                nat_sub(a1435, a1436)
            };
            nat_lt(a1437, a1438)
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
