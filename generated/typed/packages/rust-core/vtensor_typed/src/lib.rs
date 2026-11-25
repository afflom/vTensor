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

pub fn f20(v0: u64, v1: u64, v2: u64) -> bool {
    let a606 = v1;
    let a607 = 27u64;
    if if if nat_eq(a606, a607) {
        true
    } else {
        let a608 = v1;
        let a609 = 28u64;
        nat_eq(a608, a609)
    } {
        let a610 = v1;
        let a611 = v2;
        nat_eq(a610, a611)
    } else {
        false
    } {
        let a612 = v0;
        let a613 = 3u64;
        nat_eq(a612, a613)
    } else {
        f22(v1)
    }
}

pub fn f21(v0: u64) -> u64 {
    let a614 = v0;
    let a615 = 0u64;
    if nat_eq(a614, a615) {
        4u64
    } else {
        let a616 = v0;
        let a617 = 1u64;
        if if nat_eq(a616, a617) {
            true
        } else {
            let a618 = v0;
            let a619 = 30u64;
            nat_eq(a618, a619)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f22(v0: u64) -> bool {
    let a620 = 0u64;
    let a621 = f34(v0);
    if nat_lt(a620, a621) {
        true
    } else {
        let a622 = 24u64;
        let a623 = v0;
        if nat_le(a622, a623) {
            let a624 = v0;
            let a625 = 28u64;
            nat_le(a624, a625)
        } else {
            false
        }
    }
}

pub fn f23(v0: Adt1, v1: Adt1) -> bool {
    if f31(v1.clone()) {
        f31(v0)
    } else {
        let a631 = {
            let a628 = match v0.clone() {
                Adt1::C0(_, h626, _, _, _, _, _, _, _, _, _) => {
                    h626
                }
            };
            let a629 = match v1.clone() {
                Adt1::C0(_, h627, _, _, _, _, _, _, _, _, _) => {
                    h627
                }
            };
            let a630 = 1u64;
            nat_rem(a628, a629, a630)
        };
        let a632 = 0u64;
        if if if nat_eq(a631, a632) {
            let a638 = {
                let a635 = match v0.clone() {
                    Adt1::C0(_, _, h633, _, _, _, _, _, _, _, _) => {
                        h633
                    }
                };
                let a636 = match v1.clone() {
                    Adt1::C0(_, _, h634, _, _, _, _, _, _, _, _) => {
                        h634
                    }
                };
                let a637 = 1u64;
                nat_rem(a635, a636, a637)
            };
            let a639 = 0u64;
            nat_eq(a638, a639)
        } else {
            false
        } {
            let a645 = {
                let a642 = match v0.clone() {
                    Adt1::C0(_, _, _, h640, _, _, _, _, _, _, _) => {
                        h640
                    }
                };
                let a643 = match v1.clone() {
                    Adt1::C0(_, _, _, h641, _, _, _, _, _, _, _) => {
                        h641
                    }
                };
                let a644 = 1u64;
                nat_rem(a642, a643, a644)
            };
            let a646 = 0u64;
            nat_eq(a645, a646)
        } else {
            false
        } {
            let a652 = {
                let a649 = match v0 {
                    Adt1::C0(_, _, _, _, h647, _, _, _, _, _, _) => {
                        h647
                    }
                };
                let a650 = match v1 {
                    Adt1::C0(_, _, _, _, h648, _, _, _, _, _, _) => {
                        h648
                    }
                };
                let a651 = 1u64;
                nat_rem(a649, a650, a651)
            };
            let a653 = 0u64;
            nat_eq(a652, a653)
        } else {
            false
        }
    }
}

pub fn f24(v0: Adt1, v1: Adt1) -> bool {
    let a656 = match v0.clone() {
        Adt1::C0(_, h654, _, _, _, _, _, _, _, _, _) => {
            h654
        }
    };
    let a657 = match v1.clone() {
        Adt1::C0(_, h655, _, _, _, _, _, _, _, _, _) => {
            h655
        }
    };
    if if if nat_eq(a656, a657) {
        let a660 = match v0.clone() {
            Adt1::C0(_, _, h658, _, _, _, _, _, _, _, _) => {
                h658
            }
        };
        let a661 = match v1.clone() {
            Adt1::C0(_, _, h659, _, _, _, _, _, _, _, _) => {
                h659
            }
        };
        nat_eq(a660, a661)
    } else {
        false
    } {
        let a664 = match v0.clone() {
            Adt1::C0(_, _, _, h662, _, _, _, _, _, _, _) => {
                h662
            }
        };
        let a665 = match v1.clone() {
            Adt1::C0(_, _, _, h663, _, _, _, _, _, _, _) => {
                h663
            }
        };
        nat_eq(a664, a665)
    } else {
        false
    } {
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
        nat_eq(a668, a669)
    } else {
        false
    }
}

pub fn f25(v0: u64, v1: u64, v2: Adt1, v3: u64) -> bool {
    let v4: u64 = match v2.clone() {
        Adt1::C0(h670, _, _, _, _, _, _, _, _, _, _) => {
            h670
        }
    };
    let a671 = v1;
    let a672 = v3;
    if if nat_eq(a671, a672) {
        true
    } else {
        let a673 = v0;
        let a674 = 0u64;
        if nat_eq(a673, a674) {
            let a675 = v4;
            let a676 = 0u64;
            if if nat_eq(a675, a676) {
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
            let a678 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, h677, _, _, _, _, _) => {
                    h677
                }
            };
            let a679 = f27(v4);
            nat_eq(a678, a679)
        } else {
            false
        } {
            let a684 = {
                let a681 = match v2 {
                    Adt1::C0(_, h680, _, _, _, _, _, _, _, _, _) => {
                        h680
                    }
                };
                let a682 = f28(v4);
                let a683 = 1u64;
                nat_rem(a681, a682, a683)
            };
            let a685 = 0u64;
            nat_eq(a684, a685)
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
        let a686 = v0;
        let a687 = v1;
        if if nat_eq(a686, a687) {
            f22(v0)
        } else {
            false
        } {
            true
        } else {
            let a688 = f21(v0);
            let a689 = 2u64;
            if nat_eq(a688, a689) {
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
    let a690 = 24u64;
    let a691 = v0;
    if if nat_le(a690, a691) {
        let a692 = v0;
        let a693 = 28u64;
        nat_le(a692, a693)
    } else {
        false
    } {
        let m694 = f60(v0);
        match m694 {
            Adt4::C0 => {
                0u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, _, h695) => {
                        h695
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
        let m696 = f60(v0);
        match m696 {
            Adt4::C0 => {
                1u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, h697, _) => {
                        h697
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
        Adt1::C0(_, h698, _, _, _, _, _, _, _, _, _) => {
            h698
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h699, _, _, _, _, _, _, _, _) => {
            h699
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h700, _, _, _, _, _, _, _) => {
            h700
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, h701, _, _, _, _, _, _) => {
            h701
        }
    })
}

pub fn f32(v0: u64) -> Option<u64> {
    let a702 = v0;
    let a703 = 16u64;
    if if if nat_eq(a702, a703) {
        true
    } else {
        let a704 = v0;
        let a705 = 17u64;
        nat_eq(a704, a705)
    } {
        true
    } else {
        let a706 = v0;
        let a707 = 19u64;
        nat_eq(a706, a707)
    } {
        Some(1065353216u64)
    } else {
        None::<u64>
    }
}

pub fn f33(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<bool> {
    let a709 = match v1.clone() {
        Adt1::C0(h708, _, _, _, _, _, _, _, _, _, _) => {
            h708
        }
    };
    let a710 = 43u64;
    if nat_eq(a709, a710) {
        let a711 = v0;
        let a712 = 0u64;
        if if if if if if nat_eq(a711, a712) {
            let a714 = match v2.clone() {
                Adt1::C0(h713, _, _, _, _, _, _, _, _, _, _) => {
                    h713
                }
            };
            let a715 = 0u64;
            nat_eq(a714, a715)
        } else {
            false
        } {
            let a717 = match v3.clone() {
                Adt1::C0(h716, _, _, _, _, _, _, _, _, _, _) => {
                    h716
                }
            };
            let a718 = 0u64;
            if nat_eq(a717, a718) {
                true
            } else {
                let a720 = match v3.clone() {
                    Adt1::C0(h719, _, _, _, _, _, _, _, _, _, _) => {
                        h719
                    }
                };
                let a721 = 43u64;
                nat_eq(a720, a721)
            }
        } else {
            false
        } {
            let a723 = match v3.clone() {
                Adt1::C0(h722, _, _, _, _, _, _, _, _, _, _) => {
                    h722
                }
            };
            let a724 = 43u64;
            if nat_eq(a723, a724) {
                f23(v1.clone(), v2.clone())
            } else {
                f24(v1.clone(), v2.clone())
            }
        } else {
            false
        } {
            let a726 = match v2 {
                Adt1::C0(_, _, _, _, _, h725, _, _, _, _, _) => {
                    h725
                }
            };
            let a727 = 4u64;
            nat_eq(a726, a727)
        } else {
            false
        } {
            let a730 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h728, _, _, _, _, _) => {
                    h728
                }
            };
            let a731 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h729, _, _, _, _) => {
                    h729
                }
            };
            if nat_le(a730, a731) {
                let a734 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, h732, _, _, _, _) => {
                        h732
                    }
                };
                let a735 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h733, _, _, _) => {
                        h733
                    }
                };
                if nat_le(a734, a735) {
                    let a738 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, h736, _, _, _) => {
                            h736
                        }
                    };
                    let a739 = match v3 {
                        Adt1::C0(_, _, _, _, _, _, _, _, h737, _, _) => {
                            h737
                        }
                    };
                    nat_le(a738, a739)
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        } {
            let a747 = {
                let a745 = match v1.clone() {
                    Adt1::C0(_, _, h740, _, _, _, _, _, _, _, _) => {
                        h740
                    }
                };
                let a746 = {
                    let a743 = match v1.clone() {
                        Adt1::C0(_, _, _, h741, _, _, _, _, _, _, _) => {
                            h741
                        }
                    };
                    let a744 = match v1 {
                        Adt1::C0(_, _, _, _, h742, _, _, _, _, _, _) => {
                            h742
                        }
                    };
                    nat_mul(a743, a744)?
                };
                nat_mul(a745, a746)?
            };
            let a748 = 2147483647u64;
            Ok(nat_le(a747, a748))
        } else {
            Ok(false)
        }
    } else {
        if f23(v1.clone(), v2.clone()) {
            Ok(f19(match v1 {
                Adt1::C0(h749, _, _, _, _, _, _, _, _, _, _) => {
                    h749
                }
            }, match v2 {
                Adt1::C0(h750, _, _, _, _, _, _, _, _, _, _) => {
                    h750
                }
            }, match v3 {
                Adt1::C0(h751, _, _, _, _, _, _, _, _, _, _) => {
                    h751
                }
            }))
        } else {
            Ok(false)
        }
    }
}

pub fn f34(v0: u64) -> u64 {
    let a752 = v0;
    let a753 = 43u64;
    if nat_eq(a752, a753) {
        1u64
    } else {
        f21(v0)
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
        f64(match v0.clone() {
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
        f65(match v0.clone() {
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

pub fn f40(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a822 = {
        let a818 = {
            let a814 = 1u64;
            let a815 = {
                let a812 = v0;
                let a813 = 0u64;
                if nat_eq(a812, a813) {
                    0u64
                } else {
                    1u64
                }
            };
            nat_add(a814, a815)?
        };
        let a819 = {
            let a816 = v1;
            let a817 = 0u64;
            if nat_eq(a816, a817) {
                0u64
            } else {
                1u64
            }
        };
        nat_add(a818, a819)?
    };
    let a823 = {
        let a820 = v2;
        let a821 = 0u64;
        if nat_eq(a820, a821) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a822, a823)
}

pub fn f41(v0: Adt1, v1: Adt1) -> bool {
    if if if if if if f24(v0.clone(), v1.clone()) {
        let a826 = match v0.clone() {
            Adt1::C0(h824, _, _, _, _, _, _, _, _, _, _) => {
                h824
            }
        };
        let a827 = match v1.clone() {
            Adt1::C0(h825, _, _, _, _, _, _, _, _, _, _) => {
                h825
            }
        };
        nat_eq(a826, a827)
    } else {
        false
    } {
        let a830 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h828, _, _, _, _, _) => {
                h828
            }
        };
        let a831 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h829, _, _, _, _, _) => {
                h829
            }
        };
        nat_eq(a830, a831)
    } else {
        false
    } {
        let a834 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h832, _, _, _, _) => {
                h832
            }
        };
        let a835 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h833, _, _, _, _) => {
                h833
            }
        };
        nat_eq(a834, a835)
    } else {
        false
    } {
        let a838 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h836, _, _, _) => {
                h836
            }
        };
        let a839 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h837, _, _, _) => {
                h837
            }
        };
        nat_eq(a838, a839)
    } else {
        false
    } {
        let a842 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h840, _, _) => {
                h840
            }
        };
        let a843 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, h841, _, _) => {
                h841
            }
        };
        nat_eq(a842, a843)
    } else {
        false
    } {
        let a845 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h844) => {
                h844
            }
        };
        let a846 = 0u64;
        if if nat_eq(a845, a846) {
            true
        } else {
            let a848 = match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h847) => {
                    h847
                }
            };
            let a849 = 0u64;
            nat_eq(a848, a849)
        } {
            true
        } else {
            let a852 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h850) => {
                    h850
                }
            };
            let a853 = match v1 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h851) => {
                    h851
                }
            };
            nat_eq(a852, a853)
        }
    } else {
        false
    }
}

pub fn f42(v0: u64) -> u64 {
    if f18(v0) {
        let m854 = f60(v0);
        match m854 {
            Adt4::C0 => {
                0u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, _, h855) => {
                        h855
                    }
                }
            }
        }
    } else {
        let a856 = v0;
        let a857 = 26u64;
        if nat_eq(a856, a857) {
            4u64
        } else {
            f34(v0)
        }
    }
}

pub fn f43(v0: u64) -> u64 {
    let a858 = v0;
    let a859 = 26u64;
    if nat_eq(a858, a859) {
        4u64
    } else {
        let a860 = v0;
        let a861 = 27u64;
        if nat_eq(a860, a861) {
            8u64
        } else {
            0u64
        }
    }
}

pub fn f44(v0: Adt1) -> bool {
    let a863 = match v0.clone() {
        Adt1::C0(_, h862, _, _, _, _, _, _, _, _, _) => {
            h862
        }
    };
    let a864 = 1u64;
    if nat_eq(a863, a864) {
        true
    } else {
        let a867 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h865, _, _, _, _, _) => {
                h865
            }
        };
        let a868 = f34(match v0 {
            Adt1::C0(h866, _, _, _, _, _, _, _, _, _, _) => {
                h866
            }
        });
        nat_eq(a867, a868)
    }
}

pub fn f45(v0: Adt1, v1: u64) -> R<u64> {
    let a874 = {
        let a870 = match v0.clone() {
            Adt1::C0(_, h869, _, _, _, _, _, _, _, _, _) => {
                h869
            }
        };
        let a871 = f28(v1);
        let a872 = 0u64;
        nat_quot(a870, a871, a872)
    };
    let a875 = f66(match v0 {
        Adt1::C0(h873, _, _, _, _, _, _, _, _, _, _) => {
            h873
        }
    }, v1);
    nat_mul(a874, a875)
}

pub fn f46(v0: u64) -> u64 {
    if if f18(v0) {
        true
    } else {
        let a876 = v0;
        let a877 = 26u64;
        nat_eq(a876, a877)
    } {
        f28(v0)
    } else {
        0u64
    }
}

pub fn f47(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a879 = match v0.clone() {
            Adt1::C0(_, h878, _, _, _, _, _, _, _, _, _) => {
                h878
            }
        };
        let a880 = 4u64;
        nat_mul(a879, a880)?
    };
    let v2: u64 = {
        let a883 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h881, _, _, _, _) => {
                h881
            }
        };
        let a884 = match v0.clone() {
            Adt1::C0(_, _, h882, _, _, _, _, _, _, _, _) => {
                h882
            }
        };
        nat_mul(a883, a884)?
    };
    let v3: u64 = {
        let a886 = v2;
        let a887 = match v0.clone() {
            Adt1::C0(_, _, _, h885, _, _, _, _, _, _, _) => {
                h885
            }
        };
        nat_mul(a886, a887)?
    };
    let a889 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h888, _, _, _, _, _) => {
            h888
        }
    };
    let a890 = 4u64;
    if if if if nat_eq(a889, a890) {
        let a895 = {
            let a892 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h891) => {
                    h891
                }
            };
            let a893 = 4u64;
            let a894 = 1u64;
            nat_rem(a892, a893, a894)
        };
        let a896 = 0u64;
        nat_eq(a895, a896)
    } else {
        false
    } {
        let a904 = {
            let a902 = match v0.clone() {
                Adt1::C0(_, _, h897, _, _, _, _, _, _, _, _) => {
                    h897
                }
            };
            let a903 = {
                let a900 = match v0.clone() {
                    Adt1::C0(_, _, _, h898, _, _, _, _, _, _, _) => {
                        h898
                    }
                };
                let a901 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h899, _, _, _, _, _, _) => {
                        h899
                    }
                };
                nat_mul(a900, a901)?
            };
            nat_mul(a902, a903)?
        };
        let a905 = 1u64;
        if nat_le(a904, a905) {
            true
        } else {
            let a907 = v1;
            let a908 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h906, _, _, _, _) => {
                    h906
                }
            };
            if nat_le(a907, a908) {
                let a913 = {
                    let a910 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, h909, _, _, _, _) => {
                            h909
                        }
                    };
                    let a911 = 4u64;
                    let a912 = 1u64;
                    nat_rem(a910, a911, a912)
                };
                let a914 = 0u64;
                nat_eq(a913, a914)
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a916 = match v0.clone() {
            Adt1::C0(_, _, _, h915, _, _, _, _, _, _, _) => {
                h915
            }
        };
        let a917 = 1u64;
        if nat_le(a916, a917) {
            true
        } else {
            let a919 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h918, _, _, _) => {
                    h918
                }
            };
            let a920 = v2;
            nat_eq(a919, a920)
        }
    } else {
        false
    } {
        let a922 = match v0.clone() {
            Adt1::C0(_, _, _, _, h921, _, _, _, _, _, _) => {
                h921
            }
        };
        let a923 = 1u64;
        if nat_le(a922, a923) {
            Ok(true)
        } else {
            let a925 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, h924, _, _) => {
                    h924
                }
            };
            let a926 = v3;
            Ok(nat_eq(a925, a926))
        }
    } else {
        Ok(false)
    }
}

pub fn f48(v0: Adt1, v1: Adt1) -> R<bool> {
    let a928 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h927, _) => {
            h927
        }
    };
    let a929 = 0u64;
    if nat_eq(a928, a929) {
        Ok(true)
    } else {
        let a931 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h930, _) => {
                h930
            }
        };
        let a932 = 0u64;
        if nat_eq(a931, a932) {
            Ok(true)
        } else {
            let a934 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h933) => {
                    h933
                }
            };
            let a935 = 0u64;
            if nat_eq(a934, a935) {
                Ok(true)
            } else {
                let a937 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h936) => {
                        h936
                    }
                };
                let a938 = 0u64;
                if nat_eq(a937, a938) {
                    Ok(true)
                } else {
                    let a944 = {
                        let a941 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h939) => {
                                h939
                            }
                        };
                        let a942 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h940, _) => {
                                h940
                            }
                        };
                        nat_add(a941, a942)?
                    };
                    let a945 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h943) => {
                            h943
                        }
                    };
                    if nat_le(a944, a945) {
                        Ok(true)
                    } else {
                        let a951 = {
                            let a948 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h946) => {
                                    h946
                                }
                            };
                            let a949 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h947, _) => {
                                    h947
                                }
                            };
                            nat_add(a948, a949)?
                        };
                        let a952 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h950) => {
                                h950
                            }
                        };
                        Ok(nat_le(a951, a952))
                    }
                }
            }
        }
    }
}

pub fn f49(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a955 = match v0.clone() {
            Adt1::C0(_, _, h953, _, _, _, _, _, _, _, _) => {
                h953
            }
        };
        let a956 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h954, _, _, _, _) => {
                h954
            }
        };
        nat_mul(a955, a956)?
    };
    if if f44(v0.clone()) {
        let a958 = match v0.clone() {
            Adt1::C0(_, _, _, h957, _, _, _, _, _, _, _) => {
                h957
            }
        };
        let a959 = 1u64;
        if nat_eq(a958, a959) {
            true
        } else {
            let a961 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h960, _, _, _) => {
                    h960
                }
            };
            let a962 = v1;
            nat_eq(a961, a962)
        }
    } else {
        false
    } {
        let a964 = match v0.clone() {
            Adt1::C0(_, _, _, _, h963, _, _, _, _, _, _) => {
                h963
            }
        };
        let a965 = 1u64;
        if nat_eq(a964, a965) {
            Ok(true)
        } else {
            let a970 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h966, _, _) => {
                    h966
                }
            };
            let a971 = {
                let a968 = v1;
                let a969 = match v0 {
                    Adt1::C0(_, _, _, h967, _, _, _, _, _, _, _) => {
                        h967
                    }
                };
                nat_mul(a968, a969)?
            };
            Ok(nat_eq(a970, a971))
        }
    } else {
        Ok(false)
    }
}

pub fn f50(v0: u64) -> u64 {
    let a972 = v0;
    let a973 = 3u64;
    if nat_eq(a972, a973) {
        16u64
    } else {
        10u64
    }
}

pub fn f51(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a975 = v0;
        let a976 = match v1.clone() {
            Adt1::C0(_, h974, _, _, _, _, _, _, _, _, _) => {
                h974
            }
        };
        let a977 = 0u64;
        nat_quot(a975, a976, a977)
    };
    let v3: u64 = {
        let a979 = v2;
        let a980 = match v1.clone() {
            Adt1::C0(_, _, h978, _, _, _, _, _, _, _, _) => {
                h978
            }
        };
        let a981 = 0u64;
        nat_quot(a979, a980, a981)
    };
    f53({
        let a983 = v0;
        let a984 = match v1.clone() {
            Adt1::C0(_, h982, _, _, _, _, _, _, _, _, _) => {
                h982
            }
        };
        let a985 = 0u64;
        nat_rem(a983, a984, a985)
    }, {
        let a987 = v2;
        let a988 = match v1.clone() {
            Adt1::C0(_, _, h986, _, _, _, _, _, _, _, _) => {
                h986
            }
        };
        let a989 = 0u64;
        nat_rem(a987, a988, a989)
    }, {
        let a991 = v3;
        let a992 = match v1.clone() {
            Adt1::C0(_, _, _, h990, _, _, _, _, _, _, _) => {
                h990
            }
        };
        let a993 = 0u64;
        nat_rem(a991, a992, a993)
    }, {
        let a995 = v3;
        let a996 = match v1.clone() {
            Adt1::C0(_, _, _, h994, _, _, _, _, _, _, _) => {
                h994
            }
        };
        let a997 = 0u64;
        nat_quot(a995, a996, a997)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h998, _, _, _, _, _) => {
            h998
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h999, _, _, _, _) => {
            h999
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1000, _, _, _) => {
            h1000
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1001, _, _) => {
            h1001
        }
    })
}

pub fn f52(v0: u64, v1: Adt1, v2: Adt1) -> R<u64> {
    let v3: u64 = {
        let a1003 = v0;
        let a1004 = match v1.clone() {
            Adt1::C0(_, h1002, _, _, _, _, _, _, _, _, _) => {
                h1002
            }
        };
        let a1005 = 0u64;
        nat_quot(a1003, a1004, a1005)
    };
    let v4: u64 = {
        let a1007 = v3;
        let a1008 = match v1.clone() {
            Adt1::C0(_, _, h1006, _, _, _, _, _, _, _, _) => {
                h1006
            }
        };
        let a1009 = 0u64;
        nat_quot(a1007, a1008, a1009)
    };
    f53({
        let a1015 = {
            let a1011 = v0;
            let a1012 = match v1.clone() {
                Adt1::C0(_, h1010, _, _, _, _, _, _, _, _, _) => {
                    h1010
                }
            };
            let a1013 = 0u64;
            nat_rem(a1011, a1012, a1013)
        };
        let a1016 = match v2.clone() {
            Adt1::C0(_, h1014, _, _, _, _, _, _, _, _, _) => {
                h1014
            }
        };
        let a1017 = 0u64;
        nat_rem(a1015, a1016, a1017)
    }, {
        let a1023 = {
            let a1019 = v3;
            let a1020 = match v1.clone() {
                Adt1::C0(_, _, h1018, _, _, _, _, _, _, _, _) => {
                    h1018
                }
            };
            let a1021 = 0u64;
            nat_rem(a1019, a1020, a1021)
        };
        let a1024 = match v2.clone() {
            Adt1::C0(_, _, h1022, _, _, _, _, _, _, _, _) => {
                h1022
            }
        };
        let a1025 = 0u64;
        nat_rem(a1023, a1024, a1025)
    }, {
        let a1031 = {
            let a1027 = v4;
            let a1028 = match v1.clone() {
                Adt1::C0(_, _, _, h1026, _, _, _, _, _, _, _) => {
                    h1026
                }
            };
            let a1029 = 0u64;
            nat_rem(a1027, a1028, a1029)
        };
        let a1032 = match v2.clone() {
            Adt1::C0(_, _, _, h1030, _, _, _, _, _, _, _) => {
                h1030
            }
        };
        let a1033 = 0u64;
        nat_rem(a1031, a1032, a1033)
    }, {
        let a1039 = {
            let a1035 = v4;
            let a1036 = match v1 {
                Adt1::C0(_, _, _, h1034, _, _, _, _, _, _, _) => {
                    h1034
                }
            };
            let a1037 = 0u64;
            nat_quot(a1035, a1036, a1037)
        };
        let a1040 = match v2.clone() {
            Adt1::C0(_, _, _, _, h1038, _, _, _, _, _, _) => {
                h1038
            }
        };
        let a1041 = 0u64;
        nat_rem(a1039, a1040, a1041)
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h1042, _, _, _, _, _) => {
            h1042
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h1043, _, _, _, _) => {
            h1043
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1044, _, _, _) => {
            h1044
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1045, _, _) => {
            h1045
        }
    })
}

pub fn f53(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a1058 = {
        let a1054 = {
            let a1050 = {
                let a1046 = v0;
                let a1047 = v4;
                nat_mul(a1046, a1047)?
            };
            let a1051 = {
                let a1048 = v1;
                let a1049 = v5;
                nat_mul(a1048, a1049)?
            };
            nat_add(a1050, a1051)?
        };
        let a1055 = {
            let a1052 = v2;
            let a1053 = v6;
            nat_mul(a1052, a1053)?
        };
        nat_add(a1054, a1055)?
    };
    let a1059 = {
        let a1056 = v3;
        let a1057 = v7;
        nat_mul(a1056, a1057)?
    };
    nat_add(a1058, a1059)
}

pub fn f54(v0: u64, v1: Adt1, v2: u64, v3: u64) -> R<Adt3> {
    let v4: u64 = {
        let a1060 = v0;
        let a1061 = 2u64;
        let a1062 = 0u64;
        nat_quot(a1060, a1061, a1062)
    };
    let v5: u64 = f43(match v1.clone() {
        Adt1::C0(h1063, _, _, _, _, _, _, _, _, _, _) => {
            h1063
        }
    });
    let a1067 = {
        let a1064 = v0;
        let a1065 = 2u64;
        let a1066 = 0u64;
        nat_rem(a1064, a1065, a1066)
    };
    let a1068 = 0u64;
    if nat_eq(a1067, a1068) {
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 4u64, f51(v4, v1)?, 2u64, 0u64, 0u64, 2u64, v5, 8u64))
    } else {
        let a1069 = v3;
        let a1070 = v2;
        if if nat_lt(a1069, a1070) {
            let a1073 = v3;
            let a1074 = {
                let a1071 = v5;
                let a1072 = 4u64;
                if nat_eq(a1071, a1072) {
                    2147483648u64
                } else {
                    9223372036854775808u64
                }
            };
            nat_lt(a1073, a1074)
        } else {
            false
        } {
            Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, {
                let a1075 = 3u64;
                let a1076 = v4;
                nat_add(a1075, a1076)?
            }, 8u64, 8u64))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f55(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64) -> R<Adt3> {
    let v5: u64 = {
        let a1078 = 1u64;
        let a1079 = f45(v1.clone(), match v3.clone() {
            Adt1::C0(h1077, _, _, _, _, _, _, _, _, _, _) => {
                h1077
            }
        })?;
        nat_add(a1078, a1079)?
    };
    let v6: u64 = {
        let a1080 = v0;
        let a1081 = v5;
        let a1082 = 0u64;
        nat_rem(a1080, a1081, a1082)
    };
    let v7: u64 = {
        let a1083 = v0;
        let a1084 = v5;
        let a1085 = 0u64;
        nat_quot(a1083, a1084, a1085)
    };
    let v8: u64 = {
        let a1087 = v7;
        let a1088 = match v1.clone() {
            Adt1::C0(_, _, h1086, _, _, _, _, _, _, _, _) => {
                h1086
            }
        };
        let a1089 = 0u64;
        nat_rem(a1087, a1088, a1089)
    };
    let v9: u64 = {
        let a1091 = v7;
        let a1092 = match v1.clone() {
            Adt1::C0(_, _, h1090, _, _, _, _, _, _, _, _) => {
                h1090
            }
        };
        let a1093 = 0u64;
        nat_quot(a1091, a1092, a1093)
    };
    let v10: u64 = {
        let a1095 = v9;
        let a1096 = match v1.clone() {
            Adt1::C0(_, _, _, h1094, _, _, _, _, _, _, _) => {
                h1094
            }
        };
        let a1097 = 0u64;
        nat_rem(a1095, a1096, a1097)
    };
    let v11: u64 = {
        let a1099 = v9;
        let a1100 = match v1.clone() {
            Adt1::C0(_, _, _, h1098, _, _, _, _, _, _, _) => {
                h1098
            }
        };
        let a1101 = 0u64;
        nat_quot(a1099, a1100, a1101)
    };
    let a1102 = v6;
    let a1103 = 0u64;
    if nat_eq(a1102, a1103) {
        let v12: u64 = {
            let a1120 = v8;
            let a1121 = {
                let a1118 = match v2.clone() {
                    Adt1::C0(_, h1104, _, _, _, _, _, _, _, _, _) => {
                        h1104
                    }
                };
                let a1119 = {
                    let a1116 = {
                        let a1106 = v10;
                        let a1107 = match v2.clone() {
                            Adt1::C0(_, _, h1105, _, _, _, _, _, _, _, _) => {
                                h1105
                            }
                        };
                        let a1108 = 0u64;
                        nat_rem(a1106, a1107, a1108)
                    };
                    let a1117 = {
                        let a1114 = match v2.clone() {
                            Adt1::C0(_, _, h1109, _, _, _, _, _, _, _, _) => {
                                h1109
                            }
                        };
                        let a1115 = {
                            let a1111 = v11;
                            let a1112 = match v2 {
                                Adt1::C0(_, _, _, h1110, _, _, _, _, _, _, _) => {
                                    h1110
                                }
                            };
                            let a1113 = 0u64;
                            nat_rem(a1111, a1112, a1113)
                        };
                        nat_mul(a1114, a1115)?
                    };
                    nat_add(a1116, a1117)?
                };
                nat_mul(a1118, a1119)?
            };
            nat_add(a1120, a1121)?
        };
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, {
            let a1122 = 3u64;
            let a1123 = v12;
            nat_add(a1122, a1123)?
        }, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
    } else {
        let a1125 = v4;
        let a1126 = match v3.clone() {
            Adt1::C0(_, _, h1124, _, _, _, _, _, _, _, _) => {
                h1124
            }
        };
        if nat_lt(a1125, a1126) {
            let v13: u64 = {
                let a1131 = {
                    let a1127 = v6;
                    let a1128 = 1u64;
                    nat_sub(a1127, a1128)
                };
                let a1132 = f66(match v1.clone() {
                    Adt1::C0(h1129, _, _, _, _, _, _, _, _, _, _) => {
                        h1129
                    }
                }, match v3.clone() {
                    Adt1::C0(h1130, _, _, _, _, _, _, _, _, _, _) => {
                        h1130
                    }
                });
                let a1133 = 0u64;
                nat_quot(a1131, a1132, a1133)
            };
            let v14: u64 = f53({
                let a1135 = v13;
                let a1136 = f28(match v3.clone() {
                    Adt1::C0(h1134, _, _, _, _, _, _, _, _, _, _) => {
                        h1134
                    }
                });
                nat_mul(a1135, a1136)?
            }, v8, v10, v11, match v1.clone() {
                Adt1::C0(_, _, _, _, _, h1137, _, _, _, _, _) => {
                    h1137
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, h1138, _, _, _, _) => {
                    h1138
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1139, _, _, _) => {
                    h1139
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1140, _, _) => {
                    h1140
                }
            })?;
            let v15: u64 = f53(v13, v4, v10, v11, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h1141, _, _, _, _, _) => {
                    h1141
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h1142, _, _, _, _) => {
                    h1142
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1143, _, _, _) => {
                    h1143
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1144, _, _) => {
                    h1144
                }
            })?;
            let a1146 = 0u64;
            let a1147 = f46(match v3.clone() {
                Adt1::C0(h1145, _, _, _, _, _, _, _, _, _, _) => {
                    h1145
                }
            });
            if nat_lt(a1146, a1147) {
                Ok(Adt3::C6(v14, v15))
            } else {
                Ok(f67(match v1 {
                    Adt1::C0(h1148, _, _, _, _, _, _, _, _, _, _) => {
                        h1148
                    }
                }, match v3 {
                    Adt1::C0(h1149, _, _, _, _, _, _, _, _, _, _) => {
                        h1149
                    }
                }, {
                    let a1150 = v6;
                    let a1151 = 1u64;
                    nat_sub(a1150, a1151)
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
    let a1152 = v0;
    let a1153 = 8u64;
    if nat_eq(a1152, a1153) {
        Adt3::C3(1u64, 6u64, 0u64, 0u64, 1u64, 0u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
    } else {
        let a1154 = v0;
        let a1155 = 9u64;
        if nat_eq(a1154, a1155) {
            Adt3::C3(1u64, 2u64, 0u64, 0u64, 2u64, v1, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
        } else {
            let a1156 = v0;
            let a1157 = 10u64;
            if nat_eq(a1156, a1157) {
                Adt3::C3(5u64, 16u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
            } else {
                let a1158 = v0;
                let a1159 = 11u64;
                if nat_eq(a1158, a1159) {
                    Adt3::C3(1u64, 0u64, 0u64, 0u64, 2u64, 1065353216u64, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
                } else {
                    let a1160 = v0;
                    let a1161 = 12u64;
                    if nat_eq(a1160, a1161) {
                        Adt3::C3(1u64, 3u64, 0u64, 0u64, 1u64, 0u64, 1u64, 2u64, 0u64, 0u64, 4u64, 4u64)
                    } else {
                        let a1162 = v0;
                        let a1163 = 13u64;
                        if nat_eq(a1162, a1163) {
                            Adt3::C3(1u64, 0u64, 0u64, 0u64, 1u64, 1u64, 2u64, 1065353216u64, 0u64, 1u64, 4u64, 4u64)
                        } else {
                            let a1164 = v0;
                            let a1165 = 14u64;
                            if nat_eq(a1164, a1165) {
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
        Adt2::C0(_, _, _, _, _, _, _, h1166, _, _, _, _) => {
            h1166
        }
    };
    let v4: Adt1 = match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h1167, _, _, _) => {
            h1167
        }
    };
    let v5: Adt1 = match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1168, _, _) => {
            h1168
        }
    };
    let v6: u64 = match v2.clone() {
        Adt2::C0(_, _, _, h1169, _, _, _, _, _, _, _, _) => {
            h1169
        }
    };
    let v7: u64 = {
        let a1171 = v1;
        let a1172 = match v5.clone() {
            Adt1::C0(_, h1170, _, _, _, _, _, _, _, _, _) => {
                h1170
            }
        };
        let a1173 = 0u64;
        nat_quot(a1171, a1172, a1173)
    };
    let v8: u64 = f21(match v5.clone() {
        Adt1::C0(h1174, _, _, _, _, _, _, _, _, _, _) => {
            h1174
        }
    });
    let v9: u64 = {
        let a1179 = {
            let a1176 = v1;
            let a1177 = match v5.clone() {
                Adt1::C0(_, h1175, _, _, _, _, _, _, _, _, _) => {
                    h1175
                }
            };
            let a1178 = 0u64;
            nat_rem(a1176, a1177, a1178)
        };
        let a1180 = v8;
        nat_mul(a1179, a1180)?
    };
    let v10: u64 = {
        let a1182 = match v5.clone() {
            Adt1::C0(_, h1181, _, _, _, _, _, _, _, _, _) => {
                h1181
            }
        };
        let a1183 = v8;
        nat_mul(a1182, a1183)?
    };
    let a1184 = v0;
    let a1185 = 0u64;
    if nat_eq(a1184, a1185) {
        Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
            Adt1::C0(h1186, _, _, _, _, _, _, _, _, _, _) => {
                h1186
            }
        }, 0u64, 0u64, {
            let a1194 = {
                let a1190 = {
                    let a1188 = v7;
                    let a1189 = match v3 {
                        Adt1::C0(_, _, _, _, _, _, h1187, _, _, _, _) => {
                            h1187
                        }
                    };
                    nat_mul(a1188, a1189)?
                };
                let a1191 = v9;
                nat_add(a1190, a1191)?
            };
            let a1195 = {
                let a1192 = v6;
                let a1193 = 100u64;
                if nat_eq(a1192, a1193) {
                    v10
                } else {
                    0u64
                }
            };
            nat_add(a1194, a1195)?
        }, 2u64, 0u64, 0u64, 0u64, v8, 4u64))
    } else {
        let a1196 = v0;
        let a1197 = 1u64;
        if nat_eq(a1196, a1197) {
            Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                Adt1::C0(h1198, _, _, _, _, _, _, _, _, _, _) => {
                    h1198
                }
            }, 0u64, {
                let a1199 = v6;
                let a1200 = 98u64;
                if nat_eq(a1199, a1200) {
                    4u64
                } else {
                    0u64
                }
            }, {
                let a1208 = {
                    let a1204 = {
                        let a1202 = v7;
                        let a1203 = match v4 {
                            Adt1::C0(_, _, _, _, _, _, h1201, _, _, _, _) => {
                                h1201
                            }
                        };
                        nat_mul(a1202, a1203)?
                    };
                    let a1205 = v9;
                    nat_add(a1204, a1205)?
                };
                let a1209 = {
                    let a1206 = v6;
                    let a1207 = 99u64;
                    if nat_eq(a1206, a1207) {
                        v10
                    } else {
                        0u64
                    }
                };
                nat_add(a1208, a1209)?
            }, 2u64, 0u64, 0u64, 1u64, v8, 4u64))
        } else {
            let a1210 = v0;
            let a1211 = 2u64;
            if nat_eq(a1210, a1211) {
                Ok(Adt3::C4(f68(match v2 {
                    Adt2::C0(_, _, _, _, h1212, _, _, _, _, _, _, _) => {
                        h1212
                    }
                }, v5.clone(), v1), match v5 {
                    Adt1::C0(h1213, _, _, _, _, _, _, _, _, _, _) => {
                        h1213
                    }
                }, 0u64, 1u64, 0u64))
            } else {
                Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                    Adt1::C0(h1214, _, _, _, _, _, _, _, _, _, _) => {
                        h1214
                    }
                }, 1u64, 0u64, 2u64, 0u64, 3u64, {
                    let a1218 = {
                        let a1216 = v7;
                        let a1217 = match v5 {
                            Adt1::C0(_, _, _, _, _, _, h1215, _, _, _, _) => {
                                h1215
                            }
                        };
                        nat_mul(a1216, a1217)?
                    };
                    let a1219 = v9;
                    nat_add(a1218, a1219)?
                }, 4u64, v8))
            }
        }
    }
}

pub fn f60(v0: u64) -> Adt4 {
    let a1220 = v0;
    let a1221 = 0u64;
    if nat_eq(a1220, a1221) {
        Adt4::C1(Adt5::C0(0u64, 1u64, 4u64))
    } else {
        let a1222 = v0;
        let a1223 = 1u64;
        if nat_eq(a1222, a1223) {
            Adt4::C1(Adt5::C0(1u64, 1u64, 2u64))
        } else {
            let a1224 = v0;
            let a1225 = 2u64;
            if nat_eq(a1224, a1225) {
                Adt4::C1(Adt5::C0(2u64, 32u64, 18u64))
            } else {
                let a1226 = v0;
                let a1227 = 3u64;
                if nat_eq(a1226, a1227) {
                    Adt4::C1(Adt5::C0(3u64, 32u64, 20u64))
                } else {
                    let a1228 = v0;
                    let a1229 = 6u64;
                    if nat_eq(a1228, a1229) {
                        Adt4::C1(Adt5::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1230 = v0;
                        let a1231 = 7u64;
                        if nat_eq(a1230, a1231) {
                            Adt4::C1(Adt5::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1232 = v0;
                            let a1233 = 8u64;
                            if nat_eq(a1232, a1233) {
                                Adt4::C1(Adt5::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1234 = v0;
                                let a1235 = 9u64;
                                if nat_eq(a1234, a1235) {
                                    Adt4::C1(Adt5::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1236 = v0;
                                    let a1237 = 10u64;
                                    if nat_eq(a1236, a1237) {
                                        Adt4::C1(Adt5::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1238 = v0;
                                        let a1239 = 11u64;
                                        if nat_eq(a1238, a1239) {
                                            Adt4::C1(Adt5::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1240 = v0;
                                            let a1241 = 12u64;
                                            if nat_eq(a1240, a1241) {
                                                Adt4::C1(Adt5::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1242 = v0;
                                                let a1243 = 13u64;
                                                if nat_eq(a1242, a1243) {
                                                    Adt4::C1(Adt5::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1244 = v0;
                                                    let a1245 = 14u64;
                                                    if nat_eq(a1244, a1245) {
                                                        Adt4::C1(Adt5::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1246 = v0;
                                                        let a1247 = 15u64;
                                                        if nat_eq(a1246, a1247) {
                                                            Adt4::C1(Adt5::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1248 = v0;
                                                            let a1249 = 16u64;
                                                            if nat_eq(a1248, a1249) {
                                                                Adt4::C1(Adt5::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1250 = v0;
                                                                let a1251 = 17u64;
                                                                if nat_eq(a1250, a1251) {
                                                                    Adt4::C1(Adt5::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1252 = v0;
                                                                    let a1253 = 18u64;
                                                                    if nat_eq(a1252, a1253) {
                                                                        Adt4::C1(Adt5::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1254 = v0;
                                                                        let a1255 = 19u64;
                                                                        if nat_eq(a1254, a1255) {
                                                                            Adt4::C1(Adt5::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1256 = v0;
                                                                            let a1257 = 20u64;
                                                                            if nat_eq(a1256, a1257) {
                                                                                Adt4::C1(Adt5::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1258 = v0;
                                                                                let a1259 = 21u64;
                                                                                if nat_eq(a1258, a1259) {
                                                                                    Adt4::C1(Adt5::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1260 = v0;
                                                                                    let a1261 = 22u64;
                                                                                    if nat_eq(a1260, a1261) {
                                                                                        Adt4::C1(Adt5::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1262 = v0;
                                                                                        let a1263 = 23u64;
                                                                                        if nat_eq(a1262, a1263) {
                                                                                            Adt4::C1(Adt5::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1264 = v0;
                                                                                            let a1265 = 24u64;
                                                                                            if nat_eq(a1264, a1265) {
                                                                                                Adt4::C1(Adt5::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1266 = v0;
                                                                                                let a1267 = 25u64;
                                                                                                if nat_eq(a1266, a1267) {
                                                                                                    Adt4::C1(Adt5::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1268 = v0;
                                                                                                    let a1269 = 26u64;
                                                                                                    if nat_eq(a1268, a1269) {
                                                                                                        Adt4::C1(Adt5::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1270 = v0;
                                                                                                        let a1271 = 27u64;
                                                                                                        if nat_eq(a1270, a1271) {
                                                                                                            Adt4::C1(Adt5::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1272 = v0;
                                                                                                            let a1273 = 28u64;
                                                                                                            if nat_eq(a1272, a1273) {
                                                                                                                Adt4::C1(Adt5::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1274 = v0;
                                                                                                                let a1275 = 29u64;
                                                                                                                if nat_eq(a1274, a1275) {
                                                                                                                    Adt4::C1(Adt5::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1276 = v0;
                                                                                                                    let a1277 = 30u64;
                                                                                                                    if nat_eq(a1276, a1277) {
                                                                                                                        Adt4::C1(Adt5::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1278 = v0;
                                                                                                                        let a1279 = 34u64;
                                                                                                                        if nat_eq(a1278, a1279) {
                                                                                                                            Adt4::C1(Adt5::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1280 = v0;
                                                                                                                            let a1281 = 35u64;
                                                                                                                            if nat_eq(a1280, a1281) {
                                                                                                                                Adt4::C1(Adt5::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1282 = v0;
                                                                                                                                let a1283 = 39u64;
                                                                                                                                if nat_eq(a1282, a1283) {
                                                                                                                                    Adt4::C1(Adt5::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1284 = v0;
                                                                                                                                    let a1285 = 40u64;
                                                                                                                                    if nat_eq(a1284, a1285) {
                                                                                                                                        Adt4::C1(Adt5::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1286 = v0;
                                                                                                                                        let a1287 = 41u64;
                                                                                                                                        if nat_eq(a1286, a1287) {
                                                                                                                                            Adt4::C1(Adt5::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1288 = v0;
                                                                                                                                            let a1289 = 42u64;
                                                                                                                                            if nat_eq(a1288, a1289) {
                                                                                                                                                Adt4::C1(Adt5::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1290 = v0;
                                                                                                                                                let a1291 = 43u64;
                                                                                                                                                if nat_eq(a1290, a1291) {
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
        let a1294 = match v0.clone() {
            Adt1::C0(h1292, _, _, _, _, _, _, _, _, _, _) => {
                h1292
            }
        };
        let a1295 = match v1.clone() {
            Adt1::C0(h1293, _, _, _, _, _, _, _, _, _, _) => {
                h1293
            }
        };
        nat_eq(a1294, a1295)
    } else {
        false
    } {
        let a1298 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1296) => {
                h1296
            }
        };
        let a1299 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h1297) => {
                h1297
            }
        };
        nat_eq(a1298, a1299)
    } else {
        false
    } {
        let a1302 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h1300, _, _, _, _, _) => {
                h1300
            }
        };
        let a1303 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1301, _, _, _, _, _) => {
                h1301
            }
        };
        nat_eq(a1302, a1303)
    } else {
        false
    } {
        let a1306 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h1304, _, _, _, _) => {
                h1304
            }
        };
        let a1307 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1305, _, _, _, _) => {
                h1305
            }
        };
        nat_eq(a1306, a1307)
    } else {
        false
    } {
        let a1310 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1308, _, _, _) => {
                h1308
            }
        };
        let a1311 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, h1309, _, _, _) => {
                h1309
            }
        };
        nat_eq(a1310, a1311)
    } else {
        false
    } {
        let a1314 = match v0 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1312, _, _) => {
                h1312
            }
        };
        let a1315 = match v1 {
            Adt1::C0(_, _, _, _, _, _, _, _, h1313, _, _) => {
                h1313
            }
        };
        nat_eq(a1314, a1315)
    } else {
        false
    }
}

pub fn f62(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a1316 = v1;
    let a1317 = 1u64;
    if if if nat_le(a1316, a1317) {
        true
    } else {
        let a1318 = v0;
        let a1319 = v2;
        nat_le(a1318, a1319)
    } {
        let a1320 = v3;
        let a1321 = 1u64;
        if nat_le(a1320, a1321) {
            true
        } else {
            let a1328 = {
                let a1326 = v0;
                let a1327 = {
                    let a1324 = {
                        let a1322 = v1;
                        let a1323 = 1u64;
                        nat_sub(a1322, a1323)
                    };
                    let a1325 = v2;
                    nat_mul(a1324, a1325)?
                };
                nat_add(a1326, a1327)?
            };
            let a1329 = v4;
            nat_le(a1328, a1329)
        }
    } else {
        false
    } {
        let a1330 = v5;
        let a1331 = 1u64;
        if nat_le(a1330, a1331) {
            Ok(true)
        } else {
            let a1344 = {
                let a1342 = {
                    let a1336 = v0;
                    let a1337 = {
                        let a1334 = {
                            let a1332 = v1;
                            let a1333 = 1u64;
                            nat_sub(a1332, a1333)
                        };
                        let a1335 = v2;
                        nat_mul(a1334, a1335)?
                    };
                    nat_add(a1336, a1337)?
                };
                let a1343 = {
                    let a1340 = {
                        let a1338 = v3;
                        let a1339 = 1u64;
                        nat_sub(a1338, a1339)
                    };
                    let a1341 = v4;
                    nat_mul(a1340, a1341)?
                };
                nat_add(a1342, a1343)?
            };
            let a1345 = v6;
            Ok(nat_le(a1344, a1345))
        }
    } else {
        Ok(false)
    }
}

pub fn f63(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1346 = v0;
    let a1347 = 0u64;
    if if if nat_eq(a1346, a1347) {
        true
    } else {
        let a1348 = v1;
        let a1349 = 0u64;
        nat_eq(a1348, a1349)
    } {
        true
    } else {
        let a1350 = v2;
        let a1351 = 0u64;
        nat_eq(a1350, a1351)
    } {
        true
    } else {
        let a1352 = v3;
        let a1353 = 0u64;
        nat_eq(a1352, a1353)
    }
}

pub fn f64(v0: u64, v1: u64, v2: u64) -> bool {
    let a1354 = v0;
    let a1355 = 1u64;
    if nat_le(a1354, a1355) {
        true
    } else {
        let a1359 = {
            let a1356 = v1;
            let a1357 = v2;
            let a1358 = 1u64;
            nat_rem(a1356, a1357, a1358)
        };
        let a1360 = 0u64;
        nat_eq(a1359, a1360)
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
    let a1361 = v0;
    let a1362 = 1u64;
    if if nat_eq(a1361, a1362) {
        let a1363 = v1;
        let a1364 = 43u64;
        nat_eq(a1363, a1364)
    } else {
        false
    } {
        2u64
    } else {
        1u64
    }
}

pub fn f67(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt3 {
    let a1365 = f66(v0, v1);
    let a1366 = 2u64;
    if nat_eq(a1365, a1366) {
        let a1370 = {
            let a1367 = v2;
            let a1368 = 2u64;
            let a1369 = 0u64;
            nat_rem(a1367, a1368, a1369)
        };
        let a1371 = 0u64;
        if nat_eq(a1370, a1371) {
            Adt3::C3(0u64, 0u64, v0, 0u64, 0u64, v3, 2u64, 0u64, 1u64, 0u64, f21(v0), 8u64)
        } else {
            Adt3::C3(0u64, 0u64, 0u64, v1, 1u64, 0u64, 2u64, 0u64, 3u64, v4, 8u64, f42(v1))
        }
    } else {
        Adt3::C3(0u64, 0u64, v0, v1, 0u64, v3, 2u64, 0u64, 3u64, v4, f21(v0), f42(v1))
    }
}

pub fn f68(v0: u64, v1: Adt1, v2: u64) -> u64 {
    let a1372 = v0;
    let a1373 = 2u64;
    if if nat_eq(a1372, a1373) {
        let a1375 = match v1.clone() {
            Adt1::C0(h1374, _, _, _, _, _, _, _, _, _, _) => {
                h1374
            }
        };
        let a1376 = 0u64;
        if nat_eq(a1375, a1376) {
            let a1388 = {
                let a1378 = v2;
                let a1379 = match v1.clone() {
                    Adt1::C0(_, h1377, _, _, _, _, _, _, _, _, _) => {
                        h1377
                    }
                };
                let a1380 = 0u64;
                nat_rem(a1378, a1379, a1380)
            };
            let a1389 = {
                let a1386 = match v1.clone() {
                    Adt1::C0(_, h1381, _, _, _, _, _, _, _, _, _) => {
                        h1381
                    }
                };
                let a1387 = {
                    let a1383 = match v1 {
                        Adt1::C0(_, h1382, _, _, _, _, _, _, _, _, _) => {
                            h1382
                        }
                    };
                    let a1384 = 4u64;
                    let a1385 = 0u64;
                    nat_rem(a1383, a1384, a1385)
                };
                nat_sub(a1386, a1387)
            };
            nat_lt(a1388, a1389)
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

pub fn run(p0: Adt0) -> R<Adt3> {
    f0(p0)
}
