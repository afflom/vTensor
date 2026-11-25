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
    let a485 = v0;
    let a486 = match v1.clone() {
        Adt2::C0(h484, _, _, _, _, _, _, _, _, _, _, _) => {
            h484
        }
    };
    if nat_lt(a485, a486) {
        f58({
            let a487 = v0;
            let a488 = 4u64;
            let a489 = 0u64;
            nat_rem(a487, a488, a489)
        }, {
            let a490 = v0;
            let a491 = 4u64;
            let a492 = 0u64;
            nat_quot(a490, a491, a492)
        }, v1)
    } else {
        Ok(Adt3::C1)
    }
}

pub fn f17(v0: u64, v1: Adt2, v2: u64) -> R<Adt3> {
    let v3: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h493, _, _, _, _) => {
            h493
        }
    };
    let v4: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h494, _, _, _) => {
            h494
        }
    };
    let v5: Adt1 = match v1.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h495, _, _) => {
            h495
        }
    };
    let v6: u64 = {
        let a497 = match v1.clone() {
            Adt2::C0(_, h496, _, _, _, _, _, _, _, _, _, _) => {
                h496
            }
        };
        let a498 = 3u64;
        if nat_eq(a497, a498) {
            0u64
        } else {
            let a499 = f39(v3.clone())?;
            let a500 = 2u64;
            nat_mul(a499, a500)?
        }
    };
    let a502 = match v1.clone() {
        Adt2::C0(h501, _, _, _, _, _, _, _, _, _, _, _) => {
            h501
        }
    };
    let a503 = v0;
    if nat_le(a502, a503) {
        Ok(Adt3::C1)
    } else {
        let a504 = v0;
        let a505 = v6;
        if nat_lt(a504, a505) {
            let v7: u64 = {
                let a506 = v0;
                let a507 = 2u64;
                let a508 = 0u64;
                nat_quot(a506, a507, a508)
            };
            let v8: u64 = {
                let a513 = 3u64;
                let a514 = {
                    let a510 = f50(v7, v5.clone())?;
                    let a511 = f20(match v5 {
                        Adt1::C0(h509, _, _, _, _, _, _, _, _, _, _) => {
                            h509
                        }
                    });
                    let a512 = 0u64;
                    nat_quot(a510, a511, a512)
                };
                nat_add(a513, a514)?
            };
            let a518 = {
                let a515 = v0;
                let a516 = 2u64;
                let a517 = 0u64;
                nat_rem(a515, a516, a517)
            };
            let a519 = 0u64;
            if nat_eq(a518, a519) {
                Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, v8, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
            } else {
                let a520 = v2;
                let a521 = 0u64;
                if nat_eq(a520, a521) {
                    Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 2u64, 1u64, 2u64, 0u64, 0u64, v8, 8u64, 8u64))
                } else {
                    Ok(Adt3::C1)
                }
            }
        } else {
            let v9: u64 = {
                let a522 = v0;
                let a523 = v6;
                nat_sub(a522, a523)
            };
            let v10: u64 = f40(match v3.clone() {
                Adt1::C0(h524, _, _, _, _, _, _, _, _, _, _) => {
                    h524
                }
            }, match v4.clone() {
                Adt1::C0(h525, _, _, _, _, _, _, _, _, _, _) => {
                    h525
                }
            }, match v5.clone() {
                Adt1::C0(h526, _, _, _, _, _, _, _, _, _, _) => {
                    h526
                }
            })?;
            let v11: u64 = {
                let a527 = v9;
                let a528 = v10;
                let a529 = 0u64;
                nat_quot(a527, a528, a529)
            };
            let v12: u64 = {
                let a530 = v9;
                let a531 = v10;
                let a532 = 0u64;
                nat_rem(a530, a531, a532)
            };
            let v13: u64 = {
                let a534 = match v3.clone() {
                    Adt1::C0(h533, _, _, _, _, _, _, _, _, _, _) => {
                        h533
                    }
                };
                let a535 = 0u64;
                if nat_eq(a534, a535) {
                    0u64
                } else {
                    1u64
                }
            };
            let v14: u64 = {
                let a537 = match v4.clone() {
                    Adt1::C0(h536, _, _, _, _, _, _, _, _, _, _) => {
                        h536
                    }
                };
                let a538 = 0u64;
                if nat_eq(a537, a538) {
                    0u64
                } else {
                    1u64
                }
            };
            let a539 = v12;
            let a540 = v13;
            if nat_lt(a539, a540) {
                Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
                    Adt1::C0(h541, _, _, _, _, _, _, _, _, _, _) => {
                        h541
                    }
                }, 0u64, 0u64, f50(v11, v3.clone())?, 2u64, 0u64, 0u64, 0u64, f20(match v3 {
                    Adt1::C0(h542, _, _, _, _, _, _, _, _, _, _) => {
                        h542
                    }
                }), 4u64))
            } else {
                let a545 = v12;
                let a546 = {
                    let a543 = v13;
                    let a544 = v14;
                    nat_add(a543, a544)?
                };
                if nat_lt(a545, a546) {
                    Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                        Adt1::C0(h547, _, _, _, _, _, _, _, _, _, _) => {
                            h547
                        }
                    }, 0u64, 4u64, f51(v11, v3, v4.clone())?, 2u64, 0u64, 0u64, 1u64, f20(match v4 {
                        Adt1::C0(h548, _, _, _, _, _, _, _, _, _, _) => {
                            h548
                        }
                    }), 4u64))
                } else {
                    let a551 = v12;
                    let a552 = {
                        let a549 = v13;
                        let a550 = v14;
                        nat_add(a549, a550)?
                    };
                    if nat_eq(a551, a552) {
                        Ok(Adt3::C3(1u64, match v1 {
                            Adt2::C0(_, _, _, h553, _, _, _, _, _, _, _, _) => {
                                h553
                            }
                        }, 0u64, 0u64, {
                            let a554 = v13;
                            let a555 = 0u64;
                            if nat_eq(a554, a555) {
                                0u64
                            } else {
                                1u64
                            }
                        }, {
                            let a556 = v13;
                            let a557 = 0u64;
                            if nat_eq(a556, a557) {
                                f50(v11, v3.clone())?
                            } else {
                                0u64
                            }
                        }, {
                            let a558 = v14;
                            let a559 = 0u64;
                            if nat_eq(a558, a559) {
                                4u64
                            } else {
                                1u64
                            }
                        }, {
                            let a560 = v14;
                            let a561 = 0u64;
                            if nat_eq(a560, a561) {
                                f51(v11, v3, v4)?
                            } else {
                                1u64
                            }
                        }, {
                            let a563 = match v5.clone() {
                                Adt1::C0(h562, _, _, _, _, _, _, _, _, _, _) => {
                                    h562
                                }
                            };
                            let a564 = 0u64;
                            if nat_eq(a563, a564) {
                                3u64
                            } else {
                                0u64
                            }
                        }, {
                            let a566 = match v5.clone() {
                                Adt1::C0(h565, _, _, _, _, _, _, _, _, _, _) => {
                                    h565
                                }
                            };
                            let a567 = 0u64;
                            if nat_eq(a566, a567) {
                                f50(v11, v5)?
                            } else {
                                0u64
                            }
                        }, 4u64, 4u64))
                    } else {
                        Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                            Adt1::C0(h568, _, _, _, _, _, _, _, _, _, _) => {
                                h568
                            }
                        }, 1u64, 0u64, 2u64, 0u64, 3u64, f50(v11, v5.clone())?, 4u64, f20(match v5 {
                            Adt1::C0(h569, _, _, _, _, _, _, _, _, _, _) => {
                                h569
                            }
                        })))
                    }
                }
            }
        }
    }
}

pub fn f18(v0: u64) -> bool {
    let a570 = 6u64;
    let a571 = v0;
    if if if if nat_le(a570, a571) {
        let a572 = v0;
        let a573 = 23u64;
        nat_le(a572, a573)
    } else {
        false
    } {
        true
    } else {
        let a574 = v0;
        let a575 = 2u64;
        if nat_eq(a574, a575) {
            true
        } else {
            let a576 = v0;
            let a577 = 3u64;
            nat_eq(a576, a577)
        }
    } {
        true
    } else {
        let a578 = v0;
        let a579 = 29u64;
        nat_eq(a578, a579)
    } {
        true
    } else {
        let a580 = 34u64;
        let a581 = v0;
        if if nat_le(a580, a581) {
            let a582 = v0;
            let a583 = 35u64;
            nat_le(a582, a583)
        } else {
            false
        } {
            true
        } else {
            let a584 = 39u64;
            let a585 = v0;
            if nat_le(a584, a585) {
                let a586 = v0;
                let a587 = 42u64;
                nat_le(a586, a587)
            } else {
                false
            }
        }
    }
}

pub fn f19(v0: u64, v1: u64, v2: u64) -> bool {
    let a588 = v0;
    let a589 = 0u64;
    if if if nat_eq(a588, a589) {
        let a590 = v1;
        let a591 = 0u64;
        nat_eq(a590, a591)
    } else {
        false
    } {
        let a592 = v2;
        let a593 = 0u64;
        nat_eq(a592, a593)
    } else {
        false
    } {
        true
    } else {
        let a594 = v0;
        let a595 = 1u64;
        if if nat_eq(a594, a595) {
            true
        } else {
            let a596 = v0;
            let a597 = 30u64;
            nat_eq(a596, a597)
        } {
            let a598 = v1;
            let a599 = v0;
            if if nat_eq(a598, a599) {
                let a600 = v2;
                let a601 = v0;
                nat_eq(a600, a601)
            } else {
                false
            } {
                true
            } else {
                let a602 = v1;
                let a603 = 0u64;
                if nat_eq(a602, a603) {
                    let a604 = v2;
                    let a605 = v0;
                    if nat_eq(a604, a605) {
                        true
                    } else {
                        let a606 = v2;
                        let a607 = 0u64;
                        nat_eq(a606, a607)
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
    let a608 = v0;
    let a609 = 43u64;
    if nat_eq(a608, a609) {
        1u64
    } else {
        f21(v0)
    }
}

pub fn f21(v0: u64) -> u64 {
    let a610 = v0;
    let a611 = 0u64;
    if nat_eq(a610, a611) {
        4u64
    } else {
        let a612 = v0;
        let a613 = 1u64;
        if if nat_eq(a612, a613) {
            true
        } else {
            let a614 = v0;
            let a615 = 30u64;
            nat_eq(a614, a615)
        } {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f22(v0: u64) -> bool {
    let a616 = 0u64;
    let a617 = f20(v0);
    if nat_lt(a616, a617) {
        true
    } else {
        let a618 = 24u64;
        let a619 = v0;
        if nat_le(a618, a619) {
            let a620 = v0;
            let a621 = 28u64;
            nat_le(a620, a621)
        } else {
            false
        }
    }
}

pub fn f23(v0: Adt1, v1: Adt1) -> bool {
    if f32(v1.clone()) {
        f32(v0)
    } else {
        let a627 = {
            let a624 = match v0.clone() {
                Adt1::C0(_, h622, _, _, _, _, _, _, _, _, _) => {
                    h622
                }
            };
            let a625 = match v1.clone() {
                Adt1::C0(_, h623, _, _, _, _, _, _, _, _, _) => {
                    h623
                }
            };
            let a626 = 1u64;
            nat_rem(a624, a625, a626)
        };
        let a628 = 0u64;
        if if if nat_eq(a627, a628) {
            let a634 = {
                let a631 = match v0.clone() {
                    Adt1::C0(_, _, h629, _, _, _, _, _, _, _, _) => {
                        h629
                    }
                };
                let a632 = match v1.clone() {
                    Adt1::C0(_, _, h630, _, _, _, _, _, _, _, _) => {
                        h630
                    }
                };
                let a633 = 1u64;
                nat_rem(a631, a632, a633)
            };
            let a635 = 0u64;
            nat_eq(a634, a635)
        } else {
            false
        } {
            let a641 = {
                let a638 = match v0.clone() {
                    Adt1::C0(_, _, _, h636, _, _, _, _, _, _, _) => {
                        h636
                    }
                };
                let a639 = match v1.clone() {
                    Adt1::C0(_, _, _, h637, _, _, _, _, _, _, _) => {
                        h637
                    }
                };
                let a640 = 1u64;
                nat_rem(a638, a639, a640)
            };
            let a642 = 0u64;
            nat_eq(a641, a642)
        } else {
            false
        } {
            let a648 = {
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
                let a647 = 1u64;
                nat_rem(a645, a646, a647)
            };
            let a649 = 0u64;
            nat_eq(a648, a649)
        } else {
            false
        }
    }
}

pub fn f24(v0: Adt1, v1: Adt1) -> bool {
    let a652 = match v0.clone() {
        Adt1::C0(_, h650, _, _, _, _, _, _, _, _, _) => {
            h650
        }
    };
    let a653 = match v1.clone() {
        Adt1::C0(_, h651, _, _, _, _, _, _, _, _, _) => {
            h651
        }
    };
    if if if nat_eq(a652, a653) {
        let a656 = match v0.clone() {
            Adt1::C0(_, _, h654, _, _, _, _, _, _, _, _) => {
                h654
            }
        };
        let a657 = match v1.clone() {
            Adt1::C0(_, _, h655, _, _, _, _, _, _, _, _) => {
                h655
            }
        };
        nat_eq(a656, a657)
    } else {
        false
    } {
        let a660 = match v0.clone() {
            Adt1::C0(_, _, _, h658, _, _, _, _, _, _, _) => {
                h658
            }
        };
        let a661 = match v1.clone() {
            Adt1::C0(_, _, _, h659, _, _, _, _, _, _, _) => {
                h659
            }
        };
        nat_eq(a660, a661)
    } else {
        false
    } {
        let a664 = match v0 {
            Adt1::C0(_, _, _, _, h662, _, _, _, _, _, _) => {
                h662
            }
        };
        let a665 = match v1 {
            Adt1::C0(_, _, _, _, h663, _, _, _, _, _, _) => {
                h663
            }
        };
        nat_eq(a664, a665)
    } else {
        false
    }
}

pub fn f25(v0: u64, v1: u64, v2: Adt1, v3: u64) -> bool {
    let v4: u64 = match v2.clone() {
        Adt1::C0(h666, _, _, _, _, _, _, _, _, _, _) => {
            h666
        }
    };
    let a667 = v1;
    let a668 = v3;
    if if nat_eq(a667, a668) {
        true
    } else {
        let a669 = v0;
        let a670 = 0u64;
        if nat_eq(a669, a670) {
            let a671 = v4;
            let a672 = 0u64;
            if if nat_eq(a671, a672) {
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
            let a674 = match v2.clone() {
                Adt1::C0(_, _, _, _, _, h673, _, _, _, _, _) => {
                    h673
                }
            };
            let a675 = f27(v4);
            nat_eq(a674, a675)
        } else {
            false
        } {
            let a680 = {
                let a677 = match v2 {
                    Adt1::C0(_, h676, _, _, _, _, _, _, _, _, _) => {
                        h676
                    }
                };
                let a678 = f29(v4);
                let a679 = 1u64;
                nat_rem(a677, a678, a679)
            };
            let a681 = 0u64;
            nat_eq(a680, a681)
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
        let a682 = v0;
        let a683 = v1;
        if if nat_eq(a682, a683) {
            let a684 = 0u64;
            let a685 = f20(v0);
            nat_lt(a684, a685)
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
        let m692 = f59(v0);
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
        f28(v0)
    }
}

pub fn f28(v0: u64) -> u64 {
    if f18(v0) {
        let m694 = f59(v0);
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
        let a696 = v0;
        let a697 = 26u64;
        if nat_eq(a696, a697) {
            4u64
        } else {
            f20(v0)
        }
    }
}

pub fn f29(v0: u64) -> u64 {
    if f18(v0) {
        let m698 = f59(v0);
        match m698 {
            Adt4::C0 => {
                1u64
            }
            Adt4::C1(v1) => {
                match v1 {
                    Adt5::C0(_, h699, _) => {
                        h699
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
        Ok(f60(v0, v1))
    }
}

pub fn f31(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f61(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f61(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f61(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f61(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f61(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f61(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f32(v0: Adt1) -> bool {
    f62(match v0.clone() {
        Adt1::C0(_, h700, _, _, _, _, _, _, _, _, _) => {
            h700
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h701, _, _, _, _, _, _, _, _) => {
            h701
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h702, _, _, _, _, _, _, _) => {
            h702
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, h703, _, _, _, _, _, _) => {
            h703
        }
    })
}

pub fn f33(v0: u64) -> Option<u64> {
    let a704 = v0;
    let a705 = 16u64;
    if if if nat_eq(a704, a705) {
        true
    } else {
        let a706 = v0;
        let a707 = 17u64;
        nat_eq(a706, a707)
    } {
        true
    } else {
        let a708 = v0;
        let a709 = 19u64;
        nat_eq(a708, a709)
    } {
        Some(1065353216u64)
    } else {
        None::<u64>
    }
}

pub fn f34(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1) -> R<bool> {
    let a711 = match v1.clone() {
        Adt1::C0(h710, _, _, _, _, _, _, _, _, _, _) => {
            h710
        }
    };
    let a712 = 43u64;
    if nat_eq(a711, a712) {
        let a713 = v0;
        let a714 = 0u64;
        if if if if if if nat_eq(a713, a714) {
            let a716 = match v2.clone() {
                Adt1::C0(h715, _, _, _, _, _, _, _, _, _, _) => {
                    h715
                }
            };
            let a717 = 0u64;
            nat_eq(a716, a717)
        } else {
            false
        } {
            let a719 = match v3.clone() {
                Adt1::C0(h718, _, _, _, _, _, _, _, _, _, _) => {
                    h718
                }
            };
            let a720 = 0u64;
            if nat_eq(a719, a720) {
                true
            } else {
                let a722 = match v3.clone() {
                    Adt1::C0(h721, _, _, _, _, _, _, _, _, _, _) => {
                        h721
                    }
                };
                let a723 = 43u64;
                nat_eq(a722, a723)
            }
        } else {
            false
        } {
            let a725 = match v3.clone() {
                Adt1::C0(h724, _, _, _, _, _, _, _, _, _, _) => {
                    h724
                }
            };
            let a726 = 43u64;
            if nat_eq(a725, a726) {
                f23(v1.clone(), v2.clone())
            } else {
                f24(v1.clone(), v2.clone())
            }
        } else {
            false
        } {
            let a728 = match v2 {
                Adt1::C0(_, _, _, _, _, h727, _, _, _, _, _) => {
                    h727
                }
            };
            let a729 = 4u64;
            nat_eq(a728, a729)
        } else {
            false
        } {
            let a732 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, h730, _, _, _, _, _) => {
                    h730
                }
            };
            let a733 = match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h731, _, _, _, _) => {
                    h731
                }
            };
            if nat_le(a732, a733) {
                let a736 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, h734, _, _, _, _) => {
                        h734
                    }
                };
                let a737 = match v3.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h735, _, _, _) => {
                        h735
                    }
                };
                if nat_le(a736, a737) {
                    let a740 = match v3.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, h738, _, _, _) => {
                            h738
                        }
                    };
                    let a741 = match v3 {
                        Adt1::C0(_, _, _, _, _, _, _, _, h739, _, _) => {
                            h739
                        }
                    };
                    nat_le(a740, a741)
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        } {
            let a749 = {
                let a747 = match v1.clone() {
                    Adt1::C0(_, _, h742, _, _, _, _, _, _, _, _) => {
                        h742
                    }
                };
                let a748 = {
                    let a745 = match v1.clone() {
                        Adt1::C0(_, _, _, h743, _, _, _, _, _, _, _) => {
                            h743
                        }
                    };
                    let a746 = match v1 {
                        Adt1::C0(_, _, _, _, h744, _, _, _, _, _, _) => {
                            h744
                        }
                    };
                    nat_mul(a745, a746)?
                };
                nat_mul(a747, a748)?
            };
            let a750 = 2147483647u64;
            Ok(nat_le(a749, a750))
        } else {
            Ok(false)
        }
    } else {
        if f23(v1.clone(), v2.clone()) {
            Ok(f19(match v1 {
                Adt1::C0(h751, _, _, _, _, _, _, _, _, _, _) => {
                    h751
                }
            }, match v2 {
                Adt1::C0(h752, _, _, _, _, _, _, _, _, _, _) => {
                    h752
                }
            }, match v3 {
                Adt1::C0(h753, _, _, _, _, _, _, _, _, _, _) => {
                    h753
                }
            }))
        } else {
            Ok(false)
        }
    }
}

pub fn f35(v0: Adt1) -> bool {
    let v1: u64 = f20(match v0.clone() {
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
        f63(match v0.clone() {
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
        f64(match v0.clone() {
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
        if f43(v1.clone()) {
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
    f31({
        let a786 = match v0.clone() {
            Adt1::C0(_, h784, _, _, _, _, _, _, _, _, _) => {
                h784
            }
        };
        let a787 = f20(match v0.clone() {
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
    let a854 = v0;
    let a855 = 26u64;
    if nat_eq(a854, a855) {
        4u64
    } else {
        let a856 = v0;
        let a857 = 27u64;
        if nat_eq(a856, a857) {
            8u64
        } else {
            0u64
        }
    }
}

pub fn f43(v0: Adt1) -> bool {
    let a859 = match v0.clone() {
        Adt1::C0(_, h858, _, _, _, _, _, _, _, _, _) => {
            h858
        }
    };
    let a860 = 1u64;
    if nat_eq(a859, a860) {
        true
    } else {
        let a863 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, h861, _, _, _, _, _) => {
                h861
            }
        };
        let a864 = f20(match v0 {
            Adt1::C0(h862, _, _, _, _, _, _, _, _, _, _) => {
                h862
            }
        });
        nat_eq(a863, a864)
    }
}

pub fn f44(v0: Adt1, v1: u64) -> R<u64> {
    let a870 = {
        let a866 = match v0.clone() {
            Adt1::C0(_, h865, _, _, _, _, _, _, _, _, _) => {
                h865
            }
        };
        let a867 = f29(v1);
        let a868 = 0u64;
        nat_quot(a866, a867, a868)
    };
    let a871 = f65(match v0 {
        Adt1::C0(h869, _, _, _, _, _, _, _, _, _, _) => {
            h869
        }
    }, v1);
    nat_mul(a870, a871)
}

pub fn f45(v0: u64) -> u64 {
    if if f18(v0) {
        true
    } else {
        let a872 = v0;
        let a873 = 26u64;
        nat_eq(a872, a873)
    } {
        f29(v0)
    } else {
        0u64
    }
}

pub fn f46(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a875 = match v0.clone() {
            Adt1::C0(_, h874, _, _, _, _, _, _, _, _, _) => {
                h874
            }
        };
        let a876 = 4u64;
        nat_mul(a875, a876)?
    };
    let v2: u64 = {
        let a879 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h877, _, _, _, _) => {
                h877
            }
        };
        let a880 = match v0.clone() {
            Adt1::C0(_, _, h878, _, _, _, _, _, _, _, _) => {
                h878
            }
        };
        nat_mul(a879, a880)?
    };
    let v3: u64 = {
        let a882 = v2;
        let a883 = match v0.clone() {
            Adt1::C0(_, _, _, h881, _, _, _, _, _, _, _) => {
                h881
            }
        };
        nat_mul(a882, a883)?
    };
    let a885 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, h884, _, _, _, _, _) => {
            h884
        }
    };
    let a886 = 4u64;
    if if if if nat_eq(a885, a886) {
        let a891 = {
            let a888 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h887) => {
                    h887
                }
            };
            let a889 = 4u64;
            let a890 = 1u64;
            nat_rem(a888, a889, a890)
        };
        let a892 = 0u64;
        nat_eq(a891, a892)
    } else {
        false
    } {
        let a900 = {
            let a898 = match v0.clone() {
                Adt1::C0(_, _, h893, _, _, _, _, _, _, _, _) => {
                    h893
                }
            };
            let a899 = {
                let a896 = match v0.clone() {
                    Adt1::C0(_, _, _, h894, _, _, _, _, _, _, _) => {
                        h894
                    }
                };
                let a897 = match v0.clone() {
                    Adt1::C0(_, _, _, _, h895, _, _, _, _, _, _) => {
                        h895
                    }
                };
                nat_mul(a896, a897)?
            };
            nat_mul(a898, a899)?
        };
        let a901 = 1u64;
        if nat_le(a900, a901) {
            true
        } else {
            let a903 = v1;
            let a904 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h902, _, _, _, _) => {
                    h902
                }
            };
            if nat_le(a903, a904) {
                let a909 = {
                    let a906 = match v0.clone() {
                        Adt1::C0(_, _, _, _, _, _, h905, _, _, _, _) => {
                            h905
                        }
                    };
                    let a907 = 4u64;
                    let a908 = 1u64;
                    nat_rem(a906, a907, a908)
                };
                let a910 = 0u64;
                nat_eq(a909, a910)
            } else {
                false
            }
        }
    } else {
        false
    } {
        let a912 = match v0.clone() {
            Adt1::C0(_, _, _, h911, _, _, _, _, _, _, _) => {
                h911
            }
        };
        let a913 = 1u64;
        if nat_le(a912, a913) {
            true
        } else {
            let a915 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h914, _, _, _) => {
                    h914
                }
            };
            let a916 = v2;
            nat_eq(a915, a916)
        }
    } else {
        false
    } {
        let a918 = match v0.clone() {
            Adt1::C0(_, _, _, _, h917, _, _, _, _, _, _) => {
                h917
            }
        };
        let a919 = 1u64;
        if nat_le(a918, a919) {
            Ok(true)
        } else {
            let a921 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, h920, _, _) => {
                    h920
                }
            };
            let a922 = v3;
            Ok(nat_eq(a921, a922))
        }
    } else {
        Ok(false)
    }
}

pub fn f47(v0: Adt1, v1: Adt1) -> R<bool> {
    let a924 = match v0.clone() {
        Adt1::C0(_, _, _, _, _, _, _, _, _, h923, _) => {
            h923
        }
    };
    let a925 = 0u64;
    if nat_eq(a924, a925) {
        Ok(true)
    } else {
        let a927 = match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, _, _, _, h926, _) => {
                h926
            }
        };
        let a928 = 0u64;
        if nat_eq(a927, a928) {
            Ok(true)
        } else {
            let a930 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h929) => {
                    h929
                }
            };
            let a931 = 0u64;
            if nat_eq(a930, a931) {
                Ok(true)
            } else {
                let a933 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h932) => {
                        h932
                    }
                };
                let a934 = 0u64;
                if nat_eq(a933, a934) {
                    Ok(true)
                } else {
                    let a940 = {
                        let a937 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h935) => {
                                h935
                            }
                        };
                        let a938 = match v0.clone() {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, h936, _) => {
                                h936
                            }
                        };
                        nat_add(a937, a938)?
                    };
                    let a941 = match v1.clone() {
                        Adt1::C0(_, _, _, _, _, _, _, _, _, _, h939) => {
                            h939
                        }
                    };
                    if nat_le(a940, a941) {
                        Ok(true)
                    } else {
                        let a947 = {
                            let a944 = match v1.clone() {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, _, h942) => {
                                    h942
                                }
                            };
                            let a945 = match v1 {
                                Adt1::C0(_, _, _, _, _, _, _, _, _, h943, _) => {
                                    h943
                                }
                            };
                            nat_add(a944, a945)?
                        };
                        let a948 = match v0 {
                            Adt1::C0(_, _, _, _, _, _, _, _, _, _, h946) => {
                                h946
                            }
                        };
                        Ok(nat_le(a947, a948))
                    }
                }
            }
        }
    }
}

pub fn f48(v0: Adt1) -> R<bool> {
    let v1: u64 = {
        let a951 = match v0.clone() {
            Adt1::C0(_, _, h949, _, _, _, _, _, _, _, _) => {
                h949
            }
        };
        let a952 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h950, _, _, _, _) => {
                h950
            }
        };
        nat_mul(a951, a952)?
    };
    if if f43(v0.clone()) {
        let a954 = match v0.clone() {
            Adt1::C0(_, _, _, h953, _, _, _, _, _, _, _) => {
                h953
            }
        };
        let a955 = 1u64;
        if nat_eq(a954, a955) {
            true
        } else {
            let a957 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h956, _, _, _) => {
                    h956
                }
            };
            let a958 = v1;
            nat_eq(a957, a958)
        }
    } else {
        false
    } {
        let a960 = match v0.clone() {
            Adt1::C0(_, _, _, _, h959, _, _, _, _, _, _) => {
                h959
            }
        };
        let a961 = 1u64;
        if nat_eq(a960, a961) {
            Ok(true)
        } else {
            let a966 = match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h962, _, _) => {
                    h962
                }
            };
            let a967 = {
                let a964 = v1;
                let a965 = match v0 {
                    Adt1::C0(_, _, _, h963, _, _, _, _, _, _, _) => {
                        h963
                    }
                };
                nat_mul(a964, a965)?
            };
            Ok(nat_eq(a966, a967))
        }
    } else {
        Ok(false)
    }
}

pub fn f49(v0: u64) -> u64 {
    let a968 = v0;
    let a969 = 3u64;
    if nat_eq(a968, a969) {
        16u64
    } else {
        10u64
    }
}

pub fn f50(v0: u64, v1: Adt1) -> R<u64> {
    let v2: u64 = {
        let a971 = v0;
        let a972 = match v1.clone() {
            Adt1::C0(_, h970, _, _, _, _, _, _, _, _, _) => {
                h970
            }
        };
        let a973 = 0u64;
        nat_quot(a971, a972, a973)
    };
    let v3: u64 = {
        let a975 = v2;
        let a976 = match v1.clone() {
            Adt1::C0(_, _, h974, _, _, _, _, _, _, _, _) => {
                h974
            }
        };
        let a977 = 0u64;
        nat_quot(a975, a976, a977)
    };
    f52({
        let a979 = v0;
        let a980 = match v1.clone() {
            Adt1::C0(_, h978, _, _, _, _, _, _, _, _, _) => {
                h978
            }
        };
        let a981 = 0u64;
        nat_rem(a979, a980, a981)
    }, {
        let a983 = v2;
        let a984 = match v1.clone() {
            Adt1::C0(_, _, h982, _, _, _, _, _, _, _, _) => {
                h982
            }
        };
        let a985 = 0u64;
        nat_rem(a983, a984, a985)
    }, {
        let a987 = v3;
        let a988 = match v1.clone() {
            Adt1::C0(_, _, _, h986, _, _, _, _, _, _, _) => {
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
        nat_quot(a991, a992, a993)
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, h994, _, _, _, _, _) => {
            h994
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, h995, _, _, _, _) => {
            h995
        }
    }, match v1.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h996, _, _, _) => {
            h996
        }
    }, match v1 {
        Adt1::C0(_, _, _, _, _, _, _, _, h997, _, _) => {
            h997
        }
    })
}

pub fn f51(v0: u64, v1: Adt1, v2: Adt1) -> R<u64> {
    let v3: u64 = {
        let a999 = v0;
        let a1000 = match v1.clone() {
            Adt1::C0(_, h998, _, _, _, _, _, _, _, _, _) => {
                h998
            }
        };
        let a1001 = 0u64;
        nat_quot(a999, a1000, a1001)
    };
    let v4: u64 = {
        let a1003 = v3;
        let a1004 = match v1.clone() {
            Adt1::C0(_, _, h1002, _, _, _, _, _, _, _, _) => {
                h1002
            }
        };
        let a1005 = 0u64;
        nat_quot(a1003, a1004, a1005)
    };
    f52({
        let a1011 = {
            let a1007 = v0;
            let a1008 = match v1.clone() {
                Adt1::C0(_, h1006, _, _, _, _, _, _, _, _, _) => {
                    h1006
                }
            };
            let a1009 = 0u64;
            nat_rem(a1007, a1008, a1009)
        };
        let a1012 = match v2.clone() {
            Adt1::C0(_, h1010, _, _, _, _, _, _, _, _, _) => {
                h1010
            }
        };
        let a1013 = 0u64;
        nat_rem(a1011, a1012, a1013)
    }, {
        let a1019 = {
            let a1015 = v3;
            let a1016 = match v1.clone() {
                Adt1::C0(_, _, h1014, _, _, _, _, _, _, _, _) => {
                    h1014
                }
            };
            let a1017 = 0u64;
            nat_rem(a1015, a1016, a1017)
        };
        let a1020 = match v2.clone() {
            Adt1::C0(_, _, h1018, _, _, _, _, _, _, _, _) => {
                h1018
            }
        };
        let a1021 = 0u64;
        nat_rem(a1019, a1020, a1021)
    }, {
        let a1027 = {
            let a1023 = v4;
            let a1024 = match v1.clone() {
                Adt1::C0(_, _, _, h1022, _, _, _, _, _, _, _) => {
                    h1022
                }
            };
            let a1025 = 0u64;
            nat_rem(a1023, a1024, a1025)
        };
        let a1028 = match v2.clone() {
            Adt1::C0(_, _, _, h1026, _, _, _, _, _, _, _) => {
                h1026
            }
        };
        let a1029 = 0u64;
        nat_rem(a1027, a1028, a1029)
    }, {
        let a1035 = {
            let a1031 = v4;
            let a1032 = match v1 {
                Adt1::C0(_, _, _, h1030, _, _, _, _, _, _, _) => {
                    h1030
                }
            };
            let a1033 = 0u64;
            nat_quot(a1031, a1032, a1033)
        };
        let a1036 = match v2.clone() {
            Adt1::C0(_, _, _, _, h1034, _, _, _, _, _, _) => {
                h1034
            }
        };
        let a1037 = 0u64;
        nat_rem(a1035, a1036, a1037)
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, h1038, _, _, _, _, _) => {
            h1038
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, h1039, _, _, _, _) => {
            h1039
        }
    }, match v2.clone() {
        Adt1::C0(_, _, _, _, _, _, _, h1040, _, _, _) => {
            h1040
        }
    }, match v2 {
        Adt1::C0(_, _, _, _, _, _, _, _, h1041, _, _) => {
            h1041
        }
    })
}

pub fn f52(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a1054 = {
        let a1050 = {
            let a1046 = {
                let a1042 = v0;
                let a1043 = v4;
                nat_mul(a1042, a1043)?
            };
            let a1047 = {
                let a1044 = v1;
                let a1045 = v5;
                nat_mul(a1044, a1045)?
            };
            nat_add(a1046, a1047)?
        };
        let a1051 = {
            let a1048 = v2;
            let a1049 = v6;
            nat_mul(a1048, a1049)?
        };
        nat_add(a1050, a1051)?
    };
    let a1055 = {
        let a1052 = v3;
        let a1053 = v7;
        nat_mul(a1052, a1053)?
    };
    nat_add(a1054, a1055)
}

pub fn f53(v0: u64, v1: Adt1, v2: u64, v3: u64) -> R<Adt3> {
    let v4: u64 = {
        let a1056 = v0;
        let a1057 = 2u64;
        let a1058 = 0u64;
        nat_quot(a1056, a1057, a1058)
    };
    let v5: u64 = f42(match v1.clone() {
        Adt1::C0(h1059, _, _, _, _, _, _, _, _, _, _) => {
            h1059
        }
    });
    let a1063 = {
        let a1060 = v0;
        let a1061 = 2u64;
        let a1062 = 0u64;
        nat_rem(a1060, a1061, a1062)
    };
    let a1064 = 0u64;
    if nat_eq(a1063, a1064) {
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 4u64, f50(v4, v1)?, 2u64, 0u64, 0u64, 2u64, v5, 8u64))
    } else {
        let a1065 = v3;
        let a1066 = v2;
        if if nat_lt(a1065, a1066) {
            let a1069 = v3;
            let a1070 = {
                let a1067 = v5;
                let a1068 = 4u64;
                if nat_eq(a1067, a1068) {
                    2147483648u64
                } else {
                    9223372036854775808u64
                }
            };
            nat_lt(a1069, a1070)
        } else {
            false
        } {
            Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, {
                let a1071 = 3u64;
                let a1072 = v4;
                nat_add(a1071, a1072)?
            }, 8u64, 8u64))
        } else {
            Ok(Adt3::C1)
        }
    }
}

pub fn f54(v0: u64, v1: Adt1, v2: Adt1, v3: Adt1, v4: u64) -> R<Adt3> {
    let v5: u64 = {
        let a1074 = 1u64;
        let a1075 = f44(v1.clone(), match v3.clone() {
            Adt1::C0(h1073, _, _, _, _, _, _, _, _, _, _) => {
                h1073
            }
        })?;
        nat_add(a1074, a1075)?
    };
    let v6: u64 = {
        let a1076 = v0;
        let a1077 = v5;
        let a1078 = 0u64;
        nat_rem(a1076, a1077, a1078)
    };
    let v7: u64 = {
        let a1079 = v0;
        let a1080 = v5;
        let a1081 = 0u64;
        nat_quot(a1079, a1080, a1081)
    };
    let v8: u64 = {
        let a1083 = v7;
        let a1084 = match v1.clone() {
            Adt1::C0(_, _, h1082, _, _, _, _, _, _, _, _) => {
                h1082
            }
        };
        let a1085 = 0u64;
        nat_rem(a1083, a1084, a1085)
    };
    let v9: u64 = {
        let a1087 = v7;
        let a1088 = match v1.clone() {
            Adt1::C0(_, _, h1086, _, _, _, _, _, _, _, _) => {
                h1086
            }
        };
        let a1089 = 0u64;
        nat_quot(a1087, a1088, a1089)
    };
    let v10: u64 = {
        let a1091 = v9;
        let a1092 = match v1.clone() {
            Adt1::C0(_, _, _, h1090, _, _, _, _, _, _, _) => {
                h1090
            }
        };
        let a1093 = 0u64;
        nat_rem(a1091, a1092, a1093)
    };
    let v11: u64 = {
        let a1095 = v9;
        let a1096 = match v1.clone() {
            Adt1::C0(_, _, _, h1094, _, _, _, _, _, _, _) => {
                h1094
            }
        };
        let a1097 = 0u64;
        nat_quot(a1095, a1096, a1097)
    };
    let a1098 = v6;
    let a1099 = 0u64;
    if nat_eq(a1098, a1099) {
        let v12: u64 = {
            let a1116 = v8;
            let a1117 = {
                let a1114 = match v2.clone() {
                    Adt1::C0(_, h1100, _, _, _, _, _, _, _, _, _) => {
                        h1100
                    }
                };
                let a1115 = {
                    let a1112 = {
                        let a1102 = v10;
                        let a1103 = match v2.clone() {
                            Adt1::C0(_, _, h1101, _, _, _, _, _, _, _, _) => {
                                h1101
                            }
                        };
                        let a1104 = 0u64;
                        nat_rem(a1102, a1103, a1104)
                    };
                    let a1113 = {
                        let a1110 = match v2.clone() {
                            Adt1::C0(_, _, h1105, _, _, _, _, _, _, _, _) => {
                                h1105
                            }
                        };
                        let a1111 = {
                            let a1107 = v11;
                            let a1108 = match v2 {
                                Adt1::C0(_, _, _, h1106, _, _, _, _, _, _, _) => {
                                    h1106
                                }
                            };
                            let a1109 = 0u64;
                            nat_rem(a1107, a1108, a1109)
                        };
                        nat_mul(a1110, a1111)?
                    };
                    nat_add(a1112, a1113)?
                };
                nat_mul(a1114, a1115)?
            };
            nat_add(a1116, a1117)?
        };
        Ok(Adt3::C3(2u64, 3u64, 0u64, 0u64, 1u64, {
            let a1118 = 3u64;
            let a1119 = v12;
            nat_add(a1118, a1119)?
        }, 2u64, 0u64, 0u64, 2u64, 8u64, 8u64))
    } else {
        let a1121 = v4;
        let a1122 = match v3.clone() {
            Adt1::C0(_, _, h1120, _, _, _, _, _, _, _, _) => {
                h1120
            }
        };
        if nat_lt(a1121, a1122) {
            let v13: u64 = {
                let a1127 = {
                    let a1123 = v6;
                    let a1124 = 1u64;
                    nat_sub(a1123, a1124)
                };
                let a1128 = f65(match v1.clone() {
                    Adt1::C0(h1125, _, _, _, _, _, _, _, _, _, _) => {
                        h1125
                    }
                }, match v3.clone() {
                    Adt1::C0(h1126, _, _, _, _, _, _, _, _, _, _) => {
                        h1126
                    }
                });
                let a1129 = 0u64;
                nat_quot(a1127, a1128, a1129)
            };
            let v14: u64 = f52({
                let a1131 = v13;
                let a1132 = f29(match v3.clone() {
                    Adt1::C0(h1130, _, _, _, _, _, _, _, _, _, _) => {
                        h1130
                    }
                });
                nat_mul(a1131, a1132)?
            }, v8, v10, v11, match v1.clone() {
                Adt1::C0(_, _, _, _, _, h1133, _, _, _, _, _) => {
                    h1133
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, h1134, _, _, _, _) => {
                    h1134
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1135, _, _, _) => {
                    h1135
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1136, _, _) => {
                    h1136
                }
            })?;
            let v15: u64 = f52(v13, v4, v10, v11, match v3.clone() {
                Adt1::C0(_, _, _, _, _, h1137, _, _, _, _, _) => {
                    h1137
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, h1138, _, _, _, _) => {
                    h1138
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1139, _, _, _) => {
                    h1139
                }
            }, match v3.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1140, _, _) => {
                    h1140
                }
            })?;
            let a1142 = 0u64;
            let a1143 = f45(match v3.clone() {
                Adt1::C0(h1141, _, _, _, _, _, _, _, _, _, _) => {
                    h1141
                }
            });
            if nat_lt(a1142, a1143) {
                Ok(Adt3::C6(v14, v15))
            } else {
                Ok(f66(match v1 {
                    Adt1::C0(h1144, _, _, _, _, _, _, _, _, _, _) => {
                        h1144
                    }
                }, match v3 {
                    Adt1::C0(h1145, _, _, _, _, _, _, _, _, _, _) => {
                        h1145
                    }
                }, {
                    let a1146 = v6;
                    let a1147 = 1u64;
                    nat_sub(a1146, a1147)
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
    let a1148 = v0;
    let a1149 = 8u64;
    if nat_eq(a1148, a1149) {
        Adt3::C3(1u64, 6u64, 0u64, 0u64, 1u64, 0u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
    } else {
        let a1150 = v0;
        let a1151 = 9u64;
        if nat_eq(a1150, a1151) {
            Adt3::C3(1u64, 2u64, 0u64, 0u64, 2u64, v1, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
        } else {
            let a1152 = v0;
            let a1153 = 10u64;
            if nat_eq(a1152, a1153) {
                Adt3::C3(5u64, 16u64, 0u64, 0u64, 1u64, 2u64, 2u64, 0u64, 0u64, 2u64, 4u64, 4u64)
            } else {
                let a1154 = v0;
                let a1155 = 11u64;
                if nat_eq(a1154, a1155) {
                    Adt3::C3(1u64, 0u64, 0u64, 0u64, 2u64, 1065353216u64, 1u64, 2u64, 0u64, 2u64, 4u64, 4u64)
                } else {
                    let a1156 = v0;
                    let a1157 = 12u64;
                    if nat_eq(a1156, a1157) {
                        Adt3::C3(1u64, 3u64, 0u64, 0u64, 1u64, 0u64, 1u64, 2u64, 0u64, 0u64, 4u64, 4u64)
                    } else {
                        let a1158 = v0;
                        let a1159 = 13u64;
                        if nat_eq(a1158, a1159) {
                            Adt3::C3(1u64, 0u64, 0u64, 0u64, 1u64, 1u64, 2u64, 1065353216u64, 0u64, 1u64, 4u64, 4u64)
                        } else {
                            let a1160 = v0;
                            let a1161 = 14u64;
                            if nat_eq(a1160, a1161) {
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

pub fn f58(v0: u64, v1: u64, v2: Adt2) -> R<Adt3> {
    let v3: Adt1 = match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1162, _, _, _, _) => {
            h1162
        }
    };
    let v4: Adt1 = match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h1163, _, _, _) => {
            h1163
        }
    };
    let v5: Adt1 = match v2.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1164, _, _) => {
            h1164
        }
    };
    let v6: u64 = match v2.clone() {
        Adt2::C0(_, _, _, h1165, _, _, _, _, _, _, _, _) => {
            h1165
        }
    };
    let v7: u64 = {
        let a1167 = v1;
        let a1168 = match v5.clone() {
            Adt1::C0(_, h1166, _, _, _, _, _, _, _, _, _) => {
                h1166
            }
        };
        let a1169 = 0u64;
        nat_quot(a1167, a1168, a1169)
    };
    let v8: u64 = f21(match v5.clone() {
        Adt1::C0(h1170, _, _, _, _, _, _, _, _, _, _) => {
            h1170
        }
    });
    let v9: u64 = {
        let a1175 = {
            let a1172 = v1;
            let a1173 = match v5.clone() {
                Adt1::C0(_, h1171, _, _, _, _, _, _, _, _, _) => {
                    h1171
                }
            };
            let a1174 = 0u64;
            nat_rem(a1172, a1173, a1174)
        };
        let a1176 = v8;
        nat_mul(a1175, a1176)?
    };
    let v10: u64 = {
        let a1178 = match v5.clone() {
            Adt1::C0(_, h1177, _, _, _, _, _, _, _, _, _) => {
                h1177
            }
        };
        let a1179 = v8;
        nat_mul(a1178, a1179)?
    };
    let a1180 = v0;
    let a1181 = 0u64;
    if nat_eq(a1180, a1181) {
        Ok(Adt3::C3(0u64, 0u64, match v3.clone() {
            Adt1::C0(h1182, _, _, _, _, _, _, _, _, _, _) => {
                h1182
            }
        }, 0u64, 0u64, {
            let a1190 = {
                let a1186 = {
                    let a1184 = v7;
                    let a1185 = match v3 {
                        Adt1::C0(_, _, _, _, _, _, h1183, _, _, _, _) => {
                            h1183
                        }
                    };
                    nat_mul(a1184, a1185)?
                };
                let a1187 = v9;
                nat_add(a1186, a1187)?
            };
            let a1191 = {
                let a1188 = v6;
                let a1189 = 100u64;
                if nat_eq(a1188, a1189) {
                    v10
                } else {
                    0u64
                }
            };
            nat_add(a1190, a1191)?
        }, 2u64, 0u64, 0u64, 0u64, v8, 4u64))
    } else {
        let a1192 = v0;
        let a1193 = 1u64;
        if nat_eq(a1192, a1193) {
            Ok(Adt3::C3(0u64, 0u64, match v4.clone() {
                Adt1::C0(h1194, _, _, _, _, _, _, _, _, _, _) => {
                    h1194
                }
            }, 0u64, {
                let a1195 = v6;
                let a1196 = 98u64;
                if nat_eq(a1195, a1196) {
                    4u64
                } else {
                    0u64
                }
            }, {
                let a1204 = {
                    let a1200 = {
                        let a1198 = v7;
                        let a1199 = match v4 {
                            Adt1::C0(_, _, _, _, _, _, h1197, _, _, _, _) => {
                                h1197
                            }
                        };
                        nat_mul(a1198, a1199)?
                    };
                    let a1201 = v9;
                    nat_add(a1200, a1201)?
                };
                let a1205 = {
                    let a1202 = v6;
                    let a1203 = 99u64;
                    if nat_eq(a1202, a1203) {
                        v10
                    } else {
                        0u64
                    }
                };
                nat_add(a1204, a1205)?
            }, 2u64, 0u64, 0u64, 1u64, v8, 4u64))
        } else {
            let a1206 = v0;
            let a1207 = 2u64;
            if nat_eq(a1206, a1207) {
                Ok(Adt3::C4(f67(match v2 {
                    Adt2::C0(_, _, _, _, h1208, _, _, _, _, _, _, _) => {
                        h1208
                    }
                }, v5.clone(), v1), match v5 {
                    Adt1::C0(h1209, _, _, _, _, _, _, _, _, _, _) => {
                        h1209
                    }
                }, 0u64, 1u64, 0u64))
            } else {
                Ok(Adt3::C3(0u64, 0u64, 0u64, match v5.clone() {
                    Adt1::C0(h1210, _, _, _, _, _, _, _, _, _, _) => {
                        h1210
                    }
                }, 1u64, 0u64, 2u64, 0u64, 3u64, {
                    let a1214 = {
                        let a1212 = v7;
                        let a1213 = match v5 {
                            Adt1::C0(_, _, _, _, _, _, h1211, _, _, _, _) => {
                                h1211
                            }
                        };
                        nat_mul(a1212, a1213)?
                    };
                    let a1215 = v9;
                    nat_add(a1214, a1215)?
                }, 4u64, v8))
            }
        }
    }
}

pub fn f59(v0: u64) -> Adt4 {
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

pub fn f60(v0: Adt1, v1: Adt1) -> bool {
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

pub fn f61(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
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

pub fn f62(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
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

pub fn f63(v0: u64, v1: u64, v2: u64) -> bool {
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

pub fn f64(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f63(v0, v3, v6) {
        f63(v1, v4, v6)
    } else {
        false
    } {
        f63(v2, v5, v6)
    } else {
        false
    }
}

pub fn f65(v0: u64, v1: u64) -> u64 {
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

pub fn f66(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt3 {
    let a1361 = f65(v0, v1);
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

pub fn f67(v0: u64, v1: Adt1, v2: u64) -> u64 {
    let a1368 = v0;
    let a1369 = 2u64;
    if if nat_eq(a1368, a1369) {
        let a1371 = match v1.clone() {
            Adt1::C0(h1370, _, _, _, _, _, _, _, _, _, _) => {
                h1370
            }
        };
        let a1372 = 0u64;
        if nat_eq(a1371, a1372) {
            let a1384 = {
                let a1374 = v2;
                let a1375 = match v1.clone() {
                    Adt1::C0(_, h1373, _, _, _, _, _, _, _, _, _) => {
                        h1373
                    }
                };
                let a1376 = 0u64;
                nat_rem(a1374, a1375, a1376)
            };
            let a1385 = {
                let a1382 = match v1.clone() {
                    Adt1::C0(_, h1377, _, _, _, _, _, _, _, _, _) => {
                        h1377
                    }
                };
                let a1383 = {
                    let a1379 = match v1 {
                        Adt1::C0(_, h1378, _, _, _, _, _, _, _, _, _) => {
                            h1378
                        }
                    };
                    let a1380 = 4u64;
                    let a1381 = 0u64;
                    nat_rem(a1379, a1380, a1381)
                };
                nat_sub(a1382, a1383)
            };
            nat_lt(a1384, a1385)
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
