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
    let v3: u64 = f47(v0.clone(), v2.clone());
    if if if if if f41(v1.clone(), v2.clone()) {
        let a221 = 0u64;
        let a222 = v3;
        nat_lt(a221, a222)
    } else {
        false
    } {
        let a225 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h223, _, _, _, _, _) => {
                h223
            }
        };
        let a226 = f27(match v0.clone() {
            Adt1::C0(h224, _, _, _, _, _, _, _, _, _, _) => {
                h224
            }
        });
        nat_eq(a225, a226)
    } else {
        false
    } {
        let a229 = match v2.clone() {
            Adt1::C0(_, _, _, _, _, h227, _, _, _, _, _) => {
                h227
            }
        };
        let a230 = f27(match v2.clone() {
            Adt1::C0(h228, _, _, _, _, _, _, _, _, _, _) => {
                h228
            }
        });
        nat_eq(a229, a230)
    } else {
        false
    } {
        let a231 = f39(v0.clone())?;
        let a232 = f39(v2.clone())?;
        nat_eq(a231, a232)
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a233 = f39(v0.clone())?;
            let a234 = v3;
            let a235 = 0u64;
            nat_quot(a233, a234, a235)
        }, 3u64, f32(match v2.clone() {
            Adt1::C0(h236, _, _, _, _, _, _, _, _, _, _) => {
                h236
            }
        }), 33u64, 0u64, 0u64, 0u64, v0, v1, v2, false, v3)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f7(v0: u64, v1: u64, v2: Adt1, v3: Adt1) -> R<Adt3> {
    let a238 = match v2.clone() {
        Adt1::C0(h237, _, _, _, _, _, _, _, _, _, _) => {
            h237
        }
    };
    let a239 = 0u64;
    if if if if if if if if if if if nat_eq(a238, a239) {
        let a241 = match v3.clone() {
            Adt1::C0(h240, _, _, _, _, _, _, _, _, _, _) => {
                h240
            }
        };
        let a242 = 26u64;
        nat_eq(a241, a242)
    } else {
        false
    } {
        let a244 = match v2.clone() {
            Adt1::C0(_, h243, _, _, _, _, _, _, _, _, _) => {
                h243
            }
        };
        let a245 = 2147483647u64;
        nat_le(a244, a245)
    } else {
        false
    } {
        let a248 = match v3.clone() {
            Adt1::C0(_, h246, _, _, _, _, _, _, _, _, _) => {
                h246
            }
        };
        let a249 = match v2.clone() {
            Adt1::C0(_, h247, _, _, _, _, _, _, _, _, _) => {
                h247
            }
        };
        nat_le(a248, a249)
    } else {
        false
    } {
        let a252 = match v2.clone() {
            Adt1::C0(_, _, h250, _, _, _, _, _, _, _, _) => {
                h250
            }
        };
        let a253 = match v3.clone() {
            Adt1::C0(_, _, h251, _, _, _, _, _, _, _, _) => {
                h251
            }
        };
        nat_eq(a252, a253)
    } else {
        false
    } {
        let a256 = match v2.clone() {
            Adt1::C0(_, _, _, h254, _, _, _, _, _, _, _) => {
                h254
            }
        };
        let a257 = match v3.clone() {
            Adt1::C0(_, _, _, h255, _, _, _, _, _, _, _) => {
                h255
            }
        };
        nat_eq(a256, a257)
    } else {
        false
    } {
        let a260 = match v2.clone() {
            Adt1::C0(_, _, _, _, h258, _, _, _, _, _, _) => {
                h258
            }
        };
        let a261 = match v3.clone() {
            Adt1::C0(_, _, _, _, h259, _, _, _, _, _, _) => {
                h259
            }
        };
        nat_eq(a260, a261)
    } else {
        false
    } {
        let a262 = v0;
        let a263 = 67u64;
        if nat_eq(a262, a263) {
            true
        } else {
            let a264 = v1;
            let a265 = 1u64;
            if nat_le(a264, a265) {
                let a268 = match v3.clone() {
                    Adt1::C0(_, h266, _, _, _, _, _, _, _, _, _) => {
                        h266
                    }
                };
                let a269 = match v2.clone() {
                    Adt1::C0(_, h267, _, _, _, _, _, _, _, _, _) => {
                        h267
                    }
                };
                nat_eq(a268, a269)
            } else {
                false
            }
        }
    } else {
        false
    } {
        f48(v2.clone())?
    } else {
        false
    } {
        f48(v3.clone())?
    } else {
        false
    } {
        f49(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt3::C2(Adt2::C0({
            let a275 = match v3.clone() {
                Adt1::C0(_, _, h270, _, _, _, _, _, _, _, _) => {
                    h270
                }
            };
            let a276 = {
                let a273 = match v3.clone() {
                    Adt1::C0(_, _, _, h271, _, _, _, _, _, _, _) => {
                        h271
                    }
                };
                let a274 = match v3.clone() {
                    Adt1::C0(_, _, _, _, h272, _, _, _, _, _, _) => {
                        h272
                    }
                };
                nat_mul(a273, a274)?
            };
            nat_mul(a275, a276)?
        }, 3u64, None::<u64>, v0, {
            let a277 = v0;
            let a278 = 67u64;
            if nat_eq(a277, a278) {
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
    let a279 = v0;
    let a280 = 6u64;
    nat_le(a279, a280)
}

pub fn f9(v0: u64, v1: bool, v2: u64, v3: u64, v4: u64, v5: Adt1, v6: Adt1, v7: Adt1) -> R<Adt3> {
    let a281 = v2;
    let a282 = 1u64;
    if if if if if if if if if if if if if if if nat_le(a281, a282) {
        let a284 = 0u64;
        let a285 = f21(match v5.clone() {
            Adt1::C0(h283, _, _, _, _, _, _, _, _, _, _) => {
                h283
            }
        });
        nat_lt(a284, a285)
    } else {
        false
    } {
        let a288 = match v5.clone() {
            Adt1::C0(h286, _, _, _, _, _, _, _, _, _, _) => {
                h286
            }
        };
        let a289 = match v6.clone() {
            Adt1::C0(h287, _, _, _, _, _, _, _, _, _, _) => {
                h287
            }
        };
        nat_eq(a288, a289)
    } else {
        false
    } {
        let a292 = match v5.clone() {
            Adt1::C0(h290, _, _, _, _, _, _, _, _, _, _) => {
                h290
            }
        };
        let a293 = match v7.clone() {
            Adt1::C0(h291, _, _, _, _, _, _, _, _, _, _) => {
                h291
            }
        };
        nat_eq(a292, a293)
    } else {
        false
    } {
        f24(v5.clone(), v6.clone())
    } else {
        false
    } {
        f50(v5.clone())?
    } else {
        false
    } {
        f50(v6.clone())?
    } else {
        false
    } {
        f50(v7.clone())?
    } else {
        false
    } {
        let a296 = match v5.clone() {
            Adt1::C0(_, _, h294, _, _, _, _, _, _, _, _) => {
                h294
            }
        };
        let a297 = match v7.clone() {
            Adt1::C0(_, _, h295, _, _, _, _, _, _, _, _) => {
                h295
            }
        };
        nat_eq(a296, a297)
    } else {
        false
    } {
        let a300 = match v5.clone() {
            Adt1::C0(_, _, _, h298, _, _, _, _, _, _, _) => {
                h298
            }
        };
        let a301 = match v7.clone() {
            Adt1::C0(_, _, _, h299, _, _, _, _, _, _, _) => {
                h299
            }
        };
        nat_eq(a300, a301)
    } else {
        false
    } {
        let a304 = match v5.clone() {
            Adt1::C0(_, _, _, _, h302, _, _, _, _, _, _) => {
                h302
            }
        };
        let a305 = match v7.clone() {
            Adt1::C0(_, _, _, _, h303, _, _, _, _, _, _) => {
                h303
            }
        };
        nat_eq(a304, a305)
    } else {
        false
    } {
        let a312 = match v7.clone() {
            Adt1::C0(_, h306, _, _, _, _, _, _, _, _, _) => {
                h306
            }
        };
        let a313 = if v1 {
            match v5.clone() {
                Adt1::C0(_, h307, _, _, _, _, _, _, _, _, _) => {
                    h307
                }
            }
        } else {
            let a309 = match v5.clone() {
                Adt1::C0(_, h308, _, _, _, _, _, _, _, _, _) => {
                    h308
                }
            };
            let a310 = 2u64;
            let a311 = 0u64;
            nat_quot(a309, a310, a311)
        };
        nat_eq(a312, a313)
    } else {
        false
    } {
        let a315 = match v7.clone() {
            Adt1::C0(_, h314, _, _, _, _, _, _, _, _, _) => {
                h314
            }
        };
        let a316 = 2147483647u64;
        nat_le(a315, a316)
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
            let a319 = 1u64;
            let a320 = {
                let a317 = f51(v0);
                let a318 = f39(v7.clone())?;
                nat_mul(a317, a318)?
            };
            nat_add(a319, a320)?
        } else {
            let a321 = 4u64;
            let a322 = f39(v7.clone())?;
            nat_mul(a321, a322)?
        }, if f14(v0) {
            4u64
        } else {
            3u64
        }, None::<u64>, if v1 {
            98u64
        } else {
            let a323 = 99u64;
            let a324 = v2;
            nat_add(a323, a324)?
        }, v0, v3, v4, v5, v6, v7, false, 0u64)))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f10(v0: u64, v1: Adt2) -> R<Adt3> {
    let a326 = v0;
    let a327 = match v1.clone() {
        Adt2::C0(h325, _, _, _, _, _, _, _, _, _, _, _) => {
            h325
        }
    };
    if nat_lt(a326, a327) {
        let v2: u64 = {
            let a331 = v0;
            let a332 = {
                let m330 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h329, _, _, _, _) => {
                        h329
                    }
                };
                match m330 {
                    Adt1::C0(_, h328, _, _, _, _, _, _, _, _, _) => {
                        h328
                    }
                }
            };
            nat_mul(a331, a332)?
        };
        Ok(Adt3::C7(f52(v2, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h333, _, _, _, _) => {
                h333
            }
        })?, f53(v2, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h334, _, _, _, _) => {
                h334
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, h335, _, _, _) => {
                h335
            }
        })?, f52(v2, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h336, _, _) => {
                h336
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f11(v0: u64, v1: Adt2) -> R<Adt3> {
    let a338 = v0;
    let a339 = match v1.clone() {
        Adt2::C0(h337, _, _, _, _, _, _, _, _, _, _, _) => {
            h337
        }
    };
    if nat_lt(a338, a339) {
        let v2: Adt1 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h340, _, _) => {
                h340
            }
        };
        let v3: u64 = {
            let a342 = v0;
            let a343 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, _, h341) => {
                    h341
                }
            };
            nat_mul(a342, a343)?
        };
        let v4: u64 = {
            let a345 = v3;
            let a346 = match v2.clone() {
                Adt1::C0(_, h344, _, _, _, _, _, _, _, _, _) => {
                    h344
                }
            };
            let a347 = 0u64;
            nat_quot(a345, a346, a347)
        };
        let v5: u64 = {
            let a349 = v4;
            let a350 = match v2.clone() {
                Adt1::C0(_, _, h348, _, _, _, _, _, _, _, _) => {
                    h348
                }
            };
            let a351 = 0u64;
            nat_quot(a349, a350, a351)
        };
        let v6: u64 = {
            let a357 = {
                let a353 = v3;
                let a354 = match v2.clone() {
                    Adt1::C0(_, h352, _, _, _, _, _, _, _, _, _) => {
                        h352
                    }
                };
                let a355 = 0u64;
                nat_rem(a353, a354, a355)
            };
            let a358 = f28(match v2.clone() {
                Adt1::C0(h356, _, _, _, _, _, _, _, _, _, _) => {
                    h356
                }
            });
            let a359 = 0u64;
            nat_quot(a357, a358, a359)
        };
        Ok(Adt3::C6(f52(v3, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, h360, _, _, _, _) => {
                h360
            }
        })?, f54(v6, {
            let a362 = v4;
            let a363 = match v2.clone() {
                Adt1::C0(_, _, h361, _, _, _, _, _, _, _, _) => {
                    h361
                }
            };
            let a364 = 0u64;
            nat_rem(a362, a363, a364)
        }, {
            let a366 = v5;
            let a367 = match v2.clone() {
                Adt1::C0(_, _, _, h365, _, _, _, _, _, _, _) => {
                    h365
                }
            };
            let a368 = 0u64;
            nat_rem(a366, a367, a368)
        }, {
            let a370 = v5;
            let a371 = match v2.clone() {
                Adt1::C0(_, _, _, h369, _, _, _, _, _, _, _) => {
                    h369
                }
            };
            let a372 = 0u64;
            nat_quot(a370, a371, a372)
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, h373, _, _, _, _, _) => {
                h373
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h374, _, _, _, _) => {
                h374
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h375, _, _, _) => {
                h375
            }
        }, match v2 {
            Adt1::C0(_, _, _, _, _, _, _, _, h376, _, _) => {
                h376
            }
        })?))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f12(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h377, _, _, _, _) => {
            h377
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h378, _, _) => {
            h378
        }
    };
    let a380 = v0;
    let a381 = match v1.clone() {
        Adt2::C0(h379, _, _, _, _, _, _, _, _, _, _, _) => {
            h379
        }
    };
    if nat_lt(a380, a381) {
        Ok(Adt3::C5({
            let a383 = v0;
            let a384 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, _, h382, _, _, _, _) => {
                    h382
                }
            };
            nat_mul(a383, a384)?
        }, {
            let a386 = v0;
            let a387 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h385, _, _, _, _) => {
                    h385
                }
            };
            nat_mul(a386, a387)?
        }, match v2 {
            Adt1::C0(_, h388, _, _, _, _, _, _, _, _, _) => {
                h388
            }
        }, match v3 {
            Adt1::C0(_, h389, _, _, _, _, _, _, _, _, _) => {
                h389
            }
        }, {
            let a391 = match v1 {
                Adt2::C0(_, _, _, _, h390, _, _, _, _, _, _, _) => {
                    h390
                }
            };
            let a392 = 1u64;
            nat_eq(a391, a392)
        }))
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f13(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let a394 = v0;
    let a395 = match v1.clone() {
        Adt2::C0(h393, _, _, _, _, _, _, _, _, _, _, _) => {
            h393
        }
    };
    if nat_lt(a394, a395) {
        let v3: u64 = {
            let a397 = 2u64;
            let a398 = f39(match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h396, _, _, _) => {
                    h396
                }
            })?;
            nat_mul(a397, a398)?
        };
        let a399 = v0;
        let a400 = v3;
        if nat_lt(a399, a400) {
            f55(v0, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h401, _, _, _) => {
                    h401
                }
            }, {
                let m404 = match v1 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h403, _, _) => {
                        h403
                    }
                };
                match m404 {
                    Adt1::C0(_, _, h402, _, _, _, _, _, _, _, _) => {
                        h402
                    }
                }
            }, v2)
        } else {
            f56({
                let a405 = v0;
                let a406 = v3;
                nat_sub(a405, a406)
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h407, _, _, _, _) => {
                    h407
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h408, _, _, _) => {
                    h408
                }
            }, match v1 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h409, _, _) => {
                    h409
                }
            }, v2)
        }
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f14(v0: u64) -> bool {
    let a410 = v0;
    let a411 = 3u64;
    if nat_eq(a410, a411) {
        true
    } else {
        let a412 = v0;
        let a413 = 6u64;
        nat_eq(a412, a413)
    }
}

pub fn f15(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, h414, _, _, _, _, _) => {
            h414
        }
    };
    let a416 = v0;
    let a417 = match v1.clone() {
        Adt2::C0(h415, _, _, _, _, _, _, _, _, _, _, _) => {
            h415
        }
    };
    if nat_lt(a416, a417) {
        let a418 = v0;
        let a419 = 0u64;
        if nat_eq(a418, a419) {
            Ok(Adt3::C3(1u64, 6u64, 0u64, 0u64, 2u64, v3, 2u64, 0u64, 0u64, 3u64, 4u64, 4u64))
        } else {
            let v4: u64 = {
                let a420 = v0;
                let a421 = 1u64;
                nat_sub(a420, a421)
            };
            let v5: u64 = {
                let a423 = v4;
                let a424 = f51(match v1.clone() {
                    Adt2::C0(_, _, _, _, h422, _, _, _, _, _, _, _) => {
                        h422
                    }
                });
                let a425 = 0u64;
                nat_rem(a423, a424, a425)
            };
            let v6: u64 = {
                let a430 = 4u64;
                let a431 = {
                    let a427 = v4;
                    let a428 = f51(match v1.clone() {
                        Adt2::C0(_, _, _, _, h426, _, _, _, _, _, _, _) => {
                            h426
                        }
                    });
                    let a429 = 0u64;
                    nat_quot(a427, a428, a429)
                };
                nat_mul(a430, a431)?
            };
            let a432 = v5;
            let a433 = 2u64;
            if nat_lt(a432, a433) {
                f16({
                    let a434 = v6;
                    let a435 = v5;
                    nat_add(a434, a435)?
                }, v1)
            } else {
                let a436 = v5;
                let a437 = 2u64;
                if nat_eq(a436, a437) {
                    Ok(f57(0u64, 2u64, v3))
                } else {
                    let a438 = v5;
                    let a439 = 3u64;
                    if nat_eq(a438, a439) {
                        Ok(f58(0u64, 2u64, v3, {
                            let a440 = v2;
                            let a441 = 2u64;
                            nat_eq(a440, a441)
                        }))
                    } else {
                        let a442 = v5;
                        let a443 = 4u64;
                        if nat_eq(a442, a443) {
                            Ok(f57(1u64, 1u64, 3u64))
                        } else {
                            let a444 = v5;
                            let a445 = 5u64;
                            if nat_eq(a444, a445) {
                                Ok(f58(1u64, 1u64, 3u64, {
                                    let a446 = v2;
                                    let a447 = 0u64;
                                    nat_eq(a446, a447)
                                }))
                            } else {
                                let a448 = v5;
                                let a449 = 6u64;
                                if nat_eq(a448, a449) {
                                    Ok(f57(1u64, 2u64, v3))
                                } else {
                                    let a450 = v5;
                                    let a451 = 7u64;
                                    if nat_eq(a450, a451) {
                                        Ok(f58(1u64, 2u64, v3, {
                                            let a452 = v2;
                                            let a453 = 2u64;
                                            nat_eq(a452, a453)
                                        }))
                                    } else {
                                        let a455 = match v1.clone() {
                                            Adt2::C0(_, _, _, _, h454, _, _, _, _, _, _, _) => {
                                                h454
                                            }
                                        };
                                        let a456 = 3u64;
                                        if nat_eq(a455, a456) {
                                            let a457 = v5;
                                            let a458 = 15u64;
                                            if nat_eq(a457, a458) {
                                                f16({
                                                    let a459 = v6;
                                                    let a460 = 3u64;
                                                    nat_add(a459, a460)?
                                                }, v1)
                                            } else {
                                                Ok(f59(v5, match v1 {
                                                    Adt2::C0(_, _, _, _, _, h461, _, _, _, _, _, _) => {
                                                        h461
                                                    }
                                                }))
                                            }
                                        } else {
                                            let a462 = v5;
                                            let a463 = 8u64;
                                            if nat_eq(a462, a463) {
                                                Ok(Adt3::C4(2u64, {
                                                    let m466 = match v1 {
                                                        Adt2::C0(_, _, _, _, _, _, _, _, _, h465, _, _) => {
                                                            h465
                                                        }
                                                    };
                                                    match m466 {
                                                        Adt1::C0(h464, _, _, _, _, _, _, _, _, _, _) => {
                                                            h464
                                                        }
                                                    }
                                                }, 0u64, 1u64, 0u64))
                                            } else {
                                                f16({
                                                    let a467 = v6;
                                                    let a468 = 3u64;
                                                    nat_add(a467, a468)?
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
    let a470 = v0;
    let a471 = match v1.clone() {
        Adt2::C0(h469, _, _, _, _, _, _, _, _, _, _, _) => {
            h469
        }
    };
    if nat_lt(a470, a471) {
        f60({
            let a472 = v0;
            let a473 = 4u64;
            let a474 = 0u64;
            nat_rem(a472, a473, a474)
        }, {
            let a475 = v0;
            let a476 = 4u64;
            let a477 = 0u64;
            nat_quot(a475, a476, a477)
        }, v1)
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f17(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h478, _, _, _, _) => {
            h478
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h479, _, _, _) => {
            h479
        }
    };
    let v5: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h480, _, _) => {
            h480
        }
    };
    let v6: u64 = {
        let a482 = match v1.clone() {
            Adt2::C0(_, h481, _, _, _, _, _, _, _, _, _, _) => {
                h481
            }
        };
        let a483 = 3u64;
        if nat_eq(a482, a483) {
            0u64
        } else {
            let a484 = f39(v3.clone())?;
            let a485 = 2u64;
            nat_mul(a484, a485)?
        }
    };
    let a487 = match v1.clone() {
        Adt2::C0(h486, _, _, _, _, _, _, _, _, _, _, _) => {
            h486
        }
    };
    let a488 = v0;
    if nat_le(a487, a488) {
        Ok(Adt3::C1)
    } else {
        let a489 = v0;
        let a490 = v6;
        if nat_lt(a489, a490) {
            let v7: u64 = {
                let a491 = v0;
                let a492 = 2u64;
                let a493 = 0u64;
                nat_quot(a491, a492, a493)
            };
            let v8: u64 = {
                let a498 = 3u64;
                let a499 = {
                    let a495 = f52(v7, v5.clone())?;
                    let a496 = f34(match v5 {
                        Adt1::C0(h494, _, _, _, _, _, _, _, _, _, _) => {
                            h494
                        }
                    });
                    let a497 = 0u64;
                    nat_quot(a495, a496, a497)
                };
                nat_add(a498, a499)?
            };
            let a503 = {
                let a500 = v0;
                let a501 = 2u64;
                let a502 = 0u64;
                nat_rem(a500, a501, a502)
            };
            let a504 = 0u64;
            if nat_eq(a503, a504) {
                Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, v8, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
            } else {
                let a505 = v2;
                let a506 = 0u64;
                if nat_eq(a505, a506) {
                    Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 2u64, 1u64, 2u64, 0u64, 0u64, v8, 8u64, 8u64))
                } else {
                    Ok(Adt3::C1)
                }
            }
        } else {
            let v9: u64 = {
                let a507 = v0;
                let a508 = v6;
                nat_sub(a507, a508)
            };
            if f61(match v3.clone() {
                Adt1::C0(h509, _, _, _, _, _, _, _, _, _, _) => {
                    h509
                }
            }) {
                f62(v9, v1)
            } else {
                let v10: u64 = f63(match v3.clone() {
                    Adt1::C0(h510, _, _, _, _, _, _, _, _, _, _) => {
                        h510
                    }
                }, match v4.clone() {
                    Adt1::C0(h511, _, _, _, _, _, _, _, _, _, _) => {
                        h511
                    }
                }, match v5.clone() {
                    Adt1::C0(h512, _, _, _, _, _, _, _, _, _, _) => {
                        h512
                    }
                })?;
                let v11: u64 = {
                    let a513 = v9;
                    let a514 = v10;
                    let a515 = 0u64;
                    nat_quot(a513, a514, a515)
                };
                let v12: u64 = {
                    let a516 = v9;
                    let a517 = v10;
                    let a518 = 0u64;
                    nat_rem(a516, a517, a518)
                };
                let v13: u64 = {
                    let a520 = match v3.clone() {
                        Adt1::C0(h519, _, _, _, _, _, _, _, _, _, _) => {
                            h519
                        }
                    };
                    let a521 = 0u64;
                    if nat_eq(a520, a521) {
                        0u64
                    } else {
                        1u64
                    }
                };
                let v14: u64 = {
                    let a523 = match v4.clone() {
                        Adt1::C0(h522, _, _, _, _, _, _, _, _, _, _) => {
                            h522
                        }
                    };
                    let a524 = 0u64;
                    if nat_eq(a523, a524) {
                        0u64
                    } else {
                        1u64
                    }
                };
                let a525 = v12;
                let a526 = v13;
                if nat_lt(a525, a526) {
                    Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                        Adt1::C0(h527, _, _, _, _, _, _, _, _, _, _) => {
                            h527
                        }
                    }, 0u64, 0u64, f52(v11, v3.clone())?, 2u64, 0u64, 0u64, 0u64, f34(match v3 {
                        Adt1::C0(h528, _, _, _, _, _, _, _, _, _, _) => {
                            h528
                        }
                    }), 4u64))
                } else {
                    let a531 = v12;
                    let a532 = {
                        let a529 = v13;
                        let a530 = v14;
                        nat_add(a529, a530)?
                    };
                    if nat_lt(a531, a532) {
                        Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                            Adt1::C0(h533, _, _, _, _, _, _, _, _, _, _) => {
                                h533
                            }
                        }, 0u64, 4u64, f53(v11, v3, v4.clone())?, 2u64, 0u64, 0u64, 1u64, f34(match v4 {
                            Adt1::C0(h534, _, _, _, _, _, _, _, _, _, _) => {
                                h534
                            }
                        }), 4u64))
                    } else {
                        let a537 = v12;
                        let a538 = {
                            let a535 = v13;
                            let a536 = v14;
                            nat_add(a535, a536)?
                        };
                        if nat_eq(a537, a538) {
                            Ok(Adt3::C3(1u64, match v1 {
                                Adt2::C0(_, _, _, h539, _, _, _, _, _, _, _, _) => {
                                    h539
                                }
                            }, 0u64, 0u64, {
                                let a540 = v13;
                                let a541 = 0u64;
                                if nat_eq(a540, a541) {
                                    0u64
                                } else {
                                    1u64
                                }
                            }, {
                                let a542 = v13;
                                let a543 = 0u64;
                                if nat_eq(a542, a543) {
                                    f52(v11, v3.clone())?
                                } else {
                                    0u64
                                }
                            }, {
                                let a544 = v14;
                                let a545 = 0u64;
                                if nat_eq(a544, a545) {
                                    4u64
                                } else {
                                    1u64
                                }
                            }, {
                                let a546 = v14;
                                let a547 = 0u64;
                                if nat_eq(a546, a547) {
                                    f53(v11, v3, v4)?
                                } else {
                                    1u64
                                }
                            }, {
                                let a549 = match v5.clone() {
                                    Adt1::C0(h548, _, _, _, _, _, _, _, _, _, _) => {
                                        h548
                                    }
                                };
                                let a550 = 0u64;
                                if nat_eq(a549, a550) {
                                    3u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a552 = match v5.clone() {
                                    Adt1::C0(h551, _, _, _, _, _, _, _, _, _, _) => {
                                        h551
                                    }
                                };
                                let a553 = 0u64;
                                if nat_eq(a552, a553) {
                                    f52(v11, v5)?
                                } else {
                                    0u64
                                }
                            }, 4u64, 4u64))
                        } else {
                            Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                                Adt1::C0(h554, _, _, _, _, _, _, _, _, _, _) => {
                                    h554
                                }
                            }, 1u64, 0u64, 2u64, 0u64, 3u64, f52(v11, v5.clone())?, 4u64, f34(match v5 {
                                Adt1::C0(h555, _, _, _, _, _, _, _, _, _, _) => {
                                    h555
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
    let a556 = 6u64;
    let a557 = v0;
    if if if if nat_le(a556, a557) {
        let a558 = v0;
        let a559 = 23u64;
        nat_le(a558, a559)
    } else {
        false
    } {
        true
    } else {
        let a560 = v0;
        let a561 = 2u64;
        if nat_eq(a560, a561) {
            true
        } else {
            let a562 = v0;
            let a563 = 3u64;
            nat_eq(a562, a563)
        }
    } {
        true
    } else {
        let a564 = v0;
        let a565 = 29u64;
        nat_eq(a564, a565)
    } {
        true
    } else {
        let a566 = 34u64;
        let a567 = v0;
        if if nat_le(a566, a567) {
            let a568 = v0;
            let a569 = 35u64;
            nat_le(a568, a569)
        } else {
            false
        } {
            true
        } else {
            let a570 = 39u64;
            let a571 = v0;
            if nat_le(a570, a571) {
                let a572 = v0;
                let a573 = 42u64;
                nat_le(a572, a573)
            } else {
                false
            }
        }
    }
}

pub fn f19(v0: u64, v1: u64, v2: u64) -> bool {
    let a574 = v0;
    let a575 = 0u64;
    if if if nat_eq(a574, a575) {
        let a576 = v1;
        let a577 = 0u64;
        nat_eq(a576, a577)
    } else {
        false
    } {
        let a578 = v2;
        let a579 = 0u64;
        nat_eq(a578, a579)
    } else {
        false
    } {
        true
    } else {
        let a580 = v0;
        let a581 = 1u64;
        if if nat_eq(a580, a581) {
            true
        } else {
            let a582 = v0;
            let a583 = 30u64;
            nat_eq(a582, a583)
        } {
            let a584 = v1;
            let a585 = v0;
            if if nat_eq(a584, a585) {
                let a586 = v2;
                let a587 = v0;
                nat_eq(a586, a587)
            } else {
                false
            } {
                true
            } else {
                let a588 = v1;
                let a589 = 0u64;
                if nat_eq(a588, a589) {
                    let a590 = v2;
                    let a591 = v0;
                    if nat_eq(a590, a591) {
                        true
                    } else {
                        let a592 = v2;
                        let a593 = 0u64;
                        nat_eq(a592, a593)
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
    let a594 = v1;
    let a595 = 27u64;
    if if if nat_eq(a594, a595) {
        true
    } else {
        let a596 = v1;
        let a597 = 28u64;
        nat_eq(a596, a597)
    } {
        let a598 = v1;
        let a599 = v2;
        nat_eq(a598, a599)
    } else {
        false
    } {
        let a600 = v0;
        let a601 = 3u64;
        nat_eq(a600, a601)
    } else {
        f22(v1)
    }
}

pub fn f21(v0: u64) -> u64 {
    let a602 = v0;
    let a603 = 0u64;
    if nat_eq(a602, a603) {
        4u64
    } else {
        let a604 = v0;
        let a605 = 1u64;
        if if nat_eq(a604, a605) {
            true
        } else {
            let a606 = v0;
            let a607 = 30u64;
            nat_eq(a606, a607)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f22(v0: u64) -> bool {
    let a608 = 0u64;
    let a609 = f64(v0);
    if nat_lt(a608, a609) {
        true
    } else {
        let a610 = 24u64;
        let a611 = v0;
        if nat_le(a610, a611) {
            let a612 = v0;
            let a613 = 28u64;
            nat_le(a612, a613)
        } else {
            false
        }
    }
}

pub fn f23(v0: Adt1, v1: Adt1) -> bool {
    if f31(v1.clone()) {
        f31(v0)
    } else {
        let a619 = {
            let a616 = match v0.clone() {
                Adt1::C0(_, h614, _, _, _, _, _, _, _, _, _) => {
                    h614
                }
            };
            let a617 = match v1.clone() {
                Adt1::C0(_, h615, _, _, _, _, _, _, _, _, _) => {
                    h615
                }
            };
            let a618 = 1u64;
            nat_rem(a616, a617, a618)
        };
        let a620 = 0u64;
        if if if nat_eq(a619, a620) {
            let a626 = {
                let a623 = match v0.clone() {
                    Adt1::C0(_, _, h621, _, _, _, _, _, _, _, _) => {
                        h621
                    }
                };
                let a624 = match v1.clone() {
                    Adt1::C0(_, _, h622, _, _, _, _, _, _, _, _) => {
                        h622
                    }
                };
                let a625 = 1u64;
                nat_rem(a623, a624, a625)
            };
            let a627 = 0u64;
            nat_eq(a626, a627)
        } else {
            false
        } {
            let a633 = {
                let a630 = match v0.clone() {
                    Adt1::C0(_, _, _, h628, _, _, _, _, _, _, _) => {
                        h628
                    }
                };
                let a631 = match v1.clone() {
                    Adt1::C0(_, _, _, h629, _, _, _, _, _, _, _) => {
                        h629
                    }
                };
                let a632 = 1u64;
                nat_rem(a630, a631, a632)
            };
            let a634 = 0u64;
            nat_eq(a633, a634)
        } else {
            false
        } {
            let a640 = {
                let a637 = match v0 {
                    Adt1::C0(_, _, _, _, h635, _, _, _, _, _, _) => {
                        h635
                    }
                };
                let a638 = match v1 {
                    Adt1::C0(_, _, _, _, h636, _, _, _, _, _, _) => {
                        h636
                    }
                };
                let a639 = 1u64;
                nat_rem(a637, a638, a639)
            };
            let a641 = 0u64;
            nat_eq(a640, a641)
        } else {
            false
        }
    }
}

pub fn f24(v0: Adt1, v1: Adt1) -> bool {
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

pub fn f25(v0: u64, v1: u64, v2: Adt1, v3: u64) -> bool {
    let v4: u64 = match v2.clone() {
        Adt1::C0(h658, _, _, _, _, _, _, _, _, _, _) => {
            h658
        }
    };
    let a659 = v1;
    let a660 = v3;
    if if nat_eq(a659, a660) {
        true
    } else {
        let a661 = v0;
        let a662 = 0u64;
        if nat_eq(a661, a662) {
            let a663 = v4;
            let a664 = 0u64;
            if if nat_eq(a663, a664) {
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
            let a666 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, h665, _, _, _, _, _) => {
                    h665
                }
            };
            let a667 = f27(v4);
            nat_eq(a666, a667)
        } else {
            false
        } {
            let a672 = {
                let a669 = match v2 {
                    Adt1::C0(_, h668, _, _, _, _, _, _, _, _, _) => {
                        h668
                    }
                };
                let a670 = f28(v4);
                let a671 = 1u64;
                nat_rem(a669, a670, a671)
            };
            let a673 = 0u64;
            nat_eq(a672, a673)
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
        let a674 = v0;
        let a675 = v1;
        if if nat_eq(a674, a675) {
            f22(v0)
        } else {
            false
        } {
            true
        } else {
            let a676 = f21(v0);
            let a677 = 2u64;
            if nat_eq(a676, a677) {
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
    let a678 = 24u64;
    let a679 = v0;
    if if nat_le(a678, a679) {
        let a680 = v0;
        let a681 = 28u64;
        nat_le(a680, a681)
    } else {
        false
    } {
        let m682 = f65(v0);
        match m682 {
            Adt4::C0 => {
                0u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, _, h683) => {
                        h683
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
        let m684 = f65(v0);
        match m684 {
            Adt4::C0 => {
                1u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, h685, _) => {
                        h685
                    }
                }
            }
        }
    } else {
        1u64
    }
}

pub fn f29(v0: Adt1, v1: Adt1) -> R<bool> {
    if f49(v0.clone(), v1.clone())? {
        Ok(true)
    } else {
        Ok(f66(v0, v1))
    }
}

pub fn f30(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f67(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f67(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f67(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f67(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f67(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f67(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f31(v0: Adt1) -> bool {
    f68(match v0.clone() {
        Adt1::C0(_, h686, _, _, _, _, _, _, _, _, _) => {
            h686
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h687, _, _, _, _, _, _, _, _) => {
            h687
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h688, _, _, _, _, _, _, _) => {
            h688
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, h689, _, _, _, _, _, _) => {
            h689
        }
    })
}

pub fn f32(v0: u64) -> Option<u64> {
    let a690 = v0;
    let a691 = 16u64;
    if if if nat_eq(a690, a691) {
        true
    } else {
        let a692 = v0;
        let a693 = 17u64;
        nat_eq(a692, a693)
    } {
        true
    } else {
        let a694 = v0;
        let a695 = 19u64;
        nat_eq(a694, a695)
    } {
        Some(1065353216u64)
    } else {
        None::<u64>
    }
}

pub fn f33(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<bool> {
    let a697 = match v1.clone() {
        Adt1::C0(h696, _, _, _, _, _, _, _, _, _, _) => {
            h696
        }
    };
    let a698 = 43u64;
    if nat_eq(a697, a698) {
        let a699 = v0;
        let a700 = 0u64;
        if if if if if if nat_eq(a699, a700) {
            let a702 = match v2.clone() {
                Adt1::C0(h701, _, _, _, _, _, _, _, _, _, _) => {
                    h701
                }
            };
            let a703 = 0u64;
            nat_eq(a702, a703)
        } else {
            false
        } {
            let a705 = match v3.clone() {
                Adt1::C0(h704, _, _, _, _, _, _, _, _, _, _) => {
                    h704
                }
            };
            let a706 = 0u64;
            if nat_eq(a705, a706) {
                true
            } else {
                let a708 = match v3.clone() {
                    Adt1::C0(h707, _, _, _, _, _, _, _, _, _, _) => {
                        h707
                    }
                };
                let a709 = 43u64;
                nat_eq(a708, a709)
            }
        } else {
            false
        } {
            let a711 = match v3.clone() {
                Adt1::C0(h710, _, _, _, _, _, _, _, _, _, _) => {
                    h710
                }
            };
            let a712 = 43u64;
            if nat_eq(a711, a712) {
                f23(v1.clone(), v2.clone())
            } else {
                f24(v1.clone(), v2.clone())
            }
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
        if f23(v1.clone(), v2.clone()) {
            if f19(match v1.clone() {
                Adt1::C0(h737, _, _, _, _, _, _, _, _, _, _) => {
                    h737
                }
            }, match v2.clone() {
                Adt1::C0(h738, _, _, _, _, _, _, _, _, _, _) => {
                    h738
                }
            }, match v3.clone() {
                Adt1::C0(h739, _, _, _, _, _, _, _, _, _, _) => {
                    h739
                }
            }) {
                Ok(true)
            } else {
                let a740 = v0;
                let a741 = 3u64;
                if if if if nat_lt(a740, a741) {
                    f61(match v1.clone() {
                        Adt1::C0(h742, _, _, _, _, _, _, _, _, _, _) => {
                            h742
                        }
                    })
                } else {
                    false
                } {
                    let a745 = match v1.clone() {
                        Adt1::C0(h743, _, _, _, _, _, _, _, _, _, _) => {
                            h743
                        }
                    };
                    let a746 = match v2.clone() {
                        Adt1::C0(h744, _, _, _, _, _, _, _, _, _, _) => {
                            h744
                        }
                    };
                    nat_eq(a745, a746)
                } else {
                    false
                } {
                    let a749 = match v1 {
                        Adt1::C0(h747, _, _, _, _, _, _, _, _, _, _) => {
                            h747
                        }
                    };
                    let a750 = match v3 {
                        Adt1::C0(h748, _, _, _, _, _, _, _, _, _, _) => {
                            h748
                        }
                    };
                    nat_eq(a749, a750)
                } else {
                    false
                } {
                    let a752 = match v2 {
                        Adt1::C0(_, _, _, _, _, h751, _, _, _, _, _) => {
                            h751
                        }
                    };
                    let a753 = 8u64;
                    Ok(nat_eq(a752, a753))
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
    if f61(v0) {
        8u64
    } else {
        f64(v0)
    }
}

pub fn f35(v0: Adt1) -> bool {
    let v1: u64 = f34(match v0.clone() {
        Adt1::C0(h754, _, _, _, _, _, _, _, _, _, _) => {
            h754
        }
    });
    let a759 = {
        let a756 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h755) => {
                h755
            }
        };
        let a757 = v1;
        let a758 = 1u64;
        nat_rem(a756, a757, a758)
    };
    let a760 = 0u64;
    if if nat_eq(a759, a760) {
        f69(match v0.clone() {
            Adt1::C0(_, h761, _, _, _, _, _, _, _, _, _) => {
                h761
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, h762, _, _, _, _, _) => {
                h762
            }
        }, v1)
    } else {
        false
    } {
        f70(match v0.clone() {
            Adt1::C0(_, _, h763, _, _, _, _, _, _, _, _) => {
                h763
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, h764, _, _, _, _, _, _, _) => {
                h764
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, h765, _, _, _, _, _, _) => {
                h765
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h766, _, _, _, _) => {
                h766
            }
        }, match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h767, _, _, _) => {
                h767
            }
        }, match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h768, _, _) => {
                h768
            }
        }, v1)
    } else {
        false
    }
}

pub fn f36(v0: Adt1, v1: Adt1) -> R<bool> {
    let a776 = {
        let a774 = match v0.clone() {
            Adt1::C0(_, _, h769, _, _, _, _, _, _, _, _) => {
                h769
            }
        };
        let a775 = {
            let a772 = match v0.clone() {
                Adt1::C0(_, _, _, h770, _, _, _, _, _, _, _) => {
                    h770
                }
            };
            let a773 = match v0.clone() {
                Adt1::C0(_, _, _, _, h771, _, _, _, _, _, _) => {
                    h771
                }
            };
            nat_mul(a772, a773)?
        };
        nat_mul(a774, a775)?
    };
    let a777 = 9223372036854775807u64;
    if nat_le(a776, a777) {
        if f44(v1.clone()) {
            let a779 = match v1 {
                Adt1::C0(_, h778, _, _, _, _, _, _, _, _, _) => {
                    h778
                }
            };
            let a780 = 2147483647u64;
            Ok(nat_le(a779, a780))
        } else {
            let a782 = match v0 {
                Adt1::C0(_, h781, _, _, _, _, _, _, _, _, _) => {
                    h781
                }
            };
            let a783 = 2147483647u64;
            Ok(nat_le(a782, a783))
        }
    } else {
        Ok(false)
    }
}

pub fn f37(v0: Adt1) -> R<bool> {
    f30({
        let a786 = match v0.clone() {
            Adt1::C0(_, h784, _, _, _, _, _, _, _, _, _) => {
                h784
            }
        };
        let a787 = f34(match v0.clone() {
            Adt1::C0(h785, _, _, _, _, _, _, _, _, _, _) => {
                h785
            }
        });
        nat_mul(a786, a787)?
    }, match v0.clone() {
        Adt1::C0(_, _, h788, _, _, _, _, _, _, _, _) => {
            h788
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h789, _, _, _, _, _, _, _) => {
            h789
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h790, _, _, _, _, _, _) => {
            h790
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, h791, _, _, _, _) => {
            h791
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h792, _, _, _) => {
            h792
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, _, _, _, h793, _, _) => {
            h793
        }
    })
}

pub fn f38(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a800 = {
        let a794 = v0;
        let a795 = v2;
        nat_mul(a794, a795)?
    };
    let a801 = {
        let a796 = v1;
        let a797 = 0u64;
        if nat_eq(a796, a797) {
            0u64
        } else {
            let a798 = v0;
            let a799 = 2u64;
            nat_mul(a798, a799)?
        }
    };
    nat_add(a800, a801)
}

pub fn f39(v0: Adt1) -> R<u64> {
    let a810 = {
        let a804 = match v0.clone() {
            Adt1::C0(_, h802, _, _, _, _, _, _, _, _, _) => {
                h802
            }
        };
        let a805 = match v0.clone() {
            Adt1::C0(_, _, h803, _, _, _, _, _, _, _, _) => {
                h803
            }
        };
        nat_mul(a804, a805)?
    };
    let a811 = {
        let a808 = match v0.clone() {
            Adt1::C0(_, _, _, h806, _, _, _, _, _, _, _) => {
                h806
            }
        };
        let a809 = match v0 {
            Adt1::C0(_, _, _, _, h807, _, _, _, _, _, _) => {
                h807
            }
        };
        nat_mul(a808, a809)?
    };
    nat_mul(a810, a811)
}

pub fn f40(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    if f61(v1) {
        Ok(f71(v0, v1))
    } else {
        f63(v1, v2, v3)
    }
}

pub fn f41(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f24(v0.clone(), v1.clone()) {
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

pub fn f42(v0: u64) -> u64 {
    if f18(v0) {
        let m842 = f65(v0);
        match m842 {
            Adt4::C0 => {
                0u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, _, h843) => {
                        h843
                    }
                }
            }
        }
    } else {
        let a844 = v0;
        let a845 = 26u64;
        if nat_eq(a844, a845) {
            4u64
        } else {
            f64(v0)
        }
    }
}

pub fn f43(v0: u64) -> u64 {
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

pub fn f44(v0: Adt1) -> bool {
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
        let a856 = f64(match v0 {
            Adt1::C0(h854, _, _, _, _, _, _, _, _, _, _) => {
                h854
            }
        });
        nat_eq(a855, a856)
    }
}

pub fn f45(v0: Adt1, v1: u64) -> R<u64> {
    let a862 = {
        let a858 = match v0.clone() {
            Adt1::C0(_, h857, _, _, _, _, _, _, _, _, _) => {
                h857
            }
        };
        let a859 = f28(v1);
        let a860 = 0u64;
        nat_quot(a858, a859, a860)
    };
    let a863 = f72(match v0 {
        Adt1::C0(h861, _, _, _, _, _, _, _, _, _, _) => {
            h861
        }
    }, v1);
    nat_mul(a862, a863)
}

pub fn f46(v0: u64) -> u64 {
    if if f18(v0) {
        true
    } else {
        let a864 = v0;
        let a865 = 26u64;
        nat_eq(a864, a865)
    } {
        f28(v0)
    } else {
        0u64
    }
}

pub fn f47(v0: Adt1, v1: Adt1) -> u64 {
    let v2: u64 = match v0.clone() {
        Adt1::C0(h866, _, _, _, _, _, _, _, _, _, _) => {
            h866
        }
    };
    let v3: u64 = match v1.clone() {
        Adt1::C0(h867, _, _, _, _, _, _, _, _, _, _) => {
            h867
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
            let a872 = {
                let a869 = match v0.clone() {
                    Adt1::C0(_, h868, _, _, _, _, _, _, _, _, _) => {
                        h868
                    }
                };
                let a870 = f28(v2);
                let a871 = 1u64;
                nat_rem(a869, a870, a871)
            };
            let a873 = 0u64;
            if nat_eq(a872, a873) {
                let a878 = {
                    let a875 = match v1.clone() {
                        Adt1::C0(_, h874, _, _, _, _, _, _, _, _, _) => {
                            h874
                        }
                    };
                    let a876 = f28(v3);
                    let a877 = 1u64;
                    nat_rem(a875, a876, a877)
                };
                let a879 = 0u64;
                nat_eq(a878, a879)
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    } {
        let a881 = match v0.clone() {
            Adt1::C0(_, h880, _, _, _, _, _, _, _, _, _) => {
                h880
            }
        };
        let a882 = 0u64;
        if nat_eq(a881, a882) {
            f28(v3)
        } else {
            match v0 {
                Adt1::C0(_, h883, _, _, _, _, _, _, _, _, _) => {
                    h883
                }
            }
        }
    } else {
        if if if f18(v3) {
            let a884 = 0u64;
            let a885 = f21(v2);
            nat_lt(a884, a885)
        } else {
            false
        } {
            let a890 = {
                let a887 = match v0 {
                    Adt1::C0(_, h886, _, _, _, _, _, _, _, _, _) => {
                        h886
                    }
                };
                let a888 = f28(v3);
                let a889 = 1u64;
                nat_rem(a887, a888, a889)
            };
            let a891 = 0u64;
            if nat_eq(a890, a891) {
                let a896 = {
                    let a893 = match v1 {
                        Adt1::C0(_, h892, _, _, _, _, _, _, _, _, _) => {
                            h892
                        }
                    };
                    let a894 = f28(v3);
                    let a895 = 1u64;
                    nat_rem(a893, a894, a895)
                };
                let a897 = 0u64;
                nat_eq(a896, a897)
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

pub fn f48(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a899 = match v0.clone() {
            Adt1::C0(_, h898, _, _, _, _, _, _, _, _, _) => {
                h898
            }
        };
        let a900 = 4u64;
        nat_mul(a899, a900)?
    };
    let v2: u64 = {
        let a903 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h901, _, _, _, _) => {
                h901
            }
        };
        let a904 = match v0.clone() {
            Adt1::C0(_, _, h902, _, _, _, _, _, _, _, _) => {
                h902
            }
        };
        nat_mul(a903, a904)?
    };
    let v3: u64 = {
        let a906 = v2;
        let a907 = match v0.clone() {
            Adt1::C0(_, _, _, h905, _, _, _, _, _, _, _) => {
                h905
            }
        };
        nat_mul(a906, a907)?
    };
    let a909 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h908, _, _, _, _, _) => {
            h908
        }
    };
    let a910 = 4u64;
    if if if if nat_eq(a909, a910) {
        let a915 = {
            let a912 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h911) => {
                    h911
                }
            };
            let a913 = 4u64;
            let a914 = 1u64;
            nat_rem(a912, a913, a914)
        };
        let a916 = 0u64;
        nat_eq(a915, a916)
    } else {
        false
    } {
        let a924 = {
            let a922 = match v0.clone() {
                Adt1::C0(_, _, h917, _, _, _, _, _, _, _, _) => {
                    h917
                }
            };
            let a923 = {
                let a920 = match v0.clone() {
                    Adt1::C0(_, _, _, h918, _, _, _, _, _, _, _) => {
                        h918
                    }
                };
                let a921 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h919, _, _, _, _, _, _) => {
                        h919
                    }
                };
                nat_mul(a920, a921)?
            };
            nat_mul(a922, a923)?
        };
        let a925 = 1u64;
        if nat_le(a924, a925) {
            true
        } else {
            let a927 = v1;
            let a928 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h926, _, _, _, _) => {
                    h926
                }
            };
            if nat_le(a927, a928) {
                let a933 = {
                    let a930 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, h929, _, _, _, _) => {
                            h929
                        }
                    };
                    let a931 = 4u64;
                    let a932 = 1u64;
                    nat_rem(a930, a931, a932)
                };
                let a934 = 0u64;
                nat_eq(a933, a934)
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a936 = match v0.clone() {
            Adt1::C0(_, _, _, h935, _, _, _, _, _, _, _) => {
                h935
            }
        };
        let a937 = 1u64;
        if nat_le(a936, a937) {
            true
        } else {
            let a939 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h938, _, _, _) => {
                    h938
                }
            };
            let a940 = v2;
            nat_eq(a939, a940)
        }
    } else {
        false
    } {
        let a942 = match v0.clone() {
            Adt1::C0(_, _, _, _, h941, _, _, _, _, _, _) => {
                h941
            }
        };
        let a943 = 1u64;
        if nat_le(a942, a943) {
            Ok(true)
        } else {
            let a945 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, h944, _, _) => {
                    h944
                }
            };
            let a946 = v3;
            Ok(nat_eq(a945, a946))
        }
    } else {
        Ok(false)
    }
}

pub fn f49(v0: Adt1, v1: Adt1) -> R<bool> {
    let a948 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h947, _) => {
            h947
        }
    };
    let a949 = 0u64;
    if nat_eq(a948, a949) {
        Ok(true)
    } else {
        let a951 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h950, _) => {
                h950
            }
        };
        let a952 = 0u64;
        if nat_eq(a951, a952) {
            Ok(true)
        } else {
            let a954 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h953) => {
                    h953
                }
            };
            let a955 = 0u64;
            if nat_eq(a954, a955) {
                Ok(true)
            } else {
                let a957 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h956) => {
                        h956
                    }
                };
                let a958 = 0u64;
                if nat_eq(a957, a958) {
                    Ok(true)
                } else {
                    let a964 = {
                        let a961 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h959) => {
                                h959
                            }
                        };
                        let a962 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h960, _) => {
                                h960
                            }
                        };
                        nat_add(a961, a962)?
                    };
                    let a965 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h963) => {
                            h963
                        }
                    };
                    if nat_le(a964, a965) {
                        Ok(true)
                    } else {
                        let a971 = {
                            let a968 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h966) => {
                                    h966
                                }
                            };
                            let a969 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h967, _) => {
                                    h967
                                }
                            };
                            nat_add(a968, a969)?
                        };
                        let a972 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h970) => {
                                h970
                            }
                        };
                        Ok(nat_le(a971, a972))
                    }
                }
            }
        }
    }
}

pub fn f50(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a975 = match v0.clone() {
            Adt1::C0(_, _, h973, _, _, _, _, _, _, _, _) => {
                h973
            }
        };
        let a976 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h974, _, _, _, _) => {
                h974
            }
        };
        nat_mul(a975, a976)?
    };
    if if f44(v0.clone()) {
        let a978 = match v0.clone() {
            Adt1::C0(_, _, _, h977, _, _, _, _, _, _, _) => {
                h977
            }
        };
        let a979 = 1u64;
        if nat_eq(a978, a979) {
            true
        } else {
            let a981 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h980, _, _, _) => {
                    h980
                }
            };
            let a982 = v1;
            nat_eq(a981, a982)
        }
    } else {
        false
    } {
        let a984 = match v0.clone() {
            Adt1::C0(_, _, _, _, h983, _, _, _, _, _, _) => {
                h983
            }
        };
        let a985 = 1u64;
        if nat_eq(a984, a985) {
            Ok(true)
        } else {
            let a990 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h986, _, _) => {
                    h986
                }
            };
            let a991 = {
                let a988 = v1;
                let a989 = match v0 {
                    Adt1::C0(_, _, _, h987, _, _, _, _, _, _, _) => {
                        h987
                    }
                };
                nat_mul(a988, a989)?
            };
            Ok(nat_eq(a990, a991))
        }
    } else {
        Ok(false)
    }
}

pub fn f51(v0: u64) -> u64 {
    let a992 = v0;
    let a993 = 3u64;
    if nat_eq(a992, a993) {
        16u64
    } else {
        10u64
    }
}

pub fn f52(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a995 = v0;
        let a996 = match v1.clone() {
            Adt1::C0(_, h994, _, _, _, _, _, _, _, _, _) => {
                h994
            }
        };
        let a997 = 0u64;
        nat_quot(a995, a996, a997)
    };
    let v3: u64 = {
        let a999 = v2;
        let a1000 = match v1.clone() {
            Adt1::C0(_, _, h998, _, _, _, _, _, _, _, _) => {
                h998
            }
        };
        let a1001 = 0u64;
        nat_quot(a999, a1000, a1001)
    };
    f54({
        let a1003 = v0;
        let a1004 = match v1.clone() {
            Adt1::C0(_, h1002, _, _, _, _, _, _, _, _, _) => {
                h1002
            }
        };
        let a1005 = 0u64;
        nat_rem(a1003, a1004, a1005)
    }, {
        let a1007 = v2;
        let a1008 = match v1.clone() {
            Adt1::C0(_, _, h1006, _, _, _, _, _, _, _, _) => {
                h1006
            }
        };
        let a1009 = 0u64;
        nat_rem(a1007, a1008, a1009)
    }, {
        let a1011 = v3;
        let a1012 = match v1.clone() {
            Adt1::C0(_, _, _, h1010, _, _, _, _, _, _, _) => {
                h1010
            }
        };
        let a1013 = 0u64;
        nat_rem(a1011, a1012, a1013)
    }, {
        let a1015 = v3;
        let a1016 = match v1.clone() {
            Adt1::C0(_, _, _, h1014, _, _, _, _, _, _, _) => {
                h1014
            }
        };
        let a1017 = 0u64;
        nat_quot(a1015, a1016, a1017)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h1018, _, _, _, _, _) => {
            h1018
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h1019, _, _, _, _) => {
            h1019
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1020, _, _, _) => {
            h1020
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1021, _, _) => {
            h1021
        }
    })
}

pub fn f53(v0: u64, v1: Adt1, v2: Adt1) -> R<u64> {
    let v3: u64 = {
        let a1023 = v0;
        let a1024 = match v1.clone() {
            Adt1::C0(_, h1022, _, _, _, _, _, _, _, _, _) => {
                h1022
            }
        };
        let a1025 = 0u64;
        nat_quot(a1023, a1024, a1025)
    };
    let v4: u64 = {
        let a1027 = v3;
        let a1028 = match v1.clone() {
            Adt1::C0(_, _, h1026, _, _, _, _, _, _, _, _) => {
                h1026
            }
        };
        let a1029 = 0u64;
        nat_quot(a1027, a1028, a1029)
    };
    f54({
        let a1035 = {
            let a1031 = v0;
            let a1032 = match v1.clone() {
                Adt1::C0(_, h1030, _, _, _, _, _, _, _, _, _) => {
                    h1030
                }
            };
            let a1033 = 0u64;
            nat_rem(a1031, a1032, a1033)
        };
        let a1036 = match v2.clone() {
            Adt1::C0(_, h1034, _, _, _, _, _, _, _, _, _) => {
                h1034
            }
        };
        let a1037 = 0u64;
        nat_rem(a1035, a1036, a1037)
    }, {
        let a1043 = {
            let a1039 = v3;
            let a1040 = match v1.clone() {
                Adt1::C0(_, _, h1038, _, _, _, _, _, _, _, _) => {
                    h1038
                }
            };
            let a1041 = 0u64;
            nat_rem(a1039, a1040, a1041)
        };
        let a1044 = match v2.clone() {
            Adt1::C0(_, _, h1042, _, _, _, _, _, _, _, _) => {
                h1042
            }
        };
        let a1045 = 0u64;
        nat_rem(a1043, a1044, a1045)
    }, {
        let a1051 = {
            let a1047 = v4;
            let a1048 = match v1.clone() {
                Adt1::C0(_, _, _, h1046, _, _, _, _, _, _, _) => {
                    h1046
                }
            };
            let a1049 = 0u64;
            nat_rem(a1047, a1048, a1049)
        };
        let a1052 = match v2.clone() {
            Adt1::C0(_, _, _, h1050, _, _, _, _, _, _, _) => {
                h1050
            }
        };
        let a1053 = 0u64;
        nat_rem(a1051, a1052, a1053)
    }, {
        let a1059 = {
            let a1055 = v4;
            let a1056 = match v1 {
                Adt1::C0(_, _, _, h1054, _, _, _, _, _, _, _) => {
                    h1054
                }
            };
            let a1057 = 0u64;
            nat_quot(a1055, a1056, a1057)
        };
        let a1060 = match v2.clone() {
            Adt1::C0(_, _, _, _, h1058, _, _, _, _, _, _) => {
                h1058
            }
        };
        let a1061 = 0u64;
        nat_rem(a1059, a1060, a1061)
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h1062, _, _, _, _, _) => {
            h1062
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h1063, _, _, _, _) => {
            h1063
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1064, _, _, _) => {
            h1064
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1065, _, _) => {
            h1065
        }
    })
}

pub fn f54(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a1078 = {
        let a1074 = {
            let a1070 = {
                let a1066 = v0;
                let a1067 = v4;
                nat_mul(a1066, a1067)?
            };
            let a1071 = {
                let a1068 = v1;
                let a1069 = v5;
                nat_mul(a1068, a1069)?
            };
            nat_add(a1070, a1071)?
        };
        let a1075 = {
            let a1072 = v2;
            let a1073 = v6;
            nat_mul(a1072, a1073)?
        };
        nat_add(a1074, a1075)?
    };
    let a1079 = {
        let a1076 = v3;
        let a1077 = v7;
        nat_mul(a1076, a1077)?
    };
    nat_add(a1078, a1079)
}

pub fn f55(v0: u64, v1: Adt1, v2: u64, v3: u64) -> R<Adt3> {
    let v4: u64 = {
        let a1080 = v0;
        let a1081 = 2u64;
        let a1082 = 0u64;
        nat_quot(a1080, a1081, a1082)
    };
    let v5: u64 = f43(match v1.clone() {
        Adt1::C0(h1083, _, _, _, _, _, _, _, _, _, _) => {
            h1083
        }
    });
    let a1087 = {
        let a1084 = v0;
        let a1085 = 2u64;
        let a1086 = 0u64;
        nat_rem(a1084, a1085, a1086)
    };
    let a1088 = 0u64;
    if nat_eq(a1087, a1088) {
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 4u64, f52(v4, v1)?, 2u64, 0u64, 0u64, 2u64, v5, 8u64))
    } else {
        let a1089 = v3;
        let a1090 = v2;
        if if nat_lt(a1089, a1090) {
            let a1093 = v3;
            let a1094 = {
                let a1091 = v5;
                let a1092 = 4u64;
                if nat_eq(a1091, a1092) {
                    2147483648u64
                } else {
                    9223372036854775808u64
                }
            };
            nat_lt(a1093, a1094)
        } else {
            false
        } {
            Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, {
                let a1095 = 3u64;
                let a1096 = v4;
                nat_add(a1095, a1096)?
            }, 8u64, 8u64))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f56(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64) -> R<Adt3> {
    let v5: u64 = {
        let a1098 = 1u64;
        let a1099 = f45(v1.clone(), match v3.clone() {
            Adt1::C0(h1097, _, _, _, _, _, _, _, _, _, _) => {
                h1097
            }
        })?;
        nat_add(a1098, a1099)?
    };
    let v6: u64 = {
        let a1100 = v0;
        let a1101 = v5;
        let a1102 = 0u64;
        nat_rem(a1100, a1101, a1102)
    };
    let v7: u64 = {
        let a1103 = v0;
        let a1104 = v5;
        let a1105 = 0u64;
        nat_quot(a1103, a1104, a1105)
    };
    let v8: u64 = {
        let a1107 = v7;
        let a1108 = match v1.clone() {
            Adt1::C0(_, _, h1106, _, _, _, _, _, _, _, _) => {
                h1106
            }
        };
        let a1109 = 0u64;
        nat_rem(a1107, a1108, a1109)
    };
    let v9: u64 = {
        let a1111 = v7;
        let a1112 = match v1.clone() {
            Adt1::C0(_, _, h1110, _, _, _, _, _, _, _, _) => {
                h1110
            }
        };
        let a1113 = 0u64;
        nat_quot(a1111, a1112, a1113)
    };
    let v10: u64 = {
        let a1115 = v9;
        let a1116 = match v1.clone() {
            Adt1::C0(_, _, _, h1114, _, _, _, _, _, _, _) => {
                h1114
            }
        };
        let a1117 = 0u64;
        nat_rem(a1115, a1116, a1117)
    };
    let v11: u64 = {
        let a1119 = v9;
        let a1120 = match v1.clone() {
            Adt1::C0(_, _, _, h1118, _, _, _, _, _, _, _) => {
                h1118
            }
        };
        let a1121 = 0u64;
        nat_quot(a1119, a1120, a1121)
    };
    let a1122 = v6;
    let a1123 = 0u64;
    if nat_eq(a1122, a1123) {
        let v12: u64 = {
            let a1140 = v8;
            let a1141 = {
                let a1138 = match v2.clone() {
                    Adt1::C0(_, h1124, _, _, _, _, _, _, _, _, _) => {
                        h1124
                    }
                };
                let a1139 = {
                    let a1136 = {
                        let a1126 = v10;
                        let a1127 = match v2.clone() {
                            Adt1::C0(_, _, h1125, _, _, _, _, _, _, _, _) => {
                                h1125
                            }
                        };
                        let a1128 = 0u64;
                        nat_rem(a1126, a1127, a1128)
                    };
                    let a1137 = {
                        let a1134 = match v2.clone() {
                            Adt1::C0(_, _, h1129, _, _, _, _, _, _, _, _) => {
                                h1129
                            }
                        };
                        let a1135 = {
                            let a1131 = v11;
                            let a1132 = match v2 {
                                Adt1::C0(_, _, _, h1130, _, _, _, _, _, _, _) => {
                                    h1130
                                }
                            };
                            let a1133 = 0u64;
                            nat_rem(a1131, a1132, a1133)
                        };
                        nat_mul(a1134, a1135)?
                    };
                    nat_add(a1136, a1137)?
                };
                nat_mul(a1138, a1139)?
            };
            nat_add(a1140, a1141)?
        };
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, {
            let a1142 = 3u64;
            let a1143 = v12;
            nat_add(a1142, a1143)?
        }, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
    } else {
        let a1145 = v4;
        let a1146 = match v3.clone() {
            Adt1::C0(_, _, h1144, _, _, _, _, _, _, _, _) => {
                h1144
            }
        };
        if nat_lt(a1145, a1146) {
            let v13: u64 = {
                let a1151 = {
                    let a1147 = v6;
                    let a1148 = 1u64;
                    nat_sub(a1147, a1148)
                };
                let a1152 = f72(match v1.clone() {
                    Adt1::C0(h1149, _, _, _, _, _, _, _, _, _, _) => {
                        h1149
                    }
                }, match v3.clone() {
                    Adt1::C0(h1150, _, _, _, _, _, _, _, _, _, _) => {
                        h1150
                    }
                });
                let a1153 = 0u64;
                nat_quot(a1151, a1152, a1153)
            };
            let v14: u64 = f54({
                let a1155 = v13;
                let a1156 = f28(match v3.clone() {
                    Adt1::C0(h1154, _, _, _, _, _, _, _, _, _, _) => {
                        h1154
                    }
                });
                nat_mul(a1155, a1156)?
            }, v8, v10, v11, match v1.clone() {
                Adt1::C0(_, _, _, _, _, h1157, _, _, _, _, _) => {
                    h1157
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, h1158, _, _, _, _) => {
                    h1158
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1159, _, _, _) => {
                    h1159
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1160, _, _) => {
                    h1160
                }
            })?;
            let v15: u64 = f54(v13, v4, v10, v11, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h1161, _, _, _, _, _) => {
                    h1161
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h1162, _, _, _, _) => {
                    h1162
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1163, _, _, _) => {
                    h1163
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1164, _, _) => {
                    h1164
                }
            })?;
            let a1166 = 0u64;
            let a1167 = f46(match v3.clone() {
                Adt1::C0(h1165, _, _, _, _, _, _, _, _, _, _) => {
                    h1165
                }
            });
            if nat_lt(a1166, a1167) {
                Ok(Adt3::C6(v14, v15))
            } else {
                Ok(f73(match v1 {
                    Adt1::C0(h1168, _, _, _, _, _, _, _, _, _, _) => {
                        h1168
                    }
                }, match v3 {
                    Adt1::C0(h1169, _, _, _, _, _, _, _, _, _, _) => {
                        h1169
                    }
                }, {
                    let a1170 = v6;
                    let a1171 = 1u64;
                    nat_sub(a1170, a1171)
                }, v14, v15))
            }
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f57(v0: u64, v1: u64, v2: u64) -> Adt3 {
    Adt3::C3(2u64, 0u64, 0u64, 0u64, 1u64, v0, v1, v2, 0u64, 2u64, 4u64, 4u64)
}

pub fn f58(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt3 {
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

pub fn f59(v0: u64, v1: u64) -> Adt3 {
    let a1172 = v0;
    let a1173 = 8u64;
    if nat_eq(a1172, a1173) {
        Adt3::C3(1u64, 6u64, 0u64, 0u64, 1u64, 0u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
    } else {
        let a1174 = v0;
        let a1175 = 9u64;
        if nat_eq(a1174, a1175) {
            Adt3::C3(1u64, 2u64, 0u64, 0u64, 2u64, v1, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
        } else {
            let a1176 = v0;
            let a1177 = 10u64;
            if nat_eq(a1176, a1177) {
                Adt3::C3(5u64, 16u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
            } else {
                let a1178 = v0;
                let a1179 = 11u64;
                if nat_eq(a1178, a1179) {
                    Adt3::C3(1u64, 0u64, 0u64, 0u64, 2u64, 1065353216u64, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
                } else {
                    let a1180 = v0;
                    let a1181 = 12u64;
                    if nat_eq(a1180, a1181) {
                        Adt3::C3(1u64, 3u64, 0u64, 0u64, 1u64, 0u64, 1u64, 2u64, 0u64, 0u64, 4u64, 4u64)
                    } else {
                        let a1182 = v0;
                        let a1183 = 13u64;
                        if nat_eq(a1182, a1183) {
                            Adt3::C3(1u64, 0u64, 0u64, 0u64, 1u64, 1u64, 2u64, 1065353216u64, 0u64, 1u64, 4u64, 4u64)
                        } else {
                            let a1184 = v0;
                            let a1185 = 14u64;
                            if nat_eq(a1184, a1185) {
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

pub fn f60(v0: u64, v1: u64, v2: Adt2) -> R<Adt3> {
    let v3: Adt1 = match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1186, _, _, _, _) => {
            h1186
        }
    };
    let v4: Adt1 = match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h1187, _, _, _) => {
            h1187
        }
    };
    let v5: Adt1 = match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1188, _, _) => {
            h1188
        }
    };
    let v6: u64 = match v2.clone() {
        Adt2::C0(_, _, _, h1189, _, _, _, _, _, _, _, _) => {
            h1189
        }
    };
    let v7: u64 = {
        let a1191 = v1;
        let a1192 = match v5.clone() {
            Adt1::C0(_, h1190, _, _, _, _, _, _, _, _, _) => {
                h1190
            }
        };
        let a1193 = 0u64;
        nat_quot(a1191, a1192, a1193)
    };
    let v8: u64 = f21(match v5.clone() {
        Adt1::C0(h1194, _, _, _, _, _, _, _, _, _, _) => {
            h1194
        }
    });
    let v9: u64 = {
        let a1199 = {
            let a1196 = v1;
            let a1197 = match v5.clone() {
                Adt1::C0(_, h1195, _, _, _, _, _, _, _, _, _) => {
                    h1195
                }
            };
            let a1198 = 0u64;
            nat_rem(a1196, a1197, a1198)
        };
        let a1200 = v8;
        nat_mul(a1199, a1200)?
    };
    let v10: u64 = {
        let a1202 = match v5.clone() {
            Adt1::C0(_, h1201, _, _, _, _, _, _, _, _, _) => {
                h1201
            }
        };
        let a1203 = v8;
        nat_mul(a1202, a1203)?
    };
    let a1204 = v0;
    let a1205 = 0u64;
    if nat_eq(a1204, a1205) {
        Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
            Adt1::C0(h1206, _, _, _, _, _, _, _, _, _, _) => {
                h1206
            }
        }, 0u64, 0u64, {
            let a1214 = {
                let a1210 = {
                    let a1208 = v7;
                    let a1209 = match v3 {
                        Adt1::C0(_, _, _, _, _, _, h1207, _, _, _, _) => {
                            h1207
                        }
                    };
                    nat_mul(a1208, a1209)?
                };
                let a1211 = v9;
                nat_add(a1210, a1211)?
            };
            let a1215 = {
                let a1212 = v6;
                let a1213 = 100u64;
                if nat_eq(a1212, a1213) {
                    v10
                } else {
                    0u64
                }
            };
            nat_add(a1214, a1215)?
        }, 2u64, 0u64, 0u64, 0u64, v8, 4u64))
    } else {
        let a1216 = v0;
        let a1217 = 1u64;
        if nat_eq(a1216, a1217) {
            Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                Adt1::C0(h1218, _, _, _, _, _, _, _, _, _, _) => {
                    h1218
                }
            }, 0u64, {
                let a1219 = v6;
                let a1220 = 98u64;
                if nat_eq(a1219, a1220) {
                    4u64
                } else {
                    0u64
                }
            }, {
                let a1228 = {
                    let a1224 = {
                        let a1222 = v7;
                        let a1223 = match v4 {
                            Adt1::C0(_, _, _, _, _, _, h1221, _, _, _, _) => {
                                h1221
                            }
                        };
                        nat_mul(a1222, a1223)?
                    };
                    let a1225 = v9;
                    nat_add(a1224, a1225)?
                };
                let a1229 = {
                    let a1226 = v6;
                    let a1227 = 99u64;
                    if nat_eq(a1226, a1227) {
                        v10
                    } else {
                        0u64
                    }
                };
                nat_add(a1228, a1229)?
            }, 2u64, 0u64, 0u64, 1u64, v8, 4u64))
        } else {
            let a1230 = v0;
            let a1231 = 2u64;
            if nat_eq(a1230, a1231) {
                Ok(Adt3::C4(f74(match v2 {
                    Adt2::C0(_, _, _, _, h1232, _, _, _, _, _, _, _) => {
                        h1232
                    }
                }, v5.clone(), v1), match v5 {
                    Adt1::C0(h1233, _, _, _, _, _, _, _, _, _, _) => {
                        h1233
                    }
                }, 0u64, 1u64, 0u64))
            } else {
                Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                    Adt1::C0(h1234, _, _, _, _, _, _, _, _, _, _) => {
                        h1234
                    }
                }, 1u64, 0u64, 2u64, 0u64, 3u64, {
                    let a1238 = {
                        let a1236 = v7;
                        let a1237 = match v5 {
                            Adt1::C0(_, _, _, _, _, _, h1235, _, _, _, _) => {
                                h1235
                            }
                        };
                        nat_mul(a1236, a1237)?
                    };
                    let a1239 = v9;
                    nat_add(a1238, a1239)?
                }, 4u64, v8))
            }
        }
    }
}

pub fn f61(v0: u64) -> bool {
    let a1240 = v0;
    let a1241 = 27u64;
    if nat_eq(a1240, a1241) {
        true
    } else {
        let a1242 = v0;
        let a1243 = 28u64;
        nat_eq(a1242, a1243)
    }
}

pub fn f62(v0: u64, v1: Adt2) -> R<Adt3> {
    let v2: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1244, _, _, _, _) => {
            h1244
        }
    };
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h1245, _, _, _) => {
            h1245
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1246, _, _) => {
            h1246
        }
    };
    let v5: u64 = match v1 {
        Adt2::C0(_, _, _, h1247, _, _, _, _, _, _, _, _) => {
            h1247
        }
    };
    let v6: u64 = f71(v5, match v2.clone() {
        Adt1::C0(h1248, _, _, _, _, _, _, _, _, _, _) => {
            h1248
        }
    });
    let v7: u64 = {
        let a1249 = v0;
        let a1250 = v6;
        let a1251 = 0u64;
        nat_quot(a1249, a1250, a1251)
    };
    let v8: u64 = f53(v7, v2.clone(), v3)?;
    let a1252 = v6;
    let a1253 = 2u64;
    if if nat_eq(a1252, a1253) {
        let a1257 = {
            let a1254 = v0;
            let a1255 = v6;
            let a1256 = 0u64;
            nat_rem(a1254, a1255, a1256)
        };
        let a1258 = 0u64;
        nat_eq(a1257, a1258)
    } else {
        false
    } {
        Ok(Adt3::C3(6u64, 15u64, 0u64, 0u64, 4u64, v8, 2u64, 9223372036854775808u64, 0u64, 0u64, 8u64, 8u64))
    } else {
        let v9: u64 = {
            let a1260 = match v2.clone() {
                Adt1::C0(h1259, _, _, _, _, _, _, _, _, _, _) => {
                    h1259
                }
            };
            let a1261 = 27u64;
            if nat_eq(a1260, a1261) {
                let a1262 = 15u64;
                let a1263 = v5;
                nat_add(a1262, a1263)?
            } else {
                let a1264 = v5;
                let a1265 = 2u64;
                if nat_eq(a1264, a1265) {
                    14u64
                } else {
                    0u64
                }
            }
        };
        Ok(Adt3::C3(6u64, v9, 0u64, 0u64, 0u64, f52(v7, v2)?, {
            let a1266 = v6;
            let a1267 = 2u64;
            if nat_eq(a1266, a1267) {
                1u64
            } else {
                4u64
            }
        }, {
            let a1268 = v6;
            let a1269 = 2u64;
            if nat_eq(a1268, a1269) {
                0u64
            } else {
                v8
            }
        }, 3u64, f52(v7, v4)?, 8u64, 8u64))
    }
}

pub fn f63(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1280 = {
        let a1276 = {
            let a1272 = 1u64;
            let a1273 = {
                let a1270 = v0;
                let a1271 = 0u64;
                if nat_eq(a1270, a1271) {
                    0u64
                } else {
                    1u64
                }
            };
            nat_add(a1272, a1273)?
        };
        let a1277 = {
            let a1274 = v1;
            let a1275 = 0u64;
            if nat_eq(a1274, a1275) {
                0u64
            } else {
                1u64
            }
        };
        nat_add(a1276, a1277)?
    };
    let a1281 = {
        let a1278 = v2;
        let a1279 = 0u64;
        if nat_eq(a1278, a1279) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a1280, a1281)
}

pub fn f64(v0: u64) -> u64 {
    let a1282 = v0;
    let a1283 = 43u64;
    if nat_eq(a1282, a1283) {
        1u64
    } else {
        f21(v0)
    }
}

pub fn f65(v0: u64) -> Adt4 {
    let a1284 = v0;
    let a1285 = 0u64;
    if nat_eq(a1284, a1285) {
        Adt4::C1(Adt5::C0(0u64, 1u64, 4u64))
    } else {
        let a1286 = v0;
        let a1287 = 1u64;
        if nat_eq(a1286, a1287) {
            Adt4::C1(Adt5::C0(1u64, 1u64, 2u64))
        } else {
            let a1288 = v0;
            let a1289 = 2u64;
            if nat_eq(a1288, a1289) {
                Adt4::C1(Adt5::C0(2u64, 32u64, 18u64))
            } else {
                let a1290 = v0;
                let a1291 = 3u64;
                if nat_eq(a1290, a1291) {
                    Adt4::C1(Adt5::C0(3u64, 32u64, 20u64))
                } else {
                    let a1292 = v0;
                    let a1293 = 6u64;
                    if nat_eq(a1292, a1293) {
                        Adt4::C1(Adt5::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1294 = v0;
                        let a1295 = 7u64;
                        if nat_eq(a1294, a1295) {
                            Adt4::C1(Adt5::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1296 = v0;
                            let a1297 = 8u64;
                            if nat_eq(a1296, a1297) {
                                Adt4::C1(Adt5::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1298 = v0;
                                let a1299 = 9u64;
                                if nat_eq(a1298, a1299) {
                                    Adt4::C1(Adt5::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1300 = v0;
                                    let a1301 = 10u64;
                                    if nat_eq(a1300, a1301) {
                                        Adt4::C1(Adt5::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1302 = v0;
                                        let a1303 = 11u64;
                                        if nat_eq(a1302, a1303) {
                                            Adt4::C1(Adt5::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1304 = v0;
                                            let a1305 = 12u64;
                                            if nat_eq(a1304, a1305) {
                                                Adt4::C1(Adt5::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1306 = v0;
                                                let a1307 = 13u64;
                                                if nat_eq(a1306, a1307) {
                                                    Adt4::C1(Adt5::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1308 = v0;
                                                    let a1309 = 14u64;
                                                    if nat_eq(a1308, a1309) {
                                                        Adt4::C1(Adt5::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1310 = v0;
                                                        let a1311 = 15u64;
                                                        if nat_eq(a1310, a1311) {
                                                            Adt4::C1(Adt5::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1312 = v0;
                                                            let a1313 = 16u64;
                                                            if nat_eq(a1312, a1313) {
                                                                Adt4::C1(Adt5::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1314 = v0;
                                                                let a1315 = 17u64;
                                                                if nat_eq(a1314, a1315) {
                                                                    Adt4::C1(Adt5::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1316 = v0;
                                                                    let a1317 = 18u64;
                                                                    if nat_eq(a1316, a1317) {
                                                                        Adt4::C1(Adt5::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1318 = v0;
                                                                        let a1319 = 19u64;
                                                                        if nat_eq(a1318, a1319) {
                                                                            Adt4::C1(Adt5::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1320 = v0;
                                                                            let a1321 = 20u64;
                                                                            if nat_eq(a1320, a1321) {
                                                                                Adt4::C1(Adt5::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1322 = v0;
                                                                                let a1323 = 21u64;
                                                                                if nat_eq(a1322, a1323) {
                                                                                    Adt4::C1(Adt5::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1324 = v0;
                                                                                    let a1325 = 22u64;
                                                                                    if nat_eq(a1324, a1325) {
                                                                                        Adt4::C1(Adt5::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1326 = v0;
                                                                                        let a1327 = 23u64;
                                                                                        if nat_eq(a1326, a1327) {
                                                                                            Adt4::C1(Adt5::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1328 = v0;
                                                                                            let a1329 = 24u64;
                                                                                            if nat_eq(a1328, a1329) {
                                                                                                Adt4::C1(Adt5::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1330 = v0;
                                                                                                let a1331 = 25u64;
                                                                                                if nat_eq(a1330, a1331) {
                                                                                                    Adt4::C1(Adt5::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1332 = v0;
                                                                                                    let a1333 = 26u64;
                                                                                                    if nat_eq(a1332, a1333) {
                                                                                                        Adt4::C1(Adt5::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1334 = v0;
                                                                                                        let a1335 = 27u64;
                                                                                                        if nat_eq(a1334, a1335) {
                                                                                                            Adt4::C1(Adt5::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1336 = v0;
                                                                                                            let a1337 = 28u64;
                                                                                                            if nat_eq(a1336, a1337) {
                                                                                                                Adt4::C1(Adt5::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1338 = v0;
                                                                                                                let a1339 = 29u64;
                                                                                                                if nat_eq(a1338, a1339) {
                                                                                                                    Adt4::C1(Adt5::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1340 = v0;
                                                                                                                    let a1341 = 30u64;
                                                                                                                    if nat_eq(a1340, a1341) {
                                                                                                                        Adt4::C1(Adt5::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1342 = v0;
                                                                                                                        let a1343 = 34u64;
                                                                                                                        if nat_eq(a1342, a1343) {
                                                                                                                            Adt4::C1(Adt5::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1344 = v0;
                                                                                                                            let a1345 = 35u64;
                                                                                                                            if nat_eq(a1344, a1345) {
                                                                                                                                Adt4::C1(Adt5::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1346 = v0;
                                                                                                                                let a1347 = 39u64;
                                                                                                                                if nat_eq(a1346, a1347) {
                                                                                                                                    Adt4::C1(Adt5::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1348 = v0;
                                                                                                                                    let a1349 = 40u64;
                                                                                                                                    if nat_eq(a1348, a1349) {
                                                                                                                                        Adt4::C1(Adt5::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1350 = v0;
                                                                                                                                        let a1351 = 41u64;
                                                                                                                                        if nat_eq(a1350, a1351) {
                                                                                                                                            Adt4::C1(Adt5::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1352 = v0;
                                                                                                                                            let a1353 = 42u64;
                                                                                                                                            if nat_eq(a1352, a1353) {
                                                                                                                                                Adt4::C1(Adt5::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1354 = v0;
                                                                                                                                                let a1355 = 43u64;
                                                                                                                                                if nat_eq(a1354, a1355) {
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

pub fn f66(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f24(v0.clone(), v1.clone()) {
        let a1358 = match v0.clone() {
            Adt1::C0(h1356, _, _, _, _, _, _, _, _, _, _) => {
                h1356
            }
        };
        let a1359 = match v1.clone() {
            Adt1::C0(h1357, _, _, _, _, _, _, _, _, _, _) => {
                h1357
            }
        };
        nat_eq(a1358, a1359)
    } else {
        false
    } {
        let a1362 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1360) => {
                h1360
            }
        };
        let a1363 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1361) => {
                h1361
            }
        };
        nat_eq(a1362, a1363)
    } else {
        false
    } {
        let a1366 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1364, _, _, _, _, _) => {
                h1364
            }
        };
        let a1367 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1365, _, _, _, _, _) => {
                h1365
            }
        };
        nat_eq(a1366, a1367)
    } else {
        false
    } {
        let a1370 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1368, _, _, _, _) => {
                h1368
            }
        };
        let a1371 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1369, _, _, _, _) => {
                h1369
            }
        };
        nat_eq(a1370, a1371)
    } else {
        false
    } {
        let a1374 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1372, _, _, _) => {
                h1372
            }
        };
        let a1375 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1373, _, _, _) => {
                h1373
            }
        };
        nat_eq(a1374, a1375)
    } else {
        false
    } {
        let a1378 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1376, _, _) => {
                h1376
            }
        };
        let a1379 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1377, _, _) => {
                h1377
            }
        };
        nat_eq(a1378, a1379)
    } else {
        false
    }
}

pub fn f67(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a1380 = v1;
    let a1381 = 1u64;
    if if if nat_le(a1380, a1381) {
        true
    } else {
        let a1382 = v0;
        let a1383 = v2;
        nat_le(a1382, a1383)
    } {
        let a1384 = v3;
        let a1385 = 1u64;
        if nat_le(a1384, a1385) {
            true
        } else {
            let a1392 = {
                let a1390 = v0;
                let a1391 = {
                    let a1388 = {
                        let a1386 = v1;
                        let a1387 = 1u64;
                        nat_sub(a1386, a1387)
                    };
                    let a1389 = v2;
                    nat_mul(a1388, a1389)?
                };
                nat_add(a1390, a1391)?
            };
            let a1393 = v4;
            nat_le(a1392, a1393)
        }
    } else {
        false
    } {
        let a1394 = v5;
        let a1395 = 1u64;
        if nat_le(a1394, a1395) {
            Ok(true)
        } else {
            let a1408 = {
                let a1406 = {
                    let a1400 = v0;
                    let a1401 = {
                        let a1398 = {
                            let a1396 = v1;
                            let a1397 = 1u64;
                            nat_sub(a1396, a1397)
                        };
                        let a1399 = v2;
                        nat_mul(a1398, a1399)?
                    };
                    nat_add(a1400, a1401)?
                };
                let a1407 = {
                    let a1404 = {
                        let a1402 = v3;
                        let a1403 = 1u64;
                        nat_sub(a1402, a1403)
                    };
                    let a1405 = v4;
                    nat_mul(a1404, a1405)?
                };
                nat_add(a1406, a1407)?
            };
            let a1409 = v6;
            Ok(nat_le(a1408, a1409))
        }
    } else {
        Ok(false)
    }
}

pub fn f68(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1410 = v0;
    let a1411 = 0u64;
    if if if nat_eq(a1410, a1411) {
        true
    } else {
        let a1412 = v1;
        let a1413 = 0u64;
        nat_eq(a1412, a1413)
    } {
        true
    } else {
        let a1414 = v2;
        let a1415 = 0u64;
        nat_eq(a1414, a1415)
    } {
        true
    } else {
        let a1416 = v3;
        let a1417 = 0u64;
        nat_eq(a1416, a1417)
    }
}

pub fn f69(v0: u64, v1: u64, v2: u64) -> bool {
    let a1418 = v0;
    let a1419 = 1u64;
    if nat_le(a1418, a1419) {
        true
    } else {
        let a1423 = {
            let a1420 = v1;
            let a1421 = v2;
            let a1422 = 1u64;
            nat_rem(a1420, a1421, a1422)
        };
        let a1424 = 0u64;
        nat_eq(a1423, a1424)
    }
}

pub fn f70(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f69(v0, v3, v6) {
        f69(v1, v4, v6)
    } else {
        false
    } {
        f69(v2, v5, v6)
    } else {
        false
    }
}

pub fn f71(v0: u64, v1: u64) -> u64 {
    let a1425 = v1;
    let a1426 = 28u64;
    if if nat_eq(a1425, a1426) {
        let a1427 = v0;
        let a1428 = 1u64;
        nat_eq(a1427, a1428)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f72(v0: u64, v1: u64) -> u64 {
    let a1429 = v0;
    let a1430 = 1u64;
    if if nat_eq(a1429, a1430) {
        let a1431 = v1;
        let a1432 = 43u64;
        nat_eq(a1431, a1432)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f73(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt3 {
    let a1433 = f72(v0, v1);
    let a1434 = 2u64;
    if nat_eq(a1433, a1434) {
        let a1438 = {
            let a1435 = v2;
            let a1436 = 2u64;
            let a1437 = 0u64;
            nat_rem(a1435, a1436, a1437)
        };
        let a1439 = 0u64;
        if nat_eq(a1438, a1439) {
            Adt3::C3(0u64, 0u64, v0, 0u64, 0u64, v3, 2u64, 0u64, 1u64, 0u64, f21(v0), 8u64)
        } else {
            Adt3::C3(0u64, 0u64, 0u64, v1, 1u64, 0u64, 2u64, 0u64, 3u64, v4, 8u64, f42(v1))
        }
    } else {
        Adt3::C3(0u64, 0u64, v0, v1, 0u64, v3, 2u64, 0u64, 3u64, v4, f21(v0), f42(v1))
    }
}

pub fn f74(v0: u64, v1: Adt1, v2: u64) -> u64 {
    let a1440 = v0;
    let a1441 = 2u64;
    if if nat_eq(a1440, a1441) {
        let a1443 = match v1.clone() {
            Adt1::C0(h1442, _, _, _, _, _, _, _, _, _, _) => {
                h1442
            }
        };
        let a1444 = 0u64;
        if nat_eq(a1443, a1444) {
            let a1456 = {
                let a1446 = v2;
                let a1447 = match v1.clone() {
                    Adt1::C0(_, h1445, _, _, _, _, _, _, _, _, _) => {
                        h1445
                    }
                };
                let a1448 = 0u64;
                nat_rem(a1446, a1447, a1448)
            };
            let a1457 = {
                let a1454 = match v1.clone() {
                    Adt1::C0(_, h1449, _, _, _, _, _, _, _, _, _) => {
                        h1449
                    }
                };
                let a1455 = {
                    let a1451 = match v1 {
                        Adt1::C0(_, h1450, _, _, _, _, _, _, _, _, _) => {
                            h1450
                        }
                    };
                    let a1452 = 4u64;
                    let a1453 = 0u64;
                    nat_rem(a1451, a1452, a1453)
                };
                nat_sub(a1454, a1455)
            };
            nat_lt(a1456, a1457)
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
